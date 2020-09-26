import {Path2D} from "../Canvas2D";
import {SVGItem} from "./SVGItem";
import {Property} from "@nativescript/core/ui/core/properties";

export const pointsProperty = new Property<Polygon, any>({
	name: 'points'
});

export class Polygon extends SVGItem {
	points: any;


	handleValues(canvas?) {
		let ctx: any;
		if (canvas) {
			ctx = canvas.getContext('2d');
		} else {
			ctx = this._canvas.getContext('2d') as any;
		}
		ctx.save();
		let points: { x: number, y: number }[];
		if (typeof this.points === 'string') {
			points = this.points.split(' ').map(point => {
				if (typeof point === 'string') {
					const realPoints = point.split(',');
					const x = parseInt(realPoints[0]);
					const y = parseInt(realPoints[1]);
					if (isNaN(x) || isNaN(y)) {
						return null;
					}
					return {x, y};
				}
				return null;
			}).filter(value => {
				return value !== null;
			});
		}
		let path = new Path2D();
		ctx.lineWidth = this.strokeWidth;
		if (Array.isArray(points)) {
			points.forEach((point, index) => {
				if (index === 0) {
					path.moveTo(point.x, point.y);
				} else {
					path.lineTo(point.x, point.y);
				}
			});
			path.closePath();
		}

		if (this._doFill()) {
			ctx.fillStyle = this._realFill;
			ctx.fill(path);
		}

		if (this._doStroke()) {
			ctx.strokeStyle = this._realStroke;
			ctx.stroke(path);
		}
		ctx.restore();
	}
}

pointsProperty.register(Polygon);
