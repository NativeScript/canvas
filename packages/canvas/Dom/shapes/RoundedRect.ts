import { colorProperty, Property, booleanConverter, ViewBase, Screen } from '@nativescript/core';
import { Group } from '../Group';
import { Paint } from '../Paint';
import { Shadow } from '../Shadow';

const xProperty = new Property<RoundedRect, number>({
	name: 'x',
	valueConverter(value) {
		return parseFloat(value);
	},
});

const yProperty = new Property<RoundedRect, number>({
	name: 'y',
	valueConverter(value) {
		return parseFloat(value);
	},
});

const rProperty = new Property<RoundedRect, number>({
	name: 'r',
	valueConverter(value) {
		return parseFloat(value);
	},
});

export class RoundedRect extends Paint {
	x: number;
	y: number;
	width: number;
	height: number;
	r: number;

	_children: Paint[] = [];

	draw() {
		const scale = Screen.mainScreen.scale;
		const context = this._canvas.getContext('2d') as any;
		const color = this._getColor();
		context.roundRect(this.x * scale, this.y * scale, this.width * scale, this.height * scale, this.r * scale);
		if (this._children.length > 0) {
			this._children.forEach((child) => {
				const child_color = child._getColor();
				switch (child.paintStyle) {
					case 'fill':
						context.fillStyle = color;
						if (child instanceof Shadow) {
							const dx = context.shadowOffsetX;
							const dy = context.shadowOffsetY;
							const blur = context.shadowBlur;
							const shadowColor = context.shadowColor;

							context.shadowOffsetX = child.dx * scale;
							context.shadowOffsetY = child.dy * scale;
							context.shadowBlur = child.blur * scale;
							context.shadowColor = child_color;

							context.fill();

							context.shadowOffsetX = dx;
							context.shadowOffsetY = dy;
							context.shadowBlur = blur;
							context.shadowColor = shadowColor;
						} else {
							context.fill();
						}

						break;
					case 'stroke':
						context.strokeStyle = color;
						context.lineWidth = child.strokeWidth;

						if (child instanceof Shadow) {
							const dx = context.shadowOffsetX;
							const dy = context.shadowOffsetY;
							const blur = context.shadowBlur;
							const shadowColor = context.shadowColor;

							context.shadowOffsetX = child.dx * scale;
							context.shadowOffsetY = child.dy * scale;
							context.shadowBlur = child.blur * scale;
							context.shadowColor = child_color;

							context.stroke();

							context.shadowOffsetX = dx;
							context.shadowOffsetY = dy;
							context.shadowBlur = blur;
							context.shadowColor = shadowColor;
						} else {
							context.stroke();
						}

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

xProperty.register(RoundedRect);
yProperty.register(RoundedRect);
rProperty.register(RoundedRect);
