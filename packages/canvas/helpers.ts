import { NAPI_HOST } from './platform';

declare var __non_webpack_require__, CanvasModule;
export class Helpers {
	static _initialized = false;

	static get isInitialized() {
		return this._initialized;
	}

	static initialize() {
		if (this._initialized) {
			return;
		}
		if (__ANDROID__) {
			__non_webpack_require__('system_lib://libcanvasnativev8.so');
			this._initialized = true;
		}

		if (__APPLE__) {
			const cm = new CanvasModule();
			cm.install();
			this._initialized = true;
		}

		// Webpack cannot fold NAPI_HOST, and a live __non_webpack_require__ imports
		// `node:module`, which the iOS runtime lacks; the defined flags drop the branch.
		if (!__ANDROID__ && !__APPLE__ && NAPI_HOST) {
			// The addon installs globalThis.CanvasModule when it loads.
			__non_webpack_require__('system_lib://canvasnative.node');
			this._initialized = true;
		}
	}

	static base64Encode(value: string): string {
		return global.CanvasModule.__base64Encode(value);
	}

	static base64Decode(value: string): [string, ArrayBuffer] {
		return global.CanvasModule.__base64Decode(value);
	}

	static base64DecodeAsync(value: string): Promise<[string, ArrayBuffer]> {
		return global.CanvasModule.__base64DecodeAsync(value);
	}
}
