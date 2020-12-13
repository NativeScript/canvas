import { TextEncoderBase } from './common';

declare let TNSTextEncoder;

export class TextEncoder extends TextEncoderBase {
	constructor(encoding: string = 'utf-8') {
		super(TNSTextEncoder.alloc().initWithEncoding(encoding));
	}

	get encoding(): string {
		return this.native.encoding;
	}

	encode(text: string): Uint8Array {
		const raw = this.native.encodeWithText(NSString.stringWithString(text));
		return new Uint8Array(interop.bufferFromData(raw));
	}
}
