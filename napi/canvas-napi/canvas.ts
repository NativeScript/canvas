import { CanvasRenderingContext2D, WebGLRenderingContext, WebGL2RenderingContext } from './index.js';

import { Event } from '@nativescript/foundation/dom/dom-utils.js';
import type { YogaNodeLayout } from '@nativescript/foundation/layout/index.js';
import { native } from '@nativescript/foundation/views/decorators/native.js';
import { view } from '@nativescript/foundation/views/decorators/view.js';
import { View } from '@nativescript/foundation/views/view/view.js';
import { ViewBase } from '@nativescript/foundation/views/view/view-base.js';

class NSCMTLView extends NSView {
	static {
		NativeClass(this);
	}
	_device?: MTLDevice;
	_queue?: MTLCommandQueue;

	_canvas: WeakRef<Canvas> | null = null;

	get queue() {
		return this._queue;
	}

	get device() {
		return this._device;
	}

	initWithFrame(frameRect: CGRect) {
		super.initWithFrame(frameRect);
		this.wantsLayer = true;
		const layer = CAMetalLayer.layer();
		this._device = MTLCreateSystemDefaultDevice();
		this._queue = this._device.newCommandQueue();
		layer.device = this._device;
		layer.presentsWithTransaction = false;
		layer.framebufferOnly = false;
		layer.pixelFormat = MTLPixelFormat.BGRA8Unorm;

		this.layer = layer;
		return this;
	}

	get drawableSize(): CGSize {
		return (this.layer as CAMetalLayer).drawableSize;
	}

	set drawableSize(value: CGSize) {
		(this.layer as CAMetalLayer).drawableSize = value;
	}

	present() {
		const owner = this._canvas?.deref();
		if (owner) {
			const ctx = owner.getContext('2d') as CanvasRenderingContext2D | null;
			ctx?.flush();
			ctx?.present();
		}
	}

	static ObjCExposedMethods = {
		present: { returns: interop.types.void, params: [] },
	};
}

class NSCGLView extends NSOpenGLView {
	static {
		NativeClass(this);
	}
	isDirty = false;

	_canvas?: WeakRef<Canvas>;

	initWithFrame(frame: CGRect) {
		super.initWithFrame(frame);
		this.wantsLayer = true;
		return this;
	}

	prepareOpenGL() {
		super.prepareOpenGL();
	}

	clearGLContext() {
		super.clearGLContext();
	}
}

enum ContextType {
	None,
	Canvas,
	WebGL,
	WebGL2,
	WebGPU,
}

class NSCCanvas extends NSView {
	static {
		NativeClass(this);
	}

	_canvas: WeakRef<Canvas> | null = null;

	set canvas(value: Canvas) {
		this._canvas = new WeakRef(value);
		this.mtlView!._canvas = this.glkView!._canvas = this._canvas;
	}

	get canvas() {
		return this._canvas as never;
	}

	mtlView?: NSCMTLView;

	glkView?: NSCGLView;

	/**
	 * @type {0 | 1 | 2 | 3 | 4}
	 */
	_fit = 2;

	set fit(value) {
		if (typeof value === 'number') {
			this._fit = value;
			this.scaleSurface();
		}
	}

	get fit() {
		return this._fit;
	}

	initWithFrame(frame: CGRect) {
		super.initWithFrame(frame);
		this.wantsLayer = true;

		const scale = NSScreen.mainScreen.backingScaleFactor;

		const unscaledWidth = Math.floor(300 / scale);
		const unscaledHeight = Math.floor(150 / scale);

		const newFrame = CGRectMake(0, 0, unscaledWidth, unscaledHeight);
		this.mtlView = NSCMTLView.alloc().initWithFrame(newFrame);
		this.glkView = NSCGLView.alloc().initWithFrame(newFrame);
		this.mtlView.drawableSize = CGSizeMake(300, 150);

		this.initializeView();

		return this;
	}

	initializeView() {
		const glkView = this.glkView!;
		const mtlView = this.mtlView!;
		// glkView._canvas = this;
		// mtlView._canvas = this;
		// handler = NSCTouchHandler(canvas: self)
		this.layer.backgroundColor = NSColor.clearColor.CGColor;
		glkView.isHidden = true;
		mtlView.isHidden = true;
		this.addSubview(glkView);
		this.addSubview(mtlView);
		this.scaleSurface();

		this.layer.isOpaque = false;
		(mtlView.layer as CAMetalLayer).isOpaque = false;
	}

	resize() {
		// if(nativeContext == 0){
		//     scaleSurface()
		//     return
		// }
		// if(!is2D && engine == .GPU){
		//     let width = UInt32(surfaceWidth)
		//     let height =  UInt32(surfaceHeight)
		//     CanvasHelpers.resizeWebGPUWithView(nativeContext, self, width, height)
		//     scaleSurface()
		//     return
		// }
		// if(engine == .GL){
		//     EAGLContext.setCurrent(glkView.context)
		// }
		// if(engine == .GL){
		//     glkView.deleteDrawable()
		//     glkView.bindDrawable()
		// }
		// if(is2D){
		//     CanvasHelpers.resize2DContext(nativeContext, Float(surfaceWidth), Float(surfaceHeight))
		// }
		// scaleSurface()
	}

	__isLoaded = false;

	layout(): void {
		super.layout();

		if (!this.__isLoaded && this.surfaceWidth > 0 && this.surfaceHeight > 0) {
			this.__isLoaded = true;
			this.scaleSurface();
			setTimeout(() => {
				const canvas = this._canvas?.deref();
				canvas?.dispatchEvent?.(new CanvasReadyEvent());
			}, 0);
		} else {
			this.resize();
		}
	}

	weblikeScale = true;

	scaleSurface() {
		if (!this.weblikeScale == false) {
			return;
		}
		if (this.surfaceWidth == 0 || this.surfaceHeight == 0) {
			return;
		}

		var density = NSScreen.mainScreen.backingScaleFactor;

		// if (!autoScale) {
		// 	density = 1;
		// }

		let scaledInternalWidth = this.surfaceWidth / density;
		let scaledInternalHeight = this.surfaceHeight / density;

		if (scaledInternalWidth == 0 || scaledInternalHeight == 0) {
			return;
		}

		const frame = this.frame;

		if (frame.size.width == 0 || Number.isNaN(frame.size.width) || frame.size.height == 0 || !Number.isNaN(frame.size.height)) {
			return;
		}

		let scaleX = frame.size.width / scaledInternalWidth;
		let scaleY = frame.size.height / scaledInternalHeight;

		if (scaleX == 0 || Number.isNaN(scaleX) || scaleY == 0 || Number.isNaN(scaleY)) {
			return;
		}

		/**
		 * @type {CATransform3D}
		 */
		var transform = null;

		switch (this.fit) {
			case 0:
				// noop
				transform = CATransform3DIdentity;
				break;
			case 1:
				transform = CATransform3DMakeScale(scaleX, scaleY, 1);
			case 2:
				{
					const dx = (frame.size.width - scaledInternalWidth) / 2;
					const dy = (scaledInternalHeight * scaleX - scaledInternalHeight) / 2;

					transform = CATransform3DMakeScale(scaleX, scaleX, 1);

					transform = CATransform3DConcat(transform, CATransform3DMakeTranslation(dx, dy, 0));
				}
				break;
			case 3:
				{
					const dx = (scaledInternalWidth * scaleY - scaledInternalWidth) / 2;
					const dy = (frame.size.height - scaledInternalHeight) / 2;

					transform = CATransform3DMakeScale(scaleY, scaleY, 1);

					transform = CATransform3DConcat(transform, CATransform3DMakeTranslation(dx, dy, 0));
				}
				break;
			case 4:
				let scale = Math.min(Math.min(scaleX, scaleY), 1);

				transform = CATransform3DMakeScale(scale, scale, 1);
				break;
		}

		if (transform == null) {
			return;
		}
		this.glkView!.layer.transform = transform;
		this.mtlView!.layer.transform = transform;
	}

	forceLayout(width: number, height: number) {
		var unscaledWidth = width;
		var unscaledHeight = height;
		const scale = NSScreen.mainScreen.backingScaleFactor;
		if (unscaledWidth <= 0) {
			unscaledWidth = 1 / scale;
		} else {
			unscaledWidth = unscaledWidth / scale;
		}

		if (unscaledHeight <= 0) {
			unscaledHeight = 1 / scale;
		} else {
			unscaledHeight = unscaledHeight / scale;
		}

		this.glkView!.frame = CGRectMake(0, 0, unscaledWidth, unscaledHeight);
		this.mtlView!.frame = CGRectMake(0, 0, unscaledWidth, unscaledHeight);
		this.mtlView!.drawableSize = CGSizeMake(Math.floor(width), Math.floor(height));
		this.glkView!.needsLayout = true;
		this.mtlView!.needsLayout = true;
		this.glkView!.layout();
		this.mtlView!.layout();
	}

	_surfaceWidth = 300;
	get surfaceWidth() {
		return this._surfaceWidth;
	}

	set surfaceWidth(value: number) {
		if (typeof value === 'number') {
			this._surfaceWidth = value;
			this.forceLayout(value, this.surfaceHeight);
		}
	}

	_surfaceHeight = 150;
	get surfaceHeight() {
		return this._surfaceHeight;
	}

	/**
	 * @param {number} value
	 */
	set surfaceHeight(value) {
		if (typeof value === 'number') {
			this._surfaceHeight = value;
			this.forceLayout(this.surfaceWidth, value);
		}
	}
}

class CanvasReadyEvent extends Event {
	constructor(eventDict?: Record<string, any>) {
		super('ready', eventDict);
	}
}

@view({
	name: 'HTMLCanvasElement',
	tagName: 'canvas',
})
export class Canvas extends ViewBase {
	override get isLeaf() {
		return true;
	}

	_canvas: NSCCanvas;
	constructor() {
		super();
		this._canvas = NSCCanvas.alloc().initWithFrame(CGRectMake(0, 0, 500, 500));
		this._canvas.canvas = this;
		this.style.width = '100%';
		this.style.height = 'auto';
	}

	nativeView?: NSCCanvas = undefined;

	initNativeView() {
		this.nativeView = NSCCanvas.alloc().initWithFrame(CGRectZero);
		return this.nativeView;
	}

	applyLayout(parentLayout?: YogaNodeLayout) {
		super.applyLayout(parentLayout);
		if (this.nativeView) {
			this.nativeView.translatesAutoresizingMaskIntoConstraints = true;
		}
	}

	get lang(): string {
		return NSLocale.currentLocale.languageCode;
	}

	set lang(value) {
		// todo
	}

	_ignoreTouchEvents = false;

	get ignoreTouchEventsProperty() {
		return this._ignoreTouchEvents;
	}

	set ignoreTouchEventsProperty(value) {
		this._ignoreTouchEvents = value;
	}

	static forceGL = false;

	get ios() {
		return this._canvas;
	}

	// get clientWidth() {
	// 	return this.clientHeight;
	// }

	// get clientHeight() {
	// 	return 0;
	// }

	set width(value) {
		this._canvas.surfaceWidth = value;
	}

	get width() {
		return this._canvas.surfaceWidth;
	}

	get height() {
		return this._canvas.surfaceHeight;
	}

	set height(value) {
		this._canvas.surfaceHeight = value;
	}

	_contextType = ContextType.None;

	_2dContext?: CanvasRenderingContext2D;

	_webglContext?: WebGLRenderingContext;

	_webgl2Context?: WebGL2RenderingContext;

	//_gpuContext?: GPUCanvasContext;

	get __native__context() {
		switch (this._contextType) {
			case ContextType.Canvas:
				return this._2dContext;
			case ContextType.WebGL:
				return this._webglContext;
			case ContextType.WebGL2:
				return this._webgl2Context;
			case ContextType.WebGPU:
			//	return this._gpuContext;
			default:
				return null;
		}
	}

	get native() {
		return this.__native__context;
	}

	/**
	 *
	 * @param {string} type
	 * @param {number} encoderOptions
	 * @returns
	 */
	toDataURL(type: string, encoderOptions: number) {
		if (this.width === 0 || this.height === 0) {
			return 'data:,';
		}
		if (!this.native) {
			// todo
			//return this._canvas.toDataURL(type, encoderOptions);
			return 'data:,';
		}
		if (this._contextType === 4) {
			return 'data:,';
			//return this._gpuContext.toDataURL(type ?? 'image/png', encoderOptions ?? 0.92);
		}
		return this.native?.toDataURL?.(type ?? 'image/png', encoderOptions ?? 0.92);
	}

	getContext(contextType: '2d' | 'webgl' | 'experimental-webgl' | 'webgl2' | 'experimental-webgl2' | 'webgpu', options?: Record<string, any>) {
		if (this._canvas === null) {
			return null;
		}
		if (contextType === '2d') {
			if (this._2dContext) {
				return this._2dContext;
			}
			const scale = NSScreen.mainScreen.backingScaleFactor;

			if (Canvas.forceGL) {
				const handle = interop.handleof(this._canvas.glkView);
				this._2dContext = CanvasRenderingContext2D.withView(handle.toNumber(), this._canvas.surfaceWidth, this._canvas.surfaceHeight, scale, options?.alpha ?? true, 0, 90, 1);
				this._canvas.glkView!.isHidden = false;
				this._contextType = ContextType.Canvas;
			} else {
				const mtlViewHandle = interop.handleof(this._canvas.mtlView);
				const deviceHandle = interop.handleof(this._canvas.mtlView!.device);
				const queueHandle = interop.handleof(this._canvas.mtlView!.queue);
				this._2dContext = CanvasRenderingContext2D.withMtlViewDeviceQueue(mtlViewHandle.toNumber(), deviceHandle.toNumber(), queueHandle.toNumber(), options?.alpha ?? true, scale, 1, 0, 90, 1);
				this._canvas.mtlView!.isHidden = false;
				this._contextType = ContextType.Canvas;
			}

			return this._2dContext;
		} else if (contextType === 'webgl' || contextType === 'experimental-webgl') {
			if (this._webglContext) {
				return this._webglContext;
			}

			const handle = interop.handleof(this._canvas.glkView);
			this._webglContext = WebGLRenderingContext.withView(handle.toNumber(), true, false, false, false, 1, true, false, false, false, false);
			this._canvas.glkView!.isHidden = false;
			this._contextType = ContextType.WebGL;
		} else if (contextType === 'webgl2' || contextType === 'experimental-webgl2') {
			if (this._webgl2Context) {
				return this._webgl2Context;
			}

			const handle = interop.handleof(this._canvas.glkView);
			this._webgl2Context = WebGL2RenderingContext.withView(handle.toNumber(), true, false, false, false, 1, true, false, false, false, false);
			this._canvas.glkView!.isHidden = false;
			this._contextType = ContextType.WebGL2;
		}

		return null;
	}
}
