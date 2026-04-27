import { EventData, Page, Slider, knownFolders } from '@nativescript/core';
import { DemoSharedAudioContext } from '@demo/shared';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new DemoModel();
}

export function canvasReady(args: EventData) {
	const canvas = args.object as any;
	const page = canvas.page as Page;
	const ctx = page.bindingContext as any;
	if (ctx && ctx.initVisualizer) ctx.initVisualizer(canvas);
	setTimeout(() => {
		playSample(args);
	}, 6000);
}

export function playSample(args: EventData) {
	const page = (<any>args.object).page as Page;
	const ctx = page.bindingContext as any;
	const testPath = '~/assets/file-assets/audio/gs-16b-1c-44100hz.wav';
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

export class DemoModel extends DemoSharedAudioContext {}
