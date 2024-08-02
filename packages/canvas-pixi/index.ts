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
			clientWidth = context.canvas.width * Screen.mainScreen.scale;
			clientHeight = context.canvas.height * Screen.mainScreen.scale;
		}
		if (!view) {
			view = context.canvas.toHTMLCanvas();
		}

		if (view) {
			clientWidth = view.clientWidth;
			clientHeight = view.clientHeight;
		}

		view.width = view.clientWidth * Screen.mainScreen.scale;
		view.height = view.clientHeight * Screen.mainScreen.scale;

		const width = props.width || clientWidth;
		const height = props.height || clientHeight;

		//	PIXI.settings.RESOLUTION = 1;

		super({
			...props,
			resolution: Screen.mainScreen.scale,
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
