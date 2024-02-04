import { colorProperty, Property, booleanConverter } from '@nativescript/core';
import { Paint } from '../Paint';
import { Path2D } from '../../Canvas2D';

export const p1Property = new Property<Line, { x: number; y: number }>({
	name: 'p1',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export const p2Property = new Property<Line, { x: number; y: number }>({
	name: 'p2',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export class Line extends Paint {
	p1: { x: number; y: number };
	p2: { x: number; y: number };

	draw() {
		if (this.p1 === undefined || this.p2 === undefined) {
			return;
		}
		const override_color = (this.parent as any)._overrideColor;
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
		const line = new Path2D();
		line.lineTo(this.p1.x, this.p1.y);
		line.lineTo(this.p2.x, this.p2.y);
		const color = this._getColor();
		const style = this._getPaintStyle();

		if (style === 'fill') {
			context.fillStyle = color;
			context.fill(line);
		} else if (style === 'stroke') {
			context.strokeStyle = color;
			context.lineWidth = this._getStrokeWidth();
			context.stroke(line);
		}
	}
}

p1Property.register(Line);
p2Property.register(Line);
