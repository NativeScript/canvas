require('@nativescript/canvas-polyfill');
import * as Pixii from 'pixi.js';

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
		const width = props.width || clientWidth;
		const height = props.height || clientHeight;

		super({
			resolution: global.isAndroid ? window.devicePixelRatio : 1,
			...props,
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

(global as any).PIXI = (global as any).window.PIXI = (global as any).PIXI || PIXI;

export default PIXI;
