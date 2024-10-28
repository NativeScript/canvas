import { CanvasBase, doc, ignoreTouchEventsProperty, DOMRect } from './common';
import { DOMMatrix } from '../Canvas2D';
import { CanvasRenderingContext2D } from '../Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from '../WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from '../WebGL2/WebGL2RenderingContext';
import { Application, View, Screen, ImageSource, Utils, widthProperty, heightProperty } from '@nativescript/core';
import { GPUCanvasContext } from '../WebGPU';
import { handleContextOptions } from './utils';

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
	xrCompatible: false,
};

// declare const org;

enum ContextType {
	None,
	Canvas,
	WebGL,
	WebGL2,
	WebGPU,
}

function updateFit(canvas) {
	const styleWidth = canvas.style.width;
	const styleHeight = canvas.style.height;
	if (typeof styleWidth === 'object' && typeof styleHeight === 'object') {
		if (styleWidth?.unit === '%' && styleWidth.value >= 1 && styleHeight?.unit === '%' && styleHeight.value >= 1) {
			canvas._canvas.setFit(org.nativescript.canvas.CanvasFit.Fill);
		} else if ((styleWidth?.unit === 'px' || styleWidth?.unit === 'dip') && (styleHeight?.unit === 'px' || styleHeight?.unit === 'dip')) {
			const width = canvas._canvas.getSurfaceWidth();
			const height = canvas._canvas.getSurfaceHeight();
			const viewWidth = canvas._canvas.getMeasuredWidth();
			const viewHeight = canvas._canvas.getMeasuredHeight();
			if (viewWidth > width || viewHeight > height) {
				canvas._canvas.setFit(org.nativescript.canvas.CanvasFit.ScaleDown);
			} else {
				canvas._canvas.setFit(org.nativescript.canvas.CanvasFit.Fill);
			}
		} else {
			canvas._canvas.setFit(org.nativescript.canvas.CanvasFit.Fill);
		}
	} else if (typeof styleWidth === 'object' && styleHeight === 'auto') {
		if (styleWidth?.unit === 'px' || styleWidth?.unit === 'dip' || styleWidth?.unit === '%') {
			canvas._canvas.setFit(org.nativescript.canvas.CanvasFit.FitX);
		}
	} else if (styleWidth === 'auto' && typeof styleHeight === 'object') {
		if (styleHeight?.unit === 'px' || styleHeight?.unit === 'dip' || styleHeight?.unit === '%') {
			canvas._canvas.setFit(org.nativescript.canvas.CanvasFit.FitY);
		}
	} else if (styleWidth === 'auto' && styleHeight === 'auto') {
		// // when auto/auto is set force the frame size to be the same as the canvas
		// const width = Math.floor(canvas._canvas.surfaceWidth / Screen.mainScreen.scale);
		// const height = Math.floor(canvas._canvas.surfaceHeight / Screen.mainScreen.scale);
		// const newFrame = CGRectMake(frame.origin.x, frame.origin.y, width, height);
		// canvas._canvas.setFit(org.nativescript.canvas.CanvasFit.None);
		// canvas._canvas.getMeasuredHeight()
	} else {
		canvas._canvas.setFit(org.nativescript.canvas.CanvasFit.FitX);
	}
}

const viewRect_ = Symbol('[[viewRect]]');

function valueToNumber(value) {
	switch (typeof value) {
		case 'string':
			return parseFloat(value);
		case 'number':
			return value;
		default:
			return NaN;
	}
}
export class Canvas extends CanvasBase {
	_ready = false;
	private _2dContext: CanvasRenderingContext2D;
	private _webglContext: WebGLRenderingContext;
	private _webgl2Context: WebGL2RenderingContext;
	private _gpuContext: GPUCanvasContext;
	private _canvas: org.nativescript.canvas.NSCCanvas;
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
					},
				})
			);
		}
	}

	static get forceGL() {
		return org.nativescript.canvas.NSCCanvas.getForceGL();
	}

	static set forceGL(value) {
		org.nativescript.canvas.NSCCanvas.setForceGL(value);
	}

	[ignoreTouchEventsProperty.setNative](value: boolean) {
		this._canvas.setIgnoreTouchEvents(value);
	}

	// @ts-ignore
	get android() {
		return this._canvas;
	}

	get clientWidth() {
		const width = this.getMeasuredWidth();
		if (width === 0) {
			return 0;
		}
		return width / Screen.mainScreen.scale;
	}

	get clientHeight() {
		const height = this.getMeasuredHeight();
		if (height === 0) {
			return 0;
		}
		return height / Screen.mainScreen.scale;
	}

	get drawingBufferHeight() {
		if (this._canvas === undefined || this._canvas === null) {
			return 0;
		}
		return this._canvas.getDrawingBufferHeight();
	}

	get drawingBufferWidth() {
		if (this._canvas === undefined || this._canvas === null) {
			return 0;
		}
		return this._canvas.getDrawingBufferWidth();
	}

	// @ts-ignore
	get width(): number {
		if (this._canvas === undefined || this._canvas === null) {
			return 0;
		}
		return this._canvas.getSurfaceWidth();
	}

	set width(value: number) {
		if (this._canvas === undefined || this._canvas === null) {
			return;
		}
		value = valueToNumber(value);
		if (!Number.isNaN(value)) {
			this._canvas.setSurfaceWidth(value);
		}
	}

	// @ts-ignore
	get height(): number {
		if (this._canvas === undefined || this._canvas === null) {
			return 0;
		}
		return this._canvas.getSurfaceHeight();
	}

	set height(value: number) {
		if (this._canvas === undefined || this._canvas === null) {
			return;
		}
		value = valueToNumber(value);
		if (!Number.isNaN(value)) {
			this._canvas.setSurfaceHeight(value);
		}
	}

	[widthProperty.setNative](value) {
		updateFit(this);
	}

	[heightProperty.setNative](value) {
		updateFit(this);
	}

	static createCustomView() {
		const canvas = new Canvas();
		canvas._isCustom = true;
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
					get: function () {
						return parent.getMeasuredWidth() / Screen.mainScreen.scale;
					},
				});
				Object.defineProperty(parent, 'clientHeight', {
					get: function () {
						return parent.getMeasuredHeight() / Screen.mainScreen.scale;
					},
				});
			}
			if (parent && typeof parent.getBoundingClientRect !== 'function') {
				parent.getBoundingClientRect = function () {
					const view = this;
					if (!view) {
						return new DOMRect(0, 0, 0, 0);
					}
					const nativeView = view.nativeView;
					if (!view[viewRect_]) {
						view[viewRect_] = new Float32Array(8);
					}
					if (!nativeView) {
						return new DOMRect(0, 0, 0, 0);
					}

					nativeView.getBoundingClientRect(view[viewRect_]);
					return new DOMRect(view[viewRect_][6], view[viewRect_][7], view[viewRect_][4], view[viewRect_][5], view[viewRect_][0], view[viewRect_][1], view[viewRect_][2], view[viewRect_][3]);
				};
			}

			if (parent && parent.ownerDocument === undefined) {
				Object.defineProperty(parent, 'ownerDocument', {
					get: function () {
						return global?.window?.document ?? doc;
					},
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
				surfaceResize(width, height) {},
				surfaceDestroyed() {},
				surfaceCreated() {},
			})
		);
	}

	disposeNativeView(): void {
		this._canvas.setListener(null);
		this._canvas = undefined;
		super.disposeNativeView();
	}

	toDataURL(type = 'image/png', encoderOptions = 0.92) {
		if (this.width === 0 || this.height === 0) {
			return 'data:,';
		}
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

	get __native__context() {
		switch (this._contextType) {
			case ContextType.Canvas:
				return this._2dContext.native;
			case ContextType.WebGL:
				return this._webglContext.native;
			case ContextType.Canvas:
				return this._webgl2Context.native;
			case ContextType.WebGPU:
				return this._gpuContext;
			default:
				return null;
		}
	}

	get native() {
		return this.__native__context;
	}

	getContext(type: string, contextAttributes?: any): CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | GPUCanvasContext | null {
		if (!this._canvas) {
			return null;
		}

		if (typeof type === 'string') {
			if (type === '2d') {
				if (this._webglContext || this._webgl2Context) {
					return null;
				}

				if (!this._2dContext) {
					const opts = {
						...defaultOpts,
						...handleContextOptions(type, contextAttributes),
						fontColor: this.parent?.style?.color?.android ?? -16777216,
					};

					const ctx = this._canvas.create2DContext(opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible);
					this._2dContext = new (CanvasRenderingContext2D as any)(ctx);
					(this._2dContext as any)._canvas = this;
					this._2dContext._type = '2d';
					this._contextType = ContextType.Canvas;
					this._is2D = true;
				}

				return this._2dContext;
			} else if (type === 'webgl' || type === 'experimental-webgl') {
				if (this._2dContext || this._webgl2Context) {
					return null;
				}
				if (!this._webglContext) {
					const opts = { version: 1, ...defaultOpts, ...handleContextOptions(type, contextAttributes) };
					this._canvas.initContext(type, opts.alpha, false, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible);
					this._webglContext = new (WebGLRenderingContext as any)(this._canvas, opts);
					(this._webglContext as any)._canvas = this;
					this._webglContext._type = 'webgl';
					this._contextType = ContextType.WebGL;
				}
				return this._webglContext;
			} else if (type === 'webgl2' || type === 'experimental-webgl2') {
				if (this._2dContext || this._webglContext) {
					return null;
				}
				if (!this._webgl2Context) {
					const opts = { version: 2, ...defaultOpts, ...handleContextOptions(type, contextAttributes) };
					this._canvas.initContext(type, opts.alpha, false, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible);
					this._webgl2Context = new (WebGL2RenderingContext as any)(this._canvas, opts);
					(this._webgl2Context as any)._canvas = this;
					(this._webgl2Context as any)._type = 'webgl2';
					this._contextType = ContextType.WebGL2;
				}
				return this._webgl2Context;
			} else if (type === 'webgpu') {
				if (this._2dContext || this._webglContext || this._webgl2Context) {
					return null;
				}
				if (!this._gpuContext) {
					const ptr = navigator.gpu.native.__getPointer();
					this._canvas.initWebGPUContext(long(ptr));
					this._gpuContext = new (GPUCanvasContext as any)(this._canvas);
					(this._gpuContext as any)._canvas = this;
					(this._gpuContext as any)._type = 'webgpu';
					this._contextType = ContextType.WebGPU;
				}
				return this._gpuContext;
			}
		}
		return null;
	}

	private _jsBuffer: Float32Array;

	private get _boundingClientRect() {
		if (this._jsBuffer === undefined) {
			this._jsBuffer = new Float32Array(8);
			this._canvas?.setBoundsBuffer?.(this._jsBuffer as never);
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
		if (!this._canvas || !this.parent) {
			return new DOMRect(0, 0, 0, 0);
		}
		org.nativescript.canvas.NSCCanvas.getBoundingClientRect(this._canvas);
		return new DOMRect(this._boundingClientRect[6], this._boundingClientRect[7], this._boundingClientRect[4], this._boundingClientRect[5], this._boundingClientRect[0], this._boundingClientRect[1], this._boundingClientRect[2], this._boundingClientRect[3]);
	}

	setPointerCapture() {}

	releasePointerCapture() {}
}
