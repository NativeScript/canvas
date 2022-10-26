package org.nativescript.canvas

import android.opengl.*
import android.util.Log
import java.lang.Exception
import java.lang.ref.WeakReference
import java.util.concurrent.BlockingQueue
import java.util.concurrent.CountDownLatch
import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit


/**
 * Created by triniwiz on 6/9/20
 */
internal class GLContext {
	@JvmField
	var glView: WeakReference<GLView>? = null
	val mQueue: BlockingQueue<Runnable> = LinkedBlockingQueue()


	@JvmField
	var mGLThread = GLThread(this)
	var mEGLDisplay: EGLDisplay? = null
	var mEGLSurface: EGLSurface? = null
	var mEGLContext: EGLContext? = null
	var mEGLConfig: EGLConfig? = null

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
	var type = TNSCanvas.ContextType.NONE


	var surfaceWidth: Int = 0
	var surfaceHeight: Int = 0

	@JvmField
	var reference: WeakReference<TNSCanvas>? = null

	fun queueEvent(runnable: Runnable) {
		mQueue.add(runnable)
	}

	fun setTexture(texture: Any?) {
		mGLThread.mSurface = texture
		mGLThread.state = GLThread.State.None
	}

	fun startGLThread() {
		mGLThread.start()
	}

	fun resize(width: Int, height: Int, oldWidth: Int, oldHeight: Int) {
		if (surfaceWidth == 0 && surfaceHeight == 0) {
			return
		}

		if (mGLThread.state == GLThread.State.Complete && (surfaceWidth != width && surfaceHeight != height)) {
			mGLThread.state = GLThread.State.Resizing
			mGLThread.lock = CountDownLatch(1)
			mGLThread.lock?.let { lock ->
				try {
					lock.await(2, TimeUnit.SECONDS)
				} catch (e: Exception) {
				}
			}
		}
	}

	fun flush() {
		flush(false)
	}

	fun flush(wait: Boolean) {
		reference?.get()?.let { canvasView ->
			val lock = if (wait) {
				CountDownLatch(1)
			} else {
				null
			}

//			queueEvent {
//				if (canvasView.nativeContext != 0L && canvasView.invalidateState == TNSCanvas.InvalidateState.INVALIDATING) {
//					TNSCanvas.nativeFlush(canvasView.nativeContext)
//					if (!mGLThread.getPaused() && !swapBuffers(mEGLSurface)) {
//						if (TNSCanvas.enableDebug) {
//							Log.e("JS", "GLContext: Cannot swap buffers!")
//						}
//					}
//					canvasView.invalidateState = TNSCanvas.InvalidateState.NONE
//				} else {
//					// WebGL
//					if (!mGLThread.getPaused() && !swapBuffers(mEGLSurface)) {
//						if (TNSCanvas.enableDebug) {
//							Log.e("JS", "GLContext: Cannot swap buffers!")
//						}
//					}
//					canvasView.invalidateState = TNSCanvas.InvalidateState.NONE
//				}
//				lock?.countDown()
//			}

			lock?.let {
				try {
					it.await(2, TimeUnit.SECONDS)
				} catch (ignored: InterruptedException) {
				}
			}
		}
	}

	fun createEglSurface(surface: Any): EGLSurface {
		val surfaceAttribs = intArrayOf(
			EGL14.EGL_NONE
		)
		return EGL14.eglCreateWindowSurface(mEGLDisplay!!, mEGLConfig!!, surface, surfaceAttribs, 0)
	}

	fun makeEglSurfaceCurrent(surface: EGLSurface) {
		EGL14.eglMakeCurrent(mEGLDisplay, surface, surface, mEGLContext)
	}

	fun createSurface(config: EGLConfig?, surface: Any?): EGLSurface {
		if (surface == null) {
			var width = 1
			var height = 1

			reference?.get()?.let { view ->
				width = view.width
				height = view.height

				if (width == 0) {
					width = 1
				}
				if (height == 0) {
					height = 1
				}
			}

			surfaceWidth = width
			surfaceHeight = height

			val surfaceAttribs = intArrayOf(
				EGL14.EGL_WIDTH, width,
				EGL14.EGL_HEIGHT, height,
				EGL14.EGL_NONE
			)

			return EGL14.eglCreatePbufferSurface(mEGLDisplay, config, surfaceAttribs, 0)
		}

		val surfaceAttribs = intArrayOf(
			EGL14.EGL_NONE
		)

		surfaceWidth = glView?.get()?.width ?: 0
		surfaceHeight = glView?.get()?.height ?: 0

		return EGL14.eglCreateWindowSurface(mEGLDisplay, config, surface, surfaceAttribs, 0)
	}

	fun onPause() {
		queueEvent {
			EGL14.eglMakeCurrent(
				mEGLDisplay,
				EGL14.EGL_NO_SURFACE,
				EGL14.EGL_NO_SURFACE,
				EGL14.EGL_NO_CONTEXT
			)
			mGLThread.setPaused(true)
		}
	}

	fun onResume() {
		if (mGLThread.isPaused) {
			queueEvent {
				mGLThread.setPaused(false)
			}
		}
	}

	fun makeCurrent(surface: EGLSurface?): Boolean {
		return EGL14.eglMakeCurrent(mEGLDisplay, surface, surface, mEGLContext)
	}

	fun destroySurface(surface: EGLSurface?): Boolean {
		return EGL14.eglDestroySurface(mEGLDisplay, surface)
	}

	fun swapBuffers(surface: EGLSurface?): Boolean {
		return EGL14.eglSwapBuffers(mEGLDisplay, surface)
	}

	fun destroy() {
		try {
			mGLThread.setPaused(true)
			//mGLThread.interrupt()
			//mGLThread.join()
		} catch (e: InterruptedException) {
			if (TNSCanvas.enableDebug) {
				Log.e("JS", "GLContext: Can't interrupt GL thread.", e)
			}
		} finally {
			//	mGLThread = GLThread(this)
		}
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

	fun checkGLSupport() {
		var highestEsVersion = 0

		mEGLDisplay = EGL14.eglGetDisplay(EGL14.EGL_DEFAULT_DISPLAY)

		val checkES3 = hasExtension(
			EGL14.eglQueryString(mEGLDisplay, EGL14.EGL_EXTENSIONS),
			"EGL_KHR_create_context"
		)
		val configCount = IntArray(1)
		EGL14.eglGetConfigs(mEGLDisplay, null, 0, 0, configCount, 0)

		val tmpConfig = arrayOfNulls<EGLConfig>(configCount[0])
		EGL14.eglGetConfigs(mEGLDisplay, tmpConfig, 0, configCount[0], configCount, 0)

		val value = IntArray(1)


		for (i in 0 until configCount[0]) {
			if (EGL14.eglGetConfigAttrib(
					mEGLDisplay, tmpConfig[i],
					EGL14.EGL_RENDERABLE_TYPE, value, 0
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
	}

	companion object {
		var DID_CHECK_WEBGL_SUPPORT = false
		var IS_WEBGL_2_SUPPORT = false
		const val TAG = "GLContext"
		const val EGL_CONTEXT_CLIENT_VERSION = 0x3098
		const val EGL_CONTEXT_CLIENT_MINOR_VERSION = 0x30FB
		private const val EGL_OPENGL_ES_BIT = 0x0001
		private const val EGL_OPENGL_ES2_BIT = 0x0004
		private const val EGL_OPENGL_ES3_BIT_KHR = 0x0040
	}
}
