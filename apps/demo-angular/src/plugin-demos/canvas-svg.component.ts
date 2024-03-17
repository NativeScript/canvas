import { Component, NgZone } from '@angular/core';
import { DemoSharedCanvasSvg } from '@demo/shared';
import { } from '@nativescript/canvas-svg';

@Component({
	selector: 'demo-canvas-svg',
	templateUrl: 'canvas-svg.component.html',
})
export class CanvasSvgComponent {
  
  demoShared: DemoSharedCanvasSvg;
  
	constructor(private _ngZone: NgZone) {}

  ngOnInit() {
    this.demoShared = new DemoSharedCanvasSvg();
  }

}