import { CanvasBase } from './common';
import { DOMMatrix } from '../Canvas2D';
import { CanvasRenderingContext2D } from '../Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from '../WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from '../WebGL2/WebGL2RenderingContext';
import { Utils } from '@nativescript/core';

//declare var TNSCanvas, TNSCanvasListener;

export function createSVGMatrix(): DOMMatrix {
	return new DOMMatrix(TNSCanvas.createSVGMatrix());
}

export class Canvas extends CanvasBase {
	private _2dContext: CanvasRenderingContext2D;
	private _webglContext: WebGLRenderingContext;
	private _webgl2Context: WebGL2RenderingContext;
	private _canvas: any;
	private _didPause: boolean = false;
	private _isReady: boolean = false;
	private _readyListener: any;

	constructor() {
		super();
		this._canvas = TNSCanvas.alloc().initWithFrameUseCpu(CGRectZero, false);
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
				protocols: [TNSCanvasListener],
			}
		);
		this._readyListener = listener.new();
		this._canvas.setListener(this._readyListener);
	}

	get ios() {
		return this._canvas;
	}

	get clientWidth() {
		return this.width;
	}

	get clientHeight() {
		return this.height;
	}

	get width() {
		const measuredWidth = this.getMeasuredWidth();
		if (measuredWidth > 0) {
			return measuredWidth;
		}
		return this._realSize.width;
	}

	set width(value) {
		this.style.width = value;
		if (this._isCustom) {
			this._layoutNative();
		}
	}

	get height() {
		const measuredHeight = this.getMeasuredHeight();
		if (measuredHeight > 0) {
			return measuredHeight;
		}
		return this._realSize.height;
	}

	set height(value) {
		this.style.height = value;
		if (this._isCustom) {
			this._layoutNative();
		}
	}

	private _iosOverflowSafeArea = false;

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
		this.__handleGestures();
	}

	flush() {
		if (this.ios) {
			this.ios.flush();
		}
	}

	onUnloaded() {
		this.ios.pause();
		this._didPause = true;
		super.onUnloaded();
	}

	onLoaded() {
		super.onLoaded();
		if (this._didPause) {
			this.ios.resume();
			this._didPause = false;
		}
	}

	disposeNativeView(): void {
		this.off('touch, pan', this._touchEvents);
		this._canvas.setListener(null);
		this._readyListener = undefined;
		this._canvas = undefined;
		this.setNativeView(undefined);
		super.disposeNativeView();
	}

	_layoutNative() {
		if (!this.parent) {
			if ((typeof this.style.width === 'string' && this.style.width.indexOf('%')) || (typeof this.style.height === 'string' && this.style.height.indexOf('%'))) {
				return;
			}
			if (!this._isCustom) {
				return;
			}

			const size = this._realSize;
			if (!(size.width || 0) && !(size.height || 0)) {
				return;
			}
			const width = Utils.layout.toDeviceIndependentPixels(size.width || 0);
			const height = Utils.layout.toDeviceIndependentPixels(size.height || 0);
			let frameSize = this._canvas.frame.size;

			if (width === frameSize.width && height === frameSize.height) {
				return;
			}

			const frame_origin = this._canvas.frame.origin;
			const frame = CGRectMake(frame_origin.x, frame_origin.y, width, height);
			this._canvas.frame = frame;
			this._canvas.setNeedsLayout();
			this._canvas.layoutIfNeeded();
		}
	}

	getContext(type: string, options?: any): CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | null {
		if (type && type === '2d') {
			if (this._webglContext || this._webgl2Context) {
				return null;
			}
			if (!this._2dContext) {
				this._2dContext = new CanvasRenderingContext2D(this._canvas.getContextContextAttributes(type, this._handleContextOptions(type, options)));
				this._2dContext._canvas = this;
			} else {
				this._canvas.getContextContextAttributes(type, this._handleContextOptions(type, options));
			}
			this._2dContext._type = '2d';
			return this._2dContext;
		} else if (type && (type === 'webgl' || type === 'experimental-webgl')) {
			if (this._2dContext || this._webgl2Context) {
				return null;
			}
			if (!this._webglContext) {
				this._webglContext = new WebGLRenderingContext(this._canvas.getContextContextAttributes('webgl', this._handleContextOptions(type, options)));
				this._webglContext._canvas = this;
			} else {
				this._canvas.getContextContextAttributes('webgl', this._handleContextOptions(type, options));
			}
			this._webglContext._type = 'webgl';
			return this._webglContext;
		} else if (type && (type === 'webgl2' || type === 'experimental-webgl2')) {
			if (this._2dContext || this._webglContext) {
				return null;
			}
			if (!this._webgl2Context) {
				this._webgl2Context = new WebGL2RenderingContext(this._canvas.getContextContextAttributes('webgl2', this._handleContextOptions(type, options)));
				(this._webgl2Context as any)._canvas = this;
			} else {
				this._canvas.getContextContextAttributes('webgl2', this._handleContextOptions(type, options));
			}
			(this._webgl2Context as any)._type = 'webgl';
			return this._webgl2Context;
		}
		return null;
	}

	toDataURL(type = 'image/png', encoderOptions = 0.92) {
		return this._canvas.toDataURL(type, encoderOptions);
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
