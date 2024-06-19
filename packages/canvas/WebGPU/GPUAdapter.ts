import { GPUAdapterInfo } from './GPUAdapterInfo';
import { GPUDevice } from './GPUDevice';

export class GPUSupportedFeatures extends Set {
	get [Symbol.toStringTag]() {
		return 'GPUSupportedFeatures';
	}
}

export class GPUAdapter {
	_native;

	_features: GPUSupportedFeatures;
	get features() {
		if (!this._features) {
			this._features = new GPUSupportedFeatures(this._native.features as any);
		}
		return this._features;
	}

	get isFallbackAdapter() {
		return this._native?.isFallbackAdapter ?? false;
	}

	get limits() {
		return this._native?.limits;
	}

	requestAdapterInfo(): Promise<GPUAdapterInfo> {
		return new Promise((resolve, reject) => {
			const info = this._native.requestAdapterInfo();
			const ret = new GPUAdapterInfo();
			ret._native = info;
			resolve(ret);
		});
	}

	requestDevice(desc?): Promise<GPUDevice> {
		return new Promise((resolve, reject) => {
			const options = desc ?? {};
			this._native.requestDevice(options, (error, device) => {
				if (error) {
					reject(error);
				} else {
					const ret = new GPUDevice();
					ret._native = device;
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
