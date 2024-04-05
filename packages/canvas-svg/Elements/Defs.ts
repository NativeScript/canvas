import { AddChildFromBuilder } from '@nativescript/core';
import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class Defs extends SVGItem {
	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<def></defs>', 'image/svg+xml').documentElement;
	}
}
