import {Property} from "@nativescript/core";
import {Path2D} from "../Canvas2D";
import {SVGItem} from "./SVGItem";

export const cxProperty = new Property<Ellipse, any>({
	name: 'cx'
});
export const cyProperty = new Property<Ellipse, any>({
	name: 'cy'
});
export const rxProperty = new Property<Ellipse, any>({
	name: 'rx'
});
export const ryProperty = new Property<Ellipse, any>({
	name: 'ry'
});

export class Ellipse extends SVGItem {
	cx: any;
	cy: any;
	rx: any;
	ry: any;

	constructor() {
		super();
		this.stroke = 'transparent';
	}

	handleValues(canvas?) {
		let ctx: any;
		if (canvas) {
			ctx = canvas.getContext('2d');
		} else {
			ctx = this._canvas.getContext('2d') as any;
		}
		const ellipse = new Path2D();
		ellipse.ellipse(this.cx, this.cy, this.rx, this.ry, 0, 0, 2 * Math.PI);
		ctx.save();
		if (this._doFill()) {
			ctx.fillStyle = this._realFill;
			ctx.stroke(ellipse);
		}

		if (this._doStroke()) {
			ctx.strokeStyle = this._realStroke;
			ctx.fill(ellipse);
		}
		ctx.restore();
	}
}

cxProperty.register(Ellipse);
cyProperty.register(Ellipse);
rxProperty.register(Ellipse);
ryProperty.register(Ellipse);
