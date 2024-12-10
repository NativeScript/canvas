import { createRequire } from 'node:module';
import { Event } from '@nativescript/foundation/dom/dom-utils.js';
import { type YogaNodeLayout } from '@nativescript/foundation/layout/index.js';
import { view } from '@nativescript/foundation/views/decorators/view.js';
import { ViewBase } from '@nativescript/foundation/views/view/view-base.js';

// @ts-ignore
const require = createRequire(import.meta.url);

objc.import('OpenGL');
objc.import('Metal');
objc.import('MetalKit');

const { CanvasRenderingContext2D, WebGLRenderingContext, WebGL2RenderingContext, GPU, GPUCanvasContext } = require('./canvas-napi.darwin-arm64.node');

// import { CanvasRenderingContext2D, WebGLRenderingContext, WebGL2RenderingContext, GPU, GPUCanvasContext } from './index.js';

// const { CanvasRenderingContext2D, WebGLRenderingContext, WebGL2RenderingContext, GPU, GPUCanvasContext } = require('./canvas-napi.darwin-arm64.node');

class NSCMTLView extends NSView {
	static {
		NativeClass(this);
	}
	_queue?: MTLCommandQueue;
	_canvas: WeakRef<Canvas> | null = null;

	get mtlLayer() {
		return this.layer as CAMetalLayer;
	}

	get queue() {
		return this._queue;
	}

	get device() {
		return this.mtlLayer.device;
	}

	makeBackingLayer(): CAMetalLayer {
		return CAMetalLayer.layer();
	}

	initWithFrame(frameRect: CGRect) {
		super.initWithFrame(frameRect);
		this.wantsLayer = true;
		const layer = this.mtlLayer; //CAMetalLayer.layer();
		layer.device = MTLCreateSystemDefaultDevice();
		this._queue = layer.device.newCommandQueue();
		layer.presentsWithTransaction = false;
		layer.framebufferOnly = false;
		layer.pixelFormat = MTLPixelFormat.BGRA8Unorm;
		this.layer = layer;
		return this;
	}

	get drawableSize(): CGSize {
		return this.mtlLayer.drawableSize;
	}

	set drawableSize(value: CGSize) {
		this.mtlLayer.drawableSize = value;
	}

	present() {
		const owner = this._canvas?.deref();
		if (owner) {
			const ctx = owner.getContext('2d') as CanvasRenderingContext2D & { flush: () => void; present: () => void };
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
		this.layer = CAOpenGLLayer.layer();
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

enum Engine {
	None,
	CPU,
	GL,
	GPU,
}

enum Fit {
	None,
	Fill,
	FitX,
	FitY,
	ScaleDown,
}

function getGPU() {
	if (navigator.gpu && navigator.gpu instanceof GPU) {
		return navigator.gpu;
	}
	if (!('__gpu' in globalThis)) {
		Object.defineProperty(globalThis, '__gpu', {
			value: GPU.getInstance(),
			writable: true,
		});
	}
	return globalThis.__gpu;
}

class NSCCanvas extends NSView {
	static {
		NativeClass(this);
	}

	engine = Engine.None;
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

	_fit = Fit.FitX;

	_is2D: boolean = false;

	get is2D() {
		return this._is2D;
	}

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

		this.mtlViewPtr = interop.handleof(this.mtlView);
		this.mtlViewLayerPtr = interop.handleof(this.mtlView.layer);
		this.glViewPtr = interop.handleof(this.glkView);

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

	mtlViewLayerPtr?: interop.Pointer;
	mtlViewPtr?: interop.Pointer;
	glViewPtr?: interop.Pointer;
	gpuContext?: GPUCanvasContext;
	glContext: WebGLRenderingContext | WebGL2RenderingContext | undefined;

	initWebGPUContext() {
		if (this.gpuContext) {
			return;
		}
		if (this.mtlViewLayerPtr) {
			this.gpuContext = GPUCanvasContext.withLayer(getGPU(), this.mtlViewLayerPtr.toNumber(), this.surfaceWidth, this.surfaceHeight);
			if (this.gpuContext) {
				this.engine = Engine.GPU;
			}
			this.mtlView!.isHidden = false;
		}
	}

	resize() {
		if (this.engine == 0) {
			this.scaleSurface();
			return;
		}
		if (!this.is2D && this.engine == Engine.GPU && this.mtlViewLayerPtr) {
			this.gpuContext?.resize(this.mtlViewLayerPtr.toNumber(), this.surfaceWidth, this.surfaceHeight);
			this.scaleSurface();
			return;
		}
		if (this.engine == Engine.GL) {
			this.glkView?.openGLContext?.makeCurrentContext?.();
			this.glkView?.openGLContext?.update?.();
		}
		if (this.is2D) {
			(<any>this._canvas?.deref()?._2dContext)?.resize?.(this.surfaceWidth, this.surfaceHeight);
		}
		this.scaleSurface();
	}

	__isLoaded = false;

	layout(): void {
		super.layout();

		if (!this.__isLoaded && this.surfaceWidth > 0 && this.surfaceHeight > 0) {
			this.__isLoaded = true;
			this.scaleSurface();
			setTimeout(() => {
				const canvas = this._canvas?.deref();
				canvas?.dispatchEvent?.(new CanvasReadyEvent({}));
			}, 0);
		} else {
			this.resize();
		}
	}

	webLikeScale = true;

	scaleSurface() {
		if (!this.webLikeScale) {
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

const defaultGLOptions = {
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

const default2DOptions = {
	alpha: true,
	antialias: true,
	depth: false,
	failIfMajorPerformanceCaveat: false,
	powerPreference: 'default',
	premultipliedAlpha: true,
	preserveDrawingBuffer: false,
	stencil: false,
	desynchronized: false,
	xrCompatible: false,
};

function parsePowerPreference(powerPreference: string) {
	switch (powerPreference) {
		case 'default':
			return 0;
		case 'high-performance':
			return 1;
		case 'low-power':
			return 2;
		default:
			return -1;
	}
}

function handleContextOptions(type: '2d' | 'webgl' | 'webgl2' | 'experimental-webgl' | 'experimental-webgl2', contextAttributes) {
	if (!contextAttributes) {
		if (type === '2d') {
			return { ...default2DOptions, powerPreference: 0 };
		}
		if (type.indexOf('webgl') > -1) {
			return { ...defaultGLOptions, powerPreference: 0 };
		}
	}
	if (type === '2d') {
		if (contextAttributes.alpha !== undefined && typeof contextAttributes.alpha === 'boolean') {
			return { ...contextAttributes, powerPreference: 0 };
		} else {
			return { alpha: true, powerPreference: 0 };
		}
	}
	const glOptions = { ...defaultGLOptions };
	const setIfDefined = (prop: keyof typeof defaultGLOptions, value: unknown) => {
		if (value !== undefined) {
			if (prop === 'powerPreference') {
				// Handle specific conversion logic for 'powerPreference'
				glOptions[prop] = value as (typeof glOptions)[typeof prop];
			} else if (typeof value === typeof glOptions[prop]) {
				glOptions[prop] = value as (typeof glOptions)[typeof prop];
			}
		}
	};
	if (type.indexOf('webgl') > -1) {
		setIfDefined('alpha', contextAttributes.alpha);
		setIfDefined('antialias', contextAttributes.antialias);
		setIfDefined('depth', contextAttributes.depth);
		setIfDefined('failIfMajorPerformanceCaveat', contextAttributes.failIfMajorPerformanceCaveat);
		setIfDefined('powerPreference', parsePowerPreference(contextAttributes.powerPreference ?? 'default'));
		setIfDefined('premultipliedAlpha', contextAttributes.premultipliedAlpha);
		setIfDefined('preserveDrawingBuffer', contextAttributes.preserveDrawingBuffer);
		setIfDefined('stencil', contextAttributes.stencil);
		setIfDefined('desynchronized', contextAttributes.desynchronized);
		return glOptions;
	}
	return null;
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
		this._canvas = NSCCanvas.alloc().initWithFrame(CGRectZero);
		this._canvas.canvas = this;
		this.style.width = '100%';
		this.style.height = 'auto';
		this.nativeView = this._canvas;
	}

	nativeView?: NSCCanvas = undefined;

	initNativeView() {
		return this._canvas;
	}

	applyLayout(parentLayout?: YogaNodeLayout) {
		super.applyLayout(parentLayout);
		if (this.nativeView) {
			this.nativeView.translatesAutoresizingMaskIntoConstraints = true;
		}

		if (parentLayout) {
			this._canvas.frame = CGRectMake(parentLayout.left, parentLayout.top, parentLayout.width, parentLayout.height);
			this._canvas.needsLayout = true;
			this._canvas.layout();
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

	get clientWidth() {
		return this._canvas.frame.size.width;
	}

	get clientHeight() {
		return this._canvas.frame.size.height;
	}

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

	get _gpuContext() {
		return this._canvas?.gpuContext;
	}

	get __native__context() {
		switch (this._contextType) {
			case ContextType.Canvas:
				return this._2dContext;
			case ContextType.WebGL:
				return this._webglContext;
			case ContextType.WebGL2:
				return this._webgl2Context;
			case ContextType.WebGPU:
				return this._gpuContext;
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
			return this._gpuContext?.toDataURL(type ?? 'image/png', encoderOptions ?? 0.92);
		}
		return this.native?.toDataURL?.(type ?? 'image/png', encoderOptions ?? 0.92);
	}

	getContext(contextType: '2d' | 'webgl' | 'experimental-webgl' | 'webgl2' | 'experimental-webgl2' | 'webgpu', options?: Record<string, any>) {
		if (this._canvas === null) {
			return null;
		}
		if (contextType === '2d') {
			if (this._webglContext || this._webgl2Context || this._gpuContext) {
				return null;
			}
			if (this._2dContext) {
				return this._2dContext;
			}

			const opts = {
				...defaultOpts,
				...handleContextOptions(contextType, options),
				fontColor: -16777216,
			};

			const scale = NSScreen.mainScreen.backingScaleFactor;

			if (Canvas.forceGL) {
				const handle = interop.handleof(this._canvas.glkView);

				this._2dContext = CanvasRenderingContext2D.withView(handle.toNumber(), this._canvas.surfaceWidth, this._canvas.surfaceHeight, scale, opts.alpha, opts.fontColor, 90, 1);
				this._canvas.glkView!.isHidden = false;
				this._contextType = ContextType.Canvas;
				this._canvas.engine = Engine.GL;
				this._canvas._is2D = true;
			} else {
				const mtlViewHandle = interop.handleof(this._canvas.mtlView);
				const deviceHandle = interop.handleof(this._canvas.mtlView!.device);
				const queueHandle = interop.handleof(this._canvas.mtlView!.queue);
				this._2dContext = CanvasRenderingContext2D.withMtlViewDeviceQueue(mtlViewHandle.toNumber(), deviceHandle.toNumber(), queueHandle.toNumber(), opts.alpha, scale, 1, opts.fontColor, 90, 1);
				this._canvas.mtlView!.isHidden = false;
				this._contextType = ContextType.Canvas;
				this._canvas.engine = Engine.GPU;
				this._canvas._is2D = true;
			}

			if (this._2dContext) {
				(<any>this._2dContext).canvas = this;
			}

			return this._2dContext;
		} else if (contextType === 'webgl' || contextType === 'experimental-webgl') {
			if (this._2dContext || this._webgl2Context || this._gpuContext) {
				return null;
			}
			if (this._webglContext) {
				return this._webglContext;
			}

			const opts = {
				...defaultOpts,
				...handleContextOptions(contextType, options),
				fontColor: -16777216,
			};

			const handle = interop.handleof(this._canvas.glkView);
			this._webglContext = WebGLRenderingContext.withView(handle.toNumber(), opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible);
			if (this._webglContext) {
				(<any>this._webglContext).canvas = this;
			}

			this._canvas.glContext = this._webglContext;
			this._canvas.glkView!.isHidden = false;
			this._contextType = ContextType.WebGL;
			this._canvas.engine = Engine.GL;
			return this._webglContext;
		} else if (contextType === 'webgl2' || contextType === 'experimental-webgl2') {
			if (this._2dContext || this._webglContext || this._gpuContext) {
				return null;
			}
			if (this._webgl2Context) {
				return this._webgl2Context;
			}
			const opts = {
				...defaultOpts,
				...handleContextOptions(contextType, options),
				fontColor: -16777216,
			};

			const handle = interop.handleof(this._canvas.glkView);
			this._webgl2Context = WebGL2RenderingContext.withView(handle.toNumber(), opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible);
			if (this._webgl2Context) {
				(<any>this._webgl2Context).canvas = this;
			}

			this._canvas.glContext = this._webgl2Context;
			this._canvas.glkView!.isHidden = false;
			this._contextType = ContextType.WebGL2;
			this._canvas.engine = Engine.GL;
			return this._webgl2Context;
		} else if (contextType === 'webgpu') {
			if (this._2dContext || this._webglContext || this._webgl2Context) {
				return null;
			}
			if (this._gpuContext) {
				return this._gpuContext;
			}
			this._canvas.initWebGPUContext();
			this._canvas.engine = Engine.GPU;
		}

		if (this._gpuContext) {
			(<any>this._gpuContext).canvas = this;
		}

		return this._gpuContext;
	}
}

Canvas.register();
