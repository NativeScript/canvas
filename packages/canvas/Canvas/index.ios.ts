import { CanvasBase, ignorePixelScalingProperty } from './common';
import { DOMMatrix } from '../Canvas2D';
import { CanvasRenderingContext2D } from '../Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from '../WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from '../WebGL2/WebGL2RenderingContext';
import { ImageSource, Utils, profile } from '@nativescript/core';
declare var NSCCanvas, NSCCanvasListener;

export * from './common';

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
};

enum ContextType {
	None,
	Canvas,
	WebGL,
	WebGL2,
}

export class Canvas extends CanvasBase {
	private _2dContext: CanvasRenderingContext2D;
	private _webglContext: WebGLRenderingContext;
	private _webgl2Context: WebGL2RenderingContext;
	private _canvas: any;
	private _didPause: boolean = false;
	private _isReady: boolean = false;
	private _readyListener: any;

	private _contextType = ContextType.None;
	private _is2D = false;

	_didLayout = false;

	constructor() {
		super();
		this._canvas = NSCCanvas.alloc().initWithFrame(CGRectZero);
		const ref = new WeakRef(this);
		const listener = (NSObject as any).extend(
			{
				contextReady() {
					if (!this._isReady) {
						const owner = ref.get();
						if (owner) {
							owner._readyEvent();
							this._isReady = true;
						}
					}
				},
			},
			{
				protocols: [NSCCanvasListener],
			}
		);
		this._readyListener = listener.new();
		this._canvas.setListener(this._readyListener);
		this._canvas.enterBackgroundListener = () => {
			if (!this.native) {
				return;
			}
			this.native.__stopRaf();
		};

		this._canvas.becomeActiveListener = () => {
			if (!this.native) {
				return;
			}
			this.native.__startRaf();
		};
	}

	[ignorePixelScalingProperty.setNative](value: boolean) {
		this._canvas.ignorePixelScaling = value;
	}

	// @ts-ignore
	get ios() {
		return this._canvas;
	}

	get clientWidth() {
		return this.width;
	}

	get clientHeight() {
		return this.height;
	}

	//@ts-ignore
	get width() {
		const measuredWidth = this.getMeasuredWidth();
		if (measuredWidth !== 0) {
			return measuredWidth;
		}
		return this._realSize.width;
	}

	set width(value) {
		this.style.width = value;
		this._didLayout = false;
		this._layoutNative();
	}

	//@ts-ignore
	get height() {
		const measuredHeight = this.getMeasuredHeight();
		if (measuredHeight !== 0) {
			return measuredHeight;
		}
		return this._realSize.height;
	}

	set height(value) {
		this.style.height = value;
		this._didLayout = false;
		this._layoutNative();
	}

	private _iosOverflowSafeArea = false;

	//@ts-ignore
	get iosOverflowSafeArea() {
		return this._iosOverflowSafeArea;
	}

	set iosOverflowSafeArea(value: boolean) {
		const window = UIApplication.sharedApplication.windows[0];
		const topPadding = window.safeAreaInsets.top;
		const bottomPadding = window.safeAreaInsets.bottom;
		if (bottomPadding === 0) {
			this._iosOverflowSafeArea = false;
		} else {
			this._iosOverflowSafeArea = value;
		}
	}

	static createCustomView() {
		const canvas = new Canvas();
		canvas.width = 300;
		canvas.height = 150;
		canvas._isCustom = true;
		canvas._layoutNative();
		return canvas;
	}

	onLayout(left: number, top: number, right: number, bottom: number) {
		super.onLayout(left, top, right, bottom);
		const parent = this.parent as any;
		// TODO change DIPs once implemented
		if (parent && parent.clientWidth === undefined && parent.clientHeight === undefined) {
			Object.defineProperty(parent, 'clientWidth', {
				get: function () {
					return parent.getMeasuredWidth();
				},
			});
			Object.defineProperty(parent, 'clientHeight', {
				get: function () {
					return parent.getMeasuredHeight();
				},
			});
		}
	}

	createNativeView() {
		return this._canvas;
	}

	initNativeView() {
		super.initNativeView();
		this._canvas.ignorePixelScaling = this.ignorePixelScaling;
	}

	onUnloaded() {
		//	this.ios.pause();
		this._didPause = true;
		super.onUnloaded();
	}

	@profile
	onLoaded() {
		super.onLoaded();
		if (this._didPause) {
			//	this.ios.resume();
			this._didPause = false;
		}
	}

	disposeNativeView(): void {
		this._canvas.setListener(null);
		this._readyListener = undefined;
		super.disposeNativeView();
	}

	_layoutNative() {
		if (!this._isCustom) {
			return;
		}

		if (this._didLayout) {
			return;
		}

		if (!this.parent) {
			if ((typeof this.style.width === 'string' && this.style.width.indexOf('%')) || (typeof this.style.height === 'string' && this.style.height.indexOf('%'))) {
				return;
			}

			const size = this._realSize;

			const width = Utils.layout.toDeviceIndependentPixels(size.width || 1);
			const height = Utils.layout.toDeviceIndependentPixels(size.height || 1);

			this._canvas.forceLayout(width, height);

			this._didLayout = true;
		}
	}

	getContext(type: string, options?: any): CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | null {
		if (!this._canvas) {
			return null;
		}
		if (typeof type === 'string') {
			if (type === '2d') {
				if (this._webglContext || this._webgl2Context) {
					return null;
				}

				if (!this._2dContext) {
					this._layoutNative();
					const opts = Object.assign(defaultOpts, this._handleContextOptions(type, options));

					opts['fontColor'] = this.parent?.style?.color?.android || -16777216;

					//	this._canvas.initContext(type, opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.desynchronized, opts.xrCompatible);

					const ctx = this._canvas.create2DContext(opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible, opts.fontColor);

					this._2dContext = new (CanvasRenderingContext2D as any)(ctx);

					//this._2dContext = new (CanvasRenderingContext2D as any)(this._canvas, opts);

					// // @ts-ignore
					(this._2dContext as any)._canvas = this;

					this._contextType = ContextType.Canvas;
					// @ts-ignore
					this._2dContext._type = '2d';
				}

				return this._2dContext;
			} else if (type === 'webgl' || type === 'experimental-webgl') {
				if (this._2dContext || this._webgl2Context) {
					return null;
				}
				if (!this._webglContext) {
					this._layoutNative();
					const opts = Object.assign({ version: 'v1' }, Object.assign(defaultOpts, this._handleContextOptions(type, options)));

					this._canvas.initContext(type, opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible);

					this._webglContext = new (WebGLRenderingContext as any)(this._canvas, opts);
					(this._webglContext as any)._canvas = this;
					this._webglContext._type = 'webgl';
					this._contextType = ContextType.WebGL;
				}
				return this._webglContext;
			} else if (type === 'webgl2') {
				if (this._2dContext || this._webglContext) {
					return null;
				}

				if (!this._webgl2Context) {
					this._layoutNative();
					const opts = Object.assign({ version: 'v2' }, Object.assign(defaultOpts, this._handleContextOptions(type, options)));

					this._canvas.initContext(type, opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible);

					this._webgl2Context = new (WebGL2RenderingContext as any)(this._canvas, opts);
					(this._webgl2Context as any)._canvas = this;
					(this._webgl2Context as any)._type = 'webgl2';
					this._contextType = ContextType.WebGL2;
				}
				return this._webgl2Context;
			}
		}
		return null;
	}

	get __native__context() {
		switch (this._contextType) {
			case ContextType.Canvas:
				return this._2dContext.native;
			case ContextType.WebGL:
				return this._webglContext.native;
			case ContextType.Canvas:
				return this._webgl2Context.native;
			default:
				return null;
		}
	}

	get native() {
		return this.__native__context;
	}

	toDataURL(type = 'image/png', encoderOptions = 0.92) {
		return this.native.toDataURL(type, encoderOptions);
	}

	snapshot(flip: boolean = false): ImageSource | null {
		if (this._canvas) {
			const bm = this._canvas.snapshot?.(flip ?? false);
			if (bm) {
				return new ImageSource(bm);
			}
		}
		return null;
	}

	getBoundingClientRect(): {
		x: number;
		y: number;
		width: number;
		height: number;
		top: number;
		right: number;
		bottom: number;
		left: number;
	} {
		const view = this;
		const frame = view.ios.frame as CGRect;
		const width = view.width;
		const height = view.height;
		return {
			bottom: height,
			height: height,
			left: frame.origin.x,
			right: width,
			top: frame.origin.y,
			width: width,
			x: frame.origin.x,
			y: frame.origin.y,
		};
	}
}
