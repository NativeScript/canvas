import {DOMMatrixBase} from './common';

export class DOMMatrix extends DOMMatrixBase {
	constructor(instance) {
		super(instance);
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
}
