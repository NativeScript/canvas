import {Node} from "./Node";
import {Canvas} from '@nativescript/canvas';

export class Element extends Node {
	private doc: any;
	private _classList: any;
	private _width: number;
	private _height: number;
	namespaceURI: any;
	nativeElement: any;

	constructor(tagName) {
		super(tagName.toUpperCase());

		this.doc = {
			body: {
				innerHTML: "",
			},
		};
		this._classList = new Set();
		if (tagName.toLowerCase() === 'canvas') {
			this._canvas = Canvas.createCustomView();
		}
	}

	get classList() {
		return this._classList;
	}

	get tagName() {
		return this.nodeName;
	}

	setAttribute() {
	}

	removeAttribute() {
	}

	setAttributeNS() {
	}

	removeAttributeNS() {
	}

	get clientWidth() {
		return this.innerWidth;
	}

	get clientHeight() {
		return this.innerHeight;
	}

	get offsetWidth() {
		return this.innerWidth;
	}

	get offsetHeight() {
		return this.innerHeight;
	}

	get innerWidth() {
		return this.width;
	}

	get innerHeight() {
		return this.height;
	}

	set width(value) {
		this._width = value as any;
		if (this._canvas) {
			this._canvas.width = value;
		}
	}

	get width() {
		if (this._canvas) {
			return this._canvas.width;
		}
		return this._width;
	}

	set height(value) {
		this._height = value as any;
		if (this._canvas) {
			this._canvas.height = value;
		}
	}

	get height() {
		if (this._canvas) {
			return this._canvas.height;
		}
		return this._height;
	}


	toDataURL(type, encoderOptions) {
		if (!this._canvas) {
			return "";
		}
		return this._canvas.toDataURL(type, encoderOptions);
	}


	getContext(contextType, contextOptions, context) {
		return this._canvas.getContext(contextType, contextOptions);
	}

	get ontouchstart() {
		return {};
	}
}

