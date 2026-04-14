import { native_ } from './Constants';
import { GPUAdapter } from './GPUAdapter';
let gpu;
export class GPU {
	private _wgslLanguageFeatures = new Set();
	static [native_];

	get native() {
		if (!gpu) {
			gpu = new global.CanvasModule.GPU();
		}
		return gpu;
	}
	get wgslLanguageFeatures() {
		return this._wgslLanguageFeatures;
	}

	getPreferredCanvasFormat() {
		if (__IOS__) {
			return 'bgra8unorm';
		}
		return 'rgba8unorm';
	}

	requestAdapter(options: { powerPreference?: 'low-power' | 'high-performance'; isFallbackAdapter?: boolean; featureLevel?: 'core' | 'compatibility' } = { powerPreference: undefined, isFallbackAdapter: false }) {
		return new Promise<GPUAdapter | null>((resolve, reject) => {
			this.native.requestAdapter(options, (error, adapter) => {
				if (error) {
					reject(error);
				} else if (!adapter) {
					// No adapter available (no hardware Vulkan, GL fallback also unavailable).
					resolve(null);
				} else {
					resolve(GPUAdapter.fromNative(adapter));
				}
			});
		});
	}
}
