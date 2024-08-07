import { View, Property } from '@nativescript/core';

export class Source extends View {
	public src: string;
	public type: string;
}

export const srcProperty = new Property<Source, string>({
	name: 'src',
});

srcProperty.register(Source);

export const typeProperty = new Property<Source, string>({
	name: 'type',
});

typeProperty.register(Source);
