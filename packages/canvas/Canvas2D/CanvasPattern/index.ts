import { DOMMatrix } from '../DOMMatrix';

export class CanvasPattern {
	private _native;

	constructor(instance?) {
		this._native = instance;
	}

	get native() {
		return this._native;
	}

	public setTransform(matrix: DOMMatrix) {
		this.native.setTransform(matrix.native);
	}
}
