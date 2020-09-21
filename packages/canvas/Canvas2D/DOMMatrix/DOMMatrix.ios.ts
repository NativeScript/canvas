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
}
