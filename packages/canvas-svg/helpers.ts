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

		if (!this._initialized) {
			// e.g. Windows, until its native SVG module exists.
			throw new Error('@nativescript/canvas-svg is not supported on this platform yet');
		}
	}
}
