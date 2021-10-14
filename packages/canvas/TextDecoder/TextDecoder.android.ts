import { TextDecoderBase } from './common';
import { Utils } from '../utils';

export class TextDecoder extends TextDecoderBase {
	constructor(encoding: string = 'utf-8') {
		super(new org.nativescript.canvas.TNSTextDecoder(encoding));
	}

	get _isSupported() {
		return Utils.IS_SUPPORTED_TYPED_ARRAYS_VERSION;
	}

	get encoding(): string {
		return this.native.getEncoding();
	}

	decode(buffer?: ArrayBuffer | ArrayBufferView, options?: any): string {
		if (buffer instanceof ArrayBuffer || buffer instanceof Uint8Array || buffer instanceof Int8Array || buffer instanceof Uint16Array || buffer instanceof Int16Array || buffer instanceof Uint32Array || buffer instanceof Int32Array || buffer instanceof Float32Array || buffer instanceof Uint8ClampedArray) {
			if (buffer instanceof Uint8Array || buffer instanceof Int8Array || buffer instanceof Uint8ClampedArray) {
				if (this._isSupported) {
					return this.native.decodeByteBuffer(buffer as any);
				}
				return this.native.decodeByte(Array.from(buffer as any));
			} else if (buffer instanceof Uint16Array || buffer instanceof Int16Array) {
				if (this._isSupported) {
					return this.native.decodeShortBuffer(buffer as any);
				}
				return this.native.decodeShort(Array.from(buffer as any));
			} else if (buffer instanceof Int32Array || buffer instanceof Uint32Array) {
				if (this._isSupported) {
					return this.native.decodeIntBuffer(buffer as any);
				}
				return this.native.decodeInt(Array.from(buffer as any));
			} else if (buffer instanceof Float32Array) {
				if (this._isSupported) {
					return this.native.decodeFloatBuffer(buffer as any);
				}
				return this.native.decodeFloat(Array.from(buffer as any));
			} else if (buffer instanceof Float64Array) {
				if (this._isSupported) {
					return this.native.decodeDoubleBuffer(buffer as any);
				}
				return this.native.decodeDouble(Array.from(buffer as any));
			}

			if (this._isSupported) {
				return this.native.decodeByteBuffer(buffer as any);
			}
			return this.native.decodeByte(Array.from(buffer as any));
		} else {
			return '';
		}
	}
}
