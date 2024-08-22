import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class Stop extends SVGItem {
	offset: any;

	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<stop></stop>', 'image/svg+xml').documentElement;
	}
}
