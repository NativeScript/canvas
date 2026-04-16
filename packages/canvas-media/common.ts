import { View, Property, AddChildFromBuilder, booleanConverter, ContentView } from '@nativescript/core';

export class MediaBase extends ContentView implements AddChildFromBuilder {
	canPlayType(type: string): '' | 'probably' | 'maybe' {
		return '';
	}
}

export const mutedProperty = new Property<MediaBase, boolean>({
	name: 'muted',
	valueConverter: booleanConverter,
	defaultValue: false,
});
mutedProperty.register(MediaBase);

export const controlsProperty = new Property<MediaBase, boolean>({
	name: 'controls',
	valueConverter: booleanConverter,
	defaultValue: false,
});
controlsProperty.register(MediaBase);

export const loopProperty = new Property<MediaBase, boolean>({
	name: 'loop',
});

loopProperty.register(MediaBase);

export const autoplayProperty = new Property<MediaBase, boolean>({
	name: 'autoplay',
	valueConverter: booleanConverter,
	defaultValue: false,
});

autoplayProperty.register(MediaBase);

export const playsinlineProperty = new Property<MediaBase, boolean>({
	name: 'playsinline',
	valueConverter: booleanConverter,
	defaultValue: false,
});

playsinlineProperty.register(MediaBase);

export const currentTimeProperty = new Property<MediaBase, number>({
	name: 'currentTime',
	defaultValue: 0,
});

currentTimeProperty.register(MediaBase);

export const durationProperty = new Property<MediaBase, number>({
	name: 'duration',
	defaultValue: NaN,
});

durationProperty.register(MediaBase);
