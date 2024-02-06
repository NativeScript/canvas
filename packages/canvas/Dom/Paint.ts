import { Color, LayoutBase, Property } from '@nativescript/core';
import { Canvas } from '../Canvas';
import { Dom } from './Dom';

export const paintStyleProperty = new Property<Paint, 'fill' | 'stroke'>({
	name: 'paintStyle',
});

export const strokeWidthProperty = new Property<Paint, number>({
	name: 'strokeWidth',
});

export const strokeJoinProperty = new Property<Paint, 'bevel' | 'miter' | 'round'>({
	name: 'strokeJoin',
});

const defaultColor = new Color('black');
export class Paint extends LayoutBase {
	strokeWidth: number;
	strokeJoin: 'bevel' | 'miter' | 'round';
	_canvas: Canvas;
	_addCanvas(canvas: Canvas) {
		this._canvas = canvas;
	}

	paintStyle: 'fill' | 'stroke';

	_paintStyleDirty = false;

	_inGroup = false;

	[paintStyleProperty.setNative](value) {
		this._paintStyleDirty = true;
	}

	invalidate() {
		const parent = this.parent as Dom;
		if (parent != null) {
			parent._dirty?.();
		}
	}

	_getStrokeJoin() {
		return (this.parent as any)?.strokeJoin ?? this.strokeJoin ?? 'miter';
	}

	_getStrokeWidth() {
		return (this.parent as any)?.strokeWidth ?? this.strokeWidth ?? 1;
	}
	_getPaintStyle() {
		const paintStyle = this.paintStyle;
		if (paintStyle === 'stroke') {
			return paintStyle;
		}
		if (this._inGroup) {
			return (this.parent as any)?._getPaintStyle?.();
		}
		return 'fill';
	}

	_getColor() {
		const color = this.color ?? defaultColor;
		const hex = color.hex;
		if (color.name !== 'black' && hex !== '#000000') {
			return hex;
		}
		return (this.parent as any)?.color?.hex ?? hex;
	}

	draw() {
		const color = this._getColor();

		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;

		const style = this._getPaintStyle();

		context.closePath();

		if (style === 'fill') {
			context.fillStyle = color;
			context.fill();
		} else if (style === 'stroke') {
			context.lineWidth = this._getStrokeWidth();
			context.strokeStyle = color;
			context.stroke();
		}

		context.beginPath();
	}
}

paintStyleProperty.register(Paint);
strokeWidthProperty.register(Paint);
strokeJoinProperty.register(Paint);
