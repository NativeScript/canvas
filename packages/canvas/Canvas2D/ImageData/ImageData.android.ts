import { Helpers } from '../../helpers';
import { ImageDataBase } from './common';
let ctor;
export class ImageData extends ImageDataBase {
	static {
		Helpers.initialize();
		ctor = global.CanvasJSIModule.ImageData;
	}
	constructor(instance) {
		super(instance);

		if (instance?.height) {
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
				this.nativeInstance = ctor(data, width, data.length / row);
			} else if (length > 2) {
				// TODO support colorSpace
				this.nativeInstance = ctor(data, width, arguments[2]);
			}
		} else if (typeof arguments[0] === 'number') {
			const width = arguments[0];
			const height = arguments[1];
			if (length == 2) {
				this.nativeInstance = ctor(width, height);
			} else if (length > 2) {
				// TODO support colorSpace
				this.nativeInstance = ctor(width, height);
			}
		}

		// todo throw error
	}

	get data() {
		return this.nativeInstance.data;
	}

	get height() {
		return this.nativeInstance.height;
	}

	get width() {
		return this.nativeInstance.width;
	}

	static fromNative(nativeInstance) {
		return new ImageData(nativeInstance);
	}

	static from(instance: ImageData) {
		return new ImageData(instance.nativeInstance);
	}
}
