import {TextEncoderBase} from './common';

declare var com;

export class TextEncoder extends TextEncoderBase {
	constructor(encoding: string = 'utf8') {
		super(new com.github.triniwiz.canvas.TNSTextEncoder(encoding));
	}

	get encoding(): string {
		return this.native.getEncoding();
	}

	encode(text?: string): Uint8Array {
		if (text === undefined) {
			return new Uint8Array(0);
		} else if (text === null) {
			text = 'null';
		}
		const buf = (ArrayBuffer as any).from(this.native.encode(text));
		return new Uint8Array(buf);
	}
}