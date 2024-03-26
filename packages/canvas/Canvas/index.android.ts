import { CanvasBase, doc, ignoreTouchEventsProperty, DOMRect } from './common';
import { DOMMatrix } from '../Canvas2D';
import { CanvasRenderingContext2D } from '../Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from '../WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from '../WebGL2/WebGL2RenderingContext';
import { Application, View, profile, Device, Screen, knownFolders, ImageSource, Utils } from '@nativescript/core';

export function createSVGMatrix(): DOMMatrix {
	return new DOMMatrix();
}

export * from './common';

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
	xrCompatible: false
};

declare const org;

enum ContextType {
	None,
	Canvas,
	WebGL,
	WebGL2,
}

export class Canvas extends CanvasBase {
	_ready = false;
	private _2dContext: CanvasRenderingContext2D;
	private _webglContext: WebGLRenderingContext;
	private _webgl2Context: WebGL2RenderingContext;
	private _canvas;
	private _didPause: boolean = false;

	private _contextType = ContextType.None;
	private _is2D = false;
	private _isBatch = false;
	static useSurface = false;

	constructor(nativeInstance?) {
		super();
		if (nativeInstance) {
			// allows Worker usage
			this._canvas = nativeInstance;
			(global as any).__canvasLoaded = true;
		} else {
			const activity = Application.android.foregroundActivity || Application.android.startActivity || Utils.android.getApplicationContext();
			if (Canvas.useSurface) {
				this._canvas = new org.nativescript.canvas.NSCCanvas(activity, org.nativescript.canvas.NSCCanvas.SurfaceType.Surface);
			} else {
				this._canvas = new org.nativescript.canvas.NSCCanvas(activity);
			}

			(global as any).__canvasLoaded = true;
			const ref = new WeakRef(this);
			this._canvas.setTouchEventListener(
				new org.nativescript.canvas.NSCCanvas.TouchEvents({
					onEvent(event: string, nativeEvent: android.view.MotionEvent) {
						const owner = ref.get();
						if (!owner) {
							return;
						}
						owner._handleEvents(event);
					}
				})
			);
		}
	}

	[ignoreTouchEventsProperty.setNative](value: boolean) {
		this._canvas.setIgnoreTouchEvents(value);
	}

	// @ts-ignore
	get android() {
		return this._canvas;
	}

	get clientWidth() {
		return this.width;
	}

	get clientHeight() {
		return this.height;
	}

	_drawingBufferHeight = 0;
	get drawingBufferHeight() {
		return this._drawingBufferHeight;
	}

	_drawingBufferWidth = 0;
	get drawingBufferWidth() {
		return this._drawingBufferWidth;
	}

	// @ts-ignore
	get width(): any {
		return this._logicalSize.width;
	}

	set width(value) {
		if (value === undefined || value === null) {
			this.style.width = 0;
		} else {
			this.style.width = value;
		}
		this._didLayout = false;
		this._layoutNative();
	}

	// @ts-ignore
	get height(): any {
		return this._logicalSize.height;
	}

	set height(value) {
		if (value === undefined || value === null) {
			this.style.height = 0;
		} else {
			this.style.height = value;
		}
		this._didLayout = false;
		this._layoutNative();
	}

	static createCustomView() {
		const canvas = new Canvas();
		canvas._isBatch = true;
		canvas._isCustom = true;
		canvas.width = 300;
		canvas.height = 150;
		canvas._isBatch = false;
		canvas._layoutNative();
		return canvas;
	}

	createNativeView() {
		return this._canvas;
	}

	initNativeView(): void {
		super.initNativeView();
		const ref = new WeakRef(this);
		this.on(View.layoutChangedEvent, (args) => {
			const parent = this.parent as any;
			// TODO change DIPs once implemented
			if (parent && parent.clientWidth === undefined && parent.clientHeight === undefined) {
				Object.defineProperty(parent, 'clientWidth', {
					get: function() {
						return parent.getMeasuredWidth() / Screen.mainScreen.scale;
					}
				});
				Object.defineProperty(parent, 'clientHeight', {
					get: function() {
						return parent.getMeasuredHeight() / Screen.mainScreen.scale;
					}
				});
			}
			if (parent && typeof parent.getBoundingClientRect !== 'function') {
				parent.getBoundingClientRect = function() {
					const view = this;
					const nativeView = view.android;
					const width = this.width;
					const height = this.height;
					const scale = Screen.mainScreen.scale;
					return {
						bottom: nativeView.getBottom() / scale,
						height: height,
						left: nativeView.getLeft() / scale,
						right: nativeView.getRight() / scale,
						top: nativeView.getTop() / scale,
						width: width,
						x: nativeView.getX() / scale,
						y: nativeView.getY() / scale
					};
				};
			}

			if (parent && parent.ownerDocument === undefined) {
				Object.defineProperty(parent, 'ownerDocument', {
					get: function() {
						return global?.window?.document ?? doc;
					}
				});
			}
		});
		this._canvas.setListener(
			new org.nativescript.canvas.NSCCanvas.Listener({
				contextReady() {
					const owner = ref.get() as any;
					if (owner && !owner._ready) {
						owner._ready = true;
						owner._readyEvent();
					}
				},
				surfaceResize(width, height) {
					// if(this._webglContext || this._webgl2Context){
					// 	(this._webglContext || this._webgl2Context)?.resize();
					// }

					const owner = ref.get() as any;
					if (owner) {
						owner._drawingBufferWidth = width / Screen.mainScreen.scale;
						owner._drawingBufferHeight = height / Screen.mainScreen.scale;
					}
				}
			})
		);
	}

	onUnloaded() {
		this._didPause = true;
		// if (this._canvas) {
		// 	this._canvas.onPause();
		// }
		super.onUnloaded();
	}

	@profile
	onLoaded() {
		super.onLoaded();
		if (this._didPause) {
			this._didPause = false;
			// if (this._canvas) {
			// 	this._canvas.onResume();
			// }
		}
	}

	disposeNativeView(): void {
		this._canvas.setListener(null);
		this._canvas = undefined;
		super.disposeNativeView();
	}

	toDataURL(type = 'image/png', encoderOptions = 0.92) {
		return this.native.__toDataURL(type, encoderOptions);
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

	_layoutNative() {
		if (!this._isCustom) {
			return;
		}
		if (this._isBatch) {
			return;
		}
		if (this._didLayout) {
			return;
		}
		if (!this.parent) {
			if ((typeof this.width === 'string' && this.width.indexOf('%')) || (typeof this.height === 'string' && this.height.indexOf('%'))) {
				return;
			}

			if (this._canvas === undefined || this._canvas === null) {
				return;
			}

			const size = this._physicalSize;
			org.nativescript.canvas.NSCCanvas.layoutView(size.width || 0, size.height || 0, this._canvas);

			//	this._drawingBufferWidth = size.width || 1;
			//	this._drawingBufferHeight = size.height || 1;
			if (this._is2D) {
				this._2dContext.native.__resize(size.width, size.height);
			}
			this._didLayout = true;
		}
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

	_didLayout = false;

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
					const opts = {
						...defaultOpts, ...this._handleContextOptions(type, options),
						fontColor: this.parent?.style?.color?.android || -16777216
					};

					const ctx = this._canvas.create2DContext(opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible, opts.fontColor);
					this._2dContext = new (CanvasRenderingContext2D as any)(ctx);
					(this._2dContext as any)._canvas = this;
					this._2dContext._type = '2d';
					this._contextType = ContextType.Canvas;
					this._is2D = true;
				}

				return this._2dContext;
			} else if (type === 'webgl') {
				if (this._2dContext || this._webgl2Context) {
					return null;
				}
				if (!this._webglContext) {
					this._layoutNative();
					const opts = { version: 1, ...defaultOpts, ...this._handleContextOptions(type, options) };
					this._canvas.initContext(type, opts.alpha, false, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible);
					this._webglContext = new (WebGLRenderingContext as any)(this._canvas, opts);
					(this._webglContext as any)._canvas = this;
					this._webglContext._type = 'webgl';
					this._contextType = ContextType.WebGL;
				}

				return this._webglContext;
			} else if (type === 'webgl2' || type === 'experimental-webgl') {
				if (this._2dContext || this._webglContext) {
					return null;
				}
				if (!this._webgl2Context) {
					this._layoutNative();
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

	private _jsBuffer: Float32Array;

	private get _boundingClientRect() {
		if (this._jsBuffer === undefined) {
			this._jsBuffer = new Float32Array(8);
			this._canvas?.setBoundsBuffer?.(this._jsBuffer);
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
		org.nativescript.canvas.NSCCanvas.getBoundingClientRect(this._canvas);
		return new DOMRect(this._boundingClientRect[6], this._boundingClientRect[7], this._boundingClientRect[4], this._boundingClientRect[5], this._boundingClientRect[0], this._boundingClientRect[1], this._boundingClientRect[2], this._boundingClientRect[3]);
	}

	setPointerCapture() {
	}

	releasePointerCapture() {
	}
}
