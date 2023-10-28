import { CanvasBase, ignorePixelScalingProperty, upscaleProperty } from './common';
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
	xrCompatible: false,
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

	constructor() {
		super();
		const activity = Application.android.foregroundActivity || Application.android.startActivity || Utils.android.getApplicationContext();
		this._canvas = new org.nativescript.canvas.NSCCanvas(activity);
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

	[upscaleProperty.setNative](value: boolean) {
		this._canvas.setUpscale(value);
	}

	[ignorePixelScalingProperty.setNative](value: boolean) {
		this._canvas.setIgnorePixelScaling(value);
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
		if (this._drawingBufferHeight === 0) {
			return this.getMeasuredHeight();
		}
		return this._drawingBufferHeight;
	}

	_drawingBufferWidth = 0;
	get drawingBufferWidth() {
		if (this._drawingBufferWidth === 0) {
			return this.getMeasuredWidth();
		}
		return this._drawingBufferWidth;
	}

	// @ts-ignore
	get width(): any {
		if (this.getMeasuredWidth() > 0) {
			return this.getMeasuredWidth() / Screen.mainScreen.scale;
		}
		const width = this._canvas.getWidth();
		if (width === 0) {
			let rootParams = this._canvas.getLayoutParams();
			if (rootParams) {
				return rootParams.width / Screen.mainScreen.scale;
			}
		}
		return width / Screen.mainScreen.scale;
	}

	set width(value) {
		this.style.width = value;
		this._didLayout = false;
		this._layoutNative();
	}

	// @ts-ignore
	get height(): any {
		if (this.getMeasuredHeight() > 0) {
			return this.getMeasuredHeight() / Screen.mainScreen.scale;
		}
		const height = this._canvas.getHeight();
		if (height === 0) {
			let rootParams = this._canvas.getLayoutParams();
			if (rootParams) {
				return rootParams.height / Screen.mainScreen.scale;
			}
		}
		return height / Screen.mainScreen.scale;
	}

	set height(value) {
		this.style.height = value;
		this._didLayout = false;
		this._layoutNative();
	}

	static createCustomView() {
		const canvas = new Canvas();
		canvas.width = 300;
		canvas.height = 150;
		canvas._isCustom = true;
		canvas._layoutNative();
		return canvas;
	}

	createNativeView() {
		return this._canvas;
	}

	initNativeView(): void {
		super.initNativeView();
		if (this.ignorePixelScaling) {
			this._canvas.setIgnorePixelScaling(this.ignorePixelScaling);
		}
		const ref = new WeakRef(this);
		this.on(View.layoutChangedEvent, (args) => {
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
						owner._drawingBufferWidth = width;
						owner._drawingBufferHeight = height;
					}
				},
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
		if (this._didLayout) {
			return;
		}
		if (!this.parent) {
			if ((typeof this.width === 'string' && this.width.indexOf('%')) || (typeof this.height === 'string' && this.height.indexOf('%'))) {
				return;
			}
			const size = this._realSize;
			org.nativescript.canvas.NSCCanvas.layoutView(size.width || 0, size.height || 0, this._canvas);

			this._drawingBufferWidth = size.width;
			this._drawingBufferHeight = size.height;
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
					const opts = Object.assign(defaultOpts, this._handleContextOptions(type, options));

					opts['fontColor'] = this.parent?.style?.color?.android || -16777216;

					const ctx = this._canvas.create2DContext(opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible, opts.fontColor);
					this._2dContext = new (CanvasRenderingContext2D as any)(ctx);
					// // @ts-ignore
					(this._2dContext as any)._canvas = this;
					// @ts-ignore
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
					this._layoutNative();
					const opts = Object.assign({ version: 'v1' }, Object.assign(defaultOpts, this._handleContextOptions(type, options)));

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
					this._layoutNative();
					const opts = Object.assign({ version: 'v2' }, Object.assign(defaultOpts, this._handleContextOptions(type, options)));

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
		const nativeView = view.android;
		const width = this.width;
		const height = this.height;
		return {
			bottom: nativeView.getBottom(),
			height: height,
			left: nativeView.getLeft(),
			right: nativeView.getRight(),
			top: nativeView.getTop(),
			width: width,
			x: nativeView.getX(),
			y: nativeView.getY(),
		};
	}

	setPointerCapture() {}

	releasePointerCapture() {}
}
