import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class Rect extends SVGItem {
	x: any = 0;
	y: any = 0;
	rx: any = 0;
	ry: any = 0;

	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<rect></rect>', 'image/svg+xml').documentElement;
	}
}
