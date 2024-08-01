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
			clientWidth = context.canvas.width;
			clientHeight = context.canvas.height;
		}
		if (view) {
			clientWidth = view.width;
			clientHeight = view.height;
		}
		if (!view) {
			view = context.canvas.toHTMLCanvas();
		}
		const width = props.width || clientWidth;
		const height = props.height || clientHeight;

		//	PIXI.settings.RESOLUTION = 1;

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
