import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class Use extends SVGItem {
	constructor() {
		super();
		this._dom = new DOMParser().parseFromString('<use></use>');
	}
}
