declare var __non_webpack_require__, CanvasSVGModule;
declare const __WINDOWS__: boolean;

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

		if (typeof __WINDOWS__ !== 'undefined' && __WINDOWS__) {
			// The Node-API module (crates/canvas-svg-napi) installs global.SVGModule itself.
			__non_webpack_require__('system_lib://canvassvg.node');
			this._initialized = true;
		}

		if (!this._initialized) {
			throw new Error('@nativescript/canvas-svg is not supported on this platform yet');
		}
	}
}
