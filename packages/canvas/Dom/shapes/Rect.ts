import { colorProperty, Property, booleanConverter, ViewBase } from '@nativescript/core';
import { Group } from '../Group';
import { Paint } from '../Paint';

export const xProperty = new Property<Rect, number>({
	name: 'x',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export const yProperty = new Property<Rect, number>({
	name: 'y',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export const widthProperty = new Property<Rect, number>({
	name: 'width',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export const heightProperty = new Property<Rect, number>({
	name: 'height',
	valueConverter(value) {
		return parseFloat(value);
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
			this._children.forEach((child) => {
				switch (child.paintStyle) {
					case 'fill':
						context.fillStyle = child.color.hex;
						context.fill();
						break;
					case 'stroke':
						context.strokeStyle = child.color.hex;
						context.lineWidth = child.strokeWidth;
						context.stroke();
						break;
				}
			});
		} else {
			super.draw();
		}
	}

	_addViewToNativeVisualTree(view: ViewBase, atIndex?: number): boolean {
		if (view instanceof Paint) {
			this._children.push(view);
		}
		return false;
	}
}

xProperty.register(Rect);
yProperty.register(Rect);
