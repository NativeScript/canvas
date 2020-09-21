import { Component, NgZone } from '@angular/core';
import { DemoSharedCanvasPixi } from '@demo/shared';
import {} from '@nativescript/canvas-pixi';

@Component({
	selector: 'demo-canvas-pixi',
	templateUrl: 'canvas-pixi.component.html',
})
export class CanvasPixiComponent {
	demoShared: DemoSharedCanvasPixi;

	constructor(private _ngZone: NgZone) {}

	ngOnInit() {
		this.demoShared = new DemoSharedCanvasPixi();
	}
}
