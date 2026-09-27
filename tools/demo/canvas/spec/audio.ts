import { AudioContext, OfflineAudioContext } from '@nativescript/audio-context';
import { suite, test, ok, equal, closeTo, throws, rejects } from './harness';

declare const global: any;

function rms(samples: Float32Array): number {
	let sum = 0;
	for (let i = 0; i < samples.length; i++) sum += samples[i] * samples[i];
	return Math.sqrt(sum / samples.length);
}

/** A mono 16-bit WAV of `samples`, base64. */
function wavBase64(samples: Float32Array, sampleRate: number): string {
	const bytes = new DataView(new ArrayBuffer(44 + samples.length * 2));
	const text = (offset: number, value: string) => {
		for (let i = 0; i < value.length; i++) bytes.setUint8(offset + i, value.charCodeAt(i));
	};
	text(0, 'RIFF');
	bytes.setUint32(4, 36 + samples.length * 2, true);
	text(8, 'WAVEfmt ');
	bytes.setUint32(16, 16, true);
	bytes.setUint16(20, 1, true);
	bytes.setUint16(22, 1, true);
	bytes.setUint32(24, sampleRate, true);
	bytes.setUint32(28, sampleRate * 2, true);
	bytes.setUint16(32, 2, true);
	bytes.setUint16(34, 16, true);
	text(36, 'data');
	bytes.setUint32(40, samples.length * 2, true);
	for (let i = 0; i < samples.length; i++) bytes.setInt16(44 + i * 2, Math.round(samples[i] * 32767), true);
	const alphabet = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
	const raw = new Uint8Array(bytes.buffer);
	let out = '';
	for (let i = 0; i < raw.length; i += 3) {
		const n = (raw[i] << 16) | ((raw[i + 1] ?? 0) << 8) | (raw[i + 2] ?? 0);
		out += alphabet[(n >> 18) & 63] + alphabet[(n >> 12) & 63] + (i + 1 < raw.length ? alphabet[(n >> 6) & 63] : '=') + (i + 2 < raw.length ? alphabet[n & 63] : '=');
	}
	return out;
}

const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export function registerAudioSpec() {
	suite('audio.offline', () => {
		test('canvas-polyfill installs the Web Audio globals', () => {
			equal(global.AudioContext, AudioContext, 'global AudioContext');
			equal(global.OfflineAudioContext, OfflineAudioContext, 'global OfflineAudioContext');
		});

		test('renders an oscillator through a gain', async () => {
			const context = new OfflineAudioContext(1, 4800, 48000);
			const oscillator = context.createOscillator({ type: 'square', frequency: 1000 });
			oscillator.connect(context.createGain({ gain: 0.5 })).connect(context.destination);
			oscillator.start();
			const buffer = await context.startRendering();
			equal(buffer.length, 4800);
			closeTo(rms(buffer.getChannelData(0).subarray(480)), 0.5, 0.02, 'square wave rms');
			equal(context.state, 'closed');
		});

		test('fires ended on the JS thread', async () => {
			const context = new OfflineAudioContext(1, 4800, 48000);
			const source = context.createConstantSource();
			source.connect(context.destination);
			let ended = 0;
			source.onended = () => ended++;
			source.start(0);
			source.stop(0.01);
			await context.startRendering();
			await wait(100);
			equal(ended, 1, 'ended events');
		});

		test('decodes base64 audio', async () => {
			const samples = new Float32Array(4410);
			for (let i = 0; i < samples.length; i++) samples[i] = 0.5 * Math.sin((2 * Math.PI * 441 * i) / 44100);
			const context = new OfflineAudioContext(1, 128, 44100);
			const buffer = await context.decodeAudioData(wavBase64(samples, 44100));
			equal(buffer.length, 4410);
			closeTo(buffer.getChannelData(0)[25], 0.5, 1e-3, 'peak sample');
			await rejects(context.decodeAudioData(new ArrayBuffer(16)), 'garbage decodes');
		});

		test('spec violations throw', () => {
			const context = new OfflineAudioContext(1, 128, 48000);
			const oscillator = context.createOscillator();
			oscillator.start();
			throws(() => oscillator.start());
			throws(() => oscillator.connect(new OfflineAudioContext(1, 128, 48000).destination));
			throws(() => (context.createAnalyser().fftSize = 1000));
		});
	});

	suite('audio.realtime', () => {
		test('plays on the default device and changes state', async () => {
			const context = new AudioContext();
			const states: string[] = [];
			context.onstatechange = () => states.push(context.state);
			const gain = context.createGain({ gain: 0 });
			const oscillator = context.createOscillator();
			oscillator.connect(gain).connect(context.destination);
			oscillator.start();
			equal(context.state, 'running');
			ok(context.sampleRate > 0, `sampleRate ${context.sampleRate}`);
			const start = context.currentTime;
			await wait(300);
			ok(context.currentTime - start > 0.15, `currentTime advanced ${context.currentTime - start}s in 0.3s`);
			await context.suspend();
			equal(context.state, 'suspended');
			await context.resume();
			await context.close();
			equal(context.state, 'closed');
			await wait(100);
			ok(states.indexOf('suspended') >= 0 && states[states.length - 1] === 'closed', `statechange events: ${states.join(',')}`);
		});

		test('fires ended from the render thread', async () => {
			const context = new AudioContext();
			const source = context.createConstantSource({ offset: 0 });
			source.connect(context.destination);
			let ended = false;
			source.addEventListener('ended', () => (ended = true));
			source.start();
			source.stop(context.currentTime + 0.05);
			await wait(400);
			await context.close();
			ok(ended, 'ended event');
		});
	});
}
