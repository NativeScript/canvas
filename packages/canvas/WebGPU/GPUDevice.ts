import { fromObject, Observable } from '@nativescript/core';
import { GPUBuffer } from './GPUBuffer';
import { GPUTexture } from './GPUTexture';
import { adapter_, native_ } from './Constants';
import { GPUShaderModule } from './GPUShaderModule';
import { GPUQueue } from './GPUQueue';
import { GPUPipelineLayout } from './GPUPipelineLayout';
import { parseBindGroupDescriptor, parseComputePipelineDescriptor, parseRenderPipelineDescriptor, parseVertexFormat } from './Utils';
import { GPURenderPipeline } from './GPURenderPipeline';
import { GPUCommandEncoder } from './GPUCommandEncoder';
import { GPUDeviceLostInfo, GPUInternalError, GPUOutOfMemoryError, GPUValidationError } from './Errors';
import { GPUBindGroup } from './GPUBindGroup';
import { GPUBindGroupLayout } from './GPUBindGroupLayout';
import { GPUTextureView } from './GPUTextureView';
import { GPUSampler } from './GPUSampler';
import { GPUExternalTexture } from './GPUExternalTexture';
import type { GPUAddressMode, GPUCompareFunction, GPUErrorFilter, GPUFilterMode, GPUMipmapFilterMode, GPUQueryType, GPUTextureFormat, GPUTextureSampleType, GPUTextureViewDimension } from './Types';
import type { GPUBindGroupDescriptor, GPUBindGroupLayoutEntry, GPUComputePipelineDescriptor, GPUDepthStencilState, GPUExternalTextureBindingLayout, GPUFragmentState, GPUMultisampleState, GPUPrimitiveState, GPUProgrammableStage, GPURenderPipelineDescriptor, GPUVertexState } from './Interfaces';
import { GPUComputePipeline } from './GPUComputePipeline';
import { GPUQuerySet } from './GPUQuerySet';
import { GPURenderBundleEncoder } from './GPURenderBundleEncoder';
import { GPUAdapter, GPUSupportedFeatures } from './GPUAdapter';

function fixup_shader_code(code: string) {
	if (!code.includes('#storage')) {
		if (code.includes('atomic_storage')) code = '#storage atomic_storage array<atomic<i32>>\n\n' + code;
		if (code.includes('float_storage')) code = '#storage float_storage array<vec4<f32>>\n\n' + code;
	}
	code = code.replace(/@stage\(compute\)/g, '@compute');
	code = code.replace(/^type /gm, 'alias ');
	code = code.replace(/^let /gm, 'const ');
	code = code.replace(/alias\s+bool([2-4])\s*=\s*vec\1<\s*bool\s*>\s*;/gm, '');
	code = code.replace(/alias\s+float([2-4])x([2-4])\s*=\s*mat\1x\2<\s*f32\s*>\s*;/gm, '');
	// patch switch issue
	code = code.replace(/case\s+(\d+)\s*:\s*{/g, 'case $1u: {');

	// diagnostic is unsupport atm, remove after https://github.com/gfx-rs/wgpu/pull/6148
	code = code.replace(/diagnostic\s*\([^)]*\)/g, '');
	return code;
}

// Doing so because :D
export class EventTarget {
	protected _emitter?: WeakRef<Observable>;

	addEventListener(event: string, handler: any, options: AddEventListenerOptions = {}) {
		const { capture, once } = options;
		if (capture) {
			//   debug("Bubble propagation is not supported");
		}
		if (once) {
			const oldHandler = handler;
			const self = this;
			handler = (...args: any) => {
				oldHandler.call(null, ...args);
				self.removeEventListener(event, handler);
			};
		}
		let emitter: Observable;

		if (global.isAndroid) {
			emitter = this._emitter?.get?.();
		}

		if (global.isIOS) {
			emitter = this._emitter?.deref?.();
		}
		if (emitter !== null && emitter !== undefined) {
			emitter.addEventListener(event, handler, this);
		}
	}

	removeEventListener(event: string, handler?: any) {
		let emitter: Observable;

		if (global.isAndroid) {
			emitter = this._emitter?.get?.();
		}

		if (global.isIOS) {
			emitter = this._emitter?.deref?.();
		}

		if (emitter !== null && emitter !== undefined) {
			emitter.removeEventListener(event, handler);
		}
	}

	dispatchEvent(event) {
		let emitter: Observable;

		if (global.isAndroid) {
			emitter = this._emitter?.get?.();
		}

		if (global.isIOS) {
			emitter = this._emitter?.deref?.();
		}

		if (emitter !== null && emitter !== undefined) {
			emitter.notify({ ...event, eventName: event.type, object: emitter });
		}
	}
}

interface GPUExtent3DDict {
	width: number;
	height?: number /* default=1 */;
	depthOrArrayLayers?: number /* default=1 */;
}
type GPUExtent3D = [number, number, number] | [number, number] | GPUExtent3DDict;

export class GPUDevice extends EventTarget {
	[native_];
	private _lost: Promise<GPUDeviceLostInfo>;
	private _observerable: Observable;
	constructor() {
		super();
		this._observerable = fromObject({});
		this._emitter = new WeakRef(this._observerable);
	}

	private _uncapturederror(type: number, message: string) {
		let emitter: Observable;
		if (__ANDROID__) {
			emitter = this._emitter?.get();
		} else {
			emitter = this._emitter?.deref();
		}

		let error;

		switch (type) {
			case 1:
				// lost
				// noop
				break;
			case 2:
				// oom
				error = new GPUOutOfMemoryError();
				break;
			case 3:
				// validation
				error = new GPUValidationError(message);
				break;
			case 4:
				// internal
				error = new GPUInternalError();
				break;
		}

		if (emitter) {
			const has = emitter.hasListeners('uncapturederror');
			if (has) {
				emitter.notify({
					eventName: 'uncapturederror',
					object: fromObject({}),
					error,
				});
			} else {
				console.error(error);
			}
		} else {
			console.error(error);
		}
	}

	static fromNative(device, adapter: GPUAdapter) {
		if (device) {
			const ret = new GPUDevice();
			device.setuncapturederror(ret._uncapturederror.bind(this));
			ret[native_] = device;
			ret[adapter_] = adapter;
			ret._lost = new Promise((resolve, reject) => {
				ret[native_].lost
					.then((info) => {
						const ret = new GPUDeviceLostInfo();
						ret[native_] = info;
						resolve(ret);
					})
					.catch((error) => {
						reject(error);
					});
			});
			return ret;
		}
		return null;
	}

	get label() {
		return this[native_]?.label ?? '';
	}

	get lost() {
		return this._lost;
	}

	get native() {
		return this[native_];
	}
	get limits() {
		return this.native.limits;
	}

	get features(): GPUSupportedFeatures {
		return new GPUSupportedFeatures(this.native.features);
	}

	destroy() {
		this.native.destroy();
	}

	createBindGroup(descriptor: GPUBindGroupDescriptor) {
		const desc = parseBindGroupDescriptor(descriptor);

		const group = this.native.createBindGroup(desc);

		if (group) {
			return GPUBindGroup.fromNative(group);
		}
		return undefined;
	}

	createBindGroupLayout(descriptor: GPUBindGroupLayoutEntry) {
		const groupLayout = this.native.createBindGroupLayout(descriptor);
		if (groupLayout) {
			return GPUBindGroupLayout.fromNative(groupLayout);
		}
		return undefined;
	}

	createBuffer(descriptor: { label?: string; mappedAtCreation?: boolean; size: number; usage: number }) {
		const buffer = this.native.createBuffer(descriptor);
		if (buffer) {
			return GPUBuffer.fromNative(buffer, descriptor.mappedAtCreation);
		}
		return undefined;
	}

	createCommandEncoder(descriptor?: { label?: string }) {
		const encoder = this.native.createCommandEncoder(descriptor);
		return GPUCommandEncoder.fromNative(encoder);
	}

	createComputePipeline(descriptor: GPUComputePipelineDescriptor) {
		const desc = parseComputePipelineDescriptor(descriptor);
		return GPUComputePipeline.fromNative(this.native.createComputePipeline(desc));
	}

	createComputePipelineAsync(descriptor: GPUComputePipelineDescriptor) {
		return new Promise((resolve, reject) => {
			const desc = parseComputePipelineDescriptor(descriptor);

			this.native.createComputePipelineAsync(desc, (error, pipeline) => {
				if (error) {
					reject(error.error);
				} else {
					resolve(GPUComputePipeline.fromNative(pipeline));
				}
			});
		});
	}

	createPipelineLayout(descriptor: { bindGroupLayouts: GPUBindGroupLayout[]; label?: string }) {
		const desc: { bindGroupLayouts: GPUBindGroupLayout[]; label?: string } = {
			bindGroupLayouts: descriptor.bindGroupLayouts.map((layout) => {
				return layout[native_];
			}),
		};

		if (typeof descriptor?.label === 'string') {
			desc.label = descriptor.label;
		}
		return GPUPipelineLayout.fromNative(this.native.createPipelineLayout(desc));
	}

	createQuerySet(descriptor: { count: number; label?: string; type: GPUQueryType }) {
		const query = this.native.createQuerySet(descriptor);
		return GPUQuerySet.fromNative(query);
	}

	createRenderBundleEncoder(descriptor: { colorFormats: (null | GPUTextureFormat)[]; depthReadOnly?: boolean; depthStencilFormat?: GPUTextureFormat; label?: string; sampleCount?: number; stencilReadOnly?: boolean }) {
		return GPURenderBundleEncoder.fromNative(this.native.createRenderBundleEncoder(descriptor));
	}

	createRenderPipeline(descriptor: GPURenderPipelineDescriptor) {
		const desc = parseRenderPipelineDescriptor(descriptor, 'createRenderPipeline');

		return GPURenderPipeline.fromNative(this[native_].createRenderPipeline(desc));
	}

	createRenderPipelineAsync(descriptor: GPURenderPipelineDescriptor) {
		return new Promise((resolve, reject) => {
			const desc = parseRenderPipelineDescriptor(descriptor, 'createRenderPipelineAsync');
			this[native_].createRenderPipelineAsync(desc, (error, pipeline) => {
				if (error) {
					reject(error.error);
				} else {
					resolve(GPURenderPipeline.fromNative(pipeline));
				}
			});
		});
	}

	createSampler(descriptor?: { addressModeU?: GPUAddressMode; addressModeV?: GPUAddressMode; addressModeW?: GPUAddressMode; compare?: GPUCompareFunction; label?: string; lodMaxClamp?: number; lodMinClamp?: number; magFilter?: GPUFilterMode; maxAnisotropy?: number; minFilter?: GPUFilterMode; mipmapFilter?: GPUMipmapFilterMode }) {
		return GPUSampler.fromNative(this.native.createSampler(descriptor));
	}

	createShaderModule(descriptor: { label?: string; code: string; sourceMap?: object; compilationHints?: any[] }) {
		const desc: { label?: string; code: string; sourceMap?: object; compilationHints?: any[] } = {
			code: fixup_shader_code(descriptor.code),
		};
		if (typeof descriptor.label === 'string') {
			desc.label = descriptor.label;
		}

		const module = this.native.createShaderModule(desc);
		if (module) {
			return GPUShaderModule.fromNative(module);
		}

		return undefined;
	}

	createTexture(descriptor: { label?: string; size: GPUExtent3D; mipLevelCount?: number /* default=1 */; sampleCount?: number /* default=1 */; dimension?: '1d' | '2d' | '3d' /* default="2d" */; format: GPUTextureFormat; usage: number; viewFormats?: any[] /* default=[] */ }) {
		const sizeIsArray = Array.isArray(descriptor.size);

		const opts = {
			label: descriptor?.label,
			mipLevelCount: descriptor?.mipLevelCount ?? 1,
			sampleCount: descriptor?.sampleCount ?? 1,
			dimension: descriptor?.dimension ?? '2d',
			format: descriptor?.format,
			usage: descriptor?.usage,
			viewFormats: descriptor?.viewFormats ?? [],
			width: sizeIsArray ? descriptor.size[0] : (<GPUExtent3DDict>descriptor.size)?.width,
			height: sizeIsArray ? descriptor.size[1] ?? 1 : (<GPUExtent3DDict>descriptor.size)?.height ?? 1,
			depthOrArrayLayers: sizeIsArray ? descriptor.size[2] ?? 1 : (<GPUExtent3DDict>descriptor.size)?.depthOrArrayLayers ?? 1,
		};

		let hasBrga = false;
		if (__ANDROID__) {
			switch (opts.format) {
				case 'bgra8unorm':
					opts.format = 'rgba8unorm';
					hasBrga = true;
					break;
				case 'bgra8unorm-srgb':
					opts.format = 'rgba8unorm-srgb';
					hasBrga = true;
					break;
			}

			if (hasBrga) {
				console.warn(`GPUDevice:createTexture using unsupported brga format falling back to rgba counterpart.`);
			}
		}

		return GPUTexture.fromNative(this.native.createTexture(opts));
	}

	popErrorScope() {
		return new Promise((resolve, reject) => {
			this.native.popErrorScope((type, message) => {
				switch (type) {
					case 0:
						resolve(null);
						break;
					case 1:
						// should not reach
						// throw internal error
						resolve(new GPUInternalError());
						break;
					case 2:
						resolve(new GPUOutOfMemoryError());
						break;
					case 3:
						resolve(new GPUValidationError(message));
						break;
					case 4:
						resolve(new GPUInternalError());
						break;
				}
			});
		});
	}

	pushErrorScope(filter: GPUErrorFilter) {
		this.native.pushErrorScope(filter);
	}

	private _queue: GPUQueue;
	get queue() {
		if (!this._queue) {
			this._queue = GPUQueue.fromNative(this[native_].queue);
		}
		return this._queue;
	}
}
