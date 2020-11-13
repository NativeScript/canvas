export class ImageDataBase {
	readonly width;
	readonly height;
	readonly data;
	protected nativeInstance: any;

	constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	get native() {
		return this.nativeInstance;
	}
}
