export abstract class TextEncoderBase {
	public abstract readonly encoding: string;
	private nativeInstance: any;

	constructor(nativeInstance) {
		this.nativeInstance = nativeInstance;
	}

	get native(): any {
		return this.nativeInstance;
	}

	abstract encode(text: string): Uint8Array;

	static [Symbol.hasInstance](obj) {
		if (obj.native && obj.constructor.name === 'TextEncoder') return true;
	}
}
