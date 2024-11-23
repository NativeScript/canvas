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
		if (global.isAndroid) {
			__non_webpack_require__('system_lib://libcanvasnativev8.so');
			this._initialized = true;
		}

		if (global.isIOS) {
			const cm = new CanvasModule();
			cm.install();
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
