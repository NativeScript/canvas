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
import { DOMParser } from '@xmldom/xmldom';

export class Ellipse extends SVGItem {
	cx: any;
	cy: any;
	rx: any;
	ry: any;

	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<ellipse></ellipse>', 'image/svg+xml').documentElement;
	}
}

cxProperty.register(Ellipse);
cyProperty.register(Ellipse);
rxProperty.register(Ellipse);
ryProperty.register(Ellipse);
