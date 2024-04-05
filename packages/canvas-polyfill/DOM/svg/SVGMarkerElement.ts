import { SVGGraphicsElement } from './SVGGraphicsElement';
import { SVGAnimatedRect } from './SVGAnimatedRect';
import { SVGLength } from './SVGAnimatedLength';
import { SVGAngle } from './SVGUnits';
import { SVGAnimatedPreserveAspectRatio } from './SVGAnimatedPreserveAspectRatio';
export class SVGMarkerElement extends SVGGraphicsElement {
	private _viewBox = new SVGAnimatedRect(this, 'viewBox');

	private _markerHeight = new SVGLength(this, 'markerHeight', '3');

	private _markerWidth = new SVGLength(this, 'markerWidth', '3');

	private _orient = new SVGAngle(this, 'orient');

	private _preserveAspectRatio = new SVGAnimatedPreserveAspectRatio(this, 'preserveAspectRatio');

	get markerWidth() {
		return this._markerWidth;
	}

	get markerHeight() {
		return this._markerHeight;
	}

	get orient() {
		return this._orient;
	}

	get viewBox() {
		return this._viewBox;
	}

	get preserveAspectRatio() {
		return this._preserveAspectRatio;
	}
}
