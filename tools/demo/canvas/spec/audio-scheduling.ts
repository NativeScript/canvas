import { AudioContext } from '@nativescript/audio-context';
import { isAndroid } from '@nativescript/core';
import { suite, test, skip, ok } from './harness';
import { wavBase64 } from './audio';

// start(when, offset) / stop(when) against a running context, read back through an analyser.
// Windows sit 150 ms or more from every edge, so output latency does not decide a result.

const FFT = 512;

function rmsOf(analyser: any): number {
	const data = new Float32Array(FFT);
	analyser.getFloatTimeDomainData(data);
	let sum = 0;
	for (let i = 0; i < data.length; i++) sum += data[i] * data[i];
	return Math.sqrt(sum / data.length);
}

async function until(context: AudioContext, time: number) {
	const deadline = Date.now() + 5000;
	while (context.currentTime < time) {
		if (Date.now() > deadline) throw new Error(`currentTime stalled at ${context.currentTime}, waiting for ${time}`);
		await new Promise((resolve) => setTimeout(resolve, 5));
	}
}

async function runningContext() {
	const context = new AudioContext();
	const analyser = context.createAnalyser({ fftSize: FFT });
	analyser.connect(context.destination);
	// Keeps the engine pulling before anything under test starts.
	const idle = context.createConstantSource({ offset: 0 });
	idle.connect(analyser);
	idle.start();
	const deadline = Date.now() + 5000;
	while (context.currentTime <= 0.05) {
		if (Date.now() > deadline) throw new Error('context clock never started');
		await new Promise((resolve) => setTimeout(resolve, 10));
	}
	return { context, analyser };
}

/** Full-scale square: rms 1 wherever it is audible, 0 where `silent(i)` holds. */
function squareBuffer(context: AudioContext, seconds: number, silent: (i: number, length: number) => boolean = () => false) {
	const sampleRate = context.sampleRate;
	const length = Math.round(seconds * sampleRate);
	const buffer = context.createBuffer({ numberOfChannels: 1, length, sampleRate });
	const samples = new Float32Array(length);
	for (let i = 0; i < length; i++) samples[i] = silent(i, length) ? 0 : Math.floor(i / 50) % 2 === 0 ? 1 : -1;
	buffer.copyToChannel(samples, 0);
	return buffer;
}

const quiet = (rms: number) => rms < 0.02;
const loud = (rms: number) => rms > 0.5;

export function registerAudioSchedulingSpec() {
	suite('audio.scheduling', () => {
		test('buffer source sounds only between start(when) and stop(when)', async () => {
			const { context, analyser } = await runningContext();
			const source: any = context.createBufferSource();
			source.buffer = squareBuffer(context, 2);
			source.connect(analyser);
			let ended = 0;
			source.onended = () => ended++;
			const t0 = context.currentTime;
			source.start(t0 + 0.3);
			source.stop(t0 + 0.6);
			await until(context, t0 + 0.15);
			const before = rmsOf(analyser);
			await until(context, t0 + 0.45);
			const during = rmsOf(analyser);
			const endedEarly = ended;
			await until(context, t0 + 0.85);
			const after = rmsOf(analyser);
			await context.close();
			ok(quiet(before), `silent before start, rms ${before}`);
			ok(loud(during), `audible inside the window, rms ${during}`);
			ok(quiet(after), `silent after stop, rms ${after}`);
			ok(endedEarly === 0 && ended === 1, `ended once, after stop (before ${endedEarly}, total ${ended})`);
		});

		test('looping buffer source stops at stop(when)', async () => {
			const { context, analyser } = await runningContext();
			const source: any = context.createBufferSource();
			source.buffer = squareBuffer(context, 0.1);
			source.loop = true;
			source.connect(analyser);
			let ended = 0;
			source.onended = () => ended++;
			const t0 = context.currentTime;
			source.start();
			source.stop(t0 + 0.4);
			await until(context, t0 + 0.25);
			const during = rmsOf(analyser);
			await until(context, t0 + 0.7);
			const after = rmsOf(analyser);
			await context.close();
			ok(loud(during), `audible while looping, rms ${during}`);
			ok(quiet(after), `silent after stop, rms ${after}`);
			ok(ended === 1, `ended events ${ended}`);
		});

		test('buffer source start offset skips into the buffer', async () => {
			const { context, analyser } = await runningContext();
			const source: any = context.createBufferSource();
			// Silent first half, square second half.
			source.buffer = squareBuffer(context, 1, (i, length) => i < length / 2);
			source.connect(analyser);
			const t0 = context.currentTime;
			source.start(0, 0.5);
			await until(context, t0 + 0.2);
			const rms = rmsOf(analyser);
			source.stop();
			await context.close();
			ok(loud(rms), `offset 0.5 s plays the second half, rms ${rms}`);
		});

		test('buffer source ends on its own, sooner at playbackRate 2', async () => {
			const { context } = await runningContext();
			const source: any = context.createBufferSource();
			source.buffer = squareBuffer(context, 0.4);
			source.playbackRate.value = 2;
			source.connect(context.destination);
			let endedAt = -1;
			source.onended = () => (endedAt = context.currentTime);
			const t0 = context.currentTime;
			source.start();
			await until(context, t0 + 0.6);
			await context.close();
			const took = endedAt - t0;
			ok(endedAt > 0, 'ended fired');
			ok(took > 0.1 && took < 0.35, `0.4 s buffer at rate 2 ended after ${took.toFixed(3)} s`);
		});

		test('decoded audio at another sample rate plays for its duration', async () => {
			const { context, analyser } = await runningContext();
			const rate = 22050;
			const samples = new Float32Array(Math.round(rate * 0.5));
			for (let i = 0; i < samples.length; i++) samples[i] = Math.floor(i / 25) % 2 === 0 ? 0.9 : -0.9;
			const source: any = context.createBufferSource();
			const decoded = await context.decodeAudioData(wavBase64(samples, rate));
			source.buffer = decoded;
			source.connect(analyser);
			let endedAt = -1;
			source.onended = () => (endedAt = context.currentTime);
			const t0 = context.currentTime;
			source.start();
			await until(context, t0 + 0.15);
			const during = rmsOf(analyser);
			await until(context, t0 + 0.8);
			await context.close();
			ok(during > 0.5, `audible, rms ${during}`);
			const took = endedAt - t0;
			ok(endedAt > 0 && Math.abs(took - decoded.duration) < 0.1, `${decoded.duration.toFixed(3)} s buffer ended after ${took.toFixed(3)} s`);
		});

		test('a delay holds its input back by delayTime, tail included', async () => {
			const { context, analyser } = await runningContext();
			const delay = context.createDelay(1);
			delay.delayTime.value = 0.3;
			delay.connect(analyser);
			const source = context.createConstantSource({ offset: 1 });
			source.connect(delay);
			const t0 = context.currentTime;
			source.start(t0);
			source.stop(t0 + 0.3);
			await until(context, t0 + 0.15);
			const before = rmsOf(analyser);
			await until(context, t0 + 0.45);
			const during = rmsOf(analyser);
			await until(context, t0 + 0.85);
			const after = rmsOf(analyser);
			await context.close();
			ok(quiet(before), `silent for delayTime, rms ${before}`);
			ok(loud(during), `audible after the source stopped, rms ${during}`);
			ok(quiet(after), `silent once the tail ends, rms ${after}`);
		});

		test('constant source sounds only between start(when) and stop(when)', async () => {
			const { context, analyser } = await runningContext();
			const source = context.createConstantSource({ offset: 1 });
			source.connect(analyser);
			let ended = 0;
			source.onended = () => ended++;
			const t0 = context.currentTime;
			source.start(t0 + 0.3);
			source.stop(t0 + 0.6);
			await until(context, t0 + 0.15);
			const before = rmsOf(analyser);
			await until(context, t0 + 0.45);
			const during = rmsOf(analyser);
			await until(context, t0 + 0.85);
			const after = rmsOf(analyser);
			await context.close();
			ok(quiet(before), `silent before start, rms ${before}`);
			ok(loud(during), `audible inside the window, rms ${during}`);
			ok(quiet(after), `silent after stop, rms ${after}`);
			ok(ended === 1, `ended events ${ended}`);
		});

		test('buffer source start(when) lines up with gain automation', async () => {
			const { context, analyser } = await runningContext();
			const rate = context.sampleRate;
			// Both sides climb 0.01 a frame from `when`; a source starting k frames off leaves 0.01·k.
			const step = 0.01;
			const buffer = context.createBuffer({ numberOfChannels: 1, length: rate, sampleRate: rate });
			const ramp = new Float32Array(rate);
			for (let i = 0; i < rate; i++) ramp[i] = step * i;
			buffer.copyToChannel(ramp, 0);
			const source: any = context.createBufferSource();
			source.buffer = buffer;
			source.connect(analyser);
			const reference = context.createGain();
			reference.connect(analyser);
			const negative = context.createConstantSource({ offset: -1 });
			negative.connect(reference);
			const t0 = context.currentTime;
			const when = t0 + 0.3;
			reference.gain.setValueAtTime(0, t0);
			reference.gain.setValueAtTime(0, when);
			reference.gain.linearRampToValueAtTime(step * rate, when + 1);
			negative.start();
			source.start(when);
			await until(context, when + 0.2);
			const rms = rmsOf(analyser);
			await context.close();
			ok(rms < 0.02, `buffer and ramp differ by ${(rms / step).toFixed(1)} frames, rms ${rms}`);
		});

		test('gain follows setValueAtTime and linearRampToValueAtTime', async () => {
			const { context, analyser } = await runningContext();
			const gain = context.createGain();
			gain.connect(analyser);
			const source = context.createConstantSource({ offset: 1 });
			source.connect(gain);
			const t0 = context.currentTime;
			gain.gain.setValueAtTime(0, t0);
			gain.gain.setValueAtTime(1, t0 + 0.3);
			gain.gain.setValueAtTime(1, t0 + 0.6);
			gain.gain.linearRampToValueAtTime(0, t0 + 1.2);
			source.start(t0);
			await until(context, t0 + 0.15);
			const before = rmsOf(analyser);
			await until(context, t0 + 0.45);
			const held = rmsOf(analyser);
			await until(context, t0 + 0.9);
			const ramping = rmsOf(analyser);
			await until(context, t0 + 1.35);
			const after = rmsOf(analyser);
			await context.close();
			ok(quiet(before), `silent before the step, rms ${before}`);
			ok(Math.abs(held - 1) < 0.05, `1 after the step, rms ${held}`);
			ok(ramping > 0.3 && ramping < 0.7, `halfway down the ramp, rms ${ramping}`);
			ok(quiet(after), `silent after the ramp, rms ${after}`);
		});

		const PARAM_INPUT = 'a source connected to gain.gain while running opens it';
		if (isAndroid) {
			skip(PARAM_INPUT, 'no AudioNode.connect(AudioParam) on Android');
		} else {
			test(PARAM_INPUT, async () => {
				const { context, analyser } = await runningContext();
				const gain = context.createGain();
				gain.gain.value = 0;
				gain.connect(analyser);
				const source = context.createConstantSource({ offset: 1 });
				source.connect(gain);
				source.start();
				const t0 = context.currentTime;
				const modulator = context.createConstantSource({ offset: 1 });
				modulator.connect(gain.gain);
				modulator.start(t0 + 0.3);
				await until(context, t0 + 0.15);
				const before = rmsOf(analyser);
				await until(context, t0 + 0.45);
				const after = rmsOf(analyser);
				await context.close();
				ok(quiet(before), `silent before the input starts, rms ${before}`);
				ok(Math.abs(after - 1) < 0.05, `gain 0 plus input 1, rms ${after}`);
			});
		}

		test('oscillator sounds only between start(when) and stop(when)', async () => {
			const { context, analyser } = await runningContext();
			const oscillator = context.createOscillator({ type: 'square', frequency: 440 });
			oscillator.connect(analyser);
			let ended = 0;
			oscillator.onended = () => ended++;
			const t0 = context.currentTime;
			oscillator.start(t0 + 0.3);
			oscillator.stop(t0 + 0.6);
			await until(context, t0 + 0.15);
			const before = rmsOf(analyser);
			await until(context, t0 + 0.45);
			const during = rmsOf(analyser);
			await until(context, t0 + 0.85);
			const after = rmsOf(analyser);
			await context.close();
			ok(quiet(before), `silent before start, rms ${before}`);
			ok(loud(during), `audible inside the window, rms ${during}`);
			ok(quiet(after), `silent after stop, rms ${after}`);
			ok(ended === 1, `ended events ${ended}`);
		});
	});
}
