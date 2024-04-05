import { SVGGeometryElement } from './SVGGeometryElement';
import { Polyline } from '@nativescript/canvas-svg';
import { SVGPointList } from './SVGUnits';
export class SVGPolylineElement extends SVGGeometryElement {
	constructor() {
		super('polyline');
		this.nativeElement = new Polyline() as never;
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
