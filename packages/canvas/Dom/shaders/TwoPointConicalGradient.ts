import { Property } from '@nativescript/core';
import { Gradients } from './Gradients';

const startProperty = new Property<TwoPointConicalGradient, { x: number; y: number }>({
	name: 'start',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

const startRProperty = new Property<TwoPointConicalGradient, number>({
	name: 'startR',
	valueConverter: parseFloat,
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

const endProperty = new Property<TwoPointConicalGradient, { x: number; y: number }>({
	name: 'end',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

const endRProperty = new Property<TwoPointConicalGradient, number>({
	name: 'endR',
	valueConverter: parseFloat,
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

const colorsProperty = new Property<TwoPointConicalGradient, string[]>({
	name: 'colors',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export class TwoPointConicalGradient extends Gradients {
	start: { x: number; y: number };
	end: { x: number; y: number };
	colors: string[];
	startR: number;
	endR: number;

	_getColor() {
		const ctx = this._canvas.getContext('2d');
		const gradient = ctx.createRadialGradient(this.start.x, this.start.y, this.startR, this.end.x, this.end.y, this.endR);

		for (let i = 0; i < this.colors.length; i++) {
			gradient.addColorStop(i, this.colors[i]);
		}

		return gradient;
	}

	draw(): void {
		const ctx = this._canvas.getContext('2d');
		const gradient = this._getColor();
		if (this._getPaintStyle() === 'stroke') {
			ctx.strokeStyle = gradient;
		} else {
			ctx.fillStyle = gradient;
		}
	}
}

startProperty.register(TwoPointConicalGradient);
endProperty.register(TwoPointConicalGradient);
colorsProperty.register(TwoPointConicalGradient);
startRProperty.register(TwoPointConicalGradient);
endRProperty.register(TwoPointConicalGradient);
