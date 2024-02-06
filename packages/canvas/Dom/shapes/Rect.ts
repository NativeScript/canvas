import { colorProperty, Property, booleanConverter, ViewBase } from '@nativescript/core';
import { Group } from '../Group';
import { Paint } from '../Paint';

export const xProperty = new Property<Rect, number>({
	name: 'x',
	valueConverter(value) {
		return parseFloat(value);
	},
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export const yProperty = new Property<Rect, number>({
	name: 'y',
	valueConverter(value) {
		return parseFloat(value);
	},
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export const widthProperty = new Property<Rect, number>({
	name: 'width',
	valueConverter(value) {
		return parseFloat(value);
	},
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export const heightProperty = new Property<Rect, number>({
	name: 'height',
	valueConverter(value) {
		return parseFloat(value);
	},
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export class Rect extends Paint {
	x: number;
	y: number;
	width: number;
	height: number;

	_children: Paint[] = [];

	draw() {
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
		context.rect(this.x, this.y, this.width, this.height);
		if (this._children.length > 0) {
			for (const child of this._children) {
				const color = child._getColor();
				const style = child._getPaintStyle();
				switch (style) {
					case 'fill':
						context.fillStyle = color;
						context.fill();
						break;
					case 'stroke':
						context.strokeStyle = color;
						context.lineWidth = child._getStrokeWidth();
						context.lineJoin = child._getStrokeJoin();
						context.stroke();
						break;
				}
			}
		} else {
			super.draw();
		}
	}

	_addViewToNativeVisualTree(view: ViewBase, atIndex?: number): boolean {
		if (view === this._canvas) {
			this.nativeView.addView(this._canvas.nativeView);
			return true;
		} else if (view instanceof Paint) {
			view._canvas = this._canvas;
			this._children.push(view);
		}
		return false;
	}
}

xProperty.register(Rect);
yProperty.register(Rect);
widthProperty.register(Rect);
heightProperty.register(Rect);
