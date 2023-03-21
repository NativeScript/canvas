import { Property } from '@nativescript/core';
import { Paint } from './Paint';

export const dxProperty = new Property<Shadow, number>({
	name: 'dx',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export const dyProperty = new Property<Shadow, number>({
	name: 'dy',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export const blurProperty = new Property<Shadow, number>({
	name: 'blur',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export class Shadow extends Paint {
	dx: number;
	dy: number;
	blur: number;
}

dxProperty.register(Shadow);
dyProperty.register(Shadow);
blurProperty.register(Shadow);
