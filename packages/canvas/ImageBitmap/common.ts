export abstract class ImageBitmapBase {
	private nativeInstance: any;

	constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	get native() {
		return this.nativeInstance;
	}

	abstract readonly width: number;
	abstract readonly height: number;
	abstract close();
}
