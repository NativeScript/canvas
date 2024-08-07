import { AddChildFromBuilder, booleanConverter, ContentView, Property, CSSType } from '@nativescript/core';

@CSSType('Audio')
export abstract class AudioBase extends ContentView implements AddChildFromBuilder {
	public controls: boolean;
	public loop: boolean;
	public autoPlay: boolean;
}

export const controlsProperty = new Property<AudioBase, boolean>({
	name: 'controls',
	valueConverter: booleanConverter,
	defaultValue: false,
});
controlsProperty.register(AudioBase);

export const loopProperty = new Property<AudioBase, boolean>({
	name: 'loop',
	valueConverter: booleanConverter,
	defaultValue: false,
});

loopProperty.register(AudioBase);

export const autoplayProperty = new Property<AudioBase, boolean>({
	name: 'autoplay',
	valueConverter: booleanConverter,
	defaultValue: false,
});

autoplayProperty.register(AudioBase);
