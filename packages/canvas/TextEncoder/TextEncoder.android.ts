import { TextEncoderBase } from './common';
import { Helpers } from '../helpers';
let ctor;

export class TextEncoder extends TextEncoderBase {
	static {
		Helpers.initialize();
		ctor = global.CanvasJSIModule.TextEncoder;
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
		super(ctor(encoding));
		this._getMethod('encode');
	}

	get encoding(): string {
		return this.native.encoding;
	}

	encode(text?: string): Uint8Array {
		if (text === undefined) {
			return new Uint8Array(0);
		} else if (text === null) {
			text = this._encode('null');
		}
		return this._encode(text);
	}
}
