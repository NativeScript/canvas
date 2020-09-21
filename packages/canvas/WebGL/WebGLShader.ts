export class WebGLShader {
	[Symbol.toStringTag] = 'WebGLShader';
	private readonly nativeInstance: any;

	constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	public get native() {
		return this.nativeInstance;
	}

	public toString() {
		return '[object WebGLShader]';
	}
}
