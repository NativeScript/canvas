export class CanvasGradient {
	_native;
	_addColorStop;

	_getMethod(name: string) {
		if (this._addColorStop === undefined) {
			const ret = this.native[name];
			this._addColorStop = ret;
			return ret;
		}

		return this._addColorStop;
	}

	constructor(nativeInstance?: any) {
		this._native = nativeInstance;
		this._getMethod('addColorStop');
	}

	get native() {
		return this._native;
	}

	static fromNative(nativeInstance) {
		return new CanvasGradient(nativeInstance);
	}

	public addColorStop(offset: number, color: any): void {
		this._addColorStop(offset, color);
	}
}
