import '@nativescript/canvas-polyfill';
import * as Pixii from 'pixi.js';

export class TNSPIXIApplication extends Pixii.Application {
	constructor({canvas, ...props}) {
		(global as any).PIXI = (global as any).window.PIXI = (global as any).PIXI || Pixii;
		super({
			view: canvas,
			...props
		});
	}
}
