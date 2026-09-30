import { Color, LayoutBase, Property } from '@nativescript/core';
import { Canvas } from '../Canvas';
import { Dom } from './Dom';

export const paintStyleProperty = new Property<Paint, 'fill' | 'stroke'>({
	name: 'paintStyle',
	valueChanged(target) {
		target.invalidate();
	},
});

export const strokeWidthProperty = new Property<Paint, number>({
	name: 'strokeWidth',
	valueConverter: parseFloat,
	valueChanged(target) {
		target.invalidate();
	},
});

export const strokeJoinProperty = new Property<Paint, 'bevel' | 'miter' | 'round'>({
	name: 'strokeJoin',
	valueChanged(target) {
		target.invalidate();
	},
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
		let parent = this.parent as any;
		while (parent && typeof parent._dirty !== 'function') {
			parent = parent.parent;
		}
		(parent as Dom)?._dirty();
	}

	// An element's own paint props win over those it inherits.
	_getStrokeJoin(): 'bevel' | 'miter' | 'round' {
		const parent = this.parent as any;
		return this.strokeJoin ?? parent?._getStrokeJoin?.() ?? parent?.strokeJoin ?? 'miter';
	}

	_getStrokeWidth(): number {
		if (Number.isFinite(this.strokeWidth)) {
			return this.strokeWidth;
		}
		const parent = this.parent as any;
		return parent?._getStrokeWidth?.() ?? (Number.isFinite(parent?.strokeWidth) ? parent.strokeWidth : 1);
	}

	_getPaintStyle(): 'fill' | 'stroke' {
		if (this.paintStyle === 'fill' || this.paintStyle === 'stroke') {
			return this.paintStyle;
		}
		if (this._inGroup) {
			return (this.parent as any)?._getPaintStyle?.() ?? 'fill';
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

		//context.closePath();

		context.globalAlpha = this.opacity;

		if (style === 'fill') {
			context.fillStyle = color;
			context.fill();
		} else if (style === 'stroke') {
			context.lineWidth = this._getStrokeWidth();
			context.strokeStyle = color;
			context.stroke();
		}

		//context.beginPath();
	}
}

paintStyleProperty.register(Paint);
strokeWidthProperty.register(Paint);
strokeJoinProperty.register(Paint);
