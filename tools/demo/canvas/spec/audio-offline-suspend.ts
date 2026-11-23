import { AudioWorkletNode, OfflineAudioContext } from '@nativescript/audio-context';
import { isAndroid } from '@nativescript/core';
import { suite, test, skip, equal, closeTo, rejects } from './harness';

// OfflineAudioContext.suspend(when) / resume(), against what Chrome does.

const RATE = 48000;
const Q = 128;

const LEVEL = 'data:text/javascript,' + encodeURIComponent(`
class Level extends AudioWorkletProcessor {
	static get parameterDescriptors() {
		return [{ name: 'level', defaultValue: 0.25, automationRate: 'k-rate' }];
	}
	process(inputs, outputs, parameters) {
		outputs[0][0].fill(parameters.level[0]);
		return true;
	}
}
registerProcessor('level', Level);
`);

export function registerAudioOfflineSuspendSpec() {
	suite('audio.offline.suspend', () => {
		if (isAndroid) {
			skip('suspend and resume', 'not implemented on Android');
			return;
		}

		test('changes made while suspended apply from the suspend frame', async () => {
			const context = new OfflineAudioContext(1, Q * 32, RATE);
			const states: string[] = [];
			const times: number[] = [];
			let source: any;
			context.suspend((Q * 8) / RATE).then(() => {
				states.push(context.state);
				times.push(context.currentTime);
				source = context.createConstantSource();
				source.connect(context.destination);
				source.start(context.currentTime);
				// Scheduled while held, after rendering started.
				context.suspend((Q * 20) / RATE).then(() => {
					times.push(context.currentTime);
					source.stop(context.currentTime);
					context.resume();
				});
				context.resume();
				states.push(context.state);
			});
			const rendering = context.startRendering();
			equal(context.state, 'running', 'running once rendering starts');
			const samples = (await rendering).getChannelData(0);
			equal(states.join(), 'suspended,running', 'states around the first suspend');
			closeTo(times[0], (Q * 8) / RATE, 1e-9, 'currentTime at the first suspend');
			closeTo(times[1], (Q * 20) / RATE, 1e-9, 'currentTime at the second suspend');
			equal(samples[Q * 8 - 1], 0, 'silent before the first suspend');
			equal(samples[Q * 8], 1, 'source from the first suspend frame');
			equal(samples[Q * 20 - 1], 1, 'source until the second suspend');
			equal(samples[Q * 20], 0, 'stopped from the second suspend frame');
			equal(context.state, 'closed');
		});

		test('sources scheduled before rendering keep their frames across a rewiring suspend', async () => {
			const context = new OfflineAudioContext(1, Q * 16, RATE);
			const early = context.createConstantSource();
			early.connect(context.destination);
			early.start((Q * 10 + 5) / RATE);
			context.suspend((Q * 4) / RATE).then(() => {
				const late = context.createConstantSource({ offset: 0.5 });
				late.connect(context.destination);
				late.start(context.currentTime);
				context.resume();
			});
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples.findIndex((v) => v !== 0), Q * 4, 'the source added while held starts at the suspend');
			equal(samples[Q * 10 + 4], 0.5, 'only that source before the scheduled frame');
			equal(samples[Q * 10 + 5], 1.5, 'the source scheduled before rendering joins on its frame');
		});

		test('a worklet sees a param set while held from the suspend frame', async () => {
			if (typeof (globalThis as any).Worker !== 'function') return;
			const context = new OfflineAudioContext(1, Q * 16, RATE);
			await context.audioWorklet.addModule(LEVEL);
			const node = new AudioWorkletNode(context, 'level', { numberOfInputs: 0, outputChannelCount: [1] });
			node.connect(context.destination);
			context.suspend((Q * 6) / RATE).then(() => {
				node.parameters.get('level').setValueAtTime(0.75, context.currentTime);
				context.resume();
			});
			const samples = (await context.startRendering()).getChannelData(0);
			equal(samples[0], 0.25, 'the default level from the first frame');
			equal(samples[Q * 6 - 1], 0.25, 'the default level before the suspend');
			equal(samples[Q * 6], 0.75, 'the new level from the suspend frame');
		});

		test('suspends at 0 and rounds up to a quantum', async () => {
			const context = new OfflineAudioContext(1, Q * 4, 44100);
			const order: number[] = [];
			context.suspend(0).then(() => {
				order.push(context.currentTime);
				setTimeout(() => context.resume(), 5);
			});
			context.suspend(1 / 44100).then(() => {
				order.push(context.currentTime);
				context.resume();
			});
			await context.startRendering();
			equal(order.length, 2, 'both suspends resolved');
			closeTo(order[0], 0, 1e-9, 'held at 0');
			closeTo(order[1], Q / 44100, 1e-9, 'rounded up to the next quantum');
		});

		test('rejects invalid suspends and an early resume', async () => {
			const context = new OfflineAudioContext(1, Q * 4, RATE);
			const early = await rejects(context.resume(), 'resume before startRendering');
			equal(early?.name, 'InvalidStateError', 'early resume');
			equal((await rejects(context.suspend(-1), 'negative time'))?.name, 'InvalidStateError');
			equal((await rejects(context.suspend((Q * 4) / RATE), 'past the end'))?.name, 'InvalidStateError');
			context.suspend(Q / RATE).then(() => context.resume());
			equal((await rejects(context.suspend(100 / RATE), 'same quantum twice'))?.name, 'InvalidStateError');
			await context.startRendering();
			equal((await rejects(context.suspend(0), 'after rendering'))?.name, 'InvalidStateError');
			equal((await rejects(context.resume(), 'resume after rendering'))?.name, 'InvalidStateError');
		});

		test('a suspend at the held frame is rejected', async () => {
			const context = new OfflineAudioContext(1, Q * 8, RATE);
			let name = '';
			context.suspend((Q * 2) / RATE).then(async () => {
				name = (await rejects(context.suspend((Q * 2) / RATE), 'suspend at the current frame'))?.name;
				context.resume();
			});
			await context.startRendering();
			equal(name, 'InvalidStateError');
		});
	});
}
