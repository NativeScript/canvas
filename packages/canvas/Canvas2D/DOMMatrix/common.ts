export abstract class DOMMatrixBase {
	a: number;
	b: number;
	c: number;
	d: number;
	e: number;
	f: number;
	protected nativeInstance: any;

	protected constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	get native() {
		return this.nativeInstance;
	}
}
