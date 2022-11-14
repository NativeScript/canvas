export class ImageAssetBase {
	private nativeInstance: any;

	constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	get native() {
		return this.nativeInstance;
	}

	static [Symbol.hasInstance](obj) {
		if (obj?.native && obj.constructor.name === 'ImageAsset') return true;
	}

	get [Symbol.toStringTag]() {
		return 'ImageAsset';
	}
}

export enum ImageAssetSaveFormat {
	JPG,
	PNG,
	ICO,
	BMP,
	TIFF,
}
