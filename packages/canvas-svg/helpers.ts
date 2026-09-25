declare var __non_webpack_require__, CanvasSVGModule;

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
			__non_webpack_require__('system_lib://libcanvassvgnativev8.so');
			this._initialized = true;
		}

		if (__APPLE__) {
			const csm = new CanvasSVGModule();
			csm.install();
			this._initialized = true;
		}
	}
}
