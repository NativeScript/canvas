import { DOMMatrixBase } from './common';

export class DOMMatrix extends DOMMatrixBase {
	constructor(instance) {
		super(instance);
		if (!instance) {
			this.nativeInstance = org.nativescript.canvas.TNSCanvas.createSVGMatrix();
		}
	}

	get a(): number {
		return this.nativeInstance.getA();
	}

	set a(value) {
		this.nativeInstance.setA(value);
	}

	get b(): number {
		return this.nativeInstance.getB();
	}

	set b(value) {
		this.nativeInstance.setB(value);
	}

	get c(): number {
		return this.nativeInstance.getC();
	}

	set c(value) {
		this.nativeInstance.setC(value);
	}

	get d(): number {
		return this.nativeInstance.getD();
	}

	set d(value) {
		this.nativeInstance.setD(value);
	}

	get e(): number {
		return this.nativeInstance.getE();
	}

	set e(value) {
		this.nativeInstance.setE(value);
	}

	get f(): number {
		return this.nativeInstance.getF();
	}

	set f(value) {
		this.nativeInstance.setF(value);
	}

	get m11(): number {
		return this.nativeInstance.getM11();
	}

	set m11(value) {
		this.nativeInstance.setM11(value);
	}

	get m12(): number {
		return this.nativeInstance.getM12();
	}

	set m12(value) {
		this.nativeInstance.setM12(value);
	}

	get m13(): number {
		return this.nativeInstance.getM13();
	}

	set m13(value) {
		this.nativeInstance.setM13(value);
	}

	get m14(): number {
		return this.nativeInstance.getM14();
	}

	set m14(value) {
		this.nativeInstance.setM14(value);
	}


	get m21(): number {
		return this.nativeInstance.getM21();
	}

	set m21(value) {
		this.nativeInstance.setM21(value);
	}

	get m22(): number {
		return this.nativeInstance.getM22();
	}

	set m22(value) {
		this.nativeInstance.setM22(value);
	}

	get m23(): number {
		return this.nativeInstance.getM23();
	}

	set m23(value) {
		this.nativeInstance.setM23(value);
	}

	get m24(): number {
		return this.nativeInstance.getM24();
	}

	set m24(value) {
		this.nativeInstance.setM24(value);
	}

	get m31(): number {
		return this.nativeInstance.getM31();
	}

	set m31(value) {
		this.nativeInstance.setM31(value);
	}

	get m32(): number {
		return this.nativeInstance.getM32();
	}

	set m32(value) {
		this.nativeInstance.setM32(value);
	}

	get m33(): number {
		return this.nativeInstance.getM33();
	}

	set m33(value) {
		this.nativeInstance.setM33(value);
	}

	get m34(): number {
		return this.nativeInstance.getM34();
	}

	set m34(value) {
		this.nativeInstance.setM34(value);
	}

	get m41(): number {
		return this.nativeInstance.getM41();
	}

	set m41(value) {
		this.nativeInstance.setM41(value);
	}

	get m42(): number {
		return this.nativeInstance.getM42();
	}

	set m42(value) {
		this.nativeInstance.setM42(value);
	}

	get m43(): number {
		return this.nativeInstance.getM43();
	}

	set m43(value) {
		this.nativeInstance.setM43(value);
	}

	get m44(): number {
		return this.nativeInstance.getM44();
	}

	set m44(value) {
		this.nativeInstance.setM44(value);
	}
}
