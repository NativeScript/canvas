import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class Pattern extends SVGItem {
	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<pattern></pattern>', 'image/svg+xml').documentElement;
	}
}
