import { TextDecoderBase } from './common';

//declare let TNSTextDecoder;

export class TextDecoder extends TextDecoderBase {
	constructor(encoding: string = 'utf-8') {
		super(TNSTextDecoder.alloc().initWithEncoding(encoding));
	}

	get encoding(): string {
		return this.native.encoding;
	}

	decode(buffer?: ArrayBuffer | ArrayBufferView, options?: any): string {
		if (buffer instanceof ArrayBuffer || buffer instanceof Uint8ClampedArray || buffer instanceof Uint8Array || buffer instanceof Int8Array || buffer instanceof Uint16Array || buffer instanceof Int16Array || buffer instanceof Uint32Array || buffer instanceof Int32Array || buffer instanceof Float32Array) {
			if (buffer instanceof Uint8Array || buffer instanceof Uint8ClampedArray) {
				return this.native.decodeWithU8Offset(<any>buffer, buffer.byteLength, buffer.byteOffset);
			} else if (buffer instanceof Int8Array) {
				return this.native.decodeWithI8Offset(<any>buffer, buffer.byteLength, buffer.byteOffset);
			} else if (buffer instanceof Uint16Array) {
				return this.native.decodeWithU16Offset(<any>buffer, buffer.byteLength, buffer.byteOffset);
			} else if (buffer instanceof Int16Array) {
				return this.native.decodeWithI16Offset(<any>buffer, buffer.byteLength, buffer.byteOffset);
			} else if (buffer instanceof Int32Array) {
				return this.native.decodeWithI32Offset(<any>buffer, buffer.byteLength, buffer.byteOffset);
			} else if (buffer instanceof Uint32Array) {
				return this.native.decodeWithI32Offset(<any>buffer, buffer.byteLength, buffer.byteOffset);
			}

			return this.native.decodeWithU8(buffer as any, buffer.byteLength);
		} else {
			return '';
		}
	}
}
