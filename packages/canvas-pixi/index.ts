require('@nativescript/canvas-polyfill');
import * as Pixii from 'pixi.js';
import { Screen } from '@nativescript/core';
let PIXI = Pixii;

// import * as filters from 'pixi-filters';
class NSCPIXIApplication extends Pixii.Application {
	constructor({ context, view, ...props }) {
		let clientWidth = 300;
		let clientHeight = 150;
		if (context) {
			clientWidth = context.canvas.clientWidth;
			clientHeight = context.canvas.clientHeight;
		}
		if (view) {
			clientWidth = view.clientWidth;
			clientHeight = view.clientHeight;
		}
		if (!view) {
			view = document.createElement('canvas');
			view.nativeElement = context.canvas;
		}
		const width = props.width || clientWidth //* Screen.mainScreen.scale;
		const height = props.height || clientHeight //* Screen.mainScreen.scale;

		PIXI.settings.RESOLUTION = 1;


		console.log(width, height, Screen.mainScreen.widthDIPs,  Screen.mainScreen.heightDIPs, Screen.mainScreen.widthPixels,  Screen.mainScreen.heightPixels,)
		super({
			...props,
			resolution: 1,
			view,
			width,
			height,
		});
	}
}

if (!(PIXI.Application instanceof NSCPIXIApplication)) {
	PIXI.Assets.setPreferences({ preferWorkers: false });
	PIXI = {
		...PIXI,
		Application: NSCPIXIApplication as never,
	};
}

(global as any).PIXI = (global as any).window.PIXI = (global as any).PIXI || PIXI;

export default PIXI;
