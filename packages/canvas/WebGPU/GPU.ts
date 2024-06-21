import { native_ } from './Constants';
import { GPUAdapter } from './GPUAdapter';
export class GPU {
	private _wgslLanguageFeatures = new Set();
	static [native_];

	get native() {
		if (!this[native_]) {
			this[native_] = new global.CanvasModule.GPU();
		}
		return this[native_];
	}
	get wgslLanguageFeatures() {
		return this._wgslLanguageFeatures;
	}

	getPreferredCanvasFormat() {
		if (global.isIOS) {
			return 'bgra8unorm';
		}
		return 'rgba8unorm';
	}

	requestAdapter(options: { powerPreference?: 'low-power' | 'high-performance'; isFallbackAdapter?: boolean } = { powerPreference: undefined, isFallbackAdapter: false }) {
		return new Promise<GPUAdapter>((resolve, reject) => {
			this.native.requestAdapter(options, (error, adapter) => {
				console.log(error, adapter);
				if (error) {
					reject(error);
				} else {
					resolve(GPUAdapter.fromNative(adapter));
				}
			});
		});
	}
}
