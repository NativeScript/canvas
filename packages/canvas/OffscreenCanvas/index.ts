// Imports no view, so a Worker can load it.
import type { Canvas } from '../Canvas';
import { CanvasRenderingContext2D } from '../Canvas2D/CanvasRenderingContext2D';
import { ImageBitmap } from '../ImageBitmap';
import { ImageBitmapRenderingContext } from '../ImageBitmapRenderingContext';
import { handleContextOptions, holdBackAnimationFramesWhileBehind, parsePowerPreference } from '../Canvas/utils';
import { Helpers } from '../helpers';
import { WebGLRenderingContext } from '../WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from '../WebGL2/WebGL2RenderingContext';
import { GPUCanvasContext } from '../WebGPU/GPUCanvasContext';

declare const navigator: any;

export type OffscreenRenderingContextId = '2d' | 'bitmaprenderer' | 'webgl' | 'webgl2' | 'webgpu';

const CONTEXT_IDS: OffscreenRenderingContextId[] = ['2d', 'bitmaprenderer', 'webgl', 'webgl2', 'webgpu'];

const ENCODE_TYPES = ['image/png', 'image/jpeg', 'image/webp'];

/** Forwarded from the host canvas, which fires them on GPU device loss. */
const HOST_EVENTS = ['contextlost', 'contextrestored', 'webglcontextlost', 'webglcontextrestored'];

export interface ImageEncodeOptions {
	type?: string;
	quality?: number;
}

function domError(name: string, message: string) {
	const error: any = new Error(message);
	error.name = name;
	return error;
}

/** WebIDL `[EnforceRange] unsigned long long`. */
function enforceRange(value: any, prefix: string): number {
	const number = Number(value);
	if (!Number.isFinite(number)) {
		throw new TypeError(`${prefix}: Value is not a finite number.`);
	}
	const integer = Math.trunc(number);
	if (integer < 0 || integer > Number.MAX_SAFE_INTEGER) {
		throw new TypeError(`${prefix}: Value is outside the 'unsigned long long' value range.`);
	}
	return integer;
}

/** A 2D context's bitmap. The callback runs synchronously for a context source. */
function bitmapFrom2D(native: any): ImageBitmap | null {
	let result = null;
	global.CanvasModule.createImageBitmap(native, (_error, value) => {
		result = value ?? null;
	});
	return ImageBitmap.fromNative(result);
}

function bitmapFromPixels(width: number, height: number, pixels: Uint8Array, premultiplied: boolean): ImageBitmap | null {
	const asset = new global.CanvasModule.ImageAsset();
	if (!asset.fromBytesSync(width, height, pixels, premultiplied)) {
		return null;
	}
	return ImageBitmap.fromNative(global.CanvasModule.ImageBitmap.fromAsset(asset));
}

const FRAMEBUFFER = 0x8d40;
const FRAMEBUFFER_BINDING = 0x8ca6;
const PIXEL_PACK_BUFFER = 0x88eb;
const PIXEL_PACK_BUFFER_BINDING = 0x88ed;
const RGBA = 0x1908;
const UNSIGNED_BYTE = 0x1401;

/** The drawing buffer (not a bound framebuffer), top row first. */
function bitmapFromWebGL(gl: any): ImageBitmap | null {
	const width = gl.drawingBufferWidth;
	const height = gl.drawingBufferHeight;
	if (!width || !height) {
		return null;
	}
	const framebuffer = gl.getParameter(FRAMEBUFFER_BINDING);
	const packBuffer = gl._type === 'webgl2' ? gl.getParameter(PIXEL_PACK_BUFFER_BINDING) : null;
	if (framebuffer) {
		gl.bindFramebuffer(FRAMEBUFFER, null);
	}
	if (packBuffer) {
		gl.bindBuffer(PIXEL_PACK_BUFFER, null);
	}
	const rows = new Uint8Array(width * height * 4);
	gl.native.readPixels(0, 0, width, height, RGBA, UNSIGNED_BYTE, rows);
	if (framebuffer) {
		gl.bindFramebuffer(FRAMEBUFFER, framebuffer);
	}
	if (packBuffer) {
		gl.bindBuffer(PIXEL_PACK_BUFFER, packBuffer);
	}

	const stride = width * 4;
	const pixels = new Uint8Array(rows.length);
	for (let y = 0; y < height; y++) {
		pixels.set(rows.subarray((height - 1 - y) * stride, (height - y) * stride), y * stride);
	}
	const attributes = gl.getContextAttributes?.() ?? {};
	return bitmapFromPixels(width, height, pixels, attributes.alpha !== false && attributes.premultipliedAlpha !== false);
}

function bitmapFromDataURL(url: string): ImageBitmap | null {
	const comma = typeof url === 'string' ? url.indexOf(',') : -1;
	if (comma < 0 || comma === url.length - 1) {
		return null;
	}
	const [, buffer] = Helpers.base64Decode(url.substring(comma + 1));
	const asset = new global.CanvasModule.ImageAsset();
	if (!buffer || !asset.fromEncodedBytesSync(new Uint8Array(buffer))) {
		return null;
	}
	return ImageBitmap.fromNative(global.CanvasModule.ImageBitmap.fromAsset(asset));
}

/** Transparent black, or opaque black without alpha. A clip set outside save() still bounds it. */
function clear2D(ctx: any, width: number, height: number) {
	ctx.save();
	ctx.resetTransform();
	ctx.globalAlpha = 1;
	ctx.globalCompositeOperation = 'copy';
	ctx.filter = 'none';
	ctx.shadowColor = 'rgba(0, 0, 0, 0)';
	if (ctx.getContextAttributes?.().alpha === false) {
		ctx.fillStyle = '#000000';
		ctx.fillRect(0, 0, width, height);
	} else {
		ctx.clearRect(0, 0, width, height);
	}
	ctx.restore();
}

const SCISSOR_TEST = 0x0c11;
const COLOR_CLEAR_VALUE = 0x0c22;
const COLOR_WRITEMASK = 0x0c23;
const DEPTH_CLEAR_VALUE = 0x0b73;
const DEPTH_WRITEMASK = 0x0b72;
const STENCIL_CLEAR_VALUE = 0x0b91;
const STENCIL_WRITEMASK = 0x0b98;
const CLEAR_ALL = 0x4000 | 0x0100 | 0x0400;

/** Clears the drawing buffer as a present would, leaving the app's GL state as it was. */
function clearWebGL(gl: any) {
	const native = gl.native;
	const framebuffer = gl.getParameter(FRAMEBUFFER_BINDING);
	const scissor = native.isEnabled(SCISSOR_TEST);
	const color = native.getParameter(COLOR_CLEAR_VALUE);
	const colorMask = native.getParameter(COLOR_WRITEMASK);
	const depth = native.getParameter(DEPTH_CLEAR_VALUE);
	const depthMask = native.getParameter(DEPTH_WRITEMASK);
	const stencil = native.getParameter(STENCIL_CLEAR_VALUE);
	const stencilMask = native.getParameter(STENCIL_WRITEMASK);

	if (framebuffer) {
		gl.bindFramebuffer(FRAMEBUFFER, null);
	}
	if (scissor) {
		native.disable(SCISSOR_TEST);
	}
	native.colorMask(true, true, true, true);
	native.depthMask(true);
	native.stencilMask(0xffffffff);
	native.clearColor(0, 0, 0, 0);
	native.clearDepth(1);
	native.clearStencil(0);
	native.clear(CLEAR_ALL);

	native.clearColor(color[0], color[1], color[2], color[3]);
	native.colorMask(colorMask[0], colorMask[1], colorMask[2], colorMask[3]);
	native.clearDepth(depth);
	native.depthMask(depthMask);
	native.clearStencil(stencil);
	native.stencilMask(stencilMask);
	if (scissor) {
		native.enable(SCISSOR_TEST);
	}
	if (framebuffer) {
		gl.bindFramebuffer(FRAMEBUFFER, framebuffer);
	}
}

export class OffscreenCanvas {
	/** A canvas shown in a view is always threaded. */
	static threaded2D = true;
	static threadedWebGL = true;

	/** @internal Set by hosts without surfaces. */
	static _legacyHost: (() => Canvas) | null = null;

	private _width: number;
	private _height: number;
	private _surface: any = null;
	private _host: Canvas | null = null;
	private _detached = false;
	private _contextId: OffscreenRenderingContextId | null = null;
	private _context: any = null;
	private _listeners: Map<string, { listener: any; once: boolean }[]> | null = null;

	oncontextlost: ((event: any) => any) | null = null;
	oncontextrestored: ((event: any) => any) | null = null;

	constructor(width: number, height: number) {
		if (arguments.length < 2) {
			throw new TypeError(`Failed to construct 'OffscreenCanvas': 2 arguments required, but only ${arguments.length} present.`);
		}
		this._width = enforceRange(width, "Failed to construct 'OffscreenCanvas'");
		this._height = enforceRange(height, "Failed to construct 'OffscreenCanvas'");
	}

	/** @internal */
	static _fromPlaceholder(canvas: Canvas): OffscreenCanvas {
		const surface = surfaces() ? (canvas as any)._createOffscreenSurface?.() : null;
		if (surface) {
			return OffscreenCanvas._fromSurface(surface);
		}
		const offscreen = new OffscreenCanvas(canvas.width, canvas.height);
		offscreen._attachHost(canvas);
		return offscreen;
	}

	/** @internal */
	static _fromSurface(surface: any): OffscreenCanvas {
		const offscreen = new OffscreenCanvas(surface.width, surface.height);
		offscreen._surface = surface;
		return offscreen;
	}

	/** @internal Detaches `offscreen`, as a transfer does. */
	static _toHandle(offscreen: OffscreenCanvas): number {
		if (!(offscreen instanceof OffscreenCanvas)) {
			throw new TypeError("Failed to transfer: the value is not an 'OffscreenCanvas'.");
		}
		if (offscreen._detached) {
			throw domError('DataCloneError', 'An OffscreenCanvas could not be transferred because it was detached.');
		}
		if (offscreen._context) {
			throw domError('InvalidStateError', 'An OffscreenCanvas could not be transferred because it had a rendering context.');
		}
		const surface = offscreen._ensureSurface();
		if (!surface) {
			throw domError('NotSupportedError', 'An OffscreenCanvas can not be transferred on this platform yet.');
		}
		const handle = surface.toHandle();
		if (!handle) {
			throw domError('DataCloneError', 'An OffscreenCanvas could not be transferred.');
		}
		surface.dispose();
		offscreen._surface = null;
		offscreen._detached = true;
		return handle;
	}

	/** @internal */
	static _fromHandle(handle: number): OffscreenCanvas {
		const surface = surfaces()?.adopt(handle) ?? null;
		if (!surface) {
			throw domError('DataCloneError', 'The OffscreenCanvas was already received, or released.');
		}
		return OffscreenCanvas._fromSurface(surface);
	}

	/** @internal */
	static _releaseHandle(handle: number): boolean {
		return !!surfaces()?.releaseHandle(handle);
	}

	get [Symbol.toStringTag]() {
		return 'OffscreenCanvas';
	}

	get width(): number {
		return this._detached ? 0 : this._width;
	}

	set width(value: number) {
		this._assertNotDetached('width');
		this._width = enforceRange(value, "Failed to set the 'width' property on 'OffscreenCanvas'");
		this._resize();
	}

	get height(): number {
		return this._detached ? 0 : this._height;
	}

	set height(value: number) {
		this._assertNotDetached('height');
		this._height = enforceRange(value, "Failed to set the 'height' property on 'OffscreenCanvas'");
		this._resize();
	}

	/** The context's native object, which drawImage, createImageBitmap, texImage2D and copyExternalImageToTexture read. */
	get native() {
		return this._context?.native ?? null;
	}

	/** @internal */
	get _canvasHost(): Canvas | null {
		return this._host;
	}

	getContext(contextId: '2d', options?: any): CanvasRenderingContext2D | null;
	getContext(contextId: 'bitmaprenderer', options?: { alpha?: boolean }): ImageBitmapRenderingContext | null;
	getContext(contextId: 'webgl' | 'webgl2' | 'webgpu', options?: any): any;
	getContext(contextId: OffscreenRenderingContextId, options?: any): any {
		const id = String(contextId) as OffscreenRenderingContextId;
		if (CONTEXT_IDS.indexOf(id) === -1) {
			throw new TypeError(`Failed to execute 'getContext' on 'OffscreenCanvas': The provided value '${id}' is not a valid enum value of type OffscreenRenderingContextType.`);
		}
		if (this._detached) {
			throw domError('InvalidStateError', "Failed to execute 'getContext' on 'OffscreenCanvas': OffscreenCanvas object is detached.");
		}
		if (this._context) {
			return this._contextId === id ? this._context : null;
		}

		const surface = this._ensureSurface();
		const context = surface ? this._surfaceContext(surface, id, options) : this._hostContext(id, options);
		if (context) {
			this._context = context;
			this._contextId = id;
		}
		return context;
	}

	transferToImageBitmap(): ImageBitmap {
		if (this._detached) {
			throw domError('InvalidStateError', "Failed to execute 'transferToImageBitmap' on 'OffscreenCanvas': Cannot transfer an ImageBitmap from a detached OffscreenCanvas");
		}
		const context = this._context;
		if (!context) {
			throw domError('InvalidStateError', "Failed to execute 'transferToImageBitmap' on 'OffscreenCanvas': Cannot transfer an ImageBitmap from an OffscreenCanvas with no context");
		}
		let bitmap: ImageBitmap | null = null;
		switch (this._contextId) {
			case '2d':
				bitmap = bitmapFrom2D(context.native);
				clear2D(context, this._width, this._height);
				break;
			case 'bitmaprenderer':
				bitmap = bitmapFrom2D(context.native);
				context.transferFromImageBitmap(null);
				break;
			case 'webgl':
			case 'webgl2':
				bitmap = bitmapFromWebGL(context);
				clearWebGL(context);
				break;
			case 'webgpu':
				bitmap = bitmapFromDataURL(context.__toDataURL('image/png', 1));
				// The next getCurrentTexture() starts a new frame.
				try {
					if (context.native?.hasCurrentTexture !== false) {
						context.presentSurface();
					}
				} catch (e) {}
				break;
		}
		if (!bitmap) {
			throw domError('UnknownError', "Failed to execute 'transferToImageBitmap' on 'OffscreenCanvas': The canvas bitmap could not be read.");
		}
		return bitmap;
	}

	convertToBlob(options?: ImageEncodeOptions): Promise<Blob> {
		return new Promise((resolve, reject) => {
			if (this._detached) {
				reject(domError('InvalidStateError', "Failed to execute 'convertToBlob' on 'OffscreenCanvas': OffscreenCanvas object is detached."));
				return;
			}
			if (this._width === 0 || this._height === 0) {
				reject(domError('IndexSizeError', "Failed to execute 'convertToBlob' on 'OffscreenCanvas': The size of the OffscreenCanvas is zero."));
				return;
			}
			const requested = typeof options?.type === 'string' ? options.type.toLowerCase() : 'image/png';
			const type = ENCODE_TYPES.indexOf(requested) === -1 ? 'image/png' : requested;
			const quality = typeof options?.quality === 'number' && options.quality >= 0 && options.quality <= 1 ? options.quality : 0.92;

			let url: string;
			try {
				url = this._toDataURL(type, quality);
			} catch (e) {
				reject(e);
				return;
			}
			const comma = typeof url === 'string' ? url.indexOf(',') : -1;
			if (comma < 0 || comma === url.length - 1) {
				reject(domError('EncodingError', "Failed to execute 'convertToBlob' on 'OffscreenCanvas': Encoding the image failed."));
				return;
			}
			// The encoder may have fallen back to another format.
			const mime = /^data:([^;,]+)/.exec(url)?.[1] ?? type;
			Helpers.base64DecodeAsync(url.substring(comma + 1))
				.then((decoded) => {
					const buffer = decoded?.[1];
					if (!buffer) {
						reject(domError('EncodingError', "Failed to execute 'convertToBlob' on 'OffscreenCanvas': Encoding the image failed."));
						return;
					}
					resolve(new Blob([buffer], { type: mime }));
				})
				.catch(reject);
		});
	}

	addEventListener(type: string, listener: any, options?: boolean | { once?: boolean }) {
		if (!listener) {
			return;
		}
		this._listeners ??= new Map();
		const entries = this._listeners.get(type) ?? [];
		if (entries.some((entry) => entry.listener === listener)) {
			return;
		}
		entries.push({ listener, once: typeof options === 'object' && !!options?.once });
		this._listeners.set(type, entries);
	}

	removeEventListener(type: string, listener: any) {
		const entries = this._listeners?.get(type);
		if (!entries) {
			return;
		}
		const index = entries.findIndex((entry) => entry.listener === listener);
		if (index !== -1) {
			entries.splice(index, 1);
		}
	}

	dispatchEvent(event: any): boolean {
		const type = event?.type;
		if (typeof type !== 'string') {
			throw new TypeError("Failed to execute 'dispatchEvent' on 'EventTarget': parameter 1 is not of type 'Event'.");
		}
		const call = (listener: any) => {
			try {
				if (typeof listener === 'function') {
					listener.call(this, event);
				} else {
					listener?.handleEvent?.(event);
				}
			} catch (e) {
				console.error(e);
			}
		};
		const handler = (this as any)[`on${type}`];
		if (typeof handler === 'function') {
			call(handler);
		}
		const entries = this._listeners?.get(type);
		if (entries) {
			for (const entry of entries.slice()) {
				if (entry.once) {
					this.removeEventListener(type, entry.listener);
				}
				call(entry.listener);
			}
		}
		return !event.defaultPrevented;
	}

	private _assertNotDetached(name: 'width' | 'height') {
		if (this._detached) {
			throw domError('InvalidStateError', `Failed to set the '${name}' property on 'OffscreenCanvas': OffscreenCanvas object is detached.`);
		}
	}

	private _ensureSurface(): any {
		if (!this._surface && !this._host) {
			this._surface = surfaces()?.create(this._width, this._height, 1, 160, 0, 0) ?? null;
		}
		return this._surface;
	}

	private _surfaceContext(surface: any, id: OffscreenRenderingContextId, options?: any): any {
		const shown = surface.hasView;
		let context: any = null;
		switch (id) {
			case '2d':
				context = this._surface2D(surface, '2d', options, shown);
				break;
			case 'bitmaprenderer': {
				const backing = this._surface2D(surface, 'bitmaprenderer', options, shown);
				if (backing) {
					context = new ImageBitmapRenderingContext(this as any, backing, handleContextOptions('bitmaprenderer', options));
				}
				return context;
			}
			case 'webgl':
			case 'webgl2': {
				const opts = handleContextOptions(id, options);
				const threaded = shown || !!(options?.threaded ?? OffscreenCanvas.threadedWebGL);
				const powerPreference = typeof opts.powerPreference === 'number' ? opts.powerPreference : parsePowerPreference(opts.powerPreference);
				const native = surface.createWebGL(id === 'webgl2' ? 2 : 1, opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, powerPreference < 0 ? 0 : powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible, threaded);
				if (!native) {
					return null;
				}
				if (threaded) {
					holdBackAnimationFramesWhileBehind();
				}
				context = id === 'webgl2' ? new (WebGL2RenderingContext as any)(null, null, native) : new (WebGLRenderingContext as any)(native);
				context._type = id;
				break;
			}
			case 'webgpu': {
				const gpu = typeof navigator !== 'undefined' ? navigator?.gpu?.native : null;
				const native = gpu ? surface.createWebGPU(gpu) : null;
				if (!native) {
					return null;
				}
				context = new (GPUCanvasContext as any)(null, {}, native);
				break;
			}
		}
		if (context) {
			context._canvas = this;
			Object.defineProperty(context, 'canvas', { value: this, configurable: true });
		}
		return context;
	}

	private _surface2D(surface: any, type: '2d' | 'bitmaprenderer', options: any, shown: boolean): CanvasRenderingContext2D | null {
		const opts = handleContextOptions(type, options);
		const threaded = shown || !!(options?.threaded ?? OffscreenCanvas.threaded2D);
		const native = surface.create2D(opts.alpha, -16777216, threaded);
		if (!native) {
			return null;
		}
		if (threaded) {
			holdBackAnimationFramesWhileBehind();
		}
		const context = new (CanvasRenderingContext2D as any)(null, opts, native);
		context._type = '2d';
		context.__threaded = threaded;
		context._canvas = this;
		return context;
	}

	private _hostContext(id: OffscreenRenderingContextId, options?: any): any {
		const host = this._ensureHost();
		if (!host) {
			return null;
		}
		let context: any = null;
		if (id === 'bitmaprenderer') {
			const backing = host._getContext('2d', options);
			if (backing) {
				context = new ImageBitmapRenderingContext(this as any, backing, handleContextOptions('bitmaprenderer', options));
			}
		} else {
			context = host._getContext(id, options);
			if (context) {
				Object.defineProperty(context, 'canvas', { value: this, configurable: true });
			}
		}
		return context;
	}

	private _ensureHost(): Canvas | null {
		if (!this._host) {
			const host = OffscreenCanvas._legacyHost?.() ?? null;
			if (!host) {
				return null;
			}
			host._resizeBitmap(this._width, this._height);
			this._attachHost(host);
		}
		return this._host;
	}

	private _attachHost(host: Canvas) {
		this._host = host;
		const ref = new WeakRef(this);
		for (const type of HOST_EVENTS) {
			host.on(type, (args: any) => ref.deref()?._forwardHostEvent(type, args));
		}
	}

	private _forwardHostEvent(type: string, args: any) {
		let defaultPrevented = false;
		const event = {
			type,
			target: this,
			currentTarget: this,
			cancelable: true,
			get defaultPrevented() {
				return defaultPrevented;
			},
			preventDefault() {
				defaultPrevented = true;
				args?.preventDefault?.();
			},
			stopPropagation() {},
		};
		this.dispatchEvent(event);
	}

	/** Per spec a 2D context resets even when the size is unchanged. */
	private _resize() {
		if (this._surface) {
			const backing = this._contextId === 'bitmaprenderer' ? (this._context as any)?._context : this._contextId === '2d' ? this._context : null;
			backing?.native?.__resize(this._width, this._height);
			this._surface.resize(this._width, this._height);
		} else if (this._host) {
			this._host._resizeBitmap(this._width, this._height);
		} else {
			return;
		}
		if (this._contextId === '2d') {
			this._context.reset();
		}
	}

	private _toDataURL(type: string, quality: number): string {
		if (this._contextId === 'webgpu') {
			return this._context.__toDataURL(type, quality);
		}
		const native = this.native;
		if (native) {
			return native.__toDataURL(type, quality);
		}
		// No context: the bitmap is transparent black.
		const blank = global.CanvasModule.CanvasRenderingContext2D.withCpu(this._width || 1, this._height || 1, 1, true, 0, 160, 0);
		return blank?.__toDataURL(type, quality) ?? 'data:,';
	}
}

function surfaces(): any {
	Helpers.initialize();
	return global.CanvasModule?.OffscreenSurface ?? null;
}

/** Only a 2D context whose canvas is an OffscreenCanvas matches; there is no separate class. */
export class OffscreenCanvasRenderingContext2D {
	private constructor() {
		throw new TypeError('Illegal constructor');
	}

	static [Symbol.hasInstance](value: any) {
		return value instanceof CanvasRenderingContext2D && value.canvas instanceof OffscreenCanvas;
	}
}
