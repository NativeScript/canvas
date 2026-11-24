import { AudioContext, AudioWorkletNode, OfflineAudioContext } from '@nativescript/audio-context';
import { isAndroid } from '@nativescript/core';
import { suite, test, skip, equal, closeTo, ok } from './harness';

// AudioWorkletNode inputs, outputs and fan-in, against what Chrome does.

const RATE = 48000;
const Q = 128;

const PROCESSORS = `
class Constant extends AudioWorkletProcessor {
	process(inputs, outputs) {
		outputs.forEach((output, o) => output.forEach((channel) => channel.fill(0.25 * (o + 1))));
		return true;
	}
}
registerProcessor('constant', Constant);

class Doubler extends AudioWorkletProcessor {
	process(inputs, outputs) {
		const input = inputs[0], output = outputs[0];
		for (let c = 0; c < output.length; c++) for (let i = 0; i < output[c].length; i++) output[c][i] = 2 * (input[c] ? input[c][i] : 0);
		return true;
	}
}
registerProcessor('doubler', Doubler);

class InputCount extends AudioWorkletProcessor {
	process(inputs, outputs) {
		outputs[0][0].fill(inputs[0].length);
		return true;
	}
}
registerProcessor('input-count', InputCount);

class SumInputs extends AudioWorkletProcessor {
	process(inputs, outputs) {
		const out = outputs[0][0];
		out.fill(0);
		inputs.forEach((input, i) => {
			if (input[0]) for (let k = 0; k < out.length; k++) out[k] += (i + 1) * input[0][k];
		});
		return true;
	}
}
registerProcessor('sum-inputs', SumInputs);
`;

const MODULE = 'data:text/javascript,' + encodeURIComponent(PROCESSORS);

async function offline(length = Q * 8, channels = 1) {
	const context = new OfflineAudioContext(channels, length, RATE);
	await context.audioWorklet.addModule(MODULE);
	return context;
}

function constant(context: any, offset: number, when = 0) {
	const source = context.createConstantSource({ offset });
	source.start(when);
	return source;
}

const MONO = { channelCount: 1, channelCountMode: 'explicit' } as any;

export function registerAudioWorkletSpec() {
	suite('audio.worklet', () => {
		if (isAndroid) {
			skip('inputs and outputs', 'Android worklets are sources only');
			return;
		}
		if (typeof (globalThis as any).Worker !== 'function') {
			skip('inputs and outputs', 'no Worker in this runtime');
			return;
		}

		test('a source renders sample-exact offline', async () => {
			const context = await offline();
			const node = new AudioWorkletNode(context, 'constant', { numberOfInputs: 0, outputChannelCount: [1] });
			node.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			ok(
				samples.every((v) => v === 0.25),
				`first ${samples[0]}, last ${samples[samples.length - 1]}`,
			);
		});

		test('processes an input offline', async () => {
			const context = await offline();
			const node = new AudioWorkletNode(context, 'doubler', MONO);
			constant(context, 0.25).connect(node);
			node.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			ok(
				samples.every((v) => v === 0.5),
				`first ${samples[0]}, last ${samples[samples.length - 1]}`,
			);
		});

		test('sums every connection into an input', async () => {
			const context = await offline();
			const node = new AudioWorkletNode(context, 'doubler', MONO);
			constant(context, 0.25).connect(node);
			constant(context, 0.5).connect(node);
			node.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			closeTo(samples[0], 1.5, 1e-6, 'first sample');
			closeTo(samples[Q * 8 - 1], 1.5, 1e-6, 'last sample');
		});

		test('an input with nothing playing into it is empty', async () => {
			const context = await offline(Q * 3);
			const fed = new AudioWorkletNode(context, 'input-count', { outputChannelCount: [1] });
			constant(context, 1, Q / RATE).connect(fed);
			fed.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[0], 0, 'empty until the source starts');
			equal(samples[Q], 2, 'channelCount channels once it plays');

			const other = await offline(Q * 3);
			const unconnected = new AudioWorkletNode(other, 'input-count', { outputChannelCount: [1] });
			unconnected.connect(other.destination);
			const none = (await other.startRendering()).getChannelData(0);
			ok(
				none.every((v) => v === 0),
				'empty with nothing connected',
			);
		});

		test('feeds each input separately', async () => {
			const context = await offline();
			const node = new AudioWorkletNode(context, 'sum-inputs', { ...MONO, numberOfInputs: 2, outputChannelCount: [1] });
			constant(context, 0.25).connect(node, 0, 0);
			constant(context, 0.5).connect(node, 0, 1);
			node.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			// 1 * 0.25 + 2 * 0.5
			closeTo(samples[Q * 4], 1.25, 1e-6, 'weighted sum of the inputs');
		});

		test('plays each output from its own bus', async () => {
			for (const output of [0, 1]) {
				const context = await offline();
				const node = new AudioWorkletNode(context, 'constant', { numberOfInputs: 0, numberOfOutputs: 2, outputChannelCount: [1, 1] });
				node.connect(context.destination, output);
				const samples = (await context.startRendering()).getChannelData(0);
				equal(samples[Q * 4], 0.25 * (output + 1), `output ${output}`);
			}
		});

		async function realtimeLevel(wire: (context: any, analyser: any) => void, expected: number) {
			const context = new AudioContext();
			try {
				await context.audioWorklet.addModule(MODULE);
				const analyser = context.createAnalyser({ fftSize: 512 });
				wire(context, analyser);
				analyser.connect(context.destination);
				const deadline = Date.now() + 3000;
				const data = new Float32Array(512);
				while (Date.now() < deadline) {
					await new Promise((resolve) => setTimeout(resolve, 100));
					analyser.getFloatTimeDomainData(data);
					if (data.every((v) => Math.abs(v - expected) < 1e-3)) break;
				}
				ok(
					data.every((v) => Math.abs(v - expected) < 1e-3),
					`first ${data[0]}, last ${data[511]}`,
				);
			} finally {
				await context.close();
			}
		}

		// Mono up-mixes by copying and down-mixes by averaging, with no pan law.
		test('a mono source plays at full level in a realtime context', () =>
			realtimeLevel((context, analyser) => new AudioWorkletNode(context, 'constant', { numberOfInputs: 0, outputChannelCount: [1] }).connect(analyser), 0.25));

		test('processes an input in a realtime context', () =>
			realtimeLevel((context, analyser) => {
				const node = new AudioWorkletNode(context, 'doubler', MONO);
				constant(context, 0.25).connect(node);
				node.connect(analyser);
			}, 0.5));
	});
}
