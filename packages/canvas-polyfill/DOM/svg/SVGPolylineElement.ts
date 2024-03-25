import { SVGGeometryElement } from './SVGGeometryElement';
import { Polyline } from '@nativescript/canvas-svg';
import { SVGPointList } from './SVGUnits';
export class SVGPolylineElement extends SVGGeometryElement {
	constructor() {
		super('polyline');
		this.nativeElement = new Polyline();
	}

	_points = new SVGPointList();

	_animatedPoints = new SVGPointList();

	get points() {
		const points = (this._xmlDom?.documentElement ?? this._xmlDom)?.getAttribute?.('points');

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
