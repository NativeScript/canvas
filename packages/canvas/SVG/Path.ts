import {Property} from "@nativescript/core";
import {Path2D, CanvasGradient} from "../Canvas2D";
import {SVGItem} from "./SVGItem";

export const dProperty = new Property<Path, string>({
	name: 'd'
});

export class Path extends SVGItem {
	public d: string;

	handleValues(canvas?) {
		let ctx: CanvasRenderingContext2D;
		if (canvas) {
			ctx = canvas.getContext('2d');
		} else {
			ctx = this._canvas.getContext('2d') as any;
		}
		ctx.save();
		// @ts-ignore
		if (this.parent.strokeWidth !== undefined) {
			// @ts-ignore
			ctx.lineWidth = this.parent.strokeWidth;
		}
		if (this.strokeWidth !== undefined) {
			ctx.lineWidth = this.strokeWidth;
		}
		ctx.globalAlpha = this._realOpacity;
		let lastOpacity;
// @ts-ignore
		if (this._doFillOpacity()) {
			lastOpacity = ctx.globalAlpha;
			ctx.globalAlpha = this._realFillOpacity;
		}
		let path;
		if (this.d) {
			path = new Path2D(this.d);
		}
		if (path && this._doFill()) {
			if (this._realFill !== undefined && this._realFill !== 'none') {
				if (this._realFill && this._realFill.indexOf('url') > -1) {
					const fill = this._getViewById(this._realFill);
					if (fill) {
						const style = fill._getFillOrStrokeStyle();
						if (style instanceof CanvasGradient) {
							ctx.fillStyle = style;
						} else {
							ctx.fillStyle = ctx.createPattern(style, 'repeat');
						}
					}
				} else {
					ctx.fillStyle = this._realFill;
				}
				ctx.fill(path, this.fillRule);
			}
		}

		if (lastOpacity !== undefined) {
			ctx.globalAlpha = lastOpacity;
		}

		if (this.strokeLinecap) {
			ctx.lineCap = this.strokeLinecap;
		}

		if (this.strokeLinejoin) {
			ctx.lineJoin = this.strokeLinejoin;
		}
		const miterLimit = parseInt(this.strokeMiterlimitProperty, 10)
		if (!isNaN(miterLimit)) {
			ctx.miterLimit = miterLimit;
		}
		if (path && this._doStroke()) {
			if (this._realStroke !== undefined && this._realStroke !== 'none') {
				if (this._realStroke && this._realStroke.indexOf('url') > -1) {
					const stroke = this._getViewById(this._realStroke);
					if (stroke) {
						const style = stroke._getFillOrStrokeStyle();
						if (style instanceof CanvasGradient) {
							ctx.strokeStyle = style;
						} else {
							ctx.strokeStyle = ctx.createPattern(style, 'repeat');
						}
					}
				} else {
					ctx.strokeStyle = this._realStroke;
				}
				ctx.stroke(path);
			}
		}
		ctx.restore();
	}
}

dProperty.register(Path);
