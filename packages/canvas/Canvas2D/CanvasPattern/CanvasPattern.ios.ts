import {CanvasPatternBase} from './common';
import {DOMMatrix} from '../DOMMatrix/DOMMatrix';

export class CanvasPattern extends CanvasPatternBase {
	constructor(instance: any) {
		super(instance);
	}

	public setTransform(matrix: DOMMatrix) {
		this.native.setTransformWithMatrix(matrix.native);
	}
}
