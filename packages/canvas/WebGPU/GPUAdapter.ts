import { native_ } from './Constants';
import { GPUAdapterInfo } from './GPUAdapterInfo';
import { GPUDevice } from './GPUDevice';
import { GPUDeviceDescriptor } from './Interfaces';
import { GPUFeatureName } from './Types';

export class GPUSupportedFeatures extends Set<GPUFeatureName> {
	get [Symbol.toStringTag]() {
		return 'GPUSupportedFeatures';
	}
}

export class GPUAdapter {
	[native_];

	private _features: GPUSupportedFeatures;

	get label() {
		return this[native_]?.label ?? '';
	}

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
			if (Array.isArray(options.requiredFeatures)) {
				options.requiredFeatures.push('texture-adapter-specific-format-features' as never);
			} else {
				options.requiredFeatures = ['texture-adapter-specific-format-features' as never];
			}

			if (!options.requiredLimits) {
				if (__ANDROID__) {
					options.requiredLimits = new global.CanvasModule.GPUSupportedLimits();
					const limits = this[native_].limits;
					if (limits.maxSampledTexturesPerShaderStage >= 128) {
						options.requiredLimits.maxSampledTexturesPerShaderStage = 128;
					}

					if (limits.maxSamplersPerShaderStage >= 128) {
						options.requiredLimits.maxSamplersPerShaderStage = 128;
					}
				}
			} else {
				if (options.requiredLimits && typeof options.requiredLimits === 'object' && options.requiredLimits?.constructor?.name !== 'GPUSupportedLimits') {
					const requiredLimits = new global.CanvasModule.GPUSupportedLimits();
					const keys = Object.keys(options.requiredLimits);
					for (const key of keys) {
						requiredLimits[key] = options.requiredLimits[key];
					}
					options.requiredLimits = requiredLimits;
				}
			}

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
