import { colorProperty, Property, booleanConverter } from '@nativescript/core';
import { Group } from '../Group';
import { Paint } from '../Paint';

export const cxProperty = new Property<Circle, number>({
	name: 'cx',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export const cyProperty = new Property<Circle, number>({
	name: 'cy',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export const rProperty = new Property<Circle, number>({
	name: 'r',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export class Circle extends Paint {
	cx: number;
	cy: number;
	r: number;

	draw() {
		const override_color = (this.parent as any)._overrideColor;
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
		context.arc(this.cx, this.cy, this.r, 0, 2 * Math.PI);
		super.draw();
	}
}

cxProperty.register(Circle);
cyProperty.register(Circle);
rProperty.register(Circle);
