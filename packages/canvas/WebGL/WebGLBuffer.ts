export class WebGLBuffer {
	[Symbol.toStringTag] = 'WebGLBuffer';
	private readonly nativeInstance: any;

	constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	public get native() {
		return this.nativeInstance;
	}

	public toString() {
		return '[object WebGLBuffer]';
	}
}
