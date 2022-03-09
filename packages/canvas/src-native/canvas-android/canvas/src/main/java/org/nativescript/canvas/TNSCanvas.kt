package org.nativescript.canvas

import android.app.Activity
import android.app.ActivityManager
import android.app.Application
import android.app.Application.ActivityLifecycleCallbacks
import android.content.Context
import android.graphics.Bitmap
import android.graphics.Color
import android.opengl.GLES20
import android.os.*
import android.util.AttributeSet
import android.util.Base64
import android.util.Log
import android.view.Choreographer
import android.view.Choreographer.FrameCallback
import android.view.ViewGroup
import android.widget.FrameLayout
import androidx.core.text.TextUtilsCompat
import androidx.core.view.ViewCompat
import java.io.ByteArrayOutputStream
import java.lang.ref.WeakReference
import java.nio.ByteBuffer
import java.util.*
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit


/**
 * Created by triniwiz on 3/29/20
 */
class TNSCanvas : FrameLayout, FrameCallback, ActivityLifecycleCallbacks {
	internal var nativeContext: Long = 0L
	internal var surface: GLView? = null
	internal var cpuView: CPUView? = null
	var isHandleInvalidationManually = false
	internal var renderingContext2d: TNSCanvasRenderingContext? = null
	internal var scale = 1f
	internal var ctx: Context? = null

	var ignorePixelScaling: Boolean = false
		set(value) {
			field = value
			surface?.ignorePixelScaling = value
		}

	@JvmField
	internal var invalidateState = InvalidateState.NONE
	internal var contextType = ContextType.NONE
	internal var actualContextType = ""
	internal var useCpu = false

	@JvmField
	internal var contextAlpha = true

	@JvmField
	internal var contextAntialias = true

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

	private val mainHandler = Handler(Looper.getMainLooper())

	enum class InvalidateState {
		NONE, PENDING, INVALIDATING
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

	constructor(context: Context) : super(context, null) {
		init(context, false)
	}

	constructor(context: Context, useCpu: Boolean) : super(context, null) {
		init(context, useCpu)
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init(context, false)
	}

	private fun init(context: Context, useCpu: Boolean) {
		if (isInEditMode) {
			return
		}
		this.useCpu = useCpu
		if (!isLibraryLoaded) {
			System.loadLibrary("canvasnative")
			isLibraryLoaded = true
		}
		setBackgroundColor(Color.TRANSPARENT)
		surface = GLView(context)
		surface!!.gLContext!!.reference = WeakReference(this)
		ctx = context
		// scale = context.resources.displayMetrics.density
		glVersion = if (detectOpenGLES30() && !Utils.isEmulator) {
			3
		} else {
			2
		}
		surface!!.layoutParams = LayoutParams(
			ViewGroup.LayoutParams.MATCH_PARENT,
			ViewGroup.LayoutParams.MATCH_PARENT
		)
		if (useCpu) {
			cpuView = CPUView(context)
			cpuView!!.canvasView = WeakReference(this)
			cpuView!!.layoutParams = LayoutParams(
				ViewGroup.LayoutParams.MATCH_PARENT,
				ViewGroup.LayoutParams.MATCH_PARENT
			)
			initCPUThread()
			addView(cpuView)
		} else {
			addView(surface)
		}
	}

	private fun initCPUThread() {
		cpuHandlerThread = HandlerThread("CanvasViewCpuThread")
		cpuHandlerThread!!.start()
		cpuHandler = Handler(cpuHandlerThread!!.looper)
	}

	val drawingBufferWidth: Int
		get() = if (useCpu) {
			cpuView!!.width
		} else surface!!.drawingBufferWidth
	val drawingBufferHeight: Int
		get() = if (useCpu) {
			cpuView!!.height
		} else surface!!.drawingBufferHeight

	private fun detectOpenGLES30(): Boolean {
		val am = context.getSystemService(Context.ACTIVITY_SERVICE) as? ActivityManager
		return am?.let {
			it.deviceConfigurationInfo.reqGlEsVersion >= 0x30000
		} ?: false
	}

	fun onPause() {
		surface?.gLContext?.onPause()
	}

	fun onResume() {
		surface?.gLContext?.onResume()
	}

	fun destroy() {
		surface?.gLContext?.destroy();
	}

	@Synchronized
	@Throws(Throwable::class)
	protected fun finalize() {
		if (nativeContext != 0L) {
			nativeDestroyContext(nativeContext)
			nativeContext = 0
		}

		if (useCpu) {
			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2) {
				cpuHandlerThread?.quitSafely()
			}
		} else {
			surface?.gLContext?.destroy()
		}
	}

	var cpuHandler: Handler? = null
	var cpuHandlerThread: HandlerThread? = null
	fun queueEvent(runnable: Runnable?) {
		runnable?.let {
			if (useCpu) {
				if (!cpuHandlerThread!!.isAlive || cpuHandlerThread!!.isInterrupted) {
					cpuHandlerThread = null
					cpuHandler = null
					initCPUThread()
				}
				cpuHandler?.post(it)
			} else {
				surface?.queueEvent(it)
			}
		}
	}

	fun setupActivityHandler(app: Application) {
		app.unregisterActivityLifecycleCallbacks(this)
		app.registerActivityLifecycleCallbacks(this)
	}

	override fun onActivityCreated(activity: Activity, savedInstanceState: Bundle?) {}
	override fun onActivityStarted(activity: Activity) {}
	override fun onActivityResumed(activity: Activity) {
		//onResume();
	}

	override fun onActivityPaused(activity: Activity) {}
	override fun onActivityStopped(activity: Activity) {
		//onPause();
	}

	override fun onActivitySaveInstanceState(activity: Activity, outState: Bundle) {}
	override fun onActivityDestroyed(activity: Activity) {}
	interface DataURLListener {
		fun onResult(data: String?)
	}

	fun toData(): ByteArray? {
		if (contextType == ContextType.CANVAS) {
			val lock = CountDownLatch(1)
			val data = arrayOfNulls<ByteArray>(1)
			queueEvent(Runnable {
				data[0] = nativeToData(nativeContext)
				lock.countDown()
			})
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (ignore: InterruptedException) {
			}
			return data[0]
		} else if (contextType == ContextType.WEBGL) {
			val bm = surface!!.getBitmap(width, height)
			val data = ByteArray(bm!!.width * bm.height * 4)
			val buffer = ByteBuffer.wrap(data)
			bm.copyPixelsToBuffer(buffer)
			return data
		}
		return ByteArray(0)
	}

	fun snapshot(): ByteArray {
		if (contextType == ContextType.CANVAS) {
			val lock = CountDownLatch(1)
			val ss = ArrayList<ByteArray>()
			// initCanvas();
			queueEvent(Runnable {
				ss.add(nativeSnapshotCanvas(nativeContext))
				lock.countDown()
			})
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (ignore: InterruptedException) {
			}
			return ss[0]
		} else if (contextType == ContextType.WEBGL) {
			val bm = surface!!.getBitmap(width, height)
			return Utils.getBytesFromBitmap(bm)
		}
		return ByteArray(0)
	}

	fun toDataURLAsync(listener: DataURLListener) {
		toDataURLAsync("image/png", listener)
	}

	fun toDataURLAsync(type: String?, listener: DataURLListener) {
		toDataURLAsync(type, 0.92f, listener)
	}

	fun toDataURLAsync(type: String?, quality: Float, listener: DataURLListener) {
		queueEvent(Runnable { listener.onResult(nativeDataURL(nativeContext, type, quality)) })
	}

	@JvmOverloads
	fun toDataURL(type: String = "image/png", quality: Float = 0.92f): String? {
		if (contextType == ContextType.WEBGL) {
			val bm = surface!!.getBitmap(width, height)
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
		val lock = CountDownLatch(1)
		val data = arrayOfNulls<String>(1)
		queueEvent(Runnable {
			data[0] = nativeDataURL(nativeContext, type, quality)
			lock.countDown()
		})
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (ignore: InterruptedException) {
		}
		return data[0]
	}

	internal var webGLRenderingContext: TNSWebGLRenderingContext? = null
	internal var webGL2RenderingContext: TNSWebGL2RenderingContext? = null

	enum class ContextType {
		NONE, CANVAS, WEBGL
	}

	private var needRenderRequest = 0

	fun resizeViewPort() {
		queueEvent(Runnable { GLES20.glViewport(0, 0, width, height) })
	}

	internal fun initCanvas() {
		if (surface != null && nativeContext == 0L) {
			var width = surface!!.width
			var height = surface!!.height
			// dynamically created view w/o layout
			val params = surface!!.layoutParams
			if (width == 0 && params != null) {
				width = params.width
				needRenderRequest += 1
			}
			if (height == 0 && params != null) {
				height = params.height
				needRenderRequest += 1
			}
			// size was not set set to 1
			if (width == 0) {
				width = 1
				needRenderRequest += 1
			}
			if (height == 0) {
				height = 1
				needRenderRequest += 1
			}
			if (newSize == null || newSize!!.width == 0 && newSize!!.height == 0) {
				newSize = Size(width, height)
			}
			var finalWidth = width
			var finalHeight = height
			if (ignorePixelScaling) {
				if (width != 1 || height != 1) {
					val density = resources.displayMetrics.density
					finalWidth = (finalWidth / density).toInt()
					finalHeight = (finalHeight / density).toInt()
				}
			}
			surface!!.queueEvent {
				if (nativeContext == 0L && finalWidth > 0 && finalHeight > 0) {
					Log.d("com.test", "$finalWidth $finalHeight")
					// GLES20.glClearColor(1F, 1F, 1F, 1F);
					// GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT);
					val frameBuffers = IntArray(1)
					GLES20.glViewport(0, 0, finalWidth, finalHeight)
					GLES20.glGetIntegerv(GLES20.GL_FRAMEBUFFER_BINDING, frameBuffers, 0)
					var samples = 0
					if (contextAntialias) {
						samples = 4
					}
					val metrics = resources.displayMetrics
					nativeContext = nativeInitContext(
						finalWidth.toFloat(),
						finalHeight.toFloat(), scale,
						frameBuffers[0],
						samples,
						true,
						Color.BLACK,
						metrics.density,
						direction.toNative()
					)
				}
			}
		}
	}

	fun getContext(type: String): TNSCanvasRenderingContext? {
		val attributes = HashMap<String, Any>()
		if (type == "2d") {
			attributes["alpha"] = true
			attributes["desynchronized"] = false
		} else if (type.contains("webgl")) {
			attributes["alpha"] = true
			attributes["depth"] = true
			attributes["antialias"] = true
			attributes["failIfMajorPerformanceCaveat"] = false
			attributes["powerPreference"] = "default"
			attributes["premultipliedAlpha"] = true
			attributes["preserveDrawingBuffer"] = false
			attributes["stencil"] = false
			attributes["xrCompatible"] = false
			attributes["desynchronized"] = false
		}
		return getContext(type, attributes)
	}

	private fun handleAttributes(contextAttributes: Map<String, Any>?) {
		if (contextAttributes != null) {
			val keys = contextAttributes.keys
			for (key in keys) {
				val value = contextAttributes[key]
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
		}
	}

	fun getContext(type: String, contextAttributes: Map<String, Any>?): TNSCanvasRenderingContext? {
		handleAttributes(contextAttributes)
		if (type == "2d" || type == "experimental-webgl" || type == "webgl" || type == "webgl2" && Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2) {
			mainHandler.post {
				surface?.apply {
					isOpaque = !contextAlpha
				}
			}

			if (type == "2d") {
				contextType = ContextType.CANVAS
			}
			if (!useCpu && renderingContext2d == null && webGLRenderingContext == null && webGL2RenderingContext == null) {
				surface?.setupContext()
			}
		}
		when (type) {
			"2d" -> {
				actualContextType = "2d"
				if (renderingContext2d == null) {
					renderingContext2d = TNSCanvasRenderingContext2D(this)
				}
				contextType = ContextType.CANVAS
				return renderingContext2d
			}
			"webgl", "experimental-webgl" -> {
				actualContextType = "webgl"
				if (webGLRenderingContext == null) {
					webGLRenderingContext = TNSWebGLRenderingContext(this)
				}
				contextType = ContextType.WEBGL
				return webGLRenderingContext
			}
			"webgl2" -> {
				if (webGL2RenderingContext == null) {
					if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2 && GLContext.DID_CHECK_WEBGL_SUPPORT && GLContext.IS_WEBGL_2_SUPPORT) {
						actualContextType = "webgl"
						webGL2RenderingContext = TNSWebGL2RenderingContext(this)
						isWebGL = true
						contextType = ContextType.WEBGL
					} else {
						isWebGL = false
						contextType = ContextType.NONE
						return null
					}
				}
				return webGL2RenderingContext
			}
		}
		contextType = ContextType.NONE
		return null
	}

	override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
		surface?.resize(w, h)
	}

	internal var isPaused = false
	override fun onDetachedFromWindow() {
		super.onDetachedFromWindow()
		isPaused = true
		Choreographer.getInstance().removeFrameCallback(this)
	}

	override fun onAttachedToWindow() {
		super.onAttachedToWindow()
		isPaused = false
		Choreographer.getInstance().postFrameCallback(this)
	}

	interface Listener {
		fun contextReady()
	}

	var listener: Listener? = null
		set(value) {
			field = value
			surface?.setListener(listener)
		}
	internal var isWebGL = false

	class Size(var width: Int, var height: Int)

	internal var lastSize: Size? = null
	internal var newSize: Size? = null
	fun flush() {
		if (useCpu) {
			cpuView?.flush()
		} else {
			surface?.flush()
		}
	}

	companion object {
		var views: ConcurrentHashMap<*, *> = ConcurrentHashMap<Any?, Any?>()

		@JvmStatic
		fun layoutView(width: Int, height: Int, tnsCanvas: TNSCanvas) {
			var rootParams = tnsCanvas.layoutParams

			if (rootParams != null && width == rootParams.width && height == rootParams.height) {
				return;
			}

			if (width != 0 && height != 0) {
				if (rootParams == null) {
					rootParams = LayoutParams(0, 0)
				}
				rootParams.width = width
				rootParams.height = height

				tnsCanvas.layoutParams = rootParams

				val w = MeasureSpec.makeMeasureSpec(width, MeasureSpec.EXACTLY)
				val h = MeasureSpec.makeMeasureSpec(height, MeasureSpec.EXACTLY)
				tnsCanvas.measure(w, h)
				tnsCanvas.layout(0, 0, width, height)
			}
		}

		@JvmStatic
		external fun nativeInitContext(
			width: Float,
			height: Float,
			density: Float,
			bufferId: Int,
			samples: Int,
			alpha: Boolean,
			fontColor: Int,
			ppi: Float,
			direction: Int
		): Long

		@JvmStatic
		external fun nativeInitContextWithCustomSurface(
			width: Float,
			height: Float,
			density: Float,
			alpha: Boolean,
			fontColor: Int,
			ppi: Float,
			direction: Int
		): Long

		@JvmStatic
		external fun nativeResizeCustomSurface(
			context: Long,
			width: Float,
			height: Float,
			density: Float,
			alpha: Boolean,
			ppi: Int,
		)

		@JvmStatic
		external fun nativeResizeSurface(
			context: Long,
			width: Float,
			height: Float,
			density: Float,
			bufferId: Int,
			samples: Int,
			alpha: Boolean,
			ppi: Float
		)

		@JvmStatic
		private external fun nativeDestroyContext(context: Long)

		@JvmStatic
		external fun nativeFlush(context: Long)

		@JvmStatic
		external fun nativeCustomWithBitmapFlush(context: Long, view: Bitmap)

		@JvmStatic
		private external fun nativeDataURL(context: Long, type: String?, quality: Float): String?

		@JvmStatic
		private external fun nativeToData(context: Long): ByteArray?

		@JvmStatic
		private external fun nativeSnapshotCanvas(context: Long): ByteArray

		internal const val ONE_MILLISECOND_NS: Long = 1000000
		internal const val ONE_S_IN_NS = 1000 * ONE_MILLISECOND_NS
		internal var lastCall: Long = 0
		internal var isLibraryLoaded = false
		const val TAG = "CanvasView"

		@JvmStatic
		fun createSVGMatrix(): TNSDOMMatrix {
			return TNSDOMMatrix()
		}

		@JvmStatic
		internal val direction: TNSTextDirection
			get() {
				var direction = TNSTextDirection.Ltr
				if (TextUtilsCompat.getLayoutDirectionFromLocale(Locale.getDefault()) == ViewCompat.LAYOUT_DIRECTION_RTL) {
					direction = TNSTextDirection.Rtl
				}
				return direction
			}
	}
}
