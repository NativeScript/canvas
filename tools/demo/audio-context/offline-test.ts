import { OfflineAudioContext, AudioContext } from '@nativescript/audio-context';

export async function runOfflineTest() {
	console.log('[offline-test] starting offline test');
	try {
		const sampleRate = 44100;
		const durationSeconds = 2;
		const frames = sampleRate * durationSeconds;
		const off = new OfflineAudioContext(1, frames, sampleRate);
		console.log('[offline-test] created OfflineAudioContext', { frames, sampleRate });

		const osc = off.createOscillator({ type: 'sine', frequency: 440 });
		const gain = off.createGain({ gain: 0.2 });
		osc.connect(gain);
		gain.connect(off.destination);
		osc.start();

		// Render offline (native path). Returns buffer id string.
		const buffer = await off.startRendering();
		console.log('[offline-test] offline render completed, buffer=', buffer);

		// Try playing rendered buffer in a real AudioContext to verify.
		try {
			const ctx = new AudioContext();
			await ctx.resume();
			const src = ctx.createBufferSource({ buffer });
			src.connect(ctx.destination);
			src.start();
			console.log('[offline-test] playing rendered buffer in realtime context');
			// Stop after duration + small fudge and close context.
			setTimeout(
				() => {
					try {
						src.stop();
					} catch (e) {}
					console.log('[offline-test] playback finished, closed context');
				},
				durationSeconds * 1000 + 400,
			);
		} catch (e) {
			console.log('[offline-test] playback of rendered buffer failed', e);
		}
	} catch (err) {
		console.log('[offline-test] offline test failed', err);
	}
}

export default runOfflineTest;
