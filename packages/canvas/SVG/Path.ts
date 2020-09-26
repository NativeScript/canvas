import {Path2D, CanvasGradient} from "../Canvas2D";
import {SVGItem} from "./SVGItem";
import {Property} from "@nativescript/core/ui/core/properties";

export const dProperty = new Property<Path, string>({
	name: 'd'
});

export class Path extends SVGItem {
	public d: string;

	handleValues(canvas?) {
		let ctx: any;
		if (canvas) {
			ctx = canvas.getContext('2d');
		} else {
			ctx = this._canvas.getContext('2d') as any;
		}
		ctx.save();
		if (this.strokeWidth) {
			ctx.lineWidth = this.strokeWidth;
		}
		ctx.globalAlpha = this._realOpacity;
		let lastOpacity;
		if (this._doFillOpacity()) {
			lastOpacity = ctx.globalAlpha;
			ctx.globalAlpha = this._realFillOpacity;
		}
		let path;
		if (this.d) {
			path = new Path2D(this.d);
		}
		if (path && this._doFill()) {
			if (this.fill !== undefined && this.fill !== 'none') {
				if (this.fill && this.fill.indexOf('url') > -1) {
					const fill = this._getViewById(this.fill);
					if (fill) {
						const style = fill._getFillOrStrokeStyle();
						if (style instanceof CanvasGradient) {
							ctx.fillStyle = style;
						} else {
							ctx.fillStyle = ctx.createPattern(style, 'repeat');
						}
					}
				} else {
					ctx.fillStyle = this.fill;
				}
			}
			ctx.fill(path);
		}

		if (lastOpacity !== undefined) {
			ctx.globalAlpha = lastOpacity;
		}
		if (path && this._doStroke()) {
			if (this.stroke !== undefined && this.stroke !== 'none') {
				if (this.stroke && this.stroke.indexOf('url') > -1) {
					const stroke = this._getViewById(this.stroke);
					if (stroke) {
						const style = stroke._getFillOrStrokeStyle();
						if (style instanceof CanvasGradient) {
							ctx.strokeStyle = style;
						} else {
							ctx.strokeStyle = ctx.createPattern(style, 'repeat');
						}
					}
				} else {
					ctx.strokeStyle = this.stroke;
				}
			}
			ctx.stroke(path);
		}
		ctx.restore();
	}
}

dProperty.register(Path);
