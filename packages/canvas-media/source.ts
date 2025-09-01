import { View, Property } from '@nativescript/core';
const isSource_ = Symbol('[[source]]');

export class Source extends View {
	public src: string;
	public type: string;
	[isSource_] = true;

	static [Symbol.hasInstance](obj) {
		if (obj) {
			if (obj[isSource_]) {
				return true;
			}
			if ('src' in obj && 'type' in obj) {
				return true;
			}
		}
		return false;
	}
}

export const srcProperty = new Property<Source, string>({
	name: 'src',
});

export const typeProperty = new Property<Source, string>({
	name: 'type',
});

srcProperty.register(Source);

typeProperty.register(Source);
