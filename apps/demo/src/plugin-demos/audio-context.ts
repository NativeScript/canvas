import { EventData, Page, Slider, knownFolders } from '@nativescript/core';
import { DemoSharedAudioContext } from '@demo/shared';
import { AudioContext } from '@nativescript/audio-context';
export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new DemoModel();
}

export function canvasReady(args: EventData) {
	const canvas = args.object as any;
	const page = canvas.page as Page;
	const ctx = page.bindingContext as any;
	if (ctx && ctx.initVisualizer) ctx.initVisualizer(canvas);
}

export function playSample(args: EventData) {
	const page = (<any>args.object).page as Page;
	const ctx = page.bindingContext as any;
	const testPath = __APPLE__ ? '~/assets/file-assets/audio/sine441stereo.mp3' : '~/assets/file-assets/audio/gs-16b-1c-44100hz.wav';
	const resolvedPath = testPath.startsWith('~/') ? testPath.replace('~/', knownFolders.currentApp().path + '/') : testPath;
	if (ctx && ctx.playUrl) {
		const p = ctx.playUrl(resolvedPath);
		if (p && p.catch) {
			p.catch(() => {
				if (ctx.playBundledSample) ctx.playBundledSample();
			});
		}
	} else if (ctx && ctx.playBundledSample) {
		ctx.playBundledSample();
	}
}

export function stopTap(args: EventData) {
	const page = (<any>args.object).page as Page;
	const ctx = page.bindingContext as any;
	if (ctx && ctx.stop) ctx.stop();
}

export function onVolumeChange(args: EventData) {
	const slider = args.object as Slider;
	const page = slider.page as Page;
	const ctx = page.bindingContext as any;
	if (ctx && ctx.setVolume) ctx.setVolume(slider.value / 100);
}

export function panSweepTap(args: EventData) {
	(async () => {
		const page = (<any>args.object).page as Page;
		const demo = page.bindingContext as DemoSharedAudioContext;
		const duration = 4000;
		await demo.initAudio();
		const ctx = demo.ctx as AudioContext;
		const durSec = duration / 1000;
		const now = ctx.currentTime;
		const osc = ctx.createOscillator({ type: 'sine', frequency: 440 });
		const panner = ctx.createStereoPanner();

		panner.pan.setValueAtTime(-1, now);
		panner.pan.linearRampToValueAtTime(1, now + durSec);

		osc.connect(panner);

		panner.connect(demo.gainNode || ctx.destination);
		await ctx.resume();
		osc.start();
		setTimeout(() => {
			osc.stop && osc.stop();
			osc.disconnect && osc.disconnect();
			panner.disconnect && panner.disconnect();
		}, duration + 200);
		return;
	})();
}

export function positionSweepTap(args: EventData) {
	(async () => {
		const page = (<any>args.object).page as Page;
		const demo = page.bindingContext as DemoSharedAudioContext;
		const duration = 4000;
		await demo.initAudio();
		const ctx = demo.ctx as AudioContext;
		const durSec = duration / 1000;
		const now = typeof ctx.currentTime === 'number' ? ctx.currentTime : Date.now() / 1000;
		const osc = ctx.createOscillator({ type: 'sine', frequency: 440 });
		const panner = ctx.createPanner();

		panner.positionX.setValueAtTime(-1, now);
		panner.positionX.linearRampToValueAtTime(1, now + durSec);

		osc.connect(panner);
		panner.connect(demo.gainNode || ctx.destination);
		await ctx.resume();
		osc.start();
		setTimeout(() => {
			osc.stop && osc.stop();
			osc.disconnect && osc.disconnect();
			panner.disconnect && panner.disconnect();
		}, duration + 200);
		return;
	})();
}

export function splitCompressMergeTap(args: EventData) {
	(async () => {
		try {
			const page = (<any>args.object).page as Page;
			const demo = page.bindingContext as DemoSharedAudioContext;
			const duration = 2.4;
			await demo.initAudio();
			const ctx = demo.ctx as AudioContext;
			if (!ctx) return;
			if (demo.source) demo.stop();

			const sampleRate = ctx.sampleRate || 44100;
			const length = Math.max(1, Math.floor(sampleRate * duration));
			const buffer = ctx.createBuffer({ length, numberOfChannels: 2, sampleRate });
			const leftNative = buffer.getChannelData(0);
			const rightNative = buffer.getChannelData(1);
			if (!leftNative || !rightNative) return;
			const left = leftNative;
			const right = rightNative;

			for (let i = 0; i < length; i++) {
				const t = i / sampleRate;
				const fadeIn = Math.min(1, i / (sampleRate * 0.02));
				const fadeOut = Math.min(1, (length - i) / (sampleRate * 0.05));
				const envelope = Math.min(fadeIn, fadeOut);
				left[i] = envelope * (Math.sin(2 * Math.PI * 220 * t) * 0.92 + Math.sin(2 * Math.PI * 440 * t) * 0.32);
				right[i] = envelope * Math.sin(2 * Math.PI * 660 * t) * 0.24;
			}

			const src = ctx.createBufferSource();
			src.buffer = buffer;

			const splitter = ctx.createChannelSplitter({ numberOfOutputs: 2 });
			const compressor = ctx.createDynamicsCompressor({
				threshold: -36,
				knee: 24,
				ratio: 12,
				attack: 0.003,
				release: 0.2,
			});
			const merger = ctx.createChannelMerger({ numberOfInputs: 2 });

			const now = typeof ctx.currentTime === 'number' ? ctx.currentTime : Date.now() / 1000;
			compressor.threshold.setValueAtTime(-36, now);
			compressor.threshold.linearRampToValueAtTime(-24, now + duration * 0.75);
			compressor.knee.setValueAtTime(24, now);
			compressor.ratio.setValueAtTime(12, now);
			compressor.attack.setValueAtTime(0.003, now);
			compressor.release.setValueAtTime(0.2, now);

			src.connect(splitter);
			splitter.connect(compressor, 0, 0);
			compressor.connect(merger, 0, 0);
			splitter.connect(merger, 1, 1);
			merger.connect(demo.gainNode || ctx.destination);

			const cleanup = () => {
				try {
					src.disconnect && src.disconnect();
				} catch (e) {}
				try {
					splitter.disconnect && splitter.disconnect();
				} catch (e) {}
				try {
					compressor.disconnect && compressor.disconnect();
				} catch (e) {}
				try {
					merger.disconnect && merger.disconnect();
				} catch (e) {}
			};

			const ok = await demo.startSourceSafe(src);
			if (!ok) {
				cleanup();
				console.warn('splitCompressMergeTap: source failed to start');
				return;
			}

			demo.source = src;
			demo.isPlaying = true;
			src.onended = () => {
				cleanup();
				demo.source = null;
				demo.isPlaying = false;
				demo.stopVisualizer();
			};

			if (demo.analyser && demo.visualizerCanvas && !demo.rafId) demo.startVisualizer();
		} catch (error) {
			console.warn('splitCompressMergeTap failed:', error);
		}
		return;
	})();
}

export function testPanLeft(args: EventData) {
	(async () => {
		const page = (<any>args.object).page as Page;
		const demo = page.bindingContext as DemoSharedAudioContext;
		const duration = 500;
		await demo.initAudio();
		const ctx = demo.ctx as AudioContext;
		const now = typeof ctx.currentTime === 'number' ? ctx.currentTime : Date.now() / 1000;
		const osc = ctx.createOscillator({ type: 'sine', frequency: 440 });
		const panner: any = ctx.createPanner ? ctx.createPanner() : ctx.createStereoPanner ? ctx.createStereoPanner() : null;
		if (!panner) {
			console.warn('testPanLeft: panner not available on this platform');
			return;
		}
		if (panner.positionX) panner.positionX.setValueAtTime(-1, now);
		else if (panner.pan) panner.pan.setValueAtTime(-1, now);
		osc.connect(panner);
		panner.connect(demo.gainNode || ctx.destination);
		await ctx.resume();
		osc.start();
		setTimeout(() => {
			osc.stop && osc.stop();
			osc.disconnect && osc.disconnect();
			panner.disconnect && panner.disconnect();
		}, duration + 50);
		return;
	})();
}

export function testPanRight(args: EventData) {
	(async () => {
		const page = (<any>args.object).page as Page;
		const demo = page.bindingContext as DemoSharedAudioContext;
		const duration = 500;
		await demo.initAudio();
		const ctx = demo.ctx as AudioContext;
		const now = typeof ctx.currentTime === 'number' ? ctx.currentTime : Date.now() / 1000;
		const osc = ctx.createOscillator({ type: 'sine', frequency: 440 });
		const panner: any = ctx.createPanner ? ctx.createPanner() : ctx.createStereoPanner ? ctx.createStereoPanner() : null;
		if (!panner) {
			console.warn('testPanRight: panner not available on this platform');
			return;
		}
		if (panner.positionX) panner.positionX.setValueAtTime(1, now);
		else if (panner.pan) panner.pan.setValueAtTime(1, now);
		osc.connect(panner);
		panner.connect(demo.gainNode || ctx.destination);
		await ctx.resume();
		osc.start();
		setTimeout(() => {
			osc.stop && osc.stop();
			osc.disconnect && osc.disconnect();
			panner.disconnect && panner.disconnect();
		}, duration + 50);
		return;
	})();
}

export function openNativeDemoTap(args: EventData) {}

export class DemoModel extends DemoSharedAudioContext {}
