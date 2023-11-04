import { Helpers } from '../../helpers';
let ctor;
export class ImageData {
	static {
		Helpers.initialize();
		ctor = global.CanvasModule.ImageData;
	}
	_native;
	constructor(width: number, height: number);
	constructor(width: number, height: number);
	constructor(width: number, height: number, settings);
	constructor(dataArray: Uint8ClampedArray, width: number);
	constructor(dataArray: Uint8ClampedArray, width: number, height: number);
	constructor(dataArray: Uint8ClampedArray, width: number, height: number, settings);
	constructor(param0, param1, param2?, param3?) {
		if (typeof param0?.height === 'number') {
			this._native = param0;
			return this;
		}

		const length = arguments.length;

		if (length === 0) {
			throw new TypeError("Failed to construct 'ImageData': 2 arguments required, but only 0 present.");
		} else if (length === 1) {
			throw new TypeError("Failed to construct 'ImageData': 2 arguments required, but only 1 present.");
		}

		if (arguments[0] instanceof Uint8ClampedArray) {
			const data = arguments[0];
			const width = arguments[1];
			const row = width * 4;
			if (data.length % row) {
				throw Error("Failed to construct 'ImageData': The input data length is not a multiple of (4 * width");
			}
			if (length == 2) {
				this._native = new ctor(data, width, data.length / row);
			} else if (length > 2) {
				// TODO support colorSpace
				this._native = new ctor(data, width, arguments[2]);
			}
		} else if (typeof arguments[0] === 'number') {
			const width = arguments[0];
			const height = arguments[1];
			if (length == 2) {
				this._native = new ctor(width, height);
			} else if (length > 2) {
				// TODO support colorSpace
				this._native = new ctor(width, height);
			}
		}

		// todo throw error
	}

	get native() {
		return this._native;
	}

	get data() {
		return this.native.data;
	}

	get height() {
		return this.native.height;
	}

	get width() {
		return this.native.width;
	}

	static fromNative(nativeInstance) {
		return new ImageData(nativeInstance, 0, 0);
	}

	static from(instance: ImageData) {
		return new ImageData(instance.native, 0, 0);
	}
}
