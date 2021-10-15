import { Component, NgZone } from '@angular/core';
import { DemoSharedCanvasPhaserCe } from '@demo/shared';

@Component({
	selector: 'demo-canvas-phaser-ce',
	templateUrl: 'canvas-phaser-ce.component.html',
})
export class CanvasPhaserCeComponent {
	demoShared: DemoSharedCanvasPhaserCe;

	constructor(private _ngZone: NgZone) {}

	ngOnInit() {
		this.demoShared = new DemoSharedCanvasPhaserCe();
	}
}
