import { Property } from '@nativescript/core';
import { SVGItem } from './SVGItem';

export const pointsProperty = new Property<Polygon, any>({
	name: 'points',
});
import { DOMParser } from '@xmldom/xmldom';

export class Polygon extends SVGItem {
	points: any;

	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<polygon></polygon>', 'image/svg+xml').documentElement;
	}
}

pointsProperty.register(Polygon);
