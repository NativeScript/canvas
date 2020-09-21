import {CanvasPatternBase} from './common';
import {DOMMatrix} from '../DOMMatrix/DOMMatrix';

export class CanvasPattern extends CanvasPatternBase {
	constructor(instance: any) {
		super(instance);
		this.nativeInstance = instance;
	}

	public setTransform(matrix: DOMMatrix) {
		this.native.setTransform(matrix.native);
	}
}
