import { colorProperty, Property, booleanConverter } from '@nativescript/core';
import { Group } from '../Group';
import { Paint } from '../Paint';

export const pointsProperty = new Property<Points, { x: number; y: number }[]>({
	name: 'points',
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
		const override_color = (this.parent as any)._overrideColor;
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
		context.strokeStyle = this.color.hex;
		(context as any).drawPoints(this.mode ?? 'points', this.points);
	}
}

pointsProperty.register(Points);
modeProperty.register(Points);
