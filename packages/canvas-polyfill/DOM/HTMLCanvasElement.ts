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
			this._canvas = Canvas.createCustomView();
		}
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
		return this._canvas.toDataURL(type, encoderOptions);
	}

	getContext(contextType, contextOptions) {
		return this._canvas.getContext(contextType, contextOptions);
	}
}
