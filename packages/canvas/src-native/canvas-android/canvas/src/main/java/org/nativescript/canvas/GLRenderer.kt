package org.nativescript.canvas

import android.annotation.SuppressLint
import android.app.ActivityManager
import android.content.Context
import android.graphics.Bitmap
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
import javax.microedition.khronos.egl.EGL10

internal class GLRenderer(
	val canvas: TNSCanvas,
	val context: Context,
	val softwareRenderer: Boolean
) : Choreographer.FrameCallback {

	internal var nativeContext: Long = 0L
	internal var surface: GLView? = null
	internal var cpuView: CPUView? = null

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
	var mOffScreenEGLConfig: EGLConfig? = null


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

	var listener: TNSCanvas.Listener? = null
		set(value) {
			field = value
			surface?.setListener(listener)
		}


	@JvmField
	internal var invalidateState = INVALIDATE_STATE_NONE
	internal var contextType = TNSCanvas.ContextType.NONE

	class ContextAttributes {

		var alpha = true

		var antialias = true

		var depth = true

		var failIfMajorPerformanceCaveat = false

		var powerPreference: String? = "default"
			set(value) {
				field = value ?: "default"
			}

		var premultipliedAlpha = true

		var preserveDrawingBuffer = false

		var stencil = false

		var desynchronized = false

		var xrCompatible = false

		fun toJSON(): String {
			val obj = JSONObject()
			obj.put("alpha", alpha)
			obj.put("antialias", antialias)
			obj.put("depth", depth)
			obj.put("failIfMajorPerformanceCaveat", failIfMajorPerformanceCaveat)
			obj.put("premultipliedAlpha", premultipliedAlpha)
			obj.put("preserveDrawingBuffer", preserveDrawingBuffer)
			obj.put("stencil", stencil)
			obj.put("xrCompatible", xrCompatible)
			obj.put("desynchronized", desynchronized)
			obj.put("powerPreference", powerPreference)
			return obj.toString()
		}

		companion object {
			@JvmStatic
			fun fromMap(contextAttributes: Map<String, Any>): ContextAttributes {
				val attr = ContextAttributes()
				val keys = contextAttributes.keys
				for (key in keys) {
					val value = contextAttributes[key]
					when (key) {
						"alpha" -> {
							attr.alpha = value as Boolean
						}
						"antialias" -> {
							attr.antialias = value as Boolean
						}
						"depth" -> {
							attr.depth = value as Boolean
						}
						"failIfMajorPerformanceCaveat" -> {
							attr.failIfMajorPerformanceCaveat = value as Boolean
						}
						"premultipliedAlpha" -> {
							attr.premultipliedAlpha = value as Boolean
						}
						"preserveDrawingBuffer" -> {
							attr.preserveDrawingBuffer = value as Boolean
						}
						"stencil" -> {
							attr.stencil = value as Boolean
						}
						"xrCompatible" -> {
							attr.xrCompatible = value as Boolean
						}
						"desynchronized" -> attr.desynchronized = value as Boolean
						"powerPreference" -> attr.powerPreference = value as String?
						else -> {
						}
					}
				}
				return attr
			}

			@JvmStatic
			fun fromJSON(contextAttributes: JSONObject): ContextAttributes {
				val attr = ContextAttributes()
				val keys = contextAttributes.keys()
				for (key in keys) {
					val value = contextAttributes[key]
					when (key) {
						"alpha" -> {
							attr.alpha = value as Boolean
						}
						"antialias" -> {
							attr.antialias = value as Boolean
						}
						"depth" -> {
							attr.depth = value as Boolean
						}
						"failIfMajorPerformanceCaveat" -> {
							attr.failIfMajorPerformanceCaveat = value as Boolean
						}
						"premultipliedAlpha" -> {
							attr.premultipliedAlpha = value as Boolean
						}
						"preserveDrawingBuffer" -> {
							attr.preserveDrawingBuffer = value as Boolean
						}
						"stencil" -> {
							attr.stencil = value as Boolean
						}
						"xrCompatible" -> {
							attr.xrCompatible = value as Boolean
						}
						"desynchronized" -> attr.desynchronized = value as Boolean
						"powerPreference" -> attr.powerPreference = value as String?
						else -> {
						}
					}
				}
				return attr
			}

			val default = ContextAttributes()
		}
	}

	@JvmField
	var contextAttributes = ContextAttributes.default

	private var actualContextType = ""

	@JvmField
	internal var glVersion = 0

	internal var mainHandler = Handler(Looper.myLooper()!!)

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
			// Only pending state flag is accepted
			if (invalidateState == INVALIDATE_STATE_PENDING) {
				invalidateState = INVALIDATE_STATE_INVALIDATING
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
				resizeSurface(width, height)
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


	val configsCount = IntArray(1)
	val configs = arrayOfNulls<EGLConfig>(1)
	val offScreenConfigs = arrayOfNulls<EGLConfig>(1)
	private fun initConfig() {
		// Find a compatible EGLConfig
		if (mOffScreenEGLConfig != null && mEGLConfig != null) {
			return
		}

		var type = EGL14.EGL_OPENGL_ES2_BIT
		var depthSize = 16
		var stencilSize = 0
		var alpha = 8
		var useAlpha = true


		val checkES3 = hasExtension(
			EGL14.eglQueryString(mEGLDisplay, EGL10.EGL_EXTENSIONS),
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
					EGL10.EGL_RENDERABLE_TYPE, value, 0
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
			}
		}

		if (highestEsVersion >= 3) {
			IS_WEBGL_2_SUPPORT = true
		}

		DID_CHECK_WEBGL_SUPPORT = true

		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2) {
			if (glVersion > 2 && actualContextType == "webgl2" && highestEsVersion >= 3) {
				type = EGLExt.EGL_OPENGL_ES3_BIT_KHR
			}
		}

		val isContextTypeCanvas = contextType == TNSCanvas.ContextType.CANVAS

		if (contextAttributes.stencil && !isContextTypeCanvas) {
			stencilSize = 8
			contextAttributes.stencil = true
		}
		if (!contextAttributes.depth || isContextTypeCanvas) {
			contextAttributes.depth = false
			depthSize = 0
		}
		useAlpha = contextAttributes.alpha
		antialias = if (isContextTypeCanvas) {
			false
		} else {
			contextAttributes.antialias
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

		var offScreenConfigSpec = intArrayOf(
			EGL10.EGL_SURFACE_TYPE, EGL14.EGL_PBUFFER_BIT,
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

			offScreenConfigSpec = offScreenConfigSpec.copyOf(offScreenConfigSpec.size + 5)
			val ofLast = offScreenConfigSpec.size - 1
			offScreenConfigSpec[ofLast - 4] = EGL10.EGL_SAMPLE_BUFFERS
			offScreenConfigSpec[ofLast - 3] = 1
			offScreenConfigSpec[ofLast - 2] = EGL10.EGL_SAMPLES
			offScreenConfigSpec[ofLast - 1] = 4
			offScreenConfigSpec[ofLast] = EGL10.EGL_NONE
		} else {
			configSpec = configSpec.copyOf(configSpec.size + 1)
			configSpec[configSpec.size - 1] = EGL10.EGL_NONE

			offScreenConfigSpec = offScreenConfigSpec.copyOf(offScreenConfigSpec.size + 1)
			offScreenConfigSpec[offScreenConfigSpec.size - 1] = EGL10.EGL_NONE
		}

		var defaultChosen =
			EGL14.eglChooseConfig(mEGLDisplay, configSpec, 0, configs, 0, 1, configsCount, 0)

		if (defaultChosen && configs.first() == null) {

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

			defaultChosen =
				EGL14.eglChooseConfig(mEGLDisplay, configSpec, 0, configs, 0, 1, configsCount, 0)

			if (antialias) {
				contextAttributes.antialias = false
			}
		}

		if (!defaultChosen) {
			if (antialias) {
				contextAttributes.antialias = false
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
				val config =
					EGL14.eglChooseConfig(mEGLDisplay, configSpec, 0, configs, 0, 1, configsCount, 0)
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
		} else {
			// try using the closest config

			var bestConfig: EGLConfig? = null
			for (config in configs) {
				config?.let {
					val r = findConfigAttrib(it, EGL10.EGL_RED_SIZE, 0)
					val g = findConfigAttrib(it, EGL10.EGL_GREEN_SIZE, 0)
					val b = findConfigAttrib(it, EGL10.EGL_BLUE_SIZE, 0)
					val a = findConfigAttrib(it, EGL10.EGL_ALPHA_SIZE, 0)
					if (r == 8 && g == 8 && b == 8 && a == 8) {
						bestConfig = it
					}
				}

				if (bestConfig != null) {
					break
				}
			}
			if (bestConfig == null) {
				bestConfig = configs[0]
			}

			mEGLConfig = bestConfig

		}

		var offScreenChosen = EGL14.eglChooseConfig(
			mEGLDisplay,
			offScreenConfigSpec,
			0,
			offScreenConfigs,
			0,
			1,
			configsCount,
			0
		)

		if (offScreenChosen && configs.first() == null) {

			offScreenConfigSpec = intArrayOf(
				EGL10.EGL_SURFACE_TYPE, EGL14.EGL_PBUFFER_BIT,
				EGL10.EGL_RENDERABLE_TYPE, type,
				EGL10.EGL_RED_SIZE, 8,
				EGL10.EGL_GREEN_SIZE, 8,
				EGL10.EGL_BLUE_SIZE, 8,
				EGL10.EGL_ALPHA_SIZE, alpha,
				EGL10.EGL_DEPTH_SIZE, depthSize,
				EGL10.EGL_STENCIL_SIZE, stencilSize,
				EGL10.EGL_NONE
			)

			offScreenChosen = EGL14.eglChooseConfig(
				mEGLDisplay,
				offScreenConfigSpec,
				0,
				offScreenConfigs,
				0,
				1,
				configsCount,
				0
			)

			if (antialias) {
				contextAttributes.antialias = false
			}
		}


		if (!offScreenChosen) {
			if (antialias) {
				contextAttributes.antialias = false

				antialias = false

				offScreenConfigSpec = intArrayOf(
					EGL10.EGL_SURFACE_TYPE, EGL14.EGL_PBUFFER_BIT,
					EGL10.EGL_RENDERABLE_TYPE, type,
					EGL10.EGL_RED_SIZE, 8,
					EGL10.EGL_GREEN_SIZE, 8,
					EGL10.EGL_BLUE_SIZE, 8,
					EGL10.EGL_ALPHA_SIZE, alpha,
					EGL10.EGL_DEPTH_SIZE, depthSize,
					EGL10.EGL_STENCIL_SIZE, stencilSize,
					EGL10.EGL_NONE
				)
				val config = EGL14.eglChooseConfig(
					mEGLDisplay,
					offScreenConfigSpec,
					0,
					offScreenConfigs,
					0,
					1,
					configsCount,
					0
				)
				if (config) {
					mOffScreenEGLConfig = offScreenConfigs[0]
				}
			} else {
				throw IllegalArgumentException(
					"eglChooseConfig failed " + GLUtils.getEGLErrorString(
						EGL14.eglGetError()
					)
				)
			}
		} else {
			// try using the closest config

			var bestConfig: EGLConfig? = null
			for (config in offScreenConfigs) {
				config?.let {
					val r = findConfigAttrib(it, EGL10.EGL_RED_SIZE, 0)
					val g = findConfigAttrib(it, EGL10.EGL_GREEN_SIZE, 0)
					val b = findConfigAttrib(it, EGL10.EGL_BLUE_SIZE, 0)
					val a = findConfigAttrib(it, EGL10.EGL_ALPHA_SIZE, 0)
					if (r == 8 && g == 8 && b == 8 && a == 8) {
						bestConfig = it
					}
				}

				if (bestConfig != null) {
					break
				}
			}
			if (bestConfig == null) {
				bestConfig = configs[0]
			}

			mOffScreenEGLConfig = bestConfig

		}

		if (mEGLConfig == null || mOffScreenEGLConfig == null) {
			throw RuntimeException("eglConfig not initialized")
		}
	}

	private val singleIntArray = IntArray(1)

	private fun findConfigAttrib(config: EGLConfig, attribute: Int, defaultValue: Int): Int {
		return if (EGL14.eglGetConfigAttrib(mEGLDisplay, config, attribute, singleIntArray, 0)) {
			singleIntArray[0]
		} else defaultValue
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

		initConfig()

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
		if (contextAttributes.preserveDrawingBuffer) {
			// Enable buffer preservation -- allows app to draw over previous frames without clearing
			val didEnableBufferPreservation = EGL14.eglSurfaceAttrib(
				EGL14.eglGetCurrentDisplay(), EGL14.eglGetCurrentSurface(EGL14.EGL_DRAW),
				EGL14.EGL_SWAP_BEHAVIOR, EGL14.EGL_BUFFER_PRESERVED
			)
			if (!didEnableBufferPreservation) {
				contextAttributes.preserveDrawingBuffer = false
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
				Log.e(TAG, "GLContext: Cannot swap buffers!")
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

	private fun resizeSurface(width: Int, height: Int) {
		if (contextType == TNSCanvas.ContextType.NONE) {
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

		canvas.listener?.surfaceResize(width, height)

		if (swapBuffers(mEGLSurface)) {
			if (TNSCanvas.enableDebug) {
				Log.e(TAG, "GLContext: Cannot swap buffers!")
			}
		}

	}

	fun setTexture(texture: Any?) {
		mSurface = texture
		initEGL()
	}

	private fun invalidateView(func: () -> Unit) {
		invalidateState =
			invalidateState and INVALIDATE_STATE_PENDING.inv()

		func()
		invalidateState =
			invalidateState and INVALIDATE_STATE_INVALIDATING.inv()
	}

	fun flush() {
		/*
		if ((invalidateState and INVALIDATE_STATE_INVALIDATING) == INVALIDATE_STATE_INVALIDATING) {
			if (nativeContext != 0L) {
				if (softwareRenderer) {
					TNSCanvas.nativeCustomWithBitmapFlush(nativeContext, cpuView!!.view!!)
					if (Looper.myLooper() != Looper.getMainLooper()) {
						mainHandler.post {
							invalidateView {
								cpuView?.invalidate()
							}
						}
					} else {
						invalidateView {
							cpuView?.invalidate()
						}
					}
				} else {
					makeEGLContextCurrent()
					invalidateView {
						TNSCanvas.nativeFlush(nativeContext)
						if (!swapBuffers(mEGLSurface)) {
							Log.e(TAG, "GLContext: Cannot swap buffers!")
						}
					}
				}
			} else {
				if (!softwareRenderer) {
					makeEGLContextCurrent()
					invalidateView {
						if (!swapBuffers(mEGLSurface)) {
							Log.e(TAG, "GLContext: Cannot swap buffers!")
						}
					}
				}
			}
		}
		*/

		/*
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
		 */
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

		surfaceWidth = this.surface?.width ?: this.cpuView?.width ?: 0
		surfaceHeight = this.surface?.height ?: this.cpuView?.height ?: 0

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


	fun isMainThread(): Boolean {
		return Looper.myLooper() == Looper.getMainLooper()
	}


	fun getContext(type: String, contextAttributes: ContextAttributes) {
		if (actualContextType.isNotEmpty() && contextType == TNSCanvas.ContextType.NONE) {
			return
		}

		contextType = TNSCanvas.ContextType.fromString(type)
		this.contextAttributes = contextAttributes
		if (type == "2d" || type == "experimental-webgl" || type == "webgl" || type == "webgl2" && Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2) {

			if (!isMainThread()) {
				mainHandler.post {
					surface?.apply {
						isOpaque = !contextAttributes.premultipliedAlpha
					}
				}
			} else {
				surface?.apply {
					isOpaque = !contextAttributes.premultipliedAlpha
				}
			}

			if (type.contains("webgl") && !DID_CHECK_WEBGL_SUPPORT) {
				checkGLSupport()
			}

			if (type == "2d") {
				contextType = TNSCanvas.ContextType.CANVAS
			}
		}

		when (type) {
			"2d" -> {
				actualContextType = "2d"
			}
			"webgl", "experimental-webgl" -> {
				actualContextType = "webgl"
			}
			"webgl2" -> {
				if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2 && DID_CHECK_WEBGL_SUPPORT && IS_WEBGL_2_SUPPORT) {
					actualContextType = "webgl"
					contextType = TNSCanvas.ContextType.WEBGL
					isWebGL = true
				} else {
					isWebGL = false
					contextType = TNSCanvas.ContextType.NONE
					actualContextType = "none"
				}
			}
			else -> actualContextType = "none"
		}

		if (mEGLSurface == null || mEGLSurface === EGL14.EGL_NO_SURFACE) {
			initEGL()
		}
	}


	fun toDataURL(type: String = "image/png", quality: Float = 0.92f): String? {
		if (contextType == TNSCanvas.ContextType.NONE) {
			return null
		}
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
		// Invalidate state bitwise flags
		internal const val INVALIDATE_STATE_NONE = 0
		internal const val INVALIDATE_STATE_PENDING = 1
		internal const val INVALIDATE_STATE_INVALIDATING = 2

		var DID_CHECK_WEBGL_SUPPORT = false
		var IS_WEBGL_2_SUPPORT = false
		private var highestEsVersion = 0
		const val TAG = "GLContext"
		const val EGL_CONTEXT_CLIENT_VERSION = 0x3098
		const val EGL_CONTEXT_CLIENT_MINOR_VERSION = 0x30FB
		private const val EGL_OPENGL_ES_BIT = 0x0001
		private const val EGL_OPENGL_ES2_BIT = 0x0004
		private const val EGL_OPENGL_ES3_BIT_KHR = 0x0040

	}
}
