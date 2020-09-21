export class WebGLUniformLocation {
	[Symbol.toStringTag] = 'WebGLUniformLocation';
	private readonly nativeInstance: any;

	constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	public get native() {
		return this.nativeInstance;
	}

	public toString() {
		return '[object WebGLUniformLocation]';
	}
}
