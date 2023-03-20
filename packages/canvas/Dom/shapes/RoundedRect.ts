import { colorProperty, Property, booleanConverter } from '@nativescript/core';
import { Group } from '../Group';
import { Paint } from '../Paint';

const xProperty = new Property<RoundedRect, number>({
	name: 'x',
	valueConverter(value) {
		return parseFloat(value);
	},
});

const yProperty = new Property<RoundedRect, number>({
	name: 'y',
	valueConverter(value) {
		return parseFloat(value);
	},
});

const widthProperty = new Property<RoundedRect, number>({
	name: 'width',
	valueConverter(value) {
		return parseFloat(value);
	},
});

const heightProperty = new Property<RoundedRect, number>({
	name: 'height',
	valueConverter(value) {
		return parseFloat(value);
	},
});

const rProperty = new Property<RoundedRect, number>({
	name: 'r',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export class RoundedRect extends Paint {
	x: number;
	y: number;
	width: number;
	height: number;
	r: number;
	draw() {
		const override_color = (this.parent as any)._overrideColor;
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
		context.roundRect(this.x, this.y, this.width, this.height, this.r);
		super.draw();
	}
}

xProperty.register(RoundedRect);
yProperty.register(RoundedRect);
rProperty.register(RoundedRect);
