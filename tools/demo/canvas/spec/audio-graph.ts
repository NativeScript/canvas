import { AudioBuffer, OfflineAudioContext } from '@nativescript/audio-context';
import { isIOS } from '@nativescript/core';
import { suite, test, skip, equal, closeTo, ok } from './harness';

// Graph routing in an OfflineAudioContext, frame-exact, against what Chrome does.

const RATE = 48000;
const Q = 128;

function constant(context: any, offset: number, when = 0) {
	const source = context.createConstantSource({ offset });
	source.start(when);
	return source;
}

function gain(context: any, value: number) {
	const node = context.createGain();
	node.gain.value = value;
	return node;
}

export function registerAudioGraphSpec() {
	suite('audio.graph', () => {
		test('a source fans out to two gains, each reaching the destination', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const source = constant(context, 1);
			const half = gain(context, 0.5);
			const quarter = gain(context, 0.25);
			source.connect(half);
			source.connect(quarter);
			half.connect(context.destination);
			quarter.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[0], 0.75, 'both paths from the first frame');
			equal(samples[Q * 2 - 1], 0.75, 'to the last');
		});

		test('chained gains multiply', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const first = gain(context, 0.5);
			const second = gain(context, 0.25);
			constant(context, 1).connect(first);
			first.connect(second);
			second.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[0], 0.125);
			equal(samples[Q], 0.125);
		});

		test('wiring renders the same in any order', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const first = gain(context, 0.5);
			const second = gain(context, 0.25);
			// Downstream first, the source last.
			second.connect(context.destination);
			first.connect(second);
			const source = context.createConstantSource({ offset: 1 });
			source.connect(first);
			source.start(0);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[0], 0.125, 'through both gains');
			equal(samples[Q], 0.125, 'and only through them');
		});

		const FEEDBACK = 'a DelayNode in a feedback loop echoes, decaying by the loop gain';
		if (isIOS) skip(FEEDBACK, "AVAudioEngine can't render a cycle");
		else
			test(FEEDBACK, async () => {
				const context = new OfflineAudioContext(1, Q * 8, RATE);
				const delay = context.createDelay(1);
				delay.delayTime.value = (Q * 2) / RATE;
				const feedback = gain(context, 0.5);
				const source = constant(context, 1);
				source.stop(Q / RATE);
				source.connect(delay);
				delay.connect(feedback);
				feedback.connect(delay);
				delay.connect(context.destination);
				const samples = (await context.startRendering()).getChannelData(0);
				equal(samples[Q * 2 - 1], 0, 'silent for delayTime');
				equal(samples[Q * 2], 1, 'the first echo');
				equal(samples[Q * 3], 0, 'as long as the source');
				equal(samples[Q * 4], 0.5, 'the second, halved');
				equal(samples[Q * 6], 0.25, 'the third, halved again');
				equal(samples[Q * 7 - 1], 0.25, 'each a quantum long');
			});

		test('source.disconnect(node) in a chain silences the source', async () => {
			const context = new OfflineAudioContext(1, Q * 2, RATE);
			const first = gain(context, 0.5);
			const second = gain(context, 0.5);
			const source = constant(context, 1);
			source.connect(first);
			first.connect(second);
			second.connect(context.destination);
			source.disconnect(first);
			const samples = (await context.startRendering()).getChannelData(0);
			ok(
				samples.every((v) => v === 0),
				`first ${samples[0]}, last ${samples[Q * 2 - 1]}`,
			);
		});

		test('a node fans out into a delay and the destination', async () => {
			const context = new OfflineAudioContext(1, Q * 4, RATE);
			const half = gain(context, 0.5);
			const delay = context.createDelay(1);
			delay.delayTime.value = Q / RATE;
			constant(context, 1).connect(half);
			half.connect(context.destination);
			half.connect(delay);
			delay.connect(context.destination);
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[0], 0.5, 'the direct path alone first');
			equal(samples[Q - 1], 0.5, 'until delayTime');
			equal(samples[Q], 1, 'then both paths');
		});

		const EXPLICIT = 'a gain with channelCount 1, explicit, mixes a stereo source down to mono';
		if (isIOS) skip(EXPLICIT, 'no channelCount or channelCountMode on iOS');
		else
			test(EXPLICIT, async () => {
				const context = new OfflineAudioContext(2, Q, RATE);
				const buffer = new AudioBuffer({ numberOfChannels: 2, length: Q, sampleRate: RATE });
				buffer.copyToChannel(new Float32Array(Q).fill(1), 0);
				buffer.copyToChannel(new Float32Array(Q).fill(0.5), 1);
				const source: any = context.createBufferSource();
				source.buffer = buffer;
				const node: any = gain(context, 1);
				node.channelCount = 1;
				node.channelCountMode = 'explicit';
				source.connect(node);
				node.connect(context.destination);
				source.start(0);
				const rendered = await context.startRendering();
				closeTo(rendered.getChannelData(0)[0], 0.75, 1e-6, 'left gets the mono mix');
				closeTo(rendered.getChannelData(1)[0], 0.75, 1e-6, 'and so does right');
			});
	});
}
