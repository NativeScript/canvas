import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedCanvasPolyfill } from '@demo/shared';
import {} from '@nativescript/canvas-polyfill';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedCanvasPolyfill {}
