import { Component, NgZone } from '@angular/core';
import { DemoSharedCanvasMedia } from '@demo/shared';

@Component({
	selector: 'demo-canvas-media',
	templateUrl: 'canvas-media.component.html',
})
export class CanvasMediaComponent {
	demoShared: DemoSharedCanvasMedia;

	constructor(private _ngZone: NgZone) {}

	ngOnInit() {
		this.demoShared = new DemoSharedCanvasMedia();
	}
}
