import { colorProperty, Property, booleanConverter } from '@nativescript/core';
import { Group } from '../Group';
import { Paint } from '../Paint';
import { parsePoints } from '../point';

export const pointsProperty = new Property<Points, { x: number; y: number }[]>({
	name: 'points',
	valueConverter: parsePoints,
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export const modeProperty = new Property<Points, 'points' | 'lines' | 'polygon'>({
	name: 'mode',
	defaultValue: 'points',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export class Points extends Paint {
	points: { x: number; y: number }[];
	mode: 'points' | 'lines' | 'polygon';

	draw() {
		const points = parsePoints(this.points);
		if (points.length === 0) return;
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
		context.lineWidth = this._getStrokeWidth();
		context.lineJoin = this._getStrokeJoin();
		context.strokeStyle = this._getColor();
		(context as any).drawPoints(this.mode ?? 'points', points);
	}
}

pointsProperty.register(Points);
modeProperty.register(Points);
