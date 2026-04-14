import { Helpers } from '../helpers';
import { adapter_, contextPtr_, GPUTextureUsage, native_ } from './Constants';
import type { GPUDevice } from './GPUDevice';
import { GPUTexture } from './GPUTexture';
import type { GPUAdapter } from './GPUAdapter';
import type { GPUCanvasAlphaMode, GPUCanvasPresentMode, GPUExtent3D, GPUTextureFormat } from './Types';
import type { CanvasRenderingContext } from '../common';
import type { Canvas } from '../Canvas';
const device_ = Symbol('[[device]]');
export class GPUCanvasContext implements CanvasRenderingContext {
	_type;
	_canvas: Canvas | null = null;
	[device_]: GPUDevice | null = null;
	static {
		Helpers.initialize();
	}

	[native_] = null;
	[contextPtr_] = null;

	constructor(context: any, contextOptions: any = {}) {
		let nativeContext = '0';
		if (__ANDROID__) {
			nativeContext = context.getNativeContext().toString();
		}

		if (__IOS__) {
			nativeContext = context.nativeContext.toString();
		}

		const ctxPtr = BigInt(nativeContext);
		//@ts-ignore
		this[native_] = global.CanvasModule.createWebGPUContextWithPointer(ctxPtr);
		this[contextPtr_] = context;
		this._type = 'webgpu';
	}

	get context() {
		return this[native_];
	}

	get contextPtr() {
		return this[contextPtr_];
	}

	get native() {
		return this[native_] as any;
	}

	get canvas() {
		return this._canvas;
	}

	configure(options: { device: GPUDevice; format: GPUTextureFormat; usage?: number; viewFormats?: GPUTextureFormat[]; colorSpace?: 'display-p3' | 'srgb'; alphaMode?: GPUCanvasAlphaMode; presentMode?: GPUCanvasPresentMode; size?: GPUExtent3D }) {
		const opts = {
			usage: GPUTextureUsage.RENDER_ATTACHMENT,
			colorSpace: 'srgb',
			alphaMode: 'opaque',
			presentMode: 'fifo',
			...options,
		};
		if (__ANDROID__ || __IOS__) {
			const adapter = (options as any)?.device?.[adapter_];
			const capabilities = this.getCapabilities(adapter);

			if (!options.presentMode) {
				opts.presentMode = capabilities.presentModes[0];
			}

			if (!options.alphaMode) {
				opts.alphaMode = capabilities.alphaModes[0];
			} else {
				if (!capabilities.alphaModes.includes(options.alphaMode) && (options.alphaMode === 'opaque' || options.alphaMode === 'premultiplied')) {
					if (__IOS__ && options.alphaMode === 'premultiplied') {
						let index = capabilities.alphaModes.indexOf('premultiplied');
						if (index === -1) {
							index = capabilities.alphaModes.indexOf('postmultiplied');
						}
						if (index === -1) {
							index = 0;
						}

						opts.alphaMode = capabilities.alphaModes[index];
					} else {
						opts.alphaMode = capabilities.alphaModes[0];
					}
					console.warn(`GPUCanvasContext: configure alphaMode ${options.alphaMode} unsupported falling back to ${opts.alphaMode}`);
				}
			}

			if (__ANDROID__ && !capabilities.format.includes(options.format) && (options.format === 'bgra8unorm' || options.format === 'bgra8unorm-srgb')) {
				opts.format = capabilities.format[0];
				// fallback to rgba8unorm ... Android 🤪
				if (opts.format === 'rgba8unorm-srgb') {
					opts.format = 'rgba8unorm';
				}
				console.warn(`GPUCanvasContext: configure format ${options.format} unsupported falling back to ${opts.format}`);
			}

			if (__IOS__ && !capabilities.format.includes(options.format)) {
				opts.format = capabilities.format.filter((value) => {
					return value.indexOf('srgb') === -1;
				})[0];
				console.warn(`GPUCanvasContext: configure format ${options.format} unsupported falling back to ${opts.format}`);
			}

			// always force copy_src and copy_dst
			switch (typeof opts.usage) {
				case 'number':
					{
						const has_copy_src = (opts.usage & GPUTextureUsage.COPY_SRC) !== 0;
						const has_copy_dst = (opts.usage & GPUTextureUsage.COPY_DST) !== 0;
						if (!has_copy_dst && !has_copy_src) {
							opts.usage = opts.usage | GPUTextureUsage.COPY_SRC | GPUTextureUsage.COPY_DST;
						} else if (!has_copy_dst) {
							opts.usage = opts.usage | GPUTextureUsage.COPY_DST;
						} else if (!has_copy_src) {
							opts.usage = opts.usage | GPUTextureUsage.COPY_SRC;
						}
					}
					break;
				default:
					opts.usage = GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC | GPUTextureUsage.COPY_DST;

					break;
			}

			if (__IOS__ && opts.usage > capabilities.usages) {
				opts.usage = capabilities.usages;
				console.warn(`GPUCanvasContext: configure usage unsupported falling back to ${capabilities.usages}`);
			}

			if (__IOS__) {
				opts.usage = opts.usage & ~GPUTextureUsage.TEXTURE_BINDING;
			}
		}

		this[device_] = options.device;

		// Build a clean descriptor with only the fields the native layer understands.
		// Third-party renderers (e.g. Three.js r0.170+) pass non-standard fields such as
		// `toneMapping: { mode: 'standard' }` which can cause native serialization errors.
		const nativeOpts: any = {
			device: options?.device?.[native_],
			format: opts.format,
			usage: opts.usage,
			colorSpace: opts.colorSpace,
			alphaMode: opts.alphaMode,
			presentMode: opts.presentMode,
		};
		if (opts.viewFormats) nativeOpts.viewFormats = opts.viewFormats;
		if (opts.size) nativeOpts.size = opts.size;

		this.native.configure(nativeOpts);
	}

	unconfigure() {
		this.native.unconfigure();
	}

	getCurrentTexture() {
		const texture = this.native.getCurrentTexture();
		const result = GPUTexture.fromNative(texture);
		if (!result) {
			console.error('GPUCanvasContext.getCurrentTexture: native returned null/undefined — context may not be configured');
		}
		return result;
	}

	presentSurface() {
		this.native.presentSurface();
	}

	getCapabilities(adapter: GPUAdapter): {
		format: GPUTextureFormat[];
		presentModes: GPUCanvasPresentMode[];
		alphaModes: GPUCanvasAlphaMode;
		usages: number;
	} {
		return this.native.getCapabilities(adapter.native);
	}

	__toDataURL(type: string, quality: number) {
		if (this[device_]) {
			return this.native.__toDataURL(type, quality);
		} else {
			return (<any>this.canvas)._canvas.toDataURL(type, quality);
		}
	}
}
