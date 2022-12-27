export abstract class DOMMatrixBase {
	abstract a: number;
	abstract b: number;
	abstract c: number;
	abstract d: number;
	abstract e: number;
	abstract f: number;

	abstract m11: number;
	abstract m12: number;
	abstract m13: number;
	abstract m14: number;
	abstract m21: number;
	abstract m22: number;
	abstract m23: number;
	abstract m24: number;
	abstract m31: number;
	abstract m32: number;
	abstract m33: number;
	abstract m34: number;
	abstract m41: number;
	abstract m42: number;
	abstract m43: number;
	abstract m44: number;

	protected nativeInstance: any;

	protected constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	get native() {
		return this.nativeInstance;
	}

	static [Symbol.hasInstance](obj) {
		if (obj?.native && obj.constructor.name === 'DOMMatrix') return true;
	}

	get [Symbol.toStringTag]() {
		return 'DOMMatrix';
	}
}
