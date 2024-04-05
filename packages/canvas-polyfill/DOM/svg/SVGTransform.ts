const SVG_TRANSFORM_UNKNOWN = 0;
const SVG_TRANSFORM_MATRIX = 1;
const SVG_TRANSFORM_TRANSLATE = 2;
const SVG_TRANSFORM_SCALE = 3;
const SVG_TRANSFORM_ROTATE = 4;
const SVG_TRANSFORM_SKEWX = 5;
const SVG_TRANSFORM_SKEWY = 6;
import { SVGMatrix } from './SVGMatrix';
export class SVGTransformList extends Array {
	clear() {
		this.splice(0);
	}

	getItem(index: number) {
		return this[index];
	}

	insertItemBefore(newItem: SVGTransform, index: number) {
		this.splice(index - 1, 0, newItem);
	}

	replaceItem(newItem: SVGTransform, index: number) {
		this[index] = newItem;
	}

	removeItem(index: number) {
		this.splice(index, 1);
	}

	appendItem(newItem: SVGTransform) {
		this.push(newItem);
	}

	consolidate() {
		const ret = new SVGTransform();
		ret._type = SVG_TRANSFORM_MATRIX;
		for (const transform of this) {
			ret._matrix._matrix.multiplySelf(transform.matrix);
		}
		return ret;
	}

	get numberOfItems() {
		return this.length;
	}
}

export class SVGTransform {
	_type: 0 | 1 | 2 | 3 | 4 | 5 | 6;
	_rotate = 0;
	_matrix: SVGMatrix;
	constructor() {
		this._type = SVG_TRANSFORM_MATRIX;
		this._matrix = new SVGMatrix();
	}

	get type() {
		return this._type;
	}

	get rotate() {
		return this._rotate;
	}

	get matrix() {
		return this._matrix;
	}

	static SVG_TRANSFORM_UNKNOWN = SVG_TRANSFORM_UNKNOWN;
	static SVG_TRANSFORM_MATRIX = SVG_TRANSFORM_MATRIX;
	static SVG_TRANSFORM_TRANSLATE = SVG_TRANSFORM_TRANSLATE;
	static SVG_TRANSFORM_SCALE = SVG_TRANSFORM_SCALE;
	static SVG_TRANSFORM_ROTATE = SVG_TRANSFORM_ROTATE;
	static SVG_TRANSFORM_SKEWX = SVG_TRANSFORM_SKEWX;
	static SVG_TRANSFORM_SKEWY = SVG_TRANSFORM_SKEWY;

	setMatrix(matrix: SVGMatrix) {
		if (matrix?._matrix instanceof DOMMatrix) {
			this._matrix = new SVGMatrix();
			this._matrix._matrix = matrix._matrix;
			this._type = SVG_TRANSFORM_MATRIX;
		} else if (matrix instanceof SVGMatrix) {
			this._matrix = matrix;
			this._type = SVG_TRANSFORM_MATRIX;
		}
	}

	setTranslate(x: number, y: number) {
		this._matrix = this._matrix.translate(x, y);
		this._type = SVG_TRANSFORM_TRANSLATE;
	}

	setScale(sx: number, sy: number) {
		this._matrix = this._matrix.scale(sx, sy);
		this._type = SVG_TRANSFORM_SCALE;
	}

	setRotate(angle: number, cx: number, cy: number) {
		this._matrix = this._matrix.rotate(angle, cx, cy);
		this._rotate = angle;
		this._type = SVG_TRANSFORM_ROTATE;
	}

	setSkewX(angle: number) {
		this._matrix = this._matrix.skewX(angle);
		this._rotate = angle;
		this._type = SVG_TRANSFORM_SKEWX;
	}

	setSkewY(angle: number) {
		this._matrix = this._matrix.skewY(angle);
		this._rotate = angle;
		this._type = SVG_TRANSFORM_SKEWY;
	}
}
