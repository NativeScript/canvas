import { Component, NgZone } from '@angular/core';
import { DemoSharedAudioContext } from '@demo/shared';
import {} from '@nativescript/audio-context';

@Component({
	selector: 'demo-audio-context',
	templateUrl: 'audio-context.component.html',
})
export class AudioContextComponent {
	demoShared: DemoSharedAudioContext;

	constructor(private _ngZone: NgZone) {}

	ngOnInit() {
		this.demoShared = new DemoSharedAudioContext();
	}
}
