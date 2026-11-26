import { AudioBuffer, AudioContext, ConvolverNode, OfflineAudioContext } from '@nativescript/audio-context';
import { suite, test, equal, closeTo, ok, throws } from './harness';

// Node behaviour the spec pins down: channel rules, the DSP nodes, automation and the context clock.

const RATE = 48000;
const Q = 128;

function constant(context: any, offset: number, when = 0) {
	const source = context.createConstantSource({ offset });
	source.start(when);
	return source;
}

function bufferOf(...channels: Float32Array[]) {
	const out = new AudioBuffer({ numberOfChannels: channels.length, length: channels[0].length, sampleRate: RATE });
	channels.forEach((data, c) => out.copyToChannel(data, c));
	return out;
}

/** A source playing `channels` (one value each) for the whole render. */
function stereoSource(context: any, values: number[], length: number) {
	const source = context.createBufferSource();
	source.buffer = bufferOf(...values.map((v) => new Float32Array(length).fill(v)));
	source.start(0);
	return source;
}

async function until(context: AudioContext, time: number) {
	const deadline = Date.now() + 5000;
	while (context.currentTime < time) {
		if (Date.now() > deadline) throw new Error(`currentTime stalled at ${context.currentTime}`);
		await new Promise((resolve) => setTimeout(resolve, 5));
	}
}

export function registerAudioNodesSpec() {
	suite('audio.nodes.context', () => {
		test('a realtime context runs from construction, its clock from 0', async () => {
			const context = new AudioContext();
			const first = context.currentTime;
			await until(context, 0.1);
			const rate = context.sampleRate;
			await context.close();
			ok(first < 0.05, `starts near 0, at ${first}`);
			ok(rate >= 8000, `sampleRate ${rate} still readable after close`);
			ok(context.currentTime >= 0.1, `currentTime ${context.currentTime} after close`);
		});

		test('honours a sampleRate option and rejects one out of range', async () => {
			const context = new AudioContext({ sampleRate: 44100 });
			const rate = context.sampleRate;
			await context.close();
			equal(rate, 44100, 'sampleRate');
			throws(() => new AudioContext({ sampleRate: 1000 }), 'NotSupportedError');
		});

		test('getOutputTimestamp trails currentTime', async () => {
			const context = new AudioContext();
			await until(context, 0.1);
			const stamp = (context as any).getOutputTimestamp();
			const now = context.currentTime;
			await context.close();
			ok(stamp.contextTime <= now && stamp.contextTime >= 0, `contextTime ${stamp.contextTime}, currentTime ${now}`);
			ok(stamp.performanceTime > 0, `performanceTime ${stamp.performanceTime}`);
		});

		test('MediaStream nodes throw NotSupportedError', async () => {
			const context: any = new AudioContext();
			throws(() => context.createMediaStreamDestination(), 'NotSupportedError');
			await context.close();
		});

		test('a DelayNode in a realtime cycle delays by one 128-frame quantum at least', async () => {
			const context = new AudioContext();
			const analyser = context.createAnalyser({ fftSize: 32768 });
			analyser.connect(context.destination);
			const delay = context.createDelay(1);
			delay.delayTime.value = 0;
			const loop = context.createGain();
			loop.gain.value = 0.5;
			const impulse = new Float32Array(Q * 4);
			impulse[0] = 1;
			const source: any = context.createBufferSource();
			source.buffer = context.createBuffer({ numberOfChannels: 1, length: impulse.length, sampleRate: context.sampleRate });
			source.buffer.copyToChannel(impulse, 0);
			source.connect(delay);
			delay.connect(loop);
			loop.connect(delay);
			delay.connect(analyser);
			source.start();
			await until(context, context.currentTime + 0.3);
			const data = new Float32Array(32768);
			analyser.getFloatTimeDomainData(data);
			await context.close();
			const peaks: number[] = [];
			for (let i = 0; i < data.length; i++) if (Math.abs(data[i]) > 1e-3) peaks.push(i);
			ok(peaks.length >= 3, `echoes ${peaks.length}`);
			for (let k = 1; k < 3; k++) equal(peaks[k] - peaks[k - 1], Q, `echo ${k} spacing`);
			closeTo(data[peaks[1]] / data[peaks[0]], 0.5, 1e-4, 'decay per pass');
		});

		test('a split DelayNode reads its input directly again once the cycle is gone', async () => {
			const context = new OfflineAudioContext(1, Q * 4, RATE);
			const delay = context.createDelay(1);
			delay.delayTime.value = 0;
			const loop = context.createGain();
			constant(context, 1).connect(delay);
			delay.connect(loop);
			loop.connect(delay);
			loop.disconnect();
			delay.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[0], 1, 'no quantum of delay on the first frame');
		});
	});

	suite('audio.nodes.channels', () => {
		test('channel rules: panners, convolver and compressor at most 2, never max', () => {
			const context = new OfflineAudioContext(2, Q, RATE);
			for (const node of [context.createPanner(), context.createStereoPanner(), context.createConvolver(), context.createDynamicsCompressor()] as any[]) {
				equal(node.channelCountMode, 'clamped-max', `${node.constructor.name} default mode`);
				throws(() => (node.channelCount = 3), 'NotSupportedError', `${node.constructor.name} channelCount 3`);
				throws(() => (node.channelCountMode = 'max'), 'NotSupportedError', `${node.constructor.name} max`);
				node.channelCount = 1;
				node.channelCountMode = 'explicit';
			}
		});

		test('channel rules: splitter and merger are fixed', () => {
			const context = new OfflineAudioContext(2, Q, RATE);
			const splitter: any = context.createChannelSplitter({ numberOfOutputs: 4 });
			equal(splitter.channelCount, 4, 'splitter channelCount');
			throws(() => (splitter.channelCount = 2), 'InvalidStateError');
			throws(() => (splitter.channelCountMode = 'max'), 'InvalidStateError');
			throws(() => (splitter.channelInterpretation = 'speakers'), 'InvalidStateError');
			const merger: any = context.createChannelMerger({ numberOfInputs: 2 });
			throws(() => (merger.channelCount = 2), 'InvalidStateError');
			throws(() => (merger.channelCountMode = 'max'), 'InvalidStateError');
			equal(merger.numberOfInputs, 2, 'merger inputs');
			equal(splitter.numberOfOutputs, 4, 'splitter outputs');
		});

		for (const kind of ['gain', 'delay', 'biquad', 'waveshaper', 'iir', 'stereo', 'analyser']) {
			test(`${kind}: channelCount 1 explicit down-mixes a stereo input`, async () => {
				const context = new OfflineAudioContext(2, Q * 2, RATE);
				const c: any = context;
				const node: any =
					kind === 'gain'
						? c.createGain()
						: kind === 'delay'
							? c.createDelay()
							: kind === 'biquad'
								? c.createBiquadFilter({ type: 'allpass', frequency: RATE / 2 })
								: kind === 'waveshaper'
									? c.createWaveShaper()
									: kind === 'iir'
										? c.createIIRFilter([1], [1])
										: kind === 'stereo'
											? c.createStereoPanner()
											: c.createAnalyser();
				node.channelCount = 1;
				node.channelCountMode = 'explicit';
				stereoSource(context, [1, 0.5], Q * 2).connect(node);
				node.connect(context.destination);
				const out = await context.startRendering();
				const l = out.getChannelData(0)[Q];
				const r = out.getChannelData(1)[Q];
				// Mono 0.75 up-mixed; the stereo panner pans that mono centre: 0.75 cos(pi/4) each side.
				const want = kind === 'stereo' ? 0.75 * Math.SQRT1_2 : 0.75;
				closeTo(l, want, 1e-5, 'left');
				closeTo(r, want, 1e-5, 'right');
			});
		}

		test('a stereo source with equal channels pans as stereo, not as mono', async () => {
			const context = new OfflineAudioContext(2, Q, RATE);
			const panner = context.createPanner({ positionX: 1, positionZ: -1 } as any);
			stereoSource(context, [1, 1], Q).connect(panner);
			panner.connect(context.destination);
			const out = await context.startRendering();
			// Azimuth 45: L = 1 cos(pi/4), R = 1 + 1 sin(pi/4), times 1/sqrt(2) for distance.
			closeTo(out.getChannelData(0)[10], 0.5, 1e-5, 'left');
			closeTo(out.getChannelData(1)[10], (1 + Math.SQRT1_2) * Math.SQRT1_2, 1e-5, 'right');
		});

		test('a mono source pans as mono', async () => {
			const context = new OfflineAudioContext(2, Q, RATE);
			const panner = context.createPanner({ positionX: 1, positionZ: -1 } as any);
			constant(context, 1).connect(panner);
			panner.connect(context.destination);
			const out = await context.startRendering();
			closeTo(out.getChannelData(0)[10], 0.2706, 1e-4, 'left');
			closeTo(out.getChannelData(1)[10], 0.6533, 1e-4, 'right');
		});
	});

	suite('audio.nodes.dsp', () => {
		test('HRTF puts a source on the right in the right ear, on the left in the left', async () => {
			const render = async (x: number) => {
				const context = new OfflineAudioContext(2, Q * 16, RATE);
				const panner = context.createPanner({ panningModel: 'HRTF', positionX: x, positionZ: 0 } as any);
				const noise = new Float32Array(Q * 16).map((_, i) => Math.sin(i * 0.37) * 0.5 + Math.sin(i * 1.3) * 0.5);
				const source: any = context.createBufferSource();
				source.buffer = bufferOf(noise);
				source.start(0);
				source.connect(panner);
				panner.connect(context.destination);
				const out = await context.startRendering();
				const energy = (c: number) => out.getChannelData(c).reduce((sum, v) => sum + v * v, 0);
				return [energy(0), energy(1)];
			};
			const [rl, rr] = await render(1);
			const [ll, lr] = await render(-1);
			ok(rr > rl * 2, `right source: left ${rl}, right ${rr}`);
			ok(ll > lr * 2, `left source: left ${ll}, right ${lr}`);
			closeTo(rl / rr, lr / ll, 0.05, 'mirror images');
		});

		test('ConvolverOptions.buffer and disableNormalization apply in the constructor', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const response = bufferOf(Float32Array.from({ length: 10 }, (_, i) => (i === 3 ? 0.5 : 0)));
			const convolver = new ConvolverNode(context, { buffer: response, disableNormalization: true });
			equal(convolver.normalize, false, 'normalize');
			ok(convolver.buffer != null, 'buffer');
			const impulse = new Float32Array(Q * 2);
			impulse[0] = 1;
			const source: any = context.createBufferSource();
			source.buffer = bufferOf(impulse);
			source.start(0);
			source.connect(convolver);
			convolver.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			closeTo(samples[3], 0.5, 1e-6, 'the response, unscaled');
			throws(() => (convolver.buffer = new AudioBuffer({ numberOfChannels: 3, length: 4, sampleRate: RATE })), 'NotSupportedError');
		});

		test('IIRFilterNode filters with its coefficients', async () => {
			const context = new OfflineAudioContext(1, Q, RATE);
			const filter = context.createIIRFilter([0.5, 0.5], [1, -0.5]);
			const impulse = new Float32Array(Q);
			impulse[0] = 1;
			const source: any = context.createBufferSource();
			source.buffer = bufferOf(impulse);
			source.start(0);
			source.connect(filter);
			filter.connect(context.destination);
			const y = (await context.startRendering()).getChannelData(0);
			// y[n] = 0.5 x[n] + 0.5 x[n-1] + 0.5 y[n-1]
			closeTo(y[0], 0.5, 1e-6, 'y0');
			closeTo(y[1], 0.75, 1e-6, 'y1');
			closeTo(y[2], 0.375, 1e-6, 'y2');
		});

		test('WaveShaperNode maps its input through the curve', async () => {
			const context = new OfflineAudioContext(1, Q, RATE);
			const shaper = context.createWaveShaper({ curve: [0, 0.5, 1] } as any);
			constant(context, 0.5).connect(shaper);
			shaper.connect(context.destination);
			const y = (await context.startRendering()).getChannelData(0);
			closeTo(y[0], 0.75, 1e-6, 'curve at 0.5');
			throws(() => ((shaper as any).curve = new Float32Array(1)), 'InvalidStateError');
		});

		test('StereoPannerNode pans a mono input fully left', async () => {
			const context = new OfflineAudioContext(2, Q, RATE);
			const pan = context.createStereoPanner({ pan: -1 });
			constant(context, 1).connect(pan);
			pan.connect(context.destination);
			const out = await context.startRendering();
			closeTo(out.getChannelData(0)[0], 1, 1e-6, 'left');
			closeTo(out.getChannelData(1)[0], 0, 1e-6, 'right');
		});

		test('AnalyserNode reports a bin-centred sine at |X|/N with the Blackman window', async () => {
			const context = new OfflineAudioContext(1, Q * 64, RATE);
			const analyser = context.createAnalyser({ fftSize: 2048, smoothingTimeConstant: 0 });
			const oscillator = context.createOscillator({ frequency: (64 * RATE) / 2048 });
			oscillator.connect(analyser);
			analyser.connect(context.destination);
			oscillator.start(0);
			const bins = new Float32Array(1024);
			context.suspend((Q * 32) / RATE).then(() => {
				analyser.getFloatFrequencyData(bins);
				context.resume();
			});
			await context.startRendering();
			closeTo(bins[64], 20 * Math.log10(0.21), 0.1, 'peak bin dB');
			throws(() => (analyser.fftSize = 100), 'IndexSizeError');
		});

		test('a custom PeriodicWave plays its harmonics', async () => {
			const context = new OfflineAudioContext(1, Q * 4, RATE);
			const wave = context.createPeriodicWave(new Float32Array(2), Float32Array.from([0, 1]), { disableNormalization: true });
			const oscillator = context.createOscillator({ frequency: 375 });
			oscillator.setPeriodicWave(wave);
			oscillator.connect(context.destination);
			oscillator.start(0);
			const y = (await context.startRendering()).getChannelData(0);
			// 375 Hz is 128 frames a cycle: a quarter in, the sine peaks.
			closeTo(y[32], 1, 1e-3, 'sin at a quarter cycle');
			equal(oscillator.type, 'custom', 'type');
		});

		test('AudioBufferSourceNode loops between loopStart and loopEnd', async () => {
			const context = new OfflineAudioContext(1, 64, RATE);
			const source: any = context.createBufferSource();
			source.buffer = bufferOf(Float32Array.from({ length: 16 }, (_, i) => i));
			source.loop = true;
			source.loopStart = 4 / RATE;
			source.loopEnd = 8 / RATE;
			source.start(0);
			source.connect(context.destination);
			const y = (await context.startRendering()).getChannelData(0);
			equal(y[7], 7, 'plays to the loop end');
			equal(y[8], 4, 'then from loopStart');
			equal(y[12], 4, 'again');
		});
	});

	suite('audio.nodes.params', () => {
		test('exponentialRampToValueAtTime ramps exponentially', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const source = constant(context, 1);
			source.offset.setValueAtTime(1, 0);
			source.offset.exponentialRampToValueAtTime(4, Q / RATE);
			source.connect(context.destination);
			const y = (await context.startRendering()).getChannelData(0);
			closeTo(y[Q / 2], 2, 1e-4, 'geometric midpoint');
			closeTo(y[Q], 4, 1e-4, 'the target from its time');
		});

		test('setTargetAtTime approaches its target with the time constant', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const source = constant(context, 0);
			source.offset.setTargetAtTime(1, 0, Q / RATE);
			source.connect(context.destination);
			const y = (await context.startRendering()).getChannelData(0);
			closeTo(y[Q], 1 - Math.exp(-1), 1e-4, 'one time constant in');
		});

		test('setValueCurveAtTime interpolates the curve', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const source = constant(context, 0);
			source.offset.setValueCurveAtTime([0, 1, 0], 0, Q / RATE);
			source.connect(context.destination);
			const y = (await context.startRendering()).getChannelData(0);
			closeTo(y[Q / 2], 1, 1e-4, 'middle of the curve');
			closeTo(y[Q / 4], 0.5, 1e-4, 'between points');
			equal(y[Q + 10], 0, 'holds the last value');
		});

		test('AudioParam methods reject negative times; start twice is an error', () => {
			const context = new OfflineAudioContext(1, Q, RATE);
			const source = context.createConstantSource();
			throws(() => source.offset.setValueAtTime(1, -1), 'RangeError');
			throws(() => source.offset.exponentialRampToValueAtTime(0, 1), 'RangeError');
			throws(() => source.stop(), 'InvalidStateError');
			source.start(0);
			throws(() => source.start(0), 'InvalidStateError');
		});
	});
}
