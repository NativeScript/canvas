import { Helpers } from '../helpers';
import { profile } from '@nativescript/core';
let ctor;

export class TextDecoder {
	static {
		Helpers.initialize();
		ctor = global.CanvasModule.TextDecoder;
	}

	_native;
	get native() {
		return this._native;
	}

	constructor(encoding: string = 'utf-8') {
		this._native = new ctor(encoding);
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
			return this.native.decode(buffer);
		} else {
			return '';
		}
	}

	@profile
	decodeAsync(buffer?: ArrayBuffer | ArrayBufferView, options?: any): Promise<string> {
		if (buffer instanceof ArrayBuffer || buffer instanceof Uint8Array || buffer instanceof Int8Array || buffer instanceof Uint16Array || buffer instanceof Int16Array || buffer instanceof Uint32Array || buffer instanceof Int32Array || buffer instanceof Float32Array || buffer instanceof Uint8ClampedArray) {
			if (buffer.byteLength === 0) {
				return Promise.resolve('');
			}
			return this.native.decodeAsync(buffer);
		} else {
			return Promise.resolve('');
		}
	}
}
