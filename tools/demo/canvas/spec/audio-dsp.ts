import { AudioBuffer, AudioContext, OfflineAudioContext } from '@nativescript/audio-context';
import { suite, test, skip, ok, equal, closeTo, TestBody } from './harness';

// Node processing against the spec's algorithms and what Chrome renders.

declare const global: any;

const RATE = 48000;
const Q = 128;

const nativeTest = (name: string, body: TestBody) => (global.isAndroid || global.isIOS ? test(name, body) : skip(name, 'not yet on this platform'));

const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

function buffer(rate: number, ...channels: Float32Array[]) {
	const out = new AudioBuffer({ numberOfChannels: channels.length, length: channels[0].length, sampleRate: rate });
	channels.forEach((data, c) => out.copyToChannel(data, c));
	return out;
}

function play(context: any, data: Float32Array) {
	const source = context.createBufferSource();
	source.buffer = buffer(context.sampleRate, data);
	source.start(0);
	return source;
}

function energy(data: Float32Array, from = 0, to = data.length) {
	let sum = 0;
	for (let i = from; i < to; i++) sum += data[i] * data[i];
	return sum;
}

function onset(data: Float32Array, threshold: number) {
	return data.findIndex((v) => Math.abs(v) > threshold);
}

export function registerAudioDspSpec() {
	suite('audio.dsp.convolver', () => {
		nativeTest('a response longer than 32768 frames convolves exactly', async () => {
			const length = 50000;
			const response = Float32Array.from({ length }, (_, i) => Math.sin(i * 0.013) * Math.exp(-i / 20000));
			const impulses: [number, number][] = [
				[3, 1],
				[1000, -0.5],
				[20000, 0.25],
			];
			const frames = 80000;
			const input = new Float32Array(frames);
			for (const [at, value] of impulses) input[at] = value;
			const context = new OfflineAudioContext(1, frames, RATE);
			const convolver = context.createConvolver();
			convolver.normalize = false;
			convolver.buffer = buffer(RATE, response);
			play(context, input).connect(convolver);
			convolver.connect(context.destination);
			const got = (await context.startRendering()).getChannelData(0);
			let diff = 0;
			let at = 0;
			for (let n = 0; n < frames; n++) {
				let want = 0;
				for (const [t, value] of impulses) if (n >= t && n - t < length) want += value * response[n - t];
				const d = Math.abs(got[n] - want);
				if (d > diff) [diff, at] = [d, n];
			}
			ok(diff < 1e-4, `frame ${at} differs by ${diff}`);
		});

		nativeTest('a realtime convolver plays the far end of a long response on time', async () => {
			const context = new AudioContext();
			const rate = context.sampleRate;
			const response = new Float32Array(Math.round(rate * 1.2));
			response[Math.round(rate * 0.8)] = 1;
			const convolver = context.createConvolver();
			convolver.normalize = false;
			convolver.buffer = buffer(rate, response);
			const analyser = context.createAnalyser({ fftSize: 2048 });
			const mute = context.createGain({ gain: 0 });
			convolver.connect(analyser).connect(mute).connect(context.destination);
			const source = context.createConstantSource({ offset: 0.5 });
			source.connect(convolver);
			const start = context.currentTime + 0.1;
			source.start(start);
			source.stop(start + 0.1);
			const samples = new Float32Array(2048);
			let peak = 0;
			let early = 0;
			let heardAt = -1;
			const until = Date.now() + 2000;
			while (Date.now() < until) {
				analyser.getFloatTimeDomainData(samples);
				const level = samples.reduce((m, v) => Math.max(m, Math.abs(v)), 0);
				const elapsed = context.currentTime - start;
				if (elapsed < 0.6) early = Math.max(early, level);
				if (level > 0.25 && heardAt < 0) heardAt = elapsed;
				peak = Math.max(peak, level);
				await delay(15);
			}
			await context.close();
			ok(early < 0.01, `early output ${early}`);
			closeTo(peak, 0.5, 0.01, 'peak');
			ok(heardAt > 0.7 && heardAt < 1.1, `heard ${heardAt}s after the input`);
		});
	});

	suite('audio.dsp.panner', () => {
		// An impulse through an HRTF panner at (x, y, z): [left, right].
		async function hrtf(x: number, y: number, z: number, frames = Q * 4) {
			const context = new OfflineAudioContext(2, frames, RATE);
			const panner = context.createPanner({ panningModel: 'HRTF', positionX: x, positionY: y, positionZ: z });
			const input = new Float32Array(1);
			input[0] = 1;
			play(context, input).connect(panner);
			panner.connect(context.destination);
			const rendered = await context.startRendering();
			return [rendered.getChannelData(0), rendered.getChannelData(1)];
		}

		nativeTest('HRTF: a source on the right reaches the right ear first and louder', async () => {
			const [left, right] = await hrtf(1, 0, 0);
			ok(energy(right) > 4 * energy(left), `right ${energy(right)}, left ${energy(left)}`);
			const l = onset(left, 0.02);
			const r = onset(right, 0.02);
			ok(r >= 0 && l - r > 10, `onsets: left ${l}, right ${r}`);
		});

		nativeTest('HRTF: left and right mirror each other', async () => {
			const [rightL, rightR] = await hrtf(1, 0, -0.5);
			const [leftL, leftR] = await hrtf(-1, 0, -0.5);
			let diff = 0;
			for (let i = 0; i < rightL.length; i++) diff = Math.max(diff, Math.abs(rightL[i] - leftR[i]), Math.abs(rightR[i] - leftL[i]));
			ok(diff < 1e-6 && energy(rightR) > 0, `differs by ${diff}`);
		});

		nativeTest('HRTF: a source straight ahead reaches both ears alike', async () => {
			const [left, right] = await hrtf(0, 0, -1);
			let diff = 0;
			for (let i = 0; i < left.length; i++) diff = Math.max(diff, Math.abs(left[i] - right[i]));
			ok(diff < 1e-6 && energy(left) > 0.1, `differs by ${diff}, energy ${energy(left)}`);
		});

		nativeTest('HRTF: elevation changes the response', async () => {
			const [levelL] = await hrtf(0, 0, -1);
			const [aboveL] = await hrtf(0, 1, -0.2);
			let diff = 0;
			for (let i = 0; i < levelL.length; i++) diff = Math.max(diff, Math.abs(levelL[i] - aboveL[i]));
			ok(diff > 0.01, `differs by ${diff}`);
		});

		nativeTest('HRTF: the response rings on after a one-frame input', async () => {
			const [left, right] = await hrtf(1, 0, 0);
			ok(energy(right, Q, 2 * Q) + energy(left, Q, 2 * Q) > 0, 'second quantum');
			ok(energy(right, 3 * Q) + energy(left, 3 * Q) === 0, 'silent after the tail');
		});

		nativeTest('StereoPannerNode pans by pan alone, wherever the listener is', async () => {
			const context: any = new OfflineAudioContext(2, Q * 2, RATE);
			context.listener.positionX.value = 5;
			context.listener.positionZ.value = 3;
			const panner = context.createStereoPanner({ pan: 0.5 });
			const source = context.createConstantSource({ offset: 1 });
			source.connect(panner);
			panner.connect(context.destination);
			source.start(0);
			const rendered = await context.startRendering();
			const x = (0.5 + 1) / 2;
			closeTo(rendered.getChannelData(0)[Q], Math.cos((x * Math.PI) / 2), 1e-6, 'left');
			closeTo(rendered.getChannelData(1)[Q], Math.sin((x * Math.PI) / 2), 1e-6, 'right');
		});
	});

	suite('audio.dsp.automation', () => {
		// A constant 1 through a gain whose param `schedule` automates, one channel.
		async function automate(schedule: (gain: any) => void, frames = Q * 8) {
			const context = new OfflineAudioContext(1, frames, RATE);
			const gain = context.createGain();
			schedule(gain.gain);
			const source = context.createConstantSource({ offset: 1 });
			source.connect(gain);
			gain.connect(context.destination);
			source.start(0);
			return (await context.startRendering()).getChannelData(0);
		}

		nativeTest('exponentialRampToValueAtTime follows the exponential curve', async () => {
			const end = (Q * 4) / RATE;
			const got = await automate((g) => {
				g.setValueAtTime(0.01, 0);
				g.exponentialRampToValueAtTime(1, end);
			});
			for (const frame of [0, Q, 300, Q * 4 - 1]) {
				const want = 0.01 * Math.pow(100, frame / (Q * 4));
				closeTo(got[frame], want, want * 1e-4, `frame ${frame}`);
			}
			closeTo(got[Q * 5], 1, 1e-6, 'after the ramp');
		});

		nativeTest('setTargetAtTime approaches its target exponentially', async () => {
			const start = Q / RATE;
			const tau = 0.002;
			const got = await automate((g) => {
				g.setValueAtTime(1, 0);
				g.setTargetAtTime(0, start, tau);
			});
			closeTo(got[Q - 1], 1, 1e-6, 'before');
			for (const frame of [Q, Q + 96, Q * 3]) {
				const want = Math.exp(-(frame - Q) / RATE / tau);
				closeTo(got[frame], want, 1e-4, `frame ${frame}`);
			}
		});

		nativeTest('setValueCurveAtTime interpolates its values over the duration', async () => {
			const curve = new Float32Array([0, 1, 0.5]);
			const duration = (Q * 2) / RATE;
			const got = await automate((g) => g.setValueCurveAtTime(curve, Q / RATE, duration));
			closeTo(got[Q], 0, 1e-4, 'start');
			closeTo(got[Q + 64], 0.5, 1e-4, 'quarter');
			closeTo(got[Q * 2], 1, 1e-4, 'middle');
			closeTo(got[Q * 2 + 64], 0.75, 1e-4, 'three quarters');
			closeTo(got[Q * 4], 0.5, 1e-6, 'held after');
		});

		nativeTest('a ramp with no event before it starts from the current value', async () => {
			const got = await automate((g) => {
				g.value = 1;
				g.linearRampToValueAtTime(0, (Q * 4) / RATE);
			});
			closeTo(got[Q * 2], 0.5, 1e-4, 'halfway');
		});
	});

	suite('audio.dsp.sources', () => {
		// Zero crossings going up in `data`: its frequency in cycles per frame.
		function cycles(data: Float32Array) {
			let count = 0;
			for (let i = 1; i < data.length; i++) if (data[i - 1] < 0 && data[i] >= 0) count++;
			return count;
		}

		nativeTest('OscillatorNode.detune shifts the frequency by cents', async () => {
			const context = new OfflineAudioContext(1, RATE / 10, RATE);
			const oscillator: any = context.createOscillator({ frequency: 400 });
			oscillator.detune.value = 1200;
			oscillator.connect(context.destination);
			oscillator.start(0);
			const data = (await context.startRendering()).getChannelData(0);
			const count = cycles(data);
			ok(Math.abs(count - 80) <= 1, `${count} cycles in 0.1s`);
		});

		nativeTest('AudioBufferSourceNode.detune scales the playback rate', async () => {
			const context = new OfflineAudioContext(1, Q * 4, RATE);
			const ramp = Float32Array.from({ length: Q * 8 }, (_, i) => i / (Q * 8));
			const source: any = context.createBufferSource();
			source.buffer = buffer(RATE, ramp);
			source.detune.value = 1200;
			source.connect(context.destination);
			source.start(0);
			const data = (await context.startRendering()).getChannelData(0);
			closeTo(data[100], ramp[200], 1e-6, 'twice as fast');
		});

		nativeTest('AudioBufferSourceNode loops between loopStart and loopEnd', async () => {
			const context = new OfflineAudioContext(1, Q * 4, RATE);
			const data = Float32Array.from({ length: Q }, (_, i) => i);
			const source: any = context.createBufferSource();
			source.buffer = buffer(RATE, data);
			source.loop = true;
			source.loopStart = 10 / RATE;
			source.loopEnd = 20 / RATE;
			source.connect(context.destination);
			source.start(0);
			const got = (await context.startRendering()).getChannelData(0);
			const want = (frame: number) => (frame < 20 ? frame : 10 + ((frame - 20) % 10));
			let bad = -1;
			for (let i = 0; i < 200; i++) if (got[i] !== want(i)) bad = bad < 0 ? i : bad;
			ok(bad < 0, `frame ${bad}: ${got[bad]}, want ${want(bad)}`);
		});

		nativeTest('a custom PeriodicWave is normalized to a peak of 1 unless disabled', async () => {
			const peak = async (disableNormalization: boolean) => {
				const context = new OfflineAudioContext(1, RATE / 10, RATE);
				const wave = context.createPeriodicWave(new Float32Array([5, 0, 0]), new Float32Array([0, 3, 1]), { disableNormalization });
				const oscillator = context.createOscillator({ frequency: 100 });
				oscillator.setPeriodicWave(wave);
				oscillator.connect(context.destination);
				oscillator.start(0);
				const data = (await context.startRendering()).getChannelData(0);
				return data.reduce((m, v) => Math.max(m, Math.abs(v)), 0);
			};
			// 3 sin x + sin 2x peaks at 3.485; real[0], the DC term, is ignored.
			closeTo(await peak(true), 3.485, 0.005, 'unnormalized');
			closeTo(await peak(false), 1, 1e-3, 'normalized');
		});
	});

	suite('audio.dsp.nodes', () => {
		nativeTest("WaveShaperNode oversample '2x' and '4x' shape at a higher rate", async () => {
			const render = async (oversample: string) => {
				const context = new OfflineAudioContext(1, Q * 16, RATE);
				const shaper = context.createWaveShaper();
				shaper.curve = new Float32Array([-1, 1]);
				shaper.oversample = oversample as any;
				const oscillator = context.createOscillator({ frequency: 1000 });
				oscillator.connect(shaper);
				shaper.connect(context.destination);
				oscillator.start(0);
				return (await context.startRendering()).getChannelData(0);
			};
			for (const factor of ['2x', '4x']) {
				const data = await render(factor);
				// An identity curve: the same sine, late by the filters' delay (a fraction of a frame for 4x).
				let lag = 0;
				let best = Infinity;
				for (let d = 0; d < 80; d += 0.25) {
					let err = 0;
					for (let i = Q * 4; i < Q * 12; i++) err = Math.max(err, Math.abs(data[i] - Math.sin((2 * Math.PI * 1000 * (i - d)) / RATE)));
					if (err < best) [best, lag] = [err, d];
				}
				ok(best < 0.01 && lag > 0, `${factor}: off by ${best} at a lag of ${lag}`);
			}
		});

		nativeTest('DynamicsCompressorNode.reduction meters the gain reduction', async () => {
			const context = new OfflineAudioContext(1, RATE / 4, RATE);
			const compressor: any = context.createDynamicsCompressor();
			const source = context.createConstantSource({ offset: 1 });
			source.connect(compressor);
			compressor.connect(context.destination);
			source.start(0);
			equal(compressor.reduction, 0, 'before rendering');
			await context.startRendering();
			ok(compressor.reduction < -10, `reduction ${compressor.reduction}`);
		});

		nativeTest('DynamicsCompressorNode looks 6 ms ahead and applies makeup gain', async () => {
			const context = new OfflineAudioContext(1, Q * 8, RATE);
			const compressor = context.createDynamicsCompressor();
			const source = context.createConstantSource({ offset: 0.01 });
			source.connect(compressor);
			compressor.connect(context.destination);
			source.start(0);
			const data = (await context.startRendering()).getChannelData(0);
			const lookahead = Math.round(0.006 * RATE);
			equal(data[lookahead - 1], 0, 'silent before the look-ahead');
			// Below the knee nothing is reduced; makeup is 0.6 of the reduction at 0 dB (22 dB by default).
			closeTo(data[lookahead + Q], 0.01 * Math.pow(10, (0.6 * 22) / 20), 1e-5, 'made up');
		});

		nativeTest('AnalyserNode windows with Blackman and converts bytes as the spec says', async () => {
			const context = new OfflineAudioContext(1, 4096, RATE);
			const analyser = context.createAnalyser({ fftSize: 1024, smoothingTimeConstant: 0 });
			// Bin 32 exactly, so the window's gain alone sets the peak.
			const oscillator = context.createOscillator({ frequency: (32 * RATE) / 1024 });
			oscillator.connect(analyser);
			analyser.connect(context.destination);
			oscillator.start(0);
			context.suspend(2048 / RATE).then(() => {
				const spectrum = new Float32Array(512);
				analyser.getFloatFrequencyData(spectrum);
				peakDb = spectrum[32];
				const bytes = new Uint8Array(1024);
				analyser.getByteTimeDomainData(bytes);
				byteAt0 = bytes[0];
				context.resume();
			});
			let peakDb = NaN;
			let byteAt0 = -1;
			await context.startRendering();
			// A unit sine's bin: 0.5 times the Blackman window's mean, 0.42.
			closeTo(peakDb, 20 * Math.log10(0.21), 0.05, 'peak dB');
			ok(byteAt0 >= 0 && byteAt0 <= 255, `byte ${byteAt0}`);
			const silent = new OfflineAudioContext(1, Q, RATE);
			const idle = silent.createAnalyser();
			idle.connect(silent.destination);
			await silent.startRendering();
			const bytes = new Uint8Array(32);
			idle.getByteTimeDomainData(bytes);
			equal(bytes[0], 128, 'silence is 128');
		});
	});
}
