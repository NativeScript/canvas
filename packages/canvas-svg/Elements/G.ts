import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class G extends SVGItem {
	transform: string;
	x: any;
	y: any;

	constructor() {
		super();
		this._dom = new DOMParser().parseFromString('<g></g>');
	}
}
