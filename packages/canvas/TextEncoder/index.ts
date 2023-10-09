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

	constructor(encoding: string = 'utf8') {
		this._native = new ctor(encoding);
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
