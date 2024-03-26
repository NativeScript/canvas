const SVG_TRANSFORM_UNKNOWN = 0;
const SVG_TRANSFORM_MATRIX = 1;
const SVG_TRANSFORM_TRANSLATE = 2;
const SVG_TRANSFORM_SCALE = 3;
const SVG_TRANSFORM_ROTATE = 4;
const SVG_TRANSFORM_SKEWX = 5;
const SVG_TRANSFORM_SKEWY = 6;

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
			ret._matrix.multiplySelf(transform._matrix);
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
	_matrix: DOMMatrix;
	constructor() {
		this._type = SVG_TRANSFORM_UNKNOWN;
		this._matrix = new DOMMatrix();
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

	setMatrix(matrix: DOMMatrix) {
		this._matrix = matrix;
		this._type = SVG_TRANSFORM_MATRIX;
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
