import { Property } from '@nativescript/core/ui/core/view';
import { SVGItem } from './SVGItem';

export const pointsProperty = new Property<Polyline, any>({
	name: 'points'
});
import { DOMParser } from '@xmldom/xmldom';

export class Polyline extends SVGItem {
	points: any;

	constructor() {
		super();
		this._dom = new DOMParser().parseFromString('<polyline></polyline>');
	}
}

pointsProperty.register(Polyline);
