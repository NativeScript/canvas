import { LayoutBase, Property } from '@nativescript/core';
import { Canvas } from '../Canvas';

export const paintStyleProperty = new Property<Paint, 'fill' | 'stroke'>({
	name: 'paintStyle',
	defaultValue: 'fill',
});

export class Paint extends LayoutBase {
	_canvas: Canvas;
	_addCanvas(canvas: Canvas) {
		this._canvas = canvas;
	}

	paintStyle: 'fill' | 'stroke' = 'fill';

	draw() {
		const override_color = (this.parent as any)._overrideColor;
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;

		const style = this.paintStyle;
		if (style === 'fill') {
			context.fillStyle = this.color.hex;
			context.fill();
		} else if (style === 'stroke') {
			context.strokeStyle = this.color.hex;
			context.stroke();
		}
	}
}

paintStyleProperty.register(Paint);
