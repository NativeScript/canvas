import { Helpers } from '../helpers';
import { adapter_, contextPtr_, GPUTextureUsage, native_, swapchainContext_ } from './Constants';
import type { GPUDevice } from './GPUDevice';
import { GPUTexture } from './GPUTexture';
import type { GPUTextureView } from './GPUTextureView';
import type { GPUAdapter } from './GPUAdapter';
import type { GPUCanvasAlphaMode, GPUCanvasPresentMode, GPUExtent3D, GPUTextureFormat } from './Types';
import type { CanvasRenderingContext } from '../common';
import type { Canvas } from '../Canvas';
import { NAPI_HOST, POINTER_CONTEXT_HOST } from '../platform';
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

	// per-frame swapchain views and textures, released at the next presentSurface()
	private _swapchainViews: GPUTextureView[] = [];
	private _swapchainTextures: GPUTexture[] = [];

	// The native context is borrowed from the Canvas view, which frees it in disposeNativeView;
	// after that every native call here is a use-after-free. On Android a destroyed SurfaceView
	// surface leaves the swapchain unusable until the view re-creates it, and wgpu's
	// getCurrentTexture blocks the calling (UI) thread on such a surface instead of failing.
	// The native wrapper also presents the current texture on every vsync from its own RAF
	// through the same raw pointer, so that RAF has to stop before either happens.
	private _detached = false;
	private _surfaceLost = false;
	private _rafPaused = false;
	private _warnedDetached = false;
	private _warnedSurfaceLost = false;

	/** @internal */
	_registerSwapchainView(view: GPUTextureView) {
		this._swapchainViews.push(view);
	}

	constructor(context: any, contextOptions: any = {}) {
		let nativeContext = '0';
		if (__ANDROID__) {
			nativeContext = context.getNativeContext().toString();
		}

		if (POINTER_CONTEXT_HOST) {
			nativeContext = context.nativeContext.toString();
		}

		const ctxPtr = BigInt(nativeContext);
		//@ts-ignore
		this[native_] = global.CanvasModule.createWebGPUContextWithPointer(ctxPtr);
		this[contextPtr_] = context;
		this._canvas = context;
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
		if (!this._live('configure')) {
			return;
		}
		const opts = {
			usage: GPUTextureUsage.RENDER_ATTACHMENT,
			colorSpace: 'srgb',
			alphaMode: 'opaque',
			presentMode: 'fifo',
			...options,
		};
		if (__ANDROID__ || __APPLE__ || NAPI_HOST) {
			const adapter = (options as any)?.device?.[adapter_];
			const capabilities = this.getCapabilities(adapter);

			// Native answers with empty lists when the surface cannot be queried (destroyed, or not
			// created yet); configuring it anyway leaves a swapchain that hangs on acquire.
			if (!capabilities?.format?.length) {
				console.warn('GPUCanvasContext: configure skipped — the surface reports no capabilities (destroyed or not yet created)');
				return;
			}

			if (!options.presentMode) {
				opts.presentMode = capabilities.presentModes[0];
			}

			if (!options.alphaMode) {
				opts.alphaMode = capabilities.alphaModes[0];
			} else {
				if (!capabilities.alphaModes.includes(options.alphaMode) && (options.alphaMode === 'opaque' || options.alphaMode === 'premultiplied')) {
					if (__APPLE__ && options.alphaMode === 'premultiplied') {
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

			if (__APPLE__ && !capabilities.format.includes(options.format)) {
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

			if (__APPLE__) {
				opts.usage = opts.usage | GPUTextureUsage.RENDER_ATTACHMENT;
			}

			if (__APPLE__ && opts.usage > capabilities.usages) {
				opts.usage = capabilities.usages;
				console.warn(`GPUCanvasContext: configure usage unsupported falling back to ${capabilities.usages}`);
			}

			if (__APPLE__) {
				const supported = (capabilities && (capabilities as any).usages) || 0;
				const unsupported = opts.usage & ~supported;
				if (unsupported !== 0) {
					console.warn(`GPUCanvasContext: configure requested unsupported usage bits (0x${unsupported.toString(16)}), masking to supported usages=0x${supported.toString(16)}`);
					opts.usage = opts.usage & supported;
				}
			}
		}

		this[device_] = options.device;

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
		this._surfaceLost = false;
		this._warnedSurfaceLost = false;
		this._resumeNativeRaf();
	}

	unconfigure() {
		if (!this._live('unconfigure')) {
			return;
		}
		this.native.unconfigure();
	}

	getCurrentTexture() {
		if (!this._live('getCurrentTexture')) {
			this._releaseSwapchainWrappers();
			return null;
		}
		if (this._surfaceLost) {
			if (!this._warnedSurfaceLost) {
				this._warnedSurfaceLost = true;
				console.warn('GPUCanvasContext.getCurrentTexture: the surface was destroyed; returning null until it is re-created');
			}
			this._releaseSwapchainWrappers();
			return null;
		}
		// A host that presents at frame end (no presentSurface() call) leaves the last frame's
		// wrappers here; no current texture means that frame was presented.
		if (this.native.hasCurrentTexture === false) {
			this._releaseSwapchainWrappers();
		}
		const current = this.native.getCurrentTexture();
		if (!current) {
			console.error('GPUCanvasContext.getCurrentTexture: native returned empty — context may not be configured');
			return null;
		}

		const texture = (current as any).texture ?? current;
		const result = GPUTexture.fromNative(texture);
		if (!result) {
			console.error('GPUCanvasContext.getCurrentTexture: native texture wrapper contained no texture');
		} else {
			// mark as swapchain-owned and track for release at present
			(result as any)[swapchainContext_] = this;
			this._swapchainTextures.push(result);
		}
		return result;
	}

	presentSurface(_texture?: GPUTexture) {
		if (this._live('presentSurface') && !this._surfaceLost) {
			this.native.presentSurface();
		}
		this._releaseSwapchainWrappers();
	}

	// release this frame's swapchain views and textures (their point of death)
	private _releaseSwapchainWrappers() {
		const views = this._swapchainViews;
		if (views.length > 0) {
			this._swapchainViews = [];
			for (let i = 0; i < views.length; i++) {
				const view = views[i] as any;
				view?.[native_]?.destroy?.();
				if (view) {
					view[native_] = null;
				}
			}
		}
		const texs = this._swapchainTextures;
		if (texs.length > 0) {
			this._swapchainTextures = [];
			for (let i = 0; i < texs.length; i++) {
				const tex = texs[i] as any;
				tex?.[native_]?.__releaseHandle?.();
				if (tex) {
					tex[native_] = null;
				}
			}
		}
	}

	getCapabilities(adapter: GPUAdapter): {
		format: GPUTextureFormat[];
		presentModes: GPUCanvasPresentMode[];
		alphaModes: GPUCanvasAlphaMode[];
		usages: number;
	} {
		if (!this._live('getCapabilities')) {
			return { format: [], presentModes: [], alphaModes: [], usages: 0 };
		}
		return this.native.getCapabilities(adapter.native);
	}

	__toDataURL(type: string, quality: number) {
		if (!this._live('__toDataURL')) {
			return 'data:,';
		}
		if (this[device_]) {
			return this.native.__toDataURL(type, quality);
		} else {
			return (<any>this.canvas)._canvas.toDataURL(type, quality);
		}
	}

	/**
	 * @internal The Canvas view calls this right before it releases the native context. The native
	 * wrapper object is kept: its finalizer drops the context's refcount, so letting it be collected
	 * early would double-release against the view's own release.
	 */
	__detach() {
		if (this._detached) {
			return;
		}
		this._detached = true;
		this._stopNativeRaf();
		this._releaseSwapchainWrappers();
		this[device_] = null;
	}

	/** @internal Android: the SurfaceView surface is gone; nothing may touch the swapchain. */
	__surfaceLost() {
		this._surfaceLost = true;
		this._warnedSurfaceLost = false;
		this._stopNativeRaf();
		this._releaseSwapchainWrappers();
	}

	/**
	 * @internal Android: a surface exists again. The view re-attaches the swapchain natively on
	 * resize, so rendering resumes as soon as the surface answers a capabilities query; a surface
	 * that reports nothing stays paused until a later resize or a successful configure().
	 */
	__surfaceRestored() {
		if (!this._surfaceLost || this._detached || !this[native_]) {
			return;
		}
		const adapter = (this[device_] as any)?.[adapter_];
		if (!adapter) {
			this._surfaceLost = false;
			return;
		}
		let capabilities: any;
		try {
			capabilities = this.native.getCapabilities(adapter.native);
		} catch {}
		if (capabilities?.format?.length) {
			this._surfaceLost = false;
			this._warnedSurfaceLost = false;
			this._resumeNativeRaf();
		}
	}

	private _stopNativeRaf() {
		const native = this[native_];
		if (!native || typeof native.__stopRaf !== 'function') {
			return;
		}
		try {
			native.__stopRaf();
			this._rafPaused = true;
		} catch {}
	}

	private _resumeNativeRaf() {
		if (!this._rafPaused) {
			return;
		}
		this._rafPaused = false;
		const native = this[native_];
		if (!native || typeof native.__startRaf !== 'function' || native.continuousRenderMode === false) {
			return;
		}
		try {
			native.__startRaf();
		} catch {}
	}

	private _live(method: string): boolean {
		if (this._detached || !this[native_]) {
			if (!this._warnedDetached) {
				this._warnedDetached = true;
				console.warn(`GPUCanvasContext.${method}: the canvas released its native context; call ignored`);
			}
			return false;
		}
		return true;
	}
}
