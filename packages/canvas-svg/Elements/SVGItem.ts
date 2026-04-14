import { View } from '@nativescript/core';

export class SVGItem extends View {
	__domElement: any;
	_attached: boolean = false;

	//@ts-ignore
	get width() {
		return this.__domElement.getAttribute('width') as any;
	}

	//@ts-ignore
	get height() {
		return this.__domElement.getAttribute('height') as any;
	}
}
