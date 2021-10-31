export abstract class ImageDataBase {
	abstract readonly width: number;
	abstract readonly height: number;
	abstract readonly data: Uint8ClampedArray;
	protected nativeInstance: any;

	constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	get native() {
		return this.nativeInstance;
	}

	static [Symbol.hasInstance](obj) {
		if (obj.native && obj.constructor.name === 'ImageData') return true;
	}
}
