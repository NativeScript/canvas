import { Property } from '@nativescript/core';
import { SVGItem } from './SVGItem';
import { CanvasGradient, Path2D } from '../Canvas2D';

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
}

cxProperty.register(Circle);
cyProperty.register(Circle);
