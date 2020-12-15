import {AddChildFromBuilder, Property} from "@nativescript/core";
import { SVGItem } from "./SVGItem";

export const xProperty = new Property<Text, any>({
    name: 'x'
});

export const yProperty = new Property<Text, any>({
    name: 'y'
});

export const dxProperty = new Property<Text, any>({
    name: 'dx'
});

export const dyProperty = new Property<Text, any>({
    name: 'dy'
});

export class Text extends SVGItem implements AddChildFromBuilder {
    x: any;
    y: any;
    dx: any;
    dy: any;
    text: string;
    constructor() {
        super();
    }

    _addChildFromBuilder(name: string, value: any): void {
    }

    handleValues(canvas?) {
		let ctx: CanvasRenderingContext2D;
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
			ctx.fillText(this.text,this.x, this.y);
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
			ctx.strokeText(this.text,this.x, this.y);
		}


		ctx.restore();
	}
}


xProperty.register(Text);
yProperty.register(Text);
dxProperty.register(Text);
dyProperty.register(Text);
