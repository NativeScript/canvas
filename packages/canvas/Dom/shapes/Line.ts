import { colorProperty, Property, booleanConverter } from '@nativescript/core';
import { Group } from '../Group';
import { Paint } from '../Paint';

export const p1Property = new Property<Line, number>({
	name: 'p1',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export const p2Property = new Property<Line, number>({
	name: 'p2',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export class Line extends Paint {
	p1: number;
	p2: number;

	draw() {
		if (this.p1 === undefined || this.p2 === undefined) {
			return;
		}
		const override_color = (this.parent as any)._overrideColor;
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
		const line = new Path2D();
		line.lineTo(this.p1, this.p2);
		const style = this.paintStyle;
		if (style === 'fill') {
			context.fillStyle = this.color.hex;
			context.fill(line);
		} else if (style === 'stroke') {
			context.strokeStyle = this.color.hex;
			context.stroke(line);
		}
	}
}

p1Property.register(Line);
p2Property.register(Line);
