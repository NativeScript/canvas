import { Property } from '@nativescript/core';
import { SVGItem } from './SVGItem';

export const dProperty = new Property<Path, string>({
	name: 'd',
});
import { DOMParser } from '@xmldom/xmldom';

export class Path extends SVGItem {
	public d: string;

	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<path></path>', 'image/svg+xml').documentElement;
	}
}

dProperty.register(Path);
