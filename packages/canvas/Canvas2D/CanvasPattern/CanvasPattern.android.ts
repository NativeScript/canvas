import { CanvasPatternBase } from './common';
import { DOMMatrix } from '../DOMMatrix/DOMMatrix';

export class CanvasPattern extends CanvasPatternBase {
	_setTransform;

	_getMethod(name: string) {
		if (this._setTransform === undefined) {
			const ret = this.native[name];
			this._setTransform = ret;
			return ret;
		}

		return this._setTransform;
	}

	constructor(instance: any) {
		super(instance);
		this.nativeInstance = instance;
		this._getMethod('setTransform');
	}

	public setTransform(matrix: DOMMatrix) {
		this._setTransform(matrix.native);
	}
}
