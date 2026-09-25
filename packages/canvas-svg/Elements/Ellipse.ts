import { Property } from '@nativescript/core';
import { SVGItem } from './SVGItem';

export const cxProperty = new Property<Ellipse, any>({
	name: 'cx',
});
export const cyProperty = new Property<Ellipse, any>({
	name: 'cy',
});
export const rxProperty = new Property<Ellipse, any>({
	name: 'rx',
});
export const ryProperty = new Property<Ellipse, any>({
	name: 'ry',
});
import { createSvgElement } from '../NativeNode';

export class Ellipse extends SVGItem {
	cx: any;
	cy: any;
	rx: any;
	ry: any;

	constructor() {
		super();
		this.__domElement = createSvgElement('ellipse');
	}
}

cxProperty.register(Ellipse);
cyProperty.register(Ellipse);
rxProperty.register(Ellipse);
ryProperty.register(Ellipse);
