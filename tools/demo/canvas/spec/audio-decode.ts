import { AudioContext } from '@nativescript/audio-context';
import { suite, test, ok, equal } from './harness';

/** A 16-bit PCM WAV of `channels` (equal lengths), base64. */
function wav(channels: Float32Array[], sampleRate: number): string {
	const count = channels.length;
	const frames = channels[0].length;
	const dataBytes = frames * count * 2;
	const bytes = new DataView(new ArrayBuffer(44 + dataBytes));
	const text = (offset: number, value: string) => {
		for (let i = 0; i < value.length; i++) bytes.setUint8(offset + i, value.charCodeAt(i));
	};
	text(0, 'RIFF');
	bytes.setUint32(4, 36 + dataBytes, true);
	text(8, 'WAVEfmt ');
	bytes.setUint32(16, 16, true);
	bytes.setUint16(20, 1, true);
	bytes.setUint16(22, count, true);
	bytes.setUint32(24, sampleRate, true);
	bytes.setUint32(28, sampleRate * count * 2, true);
	bytes.setUint16(32, count * 2, true);
	bytes.setUint16(34, 16, true);
	text(36, 'data');
	bytes.setUint32(40, dataBytes, true);
	for (let i = 0; i < frames; i++) {
		for (let c = 0; c < count; c++) bytes.setInt16(44 + (i * count + c) * 2, Math.round(channels[c][i] * 32767), true);
	}
	const alphabet = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
	const raw = new Uint8Array(bytes.buffer);
	let out = '';
	for (let i = 0; i < raw.length; i += 3) {
		const n = (raw[i] << 16) | ((raw[i + 1] ?? 0) << 8) | (raw[i + 2] ?? 0);
		out += alphabet[(n >> 18) & 63] + alphabet[(n >> 12) & 63] + (i + 1 < raw.length ? alphabet[(n >> 6) & 63] : '=') + (i + 2 < raw.length ? alphabet[n & 63] : '=');
	}
	return out;
}

/** A ramp, so a dropped or shifted frame changes the value at that index. */
function ramp(frames: number, scale = 0.9): Float32Array {
	const out = new Float32Array(frames);
	for (let i = 0; i < frames; i++) out[i] = scale * ((i % 1000) / 500 - 1);
	return out;
}

// Chrome and the spec resample to the context's rate; iOS and Android keep the file's. Either
// way every frame must be there, so the duration is exact to one frame.
function sameDuration(buffer: { length: number; sampleRate: number }, frames: number, rate: number) {
	const expected = frames / rate;
	const actual = buffer.length / buffer.sampleRate;
	ok(Math.abs(actual - expected) <= 1 / Math.min(rate, buffer.sampleRate), `duration ${actual.toFixed(5)} s, expected ${expected.toFixed(5)} s (${buffer.length} frames at ${buffer.sampleRate} Hz)`);
}

function sameSamples(actual: Float32Array, expected: Float32Array, label: string) {
	const tolerance = 2 / 32767;
	for (const i of [0, 1, expected.length >> 1, expected.length - 2, expected.length - 1]) {
		ok(Math.abs(actual[i] - expected[i]) <= tolerance, `${label} sample ${i}: ${actual[i]} vs ${expected[i]}`);
	}
}

export function registerAudioDecodeSpec() {
	suite('audio.decode', () => {
		// The lengths straddle the decoder's 4096-frame pulls and fall short of the first one.
		for (const [frames, rate] of [
			[11025, 22050],
			[8192, 44100],
			[100, 48000],
		]) {
			test(`decodes every frame of a ${frames}-frame ${rate} Hz WAV`, async () => {
				const context = new AudioContext();
				const samples = ramp(frames);
				const buffer = await context.decodeAudioData(wav([samples], rate));
				await context.close();
				sameDuration(buffer, frames, rate);
				if (buffer.sampleRate === rate) sameSamples(buffer.getChannelData(0), samples, 'mono');
			});
		}

		test('decodes a data: URL', async () => {
			const context = new AudioContext();
			const samples = ramp(3000);
			const buffer = await context.decodeAudioData('data:audio/wav;base64,' + wav([samples], 44100));
			await context.close();
			sameDuration(buffer, 3000, 44100);
			if (buffer.sampleRate === 44100) sameSamples(buffer.getChannelData(0), samples, 'mono');
		});

		test('decodes both channels of a stereo WAV', async () => {
			const context = new AudioContext();
			const left = ramp(5000);
			const right = ramp(5000, -0.5);
			const buffer = await context.decodeAudioData(wav([left, right], 44100));
			await context.close();
			equal(buffer.numberOfChannels, 2, 'channels');
			sameDuration(buffer, 5000, 44100);
			if (buffer.sampleRate === 44100) {
				sameSamples(buffer.getChannelData(0), left, 'left');
				sameSamples(buffer.getChannelData(1), right, 'right');
			}
		});
	});
}
