import {ImageDataBase} from './common';

export class ImageData extends ImageDataBase {
	private _data: Uint8ClampedArray;

	constructor(nativeInstance: any) {
		super(nativeInstance);
		this._data = new Uint8ClampedArray(interop.bufferFromData(nativeInstance.data));
	}

	static fromNative(nativeInstance) {
		return new ImageData(nativeInstance);
	}

	static from(instance: ImageData) {
		return new ImageData(instance.nativeInstance);
	}

	public get width() {
		return this.nativeInstance.width;
	}

	public get height() {
		return this.nativeInstance.height;
	}

	public get data() {
		return this._data;
	}
}
