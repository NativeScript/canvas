import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class Pattern extends SVGItem {
	constructor() {
		super();
		this._dom = new DOMParser().parseFromString('<pattern></pattern>');
	}
}
