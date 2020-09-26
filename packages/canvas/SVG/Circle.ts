import {SVGItem} from "./SVGItem";
import {CanvasGradient, Path2D} from "../Canvas2D";
import {Property} from "@nativescript/core/ui/core/properties";

export const cxProperty = new Property<Circle, any>({
	name: 'cx'
});

export const cyProperty = new Property<Circle, any>({
	name: 'cy'
});

export const rProperty = new Property<Circle, any>({
	name: 'r'
});


export class Circle extends SVGItem {
	cx: any;
	cy: any;
	r: any;

	handleValues(canvas?) {
		let ctx: any;
		if (canvas) {
			ctx = canvas.getContext('2d');
		} else {
			ctx = this._canvas.getContext('2d') as any;
		}
		const circle: any = new Path2D();
		circle.arc(this.cx, this.cy, this.r, 0, 2 * Math.PI);
		ctx.save();
		ctx.lineWidth = this.strokeWidth;
		ctx.globalAlpha = this._realOpacity;
		let lastOpacity;
		if (this._doFillOpacity()) {
			lastOpacity = ctx.globalAlpha;
			ctx.globalAlpha = this._realFillOpacity;
		}
		if (this._doFill()) {
			if (this.fill !== undefined && this.fill !== 'none') {
				if (this.fill && this.fill.indexOf('url') > -1) {
					const fill = this._getViewById(this.fill);
					if (fill) {
						const style = fill._getFillOrStrokeStyle();
						if (style instanceof CanvasGradient) {
							if (typeof fill.gradientTransform === 'string' && fill.gradientTransform.indexOf('rotate')) {
								let rotation = parseInt(fill.gradientTransform.replace('rotate(', '').replace(')', ''));
								if (!isNaN(rotation)) {
									ctx.rotate(rotation);
								}
							}
							ctx.fillStyle = style;
						} else {
							ctx.fillStyle = ctx.createPattern(style, 'repeat');
						}
					}
				} else {
					ctx.fillStyle = this.fill;
				}
			}
			ctx.fill(circle);
		}
		if (lastOpacity !== undefined) {
			ctx.globalAlpha = lastOpacity;
		}

		if (this._doStroke()) {
			if (this.stroke !== undefined && this.stroke !== 'none') {
				if (this.stroke && this.stroke.indexOf('url') > -1) {
					const stroke = this._getViewById(this.stroke);
					if (stroke) {
						const style = stroke._getFillOrStrokeStyle();
						if (stroke instanceof CanvasGradient) {
							ctx.strokeStyle = style;
						} else {
							ctx.strokeStyle = ctx.createPattern(style, 'repeat');
						}
					}
				} else {
					ctx.strokeStyle = this.stroke;
				}
			}
			ctx.stroke(circle);
		}
		ctx.restore();
	}
}

cxProperty.register(Circle);
cyProperty.register(Circle);
