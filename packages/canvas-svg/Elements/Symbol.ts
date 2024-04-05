import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class Symbol extends SVGItem {
	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<symbol></symbol>', 'image/svg+xml').documentElement;
	}
}
