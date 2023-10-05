import { Helpers } from '../helpers';
import { profile } from '@nativescript/core';
let ctor;

export class TextEncoder {
	static {
		Helpers.initialize();
		ctor = global.CanvasModule.TextEncoder;
	}

	_native;
	get native() {
		return this._native;
	}
	_encode;

	_getMethod(name: string) {
		if (this._encode === undefined) {
			const ret = this.native[name];
			this._encode = ret;
			return ret;
		}

		return this._encode;
	}

	constructor(encoding: string = 'utf8') {
		this._native = new ctor(encoding);
		//this._getMethod('encode');
	}

	get encoding(): string {
		return this.native.encoding;
	}

	@profile
	encode(text?: string): Uint8Array {
		if (text === undefined) {
			return new Uint8Array(0);
		} else if (text === null) {
			text = this.native.encode('null');
		}
		return this.native.encode(text);
	}
}
