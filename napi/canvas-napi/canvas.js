const { CanvasRenderingContext2D, WebGLRenderingContext, WebGL2RenderingContext } = require('./index.js');

class NSCMTLView extends NSView {
	static {
		NativeClass(this);
	}
	_device;
	_queue;
	/**
	 * @param {WeakRef<Canvas>} canvas
	 */
	_canvas = null;

	get queue() {
		return this._queue;
	}

	get device() {
		return this._device;
	}

	initWithFrame(frameRect) {
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

	/**
	 * @return {CGSize}
	 */
	get drawableSize() {
		return this.layer.drawableSize;
	}

	/**
	 * @param {CGSize} value
	 */
	set drawableSize(value) {
		this.layer.drawableSize = value;
	}

	present() {
		const owner = this._canvas.deref();
		if (owner) {
			const ctx = owner.getContext('2d');
			ctx.flush();
			ctx.present();
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
	/**
	 * @param {WeakRef<Canvas>} canvas
	 */
	_canvas = null;

	initWithFrame(frame) {
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

// enum ContextType {
// 	None,
// 	Canvas,
// 	WebGL,
// 	WebGL2,
// 	WebGPU,
// }

class NSCCanvas extends NSView {
	static {
		NativeClass(this);
	}

	/**
	 * @param {WeakRef<Canvas>} canvas
	 */
	_canvas = null;

	/**
	 * @param {WeakRef<Canvas>} value
	 */
	set canvas(value) {
		this._canvas = new WeakRef(value);
		this.mtlView._canvas = this.glkView._canvas = this._canvas;
	}

	get canvas() {
		return this._canvas;
	}

	/**
	 * @type {NSCMTLView}
	 */
	mtlView;

	/**
	 * @type {NSCGLView}
	 */
	glkView;

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

	initWithFrame(frame) {
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
		this.glkView._canvas = this;
		this.mtlView._canvas = this;
		// handler = NSCTouchHandler(canvas: self)
		this.layer.backgroundColor = NSColor.clearColor.CGColor;
		this.glkView.isHidden = true;
		this.mtlView.isHidden = true;
		this.addSubview(this.glkView);
		this.addSubview(this.mtlView);
		this.scaleSurface();

		this.isOpaque = false;
		this.mtlView.isOpaque = false;
	}

	/**
	 *  @type {boolean}
	 */
	weblikeScale = true;

	scaleSurface() {
		if (!this.weblikeScale == false) {
			return;
		}
		if (this.surfaceWidth == 0 || this.surfaceHeight == 0) {
			return;
		}

		var density = NSScreen.mainScreen.backingScaleFactor;

		if (!autoScale) {
			density = 1;
		}

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
				let scale = min(min(scaleX, scaleY), 1);

				transform = CATransform3DMakeScale(scale, scale, 1);
				break;
		}

		if (transform == null) {
			return;
		}
		this.glkView.layer.transform = transform;
		this.mtlView.layer.transform = transform;
	}

	/**
	 *
	 * @param {number} width
	 * @param {number} height
	 */
	forceLayout(width, height) {
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

		this.glkView.frame = CGRectMake(0, 0, unscaledWidth, unscaledHeight);
		this.mtlView.frame = CGRectMake(0, 0, unscaledWidth, unscaledHeight);
		this.mtlView.drawableSize = CGSizeMake(Math.floor(width), Math.floor(height));
		this.glkView.needsLayout = true;
		this.mtlView.needsLayout = true;
		this.glkView.layoutSubtreeIfNeeded();
		this.mtlView.layoutSubtreeIfNeeded();
	}

	_surfaceWidth = 300;
	get surfaceWidth() {
		return this._surfaceWidth;
	}

	/**
	 * @param {number} value
	 */
	set surfaceWidth(value) {
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

class Style {
	/**
	 *  @type {Map<string, any>}
	 */
	_values;

	/**
	 *  @type {NSView}
	 */
	nativeElement;

	constructor() {
		this._values = new Map();
	}

	get width() {
		return this._values.get('width');
	}
	set width(value) {
		this._values.set('width', value);
	}

	get height() {
		return this._values.get('height');
	}
	set height(value) {
		this._values.set('height', value);
	}
}

class Canvas {
	/**
	 * @type {NSCCanvas}
	 */
	_canvas = null;
	constructor() {
		this._canvas = NSCCanvas.alloc().initWithFrame(CGRectMake(0, 0, 500, 500));
		this._canvas.canvas = this;
		this._style.nativeElement = this._canvas;
		this._style.width = '100%';
		this._style.height = 'auto';
	}

	get lang() {
		return NSLocale.currentLocale.languageCode;
	}

	/**
	 * @param {string} value
	 */
	set lang(value) {
		// todo
	}

	/**
	 * @param {boolean} value
	 */
	_ignoreTouchEvents = false;

	/**
	 * @returns {boolean}
	 */
	get ignoreTouchEventsProperty() {
		return this._ignoreTouchEvents;
	}

	set ignoreTouchEventsProperty(value) {
		this._ignoreTouchEvents = value;
	}

	/**
	 * @type {boolean}
	 */
	static forceGL = false;

	get ios() {
		return this._canvas;
	}

	get nativeView() {
		return this._canvas;
	}

	get clientWidth() {
		return 0;
	}

	get clientHeight() {
		return 0;
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

	/**
	 * @type {0 | 1 | 2 | 3 | 4}
	 */
	_contextType = 0;

	/**
	 * @type {CanvasRenderingContext2D}
	 */

	_2dContext;
	/**
	 *  @type {WebGLRenderingContext}
	 */
	_webglContext;

	/**
	 * @type {WebGL2RenderingContext}
	 */
	_webgl2Context;

	// _gpuContext: GPUCanvasContext;

	addEventListener(type, listener, options) {}

	removeEventListener(type, listener, options) {}

	_style = new Style();
	get style() {
		return this._style;
	}

	get __native__context() {
		switch (this._contextType) {
			case 1:
				return this._2dContext;
			case 2:
				return this._webglContext;
			case 3:
				return this._webgl2Context;
			case 4:
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
	toDataURL(type, encoderOptions) {
		if (this.width === 0 || this.height === 0) {
			return 'data:,';
		}
		if (!this.native) {
			// todo
			//return this._canvas.toDataURL(type, encoderOptions);
			return 'data:,';
		}
		if (this._contextType === 4) {
			return this._gpuContext.toDataURL(type ?? 'image/png', encoderOptions ?? 0.92);
		}
		return this.native?.toDataURL?.(type ?? 'image/png', encoderOptions ?? 0.92);
	}

	/**
	 * @param {("2d" | "webgl" | "experimental-webgl" | "webgl2" |"experimental-webgl2" | "webgpu")} contextType
	 */
	getContext(contextType, options) {
		if (contextType === '2d') {
			if (this._2dContext) {
				return this._2dContext;
			}
			const scale = NSScreen.mainScreen.backingScaleFactor;

			if (Canvas.forceGL) {
				const handle = interop.handleof(this._canvas.glkView);
				this._2dContext = CanvasRenderingContext2D.withView(handle.toNumber(), this._canvas.surfaceWidth, this._canvas.surfaceHeight, scale, options?.alpha ?? true, 0, 90, 1);
				this._canvas.glkView.isHidden = false;
				this._contextType = 1;
			} else {
				const mtlViewHandle = interop.handleof(this._canvas.mtlView);
				const deviceHandle = interop.handleof(this._canvas.mtlView.device);
				const queueHandle = interop.handleof(this._canvas.mtlView.queue);
				this._2dContext = CanvasRenderingContext2D.withMtlViewDeviceQueue(mtlViewHandle.toNumber(), deviceHandle.toNumber(), queueHandle.toNumber(), options?.alpha ?? true, scale, 1, 0, 90, 1);
				this._canvas.mtlView.isHidden = false;
				this._contextType = 1;
			}

			return this._2dContext;
		} else if (contextType === 'webgl' || contextType === 'experimental-webgl') {
			if (this._webglContext) {
				return this._webglContext;
			}

			const handle = interop.handleof(this._canvas.glkView);
			this._webglContext = WebGLRenderingContext.withView(handle.toNumber(), 2, true, false, false, false, 1, true, false, false, false, false, false);
			this._canvas.glkView.isHidden = false;
			this._contextType = 2;
		} else if (contextType === 'webgl2' || contextType === 'experimental-webgl2') {
			if (this._webgl2Context) {
				return this._webgl2Context;
			}

			const handle = interop.handleof(this._canvas.glkView);
			this._webgl2Context = WebGL2RenderingContext.withView(handle.toNumber(), 2, true, false, false, false, 1, true, false, false, false, false, false);
			this._canvas.glkView.isHidden = false;
			this._contextType = 2;
		}

		return null;
	}
}

module.exports.Canvas = Canvas;
