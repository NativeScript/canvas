import { Component, NgZone } from '@angular/core';
import { DemoSharedCanvasBabylon } from '@demo/shared';
import {} from '@nativescript/canvas-babylon';

@Component({
	selector: 'demo-canvas-babylon',
	templateUrl: 'canvas-babylon.component.html',
})
export class CanvasBabylonComponent {
	demoShared: DemoSharedCanvasBabylon;

	constructor(private _ngZone: NgZone) {}

	ngOnInit() {
		this.demoShared = new DemoSharedCanvasBabylon();
	}
}
