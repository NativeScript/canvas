import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class Symbol extends SVGItem {
	constructor() {
		super();
		this._dom = new DOMParser().parseFromString('<symbol></symbol>');
	}
}
