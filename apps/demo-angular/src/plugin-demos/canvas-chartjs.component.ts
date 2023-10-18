import { Component, NgZone } from '@angular/core';
import { DemoSharedCanvasChartjs } from '@demo/shared';
import { } from '@nativescript/canvas-chartjs';

@Component({
	selector: 'demo-canvas-chartjs',
	templateUrl: 'canvas-chartjs.component.html',
})
export class CanvasChartjsComponent {
  
  demoShared: DemoSharedCanvasChartjs;
  
	constructor(private _ngZone: NgZone) {}

  ngOnInit() {
    this.demoShared = new DemoSharedCanvasChartjs();
  }

}