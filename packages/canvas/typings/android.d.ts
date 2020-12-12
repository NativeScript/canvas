/// <reference path="android-declarations.d.ts"/>

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class AnimationFrame {
					public static class: java.lang.Class<com.github.triniwiz.canvas.AnimationFrame>;
					public static Companion: com.github.triniwiz.canvas.AnimationFrame.Companion;
					public raf(param0: number): void;
					public reset(): void;
					public doFrame(param0: number): void;
					public constructor();
					public fps(param0: number): void;
				}
				export module AnimationFrame {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.AnimationFrame.Companion>;
						public requestAnimationFrame(param0: kotlin.jvm.functions.Function1<any,kotlin.Unit>): number;
						public cancelAnimationFrame(param0: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class BuildConfig {
					public static class: java.lang.Class<com.github.triniwiz.canvas.BuildConfig>;
					public static DEBUG: boolean;
					public static LIBRARY_PACKAGE_NAME: string;
					public static BUILD_TYPE: string;
					public constructor();
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class CPUView {
					public static class: java.lang.Class<com.github.triniwiz.canvas.CPUView>;
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
					public constructor(param0: globalAndroid.content.Context);
					public onDraw(param0: globalAndroid.graphics.Canvas): void;
					public flush(): void;
					public getCanvasView$canvas_release(): java.lang.ref.WeakReference<com.github.triniwiz.canvas.TNSCanvas>;
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet, param2: number);
					public setCanvasView$canvas_release(param0: java.lang.ref.WeakReference<com.github.triniwiz.canvas.TNSCanvas>): void;
					public onSizeChanged(param0: number, param1: number, param2: number, param3: number): void;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class Constants {
					public static class: java.lang.Class<com.github.triniwiz.canvas.Constants>;
					public static GL_RGBA16F_EXT: number;
					public static GL_RGB16F_EXT: number;
					public static GL_RG16F_EXT: number;
					public static GL_R16F_EXT: number;
					public static GL_R32F_EXT: number;
					public static GL_RG32F_EXT: number;
					public static GL_RGBA32F_EXT: number;
					public static GL_RGB32F_EXT: number;
					public static GL_MIN_EXT: number;
					public static GL_MAX_EXT: number;
					public static GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: number;
					public static GL_UNSIGNED_NORMALIZED_EXT: number;
					public static GL_SRGB_EXT: number;
					public static GL_SRGB_ALPHA_EXT: number;
					public static GL_SRGB8_ALPHA8_EXT: number;
					public static GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: number;
					public static GL_TEXTURE_MAX_ANISOTROPY_EXT: number;
					public static GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: number;
					public static GL_QUERY_COUNTER_BITS_EXT: number;
					public static GL_CURRENT_QUERY_EXT: number;
					public static GL_QUERY_RESULT_EXT: number;
					public static GL_QUERY_RESULT_AVAILABLE_EXT: number;
					public static GL_TIME_ELAPSED_EXT: number;
					public static GL_TIMESTAMP_EXT: number;
					public static GL_GPU_DISJOINT_EXT: number;
					public static GL_FRAGMENT_SHADER_DERIVATIVE_HINT_OES: number;
					public static GL_HALF_FLOAT_OES: number;
					public static GL_VERTEX_ARRAY_BINDING_OES: number;
					public static COMPRESSED_RGB_ATC_WEBGL: number;
					public static COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL: number;
					public static COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL: number;
					public static GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG: number;
					public static GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG: number;
					public static GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: number;
					public static GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: number;
					public static GL_COMPRESSED_RGB_S3TC_DXT1_EXT: number;
					public static GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: number;
					public static GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: number;
					public static GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: number;
					public static GL_COMPRESSED_SRGB_S3TC_DXT1_EXT: number;
					public static GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: number;
					public static GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: number;
					public static GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: number;
					public static GL_MAX_COLOR_ATTACHMENTS_EXT: number;
					public static GL_MAX_DRAW_BUFFERS_EXT: number;
					public static GL_DRAW_BUFFER0_EXT: number;
					public static GL_DRAW_BUFFER1_EXT: number;
					public static GL_DRAW_BUFFER2_EXT: number;
					public static GL_DRAW_BUFFER3_EXT: number;
					public static GL_DRAW_BUFFER4_EXT: number;
					public static GL_DRAW_BUFFER5_EXT: number;
					public static GL_DRAW_BUFFER6_EXT: number;
					public static GL_DRAW_BUFFER7_EXT: number;
					public static GL_DRAW_BUFFER8_EXT: number;
					public static GL_DRAW_BUFFER9_EXT: number;
					public static GL_DRAW_BUFFER10_EXT: number;
					public static GL_DRAW_BUFFER11_EXT: number;
					public static GL_DRAW_BUFFER12_EXT: number;
					public static GL_DRAW_BUFFER13_EXT: number;
					public static GL_DRAW_BUFFER14_EXT: number;
					public static GL_DRAW_BUFFER15_EXT: number;
					public static GL_COLOR_ATTACHMENT0_EXT: number;
					public static GL_COLOR_ATTACHMENT1_EXT: number;
					public static GL_COLOR_ATTACHMENT2_EXT: number;
					public static GL_COLOR_ATTACHMENT3_EXT: number;
					public static GL_COLOR_ATTACHMENT4_EXT: number;
					public static GL_COLOR_ATTACHMENT5_EXT: number;
					public static GL_COLOR_ATTACHMENT6_EXT: number;
					public static GL_COLOR_ATTACHMENT7_EXT: number;
					public static GL_COLOR_ATTACHMENT8_EXT: number;
					public static GL_COLOR_ATTACHMENT9_EXT: number;
					public static GL_COLOR_ATTACHMENT10_EXT: number;
					public static GL_COLOR_ATTACHMENT11_EXT: number;
					public static GL_COLOR_ATTACHMENT12_EXT: number;
					public static GL_COLOR_ATTACHMENT13_EXT: number;
					public static GL_COLOR_ATTACHMENT14_EXT: number;
					public static GL_COLOR_ATTACHMENT15_EXT: number;
					public static INSTANCE: com.github.triniwiz.canvas.Constants;
					public setMAX_CLIENT_WAIT_TIMEOUT_WEBGL(param0: number): void;
					public getMAX_CLIENT_WAIT_TIMEOUT_WEBGL(): number;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class GLContext {
					public static class: java.lang.Class<com.github.triniwiz.canvas.GLContext>;
					public glView: java.lang.ref.WeakReference<com.github.triniwiz.canvas.GLView>;
					public mGLThread: com.github.triniwiz.canvas.GLContext.GLThread;
					public reference: java.lang.ref.WeakReference<com.github.triniwiz.canvas.TNSCanvas>;
					public static TAG: string;
					public static Companion: com.github.triniwiz.canvas.GLContext.Companion;
					public getPremultipliedAlpha(): boolean;
					public getDepth(): boolean;
					public queueEvent(param0: java.lang.Runnable): void;
					public resize(param0: number, param1: number): void;
					public swapBuffers(param0: javax.microedition.khronos.egl.EGLSurface): boolean;
					public setXrCompatible(param0: boolean): void;
					public init(param0: any): void;
					public isHeadless(): boolean;
					public setStencil(param0: boolean): void;
					public setAlpha(param0: boolean): void;
					public createSurface(param0: javax.microedition.khronos.egl.EGLConfig, param1: any): javax.microedition.khronos.egl.EGLSurface;
					public getStencil(): boolean;
					public destroy(): void;
					public setPremultipliedAlpha(param0: boolean): void;
					public startGLThread(): void;
					public constructor();
					public getFailIfMajorPerformanceCaveat(): boolean;
					public destroySurface(param0: javax.microedition.khronos.egl.EGLSurface): boolean;
					public getAntialias(): boolean;
					public getDesynchronized(): boolean;
					public getXrCompatible(): boolean;
					public setPowerPreference(param0: string): void;
					public setAntialias(param0: boolean): void;
					public getAlpha(): boolean;
					public flush(): void;
					public setFailIfMajorPerformanceCaveat(param0: boolean): void;
					public getPowerPreference(): string;
					public onResume(): void;
					public makeCurrent(param0: javax.microedition.khronos.egl.EGLSurface): boolean;
					public setDepth(param0: boolean): void;
					public isGLThreadStarted(): boolean;
					public getPreserveDrawingBuffer(): boolean;
					public onPause(): void;
					public setPreserveDrawingBuffer(param0: boolean): void;
					public setDesynchronized(param0: boolean): void;
				}
				export module GLContext {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLContext.Companion>;
					}
					export class GLThread {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLContext.GLThread>;
						public isStarted: boolean;
						public start(): void;
						public setMSurface(param0: any): void;
						public run(): void;
						public constructor(param0: com.github.triniwiz.canvas.GLContext);
						public interrupt(): void;
						public setPaused(param0: boolean): void;
						public constructor(param0: any);
						public getMSurface(): any;
						public getType(): com.github.triniwiz.canvas.TNSCanvas.ContextType;
						public setType(param0: com.github.triniwiz.canvas.TNSCanvas.ContextType): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class GLView {
					public static class: java.lang.Class<com.github.triniwiz.canvas.GLView>;
					public drawingBufferWidth: number;
					public drawingBufferHeight: number;
					public resize(param0: number, param1: number): void;
					public queueEvent(param0: java.lang.Runnable): void;
					public onSurfaceTextureDestroyed(param0: globalAndroid.graphics.SurfaceTexture): boolean;
					public onSurfaceTextureUpdated(param0: globalAndroid.graphics.SurfaceTexture): void;
					public flush(): void;
					public init(): void;
					public setStarting(param0: boolean): void;
					public getGLContext(): com.github.triniwiz.canvas.GLContext;
					public onSurfaceTextureAvailable(param0: globalAndroid.graphics.SurfaceTexture, param1: number, param2: number): void;
					public constructor(param0: globalAndroid.content.Context);
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
					public setStartupLock(param0: java.util.concurrent.CountDownLatch): void;
					public setupContext(): void;
					public setListener(param0: com.github.triniwiz.canvas.TNSCanvas.Listener): void;
					public getStarting(): boolean;
					public getStartupLock(): java.util.concurrent.CountDownLatch;
					public onSurfaceTextureSizeChanged(param0: globalAndroid.graphics.SurfaceTexture, param1: number, param2: number): void;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class GLViewSV {
					public static class: java.lang.Class<com.github.triniwiz.canvas.GLViewSV>;
					public constructor(param0: globalAndroid.content.Context);
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
					public surfaceDestroyed(param0: globalAndroid.view.SurfaceHolder): void;
					public queueEvent(param0: java.lang.Runnable): void;
					public surfaceCreated(param0: globalAndroid.view.SurfaceHolder): void;
					public flush(): void;
					public init(): void;
					public getGLContext(): com.github.triniwiz.canvas.GLContext;
					public setListener(param0: com.github.triniwiz.canvas.TNSCanvas.Listener): void;
					public surfaceChanged(param0: globalAndroid.view.SurfaceHolder, param1: number, param2: number, param3: number): void;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class HowToClear {
					public static class: java.lang.Class<com.github.triniwiz.canvas.HowToClear>;
					public static Skipped: com.github.triniwiz.canvas.HowToClear;
					public static JustClear: com.github.triniwiz.canvas.HowToClear;
					public static CombinedClear: com.github.triniwiz.canvas.HowToClear;
					public static values(): native.Array<com.github.triniwiz.canvas.HowToClear>;
					public static valueOf(param0: string): com.github.triniwiz.canvas.HowToClear;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class IndexedParameter {
					public static class: java.lang.Class<com.github.triniwiz.canvas.IndexedParameter>;
					public setBuffer(param0: boolean): void;
					public constructor();
					public getValue(): number;
					public setValue(param0: number): void;
					public setBufferValue(param0: number): void;
					public getBufferValue(): number;
					public isBuffer(): boolean;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class SVGView {
					public static class: java.lang.Class<com.github.triniwiz.canvas.SVGView>;
					public static Companion: com.github.triniwiz.canvas.SVGView.Companion;
					public constructor(param0: globalAndroid.content.Context);
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
					public setSrc(param0: string): void;
					public getBitmap$canvas_release(): globalAndroid.graphics.Bitmap;
					public onDraw(param0: globalAndroid.graphics.Canvas): void;
					public setBitmap$canvas_release(param0: globalAndroid.graphics.Bitmap): void;
					public onSizeChanged(param0: number, param1: number, param2: number, param3: number): void;
				}
				export module SVGView {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.SVGView.Companion>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSCanvas {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvas>;
					public pendingInvalidate: boolean;
					public contextAlpha: boolean;
					public contextAntialias: boolean;
					public contextDepth: boolean;
					public contextFailIfMajorPerformanceCaveat: boolean;
					public contextPowerPreference: string;
					public contextPremultipliedAlpha: boolean;
					public contextPreserveDrawingBuffer: boolean;
					public contextStencil: boolean;
					public contextDesynchronized: boolean;
					public contextXrCompatible: boolean;
					public mClearStencil: number;
					public mClearColor: native.Array<number>;
					public mScissorEnabled: boolean;
					public mClearDepth: number;
					public mColorMask: native.Array<boolean>;
					public mStencilMask: number;
					public mStencilMaskBack: number;
					public mStencilFuncRef: number;
					public mStencilFuncRefBack: number;
					public mStencilFuncMask: number;
					public mStencilFuncMaskBack: number;
					public mDepthMask: boolean;
					public glVersion: number;
					public static ONE_MILLISECOND_NS: number;
					public static ONE_S_IN_NS: number;
					public static TAG: string;
					public static Companion: com.github.triniwiz.canvas.TNSCanvas.Companion;
					public onActivityStarted(param0: globalAndroid.app.Activity): void;
					public onActivityCreated(param0: globalAndroid.app.Activity, param1: globalAndroid.os.Bundle): void;
					public setWebGL2RenderingContext$canvas_release(param0: com.github.triniwiz.canvas.TNSWebGL2RenderingContext): void;
					public getCpuView$canvas_release(): com.github.triniwiz.canvas.CPUView;
					public finalize(): void;
					public queueEvent(param0: java.lang.Runnable): void;
					public getLastSize$canvas_release(): com.github.triniwiz.canvas.TNSCanvas.Size;
					public onAttachedToWindow(): void;
					public getSurface$canvas_release(): com.github.triniwiz.canvas.GLView;
					public getContext(param0: string): com.github.triniwiz.canvas.TNSCanvasRenderingContext;
					public onActivitySaveInstanceState(param0: globalAndroid.app.Activity, param1: globalAndroid.os.Bundle): void;
					public getNewSize$canvas_release(): com.github.triniwiz.canvas.TNSCanvas.Size;
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
					public setCpuHandlerThread(param0: globalAndroid.os.HandlerThread): void;
					public resizeViewPort(): void;
					public setPaused$canvas_release(param0: boolean): void;
					public getCpuHandler(): globalAndroid.os.Handler;
					public setCtx$canvas_release(param0: globalAndroid.content.Context): void;
					public setActualContextType$canvas_release(param0: string): void;
					public getContextType$canvas_release(): com.github.triniwiz.canvas.TNSCanvas.ContextType;
					public onActivityPaused(param0: globalAndroid.app.Activity): void;
					public setNewSize$canvas_release(param0: com.github.triniwiz.canvas.TNSCanvas.Size): void;
					public static nativeInitContext(param0: number, param1: number, param2: number, param3: number, param4: number, param5: boolean, param6: number, param7: number, param8: number): number;
					public setScale$canvas_release(param0: number): void;
					public getActualContextType$canvas_release(): string;
					public flush(): void;
					public static nativeCustomWithBitmapFlush(param0: number, param1: globalAndroid.graphics.Bitmap): number;
					public setContextType$canvas_release(param0: com.github.triniwiz.canvas.TNSCanvas.ContextType): void;
					public isHandleInvalidationManually$canvas_release(): boolean;
					public onResume(): void;
					public getDrawingBufferHeight(): number;
					public setRenderingContext2d$canvas_release(param0: com.github.triniwiz.canvas.TNSCanvasRenderingContext): void;
					public getCpuHandlerThread(): globalAndroid.os.HandlerThread;
					public static nativeFlush(param0: number): number;
					public setCpuView$canvas_release(param0: com.github.triniwiz.canvas.CPUView): void;
					public toDataURL(param0: string): string;
					public constructor(param0: globalAndroid.content.Context, param1: boolean);
					public getDrawingBufferWidth(): number;
					public doFrame(param0: number): void;
					public getListener(): com.github.triniwiz.canvas.TNSCanvas.Listener;
					public setupActivityHandler(param0: globalAndroid.app.Application): void;
					public getNativeContext$canvas_release(): number;
					public setSurface$canvas_release(param0: com.github.triniwiz.canvas.GLView): void;
					public snapshot(): java.nio.ByteBuffer;
					public isWebGL$canvas_release(): boolean;
					public initCanvas$canvas_release(): void;
					public setNativeContext$canvas_release(param0: number): void;
					public static getDirection$canvas_release(): com.github.triniwiz.canvas.TNSTextDirection;
					public toDataURL(): string;
					public getContext(param0: string, param1: java.util.Map<string,any>): com.github.triniwiz.canvas.TNSCanvasRenderingContext;
					public isPaused$canvas_release(): boolean;
					public setListener(param0: com.github.triniwiz.canvas.TNSCanvas.Listener): void;
					public onActivityResumed(param0: globalAndroid.app.Activity): void;
					public onActivityDestroyed(param0: globalAndroid.app.Activity): void;
					public setLastSize$canvas_release(param0: com.github.triniwiz.canvas.TNSCanvas.Size): void;
					public static nativeResizeSurface(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: boolean, param7: number): void;
					public static createSVGMatrix(): com.github.triniwiz.canvas.TNSCanvasDOMMatrix;
					public onDetachedFromWindow(): void;
					public toDataURLAsync(param0: com.github.triniwiz.canvas.TNSCanvas.DataURLListener): void;
					public setWebGL$canvas_release(param0: boolean): void;
					public getUseCpu$canvas_release(): boolean;
					public setUseCpu$canvas_release(param0: boolean): void;
					public onSizeChanged(param0: number, param1: number, param2: number, param3: number): void;
					public setWebGLRenderingContext$canvas_release(param0: com.github.triniwiz.canvas.TNSWebGLRenderingContext): void;
					public getWebGL2RenderingContext$canvas_release(): com.github.triniwiz.canvas.TNSWebGL2RenderingContext;
					public constructor(param0: globalAndroid.content.Context);
					public toDataURLAsync(param0: string, param1: com.github.triniwiz.canvas.TNSCanvas.DataURLListener): void;
					public onActivityStopped(param0: globalAndroid.app.Activity): void;
					public getRenderingContext2d$canvas_release(): com.github.triniwiz.canvas.TNSCanvasRenderingContext;
					public toDataURL(param0: string, param1: number): string;
					public getCtx$canvas_release(): globalAndroid.content.Context;
					public onPause(): void;
					public toData(): native.Array<number>;
					public setHandleInvalidationManually$canvas_release(param0: boolean): void;
					public static nativeInitContextWithCustomSurface(param0: number, param1: number, param2: number, param3: boolean, param4: number, param5: number, param6: number): number;
					public getWebGLRenderingContext$canvas_release(): com.github.triniwiz.canvas.TNSWebGLRenderingContext;
					public getScale$canvas_release(): number;
					public setCpuHandler(param0: globalAndroid.os.Handler): void;
					public toDataURLAsync(param0: string, param1: number, param2: com.github.triniwiz.canvas.TNSCanvas.DataURLListener): void;
				}
				export module TNSCanvas {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvas.Companion>;
						public nativeCustomWithBitmapFlush(param0: number, param1: globalAndroid.graphics.Bitmap): number;
						public nativeResizeSurface(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: boolean, param7: number): void;
						public getDirection$canvas_release(): com.github.triniwiz.canvas.TNSTextDirection;
						public nativeInitContext(param0: number, param1: number, param2: number, param3: number, param4: number, param5: boolean, param6: number, param7: number, param8: number): number;
						public createSVGMatrix(): com.github.triniwiz.canvas.TNSCanvasDOMMatrix;
						public getViews(): java.util.concurrent.ConcurrentHashMap<any,any>;
						public nativeInitContextWithCustomSurface(param0: number, param1: number, param2: number, param3: boolean, param4: number, param5: number, param6: number): number;
						public getLastCall$canvas_release(): number;
						public setLastCall$canvas_release(param0: number): void;
						public setViews(param0: java.util.concurrent.ConcurrentHashMap<any,any>): void;
						public nativeFlush(param0: number): number;
					}
					export class ContextType {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvas.ContextType>;
						public static NONE: com.github.triniwiz.canvas.TNSCanvas.ContextType;
						public static CANVAS: com.github.triniwiz.canvas.TNSCanvas.ContextType;
						public static WEBGL: com.github.triniwiz.canvas.TNSCanvas.ContextType;
						public static valueOf(param0: string): com.github.triniwiz.canvas.TNSCanvas.ContextType;
						public static values(): native.Array<com.github.triniwiz.canvas.TNSCanvas.ContextType>;
					}
					export class DataURLListener {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvas.DataURLListener>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.TNSCanvas$DataURLListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onResult(param0: string): void;
						});
						public constructor();
						public onResult(param0: string): void;
					}
					export class Listener {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvas.Listener>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.TNSCanvas$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							contextReady(): void;
						});
						public constructor();
						public contextReady(): void;
					}
					export class Size {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvas.Size>;
						public setWidth(param0: number): void;
						public getWidth(): number;
						public setHeight(param0: number): void;
						public getHeight(): number;
						public constructor(param0: number, param1: number);
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSCanvasDOMMatrix {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvasDOMMatrix>;
					public static Companion: com.github.triniwiz.canvas.TNSCanvasDOMMatrix.Companion;
					public setM13(param0: number): void;
					public finalize(): void;
					public setD(param0: number): void;
					public setM34(param0: number): void;
					public getD(): number;
					public constructor();
					public setM42(param0: number): void;
					public setM21(param0: number): void;
					public getM32(): number;
					public setM14(param0: number): void;
					public getM22(): number;
					public getM42(): number;
					public getM12(): number;
					public setC(param0: number): void;
					public setM33(param0: number): void;
					public getB(): number;
					public getM23(): number;
					public getM13(): number;
					public getA(): number;
					public setM22(param0: number): void;
					public getM43(): number;
					public setM41(param0: number): void;
					public getM33(): number;
					public setB(param0: number): void;
					public getM24(): number;
					public getM14(): number;
					public setM32(param0: number): void;
					public getM11(): number;
					public setM11(param0: number): void;
					public setF(param0: number): void;
					public getM44(): number;
					public getF(): number;
					public getM34(): number;
					public getM41(): number;
					public getM21(): number;
					public getC(): number;
					public setM23(param0: number): void;
					public setM44(param0: number): void;
					public getM31(): number;
					public constructor(param0: number);
					public getE(): number;
					public setMatrix$canvas_release(param0: number): void;
					public getMatrix$canvas_release(): number;
					public setA(param0: number): void;
					public setM12(param0: number): void;
					public setE(param0: number): void;
					public setM31(param0: number): void;
					public setM24(param0: number): void;
					public setM43(param0: number): void;
				}
				export module TNSCanvasDOMMatrix {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvasDOMMatrix.Companion>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSCanvasGradient extends com.github.triniwiz.canvas.TNSColorStyle {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvasGradient>;
					public static Companion: com.github.triniwiz.canvas.TNSCanvasGradient.Companion;
					public getStyle(): number;
					public constructor();
					public finalize(): void;
					public getStyleType(): com.github.triniwiz.canvas.TNSColorStyleType;
					public addColorStop(param0: number, param1: string): void;
					public constructor(param0: number);
					public setStyle(param0: number): void;
				}
				export module TNSCanvasGradient {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvasGradient.Companion>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSCanvasRenderingContext {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvasRenderingContext>;
					/**
					 * Constructs a new instance of the com.github.triniwiz.canvas.TNSCanvasRenderingContext interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
					});
					public constructor();
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSCanvasRenderingContext2D extends com.github.triniwiz.canvas.TNSCanvasRenderingContext {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvasRenderingContext2D>;
					public static TAG: string;
					public static Companion: com.github.triniwiz.canvas.TNSCanvasRenderingContext2D.Companion;
					public setGlobalCompositeOperation(param0: com.github.triniwiz.canvas.TNSCompositeOperationType): void;
					public beginPath(): void;
					public createRadialGradient(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): com.github.triniwiz.canvas.TNSCanvasGradient;
					public drawImage(param0: com.github.triniwiz.canvas.TNSCanvas, param1: number, param2: number): void;
					public setLineCap(param0: com.github.triniwiz.canvas.TNSLineCap): void;
					public clip(param0: com.github.triniwiz.canvas.TNSPath2D, param1: com.github.triniwiz.canvas.TNSFillRule): void;
					public getImageData(param0: number, param1: number, param2: number, param3: number): com.github.triniwiz.canvas.TNSImageData;
					public finalize(): void;
					public createPattern(param0: com.github.triniwiz.canvas.TNSCanvas, param1: com.github.triniwiz.canvas.TNSPatternRepetition): com.github.triniwiz.canvas.TNSPattern;
					public getCanvas(): com.github.triniwiz.canvas.TNSCanvas;
					public isPointInPath(param0: com.github.triniwiz.canvas.TNSPath2D, param1: number, param2: number, param3: com.github.triniwiz.canvas.TNSFillRule): boolean;
					public constructor(param0: com.github.triniwiz.canvas.TNSCanvas);
					public fillText(param0: string, param1: number, param2: number): void;
					public clip(param0: com.github.triniwiz.canvas.TNSPath2D): void;
					public getTextAlign(): com.github.triniwiz.canvas.TNSTextAlignment;
					public strokeRect(param0: number, param1: number, param2: number, param3: number): void;
					public strokeText(param0: string, param1: number, param2: number, param3: number): void;
					public bezierCurveTo(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public putImageData(param0: com.github.triniwiz.canvas.TNSImageData, param1: number, param2: number, param3: number, param4: number): void;
					public getDirection(): com.github.triniwiz.canvas.TNSTextDirection;
					public measureText(param0: string): com.github.triniwiz.canvas.TNSTextMetrics;
					public fillRect(param0: number, param1: number, param2: number, param3: number): void;
					public translate(param0: number, param1: number): void;
					public createImageData(param0: number, param1: number): com.github.triniwiz.canvas.TNSImageData;
					public getImageSmoothingQuality(): com.github.triniwiz.canvas.TNSImageSmoothingQuality;
					public arc(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public drawImage(param0: globalAndroid.graphics.Bitmap, param1: number, param2: number, param3: number, param4: number): void;
					public putImageData(param0: com.github.triniwiz.canvas.TNSImageData, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number): void;
					public getStrokeStyle(): com.github.triniwiz.canvas.TNSColorStyle;
					public scale(param0: number, param1: number): void;
					public getGlobalAlpha(): number;
					public setStrokeStyle(param0: com.github.triniwiz.canvas.TNSColorStyle): void;
					public quadraticCurveTo(param0: number, param1: number, param2: number, param3: number): void;
					public createPattern(param0: globalAndroid.graphics.Bitmap, param1: com.github.triniwiz.canvas.TNSPatternRepetition): com.github.triniwiz.canvas.TNSPattern;
					public getShadowOffsetY(): number;
					public transform(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public getShadowBlur(): number;
					public getLineJoin(): com.github.triniwiz.canvas.TNSLineJoin;
					public createPattern(param0: com.github.triniwiz.canvas.TNSImageAsset, param1: com.github.triniwiz.canvas.TNSPatternRepetition): com.github.triniwiz.canvas.TNSPattern;
					public getImageSmoothingEnabled(): boolean;
					public setFillStyle(param0: com.github.triniwiz.canvas.TNSColorStyle): void;
					public getFont(): string;
					public stroke(param0: com.github.triniwiz.canvas.TNSPath2D): void;
					public putImageData(param0: com.github.triniwiz.canvas.TNSImageData, param1: number, param2: number): void;
					public drawImage(param0: com.github.triniwiz.canvas.TNSCanvas, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
					public putImageData(param0: com.github.triniwiz.canvas.TNSImageData, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public getLineDash(): native.Array<number>;
					public save(): void;
					public setDirection(param0: com.github.triniwiz.canvas.TNSTextDirection): void;
					public isPointInPath(param0: number, param1: number): boolean;
					public getLineWidth(): number;
					public fill(param0: com.github.triniwiz.canvas.TNSPath2D, param1: com.github.triniwiz.canvas.TNSFillRule): void;
					public setLineJoin(param0: com.github.triniwiz.canvas.TNSLineJoin): void;
					public createLinearGradient(param0: number, param1: number, param2: number, param3: number): com.github.triniwiz.canvas.TNSCanvasGradient;
					public setLineWidth(param0: number): void;
					public getMiterLimit(): number;
					public setFont(param0: string): void;
					public setLineDashOffset(param0: number): void;
					public setShadowColor(param0: string): void;
					public fill(param0: com.github.triniwiz.canvas.TNSFillRule): void;
					public getGlobalCompositeOperation(): com.github.triniwiz.canvas.TNSCompositeOperationType;
					public stroke(): void;
					public getShadowOffsetX(): number;
					public setCurrentTransform(param0: com.github.triniwiz.canvas.TNSCanvasDOMMatrix): void;
					public lineTo(param0: number, param1: number): void;
					public restore(): void;
					public closePath(): void;
					public clearRect(param0: number, param1: number, param2: number, param3: number): void;
					public drawImage(param0: com.github.triniwiz.canvas.TNSCanvas, param1: number, param2: number, param3: number, param4: number): void;
					public rect(param0: number, param1: number, param2: number, param3: number): void;
					public fill(): void;
					public ellipse(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number): void;
					public drawImage(param0: com.github.triniwiz.canvas.TNSImageAsset, param1: number, param2: number): void;
					public createImageData(param0: com.github.triniwiz.canvas.TNSImageData): com.github.triniwiz.canvas.TNSImageData;
					public fillText(param0: string, param1: number, param2: number, param3: number): void;
					public getShadowColor(): string;
					public setTextAlign(param0: com.github.triniwiz.canvas.TNSTextAlignment): void;
					public isPointInStroke(param0: com.github.triniwiz.canvas.TNSPath2D, param1: number, param2: number): boolean;
					public arcTo(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public isPointInStroke(param0: number, param1: number): boolean;
					public drawImage(param0: globalAndroid.graphics.Bitmap, param1: number, param2: number): void;
					public getLineDashOffset(): number;
					public putImageData(param0: com.github.triniwiz.canvas.TNSImageData, param1: number, param2: number, param3: number): void;
					public setGlobalAlpha(param0: number): void;
					public setImageSmoothingQuality(param0: com.github.triniwiz.canvas.TNSImageSmoothingQuality): void;
					public drawImage(param0: com.github.triniwiz.canvas.TNSImageAsset, param1: number, param2: number, param3: number, param4: number): void;
					public getLineCap(): com.github.triniwiz.canvas.TNSLineCap;
					public clip(param0: com.github.triniwiz.canvas.TNSFillRule): void;
					public clip(): void;
					public setTransform(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public arc(param0: number, param1: number, param2: number, param3: number, param4: number, param5: boolean): void;
					public ellipse(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: boolean): void;
					public rotate(param0: number): void;
					public isPointInPath(param0: number, param1: number, param2: com.github.triniwiz.canvas.TNSFillRule): boolean;
					public getFillStyle(): com.github.triniwiz.canvas.TNSColorStyle;
					public setShadowOffsetY(param0: number): void;
					public drawImage(param0: com.github.triniwiz.canvas.TNSImageAsset, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
					public moveTo(param0: number, param1: number): void;
					public setImageSmoothingEnabled(param0: boolean): void;
					public strokeText(param0: string, param1: number, param2: number): void;
					public getCurrentTransform(): com.github.triniwiz.canvas.TNSCanvasDOMMatrix;
					public resetTransform(): void;
					public setMiterLimit(param0: number): void;
					public fill(param0: com.github.triniwiz.canvas.TNSPath2D): void;
					public setShadowBlur(param0: number): void;
					public setLineDash(param0: native.Array<number>): void;
					public drawImage(param0: globalAndroid.graphics.Bitmap, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
					public setShadowOffsetX(param0: number): void;
					public isPointInPath(param0: com.github.triniwiz.canvas.TNSPath2D, param1: number, param2: number): boolean;
				}
				export module TNSCanvasRenderingContext2D {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvasRenderingContext2D.Companion>;
						public isDebug(): boolean;
						public setDebug(param0: boolean): void;
					}
					export class WhenMappings {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCanvasRenderingContext2D.WhenMappings>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSColor extends com.github.triniwiz.canvas.TNSColorStyle {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSColor>;
					public static Companion: com.github.triniwiz.canvas.TNSColor.Companion;
					public getColor(): string;
					public setColor(param0: string): void;
					public constructor(param0: string);
					public constructor();
					public getStyleType(): com.github.triniwiz.canvas.TNSColorStyleType;
					public constructor(param0: number);
				}
				export module TNSColor {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSColor.Companion>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export abstract class TNSColorStyle {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSColorStyle>;
					public static Companion: com.github.triniwiz.canvas.TNSColorStyle.Companion;
					public static nativeDestroy(param0: number): void;
					public constructor();
					public getStyleType(): com.github.triniwiz.canvas.TNSColorStyleType;
				}
				export module TNSColorStyle {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSColorStyle.Companion>;
						public nativeDestroy(param0: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSColorStyleType {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSColorStyleType>;
					public static Color: com.github.triniwiz.canvas.TNSColorStyleType;
					public static Gradient: com.github.triniwiz.canvas.TNSColorStyleType;
					public static Pattern: com.github.triniwiz.canvas.TNSColorStyleType;
					public toString(): string;
					public static values(): native.Array<com.github.triniwiz.canvas.TNSColorStyleType>;
					public static valueOf(param0: string): com.github.triniwiz.canvas.TNSColorStyleType;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSCompositeOperationType {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCompositeOperationType>;
					public static SourceOver: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static SourceIn: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static SourceOut: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static SourceAtop: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static DestinationOver: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static DestinationIn: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static DestinationOut: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static DestinationAtop: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Lighter: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Copy: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Xor: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Multiply: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Screen: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Overlay: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Darken: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Lighten: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static ColorDodge: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static ColorBurn: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static HardLight: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static SoftLight: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Difference: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Exclusion: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Hue: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Saturation: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Color: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Luminosity: com.github.triniwiz.canvas.TNSCompositeOperationType;
					public static Companion: com.github.triniwiz.canvas.TNSCompositeOperationType.Companion;
					public toString(): string;
					public toNative(): number;
					public static valueOf(param0: string): com.github.triniwiz.canvas.TNSCompositeOperationType;
					public isError$canvas_release(): boolean;
					public setError$canvas_release(param0: boolean): void;
					public static values(): native.Array<com.github.triniwiz.canvas.TNSCompositeOperationType>;
				}
				export module TNSCompositeOperationType {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSCompositeOperationType.Companion>;
						public fromNative(param0: number): com.github.triniwiz.canvas.TNSCompositeOperationType;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSFileReader {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSFileReader>;
					public static INSTANCE: com.github.triniwiz.canvas.TNSFileReader;
					public read(param0: string): native.Array<number>;
					public read(param0: java.io.File): native.Array<number>;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSFillRule {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSFillRule>;
					public static NonZero: com.github.triniwiz.canvas.TNSFillRule;
					public static EvenOdd: com.github.triniwiz.canvas.TNSFillRule;
					public static Companion: com.github.triniwiz.canvas.TNSFillRule.Companion;
					public toString(): string;
					public toNative(): number;
					public getValue(): number;
					public setValue(param0: number): void;
					public setRule(param0: string): void;
					public getRule(): string;
					public static values(): native.Array<com.github.triniwiz.canvas.TNSFillRule>;
					public static valueOf(param0: string): com.github.triniwiz.canvas.TNSFillRule;
				}
				export module TNSFillRule {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSFillRule.Companion>;
						public fromNative(param0: number): com.github.triniwiz.canvas.TNSFillRule;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSFramebufferAttachmentParameter {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSFramebufferAttachmentParameter>;
					public setRenderbuffer(param0: boolean): void;
					public constructor();
					public getValue(): number;
					public setTexture(param0: boolean): void;
					public setValue(param0: number): void;
					public isTexture(): boolean;
					public isRenderbuffer(): boolean;
					public constructor(param0: boolean, param1: boolean, param2: number);
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSImageAsset {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSImageAsset>;
					public static Companion: com.github.triniwiz.canvas.TNSImageAsset.Companion;
					public getBuffer(): java.nio.ByteBuffer;
					public loadImageFromBuffer(param0: java.nio.ByteBuffer): boolean;
					public loadImageFromBytesAsync(param0: native.Array<number>, param1: com.github.triniwiz.canvas.TNSImageAsset.Callback): void;
					public flipX(): void;
					public getHeight(): number;
					public finalize(): void;
					public getWidth(): number;
					public loadImageFromPath(param0: string): boolean;
					public loadImageFromImage(param0: globalAndroid.graphics.Bitmap): boolean;
					public save(param0: string, param1: com.github.triniwiz.canvas.TNSImageAssetFormat): boolean;
					public getNativeImageAsset$canvas_release(): number;
					public loadImageFromResource(param0: number, param1: globalAndroid.content.Context): boolean;
					public loadImageFromImageAsync(param0: globalAndroid.graphics.Bitmap, param1: com.github.triniwiz.canvas.TNSImageAsset.Callback): void;
					public getBytes(): native.Array<number>;
					public getError(): string;
					public flipY(): void;
					public scale(param0: number, param1: number): void;
					public constructor();
					public loadImageFromBufferAsync(param0: java.nio.ByteBuffer, param1: com.github.triniwiz.canvas.TNSImageAsset.Callback): void;
					public loadImageFromPathAsync(param0: string, param1: com.github.triniwiz.canvas.TNSImageAsset.Callback): void;
					public saveAsync(param0: string, param1: com.github.triniwiz.canvas.TNSImageAssetFormat, param2: com.github.triniwiz.canvas.TNSImageAsset.Callback): void;
					public loadImageFromBytes(param0: native.Array<number>): boolean;
					public setNativeImageAsset$canvas_release(param0: number): void;
				}
				export module TNSImageAsset {
					export class Callback {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSImageAsset.Callback>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.TNSImageAsset$Callback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onSuccess(param0: any): void;
							onError(param0: string): void;
						});
						public constructor();
						public onSuccess(param0: any): void;
						public onError(param0: string): void;
					}
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSImageAsset.Companion>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSImageAssetFormat {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSImageAssetFormat>;
					public static JPG: com.github.triniwiz.canvas.TNSImageAssetFormat;
					public static PNG: com.github.triniwiz.canvas.TNSImageAssetFormat;
					public static ICO: com.github.triniwiz.canvas.TNSImageAssetFormat;
					public static BMP: com.github.triniwiz.canvas.TNSImageAssetFormat;
					public static TIFF: com.github.triniwiz.canvas.TNSImageAssetFormat;
					public getFormat(): number;
					public static values(): native.Array<com.github.triniwiz.canvas.TNSImageAssetFormat>;
					public setFormat(param0: number): void;
					public static valueOf(param0: string): com.github.triniwiz.canvas.TNSImageAssetFormat;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSImageData {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSImageData>;
					public static Companion: com.github.triniwiz.canvas.TNSImageData.Companion;
					public getNativeImageData$canvas_release(): number;
					public getHeight(): number;
					public finalize(): void;
					public constructor(param0: number, param1: number, param2: number);
					public getWidth(): number;
					public getData(): java.nio.ByteBuffer;
					public setNativeImageData$canvas_release(param0: number): void;
				}
				export module TNSImageData {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSImageData.Companion>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSImageSmoothingQuality {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSImageSmoothingQuality>;
					public static Low: com.github.triniwiz.canvas.TNSImageSmoothingQuality;
					public static Medium: com.github.triniwiz.canvas.TNSImageSmoothingQuality;
					public static High: com.github.triniwiz.canvas.TNSImageSmoothingQuality;
					public static Companion: com.github.triniwiz.canvas.TNSImageSmoothingQuality.Companion;
					public static valueOf(param0: string): com.github.triniwiz.canvas.TNSImageSmoothingQuality;
					public toString(): string;
					public static values(): native.Array<com.github.triniwiz.canvas.TNSImageSmoothingQuality>;
					public toNative(): number;
					public isError$canvas_release(): boolean;
					public setError$canvas_release(param0: boolean): void;
				}
				export module TNSImageSmoothingQuality {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSImageSmoothingQuality.Companion>;
						public fromNative(param0: number): com.github.triniwiz.canvas.TNSImageSmoothingQuality;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSLineCap {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSLineCap>;
					public static Butt: com.github.triniwiz.canvas.TNSLineCap;
					public static Round: com.github.triniwiz.canvas.TNSLineCap;
					public static Square: com.github.triniwiz.canvas.TNSLineCap;
					public static Companion: com.github.triniwiz.canvas.TNSLineCap.Companion;
					public toString(): string;
					public toNative(): number;
					public static valueOf(param0: string): com.github.triniwiz.canvas.TNSLineCap;
					public isError$canvas_release(): boolean;
					public setError$canvas_release(param0: boolean): void;
					public static values(): native.Array<com.github.triniwiz.canvas.TNSLineCap>;
				}
				export module TNSLineCap {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSLineCap.Companion>;
						public fromNative(param0: number): com.github.triniwiz.canvas.TNSLineCap;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSLineJoin {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSLineJoin>;
					public static Bevel: com.github.triniwiz.canvas.TNSLineJoin;
					public static Round: com.github.triniwiz.canvas.TNSLineJoin;
					public static Miter: com.github.triniwiz.canvas.TNSLineJoin;
					public static Companion: com.github.triniwiz.canvas.TNSLineJoin.Companion;
					public static values(): native.Array<com.github.triniwiz.canvas.TNSLineJoin>;
					public toString(): string;
					public toNative(): number;
					public isError$canvas_release(): boolean;
					public setError$canvas_release(param0: boolean): void;
					public static valueOf(param0: string): com.github.triniwiz.canvas.TNSLineJoin;
				}
				export module TNSLineJoin {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSLineJoin.Companion>;
						public fromNative(param0: number): com.github.triniwiz.canvas.TNSLineJoin;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSPath2D {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSPath2D>;
					public static Companion: com.github.triniwiz.canvas.TNSPath2D.Companion;
					public addPath(param0: com.github.triniwiz.canvas.TNSPath2D, param1: com.github.triniwiz.canvas.TNSCanvasDOMMatrix): void;
					public moveTo(param0: number, param1: number): void;
					public rect(param0: number, param1: number, param2: number, param3: number): void;
					public bezierCurveTo(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public constructor(param0: string);
					public finalize(): void;
					public getPath$canvas_release(): number;
					public arc(param0: number, param1: number, param2: number, param3: number, param4: number, param5: boolean): void;
					public ellipse(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: boolean): void;
					public constructor();
					public setPath$canvas_release(param0: number): void;
					public arcTo(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public quadraticCurveTo(param0: number, param1: number, param2: number, param3: number): void;
					public constructor(param0: com.github.triniwiz.canvas.TNSPath2D);
					public lineTo(param0: number, param1: number): void;
					public closePath(): void;
				}
				export module TNSPath2D {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSPath2D.Companion>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSPattern extends com.github.triniwiz.canvas.TNSColorStyle {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSPattern>;
					public static Companion: com.github.triniwiz.canvas.TNSPattern.Companion;
					public getStyle(): number;
					public constructor();
					public setTransform(param0: com.github.triniwiz.canvas.TNSCanvasDOMMatrix): void;
					public finalize(): void;
					public getStyleType(): com.github.triniwiz.canvas.TNSColorStyleType;
					public constructor(param0: number);
					public setStyle(param0: number): void;
				}
				export module TNSPattern {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSPattern.Companion>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSPatternRepetition {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSPatternRepetition>;
					public static Repeat: com.github.triniwiz.canvas.TNSPatternRepetition;
					public static RepeatX: com.github.triniwiz.canvas.TNSPatternRepetition;
					public static RepeatY: com.github.triniwiz.canvas.TNSPatternRepetition;
					public static NoRepeat: com.github.triniwiz.canvas.TNSPatternRepetition;
					public toString(): string;
					public static valueOf(param0: string): com.github.triniwiz.canvas.TNSPatternRepetition;
					public static values(): native.Array<com.github.triniwiz.canvas.TNSPatternRepetition>;
					public getPattern(): string;
					public toNative(): number;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSTextAlignment {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSTextAlignment>;
					public static Start: com.github.triniwiz.canvas.TNSTextAlignment;
					public static Left: com.github.triniwiz.canvas.TNSTextAlignment;
					public static Center: com.github.triniwiz.canvas.TNSTextAlignment;
					public static Right: com.github.triniwiz.canvas.TNSTextAlignment;
					public static End: com.github.triniwiz.canvas.TNSTextAlignment;
					public static Companion: com.github.triniwiz.canvas.TNSTextAlignment.Companion;
					public toString(): string;
					public toNative(): number;
					public static valueOf(param0: string): com.github.triniwiz.canvas.TNSTextAlignment;
					public static values(): native.Array<com.github.triniwiz.canvas.TNSTextAlignment>;
					public isError$canvas_release(): boolean;
					public setError$canvas_release(param0: boolean): void;
				}
				export module TNSTextAlignment {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSTextAlignment.Companion>;
						public fromNative(param0: number): com.github.triniwiz.canvas.TNSTextAlignment;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSTextBaseline {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSTextBaseline>;
					public static Top: com.github.triniwiz.canvas.TNSTextBaseline;
					public static Hanging: com.github.triniwiz.canvas.TNSTextBaseline;
					public static Middle: com.github.triniwiz.canvas.TNSTextBaseline;
					public static Alphabetic: com.github.triniwiz.canvas.TNSTextBaseline;
					public static Ideographic: com.github.triniwiz.canvas.TNSTextBaseline;
					public static Bottom: com.github.triniwiz.canvas.TNSTextBaseline;
					public toString(): string;
					public static valueOf(param0: string): com.github.triniwiz.canvas.TNSTextBaseline;
					public getBaseLine(): string;
					public setBaseLine(param0: string): void;
					public static values(): native.Array<com.github.triniwiz.canvas.TNSTextBaseline>;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSTextDecoder {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSTextDecoder>;
					public static Companion: com.github.triniwiz.canvas.TNSTextDecoder.Companion;
					public constructor(param0: string);
					public constructor();
					public finalize(): void;
					public getEncoding(): string;
					public decode(param0: native.Array<number>): string;
				}
				export module TNSTextDecoder {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSTextDecoder.Companion>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSTextDirection {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSTextDirection>;
					public static Ltr: com.github.triniwiz.canvas.TNSTextDirection;
					public static Rtl: com.github.triniwiz.canvas.TNSTextDirection;
					public static valueOf(param0: string): com.github.triniwiz.canvas.TNSTextDirection;
					public toString(): string;
					public toNative(): number;
					public static values(): native.Array<com.github.triniwiz.canvas.TNSTextDirection>;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSTextEncoder {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSTextEncoder>;
					public static Companion: com.github.triniwiz.canvas.TNSTextEncoder.Companion;
					public constructor(param0: string);
					public constructor();
					public finalize(): void;
					public getEncoding(): string;
					public encode(param0: string): java.nio.ByteBuffer;
				}
				export module TNSTextEncoder {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSTextEncoder.Companion>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSTextMetrics {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSTextMetrics>;
					public static Companion: com.github.triniwiz.canvas.TNSTextMetrics.Companion;
					public getEmHeightAscent(): number;
					public getIdeographicBaseline(): number;
					public getAlphabeticBaseline(): number;
					public finalize(): void;
					public getWidth(): number;
					public getActualBoundingBoxDescent(): number;
					public getActualBoundingBoxRight(): number;
					public getActualBoundingBoxAscent(): number;
					public getFontBoundingBoxAscent(): number;
					public getEmHeightDescent(): number;
					public getHangingBaseline(): number;
					public getFontBoundingBoxDescent(): number;
					public constructor(param0: number);
					public getActualBoundingBoxLeft(): number;
				}
				export module TNSTextMetrics {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSTextMetrics.Companion>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSWebGL2RenderingContext extends com.github.triniwiz.canvas.TNSWebGLRenderingContext {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSWebGL2RenderingContext>;
					public static Companion: com.github.triniwiz.canvas.TNSWebGL2RenderingContext.Companion;
					public beginQuery(param0: number, param1: number): void;
					public getINTERLEAVED_ATTRIBS(): number;
					public getUniformBlockIndex(param0: number, param1: string): number;
					public getDRAW_BUFFER11(): number;
					public getHALF_FLOAT(): number;
					public getUNPACK_SKIP_ROWS(): number;
					public getSyncParameter(param0: number, param1: number): any;
					public getMAX_PROGRAM_TEXEL_OFFSET(): number;
					public getUNIFORM_BUFFER_OFFSET_ALIGNMENT(): number;
					public isVertexArray(param0: number): boolean;
					public getUNPACK_SKIP_PIXELS(): number;
					public deleteVertexArray(param0: number): void;
					public getCOLOR_ATTACHMENT3(): number;
					public getRG8(): number;
					public getINT_2_10_10_10_REV(): number;
					public bindSampler(param0: number, param1: number): void;
					public getRGBA32I(): number;
					public getFLOAT_MAT4x2(): number;
					public getMAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS(): number;
					public getDRAW_BUFFER5(): number;
					public getTRANSFORM_FEEDBACK_BUFFER_MODE(): number;
					public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: number): void;
					public getSTENCIL(): number;
					public getUNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER(): number;
					public deleteTransformFeedback(param0: number): void;
					public getFRAMEBUFFER_ATTACHMENT_GREEN_SIZE(): number;
					public getMAX_VARYING_COMPONENTS(): number;
					public getCOLOR_ATTACHMENT9(): number;
					public getRGB8(): number;
					public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: native.Array<number>): void;
					public drawRangeElements(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public isQuery(param0: number): boolean;
					public getPACK_SKIP_PIXELS(): number;
					public getBufferSubData(param0: number, param1: number, param2: native.Array<number>, param3: number, param4: number): void;
					public getTRANSFORM_FEEDBACK_BUFFER_BINDING(): number;
					public getMAX_COMBINED_VERTEX_UNIFORM_COMPONENTS(): number;
					public getRGB32F(): number;
					public getFRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER(): number;
					public getMAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS(): number;
					public getUNSIGNED_INT_SAMPLER_2D(): number;
					public getActiveUniformBlockName(param0: number, param1: number): string;
					public getMAX_TEXTURE_LOD_BIAS(): number;
					public getUNSIGNED_INT_SAMPLER_3D(): number;
					public getSAMPLER_2D_ARRAY(): number;
					public getDEPTH(): number;
					public getFRAMEBUFFER_ATTACHMENT_COLOR_ENCODING(): number;
					public getFRAMEBUFFER_ATTACHMENT_DEPTH_SIZE(): number;
					public getINT_SAMPLER_2D_ARRAY(): number;
					public getTRANSFORM_FEEDBACK_BUFFER(): number;
					public getPIXEL_PACK_BUFFER_BINDING(): number;
					public getTIMEOUT_EXPIRED(): number;
					public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: globalAndroid.graphics.Bitmap): void;
					public getRED(): number;
					public getActiveUniforms(param0: number, param1: native.Array<number>, param2: number): any;
					public getRG8_SNORM(): number;
					public getUNSIGNED_INT_VEC2(): number;
					public getFLOAT_MAT3x2(): number;
					public getUNPACK_IMAGE_HEIGHT(): number;
					public getMAX_DRAW_BUFFERS(): number;
					public getSTREAM_READ(): number;
					public samplerParameterf(param0: number, param1: number, param2: number): void;
					public getTEXTURE_WRAP_R(): number;
					public getR32UI(): number;
					public getSTATIC_READ(): number;
					public getRGBA_INTEGER(): number;
					public getTEXTURE_COMPARE_MODE(): number;
					public getRG16F(): number;
					public getInternalformatParameter(param0: number, param1: number, param2: number): any;
					public getTRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN(): number;
					public getR16I(): number;
					public getINT_SAMPLER_2D(): number;
					public getCOPY_WRITE_BUFFER(): number;
					public getFRAMEBUFFER_DEFAULT(): number;
					public getMAX_UNIFORM_BLOCK_SIZE(): number;
					public deleteQuery(param0: number): void;
					public vertexAttribI4i(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public getR8UI(): number;
					public getUNIFORM_IS_ROW_MAJOR(): number;
					public getINT_SAMPLER_3D(): number;
					public getMAX_VERTEX_OUTPUT_COMPONENTS(): number;
					public getDRAW_FRAMEBUFFER(): number;
					public getSRGB(): number;
					public getRG32I(): number;
					public getMAX_VERTEX_UNIFORM_BLOCKS(): number;
					public getFLOAT_MAT3x4(): number;
					public uniform3uiv(param0: number, param1: native.Array<number>): void;
					public getTRANSFORM_FEEDBACK_BUFFER_SIZE(): number;
					public getMAX_FRAGMENT_UNIFORM_BLOCKS(): number;
					public drawElementsInstanced(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public clearBufferfv(param0: number, param1: number, param2: native.Array<number>): void;
					public uniform3ui(param0: number, param1: number, param2: number, param3: number): void;
					public uniformMatrix2x4fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public getFRAMEBUFFER_ATTACHMENT_ALPHA_SIZE(): number;
					public getRGB32UI(): number;
					public createVertexArray(): number;
					public vertexAttribI4uiv(param0: number, param1: native.Array<number>): void;
					public getRGB16I(): number;
					public getMAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS(): number;
					public getActiveUniformBlockParameter(param0: number, param1: number, param2: number): any;
					public getUNIFORM_BLOCK_BINDING(): number;
					public uniformMatrix3x4fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public getRGB8I(): number;
					public deleteSync(param0: number): void;
					public getRGBA8UI(): number;
					public getRASTERIZER_DISCARD(): number;
					public getPIXEL_UNPACK_BUFFER_BINDING(): number;
					public getSAMPLER_3D(): number;
					public blitFramebuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number): void;
					public getTRANSFORM_FEEDBACK_VARYINGS(): number;
					public getSYNC_STATUS(): number;
					public getSYNC_FENCE(): number;
					public transformFeedbackVaryings(param0: number, param1: native.Array<string>, param2: number): void;
					public getDRAW_FRAMEBUFFER_BINDING(): number;
					public getUNSIGNALED(): number;
					public getRGBA8I(): number;
					public getCOLOR_ATTACHMENT2(): number;
					public getPIXEL_PACK_BUFFER(): number;
					public getPIXEL_UNPACK_BUFFER(): number;
					public getDRAW_BUFFER9(): number;
					public getQUERY_RESULT_AVAILABLE(): number;
					public uniformMatrix3x2fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public getQueryParameter(param0: number, param1: number): any;
					public getUNIFORM_SIZE(): number;
					public getTEXTURE_3D(): number;
					public drawBuffers(param0: native.Array<number>): void;
					public getTEXTURE_MAX_LEVEL(): number;
					public getCOPY_READ_BUFFER(): number;
					public vertexAttribI4ui(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public uniformMatrix4x3fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public getTRANSFORM_FEEDBACK_ACTIVE(): number;
					public getUNIFORM_BUFFER(): number;
					public getDRAW_BUFFER3(): number;
					public getDEPTH_COMPONENT32F(): number;
					public deleteSampler(param0: number): void;
					public uniformMatrix4x2fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public getUNIFORM_BLOCK_ACTIVE_UNIFORMS(): number;
					public pauseTransformFeedback(): void;
					public getFRAMEBUFFER_INCOMPLETE_MULTISAMPLE(): number;
					public getUNIFORM_BUFFER_BINDING(): number;
					public getUNSIGNED_INT_5_9_9_9_REV(): number;
					public getOBJECT_TYPE(): number;
					public getRGB9_E5(): number;
					public getRG8I(): number;
					public getRGBA16I(): number;
					public clearBufferiv(param0: number, param1: number, param2: native.Array<number>): void;
					public getTEXTURE_MIN_LOD(): number;
					public getDRAW_BUFFER12(): number;
					public getCOLOR_ATTACHMENT8(): number;
					public uniform2ui(param0: number, param1: number, param2: number): void;
					public resumeTransformFeedback(): void;
					public getCOLOR_ATTACHMENT4(): number;
					public getRGB8_SNORM(): number;
					public getTRANSFORM_FEEDBACK_PAUSED(): number;
					public getRGBA8_SNORM(): number;
					public framebufferTextureLayer(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public getDRAW_BUFFER7(): number;
					public getCOPY_WRITE_BUFFER_BINDING(): number;
					public getFLOAT_MAT4x3(): number;
					public getUNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES(): number;
					public getMAX_FRAGMENT_INPUT_COMPONENTS(): number;
					public getTEXTURE_BASE_LEVEL(): number;
					public getCOLOR_ATTACHMENT14(): number;
					public samplerParameteri(param0: number, param1: number, param2: number): void;
					public getUNSIGNED_INT_2_10_10_10_REV(): number;
					public getDRAW_BUFFER10(): number;
					public getSAMPLER_BINDING(): number;
					public getMAX_UNIFORM_BUFFER_BINDINGS(): number;
					public getR11F_G11F_B10F(): number;
					public getFLOAT_32_UNSIGNED_INT_24_8_REV(): number;
					public getSRGB8(): number;
					public createTransformFeedback(): number;
					public getMIN(): number;
					public getTEXTURE_BINDING_2D_ARRAY(): number;
					public getRENDERBUFFER_SAMPLES(): number;
					public getUNIFORM_BUFFER_SIZE(): number;
					public getCOPY_READ_BUFFER_BINDING(): number;
					public getMAX_3D_TEXTURE_SIZE(): number;
					public clearBufferfi(param0: number, param1: number, param2: number, param3: number): void;
					public compressedTexSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: number): void;
					public renderbufferStorageMultisample(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public uniform1ui(param0: number, param1: number): void;
					public getFRAGMENT_SHADER_DERIVATIVE_HINT(): number;
					public getRGB8UI(): number;
					public getCOLOR_ATTACHMENT12(): number;
					public getTransformFeedbackVarying(param0: number, param1: number): any;
					public copyTexSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
					public isTransformFeedback(param0: number): boolean;
					public getUNPACK_ROW_LENGTH(): number;
					public getCONDITION_SATISFIED(): number;
					public getR8_SNORM(): number;
					public getRG_INTEGER(): number;
					public getUNIFORM_TYPE(): number;
					public getRGBA32F(): number;
					public getSYNC_CONDITION(): number;
					public fenceSync(param0: number, param1: number): void;
					public bindBufferBase(param0: number, param1: number, param2: number): void;
					public clientWaitSync(param0: number, param1: number, param2: number): number;
					public getFLOAT_MAT2x4(): number;
					public getCOLOR_ATTACHMENT10(): number;
					public endTransformFeedback(): void;
					public getWAIT_FAILED(): number;
					public getANY_SAMPLES_PASSED_CONSERVATIVE(): number;
					public getUNSIGNED_INT_10F_11F_11F_REV(): number;
					public getVERTEX_ARRAY_BINDING(): number;
					public getIndexedParameter(param0: number, param1: number): any;
					public getSYNC_FLAGS(): number;
					public getSTREAM_COPY(): number;
					public uniform2uiv(param0: number, param1: native.Array<number>): void;
					public getTRANSFORM_FEEDBACK_BUFFER_START(): number;
					public constructor(param0: com.github.triniwiz.canvas.TNSCanvas, param1: java.util.Map<string,any>);
					public getDRAW_BUFFER8(): number;
					public constructor(param0: com.github.triniwiz.canvas.TNSCanvas);
					public getDRAW_BUFFER14(): number;
					public getRG32UI(): number;
					public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number): void;
					public invalidateFramebuffer(param0: number, param1: native.Array<number>): void;
					public getCOLOR_ATTACHMENT6(): number;
					public getMAX(): number;
					public getDEPTH_COMPONENT24(): number;
					public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: com.github.triniwiz.canvas.TNSImageAsset): void;
					public getUNSIGNED_INT_SAMPLER_2D_ARRAY(): number;
					public readBuffer(param0: number): void;
					public getDRAW_BUFFER2(): number;
					public getUNIFORM_MATRIX_STRIDE(): number;
					public getSAMPLER_2D_SHADOW(): number;
					public getR16F(): number;
					public getUNSIGNED_INT_VEC4(): number;
					public getTEXTURE_MAX_LOD(): number;
					public getUniformIndices(param0: number, param1: native.Array<string>): native.Array<number>;
					public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: globalAndroid.graphics.Bitmap): void;
					public getRGBA32UI(): number;
					public getSTATIC_COPY(): number;
					public getINVALID_INDEX(): number;
					public getMAX_SERVER_WAIT_TIMEOUT(): number;
					public getPACK_SKIP_ROWS(): number;
					public getSamplerParameter(param0: number, param1: number): any;
					public drawArraysInstanced(param0: number, param1: number, param2: number, param3: number): void;
					public clearBufferuiv(param0: number, param1: number, param2: native.Array<number>): void;
					public getREAD_BUFFER(): number;
					public getTEXTURE_COMPARE_FUNC(): number;
					public compressedTexSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: native.Array<number>, param10: number, param11: number): void;
					public getMAX_FRAGMENT_UNIFORM_COMPONENTS(): number;
					public endQuery(param0: number): void;
					public getUNSIGNED_NORMALIZED(): number;
					public uniformBlockBinding(param0: number, param1: number, param2: number): void;
					public isSync(param0: number): boolean;
					public getSRGB8_ALPHA8(): number;
					public createSampler(): number;
					public getRG32F(): number;
					public getDRAW_BUFFER1(): number;
					public getSEPARATE_ATTRIBS(): number;
					public getRGB16UI(): number;
					public texStorage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public getSYNC_GPU_COMMANDS_COMPLETE(): number;
					public getRG16I(): number;
					public getUNIFORM_ARRAY_STRIDE(): number;
					public getRGB_INTEGER(): number;
					public getRED_INTEGER(): number;
					public getSYNC_FLUSH_COMMANDS_BIT(): number;
					public getVERTEX_ATTRIB_ARRAY_INTEGER(): number;
					public getR32I(): number;
					public beginTransformFeedback(param0: number): void;
					public getRG8UI(): number;
					public getDRAW_BUFFER15(): number;
					public getDEPTH32F_STENCIL8(): number;
					public getMAX_COMBINED_UNIFORM_BLOCKS(): number;
					public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: com.github.triniwiz.canvas.TNSImageAsset): void;
					public getMAX_ELEMENT_INDEX(): number;
					public getTIMEOUT_IGNORED(): number;
					public getMAX_COLOR_ATTACHMENTS(): number;
					public getQuery(param0: number, param1: number): any;
					public getMAX_ARRAY_TEXTURE_LAYERS(): number;
					public getRGB10_A2(): number;
					public getRGBA16UI(): number;
					public getDYNAMIC_READ(): number;
					public getRGBA16F(): number;
					public getFRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE(): number;
					public getCOMPARE_REF_TO_TEXTURE(): number;
					public uniformMatrix2x3fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public getFRAMEBUFFER_ATTACHMENT_STENCIL_SIZE(): number;
					public bindVertexArray(param0: number): void;
					public getRGB16F(): number;
					public getUNSIGNED_INT_24_8(): number;
					public getDRAW_BUFFER0(): number;
					public getUNIFORM_BLOCK_DATA_SIZE(): number;
					public getFRAMEBUFFER_ATTACHMENT_RED_SIZE(): number;
					public getDEPTH24_STENCIL8(): number;
					public getANY_SAMPLES_PASSED(): number;
					public getMAX_ELEMENTS_VERTICES(): number;
					public copyBufferSubData(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public uniform1uiv(param0: number, param1: native.Array<number>): void;
					public getFragDataLocation(param0: number, param1: string): number;
					public getR16UI(): number;
					public bindTransformFeedback(param0: number, param1: number): void;
					public getCOLOR_ATTACHMENT15(): number;
					public getR32F(): number;
					public getTEXTURE_BINDING_3D(): number;
					public texStorage2D(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public getTEXTURE_IMMUTABLE_LEVELS(): number;
					public getUNSIGNED_INT_SAMPLER_CUBE(): number;
					public getDRAW_BUFFER13(): number;
					public getR8I(): number;
					public getDYNAMIC_COPY(): number;
					public getSIGNALED(): number;
					public getQUERY_RESULT(): number;
					public getUNIFORM_BLOCK_INDEX(): number;
					public getPACK_ROW_LENGTH(): number;
					public getUNSIGNED_INT_VEC3(): number;
					public getMAX_SAMPLES(): number;
					public getSIGNED_NORMALIZED(): number;
					public getFRAMEBUFFER_ATTACHMENT_BLUE_SIZE(): number;
					public getINT_SAMPLER_CUBE(): number;
					public getREAD_FRAMEBUFFER_BINDING(): number;
					public vertexAttribI4iv(param0: number, param1: native.Array<number>): void;
					public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: native.Array<number>, param11: number): void;
					public getDRAW_BUFFER4(): number;
					public getFLOAT_MAT2x3(): number;
					public getTEXTURE_IMMUTABLE_FORMAT(): number;
					public getCOLOR_ATTACHMENT1(): number;
					public getACTIVE_UNIFORM_BLOCKS(): number;
					public getCOLOR_ATTACHMENT7(): number;
					public isSampler(param0: number): boolean;
					public getCOLOR_ATTACHMENT11(): number;
					public uniform4uiv(param0: number, param1: native.Array<number>): void;
					public getUNIFORM_OFFSET(): number;
					public getRG16UI(): number;
					public getSAMPLER_2D_ARRAY_SHADOW(): number;
					public getVERTEX_ATTRIB_ARRAY_DIVISOR(): number;
					public getREAD_FRAMEBUFFER(): number;
					public getRGB10_A2UI(): number;
					public getTEXTURE_2D_ARRAY(): number;
					public getMAX_CLIENT_WAIT_TIMEOUT_WEBGL(): number;
					public vertexAttribDivisor(param0: number, param1: number): void;
					public getSAMPLER_CUBE_SHADOW(): number;
					public getTRANSFORM_FEEDBACK(): number;
					public getTRANSFORM_FEEDBACK_BINDING(): number;
					public getMAX_ELEMENTS_INDICES(): number;
					public getMIN_PROGRAM_TEXEL_OFFSET(): number;
					public getRG(): number;
					public getRGBA8(): number;
					public invalidateSubFramebuffer(param0: number, param1: native.Array<number>, param2: number, param3: number, param4: number, param5: number): void;
					public getCOLOR(): number;
					public getRGB32I(): number;
					public getCOLOR_ATTACHMENT5(): number;
					public uniform4ui(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public bindBufferRange(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public getCURRENT_QUERY(): number;
					public fromGLint(param0: native.Array<number>): native.Array<boolean>;
					public getALREADY_SIGNALED(): number;
					public createQuery(): number;
					public getMAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS(): number;
					public getMAX_VERTEX_UNIFORM_COMPONENTS(): number;
					public getDRAW_BUFFER6(): number;
					public getCOLOR_ATTACHMENT13(): number;
					public getUNPACK_SKIP_IMAGES(): number;
					public getUNIFORM_BUFFER_START(): number;
					public getR8(): number;
					public getUNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER(): number;
				}
				export module TNSWebGL2RenderingContext {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSWebGL2RenderingContext.Companion>;
					}
					export class ReturnType {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSWebGL2RenderingContext.ReturnType>;
						public static EnumType: com.github.triniwiz.canvas.TNSWebGL2RenderingContext.ReturnType;
						public static UnsignedIntType: com.github.triniwiz.canvas.TNSWebGL2RenderingContext.ReturnType;
						public static IntType: com.github.triniwiz.canvas.TNSWebGL2RenderingContext.ReturnType;
						public static BoolType: com.github.triniwiz.canvas.TNSWebGL2RenderingContext.ReturnType;
						public static valueOf(param0: string): com.github.triniwiz.canvas.TNSWebGL2RenderingContext.ReturnType;
						public static values(): native.Array<com.github.triniwiz.canvas.TNSWebGL2RenderingContext.ReturnType>;
					}
					export class WhenMappings {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSWebGL2RenderingContext.WhenMappings>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TNSWebGLRenderingContext extends com.github.triniwiz.canvas.TNSCanvasRenderingContext {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TNSWebGLRenderingContext>;
					public static SIZE_OF_BYTE: number;
					public static SIZE_OF_SHORT: number;
					public static SIZE_OF_INT: number;
					public static SIZE_OF_LONG: number;
					public static SIZE_OF_FLOAT: number;
					public static SIZE_OF_DOUBLE: number;
					public static SIZE_OF_CHAR: number;
					public static Companion: com.github.triniwiz.canvas.TNSWebGLRenderingContext.Companion;
					public uniform1iv(param0: number, param1: native.Array<number>): void;
					public bindFramebuffer(param0: number, param1: number): void;
					public vertexAttrib3fv(param0: number, param1: native.Array<number>): void;
					public getFRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL(): number;
					public getNONE(): number;
					public getSRC_ALPHA(): number;
					public getACTIVE_TEXTURE(): number;
					public getGREEN_BITS(): number;
					public stencilOpSeparate(param0: number, param1: number, param2: number, param3: number): void;
					public getCLAMP_TO_EDGE(): number;
					public getINVALID_OPERATION(): number;
					public getVALIDATE_STATUS(): number;
					public getARRAY_BUFFER(): number;
					public commit(): void;
					public flush(): void;
					public isProgram(param0: number): boolean;
					public getUNSIGNED_SHORT(): number;
					public createShader(param0: number): number;
					public getActiveAttrib(param0: number, param1: number): com.github.triniwiz.canvas.WebGLActiveInfo;
					public getCOMPRESSED_TEXTURE_FORMATS(): number;
					public compileShader(param0: number): void;
					public isEnabled(param0: number): boolean;
					public blendFuncSeparate(param0: number, param1: number, param2: number, param3: number): void;
					public getLINE_STRIP(): number;
					public getPOLYGON_OFFSET_FILL(): number;
					public getDEPTH_STENCIL(): number;
					public getRENDERBUFFER_ALPHA_SIZE(): number;
					public getFLOAT_MAT2(): number;
					public getSTENCIL_BACK_VALUE_MASK(): number;
					public getBufferParameter(param0: number, param1: number): number;
					public getBLEND_DST_ALPHA(): number;
					public getTEXTURE21(): number;
					public getTEXTURE31(): number;
					public getTEXTURE_CUBE_MAP_NEGATIVE_Z(): number;
					public getTEXTURE9(): number;
					public getTEXTURE11(): number;
					public getVERTEX_ATTRIB_ARRAY_POINTER(): number;
					public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: com.github.triniwiz.canvas.TNSCanvas): void;
					public getFRAGMENT_SHADER(): number;
					public getONE_MINUS_SRC_ALPHA(): number;
					public getSTENCIL_PASS_DEPTH_PASS(): number;
					public enable(param0: number): void;
					public getINCR(): number;
					public stencilFuncSeparate(param0: number, param1: number, param2: number, param3: number): void;
					public texParameteri(param0: number, param1: number, param2: number): void;
					public getALPHA(): number;
					public vertexAttrib4f(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public getExtension(param0: string): any;
					public getFUNC_SUBTRACT(): number;
					public getBOOL_VEC4(): number;
					public getFASTEST(): number;
					public getDYNAMIC_DRAW(): number;
					public getDECR(): number;
					public deleteFramebuffer(param0: number): void;
					public getTEXTURE_CUBE_MAP_NEGATIVE_X(): number;
					public getSTENCIL_WRITEMASK(): number;
					public getONE(): number;
					public getRenderbufferParameter(param0: number, param1: number): number;
					public getVERTEX_ATTRIB_ARRAY_TYPE(): number;
					public getTEXTURE18(): number;
					public getAttribLocation(param0: number, param1: string): number;
					public getTEXTURE2(): number;
					public generateMipmap(param0: number): void;
					public getMAX_FRAGMENT_UNIFORM_VECTORS(): number;
					public getVertexAttrib(param0: number, param1: number): any;
					public stencilMask(param0: number): void;
					public getTEXTURE(): number;
					public linkProgram(param0: number): void;
					public getFLOAT_MAT4(): number;
					public getTEXTURE_BINDING_2D(): number;
					public getHIGH_FLOAT(): number;
					public getBLEND_COLOR(): number;
					public getTEXTURE_WRAP_S(): number;
					public getDEPTH_TEST(): number;
					public uniformMatrix4fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public getVERTEX_ATTRIB_ARRAY_NORMALIZED(): number;
					public getParameter(param0: number): any;
					public getCOLOR_BUFFER_BIT(): number;
					public uniformMatrix3fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public getAttachedShaders(param0: number): native.Array<number>;
					public isBuffer(param0: number): boolean;
					public bufferData(param0: number, param1: number, param2: number): void;
					public getMAX_TEXTURE_SIZE(): number;
					public getTEXTURE4(): number;
					public getTEXTURE16(): number;
					public uniformMatrix2fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public getGL_UNSIGNED_SHORT_4_4_4_4$canvas_release(): number;
					public getFUNC_ADD(): number;
					public getTEXTURE_CUBE_MAP_POSITIVE_Y(): number;
					public getONE_MINUS_SRC_COLOR(): number;
					public finish(): void;
					public setFlipYWebGL(param0: boolean): void;
					public getINVALID_VALUE(): number;
					public getFRAMEBUFFER_BINDING(): number;
					public clearDepth(param0: number): void;
					public getSHADER_TYPE(): number;
					public getBLEND_DST_RGB(): number;
					public getOUT_OF_MEMORY(): number;
					public enableVertexAttribArray(param0: number): void;
					public bindBuffer(param0: number, param1: number): void;
					public createRenderbuffer(): number;
					public getLUMINANCE_ALPHA(): number;
					public framebufferTexture2D(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public getUNPACK_PREMULTIPLY_ALPHA_WEBGL(): number;
					public getBOOL(): number;
					public bindAttribLocation(param0: number, param1: number, param2: string): void;
					public uniform2f(param0: number, param1: number, param2: number): void;
					public getRGB565(): number;
					public scissor(param0: number, param1: number, param2: number, param3: number): void;
					public blendEquationSeparate(param0: number, param1: number): void;
					public getDELETE_STATUS(): number;
					public getRGBA(): number;
					public deleteProgram(param0: number): void;
					public uniform3f(param0: number, param1: number, param2: number, param3: number): void;
					public getUNSIGNED_SHORT_4_4_4_4(): number;
					public getVERTEX_ATTRIB_ARRAY_SIZE(): number;
					public getDEPTH_RANGE(): number;
					public getSTENCIL_BACK_FUNC(): number;
					public getFRAMEBUFFER_INCOMPLETE_DIMENSIONS(): number;
					public getALIASED_POINT_SIZE_RANGE(): number;
					public clearStencil(param0: number): void;
					public getActiveUniform(param0: number, param1: number): com.github.triniwiz.canvas.WebGLActiveInfo;
					public getNOTEQUAL(): number;
					public colorMask(param0: boolean, param1: boolean, param2: boolean, param3: boolean): void;
					public getTRIANGLES(): number;
					public getTEXTURE24(): number;
					public getCONTEXT_LOST_WEBGL(): number;
					public getTEXTURE6(): number;
					public getDrawingBufferHeight(): number;
					public getRENDERBUFFER_STENCIL_SIZE(): number;
					public getTEXTURE7(): number;
					public getPACK_ALIGNMENT(): number;
					public getPOLYGON_OFFSET_FACTOR(): number;
					public getGL_RGB$canvas_release(): number;
					public getTEXTURE0(): number;
					public getLINES(): number;
					public getCOLOR_ATTACHMENT0(): number;
					public setCanvas$canvas_release(param0: com.github.triniwiz.canvas.TNSCanvas): void;
					public getALWAYS(): number;
					public getINVERT(): number;
					public getSTATIC_DRAW(): number;
					public getBYTE(): number;
					public getONE_MINUS_DST_ALPHA(): number;
					public getLOW_FLOAT(): number;
					public getMAX_VERTEX_ATTRIBS(): number;
					public getFLOAT_VEC3(): number;
					public getVERTEX_ATTRIB_ARRAY_ENABLED(): number;
					public createBuffer(): number;
					public getIMPLEMENTATION_COLOR_READ_FORMAT(): number;
					public getINT(): number;
					public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: native.Array<number>): void;
					public getSAMPLE_ALPHA_TO_COVERAGE(): number;
					public getRENDERBUFFER_BLUE_SIZE(): number;
					public getMAX_VERTEX_UNIFORM_VECTORS(): number;
					public useProgram(param0: number): void;
					public getDONT_CARE(): number;
					public getNEAREST(): number;
					public getFRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE(): number;
					public activeTexture(param0: number): void;
					public getPOINTS(): number;
					public getShaderInfoLog(param0: number): string;
					public isTexture(param0: number): boolean;
					public getCURRENT_VERTEX_ATTRIB(): number;
					public getLOW_INT(): number;
					public getSupportedExtensions(): native.Array<string>;
					public getGREATER(): number;
					public getUNPACK_FLIP_Y_WEBGL(): number;
					public getTEXTURE28(): number;
					public getTEXTURE15(): number;
					public disable(param0: number): void;
					public getBLEND_SRC_RGB(): number;
					public getSTENCIL_BITS(): number;
					public getDEPTH_COMPONENT(): number;
					public getRENDERBUFFER_HEIGHT(): number;
					public getFRAMEBUFFER_INCOMPLETE_ATTACHMENT(): number;
					public getLINEAR_MIPMAP_LINEAR(): number;
					public getRENDERBUFFER_GREEN_SIZE(): number;
					public getBLEND_SRC_ALPHA(): number;
					public getTEXTURE_2D(): number;
					public getGL_LUMINANCE$canvas_release(): number;
					public bindTexture(param0: number, param1: number): void;
					public getNO_ERROR(): number;
					public getTEXTURE_MAG_FILTER(): number;
					public getDEPTH_BUFFER_BIT(): number;
					public getCW(): number;
					public getSRC_ALPHA_SATURATE(): number;
					public deleteRenderbuffer(param0: number): void;
					public renderbufferStorage(param0: number, param1: number, param2: number, param3: number): void;
					public vertexAttrib3f(param0: number, param1: number, param2: number, param3: number): void;
					public updateCanvas$canvas_release(): void;
					public getMAX_COMBINED_TEXTURE_IMAGE_UNITS(): number;
					public getCULL_FACE_MODE(): number;
					public getBROWSER_DEFAULT_WEBGL(): number;
					public getTEXTURE19(): number;
					public getSTENCIL_PASS_DEPTH_FAIL(): number;
					public restoreStateAfterClear(): void;
					public getALIASED_LINE_WIDTH_RANGE(): number;
					public getGL_LUMINANCE_ALPHA$canvas_release(): number;
					public getKEEP(): number;
					public getVERSION(): number;
					public vertexAttrib2f(param0: number, param1: number, param2: number): void;
					public getCONSTANT_ALPHA(): number;
					public getDEPTH_COMPONENT16(): number;
					public getNEAREST_MIPMAP_NEAREST(): number;
					public getSTENCIL_TEST(): number;
					public getINCR_WRAP(): number;
					public getRGB5_A1(): number;
					public getFramebufferAttachmentParameter(param0: number, param1: number, param2: number): com.github.triniwiz.canvas.TNSFramebufferAttachmentParameter;
					public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: globalAndroid.graphics.Bitmap): void;
					public getBLEND(): number;
					public getBLEND_EQUATION(): number;
					public getIMPLEMENTATION_COLOR_READ_TYPE(): number;
					public getREPLACE(): number;
					public getTexParameter(param0: number, param1: number): number;
					public getTEXTURE_CUBE_MAP_POSITIVE_X(): number;
					public getUNPACK_ALIGNMENT(): number;
					public getTEXTURE3(): number;
					public getSHADING_LANGUAGE_VERSION(): number;
					public getCanvas$canvas_release(): com.github.triniwiz.canvas.TNSCanvas;
					public getGL_FLOAT$canvas_release(): number;
					public getSTENCIL_BACK_PASS_DEPTH_FAIL(): number;
					public getProgramParameter(param0: number, param1: number): any;
					public stencilOp(param0: number, param1: number, param2: number): void;
					public uniform2i(param0: number, param1: number, param2: number): void;
					public getFLOAT_MAT3(): number;
					public getONE_MINUS_CONSTANT_ALPHA(): number;
					public getCULL_FACE(): number;
					public getFLOAT(): number;
					public getVERTEX_SHADER(): number;
					public getREPEAT(): number;
					public getCOLOR_WRITEMASK(): number;
					public getFRONT_FACE(): number;
					public checkFramebufferStatus(param0: number): number;
					public getTEXTURE_CUBE_MAP_NEGATIVE_Y(): number;
					public getTRIANGLE_STRIP(): number;
					public getCOLOR_CLEAR_VALUE(): number;
					public getSUBPIXEL_BITS(): number;
					public getUniform(param0: number, param1: number): any;
					public getMEDIUM_FLOAT(): number;
					public getBLEND_EQUATION_RGB(): number;
					public getDEPTH_BITS(): number;
					public uniform4f(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public viewport(param0: number, param1: number, param2: number, param3: number): void;
					public getNEAREST_MIPMAP_LINEAR(): number;
					public getTEXTURE_MIN_FILTER(): number;
					public getSAMPLER_CUBE(): number;
					public getALPHA_BITS(): number;
					public isShader(param0: number): boolean;
					public getShaderSource(param0: number): string;
					public getCURRENT_PROGRAM(): number;
					public getCCW(): number;
					public getGEQUAL(): number;
					public bindRenderbuffer(param0: number, param1: number): void;
					public getVertexAttribOffset(param0: number, param1: number): number;
					public getRENDERBUFFER_WIDTH(): number;
					public getINT_VEC4(): number;
					public getUNPACK_COLORSPACE_CONVERSION_WEBGL(): number;
					public copyTexImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number): void;
					public getLUMINANCE(): number;
					public getLINEAR_MIPMAP_NEAREST(): number;
					public getFlipYWebGL(): boolean;
					public getSTENCIL_VALUE_MASK(): number;
					public vertexAttrib4fv(param0: number, param1: native.Array<number>): void;
					public getDITHER(): number;
					public blendEquation(param0: number): void;
					public bindBuffer(param0: number, param1: any): void;
					public reset(): void;
					public drawArrays(param0: number, param1: number, param2: number): void;
					public pixelStorei(param0: number, param1: any): void;
					public getTEXTURE20(): number;
					public getFLOAT_VEC4(): number;
					public getDEPTH_ATTACHMENT(): number;
					public getFRAMEBUFFER_COMPLETE(): number;
					public getTEXTURE30(): number;
					public detachShader(param0: number, param1: number): void;
					public getVIEWPORT(): number;
					public getTEXTURE10(): number;
					public clearIfComposited(param0: number): com.github.triniwiz.canvas.HowToClear;
					public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: com.github.triniwiz.canvas.TNSImageAsset): void;
					public vertexAttrib2fv(param0: number, param1: native.Array<number>): void;
					public getGL_UNSIGNED_SHORT_5_5_5_1$canvas_release(): number;
					public getARRAY_BUFFER_BINDING(): number;
					public hint(param0: number, param1: number): void;
					public getTEXTURE25(): number;
					public getVERTEX_ATTRIB_ARRAY_BUFFER_BINDING(): number;
					public runOnGLThread$canvas_release(param0: java.lang.Runnable): void;
					public sampleCoverage(param0: number, param1: boolean): void;
					public getSTREAM_DRAW(): number;
					public getBOOL_VEC3(): number;
					public getSCISSOR_BOX(): number;
					public getTEXTURE12(): number;
					public getTEXTURE8(): number;
					public getPOLYGON_OFFSET_UNITS(): number;
					public getSTENCIL_BACK_PASS_DEPTH_PASS(): number;
					public getUNSIGNED_SHORT_5_6_5(): number;
					public cullFace(param0: number): void;
					public getDEPTH_CLEAR_VALUE(): number;
					public blendFunc(param0: number, param1: number): void;
					public bufferData(param0: number, param1: native.Array<number>, param2: number): void;
					public getShaderPrecisionFormat(param0: number, param1: number): com.github.triniwiz.canvas.WebGLShaderPrecisionFormat;
					public getRENDERER(): number;
					public getDECR_WRAP(): number;
					public framebufferRenderbuffer(param0: number, param1: number, param2: number, param3: number): void;
					public getUNSIGNED_SHORT_5_5_5_1(): number;
					public compressedTexImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: native.Array<number>): void;
					public getDST_ALPHA(): number;
					public getRENDERBUFFER_BINDING(): number;
					public getDrawingBufferWidth(): number;
					public getACTIVE_ATTRIBUTES(): number;
					public getMAX_RENDERBUFFER_SIZE(): number;
					public clearColor(param0: number, param1: number, param2: number, param3: number): void;
					public getTEXTURE29(): number;
					public getSTENCIL_CLEAR_VALUE(): number;
					public drawElements(param0: number, param1: number, param2: number, param3: number): void;
					public getFRAMEBUFFER_UNSUPPORTED(): number;
					public getUniformLocation(param0: number, param1: string): number;
					public getFRONT_AND_BACK(): number;
					public getHIGH_INT(): number;
					public clear(param0: number): void;
					public getGL_UNSIGNED_BYTE$canvas_release(): number;
					public getContextAttributes(): java.util.Map<string,any>;
					public getSTENCIL_ATTACHMENT(): number;
					public getTEXTURE27(): number;
					public getUNSIGNED_INT(): number;
					public uniform1f(param0: number, param1: number): void;
					public getSTENCIL_FUNC(): number;
					public getATTACHED_SHADERS(): number;
					public getShaderParameter(param0: number, param1: number): any;
					public uniform1i(param0: number, param1: number): void;
					public getMAX_VARYING_VECTORS(): number;
					public getSTENCIL_BUFFER_BIT(): number;
					public getSAMPLE_COVERAGE_VALUE(): number;
					public getFLOAT_VEC2(): number;
					public uniform4iv(param0: number, param1: native.Array<number>): void;
					public getDEPTH_WRITEMASK(): number;
					public isFramebuffer(param0: number): boolean;
					public getINT_VEC2(): number;
					public getMAX_VERTEX_TEXTURE_IMAGE_UNITS(): number;
					public getMEDIUM_INT(): number;
					public getSAMPLE_COVERAGE_INVERT(): number;
					public getGL_ALPHA$canvas_release(): number;
					public constructor(param0: com.github.triniwiz.canvas.TNSCanvas, param1: java.util.Map<string,any>);
					public constructor(param0: com.github.triniwiz.canvas.TNSCanvas);
					public getSTENCIL_BACK_FAIL(): number;
					public getSTENCIL_INDEX8(): number;
					public readPixels(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: native.Array<number>): void;
					public getLINK_STATUS(): number;
					public getSTENCIL_BACK_WRITEMASK(): number;
					public getLESS(): number;
					public depthMask(param0: boolean): void;
					public createTexture(): number;
					public texParameterf(param0: number, param1: number, param2: number): void;
					public getTEXTURE14(): number;
					public getBLEND_EQUATION_ALPHA(): number;
					public getRGBA4(): number;
					public getBLUE_BITS(): number;
					public getRENDERBUFFER_RED_SIZE(): number;
					public getSAMPLE_BUFFERS(): number;
					public getLEQUAL(): number;
					public getTEXTURE23(): number;
					public getLINEAR(): number;
					public getINVALID_FRAMEBUFFER_OPERATION(): number;
					public getTEXTURE13(): number;
					public getTEXTURE_WRAP_T(): number;
					public getACTIVE_UNIFORMS(): number;
					public getTEXTURE_CUBE_MAP(): number;
					public uniform3i(param0: number, param1: number, param2: number, param3: number): void;
					public getFRAMEBUFFER_ATTACHMENT_OBJECT_NAME(): number;
					public copyTexSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number): void;
					public getELEMENT_ARRAY_BUFFER_BINDING(): number;
					public depthFunc(param0: number): void;
					public getSRC_COLOR(): number;
					public getVENDOR(): number;
					public getMAX_TEXTURE_IMAGE_UNITS(): number;
					public vertexAttrib1fv(param0: number, param1: native.Array<number>): void;
					public depthRange(param0: number, param1: number): void;
					public getProgramInfoLog(param0: number): string;
					public getCONSTANT_COLOR(): number;
					public getFUNC_REVERSE_SUBTRACT(): number;
					public getRED_BITS(): number;
					public getBOOL_VEC2(): number;
					public attachShader(param0: number, param1: number): void;
					public getMIRRORED_REPEAT(): number;
					public getSTENCIL_REF(): number;
					public getCOMPILE_STATUS(): number;
					public getSTENCIL_BACK_REF(): number;
					public disableVertexAttribArray(param0: number): void;
					public getNEVER(): number;
					public getLINE_LOOP(): number;
					public getRGB(): number;
					public createFramebuffer(): number;
					public getSAMPLE_COVERAGE(): number;
					public getINT_VEC3(): number;
					public getSAMPLES(): number;
					public isRenderbuffer(param0: number): boolean;
					public getNICEST(): number;
					public getTRIANGLE_FAN(): number;
					public getONE_MINUS_DST_COLOR(): number;
					public vertexAttribPointer(param0: number, param1: number, param2: number, param3: boolean, param4: number, param5: number): void;
					public getTEXTURE_CUBE_MAP_POSITIVE_Z(): number;
					public getTEXTURE22(): number;
					public isContextLost(): boolean;
					public vertexAttrib1f(param0: number, param1: number): void;
					public getLINE_WIDTH(): number;
					public getSTENCIL_FAIL(): number;
					public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: com.github.triniwiz.canvas.TNSImageAsset): void;
					public uniform3fv(param0: number, param1: native.Array<number>): void;
					public frontFace(param0: number): void;
					public getELEMENT_ARRAY_BUFFER(): number;
					public polygonOffset(param0: number, param1: number): void;
					public getSAMPLER_2D(): number;
					public createProgram(): number;
					public blendColor(param0: number, param1: number, param2: number, param3: number): void;
					public getDEPTH_STENCIL_ATTACHMENT(): number;
					public getTEXTURE5(): number;
					public getGL_RGBA$canvas_release(): number;
					public stencilFunc(param0: number, param1: number, param2: number): void;
					public getMAX_VIEWPORT_DIMS(): number;
					public uniform3iv(param0: number, param1: native.Array<number>): void;
					public getTEXTURE_BINDING_CUBE_MAP(): number;
					public getBUFFER_USAGE(): number;
					public getFRONT(): number;
					public getDST_COLOR(): number;
					public getGENERATE_MIPMAP_HINT(): number;
					public uniform2iv(param0: number, param1: native.Array<number>): void;
					public getGL_HALF_FLOAT$canvas_release(): number;
					public lineWidth(param0: number): void;
					public bufferSubData(param0: number, param1: number, param2: native.Array<number>): void;
					public getVERTEX_ATTRIB_ARRAY_STRIDE(): number;
					public getRENDERBUFFER_INTERNAL_FORMAT(): number;
					public getFRAMEBUFFER(): number;
					public shaderSource(param0: number, param1: string): void;
					public getTEXTURE1(): number;
					public deleteShader(param0: number): void;
					public getGL_UNSIGNED_SHORT_5_6_5$canvas_release(): number;
					public validateProgram(param0: number): void;
					public uniform1fv(param0: number, param1: native.Array<number>): void;
					public getRENDERBUFFER_DEPTH_SIZE(): number;
					public compressedTexSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: native.Array<number>): void;
					public getSCISSOR_TEST(): number;
					public getSHORT(): number;
					public uniform4i(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public getRENDERBUFFER(): number;
					public clearIfComposited(): com.github.triniwiz.canvas.HowToClear;
					public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: native.Array<number>): void;
					public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: com.github.triniwiz.canvas.TNSCanvas): void;
					public getONE_MINUS_CONSTANT_COLOR(): number;
					public getEQUAL(): number;
					public getError(): number;
					public getBACK(): number;
					public uniform4fv(param0: number, param1: native.Array<number>): void;
					public getDEPTH_FUNC(): number;
					public stencilMaskSeparate(param0: number, param1: number): void;
					public getTEXTURE17(): number;
					public uniform2fv(param0: number, param1: native.Array<number>): void;
					public getFRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT(): number;
					public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: globalAndroid.graphics.Bitmap): void;
					public getINVALID_ENUM(): number;
					public getBUFFER_SIZE(): number;
					public getMAX_CUBE_MAP_TEXTURE_SIZE(): number;
					public getFRAMEBUFFER_ATTACHMENT_OBJECT_TYPE(): number;
					public deleteTexture(param0: number): void;
					public bufferData(param0: number, param1: any, param2: number): void;
					public deleteBuffer(param0: number): void;
					public getTEXTURE26(): number;
					public getZERO(): number;
					public getUNSIGNED_BYTE(): number;
				}
				export module TNSWebGLRenderingContext {
					export class Companion {
						public static class: java.lang.Class<com.github.triniwiz.canvas.TNSWebGLRenderingContext.Companion>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class Utils {
					public static class: java.lang.Class<com.github.triniwiz.canvas.Utils>;
					public static INSTANCE: com.github.triniwiz.canvas.Utils;
					public isEmulator(): boolean;
					public getByteBufferFromBitmap(param0: globalAndroid.graphics.Bitmap): java.nio.ByteBuffer;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class WebGLActiveInfo {
					public static class: java.lang.Class<com.github.triniwiz.canvas.WebGLActiveInfo>;
					public getType(): number;
					public setSize(param0: number): void;
					public getName(): string;
					public constructor();
					public setType(param0: number): void;
					public getSize(): number;
					public constructor(param0: string, param1: number, param2: number);
					public setName(param0: string): void;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class WebGLShaderPrecisionFormat {
					public static class: java.lang.Class<com.github.triniwiz.canvas.WebGLShaderPrecisionFormat>;
					public getRangeMin(): number;
					public setPrecision(param0: number): void;
					public constructor();
					public constructor(param0: number, param1: number, param2: number);
					public setRangeMin(param0: number): void;
					public getRangeMax(): number;
					public setRangeMax(param0: number): void;
					public getPrecision(): number;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class ANGLE_instanced_arrays {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.ANGLE_instanced_arrays>;
						public constructor();
						public vertexAttribDivisorANGLE(param0: number, param1: number): void;
						public drawArraysInstancedANGLE(param0: number, param1: number, param2: number, param3: number): void;
						public drawElementsInstancedANGLE(param0: number, param1: number, param2: number, param3: number, param4: number): void;
						public getVERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_blend_minmax {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_blend_minmax>;
						public constructor();
						public getMAX_EXT(): number;
						public getMIN_EXT(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_color_buffer_float {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_color_buffer_float>;
						public constructor();
						public getRGBA32F(): number;
						public setR32F(param0: number): void;
						public getRG16F(): number;
						public getR32F(): number;
						public getR11F_G11F_B10F(): number;
						public setRG32F(param0: number): void;
						public getR16F(): number;
						public setRGBA32F(param0: number): void;
						public setRG16F(param0: number): void;
						public getRGB16F(): number;
						public setR11F_G11F_B10F(param0: number): void;
						public setRGB16F(param0: number): void;
						public setR16F(param0: number): void;
						public getRG32F(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_color_buffer_half_float {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_color_buffer_half_float>;
						public constructor();
						public getRGBA16F_EXT(): number;
						public setUNSIGNED_NORMALIZED_EXT(param0: number): void;
						public setRGB16F_EXT(param0: number): void;
						public getRGB16F_EXT(): number;
						public getFRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT(): number;
						public setRGBA16F_EXT(param0: number): void;
						public setFRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT(param0: number): void;
						public getUNSIGNED_NORMALIZED_EXT(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_disjoint_timer_query {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_disjoint_timer_query>;
						public static Companion: com.github.triniwiz.canvas.extensions.EXT_disjoint_timer_query.Companion;
						public deleteQueryEXT(param0: number): void;
						public setCanvas(param0: com.github.triniwiz.canvas.TNSCanvas): void;
						public queryCounterEXT(param0: number, param1: number): void;
						public getQueryEXT(param0: number, param1: number): number;
						public constructor(param0: com.github.triniwiz.canvas.TNSCanvas);
						public endQueryEXT(param0: number): void;
						public createQueryEXT(): number;
						public getQueryObjectEXT(param0: number, param1: number): any;
						public getCanvas(): com.github.triniwiz.canvas.TNSCanvas;
						public beginQueryEXT(param0: number, param1: number): void;
						public isQueryEXT(param0: number): boolean;
					}
					export module EXT_disjoint_timer_query {
						export class Companion {
							public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_disjoint_timer_query.Companion>;
							public setTIMESTAMP_EXT(param0: number): void;
							public getCURRENT_QUERY_EXT(): number;
							public setQUERY_RESULT_AVAILABLE_EXT(param0: number): void;
							public getQUERY_RESULT_EXT(): number;
							public setCURRENT_QUERY_EXT(param0: number): void;
							public getGPU_DISJOINT_EXT(): number;
							public getQUERY_RESULT_AVAILABLE_EXT(): number;
							public getQUERY_COUNTER_BITS_EXT(): number;
							public setTIME_ELAPSED_EXT(param0: number): void;
							public setQUERY_RESULT_EXT(param0: number): void;
							public setQUERY_COUNTER_BITS_EXT(param0: number): void;
							public getTIMESTAMP_EXT(): number;
							public setGPU_DISJOINT_EXT(param0: number): void;
							public getTIME_ELAPSED_EXT(): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_sRGB {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_sRGB>;
						public setFRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT(param0: number): void;
						public getSRGB_EXT(): number;
						public getSRGB8_ALPHA8_EXT(): number;
						public constructor();
						public setSRGB_ALPHA_EXT(param0: number): void;
						public setSRGB8_ALPHA8_EXT(param0: number): void;
						public setSRGB_EXT(param0: number): void;
						public getSRGB_ALPHA_EXT(): number;
						public getFRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_shader_texture_lod {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_shader_texture_lod>;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_texture_filter_anisotropic {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_texture_filter_anisotropic>;
						public constructor();
						public setTEXTURE_MAX_ANISOTROPY_EXT(param0: number): void;
						public setMAX_TEXTURE_MAX_ANISOTROPY_EXT(param0: number): void;
						public getMAX_TEXTURE_MAX_ANISOTROPY_EXT(): number;
						public getTEXTURE_MAX_ANISOTROPY_EXT(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_element_index_uint {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_element_index_uint>;
						public constructor();
						public setUNSIGNED_INT(param0: number): void;
						public getUNSIGNED_INT(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_fbo_render_mipmap {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_fbo_render_mipmap>;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_standard_derivatives {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_standard_derivatives>;
						public constructor();
						public getFRAGMENT_SHADER_DERIVATIVE_HINT_OES(): number;
						public setFRAGMENT_SHADER_DERIVATIVE_HINT_OES(param0: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_texture_float {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_texture_float>;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_texture_float_linear {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_texture_float_linear>;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_texture_half_float {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_texture_half_float>;
						public constructor();
						public getHALF_FLOAT_OES(): number;
						public setHALF_FLOAT_OES(param0: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_texture_half_float_linear {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_texture_half_float_linear>;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_vertex_array_object {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_vertex_array_object>;
						public createVertexArrayOES(): number;
						public setCanvas(param0: com.github.triniwiz.canvas.TNSCanvas): void;
						public deleteVertexArrayOES(param0: number): void;
						public bindVertexArrayOES(param0: number): void;
						public constructor(param0: com.github.triniwiz.canvas.TNSCanvas);
						public setVERTEX_ARRAY_BINDING_OES(param0: number): void;
						public getVERTEX_ARRAY_BINDING_OES(): number;
						public getCanvas(): com.github.triniwiz.canvas.TNSCanvas;
						public isVertexArrayOES(param0: number): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_color_buffer_float {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_color_buffer_float>;
						public constructor();
						public setRGBA32F_EXT(param0: number): void;
						public setUNSIGNED_NORMALIZED_EXT(param0: number): void;
						public getRGB32F_EXT(): number;
						public setRGB32F_EXT(param0: number): void;
						public getFRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT(): number;
						public setFRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT(param0: number): void;
						public getRGBA32F_EXT(): number;
						public getUNSIGNED_NORMALIZED_EXT(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_compressed_texture_atc {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_atc>;
						public constructor();
						public setCOMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL(param0: number): void;
						public getCOMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL(): number;
						public setCOMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL(param0: number): void;
						public getCOMPRESSED_RGB_ATC_WEBGL(): number;
						public setCOMPRESSED_RGB_ATC_WEBGL(param0: number): void;
						public getCOMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_compressed_texture_etc {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_etc>;
						public setCOMPRESSED_RG11_EAC(param0: number): void;
						public setCOMPRESSED_RGB8_ETC2(param0: number): void;
						public constructor();
						public setCOMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2(param0: number): void;
						public setCOMPRESSED_SIGNED_R11_EAC(param0: number): void;
						public setCOMPRESSED_R11_EAC(param0: number): void;
						public setCOMPRESSED_RGBA8_ETC2_EAC(param0: number): void;
						public setCOMPRESSED_SRGB8_ETC2(param0: number): void;
						public setCOMPRESSED_SRGB8_ALPHA8_ETC2_EAC(param0: number): void;
						public getCOMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2(): number;
						public getCOMPRESSED_RG11_EAC(): number;
						public getCOMPRESSED_RGBA8_ETC2_EAC(): number;
						public getCOMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2(): number;
						public getCOMPRESSED_SRGB8_ALPHA8_ETC2_EAC(): number;
						public getCOMPRESSED_R11_EAC(): number;
						public setCOMPRESSED_SIGNED_RG11_EAC(param0: number): void;
						public getCOMPRESSED_SIGNED_RG11_EAC(): number;
						public getCOMPRESSED_SIGNED_R11_EAC(): number;
						public setCOMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2(param0: number): void;
						public getCOMPRESSED_RGB8_ETC2(): number;
						public getCOMPRESSED_SRGB8_ETC2(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_compressed_texture_etc1 {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_etc1>;
						public constructor();
						public setCOMPRESSED_RGB_ETC1_WEBGL(param0: number): void;
						public getCOMPRESSED_RGB_ETC1_WEBGL(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_compressed_texture_pvrtc {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_pvrtc>;
						public setCOMPRESSED_RGB_PVRTC_4BPPV1_IMG(param0: number): void;
						public setCOMPRESSED_RGBA_PVRTC_2BPPV1_IMG(param0: number): void;
						public constructor();
						public getCOMPRESSED_RGB_PVRTC_4BPPV1_IMG(): number;
						public setCOMPRESSED_RGB_PVRTC_2BPPV1_IMG(param0: number): void;
						public getCOMPRESSED_RGBA_PVRTC_4BPPV1_IMG(): number;
						public getCOMPRESSED_RGBA_PVRTC_2BPPV1_IMG(): number;
						public setCOMPRESSED_RGBA_PVRTC_4BPPV1_IMG(param0: number): void;
						public getCOMPRESSED_RGB_PVRTC_2BPPV1_IMG(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_compressed_texture_s3tc {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_s3tc>;
						public getCOMPRESSED_RGBA_S3TC_DXT1_EXT(): number;
						public setCOMPRESSED_RGBA_S3TC_DXT1_EXT(param0: number): void;
						public setCOMPRESSED_RGBA_S3TC_DXT3_EXT(param0: number): void;
						public constructor();
						public getCOMPRESSED_RGB_S3TC_DXT1_EXT(): number;
						public getCOMPRESSED_RGBA_S3TC_DXT3_EXT(): number;
						public setCOMPRESSED_RGBA_S3TC_DXT5_EXT(param0: number): void;
						public getCOMPRESSED_RGBA_S3TC_DXT5_EXT(): number;
						public setCOMPRESSED_RGB_S3TC_DXT1_EXT(param0: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_compressed_texture_s3tc_srgb {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_s3tc_srgb>;
						public getCOMPRESSED_SRGB_S3TC_DXT1_EXT(): number;
						public setCOMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT(param0: number): void;
						public constructor();
						public getCOMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT(): number;
						public setCOMPRESSED_SRGB_S3TC_DXT1_EXT(param0: number): void;
						public getCOMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT(): number;
						public setCOMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT(param0: number): void;
						public getCOMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT(): number;
						public setCOMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT(param0: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_depth_texture {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_depth_texture>;
						public constructor();
						public getUNSIGNED_INT_24_8_WEBGL(): number;
						public setUNSIGNED_INT_24_8_WEBGL(param0: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_draw_buffers {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_draw_buffers>;
						public getCOLOR_ATTACHMENT1_WEBGL(): number;
						public setDRAW_BUFFER4_WEBGL(param0: number): void;
						public setDRAW_BUFFER15_WEBGL(param0: number): void;
						public getCOLOR_ATTACHMENT10_WEBGL(): number;
						public setCOLOR_ATTACHMENT14_WEBGL(param0: number): void;
						public getDRAW_BUFFER1_WEBGL(): number;
						public setCOLOR_ATTACHMENT1_WEBGL(param0: number): void;
						public setCOLOR_ATTACHMENT13_WEBGL(param0: number): void;
						public getCOLOR_ATTACHMENT2_WEBGL(): number;
						public getCOLOR_ATTACHMENT11_WEBGL(): number;
						public setDRAW_BUFFER3_WEBGL(param0: number): void;
						public setCOLOR_ATTACHMENT15_WEBGL(param0: number): void;
						public drawBuffersWEBGL(param0: native.Array<number>): void;
						public getDRAW_BUFFER10_WEBGL(): number;
						public setMAX_DRAW_BUFFERS_WEBGL(param0: number): void;
						public getDRAW_BUFFER2_WEBGL(): number;
						public getCOLOR_ATTACHMENT3_WEBGL(): number;
						public setCOLOR_ATTACHMENT0_WEBGL(param0: number): void;
						public constructor();
						public getMAX_DRAW_BUFFERS_WEBGL(): number;
						public getDRAW_BUFFER12_WEBGL(): number;
						public setDRAW_BUFFER13_WEBGL(param0: number): void;
						public setCOLOR_ATTACHMENT11_WEBGL(param0: number): void;
						public getDRAW_BUFFER7_WEBGL(): number;
						public setDRAW_BUFFER7_WEBGL(param0: number): void;
						public getCOLOR_ATTACHMENT8_WEBGL(): number;
						public setDRAW_BUFFER2_WEBGL(param0: number): void;
						public getDRAW_BUFFER4_WEBGL(): number;
						public setDRAW_BUFFER5_WEBGL(param0: number): void;
						public getCOLOR_ATTACHMENT0_WEBGL(): number;
						public getCOLOR_ATTACHMENT13_WEBGL(): number;
						public getDRAW_BUFFER9_WEBGL(): number;
						public setCOLOR_ATTACHMENT8_WEBGL(param0: number): void;
						public setDRAW_BUFFER0_WEBGL(param0: number): void;
						public setCOLOR_ATTACHMENT3_WEBGL(param0: number): void;
						public getCOLOR_ATTACHMENT5_WEBGL(): number;
						public setCOLOR_ATTACHMENT5_WEBGL(param0: number): void;
						public getCOLOR_ATTACHMENT6_WEBGL(): number;
						public setDRAW_BUFFER10_WEBGL(param0: number): void;
						public getDRAW_BUFFER5_WEBGL(): number;
						public getCOLOR_ATTACHMENT15_WEBGL(): number;
						public getDRAW_BUFFER14_WEBGL(): number;
						public getCOLOR_ATTACHMENT14_WEBGL(): number;
						public setCOLOR_ATTACHMENT4_WEBGL(param0: number): void;
						public getCOLOR_ATTACHMENT7_WEBGL(): number;
						public setCOLOR_ATTACHMENT6_WEBGL(param0: number): void;
						public setCOLOR_ATTACHMENT10_WEBGL(param0: number): void;
						public getDRAW_BUFFER15_WEBGL(): number;
						public setDRAW_BUFFER8_WEBGL(param0: number): void;
						public getMAX_COLOR_ATTACHMENTS_WEBGL(): number;
						public getDRAW_BUFFER6_WEBGL(): number;
						public setDRAW_BUFFER9_WEBGL(param0: number): void;
						public setDRAW_BUFFER11_WEBGL(param0: number): void;
						public getDRAW_BUFFER3_WEBGL(): number;
						public getDRAW_BUFFER8_WEBGL(): number;
						public getCOLOR_ATTACHMENT12_WEBGL(): number;
						public setDRAW_BUFFER12_WEBGL(param0: number): void;
						public setMAX_COLOR_ATTACHMENTS_WEBGL(param0: number): void;
						public getDRAW_BUFFER11_WEBGL(): number;
						public setCOLOR_ATTACHMENT7_WEBGL(param0: number): void;
						public getCOLOR_ATTACHMENT4_WEBGL(): number;
						public getDRAW_BUFFER13_WEBGL(): number;
						public setCOLOR_ATTACHMENT9_WEBGL(param0: number): void;
						public setCOLOR_ATTACHMENT2_WEBGL(param0: number): void;
						public setDRAW_BUFFER1_WEBGL(param0: number): void;
						public getCOLOR_ATTACHMENT9_WEBGL(): number;
						public setCOLOR_ATTACHMENT12_WEBGL(param0: number): void;
						public getDRAW_BUFFER0_WEBGL(): number;
						public setDRAW_BUFFER6_WEBGL(param0: number): void;
						public setDRAW_BUFFER14_WEBGL(param0: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_lose_context {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_lose_context>;
						public restoreContext(): void;
						public loseContext(): void;
						public constructor(param0: com.github.triniwiz.canvas.TNSCanvas);
						public setCanvasView(param0: com.github.triniwiz.canvas.TNSCanvas): void;
						public getCanvasView(): com.github.triniwiz.canvas.TNSCanvas;
					}
				}
			}
		}
	}
}

//Generics information:

