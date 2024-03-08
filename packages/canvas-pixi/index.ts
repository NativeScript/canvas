require('@nativescript/canvas-polyfill');
import * as Pixii from 'pixi.js';

let PIXI = Pixii;
import {Screen} from '@nativescript/core';

// import * as filters from 'pixi-filters';
// PIXI.filters = { ...(PIXI.filters || {}), ...filters };


(global as any).PIXI = (global as any).window.PIXI = (global as any).PIXI || PIXI;

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
		const width = props.width || clientWidth;
		const height = props.height || clientHeight;


		super({
			...props,
			resolution: Screen.mainScreen.scale,
			context,
			view,
			width, 
			height
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

export default PIXI;
