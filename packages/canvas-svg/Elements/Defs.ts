import { AddChildFromBuilder } from '@nativescript/core';
import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class Defs extends SVGItem {
	constructor() {
		super();
		this._dom = new DOMParser().parseFromString('<def></defs>');
	}
}
