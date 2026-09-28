import { AddChildFromBuilder } from '@nativescript/core';
import { SVGItem } from './SVGItem';
import { createSvgElement } from '../NativeNode';

export class Defs extends SVGItem {
	constructor() {
		super();
		this.__domElement = createSvgElement('defs');
	}
}
