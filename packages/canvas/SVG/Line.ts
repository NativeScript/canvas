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
		if (this._doStroke()) {
			ctx.strokeStyle = this._realStroke;
			ctx.fill(line);
		}
		ctx.restore();
	}

}

x1Property.register(Line);
x2Property.register(Line);
y1Property.register(Line);
y2Property.register(Line);
