import { Helpers } from '@nativescript/canvas/helpers';

export class DOMMatrix {
	static {
		Helpers.initialize();
	}
	_native;
	constructor(instance?) {
		if (Array.isArray(instance)) {
			this._native = new global.CanvasModule.DOMMatrix(instance);
		} else if (!instance) {
			this._native = new global.CanvasModule.DOMMatrix();
		} else {
			this._native = instance;
		}
	}

	get native() {
		return this._native;
	}

	get nativeInstance() {
		return this.native;
	}

	get a(): number {
		return this.nativeInstance.a;
	}

	set a(value) {
		this.nativeInstance.a = value;
	}

	get b(): number {
		return this.nativeInstance.b;
	}

	set b(value) {
		this.nativeInstance.b = value;
	}

	get c(): number {
		return this.nativeInstance.c;
	}

	set c(value) {
		this.nativeInstance.c = value;
	}

	get d(): number {
		return this.nativeInstance.d;
	}

	set d(value) {
		this.nativeInstance.d = value;
	}

	get e(): number {
		return this.nativeInstance.e;
	}

	set e(value) {
		this.nativeInstance.e = value;
	}

	get f(): number {
		return this.nativeInstance.f;
	}

	set f(value) {
		this.nativeInstance.f = value;
	}

	get m11(): number {
		return this.nativeInstance.m11;
	}

	set m11(value) {
		this.nativeInstance.m11 = value;
	}

	get m12(): number {
		return this.nativeInstance.m12;
	}

	set m12(value) {
		this.nativeInstance.m12 = value;
	}

	get m13(): number {
		return this.nativeInstance.m13;
	}

	set m13(value) {
		this.nativeInstance.m13 = value;
	}

	get m14(): number {
		return this.nativeInstance.m14;
	}

	set m14(value) {
		this.nativeInstance.m14 = value;
	}

	get m21(): number {
		return this.nativeInstance.m21;
	}

	set m21(value) {
		this.nativeInstance.m21 = value;
	}

	get m22(): number {
		return this.nativeInstance.m22;
	}

	set m22(value) {
		this.nativeInstance.m22 = value;
	}

	get m23(): number {
		return this.nativeInstance.m23;
	}

	set m23(value) {
		this.nativeInstance.m23 = value;
	}

	get m24(): number {
		return this.nativeInstance.m24;
	}

	set m24(value) {
		this.nativeInstance.m24 = value;
	}

	get m31(): number {
		return this.nativeInstance.m31;
	}

	set m31(value) {
		this.nativeInstance.m31 = value;
	}

	get m32(): number {
		return this.nativeInstance.m32;
	}

	set m32(value) {
		this.nativeInstance.m32 = value;
	}

	get m33(): number {
		return this.nativeInstance.m33;
	}

	set m33(value) {
		this.nativeInstance.m33 = value;
	}

	get m34(): number {
		return this.nativeInstance.m34;
	}

	set m34(value) {
		this.nativeInstance.m34 = value;
	}

	get m41(): number {
		return this.nativeInstance.m41;
	}

	set m41(value) {
		this.nativeInstance.m41 = value;
	}

	get m42(): number {
		return this.nativeInstance.m42;
	}

	set m42(value) {
		this.nativeInstance.m42 = value;
	}

	get m43(): number {
		return this.nativeInstance.m43;
	}

	set m43(value) {
		this.nativeInstance.m43 = value;
	}

	get m44(): number {
		return this.nativeInstance.m44;
	}

	set m44(value) {
		this.nativeInstance.m44 = value;
	}

	translate(x: number, y: number) {
		const val = this.native.translate(x, y, this.native);
		if (val) {
			return new DOMMatrix(val);
		}

		return null;
	}

	translateSelf(x: number, y: number) {
		this.native.translateSelf(x, y);
	}

	multiplySelf(matrix: DOMMatrix) {
		this.native.multiplySelf(matrix);
	}

	preMultiplySelf(matrix: DOMMatrix) {
		this.native.premultiplySelf(matrix);
	}

	scaleNonUniform(x: number, y: number) {
		const val = this.native.scaleNonUniform(x, y, this.native);
		if (val) {
			return new DOMMatrix(val);
		}

		return null;
	}

	scale(x: number, y: number) {
		return this.scaleNonUniform(x, y);
	}

	scaleNonUniformSelf(x: number, y: number) {
		this.native.scaleNonUniformSelf(x, y);
	}

	scaleSelf(x: number, y: number) {
		this.scaleNonUniformSelf(x, y);
	}

	rotate(angle: number, cx: number, cy: number) {
		const val = this.native.rotate(angle, cx ?? 0, cy ?? 0, this.native);
		if (val) {
			return new DOMMatrix(val);
		}

		return null;
	}

	rotateSelf(angle: number, cx: number, cy: number) {
		this.native.rotateSelf(angle, cx, cy);
	}

	skewX(angle: number) {
		const val = this.native.skewX(angle, this.native);
		if (val) {
			return new DOMMatrix(val);
		}

		return null;
	}

	skewXSelf(angle: number) {
		this.native.skewXSelf(angle);
	}

	skewY(angle: number) {
		const val = this.native.skewY(angle, this.native);
		if (val) {
			return new DOMMatrix(val);
		}

		return null;
	}

	skewYSelf(angle: number) {
		this.native.skewYSelf(angle);
	}

	toJSON() {
		return {
			a: this.a,
			b: this.b,
			c: this.c,
			d: this.d,
			e: this.e,
			f: this.f,

			m11: this.m11,
			m12: this.m12,
			m13: this.m13,
			m14: this.m14,

			m21: this.m21,
			m22: this.m22,
			m23: this.m23,
			m24: this.m24,

			m31: this.m31,
			m32: this.m32,
			m33: this.m33,
			m34: this.m34,

			m41: this.m41,
			m42: this.m42,
			m43: this.m43,
			m44: this.m44,
		};
	}
}

/*
export class DOMMatrix {
	private matrix: Float64Array;
	constructor(init?) {
		if (init instanceof DOMMatrix) {
			this.matrix = init.matrix.slice();
		} else {
			this.matrix = new Float64Array([1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
		}
	}

	setIdentity() {
		const m = this.matrix;
		m[0] = m[5] = m[10] = m[15] = 1;
		for (let i = 0; i < 16; i++) {
			if (![0, 5, 10, 15].includes(i)) m[i] = 0;
		}
	}

	get is2D() {
		const m = this.matrix;
		return m[2] === 0 && m[3] === 0 && m[6] === 0 && m[7] === 0 && m[8] === 0 && m[9] === 0 && m[10] === 1 && m[11] === 0 && m[14] === 0 && m[15] === 1;
	}

	get a() {
		return this.m11;
	}
	set a(v) {
		this.m11 = v;
	}

	get b() {
		return this.m12;
	}
	set b(v) {
		this.m12 = v;
	}

	get c() {
		return this.m21;
	}
	set c(v) {
		this.m21 = v;
	}

	get d() {
		return this.m22;
	}
	set d(v) {
		this.m22 = v;
	}

	get e() {
		return this.m41;
	}
	set e(v) {
		this.m41 = v;
	}

	get f() {
		return this.m42;
	}
	set f(v) {
		this.m42 = v;
	}

	get m11() {
		return this.matrix[0];
	}
	set m11(v) {
		this.matrix[0] = v;
	}

	get m12() {
		return this.matrix[1];
	}
	set m12(v) {
		this.matrix[1] = v;
	}

	get m13() {
		return this.matrix[2];
	}
	set m13(v) {
		this.matrix[2] = v;
	}

	get m14() {
		return this.matrix[3];
	}
	set m14(v) {
		this.matrix[3] = v;
	}

	get m21() {
		return this.matrix[4];
	}
	set m21(v) {
		this.matrix[4] = v;
	}

	get m22() {
		return this.matrix[5];
	}
	set m22(v) {
		this.matrix[5] = v;
	}

	get m23() {
		return this.matrix[6];
	}
	set m23(v) {
		this.matrix[6] = v;
	}

	get m24() {
		return this.matrix[7];
	}
	set m24(v) {
		this.matrix[7] = v;
	}

	get m31() {
		return this.matrix[8];
	}
	set m31(v) {
		this.matrix[8] = v;
	}

	get m32() {
		return this.matrix[9];
	}
	set m32(v) {
		this.matrix[9] = v;
	}

	get m33() {
		return this.matrix[10];
	}
	set m33(v) {
		this.matrix[10] = v;
	}

	get m34() {
		return this.matrix[11];
	}
	set m34(v) {
		this.matrix[11] = v;
	}

	get m41() {
		return this.matrix[12];
	}
	set m41(v) {
		this.matrix[12] = v;
	}

	get m42() {
		return this.matrix[13];
	}
	set m42(v) {
		this.matrix[13] = v;
	}

	get m43() {
		return this.matrix[14];
	}
	set m43(v) {
		this.matrix[14] = v;
	}

	get m44() {
		return this.matrix[15];
	}
	set m44(v) {
		this.matrix[15] = v;
	}

	static multiply(a, b) {
		const result = new DOMMatrix();
		for (let row = 0; row < 4; row++) {
			for (let col = 0; col < 4; col++) {
				for (let i = 0; i < 4; i++) {
					result.matrix[col * 4 + row] += a.m[i * 4 + row] * b.m[col * 4 + i];
				}
			}
		}

		return result;
	}

	multiplySelf(matrix) {
		this.matrix = DOMMatrix.multiply(this, matrix).matrix;
		return this;
	}

	preMultiplySelf(matrix) {
		this.matrix = DOMMatrix.multiply(matrix, this).matrix;
		return this;
	}

	translateSelf(tx, ty, tz = 0) {
		return this.multiplySelf(DOMMatrix.translation(tx, ty, tz));
	}

	scaleSelf(sx, sy = sx, sz = 1) {
		return this.multiplySelf(DOMMatrix.scaling(sx, sy, sz));
	}

	rotateSelf(angleX, angleY = 0, angleZ = 0) {
		const rx = DOMMatrix.rotationX(angleX);
		const ry = DOMMatrix.rotationY(angleY);
		const rz = DOMMatrix.rotationZ(angleZ);
		return this.multiplySelf(rx).multiplySelf(ry).multiplySelf(rz);
	}

	static translation(tx, ty, tz) {
		const m = new DOMMatrix();
		m.matrix[12] = tx;
		m.matrix[13] = ty;
		m.matrix[14] = tz;
		return m;
	}

	static scaling(sx, sy, sz) {
		const m = new DOMMatrix();
		m.matrix[0] = sx;
		m.matrix[5] = sy;
		m.matrix[10] = sz;
		return m;
	}

	static rotationX(angle) {
		const rad = (angle * Math.PI) / 180;
		const c = Math.cos(rad),
			s = Math.sin(rad);
		const m = new DOMMatrix();
		m.matrix[5] = c;
		m.matrix[6] = s;
		m.matrix[9] = -s;
		m.matrix[10] = c;
		return m;
	}

	static rotationY(angle) {
		const rad = (angle * Math.PI) / 180;
		const c = Math.cos(rad),
			s = Math.sin(rad);
		const m = new DOMMatrix();
		m.matrix[0] = c;
		m.matrix[2] = -s;
		m.matrix[8] = s;
		m.matrix[10] = c;
		return m;
	}

	static rotationZ(angle) {
		const rad = (angle * Math.PI) / 180;
		const c = Math.cos(rad),
			s = Math.sin(rad);
		const m = new DOMMatrix();
		m.matrix[0] = c;
		m.matrix[1] = s;
		m.matrix[4] = -s;
		m.matrix[5] = c;
		return m;
	}

	clone() {
		return new DOMMatrix(this);
	}

	toString() {
		return `DOMMatrix(${Array.from(this.matrix)
			.map((n) => n.toFixed(2))
			.join(', ')})`;
	}
}
*/
