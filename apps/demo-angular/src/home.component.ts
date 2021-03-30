import { Component } from '@angular/core';

@Component({
	selector: 'demo-home',
	templateUrl: 'home.component.html',
})
export class HomeComponent {
	demos = [
	{
		name: 'canvas'
	},
	{
		name: 'canvas-babylon'
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