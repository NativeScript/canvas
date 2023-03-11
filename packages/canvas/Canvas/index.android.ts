import { CanvasBase, ignorePixelScalingProperty } from './common';
import { DOMMatrix } from '../Canvas2D';
import { CanvasRenderingContext2D } from '../Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from '../WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from '../WebGL2/WebGL2RenderingContext';
import { Application, View, profile, Device, Screen, knownFolders } from '@nativescript/core';
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
export class Canvas extends CanvasBase {
	_ready = false;
	private _2dContext: CanvasRenderingContext2D;
	private _webglContext: WebGLRenderingContext;
	private _webgl2Context: WebGL2RenderingContext;
	private _canvas;
	private _didPause: boolean = false;

	constructor() {
		super();
		const activity = Application.android.foregroundActivity || Application.android.startActivity;
		this._canvas = new org.nativescript.canvas.NSCCanvas(activity);
		(global as any).__canvasLoaded = true;
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

	// @ts-ignore
	get width() {
		if (this.getMeasuredWidth() > 0) {
			return this.getMeasuredWidth();
		}
		const width = this._canvas.getWidth();
		if (width === 0) {
			let rootParams = this._canvas.getLayoutParams();
			if (rootParams) {
				return rootParams.width;
			}
		}
		return width;
	}

	set width(value) {
		this.style.width = value;
		this._didLayout = false;
		if (this._isCustom) {
			this._layoutNative();
		}
	}

	// @ts-ignore
	get height() {
		if (this.getMeasuredHeight() > 0) {
			return this.getMeasuredHeight();
		}
		const height = this._canvas.getHeight();
		if (height === 0) {
			let rootParams = this._canvas.getLayoutParams();
			if (rootParams) {
				return rootParams.height;
			}
		}
		return height;
	}

	set height(value) {
		this.style.height = value;
		this._didLayout = false;
		if (this._isCustom) {
			this._layoutNative();
		}
	}

	static createCustomView() {
		const canvas = new Canvas();
		canvas._isCustom = true;
		canvas.width = 300;
		canvas.height = 150;
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
			console.log(this.nativeView.getWidth(), this.nativeView.getHeight());
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
					console.log('surfaceResize', width, height);
					// if(this._webglContext || this._webgl2Context){
					// 	(this._webglContext || this._webgl2Context)?.resize();
					// }
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

	toDataURL(type = 'png', encoderOptions = 0.92) {
		if (this._2dContext) {
			return (this._2dContext as any).__toDataURL(type, encoderOptions);
		}
		return (this._webglContext || (this._webgl2Context as any)).__toDataURL(type, encoderOptions);
	}

	_layoutNative() {
		if (this._didLayout) {
			return;
		}
		if (!this.parent) {
			if ((typeof this.width === 'string' && this.width.indexOf('%')) || (typeof this.height === 'string' && this.height.indexOf('%'))) {
				return;
			}
			if (!this._isCustom) {
				return;
			}
			const size = this._realSize;
			org.nativescript.canvas.NSCCanvas.layoutView(size.width || 0, size.height || 0, this._canvas);

			if (this._2dContext) {
				(this._2dContext as any).native.__resize(size.width, size.height);
			}
			this._didLayout = true;
		}
	}

	get __native__context() {
		return this._2dContext?.native ?? this._webglContext?.native ?? this._webgl2Context?.native;
	}

	get native() {
		return this.__native__context;
	}

	_didLayout = false;

	getContext(type: string, options?: any): CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | null {
		if (!this._canvas) {
			return null;
		}
		// let direction = 0;
		// if (androidx.core.text.TextUtilsCompat.getLayoutDirectionFromLocale(java.util.Locale.getDefault()) === androidx.core.view.ViewCompat.LAYOUT_DIRECTION_RTL) {
		// 	direction = 1;
		// }

		if (typeof type === 'string') {
			if (type === '2d') {
				if (this._webglContext || this._webgl2Context) {
					return null;
				}

				if (!this._2dContext) {
					this._didLayout = false;
					this._layoutNative();

					const opts = Object.assign(defaultOpts, this._handleContextOptions(type, options));

					// this.parent?.style?.color?.android || -16777216
					this._canvas.initContext(type, opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.desynchronized, opts.xrCompatible);

					this._2dContext = new (CanvasRenderingContext2D as any)(this._canvas, opts);

					// // @ts-ignore
					(this._2dContext as any)._canvas = this;
				}

				// @ts-ignore
				this._2dContext._type = '2d';
				return this._2dContext;
			} else if (type === 'webgl' || type === 'experimental-webgl') {
				if (this._2dContext || this._webgl2Context) {
					return null;
				}
				if (!this._webglContext) {
					const opts = Object.assign({ version: 'v1' }, Object.assign(defaultOpts, this._handleContextOptions(type, options)));

					this._canvas.initContext(type, opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.desynchronized, opts.xrCompatible);

					this._webglContext = new (WebGLRenderingContext as any)(this._canvas, opts);
					(this._webglContext as any)._canvas = this;
				}

				this._webglContext._type = 'webgl';
				return this._webglContext;
			} else if (type && (type === 'webgl2' || type === 'experimental-webgl2')) {
				if (this._2dContext || this._webglContext) {
					return null;
				}
				if (!this._webgl2Context) {
					const opts = Object.assign({ version: 'v2' }, Object.assign(defaultOpts, this._handleContextOptions(type, options)));

					this._canvas.initContext(type, opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.desynchronized, opts.xrCompatible);

					this._webgl2Context = new (WebGL2RenderingContext as any)(this._canvas, opts);
					(this._webgl2Context as any)._canvas = this;
				}
				(this._webgl2Context as any)._type = 'webgl2';
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
