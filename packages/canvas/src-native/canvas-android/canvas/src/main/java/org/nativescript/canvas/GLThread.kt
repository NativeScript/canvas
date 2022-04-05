package org.nativescript.canvas

import android.annotation.SuppressLint
import android.graphics.Color
import android.opengl.*
import android.os.Build
import android.util.Log
import java.util.concurrent.CountDownLatch

internal class GLThread(private val glContext: GLContext) : Thread() {
	var lock: CountDownLatch? = null

	@JvmField
	var isStarted = false

	internal var isPaused = false

	@Synchronized
	fun setPaused(paused: Boolean) {
		isPaused = paused
	}

	@Synchronized
	fun getPaused(): Boolean {
		return isPaused
	}

	@Synchronized
	override fun start() {
		super.start()
		isStarted = true
	}

	override fun interrupt() {
		super.interrupt()
		isStarted = false
	}

	var mSurface: Any? = null

	enum class State {
		None, Initializing, Complete, Resizing
	}

	var state = State.None

	@Synchronized
	fun unbind(pause: Boolean) {
		if (pause) {
			setPaused(pause)
		}
		Log.d("com.test", "Thread ${currentThread()}")
		EGL14.eglMakeCurrent(
			glContext.mEGLDisplay,
			EGL14.EGL_NO_SURFACE,
			EGL14.EGL_NO_SURFACE,
			EGL14.EGL_NO_CONTEXT
		);
	}

//	@SuppressLint("InlinedApi")
//	private fun initEGL() {
//		if (glContext.type == TNSCanvas.ContextType.NONE) {
//			return
//		}
//		if (state == State.Initializing || state == State.Complete) {
//			return
//		}
//
//		if (state != State.Resizing) {
//			state = State.Initializing
//		}
//
//		val view = glContext.reference?.get()
//		glContext.mEGLDisplay = EGL14.eglGetDisplay(EGL14.EGL_DEFAULT_DISPLAY)
//		if (glContext.mEGLDisplay == EGL14.EGL_NO_DISPLAY) {
//			throw RuntimeException(
//				"eglGetDisplay failed " + GLUtils.getEGLErrorString(
//					EGL14.eglGetError()
//				)
//			)
//		}
//		val version = IntArray(2)
//
//		if (!EGL14.eglInitialize(glContext.mEGLDisplay, version, 0, version, 1)) {
//			throw RuntimeException(
//				"eglInitialize failed " + GLUtils.getEGLErrorString(
//					EGL14.eglGetError()
//				)
//			)
//		}
//
//		// Find a compatible EGLConfig
//		val configsCount = IntArray(1)
//		val configs = arrayOfNulls<EGLConfig>(1)
//		var type = EGL14.EGL_OPENGL_ES2_BIT
//		var depthSize = 16
//		var stencilSize = 0
//		var alpha = 8
//		var useAlpha = true
//		var enableBufferPreservation = glContext.preserveDrawingBuffer
//		if (view != null) {
//			if (view.glVersion > 2 && view.actualContextType == "webgl2" && GLContext.IS_WEBGL_2_SUPPORT) {
//				type = EGLExt.EGL_OPENGL_ES3_BIT_KHR
//			}
//			if (view.contextStencil && view.contextType != TNSCanvas.ContextType.CANVAS) {
//				stencilSize = 8
//				glContext.stencil = true
//			}
//			if (!view.contextDepth || view.contextType == TNSCanvas.ContextType.CANVAS) {
//				glContext.depth = false
//				depthSize = 0
//			}
//			enableBufferPreservation = view.contextPreserveDrawingBuffer
//			useAlpha = view.contextAlpha
//			glContext.antialias = if (view.contextType == TNSCanvas.ContextType.CANVAS) {
//				false
//			} else {
//				view.contextAntialias
//			}
//		}
//		if (!useAlpha) {
//			alpha = 0
//		}
//		var configSpec = intArrayOf(
//			EGL14.EGL_RENDERABLE_TYPE, type,
//			EGL14.EGL_RED_SIZE, 8,
//			EGL14.EGL_GREEN_SIZE, 8,
//			EGL14.EGL_BLUE_SIZE, 8,
//			EGL14.EGL_ALPHA_SIZE, alpha,
//			EGL14.EGL_DEPTH_SIZE, depthSize,
//			EGL14.EGL_STENCIL_SIZE, stencilSize
//		)
//		if (glContext.antialias) {
//			configSpec = configSpec.copyOf(configSpec.size + 5)
//			val last = configSpec.size - 1
//			configSpec[last - 4] = EGL14.EGL_SAMPLE_BUFFERS
//			configSpec[last - 3] = 1
//			configSpec[last - 2] = EGL14.EGL_SAMPLES
//			configSpec[last - 1] = 4
//			configSpec[last] = EGL14.EGL_NONE
//		} else {
//			configSpec = configSpec.copyOf(configSpec.size + 1)
//			val last = configSpec.size - 1
//			configSpec[last] = EGL14.EGL_NONE
//		}
//
//		if (!EGL14.eglChooseConfig(
//				glContext.mEGLDisplay,
//				configSpec,
//				0,
//				configs,
//				0,
//				1,
//				configsCount,
//				0
//			)
//		) {
//			if (glContext.antialias) {
//				if (view != null) {
//					view.contextAntialias = false
//				}
//				configSpec = intArrayOf(
//					EGL14.EGL_RENDERABLE_TYPE, type,
//					EGL14.EGL_RED_SIZE, 8,
//					EGL14.EGL_GREEN_SIZE, 8,
//					EGL14.EGL_BLUE_SIZE, 8,
//					EGL14.EGL_ALPHA_SIZE, alpha,
//					EGL14.EGL_DEPTH_SIZE, depthSize,
//					EGL14.EGL_STENCIL_SIZE, stencilSize,
//					EGL14.EGL_NONE
//				)
//				val config =
//					EGL14.eglChooseConfig(
//						glContext.mEGLDisplay,
//						configSpec,
//						0,
//						configs,
//						0,
//						1,
//						configsCount,
//						0
//					)
//				if (config) {
//					glContext.mEGLConfig = configs[0]
//				}
//			} else {
//				throw IllegalArgumentException(
//					"eglChooseConfig failed " + GLUtils.getEGLErrorString(
//						EGL14.eglGetError()
//					)
//				)
//			}
//		} else if (configsCount[0] > 0) {
//			glContext.mEGLConfig = configs[0]
//		}
//		if (glContext.mEGLConfig == null) {
//			throw RuntimeException("eglConfig not initialized")
//		}
//		if (view != null) {
//			if (view.glVersion > 2 && view.actualContextType == "webgl2" && GLContext.IS_WEBGL_2_SUPPORT) {
//				if (Build.VERSION.SDK_INT >= 21) {
//					if (glContext.mEGLContext == null || glContext.mEGLContext === EGL14.EGL_NO_CONTEXT) {
//						glContext.mEGLContext =
//							createGLContext(3, 1, glContext.mEGLConfig!!)
//					}
//				}
//
//				if (glContext.mEGLContext == null || glContext.mEGLContext === EGL14.EGL_NO_CONTEXT) {
//					glContext.mEGLContext =
//						createGLContext(3, 0, glContext.mEGLConfig!!)
//				}
//
//			}
//		}
//		if (glContext.mEGLContext == null || glContext.mEGLContext === EGL14.EGL_NO_CONTEXT) {
//			glContext.mEGLContext =
//				createGLContext(2, 0, glContext.mEGLConfig!!)
//		}
//		glContext.mEGLSurface =
//			glContext.createSurface(glContext.mEGLConfig, mSurface)
//
//		if (glContext.mEGLSurface == null || glContext.mEGLSurface === EGL14.EGL_NO_SURFACE) {
//			val error = EGL14.eglGetError()
//			throw RuntimeException(
//				"eglCreateWindowSurface failed " + GLUtils.getEGLErrorString(
//					error
//				)
//			)
//		}
//
//		// Switch to our EGLContext
//		makeEGLContextCurrent()
////			EGL14.eglSwapInterval(EGL14.eglGetCurrentDisplay(), 0)
//		if (enableBufferPreservation) {
//			// Enable buffer preservation -- allows app to draw over previous frames without clearing
//			val didEnableBufferPreservation = EGL14.eglSurfaceAttrib(
//				EGL14.eglGetCurrentDisplay(), EGL14.eglGetCurrentSurface(EGL14.EGL_DRAW),
//				EGL14.EGL_SWAP_BEHAVIOR, EGL14.EGL_BUFFER_PRESERVED
//			)
//			if (!didEnableBufferPreservation) {
//				if (view != null) {
//					view.contextPreserveDrawingBuffer = false
//				}
//			}
//		}
//		GLES20.glClearColor(0f, 0f, 0f, 0f)
//		var bit = GLES20.GL_COLOR_BUFFER_BIT
//		if (glContext.depth) {
//			GLES20.glClearDepthf(1f)
//			bit = bit or GLES20.GL_DEPTH_BUFFER_BIT
//		}
//		if (glContext.stencil) {
//			GLES20.glClearStencil(0)
//			bit = bit or GLES20.GL_STENCIL_BUFFER_BIT
//		}
//		GLES20.glClear(bit)
//		if (!glContext.mGLThread.getPaused() && !glContext.swapBuffers(
//				glContext.mEGLSurface
//			)
//		) {
//			if (TNSCanvas.enableDebug) {
//				Log.e("JS", "GLContext: Cannot swap buffers!")
//			}
//		}
//
//		if (state != State.Resizing) {
//			state = State.Complete
//		}
//	}
//
//	private fun deInitEGL() {
//		makeEGLContextCurrent()
//		glContext.destroySurface(glContext.mEGLSurface)
//		EGL14.eglDestroyContext(
//			glContext.mEGLDisplay,
//			glContext.mEGLContext
//		)
//		EGL14.eglTerminate(glContext.mEGLDisplay)
//	}
//
//	private fun reInitEGL() {
//		if (state != State.Resizing) {
//			return
//		}
////		GLES20.glClearColor(0f, 0f, 0f, 0f)
////		GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT)
//
//		if (nativeContextPtr != 0L) {
//			glContext.reference?.get()?.let { canvasView ->
//				GLES20.glViewport(0, 0, canvasView.width, canvasView.height)
//				val frameBuffers = IntArray(1)
//				GLES20.glGetIntegerv(GLES20.GL_FRAMEBUFFER_BINDING, frameBuffers, 0)
//				var samples = 0
//				if (canvasView.contextAntialias) {
//					samples = 4
//				}
//				val metrics = canvasView.resources.displayMetrics
//
//				val ctx = canvasView.getContext("2d") as TNSCanvasRenderingContext2D
//
//				Log.d("com.test", "b4 value ${ctx
//					.font}")
//
//				TNSCanvas.nativeResizeSurface(
//					canvasView.nativeContext,
//					canvasView.drawingBufferWidth.toFloat(),
//					canvasView.drawingBufferHeight.toFloat(),
//					canvasView.scale,
//					frameBuffers[0],
//					samples,
//					true,
//					metrics.density,
//					true
//				)
//				GLES20.glFinish()
//				glContext.swapBuffers(glContext.mEGLSurface)
//
//				Log.d("com.test", "after value ${ctx
//					.font}")
//			}
//		} else {
//			if (!getPaused() && !glContext.swapBuffers(glContext.mEGLSurface)) {
//				if (TNSCanvas.enableDebug) {
//					Log.e("JS", "GLContext: Cannot swap buffers!")
//				}
//			}
//		}
//		state = State.Complete
//
//	}

	private fun createGLContext(
		contextVersion: Int,
		minorVersion: Int,
		eglConfig: EGLConfig
	): EGLContext {
		val attribs = intArrayOf(
			GLContext.EGL_CONTEXT_CLIENT_VERSION,
			contextVersion,
			GLContext.EGL_CONTEXT_CLIENT_MINOR_VERSION,
			minorVersion,
			EGL14.EGL_NONE
		)
		return EGL14.eglCreateContext(
			glContext.mEGLDisplay,
			eglConfig,
			EGL14.EGL_NO_CONTEXT,
			attribs,
			0
		)
	}

	fun makeEGLContextCurrent() {
		if (glContext.mEGLContext != EGL14.eglGetCurrentContext() ||
			glContext.mEGLSurface != EGL14.eglGetCurrentSurface(EGL14.EGL_DRAW)
		) {
			if (!glContext.makeCurrent(glContext.mEGLSurface)) {
				if (TNSCanvas.enableDebug) {
					Log.e(
						GLContext.TAG, "eglMakeCurrent failed " + GLUtils.getEGLErrorString(
							EGL14.eglGetError()
						)
					)
				}
			}
		}
	}

	private fun removeEGLContextCurrent() {
		if (glContext.mEGLContext == EGL14.eglGetCurrentContext() ||
			glContext.mEGLSurface == EGL14.eglGetCurrentSurface(EGL14.EGL_DRAW)
		) {
			if (!
				EGL14.eglMakeCurrent(
					glContext.mEGLDisplay,
					EGL14.EGL_NO_SURFACE, EGL14.EGL_NO_SURFACE,
					EGL14.EGL_NO_CONTEXT
				)
			) {
				if (TNSCanvas.enableDebug) {
					Log.e(
						GLContext.TAG, "eglMakeCurrent failed " + GLUtils.getEGLErrorString(
							EGL14.eglGetError()
						)
					)
				}
			}
		}
	}

	private var nativeContextPtr = 0L

	override fun run() {
//		while (true) {
//			try {
//				if (!isPaused) {
//					Log.d("com.test", "state?? $state")
//					Log.d("com.test", "count ${glContext.mQueue.count()}")
//					initEGL()
//					if (glContext.mEGLSurface != null) {
//						makeEGLContextCurrent()
//						reInitEGL()
//
//						if (nativeContextPtr == 0L) {
//							glContext.glView?.get()?.let { view ->
//								glContext.reference?.get()?.let { ref ->
//									if (ref.nativeContext == 0L && ref.contextType == TNSCanvas.ContextType.CANVAS) {
//										glContext.type = ref.contextType
//										val width = view.width
//										val height = view.height
//										val frameBuffers = IntArray(1)
//										if (view.drawingBufferWidth == 0) {
//											view.drawingBufferWidth = width
//										}
//										if (view.drawingBufferHeight == 0) {
//											view.drawingBufferHeight = height
//										}
//										if (view.drawingBufferWidth == 0) {
//											view.drawingBufferWidth = 1
//										}
//										if (view.drawingBufferHeight == 0) {
//											view.drawingBufferHeight = 1
//										}
//
//										GLES20.glViewport(
//											0,
//											0,
//											view.drawingBufferWidth,
//											view.drawingBufferHeight
//										)
//										GLES20.glGetIntegerv(
//											GLES20.GL_FRAMEBUFFER_BINDING,
//											frameBuffers,
//											0
//										)
//										var samples = 0
//										if (ref.contextAntialias) {
//											samples = 4
//										}
//										val metrics = view.resources.displayMetrics
//										ref.nativeContext = TNSCanvas.nativeInitContext(
//											view.drawingBufferWidth.toFloat(),
//											view.drawingBufferHeight.toFloat(),
//											ref.scale,
//											frameBuffers[0],
//											samples,
//											true,
//											Color.BLACK,
//											metrics.density * 160,
//											TNSCanvas.direction.toNative()
//										)
//										nativeContextPtr = ref.nativeContext
//										glContext.swapBuffers(glContext.mEGLSurface)
//									}
//								}
//							}
//						}
//
//						lock?.let {
//							lock?.countDown()
//							lock = null
//						}
//						glContext.mQueue.take().run()
//						removeEGLContextCurrent()
//					}
//				}
//			} catch (e: InterruptedException) {
//				break
//			}
//		}
//		deInitEGL()
	}

	init {
		priority = MAX_PRIORITY
	}
}
