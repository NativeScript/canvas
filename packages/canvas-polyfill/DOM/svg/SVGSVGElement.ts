import { SVGGraphicsElement } from './SVGGraphicsElement';
import { SVGAnimatedLength, SVGLength } from './SVGAnimatedLength';
import { SVGPoint, SVGAngle, SVGRect } from './SVGUnits';
import { SVGMatrix } from './SVGMatrix';
import { SVGTransform } from './SVGTransform';
import { SVGAnimatedRect } from './SVGAnimatedRect';
import { Svg } from '@nativescript/canvas-svg';
export class SVGSVGElement extends SVGGraphicsElement {
	private _x = new SVGAnimatedLength(this, 'x');
	private _y = new SVGAnimatedLength(this, 'y');

	private _width = new SVGAnimatedLength(this, 'width');
	// @ts-ignore
	private _height = new SVGAnimatedLength(this, 'height');

	private _viewBox = new SVGAnimatedRect(this, 'viewBox');

	constructor() {
		super('svg');
		this.nativeElement = new Svg() as never;
		this._width.baseVal.newValueSpecifiedUnits(SVGLength.SVG_LENGTHTYPE_PERCENTAGE, 100);
		this._width.animVal.newValueSpecifiedUnits(SVGLength.SVG_LENGTHTYPE_PERCENTAGE, 100);
	}

	get viewBox() {
		return this._viewBox;
	}

	get x() {
		return this._x;
	}

	get y() {
		return this._y;
	}

	// @ts-ignore
	get width() {
		return this._width;
	}

	// @ts-ignore
	set width(value: unknown) {
		// noop ??
	}

	// @ts-ignore
	get height() {
		return this._height;
	}

	// @ts-ignore
	set height(value: unknown) {}

	createSVGPoint() {
		return new SVGPoint();
	}

	createSVGAngle() {
		return new SVGAngle();
	}

	createSVGLength() {
		return new SVGLength();
	}

	createSVGMatrix() {
		return new SVGMatrix();
	}

	createSVGNumber() {
		return new SVGNumber();
	}

	createSVGRect() {
		return new SVGRect();
	}
	createSVGTransform() {
		return new SVGTransform();
	}

	createSVGTransformFromMatrix(matrix: DOMMatrix | SVGMatrix) {
		const ret = new SVGTransform();
		if (matrix instanceof DOMMatrix) {
			ret._matrix = (<any>matrix).nativeInstance.clone();
		} else if (matrix instanceof SVGMatrix) {
			ret._matrix = (<any>matrix._matrix).clone();
		}
		return ret;
	}
}
