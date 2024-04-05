import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class RadialGradient extends SVGItem {
	_views: any[];
	cx: any = '50%';
	cy: any = '50%';
	r: any = '50%';
	fx: any = '50%';
	fy: any = '50%';

	gradientTransform: string;

	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<radialGradient></radialGradient>', 'image/svg+xml').documentElement;
	}
}
