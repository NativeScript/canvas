import { ImageDataBase } from './common';

export class ImageData extends ImageDataBase {
	#data: Uint8ClampedArray;
	#width: number;
	#height: number;
	constructor(nativeInstance: any) {
		super(nativeInstance);
		this.#data = new Uint8ClampedArray(interop.bufferFromData(nativeInstance.data));
	}

	get data() {
		return this.#data;
	}

	get height() {
		if (!this.#height) {
			this.#height = this.nativeInstance.height;
		}
		return this.#height;
	}

	get width() {
		if (!this.#width) {
			this.#width = this.nativeInstance.width
		}
		return this.#width;
	}

	static fromNative(nativeInstance) {
		return new ImageData(nativeInstance);
	}

	static from(instance: ImageData) {
		return new ImageData(instance.nativeInstance);
	}
}
