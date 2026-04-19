import { Property, ViewBase } from '@nativescript/core';
import { Paint } from '../Paint';
import { Shadow } from '../Shadow';
import { Rect } from './Rect';

const rProperty = new Property<RoundedRect, number>({
	name: 'r',
	valueConverter(value) {
		return parseFloat(value);
	},
	valueChanged(target, _oldValue, _newValue) {
		target.invalidate();
	},
});

export class RoundedRect extends Rect {
	r!: number;

	draw() {
		const context = this._canvas.getContext('2d') as any;
		context.beginPath();
		context.roundRect(this.x, this.y, this.width, this.height, this.r);

		if (this._children.length > 0) {
			this._children.forEach((child) => {
				const child_color = child._getColor();
				const style = child._getPaintStyle();

				if (child instanceof Shadow) {
					const prevOffsetX = context.shadowOffsetX;
					const prevOffsetY = context.shadowOffsetY;
					const prevBlur = context.shadowBlur;
					const prevColor = context.shadowColor;

					context.shadowOffsetX = child.dx;
					context.shadowOffsetY = child.dy;
					context.shadowBlur = child.blur;
					context.shadowColor = child_color;

					if (style === 'stroke') {
						context.strokeStyle = this._getColor();
						context.lineWidth = child._getStrokeWidth();
						context.lineJoin = child._getStrokeJoin();
						context.stroke();
					} else {
						context.fillStyle = this._getColor();
						context.fill();
					}

					context.shadowOffsetX = prevOffsetX;
					context.shadowOffsetY = prevOffsetY;
					context.shadowBlur = prevBlur;
					context.shadowColor = prevColor;
				} else if (style === 'stroke') {
					context.strokeStyle = child_color;
					context.lineWidth = child._getStrokeWidth();
					context.lineJoin = child._getStrokeJoin();
					context.stroke();
				} else {
					context.fillStyle = child_color;
					context.fill();
				}
			});
		} else {
			super.draw();
		}
	}

	_addViewToNativeVisualTree(view: ViewBase, _atIndex?: number): boolean {
		if (view instanceof Paint) {
			this._children.push(view);
		}
		return false;
	}
}

rProperty.register(RoundedRect);
