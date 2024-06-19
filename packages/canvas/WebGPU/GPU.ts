import { GPUAdapter } from './GPUAdapter';

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
			global.CanvasModule.GPU.requestAdapter(options, (error, adapter) => {
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
