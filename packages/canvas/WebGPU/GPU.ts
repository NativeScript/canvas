import { GPUAdapter } from './GPUAdapter';
let gpu;
export class GPU {
	private _wgslLanguageFeatures = new Set();
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
			if (!gpu) {
				gpu = new global.CanvasModule.GPU();
			}

			gpu.requestAdapter(options, (error, adapter) => {
				console.log(error, adapter);
				if (error) {
					reject(error);
				} else {
					const ret = new GPUAdapter();
					ret._native = adapter;
					resolve(ret);
				}
			});
		});
	}
}
