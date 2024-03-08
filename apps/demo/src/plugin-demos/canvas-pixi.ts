import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedCanvasPixi } from '@demo/shared';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedCanvasPixi {}
