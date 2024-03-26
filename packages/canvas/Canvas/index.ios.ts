import { CanvasBase, doc, ignoreTouchEventsProperty, DOMRect } from './common';
import { DOMMatrix } from '../Canvas2D';
import { CanvasRenderingContext2D } from '../Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from '../WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from '../WebGL2/WebGL2RenderingContext';
import { ImageSource, Utils, profile, Screen } from '@nativescript/core';
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
	private _isBatch = false;
	_didLayout = false;

	static useSurface = false;
	_renderer;

	constructor(nativeInstance?) {
		super();
		if (nativeInstance) {
			// allows Worker usage
			this._canvas = nativeInstance;
			(global as any).__canvasLoaded = true;
		} else {
			this._canvas = NSCCanvas.alloc().initWithFrame(CGRectZero);
			this._canvas.userInteractionEnabled = true;
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

			this._canvas.touchEventListener = (event, recognizer) => {
				this._handleEvents(event);
			};
		}
	}

	[ignoreTouchEventsProperty.setNative](value: boolean) {
		this._canvas.ignoreTouchEvents = value;
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
		return this._logicalSize.width;
	}

	set width(value) {
		this.style.width = value;
		this._didLayout = false;
		this._layoutNative(true);
	}

	//@ts-ignore
	get height() {
		return this._logicalSize.height;
	}

	set height(value) {
		this.style.height = value ?? 0;
		this._didLayout = false;
		this._layoutNative(true);
	}

	private _iosOverflowSafeArea = false;

	//@ts-ignore
	get iosOverflowSafeArea() {
		return this._iosOverflowSafeArea;
	}

	set iosOverflowSafeArea(value: boolean) {
		const window = UIApplication.sharedApplication.windows[0];
		//const topPadding = window.safeAreaInsets.top;
		const bottomPadding = window.safeAreaInsets.bottom;
		if (bottomPadding === 0) {
			this._iosOverflowSafeArea = false;
		} else {
			this._iosOverflowSafeArea = value;
		}
	}

	static createCustomView() {
		const canvas = new Canvas();
		canvas._isBatch = true;
		canvas._isCustom = true;
		canvas.style.width = 300;
		canvas.style.height = 150;
		canvas._isBatch = false;
		canvas._layoutNative();
		return canvas;
	}

	onLayout(left: number, top: number, right: number, bottom: number) {
		super.onLayout(left, top, right, bottom);
		if (!this.parent) {
			return;
		}
		if (!Object.hasOwn(this.parent, 'clientWidth') && !Object.hasOwn(this.parent, 'clientHeight')) {
			Object.defineProperties(this.parent, {
				clientWidth: {
					get: function () {
						return this.getMeasuredWidth() / Screen.mainScreen.scale;
					},
				},
				clientHeight: {
					get: function () {
						return this.getMeasuredHeight() / Screen.mainScreen.scale;
					},
				},
			});
		}

		if (typeof (<any>this.parent).getBoundingClientRect !== 'function') {
			(<any>this.parent).getBoundingClientRect = function () {
				const view = this;
				const nativeView = view.nativeView ?? view.ios;
				if (!nativeView) {
					return {
						bottom: 0,
						height: 0,
						left: 0,
						right: 0,
						top: 0,
						width: 0,
						x: 0,
						y: 0,
					};
				}
				const frame = nativeView.frame as CGRect;
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
			};
		}

		if (!Object.hasOwn(this.parent, 'ownerDocument')) {
			Object.defineProperty(this.parent, 'ownerDocument', {
				get: function () {
					return global?.window?.document ?? doc;
				},
			});
		}
	}

	public onMeasure(widthMeasureSpec: number, heightMeasureSpec: number) {
		const nativeView = this.nativeView;
		if (nativeView) {
			const width = Utils.layout.getMeasureSpecSize(widthMeasureSpec);
			const height = Utils.layout.getMeasureSpecSize(heightMeasureSpec);
			this.setMeasuredDimension(width, height);
		}
	}

	createNativeView() {
		return this._canvas;
	}

	initNativeView() {
		super.initNativeView();
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
		this._canvas?.setListener?.(null);
		this._readyListener = undefined;
		this._canvas = undefined;
		super.disposeNativeView();
	}

	_layoutNative(force: boolean = false) {
		if (this._isBatch) {
			return;
		}

		if (!this._isCustom && !force) {
			return;
		}

		if (this._didLayout && !force) {
			return;
		}

		if (!this.parent || force) {
			if (!force || (typeof this.style.width === 'string' && this.style.width.indexOf('%')) || (typeof this.style.height === 'string' && this.style.height.indexOf('%'))) {
				return;
			}

			if (this._canvas === undefined || this._canvas === null) {
				return;
			}

			const size = this._logicalSize;

			// todo revisit

		//	const width = Utils.layout.toDevicePixels(size.width || 1);
		//	const height = Utils.layout.toDevicePixels(size.height || 1);

			this._canvas.forceLayout(size.width, size.height);

			// if (this._is2D) {
			// 	this._2dContext?.native?.__resize?.(width, height);
			// }

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
					this._layoutNative(true);

					const opts = { ...defaultOpts, ...this._handleContextOptions(type, options), fontColor: this.parent?.style?.color?.android || -16777216 };

					const ctx = this._canvas.create2DContext(opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible, opts.fontColor);

					this._2dContext = new (CanvasRenderingContext2D as any)(ctx);

					// // @ts-ignore
					(this._2dContext as any)._canvas = this;

					this._contextType = ContextType.Canvas;
					// @ts-ignore
					this._2dContext._type = '2d';
					this._is2D = true;
				}

				return this._2dContext;
			} else if (type === 'webgl' || type === 'experimental-webgl') {
				if (this._2dContext || this._webgl2Context) {
					return null;
				}
				if (!this._webglContext) {
					this._layoutNative(true);
					const opts = { version: 1, ...defaultOpts, ...this._handleContextOptions(type, options) };

					this._canvas.initContext(type, opts.alpha, false, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible);

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
					this._layoutNative(true);
					const opts = { version: 2, ...defaultOpts, ...this._handleContextOptions(type, options) };

					this._canvas.initContext(type, opts.alpha, false, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible);

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
			case ContextType.WebGL2:
				return this._webgl2Context.native;
			default:
				return null;
		}
	}

	get native() {
		return this.__native__context;
	}

	toDataURL(type = 'image/png', encoderOptions = 0.92) {
		return this.native?.__toDataURL?.(type, encoderOptions);
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

	private _jsBuffer: Float32Array;

	private get _boundingClientRect() {
		if (this._jsBuffer === undefined) {
			this._jsBuffer = new Float32Array(8);
		}
		return this._jsBuffer;
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
		if (!this._canvas) {
			return new DOMRect(0, 0, 0, 0);
		}

		NSCCanvas.getBoundingClientRect(this._canvas, this._boundingClientRect);

		return new DOMRect(this._boundingClientRect[6], this._boundingClientRect[7], this._boundingClientRect[4], this._boundingClientRect[5], this._boundingClientRect[0], this._boundingClientRect[1], this._boundingClientRect[2], this._boundingClientRect[3]);
	}
}
