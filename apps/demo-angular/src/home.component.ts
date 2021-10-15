import { Component } from '@angular/core';
import { RouterExtensions } from '@nativescript/angular';
@Component({
	selector: 'demo-home',
	templateUrl: 'home.component.html',
})
export class HomeComponent {

	constructor(private router: RouterExtensions){}

	demos = [
		{
			name: 'canvas',
		},
		{
			name: 'canvas-babylon',
		},
		{
			name: 'canvas-media',
		},
		{
			name: 'canvas-phaser',
		},
		{
			name: 'canvas-phaser-ce',
		},
		{
			name: 'canvas-pixi',
		},
		{
			name: 'canvas-polyfill',
		},
		{
			name: 'canvas-three',
		},
	];

	onTap(event){
		const item = this.demos[event.index];
		this.router.navigate(['/' + item.name]);
	}
}
