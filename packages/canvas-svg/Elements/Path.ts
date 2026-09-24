import { Property } from '@nativescript/core';
import { SVGItem } from './SVGItem';

export const dProperty = new Property<Path, string>({
	name: 'd',
});
import { createSvgElement } from '../NativeNode';

export class Path extends SVGItem {
	public d: string;

	constructor() {
		super();
		this.__domElement = createSvgElement('path');
	}
}

dProperty.register(Path);
