import { AudioBuffer, AudioContext, OfflineAudioContext } from '@nativescript/audio-context';
import { suite, test, skip, equal, closeTo, ok, throws, rejects, TestBody } from './harness';

// Web Audio API surface: attributes, validation and events, per the spec.

declare const global: any;

const RATE = 48000;
const Q = 128;

const nativeTest = (name: string, body: TestBody) => (global.isAndroid || global.isIOS ? test(name, body) : skip(name, 'not yet on this platform'));

const offline = (channels = 1, length = Q): any => new OfflineAudioContext(channels, length, RATE);

const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

async function waitFor(condition: () => boolean, ms = 2000) {
	const end = Date.now() + ms;
	while (!condition() && Date.now() < end) await delay(10);
	return condition();
}

function errorName(e: any) {
	return e?.name ?? '';
}

export function registerAudioApiSpec() {
	suite('audio.api', () => {
		nativeTest('nodes report numberOfInputs and numberOfOutputs', () => {
			const context = offline();
			const io = (node: any) => `${node.numberOfInputs}/${node.numberOfOutputs}`;
			equal(io(context.createConstantSource()), '0/1', 'constant source');
			equal(io(context.createOscillator()), '0/1', 'oscillator');
			equal(io(context.createBufferSource()), '0/1', 'buffer source');
			equal(io(context.destination), '1/0', 'destination');
			equal(io(context.createAnalyser()), '1/1', 'analyser');
			equal(io(context.createGain()), '1/1', 'gain');
			equal(io(context.createChannelSplitter({ numberOfOutputs: 3 })), '1/3', 'splitter');
			equal(io(context.createChannelMerger({ numberOfInputs: 3 })), '3/1', 'merger');
			equal(io(context.createChannelMerger(2)), '2/1', 'merger from a number');
			equal(io(context.createChannelSplitter(2)), '1/2', 'splitter from a number');
		});

		nativeTest('ChannelSplitterNode and ChannelMergerNode keep their channel config', () => {
			const context = offline();
			const splitter = context.createChannelSplitter({ numberOfOutputs: 4 });
			equal(splitter.channelCount, 4);
			splitter.channelCount = 4;
			throws(() => (splitter.channelCount = 2), 'InvalidStateError', 'splitter channelCount');
			throws(() => (splitter.channelCountMode = 'max'), 'InvalidStateError', 'splitter channelCountMode');
			throws(() => (splitter.channelInterpretation = 'speakers'), 'InvalidStateError', 'splitter channelInterpretation');
			const merger = context.createChannelMerger();
			throws(() => (merger.channelCount = 2), 'InvalidStateError', 'merger channelCount');
			throws(() => (merger.channelCountMode = 'max'), 'InvalidStateError', 'merger channelCountMode');
			merger.channelInterpretation = 'discrete';
			equal(merger.channelInterpretation, 'discrete');
			throws(() => context.createChannelSplitter({ numberOfOutputs: 33 }), 'IndexSizeError');
		});

		nativeTest('stereo-only nodes reject more than two channels and max mode', () => {
			const context = offline();
			for (const node of [context.createPanner(), context.createStereoPanner(), context.createConvolver(), context.createDynamicsCompressor()]) {
				node.channelCount = 1;
				throws(() => (node.channelCount = 3), 'NotSupportedError', `${node.constructor.name} channelCount 3`);
				throws(() => (node.channelCountMode = 'max'), 'NotSupportedError', `${node.constructor.name} max`);
				node.channelCountMode = 'explicit';
				equal(node.channelCountMode, 'explicit');
			}
		});

		nativeTest('channelCount 0 or above 32 is NotSupportedError', () => {
			const gain = offline().createGain();
			throws(() => (gain.channelCount = 0), 'NotSupportedError');
			throws(() => (gain.channelCount = 33), 'NotSupportedError');
			gain.channelCount = 32;
			equal(gain.channelCount, 32);
		});

		nativeTest('destination reports maxChannelCount and limits channelCount', () => {
			const context = offline(2);
			equal(context.destination.maxChannelCount, 2);
			throws(() => (context.destination.channelCount = 1), 'InvalidStateError', 'an offline destination is fixed');
			const realtime: any = new AudioContext();
			try {
				ok(realtime.destination.maxChannelCount >= 2, `maxChannelCount ${realtime.destination.maxChannelCount}`);
				throws(() => (realtime.destination.channelCount = realtime.destination.maxChannelCount + 1), 'IndexSizeError');
			} finally {
				realtime.close();
			}
		});

		nativeTest('start and stop validate their arguments and order', () => {
			const context = offline();
			const source = context.createConstantSource();
			throws(() => source.stop(), 'InvalidStateError', 'stop before start');
			throws(() => source.start(-1), 'RangeError', 'negative when');
			throws(() => source.start(NaN), 'TypeError', 'NaN when');
			const buffered = context.createBufferSource();
			throws(() => buffered.start(0, -1), 'RangeError', 'negative offset');
			throws(() => buffered.start(0, 0, -1), 'RangeError', 'negative duration');
			source.start(0);
			throws(() => source.start(0), 'InvalidStateError', 'second start');
			throws(() => source.stop(-1), 'RangeError', 'negative stop');
			source.stop(1);
			source.stop(2);
		});

		nativeTest('ended fires once, after the call to stop returns', async () => {
			const context = offline();
			const source = context.createConstantSource();
			let ended = 0;
			source.onended = () => ended++;
			throws(() => source.stop(), 'InvalidStateError');
			await delay(50);
			equal(ended, 0, 'no ended without a start');
			source.start(0);
			source.stop(0);
			equal(ended, 0, 'not inside stop()');
			ok(await waitFor(() => ended > 0), 'ended fires');
			await delay(50);
			equal(ended, 1, 'once');
		});

		nativeTest('ended fires on a buffer source whose listener came before its buffer', async () => {
			const context = offline(1, Q * 8);
			const source = context.createBufferSource();
			let ended = 0;
			source.addEventListener('ended', () => ended++);
			const buffer = new AudioBuffer({ numberOfChannels: 1, length: Q, sampleRate: RATE });
			source.buffer = buffer;
			equal(source.buffer, buffer, 'the buffer reads back');
			source.connect(context.destination);
			source.start(0);
			await context.startRendering();
			ok(await waitFor(() => ended > 0), 'ended fires');
			equal(ended, 1, 'once');
		});

		nativeTest('AudioBufferSourceNode.buffer can only be set once', () => {
			const context = offline();
			const buffer = context.createBuffer(1, Q, RATE);
			const source = context.createBufferSource({ buffer });
			equal(source.buffer, buffer, 'options.buffer is honoured');
			source.buffer = null;
			throws(() => (source.buffer = buffer), 'InvalidStateError');
		});

		nativeTest('OfflineAudioContext validates its arguments', () => {
			throws(() => new OfflineAudioContext(0, Q, RATE), 'NotSupportedError', 'no channels');
			throws(() => new OfflineAudioContext(33, Q, RATE), 'NotSupportedError', '33 channels');
			throws(() => new OfflineAudioContext(1, 0, RATE), 'NotSupportedError', 'zero length');
			throws(() => new OfflineAudioContext(1, Q, 2999), 'NotSupportedError', 'low rate');
			throws(() => new (OfflineAudioContext as any)({ numberOfChannels: 1, length: Q, sampleRate: 768001 }), 'NotSupportedError', 'high rate');
			equal(new OfflineAudioContext(2, Q, 3000).length, Q);
		});

		nativeTest('OfflineAudioContext fires complete with the rendered buffer', async () => {
			const context = offline();
			let fromHandler: any = null;
			let fromListener: any = null;
			context.oncomplete = (event: any) => (fromHandler = event);
			context.addEventListener('complete', (event: any) => (fromListener = event));
			const rendered = await context.startRendering();
			ok(await waitFor(() => !!fromHandler && !!fromListener), 'both fire');
			equal(fromHandler.type, 'complete');
			equal(fromHandler.renderedBuffer, rendered, 'renderedBuffer is the resolved buffer');
		});

		nativeTest('createBuffer takes numberOfChannels, length and sampleRate', () => {
			const context = offline();
			const buffer = context.createBuffer(2, 100, 22050);
			equal(buffer.numberOfChannels, 2);
			equal(buffer.length, 100);
			equal(buffer.sampleRate, 22050);
			equal(context.createBuffer({ numberOfChannels: 1, length: 10, sampleRate: RATE }).length, 10, 'the options form still works');
		});

		nativeTest('AudioBuffer validates its options and channel indices', () => {
			throws(() => new AudioBuffer({ numberOfChannels: 1, length: 0, sampleRate: RATE }), 'NotSupportedError', 'zero length');
			throws(() => new AudioBuffer({ numberOfChannels: 0, length: 1, sampleRate: RATE }), 'NotSupportedError', 'no channels');
			throws(() => new AudioBuffer({ numberOfChannels: 33, length: 1, sampleRate: RATE }), 'NotSupportedError', '33 channels');
			throws(() => new AudioBuffer({ numberOfChannels: 1, length: 1, sampleRate: 1000 }), 'NotSupportedError', 'low rate');
			const buffer = new AudioBuffer({ numberOfChannels: 2, length: 4, sampleRate: 22050.5 });
			equal(buffer.sampleRate, 22050.5, 'not truncated');
			throws(() => buffer.getChannelData(2), 'IndexSizeError');
			throws(() => buffer.copyFromChannel(new Float32Array(4), 2), 'IndexSizeError');
			throws(() => buffer.copyToChannel(new Float32Array(4), 2), 'IndexSizeError');
		});

		nativeTest('AudioContext validates sampleRate and latencyHint', () => {
			throws(() => new AudioContext({ sampleRate: 1000 }), 'NotSupportedError');
			throws(() => new AudioContext({ latencyHint: 'fast' as any }), 'TypeError');
		});

		nativeTest('a closed AudioContext rejects close, resume and suspend', async () => {
			const context: any = new AudioContext();
			await context.close();
			equal(errorName(await rejects(context.close())), 'InvalidStateError', 'close');
			equal(errorName(await rejects(context.resume())), 'InvalidStateError', 'resume');
			equal(errorName(await rejects(context.suspend())), 'InvalidStateError', 'suspend');
		});

		nativeTest('getOutputTimestamp returns contextTime and performanceTime', async () => {
			const context: any = new AudioContext();
			try {
				const stamp = context.getOutputTimestamp();
				ok(stamp.contextTime >= 0 && stamp.contextTime <= context.currentTime + 1e-6, `contextTime ${stamp.contextTime}`);
				ok(stamp.performanceTime >= 0 && stamp.performanceTime <= (global.performance?.now?.() ?? Date.now()), `performanceTime ${stamp.performanceTime}`);
			} finally {
				await context.close();
			}
		});

		nativeTest('decodeAudioData failure is an EncodingError and reaches errorCallback', async () => {
			const context = offline();
			let callbackError: any = null;
			const error = await rejects(
				context.decodeAudioData(new ArrayBuffer(16), null, (e: any) => (callbackError = e)),
				'garbage decodes',
			);
			equal(errorName(error), 'EncodingError');
			ok(await waitFor(() => !!callbackError), 'errorCallback runs');
			equal(errorName(callbackError), 'EncodingError');
		});

		nativeTest('DelayNode maxDelayTime must be in (0, 180)', () => {
			const context = offline();
			throws(() => context.createDelay(0), 'NotSupportedError');
			throws(() => context.createDelay(180), 'NotSupportedError');
			equal(context.createDelay(10).maxDelayTime, 10);
		});

		nativeTest('IIRFilterNode validates its coefficients', () => {
			const context = offline();
			throws(() => context.createIIRFilter([], [1]), 'NotSupportedError', 'empty feedforward');
			throws(() => context.createIIRFilter(new Array(21).fill(1), [1]), 'NotSupportedError', '21 feedforward');
			throws(() => context.createIIRFilter([1], new Array(21).fill(1)), 'NotSupportedError', '21 feedback');
			throws(() => context.createIIRFilter([0, 0], [1]), 'InvalidStateError', 'all-zero feedforward');
			throws(() => context.createIIRFilter([1], [0, 1]), 'InvalidStateError', 'feedback[0] zero');
			const node = context.createIIRFilter([1], [1]);
			throws(() => node.getFrequencyResponse(new Float32Array(2), new Float32Array(1), new Float32Array(2)), 'InvalidAccessError');
		});

		nativeTest('AnalyserNode validates its attributes', () => {
			const analyser = offline().createAnalyser();
			throws(() => (analyser.fftSize = 1000), 'IndexSizeError', 'not a power of two');
			throws(() => (analyser.fftSize = 16), 'IndexSizeError', 'too small');
			throws(() => (analyser.fftSize = 65536), 'IndexSizeError', 'too large');
			analyser.fftSize = 32;
			equal(analyser.fftSize, 32);
			throws(() => (analyser.smoothingTimeConstant = 1.5), 'IndexSizeError');
			throws(() => (analyser.minDecibels = analyser.maxDecibels), 'IndexSizeError', 'min at max');
			throws(() => (analyser.maxDecibels = analyser.minDecibels - 1), 'IndexSizeError', 'max under min');
			throws(() => offline().createAnalyser({ minDecibels: -10, maxDecibels: -20 }), 'IndexSizeError', 'options');
		});

		nativeTest('PannerNode validates distance and cone attributes', () => {
			const context = offline();
			const panner = context.createPanner();
			throws(() => (panner.refDistance = -1), 'RangeError');
			throws(() => (panner.maxDistance = 0), 'RangeError');
			throws(() => (panner.rolloffFactor = -1), 'RangeError');
			throws(() => (panner.coneOuterGain = 1.5), 'InvalidStateError');
			throws(() => context.createPanner({ refDistance: -1 }), 'RangeError', 'options');
			panner.refDistance = 0;
			equal(panner.refDistance, 0);
		});

		nativeTest('PeriodicWave real and imag must match and hold two terms', () => {
			const context = offline();
			throws(() => context.createPeriodicWave([0, 1], [0, 1, 0]), 'IndexSizeError', 'mismatch');
			throws(() => context.createPeriodicWave([0], [0]), 'IndexSizeError', 'one term');
		});

		nativeTest('WaveShaperNode.curve copies and needs two points', () => {
			const shaper = offline().createWaveShaper();
			throws(() => (shaper.curve = new Float32Array(1)), 'InvalidStateError');
			const curve = new Float32Array([-1, 0, 1]);
			shaper.curve = curve;
			curve[0] = 5;
			equal(shaper.curve[0], -1, 'the setter copies');
			shaper.curve[1] = 5;
			equal(shaper.curve[1], 0, 'the getter copies');
		});

		nativeTest('ConvolverNode.buffer must match the context sampleRate', () => {
			const context = offline();
			const convolver = context.createConvolver();
			throws(() => (convolver.buffer = new AudioBuffer({ numberOfChannels: 1, length: 4, sampleRate: 44100 })), 'NotSupportedError');
			convolver.buffer = new AudioBuffer({ numberOfChannels: 1, length: 4, sampleRate: RATE });
		});

		nativeTest("OscillatorNode type 'custom' comes only from a PeriodicWave", () => {
			const context = offline();
			const wave = context.createPeriodicWave([0, 1], [0, 0]);
			const oscillator = context.createOscillator();
			throws(() => (oscillator.type = 'custom'), 'InvalidStateError');
			oscillator.type = 'bogus';
			equal(oscillator.type, 'sine', 'an unknown type is ignored');
			oscillator.setPeriodicWave(wave);
			equal(oscillator.type, 'custom');
			oscillator.type = 'square';
			equal(oscillator.type, 'square');
			equal(context.createOscillator({ type: 'custom', periodicWave: wave }).type, 'custom', 'options.periodicWave');
			throws(() => context.createOscillator({ type: 'custom' }), 'InvalidStateError', 'custom without a wave');
		});

		nativeTest('BiquadFilterNode.getFrequencyResponse follows the filter', () => {
			const context = offline();
			const filter = context.createBiquadFilter();
			const nyquist = RATE / 2;
			const freqs = new Float32Array([0, 350, 20000, -1, nyquist + 1]);
			const mag = new Float32Array(5);
			const phase = new Float32Array(5);
			filter.getFrequencyResponse(freqs, mag, phase);
			closeTo(mag[0], 1, 1e-5, 'lowpass passes DC');
			closeTo(phase[0], 0, 1e-5, 'with no phase shift');
			closeTo(mag[1], Math.pow(10, 1 / 20), 1e-3, 'the cutoff peaks by Q dB');
			ok(mag[2] < 0.01, `stops high frequencies (${mag[2]})`);
			ok(isNaN(mag[3]) && isNaN(phase[3]) && isNaN(mag[4]) && isNaN(phase[4]), 'outside [0, nyquist] is NaN');
			filter.type = 'peaking';
			filter.gain.value = 6;
			filter.getFrequencyResponse(freqs, mag, phase);
			closeTo(mag[1], Math.pow(10, 6 / 20), 1e-3, 'peaking gain at the centre');
			throws(() => filter.getFrequencyResponse(freqs, new Float32Array(4), phase), 'InvalidAccessError');
		});

		nativeTest('AudioParams report defaultValue, minValue and maxValue', () => {
			const context = offline();
			const range = (p: any) => [p.defaultValue, p.minValue, p.maxValue].join(',');
			const FLT = 3.4028234663852886e38;
			equal(range(context.createGain().gain), `1,${-FLT},${FLT}`, 'gain');
			const biquad = context.createBiquadFilter();
			equal(range(biquad.frequency), `350,0,${RATE / 2}`, 'biquad frequency');
			equal(biquad.Q.defaultValue, 1, 'biquad Q');
			equal(biquad.gain.defaultValue, 0, 'biquad gain');
			equal(biquad.detune.defaultValue, 0, 'biquad detune');
			equal(range(context.createDelay(2).delayTime), '0,0,2', 'delayTime');
			equal(range(context.createStereoPanner().pan), '0,-1,1', 'pan');
			equal(context.createConstantSource().offset.defaultValue, 1, 'offset');
			equal(range(context.createOscillator().frequency), `440,${-RATE / 2},${RATE / 2}`, 'oscillator frequency');
			equal(context.createPanner().orientationX.defaultValue, 1, 'orientationX');
			const compressor = context.createDynamicsCompressor();
			equal(range(compressor.threshold), '-24,-100,0', 'threshold');
			equal(range(compressor.knee), '30,0,40', 'knee');
			equal(range(compressor.ratio), '12,1,20', 'ratio');
			equal(range(compressor.attack), '0.003,0,1', 'attack');
			equal(range(compressor.release), '0.25,0,1', 'release');
		});

		nativeTest('fixed k-rate params keep their automationRate', () => {
			const context = offline();
			const threshold = context.createDynamicsCompressor().threshold;
			equal(threshold.automationRate, 'k-rate');
			throws(() => (threshold.automationRate = 'a-rate'), 'InvalidStateError');
			threshold.automationRate = 'k-rate';
			const gain = context.createGain().gain;
			gain.automationRate = 'bogus';
			equal(gain.automationRate, 'a-rate', 'an unknown rate is ignored');
			gain.automationRate = 'k-rate';
			equal(gain.automationRate, 'k-rate');
			const playbackRate = context.createBufferSource().playbackRate;
			if (playbackRate) equal(playbackRate.automationRate, 'k-rate', 'playbackRate');
		});

		nativeTest('DynamicsCompressorNode.reduction is a number', () => {
			const reduction = offline().createDynamicsCompressor().reduction;
			equal(typeof reduction, 'number');
			equal(reduction, 0, 'idle');
		});
	});
}
