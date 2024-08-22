import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class Use extends SVGItem {
	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<use></use>', 'image/svg+xml').documentElement;
	}
}
