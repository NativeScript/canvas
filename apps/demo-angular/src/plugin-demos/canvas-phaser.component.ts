import { Component, NgZone } from '@angular/core';
import { DemoSharedCanvasPhaser } from '@demo/shared';
import {} from '@nativescript/canvas-phaser';

@Component({
	selector: 'demo-canvas-phaser',
	templateUrl: 'canvas-phaser.component.html',
})
export class CanvasPhaserComponent {
	demoShared: DemoSharedCanvasPhaser;

	constructor(private _ngZone: NgZone) {}

	ngOnInit() {
		this.demoShared = new DemoSharedCanvasPhaser();
	}
}
