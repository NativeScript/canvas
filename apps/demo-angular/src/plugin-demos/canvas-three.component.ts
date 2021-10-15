import { Component, NgZone } from '@angular/core';
import { DemoSharedCanvasThree } from '@demo/shared';

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

	canvasLoaded(event) {
		this.demoShared?.canvasLoaded?.(event);
	}

	loaded(event) {
		this.demoShared?.loaded?.(event);
	}

	unloaded(event) {
		this.demoShared?.unloaded?.(event);
	}
}
