import { CanvasGradientBase } from './common';

export class CanvasGradient extends CanvasGradientBase {
	readonly nativeInstance;

	_addColorStop;

	_getMethod(name: string) {
		if (this._addColorStop === undefined) {
			const ret = this.native[name];
			this._addColorStop = ret;
			return ret;
		}

		return this._addColorStop;
	}

	protected constructor(nativeInstance: any) {
		super();
		this.nativeInstance = nativeInstance;
		this._getMethod('addColorStop');
	}

	get native() {
		return this.nativeInstance;
	}

	static fromNative(nativeInstance) {
		return new CanvasGradient(nativeInstance);
	}

	public addColorStop(offset: number, color: any): void {
		this._addColorStop(offset, color);
	}
}
