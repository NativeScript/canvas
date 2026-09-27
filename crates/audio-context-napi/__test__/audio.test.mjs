// Offline contexts and the 'none' sink: no audio device needed.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import url from 'node:url';

const here = path.dirname(url.fileURLToPath(import.meta.url));
const root = path.resolve(here, '../../..');
const file = { win32: 'audio_context_napi.dll', darwin: 'libaudio_context_napi.dylib', linux: 'libaudio_context_napi.so' }[process.platform];
const addon = [process.env.AUDIO_CONTEXT_NAPI_ADDON, path.join(root, 'target', 'debug', file), path.join(root, 'target', 'release-napi', file)].filter(Boolean).find((f) => fs.existsSync(f));
const module = { exports: {} };
process.dlopen(module, addon);
const Audio = module.exports;
const assets = path.join(root, 'apps', 'demo', 'src', 'assets', 'file-assets', 'audio');

const rms = (samples) => Math.sqrt(samples.reduce((sum, v) => sum + v * v, 0) / samples.length);

test('exports the classes index.windows.ts builds on', () => {
	for (const name of ['AudioContext', 'AudioNode', 'AudioParam', 'AudioBuffer', 'PeriodicWave']) {
		assert.equal(typeof Audio[name], 'function', name);
	}
});

test('renders an oscillator through a gain offline', async () => {
	const context = Audio.AudioContext.offline(1, 4800, 48000);
	assert.equal(context.sampleRate, 48000);
	assert.equal(context.length, 4800);
	const oscillator = context.createOscillator();
	oscillator.type = 'square';
	assert.equal(oscillator.type, 'square');
	oscillator.param('frequency').value = 1000;
	const gain = context.createGain();
	gain.param('gain').value = 0.5;
	oscillator.connect(gain);
	gain.connect(context.destination());
	oscillator.start(0);
	const rendered = await context.startRendering();
	assert.equal(rendered.length, 4800);
	assert.equal(rendered.numberOfChannels, 1);
	const samples = rendered.channelData(0);
	assert.ok(Math.abs(rms(samples.subarray(480)) - 0.5) < 0.02, `rms ${rms(samples)}`);
	assert.equal(context.state, 'closed');
	await assert.rejects(async () => context.startRendering(), /InvalidStateError/);
});

test('automates params on the timeline', async () => {
	const context = Audio.AudioContext.offline(1, 48000, 48000);
	const source = context.createConstantSource();
	const offset = source.param('offset');
	assert.equal(offset.defaultValue, 1);
	offset.setValueAtTime(0, 0);
	offset.linearRampToValueAtTime(1, 1);
	source.connect(context.destination());
	source.start();
	const samples = (await context.startRendering()).channelData(0);
	assert.ok(Math.abs(samples[24000] - 0.5) < 0.01, `${samples[24000]}`);
	assert.ok(samples[47999] > 0.99);
});

test('plays buffers with loops, offsets and ended events', async () => {
	const context = Audio.AudioContext.offline(2, 9600, 48000);
	const left = new Float32Array(4800).fill(0.25);
	const right = new Float32Array(4800).fill(-0.25);
	const buffer = Audio.AudioBuffer.fromChannels([left, right], 48000);
	assert.equal(buffer.numberOfChannels, 2);
	assert.equal(buffer.duration, 0.1);
	const source = context.createBufferSource();
	source.setBuffer(buffer);
	assert.throws(() => source.setBuffer(buffer), /InvalidStateError/);
	source.loop = true;
	source.connect(context.destination());
	let ended = false;
	source.setOnended(() => (ended = true));
	source.start(0, 0, 0.15);
	assert.throws(() => source.start(), /InvalidStateError/);
	const rendered = await context.startRendering();
	const out = new Float32Array(9600);
	rendered.copyFromChannel(out, 1, 0);
	assert.equal(out[100], -0.25);
	assert.equal(out[7000], -0.25);
	assert.equal(out[7300], 0);
	await new Promise((resolve) => setTimeout(resolve, 50));
	assert.ok(ended, 'onended');
});

test('reports spec errors as exceptions instead of aborting', () => {
	const context = Audio.AudioContext.offline(1, 128, 48000);
	const other = Audio.AudioContext.offline(1, 128, 48000);
	const gain = context.createGain();
	assert.throws(() => gain.connect(other.createGain()), /InvalidAccessError/);
	assert.throws(() => gain.connect(context.createGain(), 3), /IndexSizeError/);
	assert.throws(() => context.createOscillator().stop(), /InvalidStateError/);
	assert.throws(() => (context.createAnalyser().fftSize = 1000), /IndexSizeError/);
	assert.throws(() => gain.fftSize, /GainNode has no member 'fftSize'/);
	assert.throws(() => (context.createOscillator().type = 'nope'), /TypeError/);
	assert.equal(gain.param('frequency'), null);
});

test('connects nodes to params and splits channels', async () => {
	const context = Audio.AudioContext.offline(2, 1280, 48000);
	const lfo = context.createConstantSource();
	lfo.param('offset').value = 0.25;
	const carrier = context.createConstantSource();
	const gain = context.createGain();
	gain.param('gain').value = 0;
	lfo.connectParam(gain.param('gain'));
	carrier.connect(gain);
	const merger = context.createChannelMerger(2);
	assert.equal(merger.numberOfInputs, 2);
	gain.connect(merger, 0, 1);
	merger.connect(context.destination());
	lfo.start();
	carrier.start();
	const rendered = await context.startRendering();
	assert.equal(rendered.channelData(0)[640], 0);
	assert.equal(rendered.channelData(1)[640], 0.25);
});

test('filters, analyses, shapes and compresses', async () => {
	const context = Audio.AudioContext.offline(1, 4096, 48000);
	const filter = context.createBiquadFilter();
	filter.type = 'lowpass';
	filter.param('frequency').value = 1000;
	const frequencies = new Float32Array([100, 10000]);
	const magnitude = new Float32Array(2);
	const phase = new Float32Array(2);
	filter.getFrequencyResponse(frequencies, magnitude, phase);
	assert.ok(magnitude[0] > 0.9 && magnitude[1] < 0.05, `${magnitude}`);

	const iir = context.createIirFilter([0.5, 0.5], [1]);
	iir.getFrequencyResponse(new Float32Array([0]), magnitude.subarray(0, 1), phase.subarray(0, 1));
	assert.ok(Math.abs(magnitude[0] - 1) < 1e-4);

	const shaper = context.createWaveShaper();
	shaper.setCurve(new Float32Array([-0.5, 0, 0.5]));
	shaper.oversample = '2x';
	assert.equal(shaper.oversample, '2x');
	const compressor = context.createDynamicsCompressor();
	assert.equal(compressor.param('threshold').value, -24);
	assert.equal(compressor.reduction, 0);

	const analyser = context.createAnalyser();
	analyser.fftSize = 512;
	assert.equal(analyser.frequencyBinCount, 256);
	const oscillator = context.createOscillator();
	oscillator.connect(analyser);
	analyser.connect(context.destination());
	oscillator.start();
	await context.startRendering();
	const time = new Float32Array(512);
	analyser.getFloatTimeDomainData(time);
	assert.ok(Math.max(...time) > 0.9);
	const bytes = new Uint8Array(256);
	analyser.getByteFrequencyData(bytes);
	assert.ok(bytes.some((b) => b > 200));
});

test('pans with the listener and HRTF', async () => {
	const context = Audio.AudioContext.offline(2, 2048, 44100);
	assert.equal(context.listenerParam('upY').value, 1);
	assert.equal(context.listenerParam('nope'), null);
	const panner = context.createPanner();
	panner.panningModel = 'HRTF';
	panner.distanceModel = 'linear';
	panner.refDistance = 2;
	assert.deepEqual([panner.panningModel, panner.distanceModel, panner.refDistance], ['HRTF', 'linear', 2]);
	panner.param('positionX').value = 5;
	const source = context.createConstantSource();
	source.connect(panner);
	panner.connect(context.destination());
	source.start();
	const rendered = await context.startRendering();
	assert.ok(rms(rendered.channelData(1).subarray(1024)) > rms(rendered.channelData(0).subarray(1024)));
});

test('uses a custom periodic wave', async () => {
	const context = Audio.AudioContext.offline(1, 480, 48000);
	const wave = context.createPeriodicWave(new Float32Array([0, 0]), new Float32Array([0, 1]), false);
	const oscillator = context.createOscillator();
	oscillator.setPeriodicWave(wave);
	assert.equal(oscillator.type, 'custom');
	oscillator.connect(context.destination());
	oscillator.start();
	assert.ok(rms((await context.startRendering()).channelData(0)) > 0.5);
});

test('decodes wav, mp3 and ogg, resampled to the context rate', async () => {
	const context = Audio.AudioContext.offline(1, 128, 48000);
	const wav = await context.decodeAudioFile(path.join(assets, 'gs-16b-1c-44100hz.wav'));
	assert.equal(wav.sampleRate, 48000);
	assert.equal(wav.numberOfChannels, 1);
	assert.ok(wav.duration > 15, `${wav.duration}`);
	const mp3 = await context.decodeAudioData(fs.readFileSync(path.join(assets, 'sine441stereo.mp3')));
	assert.equal(mp3.numberOfChannels, 2);
	const ogg = await context.decodeAudioFile(path.join(assets, 'ogg_sample.ogg'));
	assert.ok(ogg.length > 0);
	const base64 = fs.readFileSync(path.join(assets, 'gs-16b-1c-44100hz.wav')).toString('base64');
	assert.equal((await context.decodeAudioBase64(base64)).length, wav.length);
	assert.equal((await context.decodeAudioBase64(`data:audio/wav;base64,${base64}`)).length, wav.length);
	await assert.rejects(context.decodeAudioBase64('not base64!'), /EncodingError/);
	await assert.rejects(context.decodeAudioData(new Uint8Array([1, 2, 3, 4])), /EncodingError/);
	await assert.rejects(context.decodeAudioFile(path.join(assets, 'missing.wav')), /NotFoundError/);
});

test('runs a realtime context without a device and reports its state', async () => {
	const context = new Audio.AudioContext({ sinkId: 'none', latencyHint: 'playback', sampleRate: 44100 });
	assert.equal(context.sampleRate, 44100);
	assert.equal(context.sinkId, 'none');
	const states = [];
	context.setOnstatechange(() => states.push(context.state));
	await context.suspend();
	assert.equal(context.state, 'suspended');
	await context.resume();
	const start = context.currentTime;
	await new Promise((resolve) => setTimeout(resolve, 200));
	assert.ok(context.currentTime > start, `${context.currentTime} > ${start}`);
	await context.close();
	assert.equal(context.state, 'closed');
	await new Promise((resolve) => setTimeout(resolve, 50));
	assert.ok(states.includes('suspended') && states.at(-1) === 'closed', states.join());
	assert.throws(() => context.startRendering(), /InvalidStateError/);
});
