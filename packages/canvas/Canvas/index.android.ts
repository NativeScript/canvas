import { CanvasBase, ignorePixelScalingProperty } from './common';
import { DOMMatrix } from '../Canvas2D';
import { CanvasRenderingContext2D } from '../Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from '../WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from '../WebGL2/WebGL2RenderingContext';
import { Application, View, profile, ImageSource } from '@nativescript/core';
export function createSVGMatrix(): DOMMatrix {
	return new DOMMatrix(org.nativescript.canvas.TNSCanvas.createSVGMatrix());
}

export * from './common';

export class Canvas extends CanvasBase {
	_ready = false;
	private _2dContext: CanvasRenderingContext2D;
	private _webglContext: WebGLRenderingContext;
	private _webgl2Context: WebGL2RenderingContext;
	private _canvas;
	private _didPause: boolean = false;

	constructor(useCpu = false) {
		super();
		if (arguments.length === 1) {
			if (typeof arguments[0] === 'boolean') {
				useCpu = arguments[0];
			}
		}
		const activity = Application.android.foregroundActivity || Application.android.startActivity;
		this._canvas = new org.nativescript.canvas.TNSCanvas(activity, useCpu);
		(global as any).__canvasLoaded = true;
	}

	[ignorePixelScalingProperty.setNative](value: boolean) {
		this._canvas?.setIgnorePixelScaling?.(value);
	}

	[ignorePixelScalingProperty.setNative](value: boolean) {
		this._canvas?.setScaling?.(value);
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
		if (this._isCustom) {
			this._layoutNative();
		}
	}

	static createCustomView() {
		const canvas = new Canvas();
		canvas.width = 300;
		canvas.height = 150;
		canvas._isCustom = true;
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

		if (this.scaling) {
			this._canvas.setScaling(this.scaling);
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
			new org.nativescript.canvas.TNSCanvas.Listener({
				contextReady() {
					const owner = ref.get() as any;
					if (owner && !owner._ready) {
						owner._ready = true;
						owner._readyEvent();
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

	flush() {
		if (this._didPause) {
			return;
		}
		if (this._canvas) {
			this._canvas.flush();
		}
	}

	disposeNativeView(): void {
		this._canvas.setListener(null);
		super.disposeNativeView();
	}

	toDataURL(type = 'png', encoderOptions = 0.92) {
		return this.android.toDataURL(type, encoderOptions);
	}

	public snapshot(): ImageSource | null {
		if (this._canvas) {
			const bm = this._canvas.getImage?.();
			if (bm) {
				return new ImageSource(bm);
			}
		}
		return null;
	}

	_layoutNative() {
		if (!this.parent) {
			if ((typeof this.width === 'string' && this.width.indexOf('%')) || (typeof this.height === 'string' && this.height.indexOf('%'))) {
				return;
			}
			if (!this._isCustom) {
				return;
			}

			const size = this._realSize;
			org.nativescript.canvas.TNSCanvas.layoutView(size.width || 0, size.height || 0, this._canvas);
		}
	}

	getContext(type: string, options?: any): CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | null {
		if (!this._canvas) {
			return null;
		}
		const getNativeOptions = (options) => {
			return JSON.stringify(this._handleContextOptions(type, options));
		};

		if (typeof type === 'string') {
			if (type === '2d') {
				if (this._webglContext || this._webgl2Context) {
					return null;
				}
				if (!this._2dContext) {
					this._2dContext = new CanvasRenderingContext2D(this._canvas.getContext(type, getNativeOptions(options)));
					// @ts-ignore
					this._2dContext._canvas = this;
				} else {
					this._canvas.getContext(type, getNativeOptions(options));
				}
				// @ts-ignore
				this._2dContext._type = '2d';
				return this._2dContext;
			} else if (type === 'webgl' || type === 'experimental-webgl') {
				if (this._2dContext || this._webgl2Context) {
					return null;
				}
				if (!this._webglContext) {
					this._webglContext = new WebGLRenderingContext(this._canvas.getContext('webgl', getNativeOptions(options)));
					this._webglContext._canvas = this;
				} else {
					this._canvas.getContext('webgl', getNativeOptions(options));
				}
				this._webglContext._type = 'webgl';
				return this._webglContext;
			} else if (type && (type === 'webgl2' || type === 'experimental-webgl2')) {
				if (this._2dContext || this._webglContext) {
					return null;
				}
				if (!this._webgl2Context) {
					this._webgl2Context = new WebGL2RenderingContext(this.android.getContext('webgl2', getNativeOptions(options)));
					(this._webgl2Context as any)._canvas = this;
				} else {
					this.android.getContext('webgl2', getNativeOptions(options));
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
