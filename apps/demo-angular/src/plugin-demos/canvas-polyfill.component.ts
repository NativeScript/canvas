import { Component, NgZone } from '@angular/core';
import { DemoSharedCanvasPolyfill } from '@demo/shared';

@Component({
	selector: 'demo-canvas-polyfill',
	templateUrl: 'canvas-polyfill.component.html',
})
export class CanvasPolyfillComponent {
	demoShared: DemoSharedCanvasPolyfill;

	constructor(private _ngZone: NgZone) {}

	ngOnInit() {
		this.demoShared = new DemoSharedCanvasPolyfill();
	}
}
