import { Property } from '@nativescript/core';
import { Paint } from '../Paint';
import { Path2D } from '../../Canvas2D';
import { parsePoint } from '../point';

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
	p1?: { x: number; y: number };
	p2?: { x: number; y: number };

	draw() {
		const p1 = parsePoint(this.p1);
		const p2 = parsePoint(this.p2);
		if (!p1 || !p2) return;

		// A line has no area to fill: like Skia's drawLine it is stroked whatever its paintStyle.
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
		const line = new Path2D();
		line.moveTo(p1.x, p1.y);
		line.lineTo(p2.x, p2.y);
		context.strokeStyle = this._getColor();
		context.lineWidth = this._getStrokeWidth();
		context.lineJoin = this._getStrokeJoin();
		context.stroke(line);
	}
}

p1Property.register(Line);
p2Property.register(Line);
