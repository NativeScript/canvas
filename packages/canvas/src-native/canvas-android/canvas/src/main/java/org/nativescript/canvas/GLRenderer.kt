package org.nativescript.canvas

import android.annotation.SuppressLint
import android.app.ActivityManager
import android.content.Context
import android.graphics.Bitmap
import android.graphics.Color
import android.opengl.*
import android.os.Build
import android.os.Handler
import android.os.Looper
import android.util.Base64
import android.util.Log
import android.view.Choreographer
import android.view.ViewGroup
import android.widget.FrameLayout
import org.json.JSONObject
import java.io.ByteArrayOutputStream
import java.lang.ref.WeakReference
import java.nio.ByteBuffer
import java.nio.ByteOrder

internal class GLRenderer(
	val canvas: TNSCanvas,
	val context: Context,
	val softwareRenderer: Boolean
) : Choreographer.FrameCallback {

	internal var nativeContext: Long = 0L
	internal var surface: GLView? = null
	internal var cpuView: CPUView? = null


	internal var renderingContext2d: TNSCanvasRenderingContext? = null
	internal var scale = 1f

	var ignorePixelScaling: Boolean = false
		set(value) {
			field = value
			surface?.ignorePixelScaling = value
		}

	var isHandleInvalidationManually = false

	var mEGLDisplay: EGLDisplay? = null
	var mEGLSurface: EGLSurface? = null
	var mEGLContext: EGLContext? = null
	var mEGLConfig: EGLConfig? = null

	@JvmField
	var glView: WeakReference<GLView>? = null


	var mSurface: Any? = null

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

	var surfaceWidth: Int = 0
	var surfaceHeight: Int = 0
	internal var isWebGL = false

	var drawingBufferWidth: Int = 0
		get() {
			if (softwareRenderer) {
				return cpuView!!.width
			}
			return field
		}
		set(value) {
			if (!softwareRenderer) {
				field = value
			}
		}

	var drawingBufferHeight: Int = 0
		get() {
			if (softwareRenderer) {
				return cpuView!!.height
			}
			return field
		}
		set(value) {
			if (!softwareRenderer) {
				field = value
			}
		}

	internal var webGLRenderingContext: TNSWebGLRenderingContext? = null
	internal var webGL2RenderingContext: TNSWebGL2RenderingContext? = null

	var listener: TNSCanvas.Listener? = null
		set(value) {
			field = value
			surface?.setListener(listener)
		}


	@JvmField
	internal var invalidateState = InvalidateState.NONE
	internal var contextType = TNSCanvas.ContextType.NONE
		set(value) {
			field = value
			if (renderingContext2d == null && webGLRenderingContext == null && webGL2RenderingContext == null) {
				initEGL()
			}
		}


	internal var actualContextType = ""

	@JvmField
	internal var contextAlpha = true

	@JvmField
	internal var contextAntialias = false

	@JvmField
	internal var contextDepth = true

	@JvmField
	internal var contextFailIfMajorPerformanceCaveat = false

	@JvmField
	internal var contextPowerPreference: String? = "default"

	@JvmField
	internal var contextPremultipliedAlpha = true

	@JvmField
	internal var contextPreserveDrawingBuffer = false

	@JvmField
	internal var contextStencil = false

	@JvmField
	internal var contextDesynchronized = false

	@JvmField
	internal var contextXrCompatible = false

	@JvmField
	internal var mClearStencil = 0

	@JvmField
	internal var mClearColor = floatArrayOf(0f, 0f, 0f, 0f)

	@JvmField
	internal var mScissorEnabled = false

	@JvmField
	internal var mClearDepth = 1f

	@JvmField
	internal var mColorMask = booleanArrayOf(true, true, true, true)

	@JvmField
	internal var mStencilMask = (0xFFFFFFFF).toInt()

	@JvmField
	internal var mStencilMaskBack = (0xFFFFFFFF).toInt()

	@JvmField
	internal var mStencilFuncRef = 0

	@JvmField
	internal var mStencilFuncRefBack = 0

	@JvmField
	internal var mStencilFuncMask = (0xFFFFFFFF).toInt()

	@JvmField
	internal var mStencilFuncMaskBack = (0xFFFFFFFF).toInt()

	@JvmField
	internal var mDepthMask = true

	@JvmField
	internal var glVersion = 0

	internal var mainHandler = Handler(Looper.myLooper()!!)

	enum class InvalidateState {
		NONE, PENDING, INVALIDATING
	}

	init {
		glVersion = if (detectOpenGLES30() && !Utils.isEmulator) {
			3
		} else {
			2
		}
		if (softwareRenderer) {
			cpuView = CPUView(context)
			cpuView!!.glRenderer = this
			canvas.addView(
				cpuView, FrameLayout.LayoutParams(
					ViewGroup.LayoutParams.MATCH_PARENT,
					ViewGroup.LayoutParams.MATCH_PARENT
				)
			)
		} else {
			surface = GLView(context)
			surface?.renderer = WeakReference(this)
			canvas.addView(
				surface, FrameLayout.LayoutParams(
					ViewGroup.LayoutParams.MATCH_PARENT,
					ViewGroup.LayoutParams.MATCH_PARENT
				)
			)
		}
	}


	private fun detectOpenGLES30(): Boolean {
		val am = context.getSystemService(Context.ACTIVITY_SERVICE) as? ActivityManager
		return am?.let {
			it.deviceConfigurationInfo.reqGlEsVersion >= 0x30000
		} ?: false
	}

	override fun doFrame(frameTimeNanos: Long) {
		if (!isHandleInvalidationManually) {
			if (invalidateState == InvalidateState.PENDING) {
				invalidateState = InvalidateState.INVALIDATING
				flush()
			}
		}
		Choreographer.getInstance().postFrameCallback(this)
	}

	fun resize(width: Int, height: Int) {
		if (softwareRenderer) {
			cpuView?.resize(width, height)
		} else {
			if (surface?.isCreated == false && (drawingBufferWidth != width && drawingBufferHeight != height)) {
				resizeSurface()
			}
			surface?.resize(width, height)
		}
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
			EGL14.EGL_NONE
		)
		return EGL14.eglCreateContext(
			mEGLDisplay,
			eglConfig,
			EGL14.EGL_NO_CONTEXT,
			attribs,
			0
		)
	}

	internal fun makeEGLContextCurrent() {
		if (mEGLContext != EGL14.eglGetCurrentContext() ||
			mEGLSurface != EGL14.eglGetCurrentSurface(EGL14.EGL_DRAW)
		) {
			if (!makeCurrent(mEGLSurface)) {
				if (TNSCanvas.enableDebug) {
					Log.d(
						TAG, "eglMakeCurrent failed " + GLUtils.getEGLErrorString(
							EGL14.eglGetError()
						)
					)
				}
			}
		}
	}


	@SuppressLint("InlinedApi")
	private fun initEGL() {
		if (contextType == TNSCanvas.ContextType.NONE) {
			return
		}

		if (mEGLDisplay == null) {
			mEGLDisplay = EGL14.eglGetDisplay(EGL14.EGL_DEFAULT_DISPLAY)
		}

		if (mEGLDisplay == EGL14.EGL_NO_DISPLAY) {
			throw RuntimeException(
				"eglGetDisplay failed " + GLUtils.getEGLErrorString(
					EGL14.eglGetError()
				)
			)
		}
		val version = IntArray(2)

		if (!EGL14.eglInitialize(mEGLDisplay, version, 0, version, 1)) {
			throw RuntimeException(
				"eglInitialize failed " + GLUtils.getEGLErrorString(
					EGL14.eglGetError()
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
		if (glVersion > 2 && actualContextType == "webgl2" && GLContext.IS_WEBGL_2_SUPPORT) {
			type = EGLExt.EGL_OPENGL_ES3_BIT_KHR
		}
		if (contextStencil && contextType != TNSCanvas.ContextType.CANVAS) {
			stencilSize = 8
			stencil = true
		}
		if (!contextDepth || contextType == TNSCanvas.ContextType.CANVAS) {
			depth = false
			depthSize = 0
		}
		enableBufferPreservation = contextPreserveDrawingBuffer
		useAlpha = contextAlpha
		antialias = if (contextType == TNSCanvas.ContextType.CANVAS) {
			false
		} else {
			contextAntialias
		}
		if (!useAlpha) {
			alpha = 0
		}
		var configSpec = intArrayOf(
			EGL14.EGL_RENDERABLE_TYPE, type,
			EGL14.EGL_RED_SIZE, 8,
			EGL14.EGL_GREEN_SIZE, 8,
			EGL14.EGL_BLUE_SIZE, 8,
			EGL14.EGL_ALPHA_SIZE, alpha,
			EGL14.EGL_DEPTH_SIZE, depthSize,
			EGL14.EGL_STENCIL_SIZE, stencilSize
		)
		if (antialias) {
			configSpec = configSpec.copyOf(configSpec.size + 5)
			val last = configSpec.size - 1
			configSpec[last - 4] = EGL14.EGL_SAMPLE_BUFFERS
			configSpec[last - 3] = 1
			configSpec[last - 2] = EGL14.EGL_SAMPLES
			configSpec[last - 1] = 4
			configSpec[last] = EGL14.EGL_NONE
		} else {
			configSpec = configSpec.copyOf(configSpec.size + 1)
			val last = configSpec.size - 1
			configSpec[last] = EGL14.EGL_NONE
		}

		if (!EGL14.eglChooseConfig(
				mEGLDisplay,
				configSpec,
				0,
				configs,
				0,
				1,
				configsCount,
				0
			)
		) {
			if (antialias) {
				contextAntialias = false
				configSpec = intArrayOf(
					EGL14.EGL_RENDERABLE_TYPE, type,
					EGL14.EGL_RED_SIZE, 8,
					EGL14.EGL_GREEN_SIZE, 8,
					EGL14.EGL_BLUE_SIZE, 8,
					EGL14.EGL_ALPHA_SIZE, alpha,
					EGL14.EGL_DEPTH_SIZE, depthSize,
					EGL14.EGL_STENCIL_SIZE, stencilSize,
					EGL14.EGL_NONE
				)
				val config =
					EGL14.eglChooseConfig(
						mEGLDisplay,
						configSpec,
						0,
						configs,
						0,
						1,
						configsCount,
						0
					)
				if (config) {
					mEGLConfig = configs[0]
				}
			} else {
				throw IllegalArgumentException(
					"eglChooseConfig failed " + GLUtils.getEGLErrorString(
						EGL14.eglGetError()
					)
				)
			}
		} else if (configsCount[0] > 0) {
			mEGLConfig = configs[0]
		}
		if (mEGLConfig == null) {
			throw RuntimeException("eglConfig not initialized")
		}
		if (glVersion > 2 && actualContextType == "webgl2" && GLContext.IS_WEBGL_2_SUPPORT) {
			if (Build.VERSION.SDK_INT >= 21) {
				if (mEGLContext == null || mEGLContext === EGL14.EGL_NO_CONTEXT) {
					mEGLContext =
						createGLContext(3, 1, mEGLConfig!!)
				}
			}

			if (mEGLContext == null || mEGLContext === EGL14.EGL_NO_CONTEXT) {
				mEGLContext =
					createGLContext(3, 0, mEGLConfig!!)
			}

		}
		if (mEGLContext == null || mEGLContext === EGL14.EGL_NO_CONTEXT) {
			mEGLContext =
				createGLContext(2, 0, mEGLConfig!!)
		}

		mEGLSurface =
			createSurface(mEGLConfig, mSurface)

		if (mEGLSurface == null || mEGLSurface === EGL14.EGL_NO_SURFACE) {
			val error = EGL14.eglGetError()
			throw RuntimeException(
				"eglCreateWindowSurface failed " + GLUtils.getEGLErrorString(
					error
				)
			)
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
				contextPreserveDrawingBuffer = false
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
		if (!swapBuffers(
				mEGLSurface
			)
		) {
			if (TNSCanvas.enableDebug) {
				Log.e("JS", "GLContext: Cannot swap buffers!")
			}
		}
	}

	private fun deInitEGL() {
		makeEGLContextCurrent()
		destroySurface(mEGLSurface)
		EGL14.eglDestroyContext(
			mEGLDisplay,
			mEGLContext
		)
		EGL14.eglTerminate(mEGLDisplay)
	}

	private fun resizeSurface() {
		if (contextType == TNSCanvas.ContextType.NONE) {
			return
		}
		if (canvas.isAttachedToWindow) {
			return
		}

		if (surface?.isCreated == false) {
			val oldSurface = mEGLSurface

			mEGLSurface = createSurface(mEGLConfig, mSurface)

			if (mEGLSurface == null || mEGLSurface === EGL14.EGL_NO_SURFACE) {
				val error = EGL14.eglGetError()
				throw RuntimeException(
					"eglCreateWindowSurface failed " + GLUtils.getEGLErrorString(
						error
					)
				)
			}

			drawingBufferWidth = canvas.width
			drawingBufferHeight = canvas.height


			// Switch to our EGLContext
			makeEGLContextCurrent()

			destroySurface(oldSurface)
		}

		GLES20.glClearColor(0f, 0f, 0f, 0f)
		GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT)

		if (nativeContext != 0L) {
			GLES20.glViewport(0, 0, canvas.width, canvas.height)
			val frameBuffers = IntArray(1)
			GLES20.glGetIntegerv(GLES20.GL_FRAMEBUFFER_BINDING, frameBuffers, 0)
			var samples = 0
			if (contextAntialias) {
				samples = 4
			}
			val metrics = canvas.resources.displayMetrics

			TNSCanvas.nativeResizeSurface(
				nativeContext,
				drawingBufferWidth.toFloat(),
				drawingBufferHeight.toFloat(),
				scale,
				frameBuffers[0],
				samples,
				true,
				metrics.density,
			)
			swapBuffers(mEGLSurface)
		} else {
			if (swapBuffers(mEGLSurface)) {
				if (TNSCanvas.enableDebug) {
					Log.e("JS", "GLContext: Cannot swap buffers!")
				}
			}
		}

	}

	private fun init2D() {
		if (contextType == TNSCanvas.ContextType.CANVAS && nativeContext == 0L) {
			val width = canvas.width
			val height = canvas.height
			val frameBuffers = IntArray(1)
			val view = surface!!
			if (drawingBufferWidth == 0) {
				drawingBufferWidth = width
			}
			if (drawingBufferHeight == 0) {
				drawingBufferHeight = height
			}
			if (drawingBufferWidth == 0) {
				drawingBufferWidth = 1
			}
			if (drawingBufferHeight == 0) {
				drawingBufferHeight = 1
			}

			GLES20.glViewport(0, 0, drawingBufferWidth, drawingBufferHeight)
			GLES20.glGetIntegerv(GLES20.GL_FRAMEBUFFER_BINDING, frameBuffers, 0)
			var samples = 0
			if (contextAntialias) {
				samples = 4
			}
			val metrics = view.resources.displayMetrics
			nativeContext = TNSCanvas.nativeInitContext(
				drawingBufferWidth.toFloat(),
				drawingBufferHeight.toFloat(),
				scale,
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

	fun setTexture(texture: Any?) {
		mSurface = texture
		initEGL()
	}

	fun flush() {
		if (softwareRenderer) {
			if (nativeContext != 0L) {
				TNSCanvas.nativeCustomWithBitmapFlush(nativeContext, cpuView!!.view!!)
				if (Looper.myLooper() != Looper.getMainLooper()) {
					mainHandler.post {
						invalidateState = InvalidateState.NONE
						cpuView?.invalidate()
					}
				}
			}
		} else {
			makeEGLContextCurrent()
			if (nativeContext != 0L && invalidateState == InvalidateState.INVALIDATING) {
				TNSCanvas.nativeFlush(nativeContext)
				if (!swapBuffers(mEGLSurface)) {
					if (TNSCanvas.enableDebug) {
						Log.e("JS", "GLContext: Cannot swap buffers!")
					}
				}
				invalidateState = InvalidateState.NONE
			} else {
				// WebGL
				if (!swapBuffers(mEGLSurface)) {
					if (TNSCanvas.enableDebug) {
						Log.e("JS", "GLContext: Cannot swap buffers!")
					}
				}
				invalidateState = InvalidateState.NONE
			}
		}
	}

	fun toData(): ByteArray? {
		if (contextType == TNSCanvas.ContextType.CANVAS) {
			return TNSCanvas.nativeToDataImpl(nativeContext)
		} else if (contextType == TNSCanvas.ContextType.WEBGL) {
			val bm = surface!!.getBitmap(canvas.width, canvas.height)
			val data = ByteArray(bm!!.width * bm.height * 4)
			val buffer = ByteBuffer.wrap(data)
			bm.copyPixelsToBuffer(buffer)
			return data
		}
		return ByteArray(0)
	}

	fun snapshot(): ByteBuffer {
		makeEGLContextCurrent()
		if (contextType == TNSCanvas.ContextType.CANVAS) {
			return TNSCanvas.nativeSnapshotCanvasImpl(nativeContext)
		} else if (contextType == TNSCanvas.ContextType.WEBGL) {
			val bm = surface?.bitmap

			if (bm != null) {
				return Utils.getByteBufferFromBitmap(bm)
			}

			val buffer = ByteBuffer.allocateDirect(canvas.width * canvas.height * 4)
			buffer.order(ByteOrder.nativeOrder())
			GLES20.glFinish()
			GLES20.glReadPixels(
				0,
				0,
				canvas.width,
				canvas.height,
				GLES20.GL_RGBA,
				GLES20.GL_UNSIGNED_BYTE,
				buffer
			)
			buffer.rewind()
			return buffer
		}
		return ByteBuffer.allocate(0)
	}

	fun createSurface(config: EGLConfig?, surface: Any?): EGLSurface {
		if (surface == null) {
			var width = canvas.width
			var height = canvas.height

			if (width == 0) {
				width = 1
			}
			if (height == 0) {
				height = 1
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

	fun makeCurrent(surface: EGLSurface?): Boolean {
		return EGL14.eglMakeCurrent(mEGLDisplay, surface, surface, mEGLContext)
	}

	fun destroySurface(surface: EGLSurface?): Boolean {
		return EGL14.eglDestroySurface(mEGLDisplay, surface)
	}

	fun swapBuffers(surface: EGLSurface?): Boolean {
		return EGL14.eglSwapBuffers(mEGLDisplay, surface)
	}


	fun handleAttributes(contextAttributes: String?) {
		contextAttributes?.let {
			try {
				val obj = JSONObject(it)
				obj.keys().forEach { key ->
					val value = obj[key]
					when (key) {
						"alpha" -> {
							contextAlpha = value as Boolean
						}
						"antialias" -> {
							contextAntialias = value as Boolean
						}
						"depth" -> {
							contextDepth = value as Boolean
						}
						"failIfMajorPerformanceCaveat" -> {
							contextFailIfMajorPerformanceCaveat = value as Boolean
						}
						"premultipliedAlpha" -> {
							contextPremultipliedAlpha = value as Boolean
						}
						"preserveDrawingBuffer" -> {
							contextPreserveDrawingBuffer = value as Boolean
						}
						"stencil" -> {
							contextStencil = value as Boolean
						}
						"xrCompatible" -> {
							contextXrCompatible = value as Boolean
						}
						"desynchronized" -> contextDesynchronized = value as Boolean
						"powerPreference" -> contextPowerPreference = value as String?
						else -> {
						}
					}
				}
			} catch (e: Exception) {
			}
		}
	}

	fun isMainThread(): Boolean {
		return Looper.myLooper() == Looper.getMainLooper()
	}

	fun getContext(type: String, contextAttributes: String?): TNSCanvasRenderingContext? {
		handleAttributes(contextAttributes)
		if (type == "2d" || type == "experimental-webgl" || type == "webgl" || type == "webgl2" && Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2) {

			if (!isMainThread()) {
				mainHandler.post {
					surface?.apply {
						isOpaque = !contextAlpha
					}
				}
			} else {
				surface?.apply {
					isOpaque = !contextAlpha
				}
			}

			if (type.contains("webgl") && !DID_CHECK_WEBGL_SUPPORT) {
				checkGLSupport()
			}

			if (type == "2d") {
				contextType = TNSCanvas.ContextType.CANVAS
				if (!softwareRenderer) {
					init2D()
				}
			}
		}

		when (type) {
			"2d" -> {
				actualContextType = "2d"
				if (renderingContext2d == null) {
					renderingContext2d = TNSCanvasRenderingContext2D(canvas)
				}
				contextType = TNSCanvas.ContextType.CANVAS
				return renderingContext2d
			}
			"webgl", "experimental-webgl" -> {
				actualContextType = "webgl"
				if (webGLRenderingContext == null) {
					contextType = TNSCanvas.ContextType.WEBGL
					webGLRenderingContext = TNSWebGLRenderingContext(canvas)
				}

				return webGLRenderingContext
			}
			"webgl2" -> {
				if (webGL2RenderingContext == null) {
					if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2 && DID_CHECK_WEBGL_SUPPORT && IS_WEBGL_2_SUPPORT) {
						actualContextType = "webgl"
						contextType = TNSCanvas.ContextType.WEBGL
						webGL2RenderingContext = TNSWebGL2RenderingContext(canvas)
						isWebGL = true
					} else {
						isWebGL = false
						contextType = TNSCanvas.ContextType.NONE
						return null
					}
				}
				return webGL2RenderingContext
			}
		}
		contextType = TNSCanvas.ContextType.NONE

		return null
	}


	fun toDataURL(type: String = "image/png", quality: Float = 0.92f): String? {
		if (contextType == TNSCanvas.ContextType.WEBGL) {
			val bm = surface!!.getBitmap(canvas.width, canvas.height)
			val os = ByteArrayOutputStream()
			var dataType = "image/png"
			when (type) {
				"image/jpeg" -> dataType = "image/jpeg"
				"image/jpg" -> dataType = "image/jpg"
				else -> {
				}
			}
			if (bm != null) {
				if (type == "image/jpeg" || type == "image/jpg") {
					bm.compress(Bitmap.CompressFormat.JPEG, (quality * 100).toInt(), os)
				} else {
					bm.compress(Bitmap.CompressFormat.PNG, (quality * 100).toInt(), os)
				}
			}
			return String.format(
				"data:%s;base64,%s",
				dataType,
				Base64.encodeToString(os.toByteArray(), Base64.NO_WRAP)
			)
		}
		return TNSCanvas.nativeDataURLImpl(nativeContext, type, quality)
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
