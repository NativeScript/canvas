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
	const testPath = __IOS__ ? '~/assets/file-assets/audio/sine441stereo.mp3' : '~/assets/file-assets/audio/gs-16b-1c-44100hz.wav';
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
