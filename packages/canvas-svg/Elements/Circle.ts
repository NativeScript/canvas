import { Property } from '@nativescript/core';
import { SVGItem } from './SVGItem';
import { createSvgElement } from '../NativeNode';
export const cxProperty = new Property<Circle, any>({
	name: 'cx',
});

export const cyProperty = new Property<Circle, any>({
	name: 'cy',
});

export const rProperty = new Property<Circle, any>({
	name: 'r',
});

export class Circle extends SVGItem {
	cx: any;
	cy: any;
	r: any;
	constructor() {
		super();
		this.__domElement = createSvgElement('circle');
	}
}

cxProperty.register(Circle);
cyProperty.register(Circle);
