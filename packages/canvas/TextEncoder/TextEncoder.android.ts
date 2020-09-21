import {TextEncoderBase} from './common';

declare var com;

export class TextEncoder extends TextEncoderBase {
	constructor(encoding: string = 'utf8') {
		super(new com.github.triniwiz.canvas.TextEncoder(encoding));
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
		const value = java.nio.ByteBuffer.wrap(this.native.encode(text));
		const buf = (ArrayBuffer as any).from(value);
		return new Uint8Array(buf);
	}
}
