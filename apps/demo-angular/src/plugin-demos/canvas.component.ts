import { Component, NgZone } from '@angular/core';
import { DemoSharedCanvas } from '@demo/shared';

@Component({
	selector: 'demo-canvas',
	templateUrl: 'canvas.component.html',
})
export class CanvasComponent {
	demoShared: DemoSharedCanvas;

	constructor(private _ngZone: NgZone) {}

	ngOnInit() {
		this.demoShared = new DemoSharedCanvas();
	}
}
