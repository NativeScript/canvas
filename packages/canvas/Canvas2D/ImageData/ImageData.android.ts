import { ImageDataBase } from './common';

export class ImageData extends ImageDataBase {
	#data: Uint8ClampedArray;
	#width: number;
	#height: number;
	protected constructor(nativeInstance: any) {
		super(nativeInstance);
		// @ts-ignore
		this.#data = new Uint8ClampedArray(ArrayBuffer.from(nativeInstance.getData()));
	}

	get data() {
		return this.#data;
	}

	get height() {
		if (!this.#height) {
			this.#height = this.nativeInstance.getHeight();
		}
		return this.#height;
	}

	get width() {
		if (!this.#width) {
			this.#width = this.nativeInstance.getWidth();
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
