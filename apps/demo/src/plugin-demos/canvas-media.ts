import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedCanvasMedia } from '@demo/shared';
import {} from '@nativescript/canvas-media';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedCanvasMedia {}
