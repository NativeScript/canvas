import {Property} from "@nativescript/core/ui/core/view";
import {Path2D} from "../Canvas2D";
import {SVGItem} from "./SVGItem";

export const x1Property = new Property<Line, any>({
	name: 'x1'
});
export const y1Property = new Property<Line, any>({
	name: 'y1'
});
export const x2Property = new Property<Line, any>({
	name: 'x2'
});
export const y2Property = new Property<Line, any>({
	name: 'y2'
});

export class Line extends SVGItem {
	x1: any;
	y1: any;
	x2: any;
	y2: any;

	handleValues(canvas?) {
		let ctx: any;
		if (canvas) {
			ctx = canvas.getContext('2d');
		} else {
			ctx = this._canvas.getContext('2d') as any;
		}
		const line = new Path2D();
		line.moveTo(this.x1, this.y1);
		line.lineTo(this.x2, this.y2);
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
			ctx.fill(line);
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
			ctx.stroke(line);
		}
		ctx.restore();
	}

}

x1Property.register(Line);
x2Property.register(Line);
y1Property.register(Line);
y2Property.register(Line);
