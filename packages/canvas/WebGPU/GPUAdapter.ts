import { native_ } from './Constants';
import { GPUAdapterInfo } from './GPUAdapterInfo';
import { GPUDevice } from './GPUDevice';
import { GPUQueue } from './GPUQueue';
import { GPUDeviceDescriptor } from './Interfaces';

export class GPUSupportedFeatures extends Set<string> {
	get [Symbol.toStringTag]() {
		return 'GPUSupportedFeatures';
	}
}

export class GPUAdapter {
	[native_];

	_features: GPUSupportedFeatures;
	get features() {
		if (!this._features) {
			this._features = new GPUSupportedFeatures(this[native_].features);
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
			resolve(GPUAdapterInfo.fromNative(info));
		});
	}

	requestDevice(desc?: GPUDeviceDescriptor): Promise<GPUDevice> {
		return new Promise((resolve, reject) => {
			const options = desc ?? {};
			this[native_].requestDevice(options, (error, device) => {
				if (error) {
					reject(error);
				} else {
					const ret = GPUDevice.fromNative(device, this);
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
