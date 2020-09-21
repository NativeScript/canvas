import {ImageDataBase} from './common';

export class ImageData extends ImageDataBase {
	protected constructor(nativeInstance: any) {
		super(nativeInstance);
		// @ts-ignore
		this.data = new Uint8ClampedArray(ArrayBuffer.from(nativeInstance.getData()));
	}

	static fromNative(nativeInstance) {
		return new ImageData(nativeInstance);
	}

	static from(instance: ImageData) {
		return new ImageData(instance.nativeInstance);
	}
}
