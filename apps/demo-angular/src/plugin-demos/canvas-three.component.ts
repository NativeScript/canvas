import { Component, NgZone } from '@angular/core';
import { DemoSharedCanvasThree } from '@demo/shared';
import {} from '@nativescript/canvas-three';

@Component({
	selector: 'demo-canvas-three',
	templateUrl: 'canvas-three.component.html',
})
export class CanvasThreeComponent {
	demoShared: DemoSharedCanvasThree;

	constructor(private _ngZone: NgZone) {}

	ngOnInit() {
		this.demoShared = new DemoSharedCanvasThree();
	}
}
