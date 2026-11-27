import { AudioContext } from '@nativescript/audio-context';
import { suite, test, ok } from './harness';
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
		// The analyser holds the last 32768 frames, so it sees the whole short burst.
		for (const kind of ['constant', 'buffer', 'oscillator']) {
			test(`${kind} source stopped 20 ms after start plays 20 ms`, async () => {
				const { context, analyser } = await runningContext();
				analyser.fftSize = 32768;
				const source: any = kind === 'constant' ? context.createConstantSource({ offset: 1 }) : kind === 'buffer' ? context.createBufferSource() : context.createOscillator({ type: 'square', frequency: 50 });
				if (kind === 'buffer') {
					source.buffer = squareBuffer(context, 1);
				}
				source.connect(analyser);
				const t0 = context.currentTime;
				source.start();
				source.stop(t0 + 0.02);
				await until(context, t0 + 0.3);
				const data = new Float32Array(32768);
				analyser.getFloatTimeDomainData(data);
				const rate = context.sampleRate;
				await context.close();
				let loudFrames = 0;
				for (let i = 0; i < data.length; i++) if (Math.abs(data[i]) > 0.5) loudFrames++;
				const expected = 0.02 * rate;
				// Up to a render call either side: start() takes effect at the next one.
				ok(loudFrames > expected * 0.4 && loudFrames < expected + 1100, `${loudFrames} loud frames, expected about ${expected}`);
			});
		}

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

		test("close stops the context's sources", async () => {
			const closed = new AudioContext();
			const source = closed.createConstantSource({ offset: 1 });
			source.connect(closed.destination);
			source.start();
			await until(closed, closed.currentTime + 0.1);
			await closed.close();
			const { context, analyser } = await runningContext();
			await until(context, context.currentTime + 0.2);
			const rms = rmsOf(analyser);
			await context.close();
			ok(quiet(rms), `silent in the next context, rms ${rms}`);
		});

		test("close stops the context's sources", async () => {
			const closed = new AudioContext();
			const source = closed.createConstantSource({ offset: 1 });
			source.connect(closed.destination);
			source.start();
			await until(closed, closed.currentTime + 0.1);
			await closed.close();
			const { context, analyser } = await runningContext();
			await until(context, context.currentTime + 0.2);
			const rms = rmsOf(analyser);
			await context.close();
			ok(quiet(rms), `silent in the next context, rms ${rms}`);
		});

		test('suspend and resume act on one realtime context, not the others', async () => {
			const watch = (context: AudioContext) => {
				const analyser = context.createAnalyser({ fftSize: FFT });
				const oscillator = context.createOscillator({ frequency: 441 });
				oscillator.connect(analyser).connect(context.destination);
				oscillator.start();
				return analyser;
			};
			const snapshot = (analyser: any) => {
				const data = new Float32Array(FFT);
				analyser.getFloatTimeDomainData(data);
				return data;
			};
			const same = (a: Float32Array, b: Float32Array) => a.every((v, i) => v === b[i]);
			const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
			const a = new AudioContext();
			const b = new AudioContext();
			const outA = watch(a);
			const outB = watch(b);
			await Promise.all([a.resume(), b.resume()]);
			await until(a, a.currentTime + 0.1);
			await until(b, b.currentTime + 0.1);

			await a.suspend();
			const [frozenA, runningB, heardA, heardB] = [a.currentTime, b.currentTime, snapshot(outA), snapshot(outB)];
			await wait(300);
			ok(a.state === 'suspended' && b.state === 'running', `states ${a.state}, ${b.state}`);
			ok(a.currentTime === frozenA, `the suspended clock moved from ${frozenA} to ${a.currentTime}`);
			ok(b.currentTime > runningB + 0.15, `the other clock went from ${runningB} to ${b.currentTime}`);
			ok(same(heardA, snapshot(outA)), 'the suspended context renders nothing');
			ok(!same(heardB, snapshot(outB)), 'the other context keeps rendering');

			await a.resume();
			const resumedA = a.currentTime;
			await wait(300);
			ok(a.currentTime > resumedA + 0.15, `the resumed clock went from ${resumedA} to ${a.currentTime}`);

			await Promise.all([a.suspend(), b.suspend()]);
			const [heldA, heldB] = [a.currentTime, b.currentTime];
			await wait(300);
			ok(a.currentTime === heldA && b.currentTime === heldB, `held clocks moved: ${heldA} to ${a.currentTime}, ${heldB} to ${b.currentTime}`);

			await Promise.all([a.resume(), b.resume()]);
			const [againA, againB] = [a.currentTime, b.currentTime];
			await wait(300);
			const advanced = a.currentTime > againA + 0.15 && b.currentTime > againB + 0.15;
			await Promise.all([a.close(), b.close()]);
			ok(advanced, `both clocks resume: ${againA} to ${a.currentTime}, ${againB} to ${b.currentTime}`);
		});

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
