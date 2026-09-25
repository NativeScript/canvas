import { Property } from '@nativescript/core';
import { SVGItem } from './SVGItem';

export const pointsProperty = new Property<Polygon, any>({
	name: 'points',
});
import { createSvgElement } from '../NativeNode';

export class Polygon extends SVGItem {
	points: any;

	constructor() {
		super();
		this.__domElement = createSvgElement('polygon');
	}
}

pointsProperty.register(Polygon);
