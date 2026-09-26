/**
 * The canvas view on Node-API hosts (NativeScript Windows first; macOS/Linux later), shared by
 * every platform whose native module is crates/canvas-napi.
 *
 * The native side is `CanvasModule.NSCCanvas`: it owns the rendering context and the swapchain,
 * as the iOS `NSCCanvas` view does, and hands out the context pointer. A platform subclass only
 * creates the native view, constructs the host for it and forwards the view's layout, scale and
 * pointer events here.
 */
import { CanvasBase, DOMRect, doc, lengthToDevicePixels } from './common';
import { DOMMatrix } from '../Canvas2D';
import { CanvasRenderingContext2D } from '../Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from '../WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from '../WebGL2/WebGL2RenderingContext';
import { GPUCanvasContext } from '../WebGPU';
import { ImageBitmapRenderingContext } from '../ImageBitmapRenderingContext';
import { ImageSource, Screen, Utils, widthProperty, heightProperty } from '@nativescript/core';
import { handleContextOptions, microtask, CanvasContextType } from './utils';
import { Helpers } from '../helpers';

export function createSVGMatrix(): DOMMatrix {
	return new DOMMatrix();
}

const defaultOpts = {
	alpha: true,
	antialias: true,
	depth: true,
	failIfMajorPerformanceCaveat: false,
	powerPreference: 'default',
	premultipliedAlpha: true,
	preserveDrawingBuffer: false,
	stencil: false,
	desynchronized: false,
	xrCompatible: false,
	willReadFrequently: false,
};

enum ContextType {
	None,
	Canvas,
	BitmapRenderer,
	WebGL,
	WebGL2,
	WebGPU,
}

/** `CanvasFit` as the native hosts take it. */
export const enum CanvasFit {
	None = 0,
	Fill = 1,
	FitX = 2,
	FitY = 3,
	ScaleDown = 4,
}

function isPercentLength(value: any) {
	return (typeof value === 'object' && value?.unit === '%') || (typeof value === 'string' && value.trim().endsWith('%'));
}

function isFixedLength(value: any) {
	return value?.unit === 'px' || value?.unit === 'dip';
}

/**
 * The fit mode for a canvas's CSS size, as the iOS view picks it: both lengths set stretch the
 * buffer (or scale it down when fixed lengths are larger than it); one `auto` side keeps the
 * aspect ratio; `auto` on both shows the buffer at its natural size.
 */
export function fitForStyle(styleWidth: any, styleHeight: any, surfaceDips: { width: number; height: number }, viewDips: { width: number; height: number }): CanvasFit {
	if (typeof styleWidth === 'object' && typeof styleHeight === 'object') {
		if (isFixedLength(styleWidth) && isFixedLength(styleHeight)) {
			return viewDips.width > surfaceDips.width || viewDips.height > surfaceDips.height ? CanvasFit.ScaleDown : CanvasFit.Fill;
		}
		return CanvasFit.Fill;
	}
	if (typeof styleWidth === 'object' && styleHeight === 'auto') {
		return isFixedLength(styleWidth) || styleWidth?.unit === '%' ? CanvasFit.FitX : CanvasFit.Fill;
	}
	if (styleWidth === 'auto' && typeof styleHeight === 'object') {
		return isFixedLength(styleHeight) || styleHeight?.unit === '%' ? CanvasFit.FitY : CanvasFit.Fill;
	}
	if (styleWidth === 'auto' && styleHeight === 'auto') {
		return CanvasFit.None;
	}
	return CanvasFit.Fill;
}

export abstract class NapiCanvas extends CanvasBase {
	private _2dContext: CanvasRenderingContext2D | null = null;
	private _webglContext: WebGLRenderingContext | null = null;
	private _webgl2Context: WebGL2RenderingContext | null = null;
	private _gpuContext: GPUCanvasContext | null = null;
	private _bitmapRendererContext: ImageBitmapRenderingContext | null = null;
	private _contextType = ContextType.None;
	private _didPause = false;
	private _isReady = false;
	private _pendingWidth: number | undefined;
	private _pendingHeight: number | undefined;
	private _viewWidth = 0;
	private _viewHeight = 0;
	private _viewScaleX = 0;
	private _viewScaleY = 0;
	/** The last size given on each axis was a `%` length, so the buffer follows the view. */
	private _followWidth = false;
	private _followHeight = false;

	/** `CanvasModule.NSCCanvas`. */
	protected _canvas: any;

	static useSurface = false;
	static forceGL = false;
	surfaceOnTop = false;

	protected constructor() {
		super();
		Helpers.initialize();
		(global as any).__canvasLoaded = true;
	}

	/** A canvas not attached to a layout (offscreen drawing, tests), as on iOS/Android. */
	static createCustomView() {
		const canvas = new (this as any)();
		canvas._isCustom = true;
		return canvas;
	}

	/** Called by the platform once its native view exists: `host` is its `CanvasModule.NSCCanvas`. */
	protected _attachHost(host: any) {
		this._canvas = host;
	}

	get lang() {
		return 'en';
	}

	set lang(value: string) {
		// todo
	}

	onLoaded(): void {
		super.onLoaded();
		if (this.__native__context && this._didPause) {
			this.__native__context.__startRaf?.();
			this._didPause = false;
		}
	}

	onUnloaded(): void {
		super.onUnloaded();
		if (!this.__native__context) {
			return;
		}
		this._didPause = true;
		this.__native__context.__stopRaf?.();
	}

	// Hosts whose layout is native (Windows) never measure the view: use the size it reports.
	get clientWidth() {
		const width = this.getMeasuredWidth();
		return width === 0 ? this._viewWidth : width / Screen.mainScreen.scale;
	}

	get clientHeight() {
		const height = this.getMeasuredHeight();
		return height === 0 ? this._viewHeight : height / Screen.mainScreen.scale;
	}

	get drawingBufferWidth() {
		return this._canvas?.drawingBufferWidth ?? 0;
	}

	get drawingBufferHeight() {
		return this._canvas?.drawingBufferHeight ?? 0;
	}

	[widthProperty.setNative](value: any) {
		this.__setSurfaceWidth(value);
	}

	[heightProperty.setNative](value: any) {
		this.__setSurfaceHeight(value);
	}

	// @ts-ignore
	set width(value: any) {
		this.__setSurfaceWidth(value);
		this.__resetAfterResize();
	}

	// @ts-ignore
	get width(): number {
		if (this._pendingWidth !== undefined) {
			return this._pendingWidth;
		}
		return this._canvas?.surfaceWidth ?? 0;
	}

	// @ts-ignore
	set height(value: any) {
		this.__setSurfaceHeight(value);
		this.__resetAfterResize();
	}

	// @ts-ignore
	get height(): number {
		if (this._pendingHeight !== undefined) {
			return this._pendingHeight;
		}
		return this._canvas?.surfaceHeight ?? 0;
	}

	/** A CSS width must not touch the bitmap, so only these setters reset. */
	private __resetAfterResize() {
		try {
			(this._2dContext as any)?.reset?.();
		} catch (e) {}
	}

	private __toPixels(value: any, isWidth: boolean) {
		return typeof value === 'number' ? Math.floor(value) : lengthToDevicePixels(value, this.parent, isWidth);
	}

	// Width and height set in the same turn resize the buffer once.
	private __setSurfaceWidth(value: any) {
		const px = this.__toPixels(value, true);
		if (Number.isNaN(px)) {
			// e.g. 'auto': no size, so nothing changes.
			return;
		}
		this._followWidth = isPercentLength(value);
		if (!this._canvas) {
			return;
		}
		const width = Math.floor(px);
		if (this._pendingHeight !== undefined) {
			const height = this._pendingHeight;
			this._pendingHeight = undefined;
			this._canvas.setSurfaceSize(width, height);
			this._syncFit();
			return;
		}
		this._pendingWidth = width;
		microtask(() => {
			if (this._pendingWidth !== undefined) {
				this._canvas.surfaceWidth = this._pendingWidth;
				this._pendingWidth = undefined;
				this._syncFit();
			}
		});
	}

	private __setSurfaceHeight(value: any) {
		const px = this.__toPixels(value, false);
		if (Number.isNaN(px)) {
			return;
		}
		this._followHeight = isPercentLength(value);
		if (!this._canvas) {
			return;
		}
		const height = Math.floor(px);
		if (this._pendingWidth !== undefined) {
			const width = this._pendingWidth;
			this._pendingWidth = undefined;
			this._canvas.setSurfaceSize(width, height);
			this._syncFit();
			return;
		}
		this._pendingHeight = height;
		microtask(() => {
			if (this._pendingHeight !== undefined) {
				this._canvas.surfaceHeight = this._pendingHeight;
				this._pendingHeight = undefined;
				this._syncFit();
			}
		});
	}

	onLayout(left: number, top: number, right: number, bottom: number) {
		super.onLayout(left, top, right, bottom);
		this._shimParent();
	}

	/** Libraries size and hit-test against the canvas's parent as a DOM element. */
	private _shimParent() {
		const parent: any = this.parent;
		if (!parent) {
			return;
		}
		const owner = new WeakRef(this);
		if (!Object.hasOwn(parent, 'clientWidth') && !Object.hasOwn(parent, 'clientHeight')) {
			// Measured size where the layout is NativeScript's, else the native view's.
			const dips = (view: any, isWidth: boolean) => {
				const measured = isWidth ? view.getMeasuredWidth() : view.getMeasuredHeight();
				if (measured > 0) {
					return Math.floor(measured / Screen.mainScreen.scale);
				}
				const rect = owner.deref()?._rectOf(view.nativeView);
				return Math.floor((isWidth ? rect?.width : rect?.height) ?? 0);
			};
			Object.defineProperties(parent, {
				clientWidth: {
					get: function () {
						return dips(this, true);
					},
				},
				clientHeight: {
					get: function () {
						return dips(this, false);
					},
				},
			});
		}
		if (typeof parent.getBoundingClientRect !== 'function') {
			parent.getBoundingClientRect = function () {
				const rect = owner.deref()?._rectOf(this.nativeView);
				return rect ? new DOMRect(rect.x, rect.y, rect.width, rect.height) : new DOMRect(0, 0, 0, 0);
			};
		}
		if (!Object.hasOwn(parent, 'ownerDocument')) {
			Object.defineProperty(parent, 'ownerDocument', {
				get: function () {
					return global?.window?.document ?? doc;
				},
			});
		}
	}

	public onMeasure(widthMeasureSpec: number, heightMeasureSpec: number) {
		if (this._canvas) {
			this.setMeasuredDimension(Utils.layout.getMeasureSpecSize(widthMeasureSpec), Utils.layout.getMeasureSpecSize(heightMeasureSpec));
		}
	}

	private _syncFit() {
		if (!this._canvas) {
			return;
		}
		const scale = Screen.mainScreen.scale || 1;
		const surface = { width: Math.floor(this._canvas.surfaceWidth / scale), height: Math.floor(this._canvas.surfaceHeight / scale) };
		this._canvas.fit = fitForStyle(this.style.width, this.style.height, surface, { width: this._viewWidth, height: this._viewHeight });
	}

	/** The platform reports the native view's laid-out size, in DIPs. */
	protected _onViewSize(width: number, height: number) {
		if (!this._canvas || (width === this._viewWidth && height === this._viewHeight)) {
			return;
		}
		this._viewWidth = width;
		this._viewHeight = height;
		this._canvas.setViewSize(width, height);
		this._followPercentSize();
		this._syncFit();
		this._shimParent();
		if (!this._isReady && width > 0 && height > 0 && this._canvas.surfaceWidth > 0 && this._canvas.surfaceHeight > 0) {
			this._isReady = true;
			// Out of the layout pass, as the iOS view does.
			setTimeout(() => this._readyEvent(), 0);
		}
	}

	/** The platform reports physical pixels per DIP for the native view (DPI and transforms). */
	protected _onViewScale(scaleX: number, scaleY: number) {
		if (scaleX === this._viewScaleX && scaleY === this._viewScaleY) {
			return;
		}
		this._viewScaleX = scaleX;
		this._viewScaleY = scaleY;
		this._canvas?.setCompositionScale(scaleX, scaleY);
		this._followPercentSize();
	}

	/**
	 * A `%` CSS size is a size relative to the laid-out view. Where NativeScript does not measure
	 * the view (native layout), it only becomes known here: size the buffer to the view's pixels.
	 */
	private _followPercentSize() {
		if (!this._canvas || this._viewWidth <= 0 || this._viewHeight <= 0) {
			return;
		}
		const followWidth = this._followWidth && this._pendingWidth === undefined;
		const followHeight = this._followHeight && this._pendingHeight === undefined;
		if (!followWidth && !followHeight) {
			return;
		}
		const width = followWidth ? Math.max(1, Math.round(this._viewWidth * (this._viewScaleX || Screen.mainScreen.scale || 1))) : this._canvas.surfaceWidth;
		const height = followHeight ? Math.max(1, Math.round(this._viewHeight * (this._viewScaleY || Screen.mainScreen.scale || 1))) : this._canvas.surfaceHeight;
		if (width !== this._canvas.surfaceWidth || height !== this._canvas.surfaceHeight) {
			this._canvas.setSurfaceSize(width, height);
		}
	}

	disposeNativeView(): void {
		this._2dContext = undefined;
		this._webglContext = undefined;
		this._webgl2Context = undefined;
		this._gpuContext = undefined;
		this._bitmapRendererContext = undefined;
		this._contextType = ContextType.None;
		this._isReady = false;
		this._canvas = undefined;
		super.disposeNativeView();
	}

	/**
	 * Builds the 2D context object. `getContext('2d')` keeps it as the canvas's context;
	 * `getContext('bitmaprenderer')` keeps it privately as the handle onto the output bitmap.
	 */
	private __create2DContext(type: CanvasContextType, options?: any): CanvasRenderingContext2D {
		const opts = { ...defaultOpts, ...handleContextOptions(type, options), fontColor: (this.parent?.style?.color?.argb ?? 0xff000000) | 0 };
		const ctx = this._canvas.create2DContext(opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible, opts.fontColor, opts.willReadFrequently ?? false, opts.colorSpace ?? 0);
		const context = new (CanvasRenderingContext2D as any)(ctx, opts);
		context._canvas = this;
		context._type = '2d';
		return context;
	}

	getContext(type: string, options?: any): CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | GPUCanvasContext | null {
		if (!this._canvas || typeof type !== 'string') {
			return null;
		}
		if (type === '2d') {
			if (this._webglContext || this._webgl2Context || this._gpuContext || this._bitmapRendererContext) {
				return null;
			}
			if (!this._2dContext) {
				this._2dContext = this.__create2DContext(type, options);
				this._contextType = ContextType.Canvas;
			}
			return this._2dContext;
		}
		if (type === 'bitmaprenderer') {
			if (this._2dContext || this._webglContext || this._webgl2Context || this._gpuContext) {
				return null;
			}
			if (!this._bitmapRendererContext) {
				// The output bitmap is the canvas's own 2D surface; the 2D context is the handle
				// onto it and is never handed out as a '2d' context.
				const backing = this.__create2DContext(type, options);
				this._bitmapRendererContext = new ImageBitmapRenderingContext(this as any, backing, handleContextOptions(type, options));
				this._contextType = ContextType.BitmapRenderer;
			}
			return this._bitmapRendererContext as never;
		}
		if (type === 'webgl' || type === 'experimental-webgl' || type === 'webgl2' || type === 'experimental-webgl2') {
			const isWebGL2 = type.endsWith('webgl2');
			// Hosts gain `initContext` with their WebGL backend.
			if (this._2dContext || this._bitmapRendererContext || this._gpuContext || (isWebGL2 ? this._webglContext : this._webgl2Context) || typeof this._canvas.initContext !== 'function') {
				return null;
			}
			if (isWebGL2 ? !this._webgl2Context : !this._webglContext) {
				const opts = { version: isWebGL2 ? 2 : 1, ...defaultOpts, ...handleContextOptions(type as CanvasContextType, options) };
				this._canvas.initContext(type, opts.alpha, false, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible, false, opts.colorSpace ?? 0);
				if (isWebGL2) {
					this._webgl2Context = new (WebGL2RenderingContext as any)(this._canvas, opts);
					(this._webgl2Context as any)._canvas = this;
					(this._webgl2Context as any)._type = 'webgl2';
					this._contextType = ContextType.WebGL2;
				} else {
					this._webglContext = new (WebGLRenderingContext as any)(this._canvas, opts);
					(this._webglContext as any)._canvas = this;
					this._webglContext._type = 'webgl';
					this._contextType = ContextType.WebGL;
				}
			}
			return isWebGL2 ? this._webgl2Context : this._webglContext;
		}
		if (type === 'webgpu') {
			// Hosts gain `initWebGPUContext` with their WebGPU backend.
			if (this._2dContext || this._webglContext || this._webgl2Context || this._bitmapRendererContext || typeof this._canvas.initWebGPUContext !== 'function') {
				return null;
			}
			if (!this._gpuContext) {
				this._canvas.initWebGPUContext(BigInt(navigator.gpu.native.__getPointer()));
				this._gpuContext = new (GPUCanvasContext as any)(this._canvas);
				(this._gpuContext as any)._canvas = this;
				(this._gpuContext as any)._type = 'webgpu';
				this._contextType = ContextType.WebGPU;
			}
			return this._gpuContext;
		}
		return null;
	}

	get __native__context() {
		switch (this._contextType) {
			case ContextType.Canvas:
				return this._2dContext?.native;
			case ContextType.BitmapRenderer:
				return this._bitmapRendererContext?.native;
			case ContextType.WebGL:
				return this._webglContext?.native;
			case ContextType.WebGL2:
				return this._webgl2Context?.native;
			case ContextType.WebGPU:
				return this._gpuContext?.native;
			default:
				return null;
		}
	}

	get native() {
		return this.__native__context;
	}

	toDataURL(type = 'image/png', encoderOptions = 0.92) {
		if (this.width === 0 || this.height === 0) {
			return 'data:,';
		}
		if (this._contextType === ContextType.WebGPU) {
			return this._gpuContext.__toDataURL(type, encoderOptions);
		}
		if (!this.native) {
			// No context yet: the (transparent) bitmap is still this.width x this.height.
			const blank = global.CanvasModule.CanvasRenderingContext2D.withCpu(this.width, this.height, 1, true, 0, 96, 0);
			return blank.__toDataURL(type, encoderOptions);
		}
		return this.native?.__toDataURL?.(type, encoderOptions);
	}

	snapshot(flip: boolean = false): ImageSource | null {
		// todo: ImageSource from the platform's native image type.
		return null;
	}

	/** A native view's rectangle in window DIPs, null when it is not laid out. */
	abstract _rectOf(nativeView: any): { x: number; y: number; width: number; height: number } | null;

	getBoundingClientRect() {
		const rect = this._canvas && this.parent ? this._rectOf(this.nativeViewProtected) : null;
		if (!rect) {
			return new DOMRect(0, 0, 0, 0);
		}
		return new DOMRect(rect.x, rect.y, rect.width, rect.height);
	}
}
