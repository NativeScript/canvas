import { native_ } from './Constants';
import { GPUAdapterInfo } from './GPUAdapterInfo';
import { GPUDevice } from './GPUDevice';

export class GPUSupportedFeatures extends Set {
	get [Symbol.toStringTag]() {
		return 'GPUSupportedFeatures';
	}
}

export class GPUAdapter {
	[native_];

	_features: GPUSupportedFeatures;
	get features() {
		if (!this._features) {
			this._features = new GPUSupportedFeatures(this[native_].features as any);
		}
		return this._features;
	}

	get isFallbackAdapter() {
		return this[native_]?.isFallbackAdapter ?? false;
	}

	get limits() {
		return this[native_]?.limits;
	}

	get native() {
		return this[native_];
	}

	static fromNative(adapter) {
		if (adapter) {
			const ret = new GPUAdapter();
			ret[native_] = adapter;
			return ret;
		}
		return null;
	}

	requestAdapterInfo(): Promise<GPUAdapterInfo> {
		return new Promise((resolve, reject) => {
			const info = this[native_].requestAdapterInfo();
			const ret = new GPUAdapterInfo();
			ret._native = info;
			resolve(ret);
		});
	}

	requestDevice(desc?): Promise<GPUDevice> {
		return new Promise((resolve, reject) => {
			const options = desc ?? {};
			this[native_].requestDevice(options, (error, device) => {
				if (error) {
					reject(error);
				} else {
					const ret = GPUDevice.fromNative(device);
					resolve(ret);
				}
			});
		});
	}

	toJSON() {
		return {
			name: '',
			isFallbackAdapter: this.isFallbackAdapter,
			limits: this.limits,
			features: this.features,
		};
	}
}
