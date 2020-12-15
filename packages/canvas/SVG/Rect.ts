import { SVGItem } from "./SVGItem";
import { CanvasGradient } from '../Canvas2D';

export class Rect extends SVGItem {
	x: any = 0;
	y: any = 0;
	rx: any = 0;
	ry: any = 0;

	handleValues(canvas?) {
		let ctx: any;
		if (canvas) {
			ctx = canvas.getContext('2d');
		} else {
			ctx = this._canvas.getContext('2d') as any;
		}
		ctx.save();
		ctx.globalAlpha = this._realOpacity;
		let lastOpacity;
		if (this._doFillOpacity()) {
			lastOpacity = ctx.globalAlpha;
			ctx.globalAlpha = this._realFillOpacity;
		}

		if (this._doFill()) {
			if (this.fill !== undefined && this.fill !== 'none') {
				if (this.fill && this.fill.indexOf('url') > -1) {
					let fill = this._getViewById(this.fill);
					if (fill) {
						const style: any = fill._getFillOrStrokeStyle();
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
			ctx.fillRect(this.x, this.y, this._getRealSize(this.width, null, 'width') as any, this._getRealSize(this.height, null, 'height') as any);
		}

		if (lastOpacity !== undefined) {
			ctx.globalAlpha = lastOpacity;
		}

		if (this._doStroke()) {
			if (this.stroke !== undefined && this.stroke !== 'none') {
				if (this.stroke && this.stroke.indexOf('url') > -1) {
					const stroke = this._getViewById(this.stroke);
					const style: any = stroke._getFillOrStrokeStyle();
					if (style instanceof CanvasGradient) {
						ctx.strokeStyle = style;
					} else if (stroke) {
						ctx.strokeStyle = ctx.createPattern(style, 'repeat');
					}
				} else {
					ctx.strokeStyle = this.stroke;
				}
			}
			ctx.strokeRect(this.x, this.y, this._getRealSize(this.width, null, 'width') as any, this._getRealSize(this.height, null, 'height') as any);
		}


		ctx.restore();
	}
}
