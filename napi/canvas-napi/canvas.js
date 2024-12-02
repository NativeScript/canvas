'use strict';
var __extends =
	(this && this.__extends) ||
	(function () {
		var extendStatics = function (d, b) {
			extendStatics =
				Object.setPrototypeOf ||
				({ __proto__: [] } instanceof Array &&
					function (d, b) {
						d.__proto__ = b;
					}) ||
				function (d, b) {
					for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p];
				};
			return extendStatics(d, b);
		};
		return function (d, b) {
			if (typeof b !== 'function' && b !== null) throw new TypeError('Class extends value ' + String(b) + ' is not a constructor or null');
			extendStatics(d, b);
			function __() {
				this.constructor = d;
			}
			d.prototype = b === null ? Object.create(b) : ((__.prototype = b.prototype), new __());
		};
	})();
var __esDecorate =
	(this && this.__esDecorate) ||
	function (ctor, descriptorIn, decorators, contextIn, initializers, extraInitializers) {
		function accept(f) {
			if (f !== void 0 && typeof f !== 'function') throw new TypeError('Function expected');
			return f;
		}
		var kind = contextIn.kind,
			key = kind === 'getter' ? 'get' : kind === 'setter' ? 'set' : 'value';
		var target = !descriptorIn && ctor ? (contextIn['static'] ? ctor : ctor.prototype) : null;
		var descriptor = descriptorIn || (target ? Object.getOwnPropertyDescriptor(target, contextIn.name) : {});
		var _,
			done = false;
		for (var i = decorators.length - 1; i >= 0; i--) {
			var context = {};
			for (var p in contextIn) context[p] = p === 'access' ? {} : contextIn[p];
			for (var p in contextIn.access) context.access[p] = contextIn.access[p];
			context.addInitializer = function (f) {
				if (done) throw new TypeError('Cannot add initializers after decoration has completed');
				extraInitializers.push(accept(f || null));
			};
			var result = (0, decorators[i])(kind === 'accessor' ? { get: descriptor.get, set: descriptor.set } : descriptor[key], context);
			if (kind === 'accessor') {
				if (result === void 0) continue;
				if (result === null || typeof result !== 'object') throw new TypeError('Object expected');
				if ((_ = accept(result.get))) descriptor.get = _;
				if ((_ = accept(result.set))) descriptor.set = _;
				if ((_ = accept(result.init))) initializers.unshift(_);
			} else if ((_ = accept(result))) {
				if (kind === 'field') initializers.unshift(_);
				else descriptor[key] = _;
			}
		}
		if (target) Object.defineProperty(target, contextIn.name, descriptor);
		done = true;
	};
var __runInitializers =
	(this && this.__runInitializers) ||
	function (thisArg, initializers, value) {
		var useValue = arguments.length > 2;
		for (var i = 0; i < initializers.length; i++) {
			value = useValue ? initializers[i].call(thisArg, value) : initializers[i].call(thisArg);
		}
		return useValue ? value : void 0;
	};
var __setFunctionName =
	(this && this.__setFunctionName) ||
	function (f, name, prefix) {
		if (typeof name === 'symbol') name = name.description ? '['.concat(name.description, ']') : '';
		return Object.defineProperty(f, 'name', { configurable: true, value: prefix ? ''.concat(prefix, ' ', name) : name });
	};
var _a, _b;
Object.defineProperty(exports, '__esModule', { value: true });
var index_1 = require('./index');
var dom_utils_1 = require('@nativescript/foundation/dom/dom-utils');
var view_1 = require('@nativescript/foundation/views/decorators/view');
var view_base_1 = require('@nativescript/foundation/views/view/view-base');
var NSCMTLView = /** @class */ (function (_super) {
	__extends(NSCMTLView, _super);
	function NSCMTLView() {
		var _this = (_super !== null && _super.apply(this, arguments)) || this;
		_this._canvas = null;
		return _this;
	}
	Object.defineProperty(NSCMTLView.prototype, 'queue', {
		get: function () {
			return this._queue;
		},
		enumerable: false,
		configurable: true,
	});
	Object.defineProperty(NSCMTLView.prototype, 'device', {
		get: function () {
			return this._device;
		},
		enumerable: false,
		configurable: true,
	});
	NSCMTLView.prototype.initWithFrame = function (frameRect) {
		_super.prototype.initWithFrame.call(this, frameRect);
		this.wantsLayer = true;
		var layer = CAMetalLayer.layer();
		this._device = MTLCreateSystemDefaultDevice();
		this._queue = this._device.newCommandQueue();
		layer.device = this._device;
		layer.presentsWithTransaction = false;
		layer.framebufferOnly = false;
		layer.pixelFormat = MTLPixelFormat.BGRA8Unorm;
		this.layer = layer;
		return this;
	};
	Object.defineProperty(NSCMTLView.prototype, 'drawableSize', {
		get: function () {
			return this.layer.drawableSize;
		},
		set: function (value) {
			this.layer.drawableSize = value;
		},
		enumerable: false,
		configurable: true,
	});
	NSCMTLView.prototype.present = function () {
		var _d;
		var owner = (_d = this._canvas) === null || _d === void 0 ? void 0 : _d.deref();
		if (owner) {
			var ctx = owner.getContext('2d');
			ctx === null || ctx === void 0 ? void 0 : ctx.flush();
			ctx === null || ctx === void 0 ? void 0 : ctx.present();
		}
	};
	var _c;
	_c = NSCMTLView;
	(function () {
		NativeClass(_c);
	})();
	NSCMTLView.ObjCExposedMethods = {
		present: { returns: interop.types.void, params: [] },
	};
	return NSCMTLView;
})(NSView);
var NSCGLView = /** @class */ (function (_super) {
	__extends(NSCGLView, _super);
	function NSCGLView() {
		var _this = (_super !== null && _super.apply(this, arguments)) || this;
		_this.isDirty = false;
		return _this;
	}
	NSCGLView.prototype.initWithFrame = function (frame) {
		_super.prototype.initWithFrame.call(this, frame);
		this.wantsLayer = true;
		return this;
	};
	NSCGLView.prototype.prepareOpenGL = function () {
		_super.prototype.prepareOpenGL.call(this);
	};
	NSCGLView.prototype.clearGLContext = function () {
		_super.prototype.clearGLContext.call(this);
	};
	return NSCGLView;
})(NSOpenGLView);
_a = NSCGLView;
(function () {
	NativeClass(_a);
})();
// enum ContextType {
// 	None,
// 	Canvas,
// 	WebGL,
// 	WebGL2,
// 	WebGPU,
// }
var NSCCanvas = /** @class */ (function (_super) {
	__extends(NSCCanvas, _super);
	function NSCCanvas() {
		var _this = (_super !== null && _super.apply(this, arguments)) || this;
		_this._canvas = null;
		/**
		 * @type {0 | 1 | 2 | 3 | 4}
		 */
		_this._fit = 2;
		_this.weblikeScale = true;
		_this._surfaceWidth = 300;
		_this._surfaceHeight = 150;
		return _this;
	}
	Object.defineProperty(NSCCanvas.prototype, 'canvas', {
		get: function () {
			return this._canvas;
		},
		set: function (value) {
			this._canvas = new WeakRef(value);
			this.mtlView._canvas = this.glkView._canvas = this._canvas;
		},
		enumerable: false,
		configurable: true,
	});
	Object.defineProperty(NSCCanvas.prototype, 'fit', {
		get: function () {
			return this._fit;
		},
		set: function (value) {
			if (typeof value === 'number') {
				this._fit = value;
				this.scaleSurface();
			}
		},
		enumerable: false,
		configurable: true,
	});
	NSCCanvas.prototype.initWithFrame = function (frame) {
		_super.prototype.initWithFrame.call(this, frame);
		this.wantsLayer = true;
		var scale = NSScreen.mainScreen.backingScaleFactor;
		var unscaledWidth = Math.floor(300 / scale);
		var unscaledHeight = Math.floor(150 / scale);
		var newFrame = CGRectMake(0, 0, unscaledWidth, unscaledHeight);
		this.mtlView = NSCMTLView.alloc().initWithFrame(newFrame);
		this.glkView = NSCGLView.alloc().initWithFrame(newFrame);
		this.mtlView.drawableSize = CGSizeMake(300, 150);
		this.initializeView();
		return this;
	};
	NSCCanvas.prototype.initializeView = function () {
		var glkView = this.glkView;
		var mtlView = this.mtlView;
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
		mtlView.layer.isOpaque = false;
	};
	NSCCanvas.prototype.scaleSurface = function () {
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
		var scaledInternalWidth = this.surfaceWidth / density;
		var scaledInternalHeight = this.surfaceHeight / density;
		if (scaledInternalWidth == 0 || scaledInternalHeight == 0) {
			return;
		}
		var frame = this.frame;
		if (frame.size.width == 0 || Number.isNaN(frame.size.width) || frame.size.height == 0 || !Number.isNaN(frame.size.height)) {
			return;
		}
		var scaleX = frame.size.width / scaledInternalWidth;
		var scaleY = frame.size.height / scaledInternalHeight;
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
					var dx = (frame.size.width - scaledInternalWidth) / 2;
					var dy = (scaledInternalHeight * scaleX - scaledInternalHeight) / 2;
					transform = CATransform3DMakeScale(scaleX, scaleX, 1);
					transform = CATransform3DConcat(transform, CATransform3DMakeTranslation(dx, dy, 0));
				}
				break;
			case 3:
				{
					var dx = (scaledInternalWidth * scaleY - scaledInternalWidth) / 2;
					var dy = (frame.size.height - scaledInternalHeight) / 2;
					transform = CATransform3DMakeScale(scaleY, scaleY, 1);
					transform = CATransform3DConcat(transform, CATransform3DMakeTranslation(dx, dy, 0));
				}
				break;
			case 4:
				var scale = Math.min(Math.min(scaleX, scaleY), 1);
				transform = CATransform3DMakeScale(scale, scale, 1);
				break;
		}
		if (transform == null) {
			return;
		}
		this.glkView.layer.transform = transform;
		this.mtlView.layer.transform = transform;
	};
	NSCCanvas.prototype.forceLayout = function (width, height) {
		var unscaledWidth = width;
		var unscaledHeight = height;
		var scale = NSScreen.mainScreen.backingScaleFactor;
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
	};
	Object.defineProperty(NSCCanvas.prototype, 'surfaceWidth', {
		get: function () {
			return this._surfaceWidth;
		},
		set: function (value) {
			if (typeof value === 'number') {
				this._surfaceWidth = value;
				this.forceLayout(value, this.surfaceHeight);
			}
		},
		enumerable: false,
		configurable: true,
	});
	Object.defineProperty(NSCCanvas.prototype, 'surfaceHeight', {
		get: function () {
			return this._surfaceHeight;
		},
		/**
		 * @param {number} value
		 */
		set: function (value) {
			if (typeof value === 'number') {
				this._surfaceHeight = value;
				this.forceLayout(this.surfaceWidth, value);
			}
		},
		enumerable: false,
		configurable: true,
	});
	return NSCCanvas;
})(NSView);
_b = NSCCanvas;
(function () {
	NativeClass(_b);
})();
var Canvas = /** @class */ (function () {
	function Canvas() {
		/**
		 * @param {boolean} value
		 */
		this._ignoreTouchEvents = false;
		/**
		 * @type {0 | 1 | 2 | 3 | 4}
		 */
		this._contextType = 0;
		this._canvas = NSCCanvas.alloc().initWithFrame(CGRectMake(0, 0, 500, 500));
		this._canvas.canvas = this;
		// this._style.nativeElement = this._canvas;
		// this._style.width = '100%';
		// this._style.height = 'auto';
	}
	Object.defineProperty(Canvas.prototype, 'lang', {
		get: function () {
			return NSLocale.currentLocale.languageCode;
		},
		/**
		 * @param {string} value
		 */
		set: function (value) {
			// todo
		},
		enumerable: false,
		configurable: true,
	});
	Object.defineProperty(Canvas.prototype, 'ignoreTouchEventsProperty', {
		/**
		 * @returns {boolean}
		 */
		get: function () {
			return this._ignoreTouchEvents;
		},
		set: function (value) {
			this._ignoreTouchEvents = value;
		},
		enumerable: false,
		configurable: true,
	});
	Object.defineProperty(Canvas.prototype, 'ios', {
		get: function () {
			return this._canvas;
		},
		enumerable: false,
		configurable: true,
	});
	Object.defineProperty(Canvas.prototype, 'nativeView', {
		get: function () {
			return this._canvas;
		},
		enumerable: false,
		configurable: true,
	});
	Object.defineProperty(Canvas.prototype, 'clientWidth', {
		get: function () {
			return 0;
		},
		enumerable: false,
		configurable: true,
	});
	Object.defineProperty(Canvas.prototype, 'clientHeight', {
		get: function () {
			return 0;
		},
		enumerable: false,
		configurable: true,
	});
	Object.defineProperty(Canvas.prototype, 'width', {
		get: function () {
			return this._canvas.surfaceWidth;
		},
		set: function (value) {
			this._canvas.surfaceWidth = value;
		},
		enumerable: false,
		configurable: true,
	});
	Object.defineProperty(Canvas.prototype, 'height', {
		get: function () {
			return this._canvas.surfaceHeight;
		},
		set: function (value) {
			this._canvas.surfaceHeight = value;
		},
		enumerable: false,
		configurable: true,
	});
	Object.defineProperty(Canvas.prototype, '__native__context', {
		//_gpuContext?: GPUCanvasContext;
		get: function () {
			switch (this._contextType) {
				case 1:
					return this._2dContext;
				case 2:
					return this._webglContext;
				case 3:
					return this._webgl2Context;
				case 4:
				//	return this._gpuContext;
				default:
					return null;
			}
		},
		enumerable: false,
		configurable: true,
	});
	Object.defineProperty(Canvas.prototype, 'native', {
		get: function () {
			return this.__native__context;
		},
		enumerable: false,
		configurable: true,
	});
	/**
	 *
	 * @param {string} type
	 * @param {number} encoderOptions
	 * @returns
	 */
	Canvas.prototype.toDataURL = function (type, encoderOptions) {
		var _d, _e;
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
		return (_e = (_d = this.native) === null || _d === void 0 ? void 0 : _d.toDataURL) === null || _e === void 0 ? void 0 : _e.call(_d, type !== null && type !== void 0 ? type : 'image/png', encoderOptions !== null && encoderOptions !== void 0 ? encoderOptions : 0.92);
	};
	Canvas.prototype.getContext = function (contextType, options) {
		var _d, _e;
		if (this._canvas === null) {
			return null;
		}
		if (contextType === '2d') {
			if (this._2dContext) {
				return this._2dContext;
			}
			var scale = NSScreen.mainScreen.backingScaleFactor;
			if (Canvas.forceGL) {
				var handle = interop.handleof(this._canvas.glkView);
				this._2dContext = index_1.CanvasRenderingContext2D.withView(handle.toNumber(), this._canvas.surfaceWidth, this._canvas.surfaceHeight, scale, (_d = options === null || options === void 0 ? void 0 : options.alpha) !== null && _d !== void 0 ? _d : true, 0, 90, 1);
				this._canvas.glkView.isHidden = false;
				this._contextType = 1;
			} else {
				var mtlViewHandle = interop.handleof(this._canvas.mtlView);
				var deviceHandle = interop.handleof(this._canvas.mtlView.device);
				var queueHandle = interop.handleof(this._canvas.mtlView.queue);
				this._2dContext = index_1.CanvasRenderingContext2D.withMtlViewDeviceQueue(mtlViewHandle.toNumber(), deviceHandle.toNumber(), queueHandle.toNumber(), (_e = options === null || options === void 0 ? void 0 : options.alpha) !== null && _e !== void 0 ? _e : true, scale, 1, 0, 90, 1);
				this._canvas.mtlView.isHidden = false;
				this._contextType = 1;
			}
			return this._2dContext;
		} else if (contextType === 'webgl' || contextType === 'experimental-webgl') {
			if (this._webglContext) {
				return this._webglContext;
			}
			var handle = interop.handleof(this._canvas.glkView);
			this._webglContext = index_1.WebGLRenderingContext.withView(handle.toNumber(), true, false, false, false, 1, true, false, false, false, false);
			this._canvas.glkView.isHidden = false;
			this._contextType = 2;
		} else if (contextType === 'webgl2' || contextType === 'experimental-webgl2') {
			if (this._webgl2Context) {
				return this._webgl2Context;
			}
			var handle = interop.handleof(this._canvas.glkView);
			this._webgl2Context = index_1.WebGL2RenderingContext.withView(handle.toNumber(), true, false, false, false, 1, true, false, false, false, false);
			this._canvas.glkView.isHidden = false;
			this._contextType = 2;
		}
		return null;
	};
	/**
	 * @type {boolean}
	 */
	Canvas.forceGL = false;
	return Canvas;
})();
module.exports.Canvas = Canvas;
var CanvasReadyEvent = /** @class */ (function (_super) {
	__extends(CanvasReadyEvent, _super);
	function CanvasReadyEvent(eventDict) {
		return _super.call(this, 'ready', eventDict) || this;
	}
	return CanvasReadyEvent;
})(dom_utils_1.Event);
var CanvasView = (function () {
	var _classDecorators = [
		(0, view_1.view)({
			name: 'HTMLCanvasElement',
			tagName: 'canvas',
		}),
	];
	var _classDescriptor;
	var _classExtraInitializers = [];
	var _classThis;
	var _classSuper = view_base_1.ViewBase;
	var CanvasView = (_classThis = /** @class */ (function (_super) {
		__extends(CanvasView_1, _super);
		function CanvasView_1() {
			var _this = (_super !== null && _super.apply(this, arguments)) || this;
			_this.nativeView = undefined;
			return _this;
		}
		Object.defineProperty(CanvasView_1.prototype, 'isLeaf', {
			get: function () {
				return true;
			},
			enumerable: false,
			configurable: true,
		});
		CanvasView_1.prototype.initNativeView = function () {
			this.nativeView = NSCCanvas.alloc().initWithFrame(CGRectZero);
			return this.nativeView;
		};
		/**
		 *
		 * @param {YogaNodeLayout} parentLayout
		 */
		CanvasView_1.prototype.applyLayout = function (parentLayout) {
			_super.prototype.applyLayout.call(this, parentLayout);
			if (this.nativeView) {
				this.nativeView.translatesAutoresizingMaskIntoConstraints = true;
			}
		};
		return CanvasView_1;
	})(_classSuper));
	__setFunctionName(_classThis, 'CanvasView');
	(function () {
		var _d;
		var _metadata = typeof Symbol === 'function' && Symbol.metadata ? Object.create((_d = _classSuper[Symbol.metadata]) !== null && _d !== void 0 ? _d : null) : void 0;
		__esDecorate(null, (_classDescriptor = { value: _classThis }), _classDecorators, { kind: 'class', name: _classThis.name, metadata: _metadata }, null, _classExtraInitializers);
		CanvasView = _classThis = _classDescriptor.value;
		if (_metadata) Object.defineProperty(_classThis, Symbol.metadata, { enumerable: true, configurable: true, writable: true, value: _metadata });
		__runInitializers(_classThis, _classExtraInitializers);
	})();
	return (CanvasView = _classThis);
})();
module.exports.CanvasView = CanvasView;
