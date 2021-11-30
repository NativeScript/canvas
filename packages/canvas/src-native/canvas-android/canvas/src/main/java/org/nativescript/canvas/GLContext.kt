package org.nativescript.canvas

import android.annotation.SuppressLint
import android.graphics.Color
import android.graphics.SurfaceTexture
import android.opengl.EGL14
import android.opengl.EGLExt
import android.opengl.GLES20
import android.opengl.GLUtils
import android.util.Log
import java.lang.ref.WeakReference
import java.util.concurrent.BlockingQueue
import java.util.concurrent.LinkedBlockingQueue
import javax.microedition.khronos.egl.*


/**
 * Created by triniwiz on 6/9/20
 */
internal class GLContext {
	@JvmField
	var glView: WeakReference<GLView>? = null
	private val mQueue: BlockingQueue<Runnable> = LinkedBlockingQueue()

	@JvmField
	var mGLThread: GLThread? = null
	private var mEGLDisplay: EGLDisplay? = null
	internal var mEGLSurface: EGLSurface? = null
	private var offscreenTexture: SurfaceTexture? = null
	private var textureId = 0
	private var mEGLContext: EGLContext? = null
	private var mEGLConfig: EGLConfig? = null
	private var mEGL: EGL10? = null
	var alpha = true
	var antialias = true
	var depth = true
	var failIfMajorPerformanceCaveat = false
	var powerPreference = "default"
	var premultipliedAlpha = true
	var preserveDrawingBuffer = false
	var stencil = false
	var desynchronized = false
	var xrCompatible = false

	@JvmField
	var reference: WeakReference<TNSCanvas>? = null
	val isHeadless: Boolean
		get() = if (mGLThread != null) {
			mGLThread!!.mSurface == null
		} else true

	fun queueEvent(runnable: Runnable) {
		mQueue.add(runnable)
	}

	fun init(texture: Any?) {
		if (mGLThread != null) {
			return
		}
		mGLThread = GLThread(texture)
		mGLThread!!.priority = Thread.MAX_PRIORITY
	}

	fun startGLThread() {
		mGLThread!!.start()
	}

	fun resize(width: Int, height: Int) {
		if (reference != null) {
			val canvasView = reference!!.get()
			if (canvasView != null) {
				queueEvent {
					if (offscreenTexture != null) {
						GLES20.glClearColor(0f, 0f, 0f, 0f)
						GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT)
						if (!mGLThread!!.getPaused() && !swapBuffers(mEGLSurface)) {
							Log.e("JS", "GLContext: Cannot swap buffers!")
						}
						offscreenTexture!!.release()
						offscreenTexture = null
						textureId += 1
						offscreenTexture = SurfaceTexture(textureId)
						offscreenTexture!!.setDefaultBufferSize(width, height)
						mGLThread!!.mSurface = offscreenTexture
						mEGLSurface = createSurface(mEGLConfig, offscreenTexture)
						if (mEGLSurface == null || mEGLSurface === EGL10.EGL_NO_SURFACE) {
							val error = mEGL!!.eglGetError()
							throw RuntimeException(
								"eglCreateWindowSurface failed " + GLUtils.getEGLErrorString(
									error
								)
							)
						}
						if (canvasView.nativeContext != 0L) {
							val frameBuffers = IntArray(1)
							GLES20.glGetIntegerv(GLES20.GL_FRAMEBUFFER_BINDING, frameBuffers, 0)
							var samples = 0
							if (canvasView.contextAntialias) {
								samples = 4
							}
							val metrics = canvasView.resources.displayMetrics
							TNSCanvas.nativeResizeSurface(
								canvasView.nativeContext,
								canvasView.drawingBufferWidth.toFloat(),
								canvasView.drawingBufferHeight.toFloat(),
								canvasView.scale,
								frameBuffers[0],
								samples,
								true,
								metrics.density
							)
							GLES20.glViewport(0, 0, width, height)
							swapBuffers(mEGLSurface)
						}
					} else {
						GLES20.glClearColor(0f, 0f, 0f, 0f)
						GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT)
						if (!mGLThread!!.getPaused() && !swapBuffers(mEGLSurface)) {
							Log.e("JS", "GLContext: Cannot swap buffers!")
						}
						if (canvasView.nativeContext != 0L) {
							GLES20.glViewport(0, 0, width, height)
							val frameBuffers = IntArray(1)
							GLES20.glGetIntegerv(GLES20.GL_FRAMEBUFFER_BINDING, frameBuffers, 0)
							var samples = 0
							if (canvasView.contextAntialias) {
								samples = 4
							}
							val metrics = canvasView.resources.displayMetrics
							TNSCanvas.nativeResizeSurface(
								canvasView.nativeContext,
								canvasView.drawingBufferWidth.toFloat(),
								canvasView.drawingBufferHeight.toFloat(),
								canvasView.scale,
								frameBuffers[0],
								samples,
								true,
								metrics.density
							)
							swapBuffers(mEGLSurface)
						}
					}
				}
			}
		}
	}

	fun flush() {
		queueEvent {
			if (reference != null) {
				val canvasView = reference!!.get()
				if (canvasView != null && canvasView.nativeContext != 0L && canvasView.invalidateState == TNSCanvas.InvalidateState.INVALIDATING) {
					TNSCanvas.nativeFlush(canvasView.nativeContext)
					if (!mGLThread!!.getPaused() && !swapBuffers(mEGLSurface)) {
						Log.e("JS", "GLContext: Cannot swap buffers!")
					}
					canvasView.invalidateState = TNSCanvas.InvalidateState.NONE
				} else {
					// WebGL
					if (!mGLThread!!.getPaused() && !swapBuffers(mEGLSurface)) {
						Log.e("JS", "GLContext: Cannot swap buffers!")
					}
					if (canvasView != null) {
						canvasView.invalidateState = TNSCanvas.InvalidateState.NONE
					}
				}
			}
		}
	}

	fun createEglSurface(surface: Any): EGLSurface {
		val surfaceAttribs = intArrayOf(
			EGL14.EGL_NONE
		)
		return mEGL!!.eglCreateWindowSurface(mEGLDisplay, mEGLConfig, surface, surfaceAttribs)
	}

	fun makeEglSurfaceCurrent(surface: EGLSurface) {
		mEGL?.eglMakeCurrent(mEGLDisplay, surface, surface, mEGLContext)
	}

	fun createSurface(config: EGLConfig?, surface: Any?): EGLSurface {
		if (surface == null) {
			var width = 1
			var height = 1
			if (reference != null) {
				val view = reference!!.get()
				if (view != null) {
					width = view.width
					height = view.height
				}
				if (width == 0) {
					width = 1
				}
				if (height == 0) {
					height = 1
				}
			}
//			val surfaceAttribs = intArrayOf(
//				EGL10.EGL_WIDTH, width,
//				EGL10.EGL_HEIGHT, height,
//				EGL10.EGL_NONE
//			)
			textureId += 1
			offscreenTexture = SurfaceTexture(textureId)
			// return mEGL.eglCreatePbufferSurface(mEGLDisplay, config, surfaceAttribs);
			offscreenTexture!!.setDefaultBufferSize(width, height)
			return mEGL!!.eglCreateWindowSurface(mEGLDisplay, config, offscreenTexture, null)
		}
		return mEGL!!.eglCreateWindowSurface(mEGLDisplay, config, surface, null)
	}

	fun onPause() {
		queueEvent(Runnable {
			mEGL?.eglMakeCurrent(
				mEGLDisplay,
				EGL10.EGL_NO_SURFACE,
				EGL10.EGL_NO_SURFACE,
				EGL10.EGL_NO_CONTEXT
			)
			mGLThread?.setPaused(true)
		})
	}

	fun onResume() {
		if (mGLThread?.isPaused == true) {
			queueEvent {
				mGLThread?.setPaused(false)
			}
		}
	}

	fun makeCurrent(surface: EGLSurface?): Boolean {
		return mEGL!!.eglMakeCurrent(mEGLDisplay, surface, surface, mEGLContext)
	}

	fun destroySurface(surface: EGLSurface?): Boolean {
		return mEGL!!.eglDestroySurface(mEGLDisplay, surface)
	}

	fun swapBuffers(surface: EGLSurface?): Boolean {
		return mEGL!!.eglSwapBuffers(mEGLDisplay, surface)
	}

	val isGLThreadStarted: Boolean
		get() = if (mGLThread == null) {
			false
		} else mGLThread!!.isStarted

	fun destroy() {
		if (mGLThread != null) {
			try {
				mGLThread!!.interrupt()
				mGLThread!!.join()
			} catch (e: InterruptedException) {
				Log.e("JS", "GLContext: Can't interrupt GL thread.", e)
			}
			mGLThread = null
		}
	}

	inner class GLThread : Thread {
		@JvmField
		var isStarted = false


		internal var isPaused = false

		var type = TNSCanvas.ContextType.NONE

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

		var mSurface: Any?

		constructor() {
			mSurface = null
		}

		constructor(texture: Any?) {
			mSurface = texture
		}


		private fun hasExtension(extensions: String, name: String): Boolean {
			var start = extensions.indexOf(name)
			while (start >= 0) {
				// check that we didn't find a prefix of a longer extension name
				val end = start + name.length
				if (end == extensions.length || extensions[end] == ' ') {
					return true
				}
				start = extensions.indexOf(name, end)
			}
			return false
		}

		@SuppressLint("InlinedApi")
		private fun initEGL() {
			val view = reference!!.get()
			mEGL = EGLContext.getEGL() as EGL10
			mEGLDisplay = mEGL!!.eglGetDisplay(EGL10.EGL_DEFAULT_DISPLAY)
			if (mEGLDisplay === EGL10.EGL_NO_DISPLAY) {
				throw RuntimeException(
					"eglGetDisplay failed " + GLUtils.getEGLErrorString(
						mEGL!!.eglGetError()
					)
				)
			}
			val version = IntArray(2)
			if (!mEGL!!.eglInitialize(mEGLDisplay, version)) {
				throw RuntimeException(
					"eglInitialize failed " + GLUtils.getEGLErrorString(
						mEGL!!.eglGetError()
					)
				)
			}

			// Find a compatible EGLConfig
			val configsCount = IntArray(1)
			val configs = arrayOfNulls<EGLConfig>(1)
			var type = EGL14.EGL_OPENGL_ES2_BIT
			var depthSize = 16
			var stencilSize = 0
			var alpha = 8
			var useAlpha = true
			var enableBufferPreservation = preserveDrawingBuffer
			var highestEsVersion = 0
			if (view != null) {
				val checkES3 = hasExtension(
					mEGL!!.eglQueryString(mEGLDisplay, EGL10.EGL_EXTENSIONS),
					"EGL_KHR_create_context"
				)
				val configCount = IntArray(1)
				mEGL!!.eglGetConfigs(mEGLDisplay, null, 0, configCount)

				val tmpConfig = arrayOfNulls<EGLConfig>(configCount[0])
				mEGL!!.eglGetConfigs(mEGLDisplay, tmpConfig, configCount[0], configCount)

				val value = IntArray(1)


				for (i in 0 until configCount[0]) {
					if (mEGL!!.eglGetConfigAttrib(
							mEGLDisplay, tmpConfig[i],
							EGL10.EGL_RENDERABLE_TYPE, value
						)
					) {
						if (checkES3 && value[0] and EGL_OPENGL_ES3_BIT_KHR ==
							EGL_OPENGL_ES3_BIT_KHR
						) {
							if (highestEsVersion < 3) highestEsVersion = 3
						} else if (value[0] and EGL_OPENGL_ES2_BIT == EGL_OPENGL_ES2_BIT) {
							if (highestEsVersion < 2) highestEsVersion = 2
						} else if (value[0] and EGL_OPENGL_ES_BIT == EGL_OPENGL_ES_BIT) {
							if (highestEsVersion < 1) highestEsVersion = 1
						}
					} else {
					}
				}

				if (highestEsVersion >= 3) {
					IS_WEBGL_2_SUPPORT = true
				}

				DID_CHECK_WEBGL_SUPPORT = true


				if (view.glVersion > 2 && view.actualContextType == "webgl2" && highestEsVersion >= 3) {
					type = EGLExt.EGL_OPENGL_ES3_BIT_KHR
				}
				if (view.contextStencil && view.contextType != TNSCanvas.ContextType.CANVAS) {
					stencilSize = 8
					stencil = true
				}
				if (!view.contextDepth || view.contextType == TNSCanvas.ContextType.CANVAS) {
					depth = false
					depthSize = 0
				}
				enableBufferPreservation = view.contextPreserveDrawingBuffer
				useAlpha = view.contextAlpha
				antialias = if (view.contextType == TNSCanvas.ContextType.CANVAS) {
					false
				} else {
					view.contextAntialias
				}
			}
			if (!useAlpha) {
				alpha = 0
			}
			var configSpec = intArrayOf(
				EGL10.EGL_RENDERABLE_TYPE, type,
				EGL10.EGL_RED_SIZE, 8,
				EGL10.EGL_GREEN_SIZE, 8,
				EGL10.EGL_BLUE_SIZE, 8,
				EGL10.EGL_ALPHA_SIZE, alpha,
				EGL10.EGL_DEPTH_SIZE, depthSize,
				EGL10.EGL_STENCIL_SIZE, stencilSize
			)
			if (antialias) {
				configSpec = configSpec.copyOf(configSpec.size + 5)
				val last = configSpec.size - 1
				configSpec[last - 4] = EGL10.EGL_SAMPLE_BUFFERS
				configSpec[last - 3] = 1
				configSpec[last - 2] = EGL10.EGL_SAMPLES
				configSpec[last - 1] = 4
				configSpec[last] = EGL10.EGL_NONE
			} else {
				configSpec = configSpec.copyOf(configSpec.size + 1)
				val last = configSpec.size - 1
				configSpec[last] = EGL10.EGL_NONE
			}
			if (!mEGL!!.eglChooseConfig(mEGLDisplay, configSpec, configs, 1, configsCount)) {
				if (antialias) {
					if (view != null) {
						view.contextAntialias = false
					}
					configSpec = intArrayOf(
						EGL10.EGL_RENDERABLE_TYPE, type,
						EGL10.EGL_RED_SIZE, 8,
						EGL10.EGL_GREEN_SIZE, 8,
						EGL10.EGL_BLUE_SIZE, 8,
						EGL10.EGL_ALPHA_SIZE, alpha,
						EGL10.EGL_DEPTH_SIZE, depthSize,
						EGL10.EGL_STENCIL_SIZE, stencilSize,
						EGL10.EGL_NONE
					)
					val config = mEGL!!.eglChooseConfig(mEGLDisplay, configSpec, configs, 1, configsCount)
					if (config) {
						mEGLConfig = configs[0]
					}
				} else {
					throw IllegalArgumentException(
						"eglChooseConfig failed " + GLUtils.getEGLErrorString(
							mEGL!!.eglGetError()
						)
					)
				}
			} else if (configsCount[0] > 0) {
				mEGLConfig = configs[0]
			}
			if (mEGLConfig == null) {
				throw RuntimeException("eglConfig not initialized")
			}
			if (view != null) {
				if (view.glVersion > 2 && view.actualContextType == "webgl2" && highestEsVersion >= 3) {
					if (mEGLContext == null || mEGLContext === EGL10.EGL_NO_CONTEXT) {
						mEGLContext = createGLContext(3, 0, mEGLConfig!!)
					}
				}
			}
			if (mEGLContext == null || mEGLContext === EGL10.EGL_NO_CONTEXT) {
				mEGLContext = createGLContext(2, 0, mEGLConfig!!)
			}
			mEGLSurface = createSurface(mEGLConfig, mSurface)
			if (mEGLSurface == null || mEGLSurface === EGL10.EGL_NO_SURFACE) {
				val error = mEGL!!.eglGetError()
				throw RuntimeException("eglCreateWindowSurface failed " + GLUtils.getEGLErrorString(error))
			}

			// Switch to our EGLContext
			makeEGLContextCurrent()
//			EGL14.eglSwapInterval(EGL14.eglGetCurrentDisplay(), 0)
			if (enableBufferPreservation) {
				// Enable buffer preservation -- allows app to draw over previous frames without clearing
				val didEnableBufferPreservation = EGL14.eglSurfaceAttrib(
					EGL14.eglGetCurrentDisplay(), EGL14.eglGetCurrentSurface(EGL14.EGL_DRAW),
					EGL14.EGL_SWAP_BEHAVIOR, EGL14.EGL_BUFFER_PRESERVED
				)
				if (!didEnableBufferPreservation) {
					if (view != null) {
						view.contextPreserveDrawingBuffer = false
					}
				}
			}
			GLES20.glClearColor(0f, 0f, 0f, 0f)
			var bit = GLES20.GL_COLOR_BUFFER_BIT
			if (depth) {
				GLES20.glClearDepthf(1f)
				bit = bit or GLES20.GL_DEPTH_BUFFER_BIT
			}
			if (stencil) {
				GLES20.glClearStencil(0)
				bit = bit or GLES20.GL_STENCIL_BUFFER_BIT
			}
			GLES20.glClear(bit)
			if (!mGLThread!!.getPaused() && !swapBuffers(mEGLSurface)) {
				Log.e("JS", "GLContext: Cannot swap buffers!")
			}

		}

		private fun deInitEGL() {
			makeEGLContextCurrent()
			destroySurface(mEGLSurface)
			mEGL!!.eglDestroyContext(mEGLDisplay, mEGLContext)
			mEGL!!.eglTerminate(mEGLDisplay)
		}

		private fun createGLContext(
			contextVersion: Int,
			minorVersion: Int,
			eglConfig: EGLConfig
		): EGLContext {
			val attribs = intArrayOf(
				EGL_CONTEXT_CLIENT_VERSION,
				contextVersion,
				EGL_CONTEXT_CLIENT_MINOR_VERSION,
				minorVersion,
				EGL10.EGL_NONE
			)
			return mEGL!!.eglCreateContext(mEGLDisplay, eglConfig, EGL10.EGL_NO_CONTEXT, attribs)
		}

		private fun makeEGLContextCurrent() {
			if (mEGLContext != mEGL!!.eglGetCurrentContext() ||
				mEGLSurface != mEGL!!.eglGetCurrentSurface(EGL10.EGL_DRAW)
			) {
				if (!makeCurrent(mEGLSurface)) {
					//  throw new RuntimeException("eglMakeCurrent failed " + GLUtils.getEGLErrorString(mEGL.eglGetError()));
					if (BuildConfig.DEBUG) {
						Log.d(
							TAG, "eglMakeCurrent failed " + GLUtils.getEGLErrorString(
								mEGL!!.eglGetError()
							)
						)
					}
				}
			}
		}

		override fun run() {
			initEGL()
			val view = glView!!.get()
			if (view?.startupLock != null) {
				val ref = reference!!.get()
				if (ref != null) {
					type = ref.contextType
					makeEGLContextCurrent()
					if (type == TNSCanvas.ContextType.CANVAS && ref.nativeContext == 0L) {
						val width = view.width
						val height = view.height
						val frameBuffers = IntArray(1)
						if (view.drawingBufferWidth == 0) {
							view.drawingBufferWidth = width
						}
						if (view.drawingBufferHeight == 0) {
							view.drawingBufferHeight = height
						}
						if (view.drawingBufferWidth == 0) {
							view.drawingBufferWidth = 1
						}
						if (view.drawingBufferHeight == 0) {
							view.drawingBufferHeight = 1
						}

						GLES20.glViewport(0, 0, view.drawingBufferWidth, view.drawingBufferHeight)
						GLES20.glGetIntegerv(GLES20.GL_FRAMEBUFFER_BINDING, frameBuffers, 0)
						var samples = 0
						if (ref.contextAntialias) {
							samples = 4
						}
						val metrics = view.resources.displayMetrics
						ref.nativeContext = TNSCanvas.nativeInitContext(
							view.drawingBufferWidth.toFloat(),
							view.drawingBufferHeight.toFloat(),
							ref.scale,
							frameBuffers[0],
							samples,
							true,
							Color.BLACK,
							metrics.density * 160,
							TNSCanvas.direction.toNative()
						)

						swapBuffers(mEGLSurface)
					}
				}
				view.startupLock?.countDown()
			}
			while (true) {
				try {
					if (!isPaused) {
						makeEGLContextCurrent()
						mQueue.take().run()
					}
				} catch (e: InterruptedException) {
					break
				}
			}
			deInitEGL()
		}
	}

	companion object {
		var DID_CHECK_WEBGL_SUPPORT = false
		var IS_WEBGL_2_SUPPORT = false
		const val TAG = "GLContext"
		private const val EGL_CONTEXT_CLIENT_VERSION = 0x3098
		private const val EGL_CONTEXT_CLIENT_MINOR_VERSION = 0x30FB
		private const val EGL_OPENGL_ES_BIT = 0x0001
		private const val EGL_OPENGL_ES2_BIT = 0x0004
		private const val EGL_OPENGL_ES3_BIT_KHR = 0x0040
	}
}
