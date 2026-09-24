import { Property } from '@nativescript/core';
import { SVGItem } from './SVGItem';
import { createSvgElement } from '../NativeNode';

export const xProperty = new Property<TSpan, any>({
	name: 'x',
});

export const yProperty = new Property<TSpan, any>({
	name: 'y',
});

export const dxProperty = new Property<TSpan, any>({
	name: 'dx',
});

export const dyProperty = new Property<TSpan, any>({
	name: 'dy',
});

export class TSpan extends SVGItem {
	x: any;
	y: any;
	dx: any;
	dy: any;
	text: string;

	constructor() {
		super();
		this.__domElement = createSvgElement('tspan');
	}
}

xProperty.register(TSpan);
yProperty.register(TSpan);
dxProperty.register(TSpan);
dyProperty.register(TSpan);
