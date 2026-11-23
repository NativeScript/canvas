// The AudioWorklet scope script in a Node worker, serving an offline context.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import url from 'node:url';
import { Worker } from 'node:worker_threads';

const here = path.dirname(url.fileURLToPath(import.meta.url));
const root = path.resolve(here, '../../..');
const file = { win32: 'audio_context_napi.dll', darwin: 'libaudio_context_napi.dylib', linux: 'libaudio_context_napi.so' }[process.platform];
const addon = [process.env.AUDIO_CONTEXT_NAPI_ADDON, path.join(root, 'target', 'debug', file), path.join(root, 'target', 'release-napi', file)].filter(Boolean).find((f) => fs.existsSync(f));
const module = { exports: {} };
process.dlopen(module, addon);
const Audio = module.exports;

const scopeSource = fs.readFileSync(path.join(root, 'packages', 'audio-context', 'audio-worklet-scope.ts'), 'utf8').match(/String\.raw`([\s\S]*)`;\s*$/)[1];

// The globals a NativeScript Worker has.
const shim = `
const { parentPort, workerData } = require('node:worker_threads');
globalThis.postMessage = (message, transfer) => parentPort.postMessage(message, transfer);
globalThis.__audioWorkletLoadAddon = (file) => {
	const module = { exports: {} };
	process.dlopen(module, file);
	return module.exports;
};
parentPort.on('message', (data) => globalThis.onmessage({ data }));
(0, eval)(workerData.scope);
`;

/** Drives the scope as napi.ts does. */
class Scope {
	constructor(context) {
		this.context = context;
		this.native = context.audioWorkletScope();
		this.worker = new Worker(shim, { eval: true, workerData: { scope: scopeSource } });
		this.messages = [];
		this.waiters = [];
		this.worker.on('message', (message) => {
			this.messages.push(message);
			this.waiters = this.waiters.filter((waiter) => !waiter(message));
		});
		this.worker.postMessage({ t: 'init', scope: this.native.id, sampleRate: context.sampleRate, addon });
	}

	next(match) {
		const found = this.messages.find(match);
		if (found) return Promise.resolve(found);
		return new Promise((resolve) => this.waiters.push((message) => match(message) && (resolve(message), true)));
	}

	async addModule(source, id = 1) {
		this.worker.postMessage({ t: 'module', id, source, url: `test-module-${id}.js` });
		const done = await this.next((m) => m.t === 'module' && m.id === id);
		if (done.error) throw new Error(done.error);
	}

	descriptors(name) {
		return this.messages.find((m) => m.t === 'registered' && m.name === name).descriptors;
	}

	node(name, options = {}) {
		const node = this.context.createAudioWorkletNode(this.native, { ...options, descriptors: this.descriptors(name) });
		this.worker.postMessage({ t: 'create', id: node.workletId, name, options });
		return node;
	}

	async close() {
		this.worker.postMessage({ t: 'close' });
		await this.next((m) => m.t === 'closed');
		await this.worker.terminate();
	}
}

const processors = `
class Probe extends AudioWorkletProcessor {
	static get parameterDescriptors() {
		return [
			{ name: 'level', defaultValue: 0.25, minValue: 0, maxValue: 1, automationRate: 'k-rate' },
			{ name: 'ramp', defaultValue: 0 },
		];
	}
	constructor(options) {
		super(options);
		this.offset = options.processorOptions.offset;
		this.port.onmessage = (e) => this.port.postMessage({ echo: e.data, frame: currentFrame, rate: sampleRate });
	}
	process(inputs, outputs, parameters) {
		const [left, right] = outputs[0];
		left.fill(parameters.level[0] + this.offset);
		for (let i = 0; i < right.length; i++) right[i] = parameters.ramp.length === 1 ? parameters.ramp[0] : parameters.ramp[i];
		if (currentFrame === 0) this.port.postMessage({ first: currentTime, inputs: inputs.length, levelLength: parameters.level.length });
		return true;
	}
}
registerProcessor('probe', Probe);

class Doubler extends AudioWorkletProcessor {
	process(inputs, outputs) {
		const input = inputs[0], output = outputs[0];
		for (let c = 0; c < output.length; c++) for (let i = 0; i < 128; i++) output[c][i] = 2 * (input[c] ? input[c][i] : 0);
		return true;
	}
}
registerProcessor('doubler', Doubler);

class Broken extends AudioWorkletProcessor {
	process() {
		throw new Error('boom');
	}
}
registerProcessor('broken', Broken);

class Once extends AudioWorkletProcessor {
	constructor() {
		super();
		this.left = 2;
	}
	process(_inputs, outputs) {
		outputs[0][0].fill(1);
		return --this.left > 0;
	}
}
registerProcessor('once', Once);

class Counter extends AudioWorkletProcessor {
	process(inputs, outputs) {
		outputs[0][0].fill(inputs[0].length);
		return true;
	}
}
registerProcessor('counter', Counter);
`;

test('runs a processor with params, options and a port, sample-exact offline', async () => {
	const context = Audio.AudioContext.offline(2, 128 * 16, 48000);
	const scope = new Scope(context);
	try {
		await scope.addModule(processors);
		assert.deepEqual(
			scope.descriptors('probe').map((d) => [d.name, d.defaultValue, d.automationRate]),
			[
				['level', 0.25, 'k-rate'],
				['ramp', 0, 'a-rate'],
			],
		);
		const node = scope.node('probe', { numberOfInputs: 0, numberOfOutputs: 1, outputChannelCount: [2], processorOptions: { offset: 0.5 } });
		assert.equal(node.numberOfInputs, 0);
		assert.equal(node.channelCount, 2);
		const level = node.param('level');
		assert.equal(level.defaultValue, 0.25);
		assert.equal(level.maxValue, 1);
		assert.equal(level.automationRate, 'k-rate');
		level.setValueAtTime(0.125, (128 * 4) / 48000);
		const ramp = node.param('ramp');
		ramp.setValueAtTime(0, 0);
		ramp.linearRampToValueAtTime(1, (128 * 16) / 48000);
		node.connect(context.destination());
		const rendered = await context.startRendering();
		const left = rendered.channelData(0);
		const right = rendered.channelData(1);
		assert.equal(left[0], 0.75);
		assert.equal(left[128 * 4 - 1], 0.75);
		assert.equal(left[128 * 4], 0.625);
		assert.ok(right[1] > right[0] && Math.abs(right[1024] - 0.5) < 0.01, `${right[0]} ${right[1]} ${right[1024]}`);
		const first = await scope.next((m) => m.t === 'port' && m.data.first !== undefined);
		assert.deepEqual(first.data, { first: 0, inputs: 0, levelLength: 1 });
		scope.worker.postMessage({ t: 'port', id: node.workletId, data: { hello: [1, 2] } });
		const echo = await scope.next((m) => m.t === 'port' && m.data.echo);
		assert.deepEqual(echo.data.echo, { hello: [1, 2] });
		assert.equal(echo.data.rate, 48000);
	} finally {
		await scope.close();
	}
});

test('processes inputs', async () => {
	const context = Audio.AudioContext.offline(1, 128 * 4, 48000);
	const scope = new Scope(context);
	try {
		await scope.addModule(processors);
		const source = context.createConstantSource();
		source.param('offset').value = 0.25;
		const node = scope.node('doubler');
		source.connect(node);
		node.connect(context.destination());
		source.start(0);
		const samples = (await context.startRendering()).channelData(0);
		assert.ok(
			samples.every((v) => v === 0.5),
			`${samples[0]} ${samples[511]}`,
		);
	} finally {
		await scope.close();
	}
});

test('an input with nothing playing into it is empty', async () => {
	const context = Audio.AudioContext.offline(2, 128 * 3, 48000);
	const scope = new Scope(context);
	try {
		await scope.addModule(processors);
		const merger = context.createChannelMerger(2);
		merger.connect(context.destination());
		const fed = scope.node('counter', { outputChannelCount: [1] });
		fed.connect(merger, 0, 0);
		const unconnected = scope.node('counter', { outputChannelCount: [1] });
		unconnected.connect(merger, 0, 1);
		const source = context.createConstantSource();
		source.connect(fed);
		source.start(128 / 48000);
		const rendered = await context.startRendering();
		const [left, right] = [rendered.channelData(0), rendered.channelData(1)];
		assert.deepEqual([0, 1, 2].map((q) => left[q * 128]), [0, 1, 1], 'empty until the source starts');
		assert.ok(right.every((v) => v === 0), 'empty with nothing connected');
	} finally {
		await scope.close();
	}
});

test('a throwing processor reports processorerror and goes silent; false ends it', async () => {
	const context = Audio.AudioContext.offline(1, 128 * 4, 48000);
	const scope = new Scope(context);
	try {
		await scope.addModule(processors);
		const broken = scope.node('broken', { numberOfInputs: 0 });
		broken.connect(context.destination());
		const once = scope.node('once', { numberOfInputs: 0 });
		once.connect(context.destination());
		const samples = (await context.startRendering()).channelData(0);
		const error = await scope.next((m) => m.t === 'processorerror');
		assert.equal(error.id, broken.workletId);
		assert.match(error.message, /boom/);
		assert.equal(samples[0], 1);
		assert.equal(samples[128 * 2], 0, 'silent once process returned false');
	} finally {
		await scope.close();
	}
});

test('reports modules that fail and unknown processors', async () => {
	const context = Audio.AudioContext.offline(1, 128, 48000);
	const scope = new Scope(context);
	try {
		await assert.rejects(scope.addModule('throw new Error("bad module")', 7), /bad module/);
		await assert.rejects(scope.addModule('registerProcessor("x", 1)', 8), /not a constructor/);
		await scope.addModule(processors, 9);
		await assert.rejects(scope.addModule(processors, 10), /registered already/);
		assert.throws(() => Audio.AudioContext.offline(1, 128, 48000).createAudioWorkletNode(scope.native, { numberOfInputs: 0, numberOfOutputs: 0, descriptors: [] }), /NotSupportedError/);
	} finally {
		await scope.close();
	}
});

test('plays a processor in a realtime context, one device buffer late', async () => {
	const context = new Audio.AudioContext({ sinkId: 'none', latencyHint: 'interactive', sampleRate: 48000 });
	const scope = new Scope(context);
	try {
		await scope.addModule(processors);
		const node = scope.node('probe', { numberOfInputs: 0, numberOfOutputs: 1, outputChannelCount: [2], processorOptions: { offset: 0.5 } });
		const analyser = context.createAnalyser();
		node.connect(analyser);
		analyser.connect(context.destination());
		await context.resume();
		await new Promise((resolve) => setTimeout(resolve, 400));
		const samples = new Float32Array(analyser.fftSize);
		analyser.getFloatTimeDomainData(samples);
		// the analyser mixes 0.75 and the silent right channel down to mono
		assert.ok(
			samples.every((v) => Math.abs(v - 0.375) < 1e-6),
			`${samples[0]} ${samples.at(-1)}`,
		);
	} finally {
		await context.close();
		await scope.close();
	}
});

test('writes the scope script for the runtime Worker', () => {
	const file = Audio.audioWorkletScript(scopeSource);
	assert.equal(fs.readFileSync(file, 'utf8'), scopeSource);
	assert.equal(Audio.audioWorkletScript(scopeSource), file);
});
