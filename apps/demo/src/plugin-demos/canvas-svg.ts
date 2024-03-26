import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedCanvasSvg } from '@demo/shared';
import { } from '@nativescript/canvas-svg';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedCanvasSvg {
	
}
