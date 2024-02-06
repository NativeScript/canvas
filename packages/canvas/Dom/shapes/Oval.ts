import { colorProperty, Property, booleanConverter, ViewBase, Screen } from '@nativescript/core';
import { Group } from '../Group';
import { Paint } from '../Paint';
import { Shadow } from '../Shadow';
import { Rect } from './Rect';

export class Oval extends Rect {
	x: number;
	y: number;
	width: number;
	height: number;

	_children: Paint[] = [];

	draw() {
		const context = this._canvas.getContext('2d') as any;
		const color = this._getColor();
		if (this._children.length > 0) {
			this._children.forEach((child) => {
				const child_color = child._getColor();
				const paintStyle = child._getPaintStyle();
				switch (paintStyle) {
					case 'fill':
						context.fillStyle = color;
						if (child instanceof Shadow) {
							const dx = context.shadowOffsetX;
							const dy = context.shadowOffsetY;
							const blur = context.shadowBlur;
							const shadowColor = context.shadowColor;

							context.shadowOffsetX = child.dx;
							context.shadowOffsetY = child.dy;
							context.shadowBlur = child.blur;
							context.shadowColor = child_color;

							context.fillOval(this.x, this.y, this.width, this.height);

							context.shadowOffsetX = dx;
							context.shadowOffsetY = dy;
							context.shadowBlur = blur;
							context.shadowColor = shadowColor;
						} else {
							context.fillOval(this.x, this.y, this.width, this.height);
						}

						break;
					case 'stroke':
						context.strokeStyle = color;
						context.lineWidth = child._getStrokeWidth();
						context.lineJoin = child._getStrokeJoin();

						if (child instanceof Shadow) {
							const dx = context.shadowOffsetX;
							const dy = context.shadowOffsetY;
							const blur = context.shadowBlur;
							const shadowColor = context.shadowColor;

							context.shadowOffsetX = child.dx;
							context.shadowOffsetY = child.dy;
							context.shadowBlur = child.blur;
							context.shadowColor = child_color;

							context.strokeOval(this.x, this.y, this.width, this.height);

							context.shadowOffsetX = dx;
							context.shadowOffsetY = dy;
							context.shadowBlur = blur;
							context.shadowColor = shadowColor;
						} else {
							context.strokeOval(this.x, this.y, this.width, this.height);
						}

						break;
				}
			});
		} else {
			const paintStyle = this._getPaintStyle();
			switch (paintStyle) {
				case 'fill':
					context.fillStyle = color;
					context.fillOval(this.x, this.y, this.width, this.height);
					break;
				case 'stroke':
					context.strokeStyle = color;
					context.lineWidth = this._getStrokeWidth();
					context.lineJoin = this._getStrokeJoin();
					context.strokeOval(this.x, this.y, this.width, this.height);
					break;
			}
		}
	}

	_addViewToNativeVisualTree(view: ViewBase, atIndex?: number): boolean {
		if (view instanceof Paint) {
			this._children.push(view);
		}
		return false;
	}
}
