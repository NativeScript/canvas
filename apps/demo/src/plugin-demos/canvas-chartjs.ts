import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedCanvasChartjs } from '@demo/shared';
import { } from '@nativescript/canvas-chartjs';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedCanvasChartjs {
	
}
