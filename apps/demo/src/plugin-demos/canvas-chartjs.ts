import { Observable, EventData, Page, Screen } from '@nativescript/core';
import { DemoSharedCanvasChartjs } from '@demo/shared';
import { NativeScriptPlatform } from '@nativescript/canvas-chartjs';
import { Chart } from 'chart.js';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedCanvasChartjs {
	ready(event) {
		const canvas = event.object;
		new Chart(canvas, {
			platform: NativeScriptPlatform,
			type: 'bar',
			data: {
				labels: ['Red', 'Blue', 'Yellow', 'Green', 'Purple', 'Orange'],
				datasets: [
					{
						label: '# of Votes',
						data: [12, 19, 3, 5, 2, 3],
						borderWidth: 1,
					},
				],
			},
			options: {
				responsive: false,
				scales: {
					y: {
						beginAtZero: true,
					},
				},
			},
		});
	}
}
