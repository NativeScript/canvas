import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class LinearGradient extends SVGItem {
	_views: any[];
	x1: any = '0%';
	y1: any = '0%';
	x2: any = '100%';
	y2: any = '0%';

	gradientTransform: string;

	constructor() {
		super();
		this._dom = new DOMParser().parseFromString('<linear-gradient></linear-gradient>');
	}
}
