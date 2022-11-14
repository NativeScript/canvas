export class WebGLProgram {
	[Symbol.toStringTag] = 'WebGLProgram';
	private readonly nativeInstance: any;

	constructor(nativeInstance: any) {
		this.nativeInstance = nativeInstance;
	}

	public get native() {
		return this.nativeInstance;
	}
}
