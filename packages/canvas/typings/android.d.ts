declare module org {
	export module nativescript {
		export module canvas {
			export class AnimationFrame {
				public static class: java.lang.Class<org.nativescript.canvas.AnimationFrame>;
				public static Companion: org.nativescript.canvas.AnimationFrame.Companion;
				public reset(): void;
				public raf(param0: number): void;
				public fps(param0: number): void;
				public doFrame(param0: number): void;
				public constructor();
			}
			export module AnimationFrame {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.AnimationFrame.Companion>;
					public requestAnimationFrame(param0: kotlin.jvm.functions.Function1<any,kotlin.Unit>): number;
					public cancelAnimationFrame(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class BuildConfig {
				public static class: java.lang.Class<org.nativescript.canvas.BuildConfig>;
				public static DEBUG: boolean;
				public static LIBRARY_PACKAGE_NAME: string;
				public static BUILD_TYPE: string;
				public constructor();
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class CPUView {
				public static class: java.lang.Class<org.nativescript.canvas.CPUView>;
				public onDraw(param0: globalAndroid.graphics.Canvas): void;
				public getLock$canvas_release(): any;
				public onSizeChanged(param0: number, param1: number, param2: number, param3: number): void;
				public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
				public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet, param2: number);
				public constructor(param0: globalAndroid.content.Context);
				public setCanvasView$canvas_release(param0: java.lang.ref.WeakReference<org.nativescript.canvas.TNSCanvas>): void;
				public getCanvasView$canvas_release(): java.lang.ref.WeakReference<org.nativescript.canvas.TNSCanvas>;
				public flush(): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class Constants {
				public static class: java.lang.Class<org.nativescript.canvas.Constants>;
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
				public static INSTANCE: org.nativescript.canvas.Constants;
				public setMAX_CLIENT_WAIT_TIMEOUT_WEBGL(param0: number): void;
				public getMAX_CLIENT_WAIT_TIMEOUT_WEBGL(): number;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class GLContext {
				public static class: java.lang.Class<org.nativescript.canvas.GLContext>;
				public glView: java.lang.ref.WeakReference<org.nativescript.canvas.GLView>;
				public mGLThread: org.nativescript.canvas.GLContext.GLThread;
				public reference: java.lang.ref.WeakReference<org.nativescript.canvas.TNSCanvas>;
				public static TAG: string;
				public static Companion: org.nativescript.canvas.GLContext.Companion;
				public getFailIfMajorPerformanceCaveat(): boolean;
				public setAlpha(param0: boolean): void;
				public getPowerPreference(): string;
				public onPause(): void;
				public getStencil(): boolean;
				public setMEGLSurface$canvas_release(param0: javax.microedition.khronos.egl.EGLSurface): void;
				public getAntialias(): boolean;
				public isGLThreadStarted(): boolean;
				public flush(): void;
				public constructor();
				public isHeadless(): boolean;
				public destroy(): void;
				public resize(param0: number, param1: number): void;
				public setDepth(param0: boolean): void;
				public setPowerPreference(param0: string): void;
				public setDesynchronized(param0: boolean): void;
				public getDepth(): boolean;
				public destroySurface(param0: javax.microedition.khronos.egl.EGLSurface): boolean;
				public getPremultipliedAlpha(): boolean;
				public getXrCompatible(): boolean;
				public setPreserveDrawingBuffer(param0: boolean): void;
				public createSurface(param0: javax.microedition.khronos.egl.EGLConfig, param1: any): javax.microedition.khronos.egl.EGLSurface;
				public onResume(): void;
				public makeEglSurfaceCurrent(param0: javax.microedition.khronos.egl.EGLSurface): void;
				public swapBuffers(param0: javax.microedition.khronos.egl.EGLSurface): boolean;
				public setFailIfMajorPerformanceCaveat(param0: boolean): void;
				public init(param0: any): void;
				public createEglSurface(param0: any): javax.microedition.khronos.egl.EGLSurface;
				public setPremultipliedAlpha(param0: boolean): void;
				public setStencil(param0: boolean): void;
				public getDesynchronized(): boolean;
				public queueEvent(param0: java.lang.Runnable): void;
				public startGLThread(): void;
				public makeCurrent(param0: javax.microedition.khronos.egl.EGLSurface): boolean;
				public getMEGLSurface$canvas_release(): javax.microedition.khronos.egl.EGLSurface;
				public setXrCompatible(param0: boolean): void;
				public getPreserveDrawingBuffer(): boolean;
				public setAntialias(param0: boolean): void;
				public getAlpha(): boolean;
			}
			export module GLContext {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.GLContext.Companion>;
					public setIS_WEBGL_2_SUPPORT(param0: boolean): void;
					public getIS_WEBGL_2_SUPPORT(): boolean;
					public setDID_CHECK_WEBGL_SUPPORT(param0: boolean): void;
					public getDID_CHECK_WEBGL_SUPPORT(): boolean;
				}
				export class GLThread {
					public static class: java.lang.Class<org.nativescript.canvas.GLContext.GLThread>;
					public isStarted: boolean;
					public interrupt(): void;
					public run(): void;
					public constructor(param0: org.nativescript.canvas.GLContext);
					public getMSurface(): any;
					public constructor(param0: any);
					public start(): void;
					public getType(): org.nativescript.canvas.TNSCanvas.ContextType;
					public setType(param0: org.nativescript.canvas.TNSCanvas.ContextType): void;
					public setMSurface(param0: any): void;
					public setPaused(param0: boolean): void;
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
				public drawingBufferWidth: number;
				public drawingBufferHeight: number;
				public onSurfaceTextureDestroyed(param0: globalAndroid.graphics.SurfaceTexture): boolean;
				public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
				public getStarting(): boolean;
				public setStartupLock(param0: java.util.concurrent.CountDownLatch): void;
				public queueEvent(param0: java.lang.Runnable): void;
				public getGLContext(): org.nativescript.canvas.GLContext;
				public init(): void;
				public flush(): void;
				public onSurfaceTextureAvailable(param0: globalAndroid.graphics.SurfaceTexture, param1: number, param2: number): void;
				public resize(param0: number, param1: number): void;
				public setupContext(): void;
				public setIgnorePixelScaling(param0: boolean): void;
				public setListener(param0: org.nativescript.canvas.TNSCanvas.Listener): void;
				public onSurfaceTextureUpdated(param0: globalAndroid.graphics.SurfaceTexture): void;
				public constructor(param0: globalAndroid.content.Context);
				public setStarting(param0: boolean): void;
				public getIgnorePixelScaling(): boolean;
				public getStartupLock(): java.util.concurrent.CountDownLatch;
				public onSurfaceTextureSizeChanged(param0: globalAndroid.graphics.SurfaceTexture, param1: number, param2: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class GLViewSV {
				public static class: java.lang.Class<org.nativescript.canvas.GLViewSV>;
				public setListener(param0: org.nativescript.canvas.TNSCanvas.Listener): void;
				public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
				public constructor(param0: globalAndroid.content.Context);
				public surfaceDestroyed(param0: globalAndroid.view.SurfaceHolder): void;
				public queueEvent(param0: java.lang.Runnable): void;
				public surfaceChanged(param0: globalAndroid.view.SurfaceHolder, param1: number, param2: number, param3: number): void;
				public getGLContext(): org.nativescript.canvas.GLContext;
				public init(): void;
				public flush(): void;
				public surfaceCreated(param0: globalAndroid.view.SurfaceHolder): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class HowToClear {
				public static class: java.lang.Class<org.nativescript.canvas.HowToClear>;
				public static Skipped: org.nativescript.canvas.HowToClear;
				public static JustClear: org.nativescript.canvas.HowToClear;
				public static CombinedClear: org.nativescript.canvas.HowToClear;
				public static values(): androidNative.Array<org.nativescript.canvas.HowToClear>;
				public static valueOf(param0: string): org.nativescript.canvas.HowToClear;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class IndexedParameter {
				public static class: java.lang.Class<org.nativescript.canvas.IndexedParameter>;
				public isBuffer(): boolean;
				public setBuffer(param0: boolean): void;
				public setBufferValue(param0: number): void;
				public setValue(param0: number): void;
				public getValue(): number;
				public getBufferValue(): number;
				public constructor();
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSCanvas {
				public static class: java.lang.Class<org.nativescript.canvas.TNSCanvas>;
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
				public mClearColor: androidNative.Array<number>;
				public mScissorEnabled: boolean;
				public mClearDepth: number;
				public mColorMask: androidNative.Array<boolean>;
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
				public static Companion: org.nativescript.canvas.TNSCanvas.Companion;
				public setCpuHandler(param0: globalAndroid.os.Handler): void;
				public getCpuHandlerThread(): globalAndroid.os.HandlerThread;
				public setWebGL$canvas_release(param0: boolean): void;
				public onActivityResumed(param0: globalAndroid.app.Activity): void;
				public getWebGLRenderingContext$canvas_release(): org.nativescript.canvas.TNSWebGLRenderingContext;
				public getCpuView$canvas_release(): org.nativescript.canvas.CPUView;
				public setCpuHandlerThread(param0: globalAndroid.os.HandlerThread): void;
				public setHandleInvalidationManually(param0: boolean): void;
				public static nativeCustomWithBitmapFlush(param0: number, param1: globalAndroid.graphics.Bitmap): void;
				public flush(): void;
				public setIgnorePixelScaling(param0: boolean): void;
				public setSurface$canvas_release(param0: org.nativescript.canvas.GLView): void;
				public setUseCpu$canvas_release(param0: boolean): void;
				public getListener(): org.nativescript.canvas.TNSCanvas.Listener;
				public getSurface$canvas_release(): org.nativescript.canvas.GLView;
				public getDrawingBufferWidth(): number;
				public getIgnorePixelScaling(): boolean;
				public toDataURL(): string;
				public constructor(param0: globalAndroid.content.Context, param1: boolean);
				public getActualContextType$canvas_release(): string;
				public toData(): androidNative.Array<number>;
				public setActualContextType$canvas_release(param0: string): void;
				public resizeViewPort(): void;
				public setCpuView$canvas_release(param0: org.nativescript.canvas.CPUView): void;
				public static nativeResizeCustomSurface(param0: number, param1: number, param2: number, param3: number, param4: boolean, param5: number): void;
				public onResume(): void;
				public isWebGL$canvas_release(): boolean;
				public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
				public getRenderingContext2d$canvas_release(): org.nativescript.canvas.TNSCanvasRenderingContext;
				public getCpuHandler(): globalAndroid.os.Handler;
				public setContextType$canvas_release(param0: org.nativescript.canvas.TNSCanvas.ContextType): void;
				public static createSVGMatrix(): org.nativescript.canvas.TNSDOMMatrix;
				public getScale$canvas_release(): number;
				public toDataURLAsync(param0: string, param1: org.nativescript.canvas.TNSCanvas.DataURLListener): void;
				public static nativeFlush(param0: number): void;
				public isPaused$canvas_release(): boolean;
				public onDetachedFromWindow(): void;
				public getContextType$canvas_release(): org.nativescript.canvas.TNSCanvas.ContextType;
				public setScale$canvas_release(param0: number): void;
				public toDataURL(param0: string, param1: number): string;
				public static nativeResizeSurface(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: boolean, param7: number): void;
				public setRenderingContext2d$canvas_release(param0: org.nativescript.canvas.TNSCanvasRenderingContext): void;
				public onActivityCreated(param0: globalAndroid.app.Activity, param1: globalAndroid.os.Bundle): void;
				public onPause(): void;
				public finalize(): void;
				public doFrame(param0: number): void;
				public onActivityStarted(param0: globalAndroid.app.Activity): void;
				public toDataURLAsync(param0: string, param1: number, param2: org.nativescript.canvas.TNSCanvas.DataURLListener): void;
				public setWebGL2RenderingContext$canvas_release(param0: org.nativescript.canvas.TNSWebGL2RenderingContext): void;
				public initCanvas$canvas_release(): void;
				public setListener(param0: org.nativescript.canvas.TNSCanvas.Listener): void;
				public setLastSize$canvas_release(param0: org.nativescript.canvas.TNSCanvas.Size): void;
				public static layoutView(param0: number, param1: number, param2: org.nativescript.canvas.TNSCanvas): void;
				public constructor(param0: globalAndroid.content.Context);
				public toDataURLAsync(param0: org.nativescript.canvas.TNSCanvas.DataURLListener): void;
				public setPaused$canvas_release(param0: boolean): void;
				public getNativeContext$canvas_release(): number;
				public setCtx$canvas_release(param0: globalAndroid.content.Context): void;
				public setNewSize$canvas_release(param0: org.nativescript.canvas.TNSCanvas.Size): void;
				public getUseCpu$canvas_release(): boolean;
				public static nativeInitContextWithCustomSurface(param0: number, param1: number, param2: number, param3: boolean, param4: number, param5: number, param6: number): number;
				public setNativeContext$canvas_release(param0: number): void;
				public getWebGL2RenderingContext$canvas_release(): org.nativescript.canvas.TNSWebGL2RenderingContext;
				public snapshot(): androidNative.Array<number>;
				public onActivityDestroyed(param0: globalAndroid.app.Activity): void;
				public setWebGLRenderingContext$canvas_release(param0: org.nativescript.canvas.TNSWebGLRenderingContext): void;
				public onActivitySaveInstanceState(param0: globalAndroid.app.Activity, param1: globalAndroid.os.Bundle): void;
				public getLastSize$canvas_release(): org.nativescript.canvas.TNSCanvas.Size;
				public isHandleInvalidationManually(): boolean;
				public queueEvent(param0: java.lang.Runnable): void;
				public toDataURL(param0: string): string;
				public getCtx$canvas_release(): globalAndroid.content.Context;
				public static getDirection$canvas_release(): org.nativescript.canvas.TNSTextDirection;
				public onActivityPaused(param0: globalAndroid.app.Activity): void;
				public onSizeChanged(param0: number, param1: number, param2: number, param3: number): void;
				public static nativeInitContext(param0: number, param1: number, param2: number, param3: number, param4: number, param5: boolean, param6: number, param7: number, param8: number): number;
				public getContext(param0: string): org.nativescript.canvas.TNSCanvasRenderingContext;
				public getContext(param0: string, param1: java.util.Map<string,any>): org.nativescript.canvas.TNSCanvasRenderingContext;
				public onActivityStopped(param0: globalAndroid.app.Activity): void;
				public setupActivityHandler(param0: globalAndroid.app.Application): void;
				public onAttachedToWindow(): void;
				public getDrawingBufferHeight(): number;
				public getNewSize$canvas_release(): org.nativescript.canvas.TNSCanvas.Size;
			}
			export module TNSCanvas {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSCanvas.Companion>;
					public getViews(): java.util.concurrent.ConcurrentHashMap<any,any>;
					public getDirection$canvas_release(): org.nativescript.canvas.TNSTextDirection;
					public nativeResizeCustomSurface(param0: number, param1: number, param2: number, param3: number, param4: boolean, param5: number): void;
					public nativeResizeSurface(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: boolean, param7: number): void;
					public createSVGMatrix(): org.nativescript.canvas.TNSDOMMatrix;
					public layoutView(param0: number, param1: number, param2: org.nativescript.canvas.TNSCanvas): void;
					public nativeFlush(param0: number): void;
					public isLibraryLoaded$canvas_release(): boolean;
					public nativeCustomWithBitmapFlush(param0: number, param1: globalAndroid.graphics.Bitmap): void;
					public setLastCall$canvas_release(param0: number): void;
					public nativeInitContextWithCustomSurface(param0: number, param1: number, param2: number, param3: boolean, param4: number, param5: number, param6: number): number;
					public nativeInitContext(param0: number, param1: number, param2: number, param3: number, param4: number, param5: boolean, param6: number, param7: number, param8: number): number;
					public setLibraryLoaded$canvas_release(param0: boolean): void;
					public setViews(param0: java.util.concurrent.ConcurrentHashMap<any,any>): void;
					public getLastCall$canvas_release(): number;
				}
				export class ContextType {
					public static class: java.lang.Class<org.nativescript.canvas.TNSCanvas.ContextType>;
					public static NONE: org.nativescript.canvas.TNSCanvas.ContextType;
					public static CANVAS: org.nativescript.canvas.TNSCanvas.ContextType;
					public static WEBGL: org.nativescript.canvas.TNSCanvas.ContextType;
					public static valueOf(param0: string): org.nativescript.canvas.TNSCanvas.ContextType;
					public static values(): androidNative.Array<org.nativescript.canvas.TNSCanvas.ContextType>;
				}
				export class DataURLListener {
					public static class: java.lang.Class<org.nativescript.canvas.TNSCanvas.DataURLListener>;
					/**
					 * Constructs a new instance of the org.nativescript.canvas.TNSCanvas$DataURLListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
						onResult(param0: string): void;
					});
					public constructor();
					public onResult(param0: string): void;
				}
				export class Listener {
					public static class: java.lang.Class<org.nativescript.canvas.TNSCanvas.Listener>;
					/**
					 * Constructs a new instance of the org.nativescript.canvas.TNSCanvas$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
						contextReady(): void;
					});
					public constructor();
					public contextReady(): void;
				}
				export class Size {
					public static class: java.lang.Class<org.nativescript.canvas.TNSCanvas.Size>;
					public setWidth(param0: number): void;
					public getHeight(): number;
					public setHeight(param0: number): void;
					public getWidth(): number;
					public constructor(param0: number, param1: number);
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSCanvasGradient extends org.nativescript.canvas.TNSColorStyle {
				public static class: java.lang.Class<org.nativescript.canvas.TNSCanvasGradient>;
				public static Companion: org.nativescript.canvas.TNSCanvasGradient.Companion;
				public setStyle(param0: number): void;
				public getStyleType(): org.nativescript.canvas.TNSColorStyleType;
				public finalize(): void;
				public getStyle(): number;
				public constructor(param0: number);
				public addColorStop(param0: number, param1: string): void;
				public constructor();
			}
			export module TNSCanvasGradient {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSCanvasGradient.Companion>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSCanvasRenderingContext {
				public static class: java.lang.Class<org.nativescript.canvas.TNSCanvasRenderingContext>;
				/**
				 * Constructs a new instance of the org.nativescript.canvas.TNSCanvasRenderingContext interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
				 */
				public constructor(implementation: {
				});
				public constructor();
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSCanvasRenderingContext2D extends org.nativescript.canvas.TNSCanvasRenderingContext {
				public static class: java.lang.Class<org.nativescript.canvas.TNSCanvasRenderingContext2D>;
				public static TAG: string;
				public static Companion: org.nativescript.canvas.TNSCanvasRenderingContext2D.Companion;
				public closePath(): void;
				public arc(param0: number, param1: number, param2: number, param3: number, param4: number, param5: boolean): void;
				public getStrokeStyle(): org.nativescript.canvas.TNSColorStyle;
				public scale(param0: number, param1: number): void;
				public drawImage(param0: org.nativescript.canvas.TNSImageAsset, param1: number, param2: number, param3: number, param4: number): void;
				public setLineCap(param0: org.nativescript.canvas.TNSLineCap): void;
				public setFont(param0: string): void;
				public drawImage(param0: org.nativescript.canvas.TNSImageAsset, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
				public createPattern(param0: org.nativescript.canvas.TNSImageBitmap, param1: org.nativescript.canvas.TNSPatternRepetition): org.nativescript.canvas.TNSPattern;
				public setFillStyle(param0: org.nativescript.canvas.TNSColorStyle): void;
				public getGlobalCompositeOperation(): org.nativescript.canvas.TNSCompositeOperationType;
				public getCurrentTransform(): org.nativescript.canvas.TNSDOMMatrix;
				public drawImage(param0: org.nativescript.canvas.TNSCanvas, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
				public getImageData(param0: number, param1: number, param2: number, param3: number): org.nativescript.canvas.TNSImageData;
				public measureText(param0: string): org.nativescript.canvas.TNSTextMetrics;
				public createImageData(param0: number, param1: number): org.nativescript.canvas.TNSImageData;
				public getImageSmoothingQuality(): org.nativescript.canvas.TNSImageSmoothingQuality;
				public getCanvas(): org.nativescript.canvas.TNSCanvas;
				public isPointInStroke(param0: number, param1: number): boolean;
				public clip(param0: org.nativescript.canvas.TNSFillRule): void;
				public clip(): void;
				public getLineJoin(): org.nativescript.canvas.TNSLineJoin;
				public createLinearGradient(param0: number, param1: number, param2: number, param3: number): org.nativescript.canvas.TNSCanvasGradient;
				public setFilter(param0: string): void;
				public getTextAlign(): org.nativescript.canvas.TNSTextAlignment;
				public getShadowBlur(): number;
				public getFont(): string;
				public constructor(param0: org.nativescript.canvas.TNSCanvas);
				public rect(param0: number, param1: number, param2: number, param3: number): void;
				public createImageData(param0: org.nativescript.canvas.TNSImageData): org.nativescript.canvas.TNSImageData;
				public getDirection(): org.nativescript.canvas.TNSTextDirection;
				public getLineDash(): androidNative.Array<number>;
				public strokeText(param0: string, param1: number, param2: number): void;
				public arcTo(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getShadowColor(): string;
				public setLineDashOffset(param0: number): void;
				public drawImage(param0: org.nativescript.canvas.TNSCanvas, param1: number, param2: number, param3: number, param4: number): void;
				public isPointInPath(param0: org.nativescript.canvas.TNSPath2D, param1: number, param2: number, param3: org.nativescript.canvas.TNSFillRule): boolean;
				public createPattern(param0: org.nativescript.canvas.TNSCanvas, param1: org.nativescript.canvas.TNSPatternRepetition): org.nativescript.canvas.TNSPattern;
				public resetTransform(): void;
				public translate(param0: number, param1: number): void;
				public getShadowOffsetX(): number;
				public bezierCurveTo(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
				public fill(param0: org.nativescript.canvas.TNSPath2D, param1: org.nativescript.canvas.TNSFillRule): void;
				public setDirection(param0: org.nativescript.canvas.TNSTextDirection): void;
				public fill(param0: org.nativescript.canvas.TNSFillRule): void;
				public setGlobalAlpha(param0: number): void;
				public getShadowOffsetY(): number;
				public getFillStyle(): org.nativescript.canvas.TNSColorStyle;
				public getLineCap(): org.nativescript.canvas.TNSLineCap;
				public clip(param0: org.nativescript.canvas.TNSPath2D, param1: org.nativescript.canvas.TNSFillRule): void;
				public setImageSmoothingEnabled(param0: boolean): void;
				public restore(): void;
				public createRadialGradient(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): org.nativescript.canvas.TNSCanvasGradient;
				public getImageSmoothingEnabled(): boolean;
				public drawImage(param0: globalAndroid.graphics.Bitmap, param1: number, param2: number): void;
				public quadraticCurveTo(param0: number, param1: number, param2: number, param3: number): void;
				public fillText(param0: string, param1: number, param2: number, param3: number): void;
				public clip(param0: org.nativescript.canvas.TNSPath2D): void;
				public rotate(param0: number): void;
				public getFilter(): string;
				public setShadowColor(param0: string): void;
				public fill(): void;
				public fill(param0: org.nativescript.canvas.TNSPath2D): void;
				public isPointInPath(param0: org.nativescript.canvas.TNSPath2D, param1: number, param2: number): boolean;
				public setLineDash(param0: androidNative.Array<number>): void;
				public beginPath(): void;
				public getGlobalAlpha(): number;
				public clearRect(param0: number, param1: number, param2: number, param3: number): void;
				public drawImage(param0: globalAndroid.graphics.Bitmap, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
				public createPattern(param0: org.nativescript.canvas.TNSImageAsset, param1: org.nativescript.canvas.TNSPatternRepetition): org.nativescript.canvas.TNSPattern;
				public drawImage(param0: org.nativescript.canvas.TNSCanvas, param1: number, param2: number): void;
				public getLineDashOffset(): number;
				public arc(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public putImageData(param0: org.nativescript.canvas.TNSImageData, param1: number, param2: number): void;
				public strokeText(param0: string, param1: number, param2: number, param3: number): void;
				public isPointInPath(param0: number, param1: number): boolean;
				public isPointInStroke(param0: org.nativescript.canvas.TNSPath2D, param1: number, param2: number): boolean;
				public ellipse(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number): void;
				public setStrokeStyle(param0: org.nativescript.canvas.TNSColorStyle): void;
				public stroke(): void;
				public strokeRect(param0: number, param1: number, param2: number, param3: number): void;
				public setShadowOffsetX(param0: number): void;
				public setGlobalCompositeOperation(param0: org.nativescript.canvas.TNSCompositeOperationType): void;
				public drawImage(param0: org.nativescript.canvas.TNSImageBitmap, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
				public setTextAlign(param0: org.nativescript.canvas.TNSTextAlignment): void;
				public save(): void;
				public setImageSmoothingQuality(param0: org.nativescript.canvas.TNSImageSmoothingQuality): void;
				public putImageData(param0: org.nativescript.canvas.TNSImageData, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number): void;
				public setLineJoin(param0: org.nativescript.canvas.TNSLineJoin): void;
				public lineTo(param0: number, param1: number): void;
				public drawImage(param0: org.nativescript.canvas.TNSImageAsset, param1: number, param2: number): void;
				public setShadowBlur(param0: number): void;
				public finalize(): void;
				public stroke(param0: org.nativescript.canvas.TNSPath2D): void;
				public fillRect(param0: number, param1: number, param2: number, param3: number): void;
				public drawImage(param0: org.nativescript.canvas.TNSImageBitmap, param1: number, param2: number): void;
				public setShadowOffsetY(param0: number): void;
				public transform(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
				public moveTo(param0: number, param1: number): void;
				public getMiterLimit(): number;
				public drawImage(param0: org.nativescript.canvas.TNSImageBitmap, param1: number, param2: number, param3: number, param4: number): void;
				public setTransform(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
				public setLineWidth(param0: number): void;
				public createPattern(param0: globalAndroid.graphics.Bitmap, param1: org.nativescript.canvas.TNSPatternRepetition): org.nativescript.canvas.TNSPattern;
				public drawImage(param0: globalAndroid.graphics.Bitmap, param1: number, param2: number, param3: number, param4: number): void;
				public fillText(param0: string, param1: number, param2: number): void;
				public setMiterLimit(param0: number): void;
				public isPointInPath(param0: number, param1: number, param2: org.nativescript.canvas.TNSFillRule): boolean;
				public getLineWidth(): number;
				public ellipse(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: boolean): void;
				public setCurrentTransform(param0: org.nativescript.canvas.TNSDOMMatrix): void;
			}
			export module TNSCanvasRenderingContext2D {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSCanvasRenderingContext2D.Companion>;
					public setDebug(param0: boolean): void;
					public isDebug(): boolean;
				}
				export class WhenMappings {
					public static class: java.lang.Class<org.nativescript.canvas.TNSCanvasRenderingContext2D.WhenMappings>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSColor extends org.nativescript.canvas.TNSColorStyle {
				public static class: java.lang.Class<org.nativescript.canvas.TNSColor>;
				public static Companion: org.nativescript.canvas.TNSColor.Companion;
				public setColor(param0: string): void;
				public getColor(): string;
				public getStyleType(): org.nativescript.canvas.TNSColorStyleType;
				public constructor(param0: number);
				public constructor(param0: string);
				public constructor();
			}
			export module TNSColor {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSColor.Companion>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export abstract class TNSColorStyle {
				public static class: java.lang.Class<org.nativescript.canvas.TNSColorStyle>;
				public static Companion: org.nativescript.canvas.TNSColorStyle.Companion;
				public getStyleType(): org.nativescript.canvas.TNSColorStyleType;
				public static nativeDestroy(param0: number): void;
				public constructor();
			}
			export module TNSColorStyle {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSColorStyle.Companion>;
					public nativeDestroy(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSColorStyleType {
				public static class: java.lang.Class<org.nativescript.canvas.TNSColorStyleType>;
				public static Color: org.nativescript.canvas.TNSColorStyleType;
				public static Gradient: org.nativescript.canvas.TNSColorStyleType;
				public static Pattern: org.nativescript.canvas.TNSColorStyleType;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSColorStyleType>;
				public toString(): string;
				public static valueOf(param0: string): org.nativescript.canvas.TNSColorStyleType;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSCompositeOperationType {
				public static class: java.lang.Class<org.nativescript.canvas.TNSCompositeOperationType>;
				public static SourceOver: org.nativescript.canvas.TNSCompositeOperationType;
				public static SourceIn: org.nativescript.canvas.TNSCompositeOperationType;
				public static SourceOut: org.nativescript.canvas.TNSCompositeOperationType;
				public static SourceAtop: org.nativescript.canvas.TNSCompositeOperationType;
				public static DestinationOver: org.nativescript.canvas.TNSCompositeOperationType;
				public static DestinationIn: org.nativescript.canvas.TNSCompositeOperationType;
				public static DestinationOut: org.nativescript.canvas.TNSCompositeOperationType;
				public static DestinationAtop: org.nativescript.canvas.TNSCompositeOperationType;
				public static Lighter: org.nativescript.canvas.TNSCompositeOperationType;
				public static Copy: org.nativescript.canvas.TNSCompositeOperationType;
				public static Xor: org.nativescript.canvas.TNSCompositeOperationType;
				public static Multiply: org.nativescript.canvas.TNSCompositeOperationType;
				public static Screen: org.nativescript.canvas.TNSCompositeOperationType;
				public static Overlay: org.nativescript.canvas.TNSCompositeOperationType;
				public static Darken: org.nativescript.canvas.TNSCompositeOperationType;
				public static Lighten: org.nativescript.canvas.TNSCompositeOperationType;
				public static ColorDodge: org.nativescript.canvas.TNSCompositeOperationType;
				public static ColorBurn: org.nativescript.canvas.TNSCompositeOperationType;
				public static HardLight: org.nativescript.canvas.TNSCompositeOperationType;
				public static SoftLight: org.nativescript.canvas.TNSCompositeOperationType;
				public static Difference: org.nativescript.canvas.TNSCompositeOperationType;
				public static Exclusion: org.nativescript.canvas.TNSCompositeOperationType;
				public static Hue: org.nativescript.canvas.TNSCompositeOperationType;
				public static Saturation: org.nativescript.canvas.TNSCompositeOperationType;
				public static Color: org.nativescript.canvas.TNSCompositeOperationType;
				public static Luminosity: org.nativescript.canvas.TNSCompositeOperationType;
				public static Companion: org.nativescript.canvas.TNSCompositeOperationType.Companion;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSCompositeOperationType>;
				public isError$canvas_release(): boolean;
				public setError$canvas_release(param0: boolean): void;
				public toString(): string;
				public toNative(): number;
				public static valueOf(param0: string): org.nativescript.canvas.TNSCompositeOperationType;
			}
			export module TNSCompositeOperationType {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSCompositeOperationType.Companion>;
					public fromNative(param0: number): org.nativescript.canvas.TNSCompositeOperationType;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSDOMMatrix {
				public static class: java.lang.Class<org.nativescript.canvas.TNSDOMMatrix>;
				public static Companion: org.nativescript.canvas.TNSDOMMatrix.Companion;
				public getD(): number;
				public getM24(): number;
				public getM44(): number;
				public setMatrix$canvas_release(param0: number): void;
				public getA(): number;
				public setB(param0: number): void;
				public getM34(): number;
				public getM41(): number;
				public getM14(): number;
				public constructor();
				public setM22(param0: number): void;
				public setM43(param0: number): void;
				public setE(param0: number): void;
				public setM31(param0: number): void;
				public setM14(param0: number): void;
				public setM32(param0: number): void;
				public setC(param0: number): void;
				public getM42(): number;
				public getM32(): number;
				public getB(): number;
				public getM22(): number;
				public getM12(): number;
				public setM44(param0: number): void;
				public getC(): number;
				public getM31(): number;
				public getM11(): number;
				public getM21(): number;
				public setM13(param0: number): void;
				public setM33(param0: number): void;
				public setD(param0: number): void;
				public finalize(): void;
				public setM24(param0: number): void;
				public setM21(param0: number): void;
				public getE(): number;
				public getM43(): number;
				public getM33(): number;
				public getM23(): number;
				public setM12(param0: number): void;
				public getM13(): number;
				public getF(): number;
				public setA(param0: number): void;
				public setM41(param0: number): void;
				public setM23(param0: number): void;
				public getMatrix$canvas_release(): number;
				public setM42(param0: number): void;
				public setF(param0: number): void;
				public constructor(param0: number);
				public setM34(param0: number): void;
				public setM11(param0: number): void;
			}
			export module TNSDOMMatrix {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSDOMMatrix.Companion>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSFileReader {
				public static class: java.lang.Class<org.nativescript.canvas.TNSFileReader>;
				public static INSTANCE: org.nativescript.canvas.TNSFileReader;
				public read(param0: java.io.File): androidNative.Array<number>;
				public read(param0: string): androidNative.Array<number>;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSFillRule {
				public static class: java.lang.Class<org.nativescript.canvas.TNSFillRule>;
				public static NonZero: org.nativescript.canvas.TNSFillRule;
				public static EvenOdd: org.nativescript.canvas.TNSFillRule;
				public static Companion: org.nativescript.canvas.TNSFillRule.Companion;
				public setRule(param0: string): void;
				public getRule(): string;
				public setValue(param0: number): void;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSFillRule>;
				public static valueOf(param0: string): org.nativescript.canvas.TNSFillRule;
				public toString(): string;
				public getValue(): number;
				public toNative(): number;
			}
			export module TNSFillRule {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSFillRule.Companion>;
					public fromNative(param0: number): org.nativescript.canvas.TNSFillRule;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSFramebufferAttachmentParameter {
				public static class: java.lang.Class<org.nativescript.canvas.TNSFramebufferAttachmentParameter>;
				public setTexture(param0: boolean): void;
				public setRenderbuffer(param0: boolean): void;
				public constructor(param0: boolean, param1: boolean, param2: number);
				public isRenderbuffer(): boolean;
				public setValue(param0: number): void;
				public isTexture(): boolean;
				public getValue(): number;
				public constructor();
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSImageAsset {
				public static class: java.lang.Class<org.nativescript.canvas.TNSImageAsset>;
				public static Companion: org.nativescript.canvas.TNSImageAsset.Companion;
				public getHeight(): number;
				public loadImageFromUrl(param0: string): boolean;
				public scale(param0: number, param1: number): void;
				public flipY(): void;
				public static nativeGetWidthImpl$canvas_release(param0: number): number;
				public loadImageFromResource(param0: number, param1: globalAndroid.content.Context): boolean;
				public finalize(): void;
				public getBytes(): androidNative.Array<number>;
				public loadImageFromImageAsync(param0: globalAndroid.graphics.Bitmap, param1: org.nativescript.canvas.TNSImageAsset.Callback): void;
				public constructor();
				public getNativeImageAsset$canvas_release(): number;
				public loadImageFromUrlAsync(param0: string, param1: org.nativescript.canvas.TNSImageAsset.Callback): void;
				public loadImageFromPath(param0: string): boolean;
				public loadImageFromBytes(param0: androidNative.Array<number>): boolean;
				public flipX(): void;
				public loadImageFromImage(param0: globalAndroid.graphics.Bitmap): boolean;
				public getWidth(): number;
				public save(param0: string, param1: org.nativescript.canvas.TNSImageAssetFormat): boolean;
				public saveAsync(param0: string, param1: org.nativescript.canvas.TNSImageAssetFormat, param2: org.nativescript.canvas.TNSImageAsset.Callback): void;
				public setNativeImageAsset$canvas_release(param0: number): void;
				public getError(): string;
				public static nativeGetHeightImpl$canvas_release(param0: number): number;
				public loadImageFromBytesAsync(param0: androidNative.Array<number>, param1: org.nativescript.canvas.TNSImageAsset.Callback): void;
				public loadImageFromPathAsync(param0: string, param1: org.nativescript.canvas.TNSImageAsset.Callback): void;
				public static nativeDestroyImpl$canvas_release(param0: number): void;
			}
			export module TNSImageAsset {
				export class ByteArrayOutputStream2 {
					public static class: java.lang.Class<org.nativescript.canvas.TNSImageAsset.ByteArrayOutputStream2>;
					public constructor();
					public buf(): androidNative.Array<number>;
					public constructor(param0: number);
				}
				export class Callback {
					public static class: java.lang.Class<org.nativescript.canvas.TNSImageAsset.Callback>;
					/**
					 * Constructs a new instance of the org.nativescript.canvas.TNSImageAsset$Callback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
						onSuccess(param0: any): void;
						onError(param0: string): void;
					});
					public constructor();
					public onError(param0: string): void;
					public onSuccess(param0: any): void;
				}
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSImageAsset.Companion>;
					public nativeGetHeightImpl$canvas_release(param0: number): number;
					public nativeGetWidthImpl$canvas_release(param0: number): number;
					public nativeDestroyImpl$canvas_release(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSImageAssetFormat {
				public static class: java.lang.Class<org.nativescript.canvas.TNSImageAssetFormat>;
				public static JPG: org.nativescript.canvas.TNSImageAssetFormat;
				public static PNG: org.nativescript.canvas.TNSImageAssetFormat;
				public static ICO: org.nativescript.canvas.TNSImageAssetFormat;
				public static BMP: org.nativescript.canvas.TNSImageAssetFormat;
				public static TIFF: org.nativescript.canvas.TNSImageAssetFormat;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSImageAssetFormat>;
				public setFormat(param0: number): void;
				public static valueOf(param0: string): org.nativescript.canvas.TNSImageAssetFormat;
				public getFormat(): number;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSImageBitmap {
				public static class: java.lang.Class<org.nativescript.canvas.TNSImageBitmap>;
				public static Companion: org.nativescript.canvas.TNSImageBitmap.Companion;
				public getHeight(): number;
				public finalize(): void;
				public static createFromBuffer(param0: java.nio.ByteBuffer, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: org.nativescript.canvas.TNSImageBitmap.Options, param8: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromBufferEncoded(param0: java.nio.ByteBuffer, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public setNativeImageAsset(param0: number): void;
				public static createFromBytes(param0: androidNative.Array<number>, param1: number, param2: number, param3: org.nativescript.canvas.TNSImageBitmap.Options, param4: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromBytes(param0: androidNative.Array<number>, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: org.nativescript.canvas.TNSImageBitmap.Options, param8: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromImageBitmap(param0: org.nativescript.canvas.TNSImageBitmap, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromBytes(param0: androidNative.Array<number>, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromBytesEncoded(param0: androidNative.Array<number>, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromBufferEncoded(param0: java.nio.ByteBuffer, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromBytesEncoded(param0: androidNative.Array<number>, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromImageData(param0: org.nativescript.canvas.TNSImageData, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public close(): void;
				public static createFromCanvas(param0: org.nativescript.canvas.TNSCanvas, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromImageData(param0: org.nativescript.canvas.TNSImageData, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public getWidth(): number;
				public static createFromBuffer(param0: java.nio.ByteBuffer, param1: number, param2: number, param3: org.nativescript.canvas.TNSImageBitmap.Options, param4: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromBitmap(param0: globalAndroid.graphics.Bitmap, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromCanvas(param0: org.nativescript.canvas.TNSCanvas, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromImageBitmap(param0: org.nativescript.canvas.TNSImageBitmap, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public getNativeImageAsset(): number;
				public constructor(param0: number);
				public static createFromImageAsset(param0: org.nativescript.canvas.TNSImageAsset, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromImageAsset(param0: org.nativescript.canvas.TNSImageAsset, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				public static createFromBitmap(param0: globalAndroid.graphics.Bitmap, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
			}
			export module TNSImageBitmap {
				export class Callback {
					public static class: java.lang.Class<org.nativescript.canvas.TNSImageBitmap.Callback>;
					/**
					 * Constructs a new instance of the org.nativescript.canvas.TNSImageBitmap$Callback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
						onSuccess(param0: org.nativescript.canvas.TNSImageBitmap): void;
						onError(param0: string): void;
					});
					public constructor();
					public onError(param0: string): void;
					public onSuccess(param0: org.nativescript.canvas.TNSImageBitmap): void;
				}
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSImageBitmap.Companion>;
					public createFromImageData(param0: org.nativescript.canvas.TNSImageData, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromBytes(param0: androidNative.Array<number>, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromCanvas(param0: org.nativescript.canvas.TNSCanvas, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromImageAsset(param0: org.nativescript.canvas.TNSImageAsset, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromImageData(param0: org.nativescript.canvas.TNSImageData, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromCanvas(param0: org.nativescript.canvas.TNSCanvas, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromImageBitmap(param0: org.nativescript.canvas.TNSImageBitmap, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromBytesEncoded(param0: androidNative.Array<number>, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromImageBitmap(param0: org.nativescript.canvas.TNSImageBitmap, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromBuffer(param0: java.nio.ByteBuffer, param1: number, param2: number, param3: org.nativescript.canvas.TNSImageBitmap.Options, param4: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromBufferEncoded(param0: java.nio.ByteBuffer, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromBufferEncoded(param0: java.nio.ByteBuffer, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public getFAILED_TO_LOAD(): string;
					public createFromBytes(param0: androidNative.Array<number>, param1: number, param2: number, param3: org.nativescript.canvas.TNSImageBitmap.Options, param4: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromImageAsset(param0: org.nativescript.canvas.TNSImageAsset, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromBytes(param0: androidNative.Array<number>, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: org.nativescript.canvas.TNSImageBitmap.Options, param8: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromBuffer(param0: java.nio.ByteBuffer, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: org.nativescript.canvas.TNSImageBitmap.Options, param8: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromBytesEncoded(param0: androidNative.Array<number>, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromBitmap(param0: globalAndroid.graphics.Bitmap, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap.Options, param6: org.nativescript.canvas.TNSImageBitmap.Callback): void;
					public createFromBitmap(param0: globalAndroid.graphics.Bitmap, param1: org.nativescript.canvas.TNSImageBitmap.Options, param2: org.nativescript.canvas.TNSImageBitmap.Callback): void;
				}
				export class Options {
					public static class: java.lang.Class<org.nativescript.canvas.TNSImageBitmap.Options>;
					public getPremultiplyAlpha(): org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha;
					public getResizeQuality(): org.nativescript.canvas.TNSImageBitmapResizeQuality;
					public setResizeWidth(param0: number): void;
					public setPremultiplyAlpha(param0: org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha): void;
					public setColorSpaceConversion(param0: org.nativescript.canvas.TNSImageBitmapColorSpaceConversion): void;
					public getResizeHeight(): number;
					public getColorSpaceConversion(): org.nativescript.canvas.TNSImageBitmapColorSpaceConversion;
					public setFlipY(param0: boolean): void;
					public setResizeHeight(param0: number): void;
					public constructor();
					public getResizeWidth(): number;
					public setResizeQuality(param0: org.nativescript.canvas.TNSImageBitmapResizeQuality): void;
					public getFlipY(): boolean;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSImageBitmapColorSpaceConversion {
				public static class: java.lang.Class<org.nativescript.canvas.TNSImageBitmapColorSpaceConversion>;
				public static Default: org.nativescript.canvas.TNSImageBitmapColorSpaceConversion;
				public static None: org.nativescript.canvas.TNSImageBitmapColorSpaceConversion;
				public static Companion: org.nativescript.canvas.TNSImageBitmapColorSpaceConversion.Companion;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSImageBitmapColorSpaceConversion>;
				public static valueOf(param0: string): org.nativescript.canvas.TNSImageBitmapColorSpaceConversion;
				public toString(): string;
				public getValue(): number;
				public toNative(): number;
				public getSpace(): string;
			}
			export module TNSImageBitmapColorSpaceConversion {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSImageBitmapColorSpaceConversion.Companion>;
					public fromNative(param0: number): org.nativescript.canvas.TNSImageBitmapColorSpaceConversion;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSImageBitmapPremultiplyAlpha {
				public static class: java.lang.Class<org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha>;
				public static Default: org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha;
				public static Premultiply: org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha;
				public static None: org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha;
				public static Companion: org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha.Companion;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha>;
				public toString(): string;
				public getValue(): number;
				public toNative(): number;
				public getAlpha(): string;
				public static valueOf(param0: string): org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha;
			}
			export module TNSImageBitmapPremultiplyAlpha {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha.Companion>;
					public fromNative(param0: number): org.nativescript.canvas.TNSImageBitmapPremultiplyAlpha;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSImageBitmapResizeQuality {
				public static class: java.lang.Class<org.nativescript.canvas.TNSImageBitmapResizeQuality>;
				public static Low: org.nativescript.canvas.TNSImageBitmapResizeQuality;
				public static Medium: org.nativescript.canvas.TNSImageBitmapResizeQuality;
				public static High: org.nativescript.canvas.TNSImageBitmapResizeQuality;
				public static Pixelated: org.nativescript.canvas.TNSImageBitmapResizeQuality;
				public static Companion: org.nativescript.canvas.TNSImageBitmapResizeQuality.Companion;
				public getQuality(): string;
				public static valueOf(param0: string): org.nativescript.canvas.TNSImageBitmapResizeQuality;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSImageBitmapResizeQuality>;
				public toString(): string;
				public getValue(): number;
				public toNative(): number;
			}
			export module TNSImageBitmapResizeQuality {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSImageBitmapResizeQuality.Companion>;
					public fromNative(param0: number): org.nativescript.canvas.TNSImageBitmapResizeQuality;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSImageData {
				public static class: java.lang.Class<org.nativescript.canvas.TNSImageData>;
				public static Companion: org.nativescript.canvas.TNSImageData.Companion;
				public getDataStore$canvas_release(): java.nio.ByteBuffer;
				public constructor(param0: number, param1: number, param2: number);
				public getWidth(): number;
				public getHeight(): number;
				public getData(): java.nio.ByteBuffer;
				public setDataStore$canvas_release(param0: java.nio.ByteBuffer): void;
				public getNativeImageData$canvas_release(): number;
				public finalize(): void;
				public setNativeImageData$canvas_release(param0: number): void;
			}
			export module TNSImageData {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSImageData.Companion>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSImageSmoothingQuality {
				public static class: java.lang.Class<org.nativescript.canvas.TNSImageSmoothingQuality>;
				public static Low: org.nativescript.canvas.TNSImageSmoothingQuality;
				public static Medium: org.nativescript.canvas.TNSImageSmoothingQuality;
				public static High: org.nativescript.canvas.TNSImageSmoothingQuality;
				public static Companion: org.nativescript.canvas.TNSImageSmoothingQuality.Companion;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSImageSmoothingQuality>;
				public static valueOf(param0: string): org.nativescript.canvas.TNSImageSmoothingQuality;
				public isError$canvas_release(): boolean;
				public setError$canvas_release(param0: boolean): void;
				public toString(): string;
				public toNative(): number;
			}
			export module TNSImageSmoothingQuality {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSImageSmoothingQuality.Companion>;
					public fromNative(param0: number): org.nativescript.canvas.TNSImageSmoothingQuality;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSLineCap {
				public static class: java.lang.Class<org.nativescript.canvas.TNSLineCap>;
				public static Butt: org.nativescript.canvas.TNSLineCap;
				public static Round: org.nativescript.canvas.TNSLineCap;
				public static Square: org.nativescript.canvas.TNSLineCap;
				public static Companion: org.nativescript.canvas.TNSLineCap.Companion;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSLineCap>;
				public isError$canvas_release(): boolean;
				public setError$canvas_release(param0: boolean): void;
				public toString(): string;
				public toNative(): number;
				public static valueOf(param0: string): org.nativescript.canvas.TNSLineCap;
			}
			export module TNSLineCap {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSLineCap.Companion>;
					public fromNative(param0: number): org.nativescript.canvas.TNSLineCap;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSLineJoin {
				public static class: java.lang.Class<org.nativescript.canvas.TNSLineJoin>;
				public static Bevel: org.nativescript.canvas.TNSLineJoin;
				public static Round: org.nativescript.canvas.TNSLineJoin;
				public static Miter: org.nativescript.canvas.TNSLineJoin;
				public static Companion: org.nativescript.canvas.TNSLineJoin.Companion;
				public isError$canvas_release(): boolean;
				public setError$canvas_release(param0: boolean): void;
				public static valueOf(param0: string): org.nativescript.canvas.TNSLineJoin;
				public toString(): string;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSLineJoin>;
				public toNative(): number;
			}
			export module TNSLineJoin {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSLineJoin.Companion>;
					public fromNative(param0: number): org.nativescript.canvas.TNSLineJoin;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSPath2D {
				public static class: java.lang.Class<org.nativescript.canvas.TNSPath2D>;
				public static Companion: org.nativescript.canvas.TNSPath2D.Companion;
				public closePath(): void;
				public lineTo(param0: number, param1: number): void;
				public setPath$canvas_release(param0: number): void;
				public arc(param0: number, param1: number, param2: number, param3: number, param4: number, param5: boolean): void;
				public constructor(param0: org.nativescript.canvas.TNSPath2D);
				public finalize(): void;
				public constructor(param0: string);
				public constructor();
				public quadraticCurveTo(param0: number, param1: number, param2: number, param3: number): void;
				public rect(param0: number, param1: number, param2: number, param3: number): void;
				public bezierCurveTo(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
				public ellipse(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: boolean): void;
				public addPath(param0: org.nativescript.canvas.TNSPath2D, param1: org.nativescript.canvas.TNSDOMMatrix): void;
				public addPath(param0: org.nativescript.canvas.TNSPath2D): void;
				public arcTo(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getPath$canvas_release(): number;
				public moveTo(param0: number, param1: number): void;
			}
			export module TNSPath2D {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSPath2D.Companion>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSPattern extends org.nativescript.canvas.TNSColorStyle {
				public static class: java.lang.Class<org.nativescript.canvas.TNSPattern>;
				public static Companion: org.nativescript.canvas.TNSPattern.Companion;
				public setStyle(param0: number): void;
				public finalize(): void;
				public getStyleType(): org.nativescript.canvas.TNSColorStyleType;
				public getStyle(): number;
				public constructor(param0: number);
				public setTransform(param0: org.nativescript.canvas.TNSDOMMatrix): void;
				public constructor();
			}
			export module TNSPattern {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSPattern.Companion>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSPatternRepetition {
				public static class: java.lang.Class<org.nativescript.canvas.TNSPatternRepetition>;
				public static Repeat: org.nativescript.canvas.TNSPatternRepetition;
				public static RepeatX: org.nativescript.canvas.TNSPatternRepetition;
				public static RepeatY: org.nativescript.canvas.TNSPatternRepetition;
				public static NoRepeat: org.nativescript.canvas.TNSPatternRepetition;
				public getPattern(): string;
				public toString(): string;
				public toNative(): number;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSPatternRepetition>;
				public static valueOf(param0: string): org.nativescript.canvas.TNSPatternRepetition;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSSVG {
				public static class: java.lang.Class<org.nativescript.canvas.TNSSVG>;
				public static Companion: org.nativescript.canvas.TNSSVG.Companion;
				public getBitmap$canvas_release(): globalAndroid.graphics.Bitmap;
				public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
				public setSrc(param0: string): void;
				public setSrcPath(param0: string): void;
				public flush(): void;
				public setBitmap$canvas_release(param0: globalAndroid.graphics.Bitmap): void;
				public setIgnorePixelScaling(param0: boolean): void;
				public onDraw(param0: globalAndroid.graphics.Canvas): void;
				public getLock$canvas_release(): any;
				public onSizeChanged(param0: number, param1: number, param2: number, param3: number): void;
				public constructor(param0: globalAndroid.content.Context);
				public getIgnorePixelScaling(): boolean;
				public constructor(param0: globalAndroid.content.Context, param1: boolean);
			}
			export module TNSSVG {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSSVG.Companion>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSTextAlignment {
				public static class: java.lang.Class<org.nativescript.canvas.TNSTextAlignment>;
				public static Start: org.nativescript.canvas.TNSTextAlignment;
				public static Left: org.nativescript.canvas.TNSTextAlignment;
				public static Center: org.nativescript.canvas.TNSTextAlignment;
				public static Right: org.nativescript.canvas.TNSTextAlignment;
				public static End: org.nativescript.canvas.TNSTextAlignment;
				public static Companion: org.nativescript.canvas.TNSTextAlignment.Companion;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSTextAlignment>;
				public isError$canvas_release(): boolean;
				public setError$canvas_release(param0: boolean): void;
				public toString(): string;
				public toNative(): number;
				public static valueOf(param0: string): org.nativescript.canvas.TNSTextAlignment;
			}
			export module TNSTextAlignment {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSTextAlignment.Companion>;
					public fromNative(param0: number): org.nativescript.canvas.TNSTextAlignment;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSTextBaseline {
				public static class: java.lang.Class<org.nativescript.canvas.TNSTextBaseline>;
				public static Top: org.nativescript.canvas.TNSTextBaseline;
				public static Hanging: org.nativescript.canvas.TNSTextBaseline;
				public static Middle: org.nativescript.canvas.TNSTextBaseline;
				public static Alphabetic: org.nativescript.canvas.TNSTextBaseline;
				public static Ideographic: org.nativescript.canvas.TNSTextBaseline;
				public static Bottom: org.nativescript.canvas.TNSTextBaseline;
				public getBaseLine(): string;
				public static valueOf(param0: string): org.nativescript.canvas.TNSTextBaseline;
				public setBaseLine(param0: string): void;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSTextBaseline>;
				public toString(): string;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSTextDecoder {
				public static class: java.lang.Class<org.nativescript.canvas.TNSTextDecoder>;
				public static Companion: org.nativescript.canvas.TNSTextDecoder.Companion;
				public getEncoding(): string;
				public decodeDoubleBuffer(param0: androidNative.Array<number>): string;
				public decode(param0: java.nio.ByteBuffer): string;
				public finalize(): void;
				public decodeIntBuffer(param0: androidNative.Array<number>): string;
				public decode(param0: androidNative.Array<number>): string;
				public decodeByteBuffer(param0: java.nio.ByteBuffer): string;
				public decodeFloatBuffer(param0: androidNative.Array<number>): string;
				public decodeShortBuffer(param0: androidNative.Array<number>): string;
				public constructor(param0: string);
				public constructor();
			}
			export module TNSTextDecoder {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSTextDecoder.Companion>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSTextDirection {
				public static class: java.lang.Class<org.nativescript.canvas.TNSTextDirection>;
				public static Ltr: org.nativescript.canvas.TNSTextDirection;
				public static Rtl: org.nativescript.canvas.TNSTextDirection;
				public toString(): string;
				public toNative(): number;
				public static values(): androidNative.Array<org.nativescript.canvas.TNSTextDirection>;
				public static valueOf(param0: string): org.nativescript.canvas.TNSTextDirection;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSTextEncoder {
				public static class: java.lang.Class<org.nativescript.canvas.TNSTextEncoder>;
				public static Companion: org.nativescript.canvas.TNSTextEncoder.Companion;
				public getEncoding(): string;
				public encode(param0: string): java.nio.ByteBuffer;
				public finalize(): void;
				public constructor(param0: string);
				public constructor();
			}
			export module TNSTextEncoder {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSTextEncoder.Companion>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSTextMetrics {
				public static class: java.lang.Class<org.nativescript.canvas.TNSTextMetrics>;
				public static Companion: org.nativescript.canvas.TNSTextMetrics.Companion;
				public getEmHeightDescent(): number;
				public getIdeographicBaseline(): number;
				public finalize(): void;
				public getActualBoundingBoxLeft(): number;
				public getActualBoundingBoxAscent(): number;
				public getEmHeightAscent(): number;
				public getActualBoundingBoxDescent(): number;
				public getWidth(): number;
				public getFontBoundingBoxAscent(): number;
				public getHangingBaseline(): number;
				public getFontBoundingBoxDescent(): number;
				public getActualBoundingBoxRight(): number;
				public constructor(param0: number);
				public getAlphabeticBaseline(): number;
			}
			export module TNSTextMetrics {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSTextMetrics.Companion>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSWebGL2RenderingContext extends org.nativescript.canvas.TNSWebGLRenderingContext {
				public static class: java.lang.Class<org.nativescript.canvas.TNSWebGL2RenderingContext>;
				public static Companion: org.nativescript.canvas.TNSWebGL2RenderingContext.Companion;
				public compressedTexSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.ByteBuffer, param10: number, param11: number): void;
				public getRG32UI(): number;
				public getSYNC_GPU_COMMANDS_COMPLETE(): number;
				public texImage3DFloat(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: androidNative.Array<number>): void;
				public getRGBA16F(): number;
				public uniform2uiv(param0: number, param1: androidNative.Array<number>): void;
				public getDRAW_BUFFER10(): number;
				public getRG8I(): number;
				public getUNIFORM_ARRAY_STRIDE(): number;
				public getUNPACK_SKIP_PIXELS(): number;
				public isVertexArray(param0: number): boolean;
				public getFLOAT_MAT4x3(): number;
				public constructor(param0: org.nativescript.canvas.TNSCanvas, param1: java.util.Map<string,any>);
				public texStorage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
				public getBufferSubData(param0: number, param1: number, param2: androidNative.Array<number>, param3: number, param4: number): void;
				public getUNSIGNED_INT_24_8(): number;
				public getTRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN(): number;
				public getMAX_CLIENT_WAIT_TIMEOUT_WEBGL(): number;
				public getFRAMEBUFFER_INCOMPLETE_MULTISAMPLE(): number;
				public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: java.nio.ShortBuffer, param11: number): void;
				public beginTransformFeedback(param0: number): void;
				public getDRAW_FRAMEBUFFER(): number;
				public getPIXEL_UNPACK_BUFFER_BINDING(): number;
				public getRGBA16UI(): number;
				public getANY_SAMPLES_PASSED_CONSERVATIVE(): number;
				public getDEPTH(): number;
				public getTEXTURE_2D_ARRAY(): number;
				public createTransformFeedback(): number;
				public fenceSync(param0: number, param1: number): void;
				public vertexAttribI4ui(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getUNIFORM_TYPE(): number;
				public endQuery(param0: number): void;
				public transformFeedbackVaryings(param0: number, param1: androidNative.Array<string>, param2: number): void;
				public getDRAW_BUFFER5(): number;
				public getSAMPLER_BINDING(): number;
				public getMAX_UNIFORM_BUFFER_BINDINGS(): number;
				public texSubImage3DInt(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: androidNative.Array<number>, param11: number): void;
				public getSAMPLER_CUBE_SHADOW(): number;
				public getFRAMEBUFFER_ATTACHMENT_RED_SIZE(): number;
				public getR16F(): number;
				public getUNSIGNED_INT_VEC2(): number;
				public getActiveUniformBlockName(param0: number, param1: number): string;
				public vertexAttribDivisor(param0: number, param1: number): void;
				public getUNSIGNED_INT_SAMPLER_2D(): number;
				public getCOLOR_ATTACHMENT14(): number;
				public getTEXTURE_3D(): number;
				public getR16UI(): number;
				public getUNSIGNED_INT_SAMPLER_3D(): number;
				public texImage3DDoubleBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.DoubleBuffer): void;
				public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.IntBuffer): void;
				public compressedTexSubImage3DByteBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.ByteBuffer, param10: number, param11: number): void;
				public drawBuffers(param0: androidNative.Array<number>): void;
				public getUNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER(): number;
				public getR32F(): number;
				public getDRAW_BUFFER7(): number;
				public texImage3DShortBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.ShortBuffer): void;
				public getRGB10_A2UI(): number;
				public getVERTEX_ARRAY_BINDING(): number;
				public uniformMatrix2x3fv(param0: number, param1: boolean, param2: androidNative.Array<number>): void;
				public getTRANSFORM_FEEDBACK_BUFFER_BINDING(): number;
				public compressedTexSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: androidNative.Array<number>, param10: number, param11: number): void;
				public getPIXEL_PACK_BUFFER(): number;
				public getMAX_PROGRAM_TEXEL_OFFSET(): number;
				public getTRANSFORM_FEEDBACK_ACTIVE(): number;
				public getUNIFORM_BLOCK_BINDING(): number;
				public getRG_INTEGER(): number;
				public getSYNC_FLAGS(): number;
				public drawRangeElements(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
				public getANY_SAMPLES_PASSED(): number;
				public vertexAttribI4uiv(param0: number, param1: androidNative.Array<number>): void;
				public getTEXTURE_MAX_LOD(): number;
				public getTransformFeedbackVarying(param0: number, param1: number): any;
				public renderbufferStorageMultisample(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getUNSIGNED_INT_10F_11F_11F_REV(): number;
				public getPACK_SKIP_PIXELS(): number;
				public texSubImage3DByte(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: androidNative.Array<number>, param11: number): void;
				public getTRANSFORM_FEEDBACK_BUFFER(): number;
				public getRGB8(): number;
				public getTRANSFORM_FEEDBACK_PAUSED(): number;
				public getUNSIGNED_INT_VEC4(): number;
				public getMAX_VERTEX_UNIFORM_COMPONENTS(): number;
				public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: androidNative.Array<number>, param11: number): void;
				public uniform2ui(param0: number, param1: number, param2: number): void;
				public uniformMatrix3x2fvBuffer(param0: number, param1: boolean, param2: java.nio.FloatBuffer): void;
				public getCOLOR_ATTACHMENT4(): number;
				public getMAX(): number;
				public getINT_SAMPLER_CUBE(): number;
				public getSTREAM_READ(): number;
				public getCOPY_READ_BUFFER_BINDING(): number;
				public getPIXEL_PACK_BUFFER_BINDING(): number;
				public bindTransformFeedback(param0: number, param1: number): void;
				public getUNSIGNED_INT_5_9_9_9_REV(): number;
				public getCOPY_WRITE_BUFFER_BINDING(): number;
				public getUNIFORM_MATRIX_STRIDE(): number;
				public getCOLOR_ATTACHMENT6(): number;
				public getVERTEX_ATTRIB_ARRAY_INTEGER(): number;
				public beginQuery(param0: number, param1: number): void;
				public getRG8UI(): number;
				public getIndexedParameter(param0: number, param1: number): any;
				public getDRAW_BUFFER12(): number;
				public getFRAMEBUFFER_ATTACHMENT_GREEN_SIZE(): number;
				public getQueryParameter(param0: number, param1: number): any;
				public copyTexSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
				public uniform4uiv(param0: number, param1: androidNative.Array<number>): void;
				public getRGB32UI(): number;
				public getSYNC_STATUS(): number;
				public uniformMatrix3x4fv(param0: number, param1: boolean, param2: androidNative.Array<number>): void;
				public getTRANSFORM_FEEDBACK_VARYINGS(): number;
				public getCOPY_WRITE_BUFFER(): number;
				public getRGB9_E5(): number;
				public pauseTransformFeedback(): void;
				public texSubImage3DIntBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: java.nio.IntBuffer, param11: number): void;
				public texImage3DLongBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.LongBuffer): void;
				public getDRAW_BUFFER3(): number;
				public getR32I(): number;
				public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: java.nio.FloatBuffer, param11: number): void;
				public samplerParameterf(param0: number, param1: number, param2: number): void;
				public getQUERY_RESULT_AVAILABLE(): number;
				public getTRANSFORM_FEEDBACK(): number;
				public getCONDITION_SATISFIED(): number;
				public getUNIFORM_BUFFER_OFFSET_ALIGNMENT(): number;
				public texImage3DDouble(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: androidNative.Array<number>): void;
				public isSync(param0: number): boolean;
				public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: org.nativescript.canvas.TNSImageAsset): void;
				public getDRAW_BUFFER8(): number;
				public getRENDERBUFFER_SAMPLES(): number;
				public getUNSIGNED_INT_2_10_10_10_REV(): number;
				public getSYNC_FLUSH_COMMANDS_BIT(): number;
				public getMAX_ELEMENT_INDEX(): number;
				public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: globalAndroid.graphics.Bitmap): void;
				public getUniformIndices(param0: number, param1: androidNative.Array<string>): androidNative.Array<number>;
				public deleteQuery(param0: number): void;
				public getDEPTH_COMPONENT32F(): number;
				public getMAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS(): number;
				public copyBufferSubData(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getRGBA8_SNORM(): number;
				public getINT_SAMPLER_2D_ARRAY(): number;
				public compressedTexSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: number): void;
				public getCOLOR_ATTACHMENT15(): number;
				public clearBufferfi(param0: number, param1: number, param2: number, param3: number): void;
				public getINVALID_INDEX(): number;
				public uniformMatrix3x2fv(param0: number, param1: boolean, param2: androidNative.Array<number>): void;
				public getPACK_ROW_LENGTH(): number;
				public getMAX_ARRAY_TEXTURE_LAYERS(): number;
				public fromGLint(param0: androidNative.Array<number>): androidNative.Array<boolean>;
				public isTransformFeedback(param0: number): boolean;
				public drawArraysInstanced(param0: number, param1: number, param2: number, param3: number): void;
				public texImage3DInt(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: androidNative.Array<number>): void;
				public getUNSIGNED_INT_SAMPLER_CUBE(): number;
				public getRG16F(): number;
				public texSubImage3DFloatBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: java.nio.FloatBuffer, param11: number): void;
				public getRGB10_A2(): number;
				public getFLOAT_MAT2x3(): number;
				public uniformMatrix4x3fv(param0: number, param1: boolean, param2: androidNative.Array<number>): void;
				public getR8(): number;
				public getMAX_ELEMENTS_VERTICES(): number;
				public getFRAMEBUFFER_ATTACHMENT_BLUE_SIZE(): number;
				public getTIMEOUT_IGNORED(): number;
				public getUNIFORM_BLOCK_INDEX(): number;
				public getDRAW_BUFFER4(): number;
				public getFRAGMENT_SHADER_DERIVATIVE_HINT(): number;
				public getRGBA8I(): number;
				public getCOLOR_ATTACHMENT3(): number;
				public uniformMatrix4x2fv(param0: number, param1: boolean, param2: androidNative.Array<number>): void;
				public getTEXTURE_BINDING_3D(): number;
				public bindVertexArray(param0: number): void;
				public getRG(): number;
				public getMAX_SAMPLES(): number;
				public endTransformFeedback(): void;
				public getMAX_VARYING_COMPONENTS(): number;
				public texStorage2D(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getCOLOR_ATTACHMENT12(): number;
				public getTRANSFORM_FEEDBACK_BUFFER_SIZE(): number;
				public getUNIFORM_IS_ROW_MAJOR(): number;
				public deleteSampler(param0: number): void;
				public uniform4ui(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getUNIFORM_SIZE(): number;
				public getCOPY_READ_BUFFER(): number;
				public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.LongBuffer): void;
				public uniformMatrix2x3fvBuffer(param0: number, param1: boolean, param2: java.nio.FloatBuffer): void;
				public getCOLOR_ATTACHMENT7(): number;
				public getR8_SNORM(): number;
				public getFragDataLocation(param0: number, param1: string): number;
				public getR32UI(): number;
				public getMAX_FRAGMENT_UNIFORM_BLOCKS(): number;
				public getDRAW_BUFFER0(): number;
				public getTEXTURE_MIN_LOD(): number;
				public getRGBA_INTEGER(): number;
				public texImage3DShort(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: androidNative.Array<number>): void;
				public uniformMatrix3x4fvBuffer(param0: number, param1: boolean, param2: java.nio.FloatBuffer): void;
				public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.ByteBuffer): void;
				public getDRAW_BUFFER13(): number;
				public getRASTERIZER_DISCARD(): number;
				public getMIN_PROGRAM_TEXEL_OFFSET(): number;
				public getUNIFORM_BLOCK_ACTIVE_UNIFORMS(): number;
				public uniform2uivBuffer(param0: number, param1: java.nio.IntBuffer): void;
				public getDYNAMIC_READ(): number;
				public getCOLOR_ATTACHMENT13(): number;
				public getUNSIGNED_INT_VEC3(): number;
				public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: java.nio.DoubleBuffer, param11: number): void;
				public getRGBA32UI(): number;
				public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: globalAndroid.graphics.Bitmap): void;
				public getDEPTH24_STENCIL8(): number;
				public vertexAttribI4iv(param0: number, param1: androidNative.Array<number>): void;
				public getRGB32I(): number;
				public getR8I(): number;
				public texImage3DFloatBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.FloatBuffer): void;
				public getFRAMEBUFFER_ATTACHMENT_STENCIL_SIZE(): number;
				public getDYNAMIC_COPY(): number;
				public getTEXTURE_WRAP_R(): number;
				public getTRANSFORM_FEEDBACK_BUFFER_START(): number;
				public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: org.nativescript.canvas.TNSCanvas): void;
				public createSampler(): number;
				public getFRAMEBUFFER_ATTACHMENT_ALPHA_SIZE(): number;
				public getInternalformatParameter(param0: number, param1: number, param2: number): any;
				public getINT_2_10_10_10_REV(): number;
				public texSubImage3DShort(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: androidNative.Array<number>, param11: number): void;
				public readBuffer(param0: number): void;
				public uniform1ui(param0: number, param1: number): void;
				public getMAX_DRAW_BUFFERS(): number;
				public texSubImage3DLong(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: androidNative.Array<number>, param11: number): void;
				public getFRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE(): number;
				public resumeTransformFeedback(): void;
				public getSIGNED_NORMALIZED(): number;
				public getFRAMEBUFFER_ATTACHMENT_COLOR_ENCODING(): number;
				public getSEPARATE_ATTRIBS(): number;
				public uniformMatrix4x2fvBuffer(param0: number, param1: boolean, param2: java.nio.FloatBuffer): void;
				public getPIXEL_UNPACK_BUFFER(): number;
				public getDEPTH32F_STENCIL8(): number;
				public getUNPACK_SKIP_IMAGES(): number;
				public vertexAttribI4ivBuffer(param0: number, param1: java.nio.IntBuffer): void;
				public getFLOAT_32_UNSIGNED_INT_24_8_REV(): number;
				public getINT_SAMPLER_3D(): number;
				public getMAX_COMBINED_VERTEX_UNIFORM_COMPONENTS(): number;
				public texSubImage3DFloat(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: androidNative.Array<number>, param11: number): void;
				public getRGB8I(): number;
				public getINT_SAMPLER_2D(): number;
				public getUNIFORM_BUFFER(): number;
				public texImage3DByte(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: androidNative.Array<number>): void;
				public samplerParameteri(param0: number, param1: number, param2: number): void;
				public getRG16UI(): number;
				public getUNSIGNED_INT_SAMPLER_2D_ARRAY(): number;
				public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: java.nio.IntBuffer, param11: number): void;
				public getUNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER(): number;
				public getRGB16I(): number;
				public getDRAW_BUFFER6(): number;
				public getUniformBlockIndex(param0: number, param1: string): number;
				public getRGBA8(): number;
				public vertexAttribI4i(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getFLOAT_MAT2x4(): number;
				public invalidateFramebuffer(param0: number, param1: androidNative.Array<number>): void;
				public getRG8_SNORM(): number;
				public getSTATIC_READ(): number;
				public getDRAW_BUFFER1(): number;
				public getMAX_COLOR_ATTACHMENTS(): number;
				public getMAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS(): number;
				public getFRAMEBUFFER_ATTACHMENT_DEPTH_SIZE(): number;
				public getMAX_3D_TEXTURE_SIZE(): number;
				public getRG16I(): number;
				public getSamplerParameter(param0: number, param1: number): any;
				public drawBuffers(param0: java.nio.IntBuffer): void;
				public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: org.nativescript.canvas.TNSCanvas): void;
				public getActiveUniforms(param0: number, param1: androidNative.Array<number>, param2: number): any;
				public getRGB8_SNORM(): number;
				public getRGB16F(): number;
				public getFLOAT_MAT3x2(): number;
				public getDEPTH_COMPONENT24(): number;
				public getWAIT_FAILED(): number;
				public getTIMEOUT_EXPIRED(): number;
				public getALREADY_SIGNALED(): number;
				public getCOLOR_ATTACHMENT11(): number;
				public getDRAW_FRAMEBUFFER_BINDING(): number;
				public blitFramebuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number): void;
				public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.DoubleBuffer): void;
				public deleteTransformFeedback(param0: number): void;
				public drawElementsInstanced(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getTEXTURE_COMPARE_FUNC(): number;
				public getRED(): number;
				public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: number): void;
				public deleteSync(param0: number): void;
				public getFRAMEBUFFER_DEFAULT(): number;
				public getSAMPLER_2D_SHADOW(): number;
				public uniformBlockBinding(param0: number, param1: number, param2: number): void;
				public getCOMPARE_REF_TO_TEXTURE(): number;
				public getDRAW_BUFFER14(): number;
				public getTEXTURE_BINDING_2D_ARRAY(): number;
				public getQuery(param0: number, param1: number): any;
				public getSIGNALED(): number;
				public framebufferTextureLayer(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getTEXTURE_MAX_LEVEL(): number;
				public createVertexArray(): number;
				public getSRGB(): number;
				public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: java.nio.LongBuffer, param11: number): void;
				public uniform3uivBuffer(param0: number, param1: java.nio.IntBuffer): void;
				public uniformMatrix4x3fvBuffer(param0: number, param1: boolean, param2: java.nio.FloatBuffer): void;
				public getREAD_BUFFER(): number;
				public getFRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER(): number;
				public getSYNC_FENCE(): number;
				public clearBufferuiv(param0: number, param1: number, param2: androidNative.Array<number>): void;
				public uniform3ui(param0: number, param1: number, param2: number, param3: number): void;
				public getUNPACK_ROW_LENGTH(): number;
				public getMAX_TEXTURE_LOD_BIAS(): number;
				public uniform1uivBuffer(param0: number, param1: java.nio.IntBuffer): void;
				public getSTENCIL(): number;
				public getFLOAT_MAT3x4(): number;
				public getR11F_G11F_B10F(): number;
				public isQuery(param0: number): boolean;
				public getSRGB8_ALPHA8(): number;
				public bindBufferBase(param0: number, param1: number, param2: number): void;
				public getTEXTURE_COMPARE_MODE(): number;
				public getTRANSFORM_FEEDBACK_BUFFER_MODE(): number;
				public getCOLOR_ATTACHMENT8(): number;
				public getUNIFORM_BUFFER_SIZE(): number;
				public texSubImage3DDouble(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: androidNative.Array<number>, param11: number): void;
				public bindBufferRange(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getDRAW_BUFFER9(): number;
				public getSYNC_CONDITION(): number;
				public getCOLOR_ATTACHMENT10(): number;
				public getOBJECT_TYPE(): number;
				public getRGBA16I(): number;
				public getMAX_SERVER_WAIT_TIMEOUT(): number;
				public getRG32F(): number;
				public texSubImage3DShortBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: java.nio.ShortBuffer, param11: number): void;
				public texSubImage3DLongBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: java.nio.LongBuffer, param11: number): void;
				public createQuery(): number;
				public getHALF_FLOAT(): number;
				public getINTERLEAVED_ATTRIBS(): number;
				public constructor(param0: org.nativescript.canvas.TNSCanvas);
				public getDRAW_BUFFER2(): number;
				public getCOLOR_ATTACHMENT2(): number;
				public getACTIVE_UNIFORM_BLOCKS(): number;
				public getRGB32F(): number;
				public getUNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES(): number;
				public getMAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS(): number;
				public vertexAttribI4uivBuffer(param0: number, param1: java.nio.IntBuffer): void;
				public getCOLOR_ATTACHMENT1(): number;
				public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.ShortBuffer): void;
				public getTEXTURE_BASE_LEVEL(): number;
				public texSubImage3DDoubleBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: java.nio.DoubleBuffer, param11: number): void;
				public drawBuffersBuffer(param0: java.nio.IntBuffer): void;
				public clearBufferfv(param0: number, param1: number, param2: androidNative.Array<number>): void;
				public isSampler(param0: number): boolean;
				public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: androidNative.Array<number>): void;
				public getCURRENT_QUERY(): number;
				public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.FloatBuffer): void;
				public getSRGB8(): number;
				public getRGBA32I(): number;
				public getParameter(param0: number): any;
				public getRGBA8UI(): number;
				public uniformMatrix2x4fvBuffer(param0: number, param1: boolean, param2: java.nio.FloatBuffer): void;
				public clearBufferiv(param0: number, param1: number, param2: androidNative.Array<number>): void;
				public getMAX_UNIFORM_BLOCK_SIZE(): number;
				public getMIN(): number;
				public getREAD_FRAMEBUFFER(): number;
				public getUNSIGNALED(): number;
				public getRG8(): number;
				public getPACK_SKIP_ROWS(): number;
				public getVERTEX_ATTRIB_ARRAY_DIVISOR(): number;
				public texImage3DByteBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.ByteBuffer): void;
				public uniform1uiv(param0: number, param1: androidNative.Array<number>): void;
				public getUNPACK_SKIP_ROWS(): number;
				public getUNPACK_IMAGE_HEIGHT(): number;
				public getMAX_FRAGMENT_UNIFORM_COMPONENTS(): number;
				public getRGB_INTEGER(): number;
				public uniform3uiv(param0: number, param1: androidNative.Array<number>): void;
				public getCOLOR(): number;
				public getMAX_VERTEX_OUTPUT_COMPONENTS(): number;
				public getRGB8UI(): number;
				public getSAMPLER_3D(): number;
				public getDRAW_BUFFER15(): number;
				public getRGBA32F(): number;
				public texImage3DIntBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: java.nio.IntBuffer): void;
				public invalidateSubFramebuffer(param0: number, param1: androidNative.Array<number>, param2: number, param3: number, param4: number, param5: number): void;
				public uniformMatrix2x4fv(param0: number, param1: boolean, param2: androidNative.Array<number>): void;
				public getR8UI(): number;
				public getFLOAT_MAT4x2(): number;
				public getMAX_VERTEX_UNIFORM_BLOCKS(): number;
				public getQUERY_RESULT(): number;
				public getUNIFORM_BLOCK_DATA_SIZE(): number;
				public getSyncParameter(param0: number, param1: number): any;
				public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number): void;
				public texImage3DLong(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: androidNative.Array<number>): void;
				public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: java.nio.ByteBuffer, param11: number): void;
				public getCOLOR_ATTACHMENT9(): number;
				public getUNIFORM_BUFFER_BINDING(): number;
				public getSTREAM_COPY(): number;
				public getTEXTURE_IMMUTABLE_LEVELS(): number;
				public getUNSIGNED_NORMALIZED(): number;
				public getMAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS(): number;
				public getMAX_ELEMENTS_INDICES(): number;
				public getSAMPLER_2D_ARRAY(): number;
				public getUNIFORM_BUFFER_START(): number;
				public getRGB16UI(): number;
				public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: org.nativescript.canvas.TNSImageAsset): void;
				public getActiveUniformBlockParameter(param0: number, param1: number, param2: number): any;
				public uniform4uivBuffer(param0: number, param1: java.nio.IntBuffer): void;
				public getSTATIC_COPY(): number;
				public bindSampler(param0: number, param1: number): void;
				public deleteVertexArray(param0: number): void;
				public getR16I(): number;
				public clientWaitSync(param0: number, param1: number, param2: number): number;
				public getDRAW_BUFFER11(): number;
				public getMAX_COMBINED_UNIFORM_BLOCKS(): number;
				public getREAD_FRAMEBUFFER_BINDING(): number;
				public getSAMPLER_2D_ARRAY_SHADOW(): number;
				public texSubImage3DByteBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: java.nio.ByteBuffer, param11: number): void;
				public getTRANSFORM_FEEDBACK_BINDING(): number;
				public getMAX_FRAGMENT_INPUT_COMPONENTS(): number;
				public getUNIFORM_OFFSET(): number;
				public getRG32I(): number;
				public getCOLOR_ATTACHMENT5(): number;
				public getRED_INTEGER(): number;
				public getTEXTURE_IMMUTABLE_FORMAT(): number;
			}
			export module TNSWebGL2RenderingContext {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSWebGL2RenderingContext.Companion>;
				}
				export class ReturnType {
					public static class: java.lang.Class<org.nativescript.canvas.TNSWebGL2RenderingContext.ReturnType>;
					public static EnumType: org.nativescript.canvas.TNSWebGL2RenderingContext.ReturnType;
					public static UnsignedIntType: org.nativescript.canvas.TNSWebGL2RenderingContext.ReturnType;
					public static IntType: org.nativescript.canvas.TNSWebGL2RenderingContext.ReturnType;
					public static BoolType: org.nativescript.canvas.TNSWebGL2RenderingContext.ReturnType;
					public static values(): androidNative.Array<org.nativescript.canvas.TNSWebGL2RenderingContext.ReturnType>;
					public static valueOf(param0: string): org.nativescript.canvas.TNSWebGL2RenderingContext.ReturnType;
				}
				export class WhenMappings {
					public static class: java.lang.Class<org.nativescript.canvas.TNSWebGL2RenderingContext.WhenMappings>;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class TNSWebGLRenderingContext extends org.nativescript.canvas.TNSCanvasRenderingContext {
				public static class: java.lang.Class<org.nativescript.canvas.TNSWebGLRenderingContext>;
				public static SIZE_OF_BYTE: number;
				public static SIZE_OF_SHORT: number;
				public static SIZE_OF_INT: number;
				public static SIZE_OF_LONG: number;
				public static SIZE_OF_FLOAT: number;
				public static SIZE_OF_DOUBLE: number;
				public static SIZE_OF_CHAR: number;
				public static Companion: org.nativescript.canvas.TNSWebGLRenderingContext.Companion;
				public getINT_VEC2(): number;
				public getDEPTH_STENCIL(): number;
				public getSAMPLE_COVERAGE(): number;
				public compressedTexImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.FloatBuffer): void;
				public blendColor(param0: number, param1: number, param2: number, param3: number): void;
				public getPOLYGON_OFFSET_FACTOR(): number;
				public getTEXTURE16(): number;
				public cullFace(param0: number): void;
				public getTEXTURE_CUBE_MAP_POSITIVE_Z(): number;
				public getTEXTURE26(): number;
				public getRENDERBUFFER_RED_SIZE(): number;
				public getFLOAT(): number;
				public getMEDIUM_INT(): number;
				public sampleCoverage(param0: number, param1: boolean): void;
				public getUNSIGNED_SHORT_5_6_5(): number;
				public getUNPACK_PREMULTIPLY_ALPHA_WEBGL(): number;
				public getNEAREST(): number;
				public constructor(param0: org.nativescript.canvas.TNSCanvas, param1: java.util.Map<string,any>);
				public compressedTexSubImage2DShortBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: java.nio.ShortBuffer): void;
				public getFUNC_SUBTRACT(): number;
				public compressedTexImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: androidNative.Array<number>): void;
				public getTEXTURE_MIN_FILTER(): number;
				public depthMask(param0: boolean): void;
				public getUNSIGNED_SHORT(): number;
				public getBUFFER_SIZE(): number;
				public compressedTexImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.ShortBuffer): void;
				public getBLEND_EQUATION_RGB(): number;
				public getBOOL_VEC4(): number;
				public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: org.nativescript.canvas.TNSImageAsset): void;
				public getRGB565(): number;
				public clearStencil(param0: number): void;
				public detachShader(param0: number, param1: number): void;
				public getCONSTANT_ALPHA(): number;
				public texParameterf(param0: number, param1: number, param2: number): void;
				public getTEXTURE(): number;
				public texImage2DIntBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.IntBuffer): void;
				public getVERTEX_ATTRIB_ARRAY_STRIDE(): number;
				public texImage2DInt(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: androidNative.Array<number>): void;
				public getGL_FLOAT$canvas_release(): number;
				public getELEMENT_ARRAY_BUFFER_BINDING(): number;
				public getLEQUAL(): number;
				public getTEXTURE_BINDING_CUBE_MAP(): number;
				public getGL_HALF_FLOAT$canvas_release(): number;
				public getCULL_FACE_MODE(): number;
				public getContextAttributes(): java.util.Map<string,any>;
				public getSTENCIL_CLEAR_VALUE(): number;
				public setCanvas$canvas_release(param0: org.nativescript.canvas.TNSCanvas): void;
				public deleteProgram(param0: number): void;
				public uniform3fvBuffer(param0: number, param1: java.nio.FloatBuffer): void;
				public getREPLACE(): number;
				public getDEPTH_RANGE(): number;
				public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.IntBuffer): void;
				public bufferData(param0: number, param1: androidNative.Array<number>, param2: number): void;
				public getCLAMP_TO_EDGE(): number;
				public vertexAttrib2fvBuffer(param0: number, param1: java.nio.FloatBuffer): void;
				public getCOLOR_BUFFER_BIT(): number;
				public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageAsset): void;
				public bufferSubData(param0: number, param1: number, param2: java.nio.IntBuffer): void;
				public uniform2ivBuffer(param0: number, param1: java.nio.IntBuffer): void;
				public getTEXTURE1(): number;
				public bufferSubDataFloat(param0: number, param1: number, param2: androidNative.Array<number>): void;
				public uniform2fv(param0: number, param1: androidNative.Array<number>): void;
				public getTEXTURE_2D(): number;
				public getINVALID_VALUE(): number;
				public isContextLost(): boolean;
				public texSubImage2DIntBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.IntBuffer): void;
				public getNOTEQUAL(): number;
				public shaderSource(param0: number, param1: string): void;
				public vertexAttrib3fvBuffer(param0: number, param1: java.nio.FloatBuffer): void;
				public getCanvas$canvas_release(): org.nativescript.canvas.TNSCanvas;
				public getError(): number;
				public getACTIVE_TEXTURE(): number;
				public compressedTexImage2DShort(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: androidNative.Array<number>): void;
				public getVERTEX_ATTRIB_ARRAY_NORMALIZED(): number;
				public getFRAMEBUFFER_INCOMPLETE_ATTACHMENT(): number;
				public readPixelsShort(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: androidNative.Array<number>): void;
				public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.ByteBuffer): void;
				public getMEDIUM_FLOAT(): number;
				public getFRAMEBUFFER_UNSUPPORTED(): number;
				public getBOOL(): number;
				public static nativeTexImage2DTexture(param0: number, param1: number, param2: number, param3: number): void;
				public uniform1fv(param0: number, param1: androidNative.Array<number>): void;
				public getTEXTURE_WRAP_S(): number;
				public enableVertexAttribArray(param0: number): void;
				public getDONT_CARE(): number;
				public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: org.nativescript.canvas.TNSCanvas): void;
				public getMAX_VARYING_VECTORS(): number;
				public uniformMatrix3fv(param0: number, param1: boolean, param2: androidNative.Array<number>): void;
				public getSHADER_TYPE(): number;
				public getBLEND_DST_RGB(): number;
				public bufferData(param0: number, param1: java.nio.FloatBuffer, param2: number): void;
				public uniform4f(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public uniform2fvBuffer(param0: number, param1: java.nio.FloatBuffer): void;
				public vertexAttrib1fv(param0: number, param1: androidNative.Array<number>): void;
				public getTEXTURE28(): number;
				public getAttachedShaders(param0: number): androidNative.Array<number>;
				public texSubImage2DInt(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: androidNative.Array<number>): void;
				public uniformMatrix2fv(param0: number, param1: boolean, param2: androidNative.Array<number>): void;
				public getFRAMEBUFFER_BINDING(): number;
				public getOUT_OF_MEMORY(): number;
				public getCOLOR_CLEAR_VALUE(): number;
				public getTEXTURE21(): number;
				public getTRIANGLES(): number;
				public compressedTexImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.IntBuffer): void;
				public getONE_MINUS_SRC_COLOR(): number;
				public getMAX_RENDERBUFFER_SIZE(): number;
				public getGL_RGBA$canvas_release(): number;
				public compressedTexSubImage2DByte(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: androidNative.Array<number>): void;
				public getRenderbufferParameter(param0: number, param1: number): number;
				public getSRC_ALPHA(): number;
				public getDITHER(): number;
				public getFRAMEBUFFER_COMPLETE(): number;
				public getLINEAR_MIPMAP_NEAREST(): number;
				public getACTIVE_ATTRIBUTES(): number;
				public getTexParameter(param0: number, param1: number): number;
				public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.FloatBuffer): void;
				public getDELETE_STATUS(): number;
				public getGL_RGB$canvas_release(): number;
				public getVERTEX_ATTRIB_ARRAY_TYPE(): number;
				public getMIRRORED_REPEAT(): number;
				public bufferData(param0: number, param1: java.nio.ByteBuffer, param2: number): void;
				public texSubImage2DShort(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: androidNative.Array<number>): void;
				public compressedTexImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.ByteBuffer): void;
				public getDEPTH_BITS(): number;
				public getVertexAttrib(param0: number, param1: number): any;
				public getNEAREST_MIPMAP_NEAREST(): number;
				public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.ShortBuffer): void;
				public bufferSubDataByte(param0: number, param1: number, param2: androidNative.Array<number>): void;
				public bufferSubDataInt(param0: number, param1: number, param2: androidNative.Array<number>): void;
				public getSTENCIL_VALUE_MASK(): number;
				public attachShader(param0: number, param1: number): void;
				public texParameteri(param0: number, param1: number, param2: number): void;
				public compressedTexSubImage2DIntBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: java.nio.IntBuffer): void;
				public getLINE_LOOP(): number;
				public getSTATIC_DRAW(): number;
				public getONE(): number;
				public deleteTexture(param0: number): void;
				public updateCanvas(): void;
				public bindTexture(param0: number, param1: number): void;
				public getFLOAT_MAT2(): number;
				public readPixels(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.IntBuffer): void;
				public getCURRENT_PROGRAM(): number;
				public getDrawingBufferHeight(): number;
				public getUniformLocation(param0: number, param1: string): number;
				public getBOOL_VEC2(): number;
				public clear(param0: number): void;
				public linkProgram(param0: number): void;
				public getTEXTURE23(): number;
				public getSRC_COLOR(): number;
				public createBuffer(): number;
				public getBLEND_SRC_ALPHA(): number;
				public getFRONT_FACE(): number;
				public getMAX_TEXTURE_IMAGE_UNITS(): number;
				public getShaderSource(param0: number): string;
				public uniform1fvBuffer(param0: number, param1: java.nio.FloatBuffer): void;
				public clearIfComposited(param0: number): org.nativescript.canvas.HowToClear;
				public texImage2DShort(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: androidNative.Array<number>): void;
				public getShaderParameter(param0: number, param1: number): any;
				public getUNSIGNED_BYTE(): number;
				public deleteRenderbuffer(param0: number): void;
				public getVIEWPORT(): number;
				public getSTENCIL_BACK_PASS_DEPTH_PASS(): number;
				public getNEAREST_MIPMAP_LINEAR(): number;
				public getDEPTH_COMPONENT(): number;
				public getLINEAR_MIPMAP_LINEAR(): number;
				public getSCISSOR_BOX(): number;
				public uniform4ivBuffer(param0: number, param1: java.nio.IntBuffer): void;
				public bufferDataShortBuffer(param0: number, param1: java.nio.ShortBuffer, param2: number): void;
				public uniform4fvBuffer(param0: number, param1: java.nio.FloatBuffer): void;
				public getCCW(): number;
				public getGEQUAL(): number;
				public getTEXTURE3(): number;
				public texSubImage2DByte(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: androidNative.Array<number>): void;
				public getLOW_INT(): number;
				public getSCISSOR_TEST(): number;
				public getGL_LUMINANCE$canvas_release(): number;
				public getTEXTURE19(): number;
				public disableVertexAttribArray(param0: number): void;
				public getGL_UNSIGNED_SHORT_5_5_5_1$canvas_release(): number;
				public getONE_MINUS_CONSTANT_COLOR(): number;
				public bufferSubDataFloatBuffer(param0: number, param1: number, param2: java.nio.FloatBuffer): void;
				public getARRAY_BUFFER_BINDING(): number;
				public getGL_ALPHA$canvas_release(): number;
				public getRGBA4(): number;
				public getRENDERBUFFER_INTERNAL_FORMAT(): number;
				public bufferDataByte(param0: number, param1: androidNative.Array<number>, param2: number): void;
				public readPixelsByte(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: androidNative.Array<number>): void;
				public getSTENCIL_FAIL(): number;
				public getINCR_WRAP(): number;
				public getUNSIGNED_INT(): number;
				public getEQUAL(): number;
				public bufferData(param0: number, param1: java.nio.ShortBuffer, param2: number): void;
				public getCOMPRESSED_TEXTURE_FORMATS(): number;
				public getTEXTURE_CUBE_MAP_POSITIVE_X(): number;
				public getTEXTURE14(): number;
				public getRGB5_A1(): number;
				public compressedTexSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: java.nio.ByteBuffer): void;
				public uniform1f(param0: number, param1: number): void;
				public getBLEND_COLOR(): number;
				public activeTexture(param0: number): void;
				public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.IntBuffer): void;
				public getINT_VEC4(): number;
				public stencilMaskSeparate(param0: number, param1: number): void;
				public getVERSION(): number;
				public blendFuncSeparate(param0: number, param1: number, param2: number, param3: number): void;
				public getPOLYGON_OFFSET_FILL(): number;
				public uniformMatrix4fv(param0: number, param1: boolean, param2: androidNative.Array<number>): void;
				public compressedTexSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: java.nio.IntBuffer): void;
				public texImage2DFloat(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: androidNative.Array<number>): void;
				public getTEXTURE24(): number;
				public deleteBuffer(param0: number): void;
				public uniform3fv(param0: number, param1: androidNative.Array<number>): void;
				public disable(param0: number): void;
				public createProgram(): number;
				public getGREEN_BITS(): number;
				public bufferDataByteBuffer(param0: number, param1: java.nio.ByteBuffer, param2: number): void;
				public getBufferParameter(param0: number, param1: number): number;
				public frontFace(param0: number): void;
				public getMAX_COMBINED_TEXTURE_IMAGE_UNITS(): number;
				public getDEPTH_FUNC(): number;
				public getFramebufferAttachmentParameter(param0: number, param1: number, param2: number): org.nativescript.canvas.TNSFramebufferAttachmentParameter;
				public flush(): void;
				public compressedTexImage2DByteBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.ByteBuffer): void;
				public blendEquation(param0: number): void;
				public compressedTexImage2DFloatBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.FloatBuffer): void;
				public getDEPTH_WRITEMASK(): number;
				public readPixelsByteBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.ByteBuffer): void;
				public bindBuffer(param0: number, param1: any): void;
				public uniform2iv(param0: number, param1: androidNative.Array<number>): void;
				public uniform4fv(param0: number, param1: androidNative.Array<number>): void;
				public bindFramebuffer(param0: number, param1: number): void;
				public texImage2DShortBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.ShortBuffer): void;
				public getProgramParameter(param0: number, param1: number): any;
				public texImage2DByte(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: androidNative.Array<number>): void;
				public getTEXTURE11(): number;
				public getFUNC_REVERSE_SUBTRACT(): number;
				public getTEXTURE18(): number;
				public getRENDERBUFFER(): number;
				public getTEXTURE2(): number;
				public compressedTexImage2DInt(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: androidNative.Array<number>): void;
				public drawArrays(param0: number, param1: number, param2: number): void;
				public getLINE_WIDTH(): number;
				public getPOINTS(): number;
				public bufferSubDataShortBuffer(param0: number, param1: number, param2: java.nio.ShortBuffer): void;
				public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSImageBitmap): void;
				public uniform4iv(param0: number, param1: androidNative.Array<number>): void;
				public getDECR_WRAP(): number;
				public depthRange(param0: number, param1: number): void;
				public getDEPTH_COMPONENT16(): number;
				public getPACK_ALIGNMENT(): number;
				public getDEPTH_STENCIL_ATTACHMENT(): number;
				public getFRONT(): number;
				public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: org.nativescript.canvas.TNSCanvas): void;
				public bindAttribLocation(param0: number, param1: number, param2: string): void;
				public getDST_ALPHA(): number;
				public vertexAttrib4f(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getMAX_TEXTURE_SIZE(): number;
				public restoreStateAfterClear(): void;
				public getONE_MINUS_DST_COLOR(): number;
				public validateProgram(param0: number): void;
				public getUNSIGNED_SHORT_4_4_4_4(): number;
				public getBLEND_EQUATION(): number;
				public getMAX_FRAGMENT_UNIFORM_VECTORS(): number;
				public getFLOAT_MAT3(): number;
				public bufferSubData(param0: number, param1: number, param2: androidNative.Array<number>): void;
				public getBOOL_VEC3(): number;
				public getINT(): number;
				public uniform3iv(param0: number, param1: androidNative.Array<number>): void;
				public getDYNAMIC_DRAW(): number;
				public getIMPLEMENTATION_COLOR_READ_FORMAT(): number;
				public getRENDERBUFFER_DEPTH_SIZE(): number;
				public bufferDataInt(param0: number, param1: androidNative.Array<number>, param2: number): void;
				public getTEXTURE6(): number;
				public compressedTexSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: androidNative.Array<number>): void;
				public getSUBPIXEL_BITS(): number;
				public depthFunc(param0: number): void;
				public getSTENCIL_BACK_VALUE_MASK(): number;
				public getProgramInfoLog(param0: number): string;
				public getTEXTURE10(): number;
				public getONE_MINUS_CONSTANT_ALPHA(): number;
				public getTRIANGLE_STRIP(): number;
				public getALWAYS(): number;
				public getTEXTURE20(): number;
				public createShader(param0: number): number;
				public getTEXTURE30(): number;
				public getShaderPrecisionFormat(param0: number, param1: number): org.nativescript.canvas.WebGLShaderPrecisionFormat;
				public getGREATER(): number;
				public bufferDataFloat(param0: number, param1: androidNative.Array<number>, param2: number): void;
				public stencilFuncSeparate(param0: number, param1: number, param2: number, param3: number): void;
				public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.ShortBuffer): void;
				public getRGB(): number;
				public getUNPACK_COLORSPACE_CONVERSION_WEBGL(): number;
				public getTEXTURE_CUBE_MAP_NEGATIVE_X(): number;
				public getDEPTH_BUFFER_BIT(): number;
				public getSAMPLES(): number;
				public getDECR(): number;
				public setFlipYWebGL(param0: boolean): void;
				public renderbufferStorage(param0: number, param1: number, param2: number, param3: number): void;
				public texSubImage2DShortBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.ShortBuffer): void;
				public getBUFFER_USAGE(): number;
				public getFLOAT_VEC2(): number;
				public getUniform(param0: number, param1: number): any;
				public getMAX_VERTEX_TEXTURE_IMAGE_UNITS(): number;
				public drawElements(param0: number, param1: number, param2: number, param3: number): void;
				public getATTACHED_SHADERS(): number;
				public getTRIANGLE_FAN(): number;
				public getLINEAR(): number;
				public getVERTEX_SHADER(): number;
				public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.FloatBuffer): void;
				public createTexture(): number;
				public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.ByteBuffer): void;
				public getSTENCIL_BACK_WRITEMASK(): number;
				public getDEPTH_ATTACHMENT(): number;
				public readPixels(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.ShortBuffer): void;
				public deleteShader(param0: number): void;
				public blendEquationSeparate(param0: number, param1: number): void;
				public getBLEND(): number;
				public compressedTexSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: java.nio.FloatBuffer): void;
				public isRenderbuffer(param0: number): boolean;
				public pixelStorei(param0: number, param1: any): void;
				public getSAMPLE_COVERAGE_VALUE(): number;
				public bindBuffer(param0: number, param1: number): void;
				public getSAMPLE_COVERAGE_INVERT(): number;
				public getSTENCIL_PASS_DEPTH_PASS(): number;
				public stencilMask(param0: number): void;
				public framebufferTexture2D(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public uniform1iv(param0: number, param1: androidNative.Array<number>): void;
				public getVertexAttribOffset(param0: number, param1: number): number;
				public getARRAY_BUFFER(): number;
				public getTEXTURE17(): number;
				public getAttribLocation(param0: number, param1: string): number;
				public lineWidth(param0: number): void;
				public getUNPACK_ALIGNMENT(): number;
				public getBLEND_EQUATION_ALPHA(): number;
				public getTEXTURE27(): number;
				public getRENDERER(): number;
				public getSHORT(): number;
				public stencilFunc(param0: number, param1: number, param2: number): void;
				public getTEXTURE15(): number;
				public compressedTexImage2DFloat(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: androidNative.Array<number>): void;
				public getONE_MINUS_DST_ALPHA(): number;
				public readPixelsFloat(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: androidNative.Array<number>): void;
				public getTEXTURE0(): number;
				public compressedTexSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: java.nio.ShortBuffer): void;
				public uniformMatrix3fvBuffer(param0: number, param1: boolean, param2: java.nio.FloatBuffer): void;
				public readPixelsShortBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.ShortBuffer): void;
				public texImage2DByteBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.ByteBuffer): void;
				public getFRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL(): number;
				public getINVALID_FRAMEBUFFER_OPERATION(): number;
				public getTEXTURE25(): number;
				public getTEXTURE_CUBE_MAP_POSITIVE_Y(): number;
				public checkFramebufferStatus(param0: number): number;
				public readPixels(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.ByteBuffer): void;
				public getTEXTURE5(): number;
				public getFLOAT_MAT4(): number;
				public getRENDERBUFFER_ALPHA_SIZE(): number;
				public vertexAttribPointer(param0: number, param1: number, param2: number, param3: boolean, param4: number, param5: number): void;
				public uniformMatrix2fvBuffer(param0: number, param1: boolean, param2: java.nio.FloatBuffer): void;
				public getIMPLEMENTATION_COLOR_READ_TYPE(): number;
				public uniform1i(param0: number, param1: number): void;
				public getINVALID_ENUM(): number;
				public getFRAGMENT_SHADER(): number;
				public getExtension(param0: string): any;
				public getSAMPLER_2D(): number;
				public getZERO(): number;
				public getVALIDATE_STATUS(): number;
				public getCOLOR_ATTACHMENT0(): number;
				public vertexAttrib4fv(param0: number, param1: androidNative.Array<number>): void;
				public getTEXTURE_BINDING_2D(): number;
				public bufferSubDataByteBuffer(param0: number, param1: number, param2: java.nio.ByteBuffer): void;
				public scissor(param0: number, param1: number, param2: number, param3: number): void;
				public colorMask(param0: boolean, param1: boolean, param2: boolean, param3: boolean): void;
				public getTEXTURE_CUBE_MAP_NEGATIVE_Z(): number;
				public getBLEND_DST_ALPHA(): number;
				public getFRAMEBUFFER(): number;
				public getINCR(): number;
				public getINT_VEC3(): number;
				public readPixels(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.FloatBuffer): void;
				public enable(param0: number): void;
				public getTEXTURE12(): number;
				public getBACK(): number;
				public getINVALID_OPERATION(): number;
				public getINVERT(): number;
				public readPixels(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: androidNative.Array<number>): void;
				public getDEPTH_TEST(): number;
				public useProgram(param0: number): void;
				public getSTENCIL_BACK_FUNC(): number;
				public getSupportedExtensions(): androidNative.Array<string>;
				public clearColor(param0: number, param1: number, param2: number, param3: number): void;
				public getFLOAT_VEC4(): number;
				public generateMipmap(param0: number): void;
				public getALPHA(): number;
				public deleteFramebuffer(param0: number): void;
				public vertexAttrib3fv(param0: number, param1: androidNative.Array<number>): void;
				public uniform3i(param0: number, param1: number, param2: number, param3: number): void;
				public texSubImage2DByteBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.ByteBuffer): void;
				public getVERTEX_ATTRIB_ARRAY_SIZE(): number;
				public bindRenderbuffer(param0: number, param1: number): void;
				public copyTexImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number): void;
				public getNO_ERROR(): number;
				public stencilOpSeparate(param0: number, param1: number, param2: number, param3: number): void;
				public compressedTexImage2DShortBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.ShortBuffer): void;
				public reset(): void;
				public getTEXTURE9(): number;
				public uniform4i(param0: number, param1: number, param2: number, param3: number, param4: number): void;
				public getVENDOR(): number;
				public viewport(param0: number, param1: number, param2: number, param3: number): void;
				public getVERTEX_ATTRIB_ARRAY_POINTER(): number;
				public compressedTexImage2DIntBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.IntBuffer): void;
				public getLESS(): number;
				public getTEXTURE7(): number;
				public getShaderInfoLog(param0: number): string;
				public compressedTexSubImage2DInt(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: androidNative.Array<number>): void;
				public getVERTEX_ATTRIB_ARRAY_ENABLED(): number;
				public getVERTEX_ATTRIB_ARRAY_BUFFER_BINDING(): number;
				public getRENDERBUFFER_BINDING(): number;
				public getLUMINANCE(): number;
				public getSTREAM_DRAW(): number;
				public vertexAttrib2fv(param0: number, param1: androidNative.Array<number>): void;
				public getRENDERBUFFER_STENCIL_SIZE(): number;
				public isBuffer(param0: number): boolean;
				public getNEVER(): number;
				public getMAX_VERTEX_ATTRIBS(): number;
				public getFRONT_AND_BACK(): number;
				public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: org.nativescript.canvas.TNSImageBitmap): void;
				public getTEXTURE13(): number;
				public getSTENCIL_WRITEMASK(): number;
				public getFRAMEBUFFER_ATTACHMENT_OBJECT_NAME(): number;
				public uniform1ivBuffer(param0: number, param1: java.nio.IntBuffer): void;
				public getONE_MINUS_SRC_ALPHA(): number;
				public texImage2DFloatBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.FloatBuffer): void;
				public vertexAttrib4fvBuffer(param0: number, param1: java.nio.FloatBuffer): void;
				public getALIASED_POINT_SIZE_RANGE(): number;
				public getLOW_FLOAT(): number;
				public bufferData(param0: number, param1: number, param2: number): void;
				public uniform3f(param0: number, param1: number, param2: number, param3: number): void;
				public readPixelsIntBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.IntBuffer): void;
				public getGL_UNSIGNED_BYTE$canvas_release(): number;
				public isEnabled(param0: number): boolean;
				public getUNSIGNED_SHORT_5_5_5_1(): number;
				public getCONTEXT_LOST_WEBGL(): number;
				public getSTENCIL_BITS(): number;
				public getGL_LUMINANCE_ALPHA$canvas_release(): number;
				public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: androidNative.Array<number>): void;
				public vertexAttrib3f(param0: number, param1: number, param2: number, param3: number): void;
				public isFramebuffer(param0: number): boolean;
				public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: androidNative.Array<number>): void;
				public getSTENCIL_BUFFER_BIT(): number;
				public getActiveUniform(param0: number, param1: number): org.nativescript.canvas.WebGLActiveInfo;
				public vertexAttrib1f(param0: number, param1: number): void;
				public getCONSTANT_COLOR(): number;
				public getFUNC_ADD(): number;
				public vertexAttrib1fvBuffer(param0: number, param1: java.nio.FloatBuffer): void;
				public compileShader(param0: number): void;
				public commit(): void;
				public getTEXTURE29(): number;
				public readPixelsFloatBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: java.nio.FloatBuffer): void;
				public constructor(param0: org.nativescript.canvas.TNSCanvas);
				public getSTENCIL_BACK_REF(): number;
				public isProgram(param0: number): boolean;
				public getDEPTH_CLEAR_VALUE(): number;
				public clearDepth(param0: number): void;
				public hint(param0: number, param1: number): void;
				public isShader(param0: number): boolean;
				public bufferDataIntBuffer(param0: number, param1: java.nio.IntBuffer, param2: number): void;
				public getLINES(): number;
				public getMAX_CUBE_MAP_TEXTURE_SIZE(): number;
				public getSTENCIL_FUNC(): number;
				public getBROWSER_DEFAULT_WEBGL(): number;
				public getCOMPILE_STATUS(): number;
				public runOnGLThread$canvas_release(param0: java.lang.Runnable): void;
				public getMAX_VIEWPORT_DIMS(): number;
				public createFramebuffer(): number;
				public getPOLYGON_OFFSET_UNITS(): number;
				public getMAX_VERTEX_UNIFORM_VECTORS(): number;
				public getELEMENT_ARRAY_BUFFER(): number;
				public getRENDERBUFFER_GREEN_SIZE(): number;
				public framebufferRenderbuffer(param0: number, param1: number, param2: number, param3: number): void;
				public bufferSubData(param0: number, param1: number, param2: java.nio.ByteBuffer): void;
				public clearIfComposited(): org.nativescript.canvas.HowToClear;
				public bufferData(param0: number, param1: java.nio.IntBuffer, param2: number): void;
				public getParameter(param0: number): any;
				public bufferDataShort(param0: number, param1: androidNative.Array<number>, param2: number): void;
				public static nativeReadPixels(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
				public getRENDERBUFFER_BLUE_SIZE(): number;
				public getSTENCIL_ATTACHMENT(): number;
				public getSTENCIL_TEST(): number;
				public getSTENCIL_INDEX8(): number;
				public getKEEP(): number;
				public compressedTexSubImage2DShort(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: androidNative.Array<number>): void;
				public getFRAMEBUFFER_ATTACHMENT_OBJECT_TYPE(): number;
				public getRENDERBUFFER_HEIGHT(): number;
				public getNONE(): number;
				public getSAMPLER_CUBE(): number;
				public texSubImage2DFloatBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: java.nio.FloatBuffer): void;
				public getTEXTURE8(): number;
				public getCURRENT_VERTEX_ATTRIB(): number;
				public vertexAttrib2f(param0: number, param1: number, param2: number): void;
				public getLINK_STATUS(): number;
				public stencilOp(param0: number, param1: number, param2: number): void;
				public compressedTexSubImage2DByteBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: java.nio.ByteBuffer): void;
				public finish(): void;
				public uniform2f(param0: number, param1: number, param2: number): void;
				public getSAMPLE_ALPHA_TO_COVERAGE(): number;
				public getSRC_ALPHA_SATURATE(): number;
				public getUNPACK_FLIP_Y_WEBGL(): number;
				public getLUMINANCE_ALPHA(): number;
				public getGL_UNSIGNED_SHORT_4_4_4_4$canvas_release(): number;
				public createRenderbuffer(): number;
				public readPixelsInt(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: androidNative.Array<number>): void;
				public getDrawingBufferWidth(): number;
				public getTEXTURE_MAG_FILTER(): number;
				public getFASTEST(): number;
				public getHIGH_FLOAT(): number;
				public getFRAMEBUFFER_INCOMPLETE_DIMENSIONS(): number;
				public getDST_COLOR(): number;
				public getFRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT(): number;
				public compressedTexSubImage2DFloatBuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: java.nio.FloatBuffer): void;
				public getLINE_STRIP(): number;
				public getBLUE_BITS(): number;
				public getRENDERBUFFER_WIDTH(): number;
				public getACTIVE_UNIFORMS(): number;
				public copyTexSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number): void;
				public getRGBA(): number;
				public bufferSubData(param0: number, param1: number, param2: java.nio.FloatBuffer): void;
				public uniform3ivBuffer(param0: number, param1: java.nio.IntBuffer): void;
				public getRED_BITS(): number;
				public getTEXTURE22(): number;
				public getTEXTURE_CUBE_MAP_NEGATIVE_Y(): number;
				public getFLOAT_VEC3(): number;
				public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: globalAndroid.graphics.Bitmap): void;
				public isTexture(param0: number): boolean;
				public getNICEST(): number;
				public getTEXTURE31(): number;
				public bufferSubData(param0: number, param1: number, param2: java.nio.ShortBuffer): void;
				public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: globalAndroid.graphics.Bitmap): void;
				public getBYTE(): number;
				public getBLEND_SRC_RGB(): number;
				public getTEXTURE_CUBE_MAP(): number;
				public getREPEAT(): number;
				public getGL_UNSIGNED_SHORT_5_6_5$canvas_release(): number;
				public blendFunc(param0: number, param1: number): void;
				public getTEXTURE4(): number;
				public compressedTexImage2DByte(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: androidNative.Array<number>): void;
				public getALIASED_LINE_WIDTH_RANGE(): number;
				public getTEXTURE_WRAP_T(): number;
				public polygonOffset(param0: number, param1: number): void;
				public uniformMatrix4fvBuffer(param0: number, param1: boolean, param2: java.nio.FloatBuffer): void;
				public getHIGH_INT(): number;
				public getSAMPLE_BUFFERS(): number;
				public getActiveAttrib(param0: number, param1: number): org.nativescript.canvas.WebGLActiveInfo;
				public getSTENCIL_BACK_PASS_DEPTH_FAIL(): number;
				public getALPHA_BITS(): number;
				public bufferSubDataShort(param0: number, param1: number, param2: androidNative.Array<number>): void;
				public bufferSubDataIntBuffer(param0: number, param1: number, param2: java.nio.IntBuffer): void;
				public getCW(): number;
				public getFRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE(): number;
				public getFlipYWebGL(): boolean;
				public compressedTexSubImage2DFloat(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: androidNative.Array<number>): void;
				public uniform2i(param0: number, param1: number, param2: number): void;
				public getSHADING_LANGUAGE_VERSION(): number;
				public bufferData(param0: number, param1: any, param2: number): void;
				public bufferDataFloatBuffer(param0: number, param1: java.nio.FloatBuffer, param2: number): void;
				public getCULL_FACE(): number;
				public getSTENCIL_PASS_DEPTH_FAIL(): number;
				public getSTENCIL_REF(): number;
				public getSTENCIL_BACK_FAIL(): number;
				public getCOLOR_WRITEMASK(): number;
				public getGENERATE_MIPMAP_HINT(): number;
			}
			export module TNSWebGLRenderingContext {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TNSWebGLRenderingContext.Companion>;
					public nativeTexImage2DTexture(param0: number, param1: number, param2: number, param3: number): void;
					public nativeReadPixels(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
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
				public static Companion: org.nativescript.canvas.TextureRender.Companion;
				public setHeight(param0: number): void;
				public getHeight(): number;
				public getTextureId(): number;
				public setAb(param0: number): void;
				public surfaceCreated(): void;
				public drawFrame(param0: globalAndroid.graphics.SurfaceTexture, param1: number, param2: number, param3: number, param4: number, param5: boolean): void;
				public setWidth(param0: number): void;
				public getMatrix(): androidNative.Array<number>;
				public constructor();
				public getAb(): number;
				public setRbo(param0: number): void;
				public getWidth(): number;
				public getSamplerPos(): number;
				public getPos(): number;
				public getMatrixPos(): number;
				public setFbo(param0: number): void;
				public setMatrixPos(param0: number): void;
				public setSamplerPos(param0: number): void;
				public getRbo(): number;
				public getFbo(): number;
				public setTextureId(param0: number): void;
				public setPos(param0: number): void;
				public setMatrix(param0: androidNative.Array<number>): void;
			}
			export module TextureRender {
				export class Companion {
					public static class: java.lang.Class<org.nativescript.canvas.TextureRender.Companion>;
					public getVextexCoords(): androidNative.Array<number>;
					public getVextexBuf(): java.nio.FloatBuffer;
					public setVextexBuf(param0: java.nio.FloatBuffer): void;
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
				public static createSurfaceTexture(param0: org.nativescript.canvas.TNSWebGLRenderingContext): androidNative.Array<any>;
				public static attachToGLContext(param0: org.nativescript.canvas.TNSWebGLRenderingContext, param1: globalAndroid.graphics.SurfaceTexture, param2: org.nativescript.canvas.TextureRender): void;
				public isEmulator(): boolean;
				public getBytesFromBitmap(param0: globalAndroid.graphics.Bitmap): androidNative.Array<number>;
				public static detachFromGLContext(param0: org.nativescript.canvas.TNSWebGLRenderingContext, param1: globalAndroid.graphics.SurfaceTexture): void;
				public static createRenderAndAttachToGLContext(param0: org.nativescript.canvas.TNSWebGLRenderingContext, param1: globalAndroid.graphics.SurfaceTexture): org.nativescript.canvas.TextureRender;
				public static updateTexImage(param0: org.nativescript.canvas.TNSWebGLRenderingContext, param1: globalAndroid.graphics.SurfaceTexture, param2: org.nativescript.canvas.TextureRender, param3: number, param4: number, param5: number, param6: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class WebGLActiveInfo {
				public static class: java.lang.Class<org.nativescript.canvas.WebGLActiveInfo>;
				public setType(param0: number): void;
				public setName(param0: string): void;
				public getType(): number;
				public getSize(): number;
				public getName(): string;
				public constructor(param0: string, param1: number, param2: number);
				public setSize(param0: number): void;
				public constructor();
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export class WebGLShaderPrecisionFormat {
				public static class: java.lang.Class<org.nativescript.canvas.WebGLShaderPrecisionFormat>;
				public constructor(param0: number, param1: number, param2: number);
				public getRangeMax(): number;
				public getPrecision(): number;
				public getRangeMin(): number;
				public setRangeMax(param0: number): void;
				public setPrecision(param0: number): void;
				public setRangeMin(param0: number): void;
				public constructor();
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class ANGLE_instanced_arrays {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.ANGLE_instanced_arrays>;
					public vertexAttribDivisorANGLE(param0: number, param1: number): void;
					public getVERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE(): number;
					public drawElementsInstancedANGLE(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public constructor();
					public drawArraysInstancedANGLE(param0: number, param1: number, param2: number, param3: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class EXT_blend_minmax {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.EXT_blend_minmax>;
					public getMIN_EXT(): number;
					public constructor();
					public getMAX_EXT(): number;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class EXT_color_buffer_float {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.EXT_color_buffer_float>;
					public getR32F(): number;
					public setR11F_G11F_B10F(param0: number): void;
					public getRG16F(): number;
					public setRGB16F(param0: number): void;
					public setRG16F(param0: number): void;
					public setR32F(param0: number): void;
					public getRGB16F(): number;
					public getR16F(): number;
					public getR11F_G11F_B10F(): number;
					public getRG32F(): number;
					public constructor();
					public setR16F(param0: number): void;
					public getRGBA32F(): number;
					public setRGBA32F(param0: number): void;
					public setRG32F(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class EXT_color_buffer_half_float {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.EXT_color_buffer_half_float>;
					public getFRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT(): number;
					public getUNSIGNED_NORMALIZED_EXT(): number;
					public getRGBA16F_EXT(): number;
					public constructor();
					public setRGB16F_EXT(param0: number): void;
					public setFRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT(param0: number): void;
					public getRGB16F_EXT(): number;
					public setUNSIGNED_NORMALIZED_EXT(param0: number): void;
					public setRGBA16F_EXT(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class EXT_disjoint_timer_query {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.EXT_disjoint_timer_query>;
					public static Companion: org.nativescript.canvas.extensions.EXT_disjoint_timer_query.Companion;
					public getQueryObjectEXT(param0: number, param1: number): any;
					public beginQueryEXT(param0: number, param1: number): void;
					public getQueryEXT(param0: number, param1: number): number;
					public createQueryEXT(): number;
					public setCanvas(param0: org.nativescript.canvas.TNSCanvas): void;
					public endQueryEXT(param0: number): void;
					public constructor(param0: org.nativescript.canvas.TNSCanvas);
					public isQueryEXT(param0: number): boolean;
					public queryCounterEXT(param0: number, param1: number): void;
					public deleteQueryEXT(param0: number): void;
					public getCanvas(): org.nativescript.canvas.TNSCanvas;
				}
				export module EXT_disjoint_timer_query {
					export class Companion {
						public static class: java.lang.Class<org.nativescript.canvas.extensions.EXT_disjoint_timer_query.Companion>;
						public setCURRENT_QUERY_EXT(param0: number): void;
						public setTIME_ELAPSED_EXT(param0: number): void;
						public getCURRENT_QUERY_EXT(): number;
						public setQUERY_RESULT_EXT(param0: number): void;
						public getQUERY_COUNTER_BITS_EXT(): number;
						public setQUERY_RESULT_AVAILABLE_EXT(param0: number): void;
						public getQUERY_RESULT_EXT(): number;
						public getTIMESTAMP_EXT(): number;
						public getQUERY_RESULT_AVAILABLE_EXT(): number;
						public getGPU_DISJOINT_EXT(): number;
						public setQUERY_COUNTER_BITS_EXT(param0: number): void;
						public setGPU_DISJOINT_EXT(param0: number): void;
						public setTIMESTAMP_EXT(param0: number): void;
						public getTIME_ELAPSED_EXT(): number;
					}
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class EXT_sRGB {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.EXT_sRGB>;
					public getSRGB_EXT(): number;
					public getSRGB_ALPHA_EXT(): number;
					public setSRGB_ALPHA_EXT(param0: number): void;
					public constructor();
					public getSRGB8_ALPHA8_EXT(): number;
					public setSRGB8_ALPHA8_EXT(param0: number): void;
					public setFRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT(param0: number): void;
					public setSRGB_EXT(param0: number): void;
					public getFRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT(): number;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class EXT_shader_texture_lod {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.EXT_shader_texture_lod>;
					public constructor();
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class EXT_texture_filter_anisotropic {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.EXT_texture_filter_anisotropic>;
					public setMAX_TEXTURE_MAX_ANISOTROPY_EXT(param0: number): void;
					public getTEXTURE_MAX_ANISOTROPY_EXT(): number;
					public getMAX_TEXTURE_MAX_ANISOTROPY_EXT(): number;
					public constructor();
					public setTEXTURE_MAX_ANISOTROPY_EXT(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class OES_element_index_uint {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.OES_element_index_uint>;
					public constructor();
					public getUNSIGNED_INT(): number;
					public setUNSIGNED_INT(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class OES_fbo_render_mipmap {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.OES_fbo_render_mipmap>;
					public constructor();
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class OES_standard_derivatives {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.OES_standard_derivatives>;
					public getFRAGMENT_SHADER_DERIVATIVE_HINT_OES(): number;
					public constructor();
					public setFRAGMENT_SHADER_DERIVATIVE_HINT_OES(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class OES_texture_float {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.OES_texture_float>;
					public constructor();
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class OES_texture_float_linear {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.OES_texture_float_linear>;
					public constructor();
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class OES_texture_half_float {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.OES_texture_half_float>;
					public constructor();
					public getHALF_FLOAT_OES(): number;
					public setHALF_FLOAT_OES(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class OES_texture_half_float_linear {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.OES_texture_half_float_linear>;
					public constructor();
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class OES_vertex_array_object {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.OES_vertex_array_object>;
					public deleteVertexArrayOES(param0: number): void;
					public bindVertexArrayOES(param0: number): void;
					public setCanvas(param0: org.nativescript.canvas.TNSCanvas): void;
					public createVertexArrayOES(): number;
					public constructor(param0: org.nativescript.canvas.TNSCanvas);
					public getVERTEX_ARRAY_BINDING_OES(): number;
					public isVertexArrayOES(param0: number): boolean;
					public setVERTEX_ARRAY_BINDING_OES(param0: number): void;
					public getCanvas(): org.nativescript.canvas.TNSCanvas;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class WEBGL_color_buffer_float {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.WEBGL_color_buffer_float>;
					public getFRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT(): number;
					public getUNSIGNED_NORMALIZED_EXT(): number;
					public getRGB32F_EXT(): number;
					public constructor();
					public setRGBA32F_EXT(param0: number): void;
					public setFRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT(param0: number): void;
					public setUNSIGNED_NORMALIZED_EXT(param0: number): void;
					public setRGB32F_EXT(param0: number): void;
					public getRGBA32F_EXT(): number;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class WEBGL_compressed_texture_atc {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.WEBGL_compressed_texture_atc>;
					public setCOMPRESSED_RGB_ATC_WEBGL(param0: number): void;
					public getCOMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL(): number;
					public constructor();
					public setCOMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL(param0: number): void;
					public getCOMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL(): number;
					public setCOMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL(param0: number): void;
					public getCOMPRESSED_RGB_ATC_WEBGL(): number;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class WEBGL_compressed_texture_etc {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.WEBGL_compressed_texture_etc>;
					public getCOMPRESSED_SIGNED_RG11_EAC(): number;
					public setCOMPRESSED_SRGB8_ETC2(param0: number): void;
					public getCOMPRESSED_SRGB8_ETC2(): number;
					public setCOMPRESSED_RG11_EAC(param0: number): void;
					public setCOMPRESSED_RGB8_ETC2(param0: number): void;
					public setCOMPRESSED_SIGNED_RG11_EAC(param0: number): void;
					public getCOMPRESSED_RGBA8_ETC2_EAC(): number;
					public setCOMPRESSED_RGBA8_ETC2_EAC(param0: number): void;
					public setCOMPRESSED_R11_EAC(param0: number): void;
					public setCOMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2(param0: number): void;
					public getCOMPRESSED_SIGNED_R11_EAC(): number;
					public getCOMPRESSED_RG11_EAC(): number;
					public getCOMPRESSED_RGB8_ETC2(): number;
					public getCOMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2(): number;
					public setCOMPRESSED_SIGNED_R11_EAC(param0: number): void;
					public constructor();
					public getCOMPRESSED_SRGB8_ALPHA8_ETC2_EAC(): number;
					public getCOMPRESSED_R11_EAC(): number;
					public getCOMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2(): number;
					public setCOMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2(param0: number): void;
					public setCOMPRESSED_SRGB8_ALPHA8_ETC2_EAC(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class WEBGL_compressed_texture_etc1 {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.WEBGL_compressed_texture_etc1>;
					public getCOMPRESSED_RGB_ETC1_WEBGL(): number;
					public constructor();
					public setCOMPRESSED_RGB_ETC1_WEBGL(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class WEBGL_compressed_texture_pvrtc {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.WEBGL_compressed_texture_pvrtc>;
					public setCOMPRESSED_RGBA_PVRTC_2BPPV1_IMG(param0: number): void;
					public getCOMPRESSED_RGBA_PVRTC_2BPPV1_IMG(): number;
					public getCOMPRESSED_RGB_PVRTC_4BPPV1_IMG(): number;
					public getCOMPRESSED_RGB_PVRTC_2BPPV1_IMG(): number;
					public constructor();
					public getCOMPRESSED_RGBA_PVRTC_4BPPV1_IMG(): number;
					public setCOMPRESSED_RGB_PVRTC_2BPPV1_IMG(param0: number): void;
					public setCOMPRESSED_RGBA_PVRTC_4BPPV1_IMG(param0: number): void;
					public setCOMPRESSED_RGB_PVRTC_4BPPV1_IMG(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class WEBGL_compressed_texture_s3tc {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.WEBGL_compressed_texture_s3tc>;
					public getCOMPRESSED_RGBA_S3TC_DXT3_EXT(): number;
					public setCOMPRESSED_RGB_S3TC_DXT1_EXT(param0: number): void;
					public constructor();
					public getCOMPRESSED_RGB_S3TC_DXT1_EXT(): number;
					public getCOMPRESSED_RGBA_S3TC_DXT1_EXT(): number;
					public setCOMPRESSED_RGBA_S3TC_DXT1_EXT(param0: number): void;
					public setCOMPRESSED_RGBA_S3TC_DXT3_EXT(param0: number): void;
					public setCOMPRESSED_RGBA_S3TC_DXT5_EXT(param0: number): void;
					public getCOMPRESSED_RGBA_S3TC_DXT5_EXT(): number;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class WEBGL_compressed_texture_s3tc_srgb {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.WEBGL_compressed_texture_s3tc_srgb>;
					public setCOMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT(param0: number): void;
					public getCOMPRESSED_SRGB_S3TC_DXT1_EXT(): number;
					public setCOMPRESSED_SRGB_S3TC_DXT1_EXT(param0: number): void;
					public getCOMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT(): number;
					public constructor();
					public getCOMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT(): number;
					public getCOMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT(): number;
					public setCOMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT(param0: number): void;
					public setCOMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT(param0: number): void;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class WEBGL_depth_texture {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.WEBGL_depth_texture>;
					public constructor();
					public setUNSIGNED_INT_24_8_WEBGL(param0: number): void;
					public getUNSIGNED_INT_24_8_WEBGL(): number;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class WEBGL_draw_buffers {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.WEBGL_draw_buffers>;
					public setDRAW_BUFFER10_WEBGL(param0: number): void;
					public getDRAW_BUFFER8_WEBGL(): number;
					public setCOLOR_ATTACHMENT10_WEBGL(param0: number): void;
					public setCOLOR_ATTACHMENT2_WEBGL(param0: number): void;
					public getCOLOR_ATTACHMENT3_WEBGL(): number;
					public setDRAW_BUFFER7_WEBGL(param0: number): void;
					public setDRAW_BUFFER11_WEBGL(param0: number): void;
					public setCOLOR_ATTACHMENT1_WEBGL(param0: number): void;
					public getDRAW_BUFFER7_WEBGL(): number;
					public setDRAW_BUFFER8_WEBGL(param0: number): void;
					public constructor();
					public setDRAW_BUFFER9_WEBGL(param0: number): void;
					public setCOLOR_ATTACHMENT11_WEBGL(param0: number): void;
					public getCOLOR_ATTACHMENT2_WEBGL(): number;
					public setDRAW_BUFFER4_WEBGL(param0: number): void;
					public getCOLOR_ATTACHMENT5_WEBGL(): number;
					public setCOLOR_ATTACHMENT12_WEBGL(param0: number): void;
					public getDRAW_BUFFER0_WEBGL(): number;
					public setDRAW_BUFFER6_WEBGL(param0: number): void;
					public getCOLOR_ATTACHMENT14_WEBGL(): number;
					public setDRAW_BUFFER1_WEBGL(param0: number): void;
					public getDRAW_BUFFER11_WEBGL(): number;
					public setDRAW_BUFFER13_WEBGL(param0: number): void;
					public setCOLOR_ATTACHMENT4_WEBGL(param0: number): void;
					public getDRAW_BUFFER5_WEBGL(): number;
					public getCOLOR_ATTACHMENT0_WEBGL(): number;
					public setCOLOR_ATTACHMENT9_WEBGL(param0: number): void;
					public setCOLOR_ATTACHMENT7_WEBGL(param0: number): void;
					public getDRAW_BUFFER3_WEBGL(): number;
					public getDRAW_BUFFER12_WEBGL(): number;
					public setCOLOR_ATTACHMENT14_WEBGL(param0: number): void;
					public getCOLOR_ATTACHMENT8_WEBGL(): number;
					public setMAX_COLOR_ATTACHMENTS_WEBGL(param0: number): void;
					public getCOLOR_ATTACHMENT9_WEBGL(): number;
					public getMAX_COLOR_ATTACHMENTS_WEBGL(): number;
					public getCOLOR_ATTACHMENT13_WEBGL(): number;
					public setDRAW_BUFFER2_WEBGL(param0: number): void;
					public setDRAW_BUFFER14_WEBGL(param0: number): void;
					public getDRAW_BUFFER2_WEBGL(): number;
					public setMAX_DRAW_BUFFERS_WEBGL(param0: number): void;
					public setCOLOR_ATTACHMENT15_WEBGL(param0: number): void;
					public setCOLOR_ATTACHMENT6_WEBGL(param0: number): void;
					public getCOLOR_ATTACHMENT7_WEBGL(): number;
					public getDRAW_BUFFER4_WEBGL(): number;
					public getDRAW_BUFFER13_WEBGL(): number;
					public getCOLOR_ATTACHMENT11_WEBGL(): number;
					public getCOLOR_ATTACHMENT12_WEBGL(): number;
					public getDRAW_BUFFER14_WEBGL(): number;
					public setDRAW_BUFFER15_WEBGL(param0: number): void;
					public setDRAW_BUFFER3_WEBGL(param0: number): void;
					public setCOLOR_ATTACHMENT0_WEBGL(param0: number): void;
					public getCOLOR_ATTACHMENT1_WEBGL(): number;
					public getCOLOR_ATTACHMENT15_WEBGL(): number;
					public setDRAW_BUFFER0_WEBGL(param0: number): void;
					public getCOLOR_ATTACHMENT6_WEBGL(): number;
					public setDRAW_BUFFER5_WEBGL(param0: number): void;
					public getDRAW_BUFFER10_WEBGL(): number;
					public getCOLOR_ATTACHMENT10_WEBGL(): number;
					public getDRAW_BUFFER6_WEBGL(): number;
					public getDRAW_BUFFER15_WEBGL(): number;
					public setCOLOR_ATTACHMENT5_WEBGL(param0: number): void;
					public setCOLOR_ATTACHMENT8_WEBGL(param0: number): void;
					public getMAX_DRAW_BUFFERS_WEBGL(): number;
					public drawBuffersWEBGL(param0: androidNative.Array<number>): void;
					public setCOLOR_ATTACHMENT13_WEBGL(param0: number): void;
					public getDRAW_BUFFER9_WEBGL(): number;
					public setCOLOR_ATTACHMENT3_WEBGL(param0: number): void;
					public getCOLOR_ATTACHMENT4_WEBGL(): number;
					public setDRAW_BUFFER12_WEBGL(param0: number): void;
					public getDRAW_BUFFER1_WEBGL(): number;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module canvas {
			export module extensions {
				export class WEBGL_lose_context {
					public static class: java.lang.Class<org.nativescript.canvas.extensions.WEBGL_lose_context>;
					public getCanvasView(): org.nativescript.canvas.TNSCanvas;
					public restoreContext(): void;
					public loseContext(): void;
					public setCanvasView(param0: org.nativescript.canvas.TNSCanvas): void;
					public constructor(param0: org.nativescript.canvas.TNSCanvas);
				}
			}
		}
	}
}

//Generics information:

