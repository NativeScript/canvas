import { Property } from '@nativescript/core';
import { SVGItem } from './SVGItem';

export const pointsProperty = new Property<Polyline, any>({
	name: 'points',
});
import { DOMParser } from '@xmldom/xmldom';

export class Polyline extends SVGItem {
	points: any;

	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<polyline></polyline>', 'image/svg+xml').documentElement;
	}
}

pointsProperty.register(Polyline);
