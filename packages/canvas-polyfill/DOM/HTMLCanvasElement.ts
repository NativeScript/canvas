import { Canvas } from '@nativescript/canvas';
import { Element } from './Element';

export class HTMLCanvasElement extends Element {
	constructor() {
		super('canvas');
		let canvas = undefined;
		if (arguments.length > 1) {
			canvas = arguments[1];
		}

		if (canvas instanceof Canvas) {
			this._canvas = canvas;
		} else {
			this._canvas = (Canvas as any).createCustomView(true);
		}

		this.__instanceType = 52;
	}

	set width(value) {
		this._canvas.width = value;
	}

	get width() {
		return this._canvas.width;
	}

	set height(value) {
		this._canvas.height = value;
	}

	get height() {
		return this._canvas.height;
	}

	toDataURL(type, encoderOptions) {
		console.log('toDataURL', type);
		return this._canvas.toDataURL(type, encoderOptions);
	}

	getContext(contextType, contextOptions) {
		const ctx = this._canvas.getContext(contextType, contextOptions);
		ctx.canvas = this;
		(this as any).__native__context = ctx;
		return ctx;
	}

}
