export class TextEncoderBase {
	public readonly encoding: string;
	private nativeInstance: any;

	constructor(nativeInstance) {
		this.nativeInstance = nativeInstance;
	}

	get native(): any {
		return this.nativeInstance;
	}

	encode(text: string): Uint8Array {
		return null;
	}
}
