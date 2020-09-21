export class ImageDataBase {
	width = 0;
	height = 0;
	data = null;
	protected nativeInstance: any;

	constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	get native() {
		return this.nativeInstance;
	}
}
