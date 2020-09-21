import {DOMMatrixBase} from "../DOMMatrix/common";

export abstract class CanvasPatternBase {
	protected nativeInstance: any;

	protected constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	get native() {
		return this.nativeInstance;
	}

	public abstract setTransform(matrix: DOMMatrixBase);
}
