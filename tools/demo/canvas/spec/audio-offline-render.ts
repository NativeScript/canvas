import { AudioBuffer, OfflineAudioContext } from '@nativescript/audio-context';
import { isAndroid } from '@nativescript/core';
import { suite, test, skip, equal, closeTo, ok } from './harness';

// OfflineAudioContext rendering, against what Chrome does.

const RATE = 48000;
const Q = 128;

function constant(context: any, offset: number, when = 0) {
	const source = context.createConstantSource({ offset });
	source.start(when);
	return source;
}

function stereo(length: number, left: number, right: number) {
	const buffer = new AudioBuffer({ numberOfChannels: 2, length, sampleRate: RATE });
	buffer.copyToChannel(new Float32Array(length).fill(left), 0);
	buffer.copyToChannel(new Float32Array(length).fill(right), 1);
	return buffer;
}

export function registerAudioOfflineRenderSpec() {
	suite('audio.offline.render', () => {
		test('renders the whole length and closes', async () => {
			const context = new OfflineAudioContext(1, Q * 8, RATE);
			constant(context, 0.5).connect(context.destination);
			const buffer = await context.startRendering();
			equal(buffer.length, Q * 8, 'length');
			const samples = buffer.getChannelData(0);
			ok(
				samples.every((v) => v === 0.5),
				`first ${samples[0]}, last ${samples[Q * 8 - 1]}`,
			);
			equal(context.state, 'closed', 'state');
			closeTo(context.currentTime, (Q * 8) / RATE, 1e-9, 'currentTime');
		});

		test('starts a source on its frame', async () => {
			for (const frame of [Q * 4, 100]) {
				const context = new OfflineAudioContext(1, Q * 8, RATE);
				constant(context, 0.5, frame / RATE).connect(context.destination);
				const samples = (await context.startRendering()).getChannelData(0);
				equal(samples[frame - 1], 0, `silent before frame ${frame}`);
				equal(samples[frame], 0.5, `playing from frame ${frame}`);
			}
		});

		test('applies setValueAtTime on its frame', async () => {
			const context = new OfflineAudioContext(1, Q * 8, RATE);
			const gain = context.createGain();
			gain.gain.setValueAtTime(1, 0);
			gain.gain.setValueAtTime(0.5, (Q * 4) / RATE);
			constant(context, 0.5).connect(gain);
			gain.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[Q * 4 - 1], 0.5, 'before the event');
			equal(samples[Q * 4], 0.25, 'from the event');
		});

		test('keeps samples above 1', async () => {
			const context = new OfflineAudioContext(1, Q * 4, RATE);
			constant(context, 1).connect(context.destination);
			constant(context, 0.5).connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[Q], 1.5);
		});

		test("renders every channel at the context's rate", async () => {
			const context = new OfflineAudioContext(2, Q * 4, 44100);
			const oscillator = context.createOscillator();
			oscillator.frequency.value = 441;
			oscillator.connect(context.destination);
			oscillator.start(0);
			const buffer = await context.startRendering();
			equal(buffer.numberOfChannels, 2, 'channels');
			equal(buffer.sampleRate, 44100, 'sampleRate');
			// 100 frames per cycle: a peak at frame 25.
			closeTo(buffer.getChannelData(0)[25], 1, 1e-4, 'left');
			closeTo(buffer.getChannelData(1)[25], 1, 1e-4, 'right');
		});

		test('delays by delayTime, tail included', async () => {
			const context = new OfflineAudioContext(1, Q * 8, RATE);
			const delay = context.createDelay(1);
			equal(delay.delayTime.value, 0, 'delayTime defaults to 0');
			delay.delayTime.value = (Q * 2) / RATE;
			const source = constant(context, 0.5);
			source.stop((Q * 2) / RATE);
			source.connect(delay);
			delay.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[Q * 2 - 1], 0, 'silent for delayTime');
			equal(samples[Q * 2], 0.5, 'delayed from delayTime');
			equal(samples[Q * 4 - 1], 0.5, 'the tail outlives the source');
			equal(samples[Q * 4], 0, 'and ends delayTime after it');
		});

		test('delays into a gain', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const delay = context.createDelay();
			delay.delayTime.value = 10 / RATE;
			const gain = context.createGain({ gain: 0.5 } as any);
			constant(context, 0.5).connect(delay);
			delay.connect(gain);
			gain.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[9], 0, 'before delayTime');
			equal(samples[10], 0.25, 'delayed and scaled');
		});

		test('sums sources connected with an output and input index', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			constant(context, 0.5).connect(context.destination, 0);
			constant(context, 0.25).connect(context.destination, 0);
			constant(context, 0.125).connect(context.destination, 0, 0);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[Q], 0.875);
		});

		test('a splitter output into a filter filters only that channel', async () => {
			const context = new OfflineAudioContext(2, Q * 32, RATE);
			const source: any = context.createBufferSource();
			source.buffer = stereo(Q * 32, 1, 1);
			const splitter = context.createChannelSplitter({ numberOfOutputs: 2 });
			const merger = context.createChannelMerger({ numberOfInputs: 2 });
			// A highpass blocks DC: only the right channel goes through it.
			const filter = context.createBiquadFilter();
			filter.type = 'highpass';
			filter.frequency.value = 1000;
			source.connect(splitter);
			splitter.connect(merger, 0, 0);
			splitter.connect(filter, 1);
			filter.connect(merger, 0, 1);
			merger.connect(context.destination);
			source.start(0);
			const rendered = await context.startRendering();
			const last = Q * 32 - 1;
			closeTo(rendered.getChannelData(0)[last], 1, 1e-4, 'left passes');
			closeTo(rendered.getChannelData(1)[last], 0, 1e-3, 'right is filtered');
		});

		test('disconnect(node) cuts only that connection', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const first = context.createGain();
			const half = context.createGain({ gain: 0.5 } as any);
			const quarter = context.createGain({ gain: 0.25 } as any);
			half.connect(context.destination);
			quarter.connect(context.destination);
			constant(context, 1).connect(first);
			first.connect(half);
			first.disconnect(half);
			first.connect(quarter);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[0], 0.25, 'through the remaining connection');
			equal(samples[Q], 0.25, 'and only it');
		});

		test('disconnect(node) keeps the source\'s other connections', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const half = context.createGain({ gain: 0.5 } as any);
			const quarter = context.createGain({ gain: 0.25 } as any);
			half.connect(context.destination);
			quarter.connect(context.destination);
			const source = constant(context, 1);
			source.connect(half);
			source.connect(quarter);
			source.disconnect(half);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[0], 0.25, 'from the first frame');
			equal(samples[Q], 0.25, 'and only through the other gain');
		});

		if (isAndroid) skip('disconnect(output) cuts only that output', 'disconnecting a splitter output on Android silences later realtime worklets');
		else test('disconnect(output) cuts only that output', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const source: any = context.createBufferSource();
			source.buffer = stereo(Q * 2, 1, 0.5);
			const splitter = context.createChannelSplitter({ numberOfOutputs: 2 });
			source.connect(splitter);
			splitter.connect(context.destination, 0);
			splitter.connect(context.destination, 1);
			splitter.disconnect(1);
			source.start(0);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[Q], 1, 'output 0 alone');
		});

		if (isAndroid) skip('disconnect(node, output, input) cuts only that connection', 'disconnecting a splitter output on Android silences later realtime worklets');
		else test('disconnect(node, output, input) cuts only that connection', async () => {
			const context = new OfflineAudioContext(2, Q * 2, RATE);
			const source: any = context.createBufferSource();
			source.buffer = stereo(Q * 2, 1, 0.5);
			const splitter = context.createChannelSplitter({ numberOfOutputs: 2 });
			const merger = context.createChannelMerger({ numberOfInputs: 2 });
			source.connect(splitter);
			splitter.connect(merger, 0, 0);
			splitter.connect(merger, 1, 1);
			splitter.connect(merger, 0, 1);
			splitter.disconnect(merger, 0, 1);
			merger.connect(context.destination);
			source.start(0);
			const rendered = await context.startRendering();
			equal(rendered.getChannelData(0)[Q], 1, 'left from output 0');
			equal(rendered.getChannelData(1)[Q], 0.5, 'right from output 1 only');
		});

		test('a source disconnected from a delay plays undelayed', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const delay = context.createDelay(1);
			delay.delayTime.value = Q / RATE;
			delay.connect(context.destination);
			const gain = context.createGain({ gain: 0.5 } as any);
			gain.connect(context.destination);
			const source = constant(context, 1);
			source.connect(delay);
			source.disconnect(delay);
			source.connect(gain);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[0], 0.5, 'from the first frame');
			equal(samples[Q], 0.5, 'and not doubled by the delay');
		});

		test('a stereo buffer plays its mono down-mix in a mono context', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const source: any = context.createBufferSource();
			source.buffer = stereo(Q * 2, 1, 0.5);
			source.connect(context.destination);
			source.start(0);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[0], 0.75, 'the average of left and right');
		});

		const PARAM_INPUT = 'a node connected to a gain or delayTime param adds to it per frame';
		test(PARAM_INPUT, async () => {
			const context = new OfflineAudioContext(1, Q * 4, RATE);
			const gain = context.createGain();
			gain.gain.value = 0.5;
			constant(context, 1).connect(gain);
			constant(context, 0.25, (Q * 2) / RATE).connect(gain.gain);
			gain.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[Q * 2 - 1], 0.5, 'the intrinsic value alone');
			equal(samples[Q * 2], 0.75, 'plus the input from its first frame');

			// 1/1024 s is 32 frames at this rate, exact in float.
			const rate = 32768;
			const other = new OfflineAudioContext(1, Q * 4, rate);
			const delay = other.createDelay(1);
			constant(other, 1 / 1024).connect(delay.delayTime);
			const source = constant(other, 0.5);
			source.stop(Q / rate);
			source.connect(delay);
			delay.connect(other.destination);
			const delayed = (await other.startRendering()).getChannelData(0);
			equal(delayed[31], 0, 'silent for the connected delayTime');
			equal(delayed[32], 0.5, 'delayed by it');
			equal(delayed[Q + 32], 0, 'and ends that long after the source');
		});

		// Renders `build(context, input)` per channel of a stereo context: input 0 is fed by the
		// param under test, input 1 by the reference, and both must match on every frame.
		async function sameOnEveryFrame(build: (context: any, merger: any) => void, label: string) {
			const context = new OfflineAudioContext(2, Q * 8, RATE);
			const merger = context.createChannelMerger({ numberOfInputs: 2 });
			merger.connect(context.destination);
			build(context, merger);
			const rendered = await context.startRendering();
			const got = rendered.getChannelData(0);
			const want = rendered.getChannelData(1);
			let worst = 0;
			let at = 0;
			for (let i = 0; i < got.length; i++) {
				const d = Math.abs(got[i] - want[i]);
				if (d > worst) [worst, at] = [d, i];
			}
			ok(worst < 1e-5 && want.some((v) => v !== 0), `${label}: frame ${at} differs by ${worst}`);
		}

		// The connected source starts on a frame inside a quantum, matched by a setValueAtTime there.
		const ON = Q * 3 + 5;

		const DISCONNECT_PARAM = 'disconnect(param) keeps the source\'s connection to the node';
		const OSC_PARAM = 'a node connected to OscillatorNode.frequency adds to it per frame';
		const BIQUAD_PARAM = 'a node connected to BiquadFilterNode.frequency adds to it per frame';
		const OFFSET_PARAM = 'a node connected to ConstantSourceNode.offset adds to it per frame';
		test(DISCONNECT_PARAM, async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const gain = context.createGain({ gain: 0.5 } as any);
			gain.connect(context.destination);
			const source = constant(context, 0.25);
			source.connect(gain);
			source.connect(gain.gain);
			source.disconnect(gain.gain);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[Q], 0.125, 'through the gain at its own value');
		});

		if (isAndroid) skip(OSC_PARAM, 'connected audio drifts from automation on Android oscillator frequency');
		else test(OSC_PARAM, () =>
			sameOnEveryFrame((context, merger) => {
				const driven = context.createOscillator({ frequency: 441 } as any);
				constant(context, 441, ON / RATE).connect(driven.frequency);
				const reference = context.createOscillator({ frequency: 441 } as any);
				reference.frequency.setValueAtTime(882, ON / RATE);
				driven.connect(merger, 0, 0);
				reference.connect(merger, 0, 1);
				driven.start(0);
				reference.start(0);
			}, 'frequency'));

		if (isAndroid) skip(BIQUAD_PARAM, 'BiquadFilterNode params ignore audio on Android');
		else test(BIQUAD_PARAM, () =>
			sameOnEveryFrame((context, merger) => {
				const source = context.createOscillator({ type: 'sawtooth', frequency: 441 } as any);
				const driven = context.createBiquadFilter();
				driven.frequency.value = 1000;
				constant(context, 1000, ON / RATE).connect(driven.frequency);
				const reference = context.createBiquadFilter();
				reference.frequency.value = 1000;
				reference.frequency.setValueAtTime(2000, ON / RATE);
				source.connect(driven);
				source.connect(reference);
				driven.connect(merger, 0, 0);
				reference.connect(merger, 0, 1);
				source.start(0);
			}, 'biquad'));

		test(OFFSET_PARAM, async () => {
			const context = new OfflineAudioContext(1, Q * 8, RATE);
			const source = constant(context, 0.25);
			constant(context, 0.5, ON / RATE).connect(source.offset);
			source.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[ON - 1], 0.25, 'its own offset before the input starts');
			equal(samples[ON], 0.75, 'plus the input from its first frame');
		});
	});
}
