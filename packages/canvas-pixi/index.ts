require('@nativescript/canvas-polyfill');
import * as Pixii from 'pixi.js';

export class TNSPIXIApplication extends Pixii.Application {
	constructor({ canvas, ...props }) {
		console.log('ab');
		(global as any).PIXI = (global as any).window.PIXI = (global as any).PIXI || Pixii;
		console.log('aaaa');
		super({
			view: canvas,
			width: canvas.width || undefined,
			height: canvas.height || undefined,
			...props,
		});
	}
}
