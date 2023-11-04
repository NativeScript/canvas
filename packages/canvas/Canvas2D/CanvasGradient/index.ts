export class CanvasGradient {
	private _native;

	constructor(nativeInstance?: any) {
		this._native = nativeInstance;
	}

	get native() {
		return this._native;
	}

	static fromNative(nativeInstance) {
		return new CanvasGradient(nativeInstance);
	}

	public addColorStop(offset: number, color: any): void {
		this.native.addColorStop(offset, color);
	}
}
