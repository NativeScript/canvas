import { DOMMatrixBase } from './common';
import { Helpers } from '../../helpers';
let ctor;

export class DOMMatrix extends DOMMatrixBase {
	static {
		Helpers.initialize();
		ctor = global.CanvasJSIModule.DOMMatrix;
	}
	constructor(instance) {
		super(instance);
		if (Array.isArray(instance)) {
			this.nativeInstance = ctor(instance);
		} else if (!instance) {
			this.nativeInstance = ctor();
		}
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
