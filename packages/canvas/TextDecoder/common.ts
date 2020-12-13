export abstract class TextDecoderBase {
	public abstract readonly encoding: string;
	private nativeInstance: any;

	constructor(nativeInstance) {
		this.nativeInstance = nativeInstance;
	}

	get native(): any {
		return this.nativeInstance;
	}

	abstract decode(buffer: ArrayBuffer | ArrayBufferView, options?: any): string;
}
