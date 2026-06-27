import { native_ } from './Constants';
import { GPUAdapterInfo } from './GPUAdapterInfo';
import { GPUDevice } from './GPUDevice';
import { GPUDeviceDescriptor } from './Interfaces';
import type { GPUAdapterImpl } from './NativeImpl';
import { GPUFeatureName } from './Types';

export class GPUSupportedFeatures extends Set<GPUFeatureName> {
	get [Symbol.toStringTag]() {
		return 'GPUSupportedFeatures';
	}
}

export class GPUAdapter {
	[native_]: GPUAdapterImpl;

	constructor(adapter?: GPUAdapterImpl) {
		if (adapter) {
			this[native_] = adapter;
		} else {
			throw new TypeError('Failed to construct GPUAdapter: parameter 1 is not of type global.GPUAdapter');
		}
	}

	private _features?: GPUSupportedFeatures;

	get label() {
		return this[native_]?.label ?? '';
	}

	get features() {
		if (!this._features) {
			this._features = new GPUSupportedFeatures(this[native_].features as never);
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

	static fromNative(adapter: GPUAdapterImpl) {
		return new GPUAdapter(adapter);
	}

	requestAdapterInfo(): Promise<GPUAdapterInfo> {
		return new Promise((resolve, reject) => {
			const info = this[native_].requestAdapterInfo();
			resolve(GPUAdapterInfo.fromNative(info) as never);
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
				if (__ANDROID__ || __APPLE__) {
					//@ts-ignore
					const requiredLimits = new global.CanvasModule.GPUSupportedLimits();
					const limits = this[native_].limits;

					const MAX_LIMIT_KEYS = [
						'maxTextureDimension1d',
						'maxTextureDimension2d',
						'maxTextureDimension3d',
						'maxTextureArrayLayers',
						'maxBindGroups',
						'maxBindingsPerBindGroup',
						'maxDynamicUniformBuffersPerPipelineLayout',
						'maxDynamicStorageBuffersPerPipelineLayout',
						'maxSampledTexturesPerShaderStage',
						'maxSamplersPerShaderStage',
						'maxStorageBuffersPerShaderStage',
						'maxStorageTexturesPerShaderStage',
						'maxUniformBuffersPerShaderStage',
						'maxUniformBufferBindingSize',
						'maxStorageBufferBindingSize',
						'maxVertexBuffers',
						'maxBufferSize',
						'maxVertexAttributes',
						'maxVertexBufferArrayStride',
						'maxInterStageShaderVariables',
						'maxColorAttachments',
						'maxColorAttachmentBytesPerSample',
						'maxComputeWorkgroupStorageSize',
						'maxComputeInvocationsPerWorkgroup',
						'maxComputeWorkgroupSizeX',
						'maxComputeWorkgroupSizeY',
						'maxComputeWorkgroupSizeZ',
						'maxComputeWorkgroupsPerDimension',
						'maxNonSamplerBindings',
					];
					const MIN_LIMIT_KEYS = ['minUniformBufferOffsetAlignment', 'minStorageBufferOffsetAlignment'];
					for (const key of MAX_LIMIT_KEYS) {
						const adapterVal = (limits as any)[key];
						if (typeof adapterVal === 'number' && typeof requiredLimits[key] === 'number') {
							if (requiredLimits[key] > adapterVal) {
								requiredLimits[key] = adapterVal;
							}
						}
					}
					for (const key of MIN_LIMIT_KEYS) {
						const adapterVal = (limits as any)[key];
						if (typeof adapterVal === 'number' && typeof requiredLimits[key] === 'number') {
							if (requiredLimits[key] < adapterVal) {
								requiredLimits[key] = adapterVal;
							}
						}
					}

					if (limits.maxSampledTexturesPerShaderStage >= 128) {
						requiredLimits.maxSampledTexturesPerShaderStage = 128;
					}

					if (limits.maxSamplersPerShaderStage >= 128) {
						requiredLimits.maxSamplersPerShaderStage = 128;
					}

					options.requiredLimits = requiredLimits;
				}
			} else {
				if (options.requiredLimits && typeof options.requiredLimits === 'object' && options.requiredLimits?.constructor?.name !== 'GPUSupportedLimits') {
					const keys = Object.keys(options.requiredLimits);
					if (keys.length === 0) {
						delete (options as any).requiredLimits;
					} else {
						//@ts-ignore
						const requiredLimits = new global.CanvasModule.GPUSupportedLimits();
						for (const key of keys) {
							requiredLimits[key] = options.requiredLimits[key];
						}
						const adapterLimits = this[native_].limits;
						for (const key of keys) {
							const adapterVal = (adapterLimits as any)[key];
							if (typeof adapterVal === 'number' && typeof requiredLimits[key] === 'number') {
								if (!key.startsWith('min') && requiredLimits[key] > adapterVal) {
									requiredLimits[key] = adapterVal;
								} else if (key.startsWith('min') && requiredLimits[key] < adapterVal) {
									requiredLimits[key] = adapterVal;
								}
							}
						}
						options.requiredLimits = requiredLimits;
					}
				}
			}

			this[native_].requestDevice(options, (error, device) => {
				if (error) {
					reject(error);
				} else {
					const ret = GPUDevice.fromNative(device, this);
					resolve(ret as never);
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
