import { Property } from '@nativescript/core';
import { SVGItem } from './SVGItem';

export const pointsProperty = new Property<Polyline, any>({
	name: 'points',
});
import { createSvgElement } from '../NativeNode';

export class Polyline extends SVGItem {
	points: any;

	constructor() {
		super();
		this.__domElement = createSvgElement('polyline');
	}
}

pointsProperty.register(Polyline);
