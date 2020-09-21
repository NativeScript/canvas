import {Observable, EventData, Page} from '@nativescript/core';
import {DemoSharedCanvasPhaserCe} from '@demo/shared';
import {} from '@nativescript/canvas-phaser-ce';

let page: Page;

export function navigatingTo(args: EventData) {
	page = <Page>args.object;
	page.bindingContext = new DemoModel();
	//page.on('navigatingFrom', navigatingFrom)
}

let fpsTimer;

export function loaded(args) {
	const canvas = args.object;
	page.bindingContext.canvasLoaded(args);
}

export function navigatingFrom(args) {
	console.log('navigatingFrom');
	if (fpsTimer) {
		clearInterval(fpsTimer)
		fpsTimer = undefined;
	}
}

export class DemoModel extends DemoSharedCanvasPhaserCe {
}
