import { native_ } from './Constants';
import { GPUAdapterInfo } from './GPUAdapterInfo';
import { GPUDevice } from './GPUDevice';
import { GPUDeviceDescriptor } from './Interfaces';
import type { GPUAdapterImpl } from './NativeImpl';
import { GPUFeatureName } from './Types';
import { NAPI_HOST } from '../platform';

export class GPUSupportedFeatures extends Set<GPUFeatureName> {
	get [Symbol.toStringTag]() {
		return 'GPUSupportedFeatures';
	}
}

/** Lowered to the adapter's value when over it */
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
/** Raised to the adapter's value when under it */
const MIN_LIMIT_KEYS = ['minUniformBufferOffsetAlignment', 'minStorageBufferOffsetAlignment'];

/** Keeps `required` within what `adapter` supports */
function clampToAdapter(required: Record<string, number>, adapter: Record<string, any>): void {
	for (const key of MAX_LIMIT_KEYS) {
		const have = adapter[key];

		if (typeof have === 'number' && typeof required[key] === 'number' && required[key] > have) required[key] = have;
	}
	for (const key of MIN_LIMIT_KEYS) {
		const have = adapter[key];

		if (typeof have === 'number' && typeof required[key] === 'number' && required[key] < have) required[key] = have;
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
				if (__ANDROID__ || __APPLE__ || NAPI_HOST) {
					//@ts-ignore
					const requiredLimits = new global.CanvasModule.GPUSupportedLimits();
					const limits = this[native_].limits;

					clampToAdapter(requiredLimits, limits);

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
						// Clamp every limit: unnamed ones default to WebGPU's, which some adapters can't meet.
						clampToAdapter(requiredLimits, this[native_].limits);
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
