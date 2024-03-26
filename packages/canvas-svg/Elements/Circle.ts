import { Property } from '@nativescript/core';
import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';
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
	constructor(){
		super();
		this._dom = new DOMParser().parseFromString('<circle></circle>');
	}
}

cxProperty.register(Circle);
cyProperty.register(Circle);
