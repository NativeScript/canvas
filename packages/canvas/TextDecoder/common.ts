export class TextDecoderBase {
	public readonly encoding: string;
	private nativeInstance: any;

	constructor(nativeInstance) {
		this.nativeInstance = nativeInstance;
	}

	get native(): any {
		return this.nativeInstance;
	}

	decode(buffer: ArrayBuffer | ArrayBufferView, options?: any): string {
		return null;
	}
}
