import { DOMMatrix } from '@nativescript/canvas';
export class SVGMatrix {
	_matrix = new DOMMatrix();

	get a(): number {
		return this._matrix.a;
	}

	set a(value) {
		this._matrix.a = value;
	}

	get b(): number {
		return this._matrix.b;
	}

	set b(value) {
		this._matrix.b = value;
	}

	get c(): number {
		return this._matrix.c;
	}

	set c(value) {
		this._matrix.c = value;
	}

	get d(): number {
		return this._matrix.d;
	}

	set d(value) {
		this._matrix.d = value;
	}

	get e(): number {
		return this._matrix.e;
	}

	set e(value) {
		this._matrix.e = value;
	}

	get f(): number {
		return this._matrix.f;
	}

	set f(value) {
		this._matrix.f = value;
	}

	rotate(rotX?: number, rotY?: number, rotZ?: number) {
		const ret = new SVGMatrix();
		ret._matrix = this._matrix.rotate(rotX, rotY, rotZ);
		return ret;
	}

	scale(scaleX?: number, scaleY?: number, scaleZ?: number, originX?: number, originY?: number, originZ?: number) {
		const ret = new SVGMatrix();
		ret._matrix = this._matrix.scale(scaleX, scaleY);
		return ret;
	}

	scaleNonUniform(scaleX?: number, scaleY?: number) {
		const ret = new SVGMatrix();
		ret._matrix = this._matrix.scaleNonUniform(scaleX, scaleY);
		return ret;
	}

	skewX(sx?: number) {
		const ret = new SVGMatrix();
		ret._matrix = this._matrix.skewX(sx);
		return ret;
	}
	skewY(sy?: number) {
		const ret = new SVGMatrix();
		ret._matrix = this._matrix.skewY(sy);
		return ret;
	}
}
