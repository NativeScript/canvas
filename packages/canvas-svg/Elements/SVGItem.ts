import { View } from '@nativescript/core';

export class SVGItem extends View {
	_dom: Document;
	_attached: boolean = false;

	//@ts-ignore
	get width() {
		return this._dom.documentElement.getAttribute('width') as any;
	}

	//@ts-ignore
	get height() {
		return this._dom.documentElement.getAttribute('height') as any;
	}
}
