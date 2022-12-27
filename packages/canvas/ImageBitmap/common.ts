export abstract class ImageBitmapBase {
	private nativeInstance: any;

	constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	get native() {
		return this.nativeInstance;
	}

	static [Symbol.hasInstance](obj) {
		if (obj?.native && obj.constructor.name === 'ImageBitmap') return true;
	}

	abstract readonly width: number;
	abstract readonly height: number;
	abstract close();

	get [Symbol.toStringTag]() {
		return 'ImageBitmap';
	}
}
