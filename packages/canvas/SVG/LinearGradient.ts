import {SVGItem} from "./SVGItem";
import {SVG} from "./SVG";
import {AddChildFromBuilder} from "@nativescript/core";
import {Stop} from "./Stop";

export class LinearGradient extends SVGItem implements AddChildFromBuilder {
	_views: any[];
	x1: any = '0%';
	y1: any = '0%';
	x2: any = '100%';
	y2: any = '0%';

	gradientTransform: string;

	constructor() {
		super();
		this._views = [];
	}

	_getFillOrStrokeStyle(canvas?) {
		let ctx: any;
		if (canvas) {
			ctx = canvas.getContext('2d');
		} else {
			ctx = this._canvas.getContext('2d') as any;
		}
		let svg: any = this.parent;
		while (!(svg instanceof SVG)) {
			svg = svg.parent;
			if (svg === undefined || svg === null) {
				break;
			}
		}

		const height = svg ? svg.getMeasuredHeight() : 0;
		const width = svg ? svg.getMeasuredWidth() : 0;
		const realX1 = this._getRealSize(this.x1, width);
		const realX2 = this._getRealSize(this.x2, width);
		const realY1 = this._getRealSize(this.y1, height);
		const realY2 = this._getRealSize(this.y2, height);
		const gradient = ctx.createLinearGradient(realX1, realY1, realX2, realY2);
		this._views.forEach((child) => {
			if (child instanceof Stop) {
				const offset = child._realOffset();
				gradient.addColorStop(offset, child.stopColor);
			}
		});
		return gradient;
	}

	_addChildFromBuilder(name: string, value: any): void {
		value._canvas = this._canvas;
		value.parent = this;
		this._views.push(value);
		this._appendChild(value.id || value._domId, value);
	}
}
