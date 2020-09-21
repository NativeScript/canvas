import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedCanvasThree } from '@demo/shared';
import {} from '@nativescript/canvas-three';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedCanvasThree {}
