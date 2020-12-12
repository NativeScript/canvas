import {DOMMatrixBase} from './common';

export class DOMMatrix extends DOMMatrixBase {
	constructor(instance) {
		super(instance);
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

	public get m11(): number {
		return this.nativeInstance.m11;
	}

	public set m11(value: number) {
		this.nativeInstance.m11 = value;
	}

	public get m12(): number {
		return this.nativeInstance.m12;
	}

	public set m12(value: number) {
		this.nativeInstance.m12 = value;
	}

	public get m13(): number {
		return this.nativeInstance.m13;
	}

	public set m13(value: number) {
		this.nativeInstance.m13 = value;
	}

	public get m14(): number {
		return this.nativeInstance.m14;
	}

	public set m14(value: number) {
		this.nativeInstance.m14 = value;
	}

	public get m21(): number {
		return this.nativeInstance.m21;
	}

	public set m21(value: number) {
		this.nativeInstance.m11 = value;
	}

	public get m22(): number {
		return this.nativeInstance.m22;
	}

	public set m22(value: number) {
		this.nativeInstance.m22 = value;
	}

	public get m23(): number {
		return this.nativeInstance.m23;
	}

	public set m23(value: number) {
		this.nativeInstance.m23 = value;
	}

	public get m24(): number {
		return this.nativeInstance.m24;
	}

	public set m24(value: number) {
		this.nativeInstance.m24 = value;
	}

	public get m31(): number {
		return this.nativeInstance.m31;
	}

	public set m31(value: number) {
		this.nativeInstance.m31 = value;
	}

	public get m32(): number {
		return this.nativeInstance.m32;
	}

	public set m32(value: number) {
		this.nativeInstance.m32 = value;
	}

	public get m33(): number {
		return this.nativeInstance.m33;
	}

	public set m33(value: number) {
		this.nativeInstance.m33 = value;
	}

	public get m34(): number {
		return this.nativeInstance.m34;
	}

	public set m34(value: number) {
		this.nativeInstance.m34 = value;
	}

	public get m41(): number {
		return this.nativeInstance.m41;
	}

	public set m41(value: number) {
		this.nativeInstance.m41 = value;
	}

	public get m42(): number {
		return this.nativeInstance.m42;
	}

	public set m42(value: number) {
		this.nativeInstance.m42 = value;
	}

	public get m43(): number {
		return this.nativeInstance.m43;
	}

	public set m43(value: number) {
		this.nativeInstance.m43 = value;
	}

	public get m44(): number {
		return this.nativeInstance.m44;
	}

	public set m44(value: number) {
		this.nativeInstance.m44 = value;
	}
}
