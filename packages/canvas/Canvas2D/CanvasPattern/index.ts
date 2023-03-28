import { DOMMatrix } from '../DOMMatrix';

export class CanvasPattern {
	_setTransform;
	_native;
	_getMethod(name: string) {
		if (this._setTransform === undefined) {
			const ret = this.native[name];
			this._setTransform = ret;
			return ret;
		}

		return this._setTransform;
	}

	constructor(instance?) {
		this._native = instance;
		this._getMethod('setTransform');
	}

	get native() {
		return this._native;
	}

	public setTransform(matrix: DOMMatrix) {
		this._setTransform(matrix.native);
	}
}
