import { Property } from '@nativescript/core';
import { Gradients } from './Gradients';

const startProperty = new Property<LinearGradient, { x: number; y: number }>({
	name: 'start',
});

const endProperty = new Property<LinearGradient, { x: number; y: number }>({
	name: 'end',
});

const colorsProperty = new Property<LinearGradient, string[]>({
	name: 'colors',
});

export class LinearGradient extends Gradients {
	start: { x: number; y: number };
	end: { x: number; y: number };
	colors: string[];

	_getColor() {
		const ctx = this._canvas.getContext('2d');
		const gradient = ctx.createLinearGradient(this.start.x, this.start.y, this.end.x, this.end.y);

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

startProperty.register(LinearGradient);
endProperty.register(LinearGradient);
colorsProperty.register(LinearGradient);
