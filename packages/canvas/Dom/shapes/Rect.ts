import { colorProperty, Property, booleanConverter } from '@nativescript/core';
import { Group } from '../Group';
import { Paint } from '../Paint';

export const xProperty = new Property<Rect, number>({
	name: 'x',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export const yProperty = new Property<Rect, number>({
	name: 'y',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export const widthProperty = new Property<Rect, number>({
	name: 'width',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export const heightProperty = new Property<Rect, number>({
	name: 'height',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export class Rect extends Paint {
	x: number;
	y: number;
	width: number;
	height: number;

	draw() {
		const override_color = (this.parent as any)._overrideColor;
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
		context.rect(this.x, this.y, this.width, this.height);
		super.draw();
	}
}

xProperty.register(Rect);
yProperty.register(Rect);
