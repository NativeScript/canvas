import { View } from '@nativescript/core';
import { Property } from '@nativescript/core/ui/core/view';

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
