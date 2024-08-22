import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class ClipPath extends SVGItem {
	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<clip-path></clip-path>', 'image/svg+xml').documentElement;
	}
}
