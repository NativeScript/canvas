declare module org {
	export module nativescript {
		export module canvas {
			export class CanvasFit {
				public static class: java.lang.Class<org.nativescript.canvas.CanvasFit>;
				public static None: org.nativescript.canvas.CanvasFit;
				public static Fill: org.nativescript.canvas.CanvasFit;
				public static FitX: org.nativescript.canvas.CanvasFit;
				public static FitY: org.nativescript.canvas.CanvasFit;
				public static ScaleDown: org.nativescript.canvas.CanvasFit;
				public static values(): androidNative.Array<org.nativescript.canvas.CanvasFit>;
				public static getEntries(): any;
				public getValue(): number;
				public static valueOf(value: string): org.nativescript.canvas.CanvasFit;
			}
			export module CanvasFit {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.CanvasFit.Companion>;
					public fromNative(value: number): org.nativescript.canvas.CanvasFit;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class GC {
				public static class: java.lang.Class<org.nativescript.canvas.GC>;
				public static getInstance(): org.nativescript.canvas.GC;
				public restartWatch(): void;
				public static watchObject(key: number, value: java.nio.ByteBuffer): void;
				public watchItem(value: org.nativescript.canvas.GC.Object): void;
				public static getBufferRefQue(): java.lang.ref.ReferenceQueue<org.nativescript.canvas.GC.Object>;
				public constructor();
			}
			export module GC {
				export class BufferWrapper extends org.nativescript.canvas.GC.Object {
					public static class: java.lang.Class<org.nativescript.canvas.GC.BufferWrapper>;
					public getValue(): any;
					public constructor(nativePtr: number, value: any);
					public dispose(): void;
				}
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.GC.Companion>;
					public watchObject(key: number, value: java.nio.ByteBuffer): void;
					public getInstance(): org.nativescript.canvas.GC;
					public getBufferRefQue(): java.lang.ref.ReferenceQueue<org.nativescript.canvas.GC.Object>;
				}
				export class Object {
					public static class: java.lang.Class<org.nativescript.canvas.GC.Object>;
					/**
					 * Constructs a new instance of the org.nativescript.canvas.GC$Object interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: { dispose(): void });
					public constructor();
					public dispose(): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class GLView {
				public static class: java.lang.Class<org.nativescript.canvas.GLView>;
				public setReady$canvas_release(value: boolean): void;
				public constructor(context: globalAndroid.content.Context);
				public setSurface$canvas_release(value: globalAndroid.view.Surface): void;
				public setCreated$canvas_release(value: boolean): void;
				public getCanvas$canvas_release(): org.nativescript.canvas.NSCCanvas;
				public isCreatedWithZeroSized$canvas_release(): boolean;
				public isReady$canvas_release(): boolean;
				public onSurfaceTextureUpdated(surface: globalAndroid.graphics.SurfaceTexture): void;
				public init(): void;
				public onSurfaceTextureSizeChanged(it: globalAndroid.graphics.SurfaceTexture, this_: number, surface: number): void;
				public onSurfaceTextureAvailable(it: globalAndroid.graphics.SurfaceTexture, this_: number, surface: number): void;
				public constructor(context: globalAndroid.content.Context, attrs: globalAndroid.util.AttributeSet);
				public onSurfaceTextureDestroyed(surface: globalAndroid.graphics.SurfaceTexture): boolean;
				public getSurface$canvas_release(): globalAndroid.view.Surface;
				public isCreated$canvas_release(): boolean;
				public setCreatedWithZeroSized$canvas_release(value: boolean): void;
				public setCanvas$canvas_release(value: org.nativescript.canvas.NSCCanvas): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class GLViewSV {
				public static class: java.lang.Class<org.nativescript.canvas.GLViewSV>;
				public setReady$canvas_release(value: boolean): void;
				public constructor(context: globalAndroid.content.Context, attrs: globalAndroid.util.AttributeSet);
				public constructor(context: globalAndroid.content.Context);
				public getCanvas$canvas_release(): org.nativescript.canvas.NSCCanvas;
				public isReady$canvas_release(): boolean;
				public surfaceCreated(holder: globalAndroid.view.SurfaceHolder): void;
				public surfaceChanged(it: globalAndroid.view.SurfaceHolder, this_: number, holder: number, format: number): void;
				public surfaceDestroyed(holder: globalAndroid.view.SurfaceHolder): void;
				public setCanvas$canvas_release(value: org.nativescript.canvas.NSCCanvas): void;
				public init(): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class Helpers {
				public static class: java.lang.Class<org.nativescript.canvas.Helpers>;
				public static INSTANCE: org.nativescript.canvas.Helpers;
				public getBitmap(it: globalAndroid.graphics.drawable.Drawable): globalAndroid.graphics.Bitmap;
				public getEmptyFloat(): androidNative.Array<number>;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class ImageBitmapColorSpaceConversion {
				public static class: java.lang.Class<org.nativescript.canvas.ImageBitmapColorSpaceConversion>;
				public static Default: org.nativescript.canvas.ImageBitmapColorSpaceConversion;
				public static None: org.nativescript.canvas.ImageBitmapColorSpaceConversion;
				public static getEntries(): any;
				public static valueOf(value: string): org.nativescript.canvas.ImageBitmapColorSpaceConversion;
				public static values(): androidNative.Array<org.nativescript.canvas.ImageBitmapColorSpaceConversion>;
				public getValue(): number;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class ImageBitmapImageOrientation {
				public static class: java.lang.Class<org.nativescript.canvas.ImageBitmapImageOrientation>;
				public static FromImage: org.nativescript.canvas.ImageBitmapImageOrientation;
				public static FlipY: org.nativescript.canvas.ImageBitmapImageOrientation;
				public static None: org.nativescript.canvas.ImageBitmapImageOrientation;
				public static valueOf(value: string): org.nativescript.canvas.ImageBitmapImageOrientation;
				public getValue(): number;
				public static values(): androidNative.Array<org.nativescript.canvas.ImageBitmapImageOrientation>;
				public static getEntries(): any;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class ImageBitmapPremultiplyAlpha {
				public static class: java.lang.Class<org.nativescript.canvas.ImageBitmapPremultiplyAlpha>;
				public static Default: org.nativescript.canvas.ImageBitmapPremultiplyAlpha;
				public static PremultiplyAlpha: org.nativescript.canvas.ImageBitmapPremultiplyAlpha;
				public static None: org.nativescript.canvas.ImageBitmapPremultiplyAlpha;
				public static valueOf(value: string): org.nativescript.canvas.ImageBitmapPremultiplyAlpha;
				public static values(): androidNative.Array<org.nativescript.canvas.ImageBitmapPremultiplyAlpha>;
				public static getEntries(): any;
				public getValue(): number;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class ImageBitmapResizeQuality {
				public static class: java.lang.Class<org.nativescript.canvas.ImageBitmapResizeQuality>;
				public static Low: org.nativescript.canvas.ImageBitmapResizeQuality;
				public static Medium: org.nativescript.canvas.ImageBitmapResizeQuality;
				public static High: org.nativescript.canvas.ImageBitmapResizeQuality;
				public static Pixelated: org.nativescript.canvas.ImageBitmapResizeQuality;
				public static valueOf(value: string): org.nativescript.canvas.ImageBitmapResizeQuality;
				public static values(): androidNative.Array<org.nativescript.canvas.ImageBitmapResizeQuality>;
				public static getEntries(): any;
				public getValue(): number;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class NSCCanvas {
				public static class: java.lang.Class<org.nativescript.canvas.NSCCanvas>;
				public textureView: org.nativescript.canvas.GLView;
				public surfaceView: org.nativescript.canvas.GLViewSV;
				public static TAG: string = 'CanvasView';
				public create2DContext(alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number, premultipliedAlpha: boolean, preserveDrawingBuffer: boolean, stencil: boolean, desynchronized: boolean, xrCompatible: boolean): number;
				public makeContextCurrent(): void;
				public setSurfaceView(value: org.nativescript.canvas.GLViewSV): void;
				public static context2DTest(context: number): void;
				//public initContextWithContextAttributes($i$a$-let-NSCCanvas$initContextWithContextAttributes$1: string, it: boolean, density: boolean, width: boolean, height: boolean, density: number, $i$a$-run-NSCCanvas$initContextWithContextAttributes$2: boolean, $this$initContextWithContextAttributes_u24lambda_u244: boolean, version: boolean, surface: boolean, this_: boolean): void;
				public static nativeUpdateWebGLSurface(param0: globalAndroid.view.Surface, param1: number): void;
				public static setStore(value: java.util.concurrent.ConcurrentHashMap<any, any>): void;
				public getTextureView(): org.nativescript.canvas.GLView;
				public isAttachedToWindow$canvas_release(): boolean;
				public getTouchEventListener(): org.nativescript.canvas.NSCCanvas.TouchEvents;
				public static getStore(): java.util.concurrent.ConcurrentHashMap<any, any>;
				public setSurfaceWidth(value: number): void;
				public static nativeInitWebGPU(param0: number, param1: globalAndroid.view.Surface, param2: number, param3: number): number;
				public snapshot(): globalAndroid.graphics.Bitmap;
				public isPaused$canvas_release(): boolean;
				public static getEnableDebug(): boolean;
				public static getBoundingClientRectJSON(canvas: org.nativescript.canvas.NSCCanvas): string;
				public getFit(): org.nativescript.canvas.CanvasFit;
				public constructor(context: globalAndroid.content.Context, type: org.nativescript.canvas.NSCCanvas.SurfaceType);
				public static removeBuffer(key: string): void;
				public getBoundsBuffer(): java.nio.FloatBuffer;
				public setListener(value: org.nativescript.canvas.NSCCanvas.Listener): void;
				public static nativeInitContextWithCustomSurface(param0: number, param1: number, param2: number, param3: boolean, param4: number, param5: number, param6: number): number;
				public setSurfaceHeight(value: number): void;
				public static nativeContext2DRender(param0: number): void;
				public static nativeUpdateWebGLNoSurface(param0: number, param1: number, param2: number): void;
				public static storeBuffer(key: number, buffer: java.nio.ByteBuffer): void;
				public static nativeInitWebGL(param0: globalAndroid.view.Surface, param1: boolean, param2: boolean, param3: boolean, param4: boolean, param5: number, param6: boolean, param7: boolean, param8: boolean, param9: boolean, param10: boolean, param11: number): number;
				public initContext(type: string, alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number, premultipliedAlpha: boolean, preserveDrawingBuffer: boolean, stencil: boolean, desynchronized: boolean): void;
				public static nativeCreate2dContextVulkan(param0: number, param1: number, param2: globalAndroid.view.Surface, param3: boolean, param4: number, param5: number, param6: number, param7: number): number;
				public onTouchEvent(event: globalAndroid.view.MotionEvent): boolean;
				public static getDirection(): number;
				public static nativeUpdate2DSurfaceNoSurface(param0: number, param1: number, param2: number): void;
				public initContext(type: string, alpha: boolean): void;
				public onAttachedToWindow(): void;
				public surfaceDestroyed$canvas_release(): void;
				public static storeBuffer(key: string, buffer: java.nio.ByteBuffer): void;
				public constructor(context: globalAndroid.content.Context);
				public static nativeCreate2DContext(param0: globalAndroid.view.Surface, param1: boolean, param2: number, param3: number, param4: number, param5: number): number;
				public initContext(type: string, alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean): void;
				public static nativeCustomWithBitmapFlush(param0: number, param1: globalAndroid.graphics.Bitmap): void;
				public getSurfaceHeight(): number;
				public static nativeContext2DTest(param0: number): void;
				public setForceGL(value: boolean): void;
				public getSurfaceWidth(): number;
				public constructor(context: globalAndroid.content.Context, attrs: globalAndroid.util.AttributeSet);
				public initContext(type: string, alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number, premultipliedAlpha: boolean, preserveDrawingBuffer: boolean, stencil: boolean): void;
				public getDrawingBufferWidth(): number;
				public static removeBuffer(key: number): void;
				public static nativeContext2DPathTest(param0: number): void;
				public static nativeResizeCustomSurface(param0: number, param1: number, param2: number, param3: number, param4: boolean, param5: number): void;
				public static getBuffer(key: number): java.nio.ByteBuffer;
				public getNativeContext(): number;
				public getIgnoreTouchEvents(): boolean;
				public static context2DRender(context: number): void;
				public setTouchEventListener(value: org.nativescript.canvas.NSCCanvas.TouchEvents): void;
				public resize$canvas_release(): void;
				public static getViews(): java.util.concurrent.ConcurrentHashMap<any, any>;
				public static WebGLContextRender(gl: number, context: number, internalFormat: number, format: number): void;
				public static context2DPathTest(context: number): void;
				public snapshot(needsToFlip: boolean): globalAndroid.graphics.Bitmap;
				public static nativeWebGLC2DRender(param0: number, param1: number, param2: number, param3: number): void;
				public initContext(type: string, alpha: boolean, antialias: boolean, depth: boolean): void;
				public setTextureView(value: org.nativescript.canvas.GLView): void;
				public onDetachedFromWindow(): void;
				public static getBuffers(key: number): androidNative.Array<any>;
				public static nativeWriteCurrentWebGLContextToBitmap(param0: number, param1: globalAndroid.graphics.Bitmap): void;
				public initContext(type: string, alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number, premultipliedAlpha: boolean, preserveDrawingBuffer: boolean): void;
				public setAttachedToWindow$canvas_release(value: boolean): void;
				public initContext(type: string, alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number, premultipliedAlpha: boolean): void;
				public static nativeUpdate2DSurface(param0: globalAndroid.view.Surface, param1: number): void;
				public finalize(): void;
				public static getBoundingClientRect(canvas: org.nativescript.canvas.NSCCanvas): void;
				public initContext(type: string, alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number): void;
				public initContext(type: string, alpha: boolean, antialias: boolean): void;
				public forceResize(): void;
				public static nativeReleaseWebGL(param0: number): void;
				public static layoutSurface(width: number, height: number, canvas: org.nativescript.canvas.NSCCanvas): void;
				public getSurfaceView(): org.nativescript.canvas.GLViewSV;
				public initContext(type: string): void;
				public static nativeMakeWebGLCurrent(param0: number): boolean;
				public setBoundsBuffer(value: java.nio.FloatBuffer): void;
				public static storeBuffers(key: string, buffer: androidNative.Array<java.nio.ByteBuffer>): void;
				public static setViews(value: java.util.concurrent.ConcurrentHashMap<any, any>): void;
				public setPaused$canvas_release(value: boolean): void;
				public initContext(type: string, alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number, premultipliedAlpha: boolean, preserveDrawingBuffer: boolean, stencil: boolean, desynchronized: boolean, xrCompatible: boolean): void;
				public initContextWithJsonString(value: string, key: string): void;
				public setIgnoreTouchEvents(value: boolean): void;
				public getListener(): org.nativescript.canvas.NSCCanvas.Listener;
				public static loadLib(): void;
				public getForceGL(): boolean;
				public static setEnableDebug(value: boolean): void;
				public static nativeInitWebGLNoSurface(param0: number, param1: number, param2: boolean, param3: boolean, param4: boolean, param5: boolean, param6: number, param7: boolean, param8: boolean, param9: boolean, param10: boolean, param11: boolean, param12: number): number;
				public static getBuffer(key: string): java.nio.ByteBuffer;
				public initWebGPUContext(it: number): void;
				public setFit(value: org.nativescript.canvas.CanvasFit): void;
				public getDrawingBufferHeight(): number;
				public static nativeResizeWebGPU(param0: number, param1: globalAndroid.view.Surface, param2: number, param3: number): void;
				public static setForceGL(value: boolean): void;
				public static getForceGL(): boolean;
			}
			export module NSCCanvas {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.NSCCanvas.Companion>;
					public removeBuffer(key: number): void;
					public getDirection(): number;
					public nativeWebGLC2DRender(context: number, c2d: number, internalFormat: number, format: number): void;
					public layoutSurface(width: number, height: number, canvas: org.nativescript.canvas.NSCCanvas): void;
					public append$canvas_release(key: string, value: number, sb: java.lang.StringBuilder, isLast: boolean): void;
					public context2DRender(context: number): void;
					public storeBuffers(key: string, buffer: androidNative.Array<java.nio.ByteBuffer>): void;
					public storeBuffer(key: number, buffer: java.nio.ByteBuffer): void;
					public nativeInitWebGPU(instance: number, surface: globalAndroid.view.Surface, width: number, height: number): number;
					public nativeContext2DTest(context: number): void;
					public setLibraryLoaded$canvas_release(value: boolean): void;
					public getBuffer(key: number): java.nio.ByteBuffer;
					public nativeReleaseWebGL(context: number): void;
					public WebGLContextRender(gl: number, context: number, internalFormat: number, format: number): void;
					public storeBuffer(key: string, buffer: java.nio.ByteBuffer): void;
					public removeBuffer(key: string): void;
					public getBoundingClientRect(density: org.nativescript.canvas.NSCCanvas): void;
					public nativeInitWebGL(surface: globalAndroid.view.Surface, alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number, premultipliedAlpha: boolean, preserveDrawingBuffer: boolean, stencil: boolean, desynchronized: boolean, xrCompatible: boolean, version: number): number;
					public getBoundingClientRectJSON(sb: org.nativescript.canvas.NSCCanvas): string;
					public nativeUpdate2DSurface(surface: globalAndroid.view.Surface, context: number): void;
					public context2DTest(context: number): void;
					public nativeInitWebGLNoSurface(width: number, height: number, alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number, premultipliedAlpha: boolean, preserveDrawingBuffer: boolean, stencil: boolean, desynchronized: boolean, xrCompatible: boolean, version: number): number;
					public nativeResizeWebGPU(context: number, surface: globalAndroid.view.Surface, width: number, height: number): void;
					public getViews(): java.util.concurrent.ConcurrentHashMap<any, any>;
					public loadLib(): void;
					public nativeCustomWithBitmapFlush(context: number, view: globalAndroid.graphics.Bitmap): void;
					public context2DPathTest(context: number): void;
					public getStore(): java.util.concurrent.ConcurrentHashMap<any, any>;
					public getEnableDebug(): boolean;
					public nativeCreate2dContextVulkan(width: number, height: number, surface: globalAndroid.view.Surface, alpha: boolean, density: number, fontColor: number, ppi: number, direction: number): number;
					public layoutSurface(rootParams: number, w: number, h: org.nativescript.canvas.NSCCanvas): void;
					public nativeCreate2DContext(surface: globalAndroid.view.Surface, alpha: boolean, density: number, fontColor: number, ppi: number, direction: number): number;
					public nativeMakeWebGLCurrent(context: number): boolean;
					public nativeWriteCurrentWebGLContextToBitmap(context: number, bitmap: globalAndroid.graphics.Bitmap): void;
					public isLibraryLoaded$canvas_release(): boolean;
					public setViews(value: java.util.concurrent.ConcurrentHashMap<any, any>): void;
					public setStore(value: java.util.concurrent.ConcurrentHashMap<any, any>): void;
					public nativeContext2DPathTest(context: number): void;
					public getBuffers(key: number): androidNative.Array<any>;
					public nativeResizeCustomSurface(context: number, width: number, height: number, density: number, alpha: boolean, ppi: number): void;
					public getBuffer(key: string): java.nio.ByteBuffer;
					public nativeUpdateWebGLSurface(surface: globalAndroid.view.Surface, context: number): void;
					public nativeUpdate2DSurfaceNoSurface(width: number, height: number, context: number): void;
					public nativeInitContextWithCustomSurface(width: number, height: number, density: number, alpha: boolean, fontColor: number, ppi: number, direction: number): number;
					public setEnableDebug(value: boolean): void;
					public nativeContext2DRender(context: number): void;
					public nativeUpdateWebGLNoSurface(width: number, height: number, context: number): void;
					public static setForceGL(value: boolean): void;
					public static getForceGL(): boolean;
				}
				export module Companion {
					export class WhenMappings {
						public static class: java.lang.Class<org.nativescript.canvas.NSCCanvas.Companion.WhenMappings>;
					}
				}
				export class Engine {
					public static class: java.lang.Class<org.nativescript.canvas.NSCCanvas.Engine>;
					public static None: org.nativescript.canvas.NSCCanvas.Engine;
					public static CPU: org.nativescript.canvas.NSCCanvas.Engine;
					public static GL: org.nativescript.canvas.NSCCanvas.Engine;
					public static GPU: org.nativescript.canvas.NSCCanvas.Engine;
					public static values(): androidNative.Array<org.nativescript.canvas.NSCCanvas.Engine>;
					public static valueOf(value: string): org.nativescript.canvas.NSCCanvas.Engine;
					public static getEntries(): any;
				}
				export class Listener {
					public static class: java.lang.Class<org.nativescript.canvas.NSCCanvas.Listener>;
					/**
					 * Constructs a new instance of the org.nativescript.canvas.NSCCanvas$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: { contextReady(): void; surfaceResize(param0: number, param1: number): void });
					public constructor();
					public contextReady(): void;
					public surfaceResize(param0: number, param1: number): void;
				}
				export class SurfaceType {
					public static class: java.lang.Class<org.nativescript.canvas.NSCCanvas.SurfaceType>;
					public static Texture: org.nativescript.canvas.NSCCanvas.SurfaceType;
					public static Surface: org.nativescript.canvas.NSCCanvas.SurfaceType;
					public static getEntries(): any;
					public static values(): androidNative.Array<org.nativescript.canvas.NSCCanvas.SurfaceType>;
					public static valueOf(value: string): org.nativescript.canvas.NSCCanvas.SurfaceType;
				}
				export class TouchEvents {
					public static class: java.lang.Class<org.nativescript.canvas.NSCCanvas.TouchEvents>;
					/**
					 * Constructs a new instance of the org.nativescript.canvas.NSCCanvas$TouchEvents interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: { onEvent(param0: string, param1: globalAndroid.view.MotionEvent): void });
					public constructor();
					public onEvent(param0: string, param1: globalAndroid.view.MotionEvent): void;
				}
				export class WhenMappings {
					public static class: java.lang.Class<org.nativescript.canvas.NSCCanvas.WhenMappings>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class NSCCanvasRenderingContext2D {
				public static class: java.lang.Class<org.nativescript.canvas.NSCCanvasRenderingContext2D>;
				public static drawImage(context: number, image: number, sx: number, sy: number, sWidth: number, sHeight: number, dx: number, dy: number, dWidth: number, dHeight: number): boolean;
				public static createPattern(context: number, bitmap: globalAndroid.graphics.Bitmap, repetition: string): number;
				public static scale(context: number, x: number, y: number): void;
				public static drawImage(context: number, image: globalAndroid.graphics.Bitmap, dx: number, dy: number, dWidth: number, dHeight: number): boolean;
				public static drawImage(context: number, image: number, dx: number, dy: number): boolean;
				public static drawImage(context: number, image: globalAndroid.graphics.Bitmap, dx: number, dy: number): boolean;
				public static drawAtlas(context: number, bitmap: globalAndroid.graphics.Bitmap, xform: androidNative.Array<number>, tex: androidNative.Array<number>, colors: androidNative.Array<number>, blendMode: number): void;
				public static drawImage(context: number, image: number, dx: number, dy: number, dWidth: number, dHeight: number): boolean;
				public static drawImage(context: number, image: globalAndroid.graphics.Bitmap, sx: number, sy: number, sWidth: number, sHeight: number, dx: number, dy: number, dWidth: number, dHeight: number): boolean;
				public constructor();
			}
			export module NSCCanvasRenderingContext2D {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.NSCCanvasRenderingContext2D.Companion>;
					public scale(context: number, x: number, y: number): void;
					public drawImage(height: number, this_: globalAndroid.graphics.Bitmap, context: number, image: number): boolean;
					public drawImage(context: number, image: number, dx: number, dy: number): boolean;
					public drawImage(context: number, image: number, sx: number, sy: number, sWidth: number, sHeight: number, dx: number, dy: number, dWidth: number, dHeight: number): boolean;
					public drawAtlas(context: number, bitmap: globalAndroid.graphics.Bitmap, xform: androidNative.Array<number>, tex: androidNative.Array<number>, colors: androidNative.Array<number>, blendMode: number): void;
					public drawImage(height: number, this_: globalAndroid.graphics.Bitmap, context: number, image: number, dx: number, dy: number): boolean;
					public drawImage(height: number, this_: globalAndroid.graphics.Bitmap, context: number, image: number, sx: number, sy: number, sWidth: number, sHeight: number, dx: number, dy: number): boolean;
					public drawImage(context: number, image: number, dx: number, dy: number, dWidth: number, dHeight: number): boolean;
					public createPattern(context: number, bitmap: globalAndroid.graphics.Bitmap, repetition: string): number;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class NSCImageAsset {
				public static class: java.lang.Class<org.nativescript.canvas.NSCImageAsset>;
				public static loadImageFromBitmapAsync(asset: number, bitmap: globalAndroid.graphics.Bitmap, callback: org.nativescript.canvas.NSCImageAsset.Callback): void;
				public static loadImageFromBytesAsync(asset: number, bitmap: globalAndroid.graphics.Bitmap, callback: org.nativescript.canvas.NSCImageAsset.Callback): void;
				public finalize(): void;
				public static nativeDestroyImageAsset(param0: number): void;
				public static loadImageFromPathAsync(asset: number, path: string, callback: org.nativescript.canvas.NSCImageAsset.Callback): void;
				public static createImageAsset(): number;
				public static destroyImageAsset(asset: number): void;
				public static loadImageFromResourceAsync(resources: globalAndroid.content.res.Resources, asset: number, image: number, callback: org.nativescript.canvas.NSCImageAsset.Callback): void;
				public static loadImageFromBytesAsync(asset: number, bytes: androidNative.Array<number>, callback: org.nativescript.canvas.NSCImageAsset.Callback): void;
				public static loadImageFromUrlAsync(asset: number, url: string, callback: org.nativescript.canvas.NSCImageAsset.Callback): void;
				public static loadWebPAsync(asset: number, path: string, callback: org.nativescript.canvas.NSCImageAsset.Callback): void;
				public static loadWebP(asset: number, path: string): boolean;
				public static getDimensions(asset: number): androidNative.Array<number>;
				public constructor(asset: number);
				public static loadImageFromResource(resources: globalAndroid.content.res.Resources, asset: number, image: number): boolean;
				public static loadImageFromBufferAsync(asset: number, buffer: java.nio.ByteBuffer, callback: org.nativescript.canvas.NSCImageAsset.Callback): void;
				public static loadImageFromBitmap(asset: number, bitmap: globalAndroid.graphics.Bitmap): boolean;
				public static getError(asset: number): string;
				public static loadFromPath(asset: number, path: string): boolean;
				public getAsset(): number;
			}
			export module NSCImageAsset {
				export class Callback {
					public static class: java.lang.Class<org.nativescript.canvas.NSCImageAsset.Callback>;
					/**
					 * Constructs a new instance of the org.nativescript.canvas.NSCImageAsset$Callback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: { onComplete(param0: boolean): void });
					public constructor();
					public onComplete(param0: boolean): void;
				}
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.NSCImageAsset.Companion>;
					public loadImageFromResource(canvas: globalAndroid.content.res.Resources, bitmap: number, this_: number): boolean;
					public loadImageFromBufferAsync(this_: number, asset: java.nio.ByteBuffer, buffer: org.nativescript.canvas.NSCImageAsset.Callback): void;
					public loadImageFromBytesAsync(this_: number, asset: globalAndroid.graphics.Bitmap, bitmap: org.nativescript.canvas.NSCImageAsset.Callback): void;
					public getError(asset: number): string;
					public loadImageFromPathAsync(this_: number, asset: string, path: org.nativescript.canvas.NSCImageAsset.Callback): void;
					public nativeDestroyImageAsset(asset: number): void;
					public loadImageFromBitmap(canvas: number, this_: globalAndroid.graphics.Bitmap): boolean;
					public loadWebPAsync(this_: number, asset: string, path: org.nativescript.canvas.NSCImageAsset.Callback): void;
					public loadImageFromBytesAsync(this_: number, asset: androidNative.Array<number>, bytes: org.nativescript.canvas.NSCImageAsset.Callback): void;
					public loadImageFromResourceAsync(this_: globalAndroid.content.res.Resources, resources: number, asset: number, image: org.nativescript.canvas.NSCImageAsset.Callback): void;
					public loadWebP(this_: number, asset: string): boolean;
					public loadFromPath(asset: number, path: string): boolean;
					public loadImageFromUrlAsync(this_: number, asset: string, url: org.nativescript.canvas.NSCImageAsset.Callback): void;
					public createImageAsset(): number;
					public loadImageFromBitmapAsync(this_: number, asset: globalAndroid.graphics.Bitmap, bitmap: org.nativescript.canvas.NSCImageAsset.Callback): void;
					public destroyImageAsset(asset: number): void;
					public getDimensions(this_: number): androidNative.Array<number>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class NSCImageBitmap {
				public static class: java.lang.Class<org.nativescript.canvas.NSCImageBitmap>;
				public static createFrom(asset: number, data: java.nio.ByteBuffer, callback: org.nativescript.canvas.NSCImageBitmap.Callback): void;
				public static createFromRectOptions(asset: number, data: java.nio.ByteBuffer, x: number, y: number, width: number, height: number, options: org.nativescript.canvas.NSCImageBitmap.Options, callback: org.nativescript.canvas.NSCImageBitmap.Callback): void;
				public static createFromRectOptions(asset: number, data: androidNative.Array<number>, x: number, y: number, width: number, height: number, options: org.nativescript.canvas.NSCImageBitmap.Options, callback: org.nativescript.canvas.NSCImageBitmap.Callback): void;
				public static createFromOptions(asset: number, data: java.nio.ByteBuffer, options: org.nativescript.canvas.NSCImageBitmap.Options, callback: org.nativescript.canvas.NSCImageBitmap.Callback): void;
				public finalize(): void;
				public static createFromOptions(asset: number, data: androidNative.Array<number>, options: org.nativescript.canvas.NSCImageBitmap.Options, callback: org.nativescript.canvas.NSCImageBitmap.Callback): void;
				public constructor(asset: number);
				public static createFrom(asset: number, data: androidNative.Array<number>, callback: org.nativescript.canvas.NSCImageBitmap.Callback): void;
				public getAsset(): number;
			}
			export module NSCImageBitmap {
				export class Callback {
					public static class: java.lang.Class<org.nativescript.canvas.NSCImageBitmap.Callback>;
					/**
					 * Constructs a new instance of the org.nativescript.canvas.NSCImageBitmap$Callback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: { onComplete(param0: boolean): void });
					public constructor();
					public onComplete(param0: boolean): void;
				}
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.NSCImageBitmap.Companion>;
					public createFromRectOptions(this_: number, asset: java.nio.ByteBuffer, data: number, x: number, y: number, width: number, height: org.nativescript.canvas.NSCImageBitmap.Options, options: org.nativescript.canvas.NSCImageBitmap.Callback): void;
					public createFrom(this_: number, asset: java.nio.ByteBuffer, data: org.nativescript.canvas.NSCImageBitmap.Callback): void;
					public createFromOptions(this_: number, asset: java.nio.ByteBuffer, data: org.nativescript.canvas.NSCImageBitmap.Options, options: org.nativescript.canvas.NSCImageBitmap.Callback): void;
					public createFromRectOptions(this_: number, asset: androidNative.Array<number>, data: number, x: number, y: number, width: number, height: org.nativescript.canvas.NSCImageBitmap.Options, options: org.nativescript.canvas.NSCImageBitmap.Callback): void;
					public createFromOptions(this_: number, asset: androidNative.Array<number>, data: org.nativescript.canvas.NSCImageBitmap.Options, options: org.nativescript.canvas.NSCImageBitmap.Callback): void;
					public createFrom(this_: number, asset: androidNative.Array<number>, data: org.nativescript.canvas.NSCImageBitmap.Callback): void;
				}
				export class Options {
					public static class: java.lang.Class<org.nativescript.canvas.NSCImageBitmap.Options>;
					public getColorSpaceConversion(): org.nativescript.canvas.ImageBitmapColorSpaceConversion;
					public getResizeHeight(): number;
					public setImageOrientation(value: org.nativescript.canvas.ImageBitmapImageOrientation): void;
					public getImageOrientation(): org.nativescript.canvas.ImageBitmapImageOrientation;
					public getPremultiplyAlpha(): org.nativescript.canvas.ImageBitmapPremultiplyAlpha;
					public setResizeHeight(value: number): void;
					public getResizeQuality(): org.nativescript.canvas.ImageBitmapResizeQuality;
					public setResizeQuality(value: org.nativescript.canvas.ImageBitmapResizeQuality): void;
					public setResizeWidth(value: number): void;
					public constructor();
					public setPremultiplyAlpha(value: org.nativescript.canvas.ImageBitmapPremultiplyAlpha): void;
					public setColorSpaceConversion(value: org.nativescript.canvas.ImageBitmapColorSpaceConversion): void;
					public getResizeWidth(): number;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class NSCSVG {
				public static class: java.lang.Class<org.nativescript.canvas.NSCSVG>;
				public getBitmap$canvas_release(): globalAndroid.graphics.Bitmap;
				public setSrc(src: string): void;
				public setSrcPath(path: string): void;
				public setBitmap$canvas_release(value: globalAndroid.graphics.Bitmap): void;
				public getLock$canvas_release(): any;
				public constructor(context: globalAndroid.content.Context, attrs: globalAndroid.util.AttributeSet);
				public constructor(context: globalAndroid.content.Context);
				public getSync(): boolean;
				public onDraw(it: globalAndroid.graphics.Canvas): void;
				public onSizeChanged(it: number, metrics: number, this_: number, w: number): void;
				public setSync(value: boolean): void;
				public flush(): void;
			}
			export module NSCSVG {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.NSCSVG.Companion>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class NSCTouchHandler {
				public static class: java.lang.Class<org.nativescript.canvas.NSCTouchHandler>;
				public constructor(canvas: org.nativescript.canvas.NSCCanvas);
				public getCanvas(): org.nativescript.canvas.NSCCanvas;
				public handle(scale: globalAndroid.view.MotionEvent): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class NSCWebGLRenderingContext {
				public static class: java.lang.Class<org.nativescript.canvas.NSCWebGLRenderingContext>;
				public static texImage2D(context: number, target: number, level: number, internalformat: number, format: number, type: number, image: globalAndroid.graphics.Bitmap, flipY: boolean): void;
				public constructor();
				public static texSubImage2D(context: number, target: number, level: number, xoffset: number, yoffset: number, format: number, type: number, image: globalAndroid.graphics.Bitmap, flipY: boolean): void;
			}
			export module NSCWebGLRenderingContext {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.NSCWebGLRenderingContext.Companion>;
					public texSubImage2D(context: number, target: number, level: number, xoffset: number, yoffset: number, format: number, type: number, image: globalAndroid.graphics.Bitmap, flipY: boolean): void;
					public texImage2D(context: number, target: number, level: number, internalformat: number, format: number, type: number, image: globalAndroid.graphics.Bitmap, flipY: boolean): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TextureRender {
				public static class: java.lang.Class<org.nativescript.canvas.TextureRender>;
				public setPos(value: number): void;
				public getHeight(): number;
				public setMatrix(value: androidNative.Array<number>): void;
				public surfaceCreated(): void;
				public setAb(value: number): void;
				public constructor();
				public getMatrix(): androidNative.Array<number>;
				public setSamplerPos(value: number): void;
				public setRbo(value: number): void;
				public getFbo(): number;
				public setWidth(value: number): void;
				public setHeight(value: number): void;
				public setTextureId(value: number): void;
				public setMatrixPos(value: number): void;
				public getTextureId(): number;
				public static updateTexImageAndGetTransformMatrix(surfaceTexture: globalAndroid.graphics.SurfaceTexture, matrix: androidNative.Array<number>): void;
				public getAb(): number;
				public getWidth(): number;
				public getSamplerPos(): number;
				public drawFrame(st: globalAndroid.graphics.SurfaceTexture, width: number, height: number, internalFormat: number, format: number, flipYWebGL: boolean): void;
				public setFbo(value: number): void;
				public getPos(): number;
				public getMatrixPos(): number;
				public getMProgram(): number;
				public setMProgram(value: number): void;
				public getRbo(): number;
			}
			export module TextureRender {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TextureRender.Companion>;
					public updateTexImageAndGetTransformMatrix(surfaceTexture: globalAndroid.graphics.SurfaceTexture, matrix: androidNative.Array<number>): void;
					public getVextexCoords(): androidNative.Array<number>;
					public getVextexBuf(): java.nio.FloatBuffer;
					public setVextexBuf(value: java.nio.FloatBuffer): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class Utils {
				public static class: java.lang.Class<org.nativescript.canvas.Utils>;
				public static INSTANCE: org.nativescript.canvas.Utils;
				public getByteBufferFromBitmap(bitmap: globalAndroid.graphics.Bitmap): java.nio.ByteBuffer;
				public isEmulator(): boolean;
				public static createSurfaceTexture(render: number): androidNative.Array<any>;
				public getBytesFromBitmap(bitmap: globalAndroid.graphics.Bitmap): androidNative.Array<number>;
				public static updateTexImage(state: number, flipY: boolean, texture: globalAndroid.graphics.SurfaceTexture, render: org.nativescript.canvas.TextureRender, width: number, height: number, internalFormat: number, format: number): void;
				public static createRenderAndAttachToGLContext(render: number, state: globalAndroid.graphics.SurfaceTexture): org.nativescript.canvas.TextureRender;
				public static attachToGLContext(state: number, texture: globalAndroid.graphics.SurfaceTexture, render: org.nativescript.canvas.TextureRender): void;
				public static detachFromGLContext(state: number, texture: globalAndroid.graphics.SurfaceTexture): void;
			}
		}
	}
}

//Generics information:
