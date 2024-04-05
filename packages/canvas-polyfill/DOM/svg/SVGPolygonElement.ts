import { SVGGeometryElement } from './SVGGeometryElement';
import { Polygon } from '@nativescript/canvas-svg';
import { SVGPointList } from './SVGUnits';
export class SVGPolygonElement extends SVGGeometryElement {
	constructor() {
		super('polygon');
		this.nativeElement = new Polygon() as never;
	}

	_points = new SVGPointList();

	_animatedPoints = new SVGPointList();

	get points() {
		const points = this.getAttribute?.('points');

		if (points) {
			const items = SVGPointList.fromString(points);
			this._points.splice(0, this._points.length, ...items);
		}

		return this._points;
	}

	get animatedPoints() {
		return this._animatedPoints;
	}
}
