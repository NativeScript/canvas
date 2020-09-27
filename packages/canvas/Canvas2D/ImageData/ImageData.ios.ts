import { ImageDataBase } from './common';

export class ImageData extends ImageDataBase {
	constructor(nativeInstance: any) {
		super(nativeInstance);
		this.data = new Uint8ClampedArray(interop.bufferFromData(nativeInstance.data));
	}

	static fromNative(nativeInstance) {
		return new ImageData(nativeInstance);
	}

	static from(instance: ImageData) {
		return new ImageData(instance.nativeInstance);
	}
}
