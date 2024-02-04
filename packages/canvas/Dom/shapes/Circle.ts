import { colorProperty, Property, booleanConverter, ViewBase, ShorthandProperty, Style } from '@nativescript/core';
import { Group } from '../Group';
import { Paint } from '../Paint';

export const cxProperty = new Property<Circle, number>({
	name: 'cx',
	valueConverter(value) {
		return parseFloat(value);
	},
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export const cyProperty = new Property<Circle, number>({
	name: 'cy',
	valueConverter(value) {
		return parseFloat(value);
	},
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export const rProperty = new Property<Circle, number>({
	name: 'r',
	valueConverter(value) {
		return parseFloat(value);
	},
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

interface Vector {
	x: number;
	y: number;
}

export const cProperty = new ShorthandProperty<Style, Vector>({
	name: 'c',
	cssName: 'c',
	getter: function () {
		return { x: (this as any).cx, y: (this as any).cy };
	},
	converter(value): any {
		if (typeof value === 'string') {
			try {
				const val = JSON.parse(value);
				return [
					[cxProperty, val.x as number],
					[cyProperty, val.y as number],
				];
			} catch (error) {}
		} else {
			return [
				[cxProperty, value.x],
				[cyProperty, value.y],
			];
		}
		return [];
	},
});

export class Circle extends Paint {
	cx: number;
	cy: number;
	r: number;

	_children: Paint[] = [];

	get c() {
		return (this.style as any).c;
	}

	set c(value) {
		(this.style as any).c = value;
	}

	draw() {
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;

		const c = this.c;

		context.arc(c.x ?? this.cx, c.y ?? this.cy, this.r, 0, 2 * Math.PI);
		if (this._children.length > 0) {
			this._children.forEach((child) => {
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
						context.stroke();
						break;
				}
			});
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

cxProperty.register(Circle);
cyProperty.register(Circle);
rProperty.register(Circle);
cProperty.register(Style);
