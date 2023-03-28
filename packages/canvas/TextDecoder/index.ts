import { Helpers } from '../helpers';
import { profile } from '@nativescript/core';
let ctor;

export class TextDecoder {
	static {
		Helpers.initialize();
		ctor = global.CanvasJSIModule.TextDecoder;
	}

	_native;
	get native() {
		return this._native;
	}
	_decode;

	_getMethod(name: string) {
		if (this._decode === undefined) {
			const ret = this.native[name];
			this._decode = ret;
			return ret;
		}

		return this._decode;
	}

	constructor(encoding: string = 'utf-8') {
		this._native = ctor(encoding);
		this._getMethod('decode');
	}

	get encoding(): string {
		return this.native.encoding;
	}

	@profile
	decode(buffer?: ArrayBuffer | ArrayBufferView, options?: any): string {
		if (buffer instanceof ArrayBuffer || buffer instanceof Uint8Array || buffer instanceof Int8Array || buffer instanceof Uint16Array || buffer instanceof Int16Array || buffer instanceof Uint32Array || buffer instanceof Int32Array || buffer instanceof Float32Array || buffer instanceof Uint8ClampedArray) {
			if (buffer.byteLength === 0) {
				return '';
			}
			return this._decode(buffer);
		} else {
			return '';
		}
	}
}
