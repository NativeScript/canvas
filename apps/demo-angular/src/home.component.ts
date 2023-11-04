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
		name: 'canvas'
	},
	{
		name: 'canvas-babylon'
	},
	{
		name: 'canvas-chartjs'
	},
	{
		name: 'canvas-media'
	},
	{
		name: 'canvas-phaser'
	},
	{
		name: 'canvas-phaser-ce'
	},
	{
		name: 'canvas-pixi'
	},
	{
		name: 'canvas-polyfill'
	},
	{
		name: 'canvas-three'
	}
];
}