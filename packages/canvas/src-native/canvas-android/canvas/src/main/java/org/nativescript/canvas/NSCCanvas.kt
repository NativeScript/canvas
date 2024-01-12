package org.nativescript.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Color
import android.graphics.Matrix
import android.os.*
import android.util.AttributeSet
import android.util.Log
import android.view.MotionEvent
import android.view.Surface
import android.view.ViewGroup
import android.widget.FrameLayout
import androidx.core.text.TextUtilsCompat
import androidx.core.view.ViewCompat
import androidx.core.view.drawToBitmap
import dalvik.annotation.optimization.CriticalNative
import dalvik.annotation.optimization.FastNative
import org.json.JSONObject
import java.nio.ByteBuffer
import java.nio.FloatBuffer
import java.nio.IntBuffer
import java.util.*
import java.util.concurrent.ConcurrentHashMap


/**
 * Created by triniwiz on 3/29/20
 */
class NSCCanvas : FrameLayout {

	var nativeGL: Long = 0
		private set

	var nativeContext: Long = 0
		private set

	private var native2DContext: Long = 0

	enum class SurfaceType {
		Texture,
		Surface
	}

	private val handler = NSCTouchHandler(this)

	interface TouchEvents {
		fun onEvent(event: String, motionEvent: MotionEvent)
	}

	var ignoreTouchEvents = false

	override fun onTouchEvent(event: MotionEvent): Boolean {
		if (ignoreTouchEvents) {
			return false
		}
		handler.handle(event)
		return true
	}

	var touchEventListener: TouchEvents? = null

	var boundsBuffer: FloatBuffer? = null

	private var didUpscale = false
	var upscale: Boolean = false
		set(value) {
			field = value
			didUpscale = true;
			updateParams(width, height)
		}

	var ignorePixelScaling: Boolean
		set(value) {
			if (surfaceType == SurfaceType.Surface) {
				surfaceView.ignorePixelScaling = value
			}
			textureView.ignorePixelScaling = value
		}
		get() {
			if (surfaceType == SurfaceType.Surface) {
				return surfaceView.ignorePixelScaling
			}
			return textureView.ignorePixelScaling
		}

	private var surfaceType = SurfaceType.Texture
	lateinit var textureView: GLView
	private lateinit var surfaceView: GLViewSV

	private var isAlpha = false

	constructor(context: Context) : super(context, null) {
		init(context, null, SurfaceType.Texture)
	}

	constructor(context: Context, type: SurfaceType) : super(context, null) {
		init(context, null, type)
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init(context, attrs, SurfaceType.Texture)
	}

	private fun init(context: Context, attrs: AttributeSet?, type: SurfaceType) {
		textureView = GLView(context, attrs)
		textureView.canvas = this
		surfaceView = GLViewSV(context, attrs)
		surfaceView.canvas = this
		surfaceType = type
		setBackgroundColor(Color.TRANSPARENT)
		when (surfaceType) {
			SurfaceType.Texture -> {
				addView(
					textureView,
					LayoutParams(
						ViewGroup.LayoutParams.MATCH_PARENT,
						ViewGroup.LayoutParams.MATCH_PARENT
					)
				)
			}

			SurfaceType.Surface -> {
				addView(
					surfaceView,
					LayoutParams(
						ViewGroup.LayoutParams.MATCH_PARENT,
						ViewGroup.LayoutParams.MATCH_PARENT
					)
				)
			}
		}
	}


	val drawingBufferWidth: Int
		get() {
			return width
		}

	val drawingBufferHeight: Int
		get() {
			return height
		}

	@Synchronized
	@Throws(Throwable::class)
	protected fun finalize() {
		if (nativeContext != 0L) {
			nativeReleaseGLPointer(nativeContext)
			nativeContext = 0
		}
		if (nativeGL != 0L) {
			nativeReleaseGL(nativeGL)
			nativeGL = 0
		}
	}

	@JvmOverloads
	fun initContext(
		type: String,
		alpha: Boolean = true,
		antialias: Boolean = true,
		depth: Boolean = true,
		failIfMajorPerformanceCaveat: Boolean = false,
		powerPreference: String = "default",
		premultipliedAlpha: Boolean = true,
		preserveDrawingBuffer: Boolean = false,
		stencil: Boolean = false,
		desynchronized: Boolean = false,
		xrCompatible: Boolean = false
	) {
		initContextWithContextAttributes(
			type,
			alpha,
			antialias,
			depth,
			failIfMajorPerformanceCaveat,
			powerPreference,
			premultipliedAlpha,
			preserveDrawingBuffer,
			stencil,
			desynchronized,
			xrCompatible
		)
	}

	fun initContextWithJsonString(type: String, contextAttributes: String?) {
		if (nativeGL != 0L) {
			return
		}
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

		contextAttributes?.let {
			try {
				val attr = JSONObject(it)
				attr.keys().forEach { key ->
					val value = attr[key]
					when (key) {
						"alpha" -> {
							alpha = (value as? Boolean) ?: true
						}

						"antialias" -> {
							antialias = value as? Boolean ?: true
						}

						"depth" -> {
							depth = value as? Boolean ?: true
						}

						"failIfMajorPerformanceCaveat" -> {
							failIfMajorPerformanceCaveat = value as? Boolean ?: false
						}

						"premultipliedAlpha" -> {
							premultipliedAlpha = value as? Boolean ?: true
						}

						"preserveDrawingBuffer" -> {
							preserveDrawingBuffer = value as? Boolean ?: false
						}

						"stencil" -> {
							stencil = value as? Boolean ?: false
						}

						"xrCompatible" -> {
							xrCompatible = value as? Boolean ?: false
						}

						"desynchronized" -> {
							desynchronized = value as? Boolean ?: false
						}

						"powerPreference" -> {
							powerPreference = value as? String ?: "default"
						}

						else -> {}
					}
				}
			} catch (_: Exception) {
			}
		}

		initContext(
			type,
			alpha,
			antialias,
			depth,
			failIfMajorPerformanceCaveat,
			powerPreference,
			premultipliedAlpha,
			preserveDrawingBuffer,
			stencil,
			desynchronized,
			xrCompatible
		)

		this.isAlpha = alpha
	}

	fun initContextWithContextAttributes(
		type: String,
		alpha: Boolean,
		antialias: Boolean,
		depth: Boolean,
		failIfMajorPerformanceCaveat: Boolean,
		powerPreference: String,
		premultipliedAlpha: Boolean,
		preserveDrawingBuffer: Boolean,
		stencil: Boolean,
		desynchronized: Boolean,
		xrCompatible: Boolean
	) {
		if (nativeGL != 0L) {
			return
		}
		var version = -1
		when (type) {
			"2d" -> {
				version = 0
				is2D = true
			}

			"experimental-webgl", "webgl" -> {
				version = 1
			}

			"webgl2" -> {
				version = 2
			}
		}

		if (version == -1) {
			return
		}

		val surface = if (surfaceType == SurfaceType.Surface) {
			surfaceView.holder.surface
		} else {
			textureView.isOpaque = !alpha
			textureView.surface
		}

		surface?.let {
			nativeGL = nativeInitGL(
				it,
				alpha,
				antialias,
				depth,
				failIfMajorPerformanceCaveat,
				powerPreference,
				premultipliedAlpha,
				preserveDrawingBuffer,
				stencil,
				desynchronized,
				xrCompatible,
				version,
				is2D
			)

			nativeContext = nativeGetGLPointer(nativeGL)

		} ?: run {
			nativeGL = nativeInitGLNoSurface(
				this.drawingBufferWidth,
				this.drawingBufferHeight,
				alpha,
				antialias,
				depth,
				failIfMajorPerformanceCaveat,
				powerPreference,
				premultipliedAlpha,
				preserveDrawingBuffer,
				stencil,
				desynchronized,
				xrCompatible,
				version,
				is2D
			)

			nativeContext = nativeGetGLPointer(nativeGL)
		}

		this.isAlpha = alpha
	}

	internal var is2D = false

	fun create2DContext(
		alpha: Boolean,
		antialias: Boolean,
		depth: Boolean,
		failIfMajorPerformanceCaveat: Boolean,
		powerPreference: String,
		premultipliedAlpha: Boolean,
		preserveDrawingBuffer: Boolean,
		stencil: Boolean,
		desynchronized: Boolean,
		xrCompatible: Boolean,
		fontColor: Int
	): Long {

		if (native2DContext != 0L) {
			return native2DContext
		}

		initContext(
			"2d",
			alpha,
			antialias,
			depth,
			failIfMajorPerformanceCaveat,
			powerPreference,
			premultipliedAlpha,
			preserveDrawingBuffer,
			stencil,
			desynchronized,
			xrCompatible
		)

		val density = if (ignorePixelScaling) {
			1F
		} else {
			resources.displayMetrics.density
		}

		val samples = if (antialias) {
			4
		} else {
			0
		}

		native2DContext = nativeCreate2DContext(
			nativeGL,
			this.drawingBufferWidth,
			this.drawingBufferHeight,
			alpha,
			density,
			samples,
			fontColor,
			(resources.displayMetrics.densityDpi * 160).toFloat(),
			direction
		)
		return native2DContext
	}

	internal var isPaused = false
	internal var isAttachedToWindow = false

	override fun onDetachedFromWindow() {
		super.onDetachedFromWindow()
		isPaused = true
		isAttachedToWindow = false
	}

	override fun onAttachedToWindow() {
		super.onAttachedToWindow()
		isPaused = false
		isAttachedToWindow = true
	}

	private fun updateParams(w: Int, h: Int) {
		if (!didUpscale) {
			return
		}
		if (upscale) {
			val density = resources.displayMetrics.density
			if (surfaceType == SurfaceType.Surface) {
				surfaceView.layoutParams = LayoutParams((w / density).toInt(), (h / density).toInt())
			} else {
				textureView.layoutParams = LayoutParams((w / density).toInt(), (h / density).toInt())
			}
			clipChildren = true
			clipToPadding = true
			ignorePixelScaling = true
		} else {
			if (surfaceType == SurfaceType.Surface) {
				surfaceView.layoutParams = LayoutParams(
					ViewGroup.LayoutParams.MATCH_PARENT,
					ViewGroup.LayoutParams.MATCH_PARENT
				)
			} else {
				textureView.layoutParams = LayoutParams(
					ViewGroup.LayoutParams.MATCH_PARENT,
					ViewGroup.LayoutParams.MATCH_PARENT
				)
			}
			clipChildren = true
			clipToPadding = true
			ignorePixelScaling = false
			didUpscale = false
		}
	}

	override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
		super.onSizeChanged(w, h, oldw, oldh)
		updateParams(w, h)
		listener?.surfaceResize(w, h)
	}

	internal fun resize() {
		if (nativeGL != 0L) {
			val surface = if (surfaceType == SurfaceType.Surface) {
				surfaceView.holder.surface
			} else {
				textureView.surface
			}

			surface?.let {
				nativeUpdateGLSurface(it, nativeGL)
				if (is2D) {
					nativeUpdate2DSurface(it, native2DContext)
				}
			} ?: run {
				nativeUpdateGLNoSurface(this.drawingBufferWidth, this.drawingBufferHeight, nativeGL)
				if (is2D) {
					nativeUpdate2DSurfaceNoSurface(
						this.drawingBufferWidth,
						this.drawingBufferHeight,
						native2DContext
					)
				}
			}

		}
	}

	interface Listener {
		fun contextReady()
		fun surfaceResize(width: Int, height: Int)
	}

	var listener: Listener? = null


	private val defaultMatrix = Matrix()
	private val invertMatrix = Matrix()
	private val invertFlipMatrix = Matrix()

	init {
		defaultMatrix.postScale(-1f, 1f)
		invertMatrix.postScale(1f, -1f)
		invertFlipMatrix.postScale(-1f, -1f)
	}


	fun makeContextCurrent() {
		nativeMakeGLCurrent(nativeGL)
	}


	@JvmOverloads
	fun snapshot(flip: Boolean = false): Bitmap? {
		var bitmap: Bitmap? = null
		var needsToFlip = false
		if (is2D) {
			bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
			nativeCustomWithBitmapFlush(native2DContext, bitmap!!)
			return bitmap
		} else {
			if (surfaceType == SurfaceType.Surface) {
				try {
					bitmap = surfaceView.drawToBitmap()
				} catch (_: Exception) {
				}
			} else {
				bitmap = textureView.getBitmap(width, height)
			}


			if (bitmap == null) {
				needsToFlip = true
				bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
				nativeWriteCurrentGLContextToBitmap(nativeGL, bitmap!!)
			}
		}

		if (needsToFlip) {
			bitmap = if (flip) {
				Bitmap.createBitmap(bitmap, 0, 0, width, height, invertFlipMatrix, true)
			} else {
				Bitmap.createBitmap(bitmap, 0, 0, width, height, invertMatrix, true)
			}
		} else if (flip) {
			bitmap = Bitmap.createBitmap(bitmap, 0, 0, width, height, defaultMatrix, true)
		}

		return bitmap
	}

	companion object {
		var views = ConcurrentHashMap<Any?, Any?>()

		@JvmStatic
		var store = ConcurrentHashMap<Any?, Any?>()

		@JvmStatic
		fun getBuffer(key: String): ByteBuffer? {
			return store[key] as? ByteBuffer
		}

		@JvmStatic
		fun storeBuffer(key: String, buffer: ByteBuffer) {
			store[key] = buffer
		}

		@JvmStatic
		fun storeBuffers(key: String, buffer: Array<ByteBuffer>) {
			store[key] = buffer
		}

		@JvmStatic
		fun removeBuffer(key: String) {
			store.remove(key)
		}

		@JvmStatic
		fun getBuffer(key: Int): ByteBuffer? {
			return store[key] as? ByteBuffer
		}

		@JvmStatic
		fun getBuffers(key: Int): Array<*>? {
			return store[key] as? Array<*>
		}

		@JvmStatic
		fun storeBuffer(key: Int, buffer: ByteBuffer) {
			store[key] = buffer
		}

		@JvmStatic
		fun removeBuffer(key: Int) {
			store.remove(key)
		}

		internal var isLibraryLoaded = false
		const val TAG = "CanvasView"

		init {
			loadLib()
		}

		@JvmStatic
		fun loadLib() {
			if (!isLibraryLoaded) {
				System.loadLibrary("canvasnative")
				System.loadLibrary("canvasnativev8")
				isLibraryLoaded = true
			}
		}


		@JvmStatic
		var enableDebug = false

		@JvmStatic
		fun layoutView(width: Float, height: Float, canvas: NSCCanvas) {
			layoutView(width.toInt(), height.toInt(), canvas)
		}

		@JvmStatic
		fun layoutView(width: Int, height: Int, canvas: NSCCanvas) {
			var rootParams = canvas.layoutParams

			if (rootParams != null && width == rootParams.width && height == rootParams.height) {
				return
			}

			if (width != 0 && height != 0) {
				if (rootParams == null) {
					rootParams = LayoutParams(0, 0)
				}
				rootParams.width = width
				rootParams.height = height

				canvas.layoutParams = rootParams

				val w = MeasureSpec.makeMeasureSpec(width, MeasureSpec.EXACTLY)
				val h = MeasureSpec.makeMeasureSpec(height, MeasureSpec.EXACTLY)
				canvas.measure(w, h)
				canvas.layout(0, 0, width, height)
			}
		}

		@JvmStatic
		fun getBoundingClientRect(canvas: NSCCanvas) {
			canvas.boundsBuffer?.let { buffer ->
				val density = canvas.context.resources.displayMetrics.density
				val densityInverse = 1.0f / density
				buffer.put(0, canvas.top * densityInverse)
				buffer.put(1, canvas.right * densityInverse)
				buffer.put(2, canvas.bottom * densityInverse)
				buffer.put(3, canvas.left * densityInverse)
				buffer.put(4, canvas.width * densityInverse)
				buffer.put(5, canvas.height * densityInverse)
				buffer.put(6, canvas.x * densityInverse)
				buffer.put(7, canvas.y * densityInverse)
			}
		}

		@JvmStatic
		fun getBoundingClientRectJSON(canvas: NSCCanvas): String {
			val density = canvas.context.resources.displayMetrics.density

			val sb = StringBuilder()
			sb.append("{")
			val densityInverse = 1.0f / density

			append("top", canvas.top * densityInverse, sb)
			append("right", canvas.right * densityInverse, sb)
			append("bottom", canvas.bottom * densityInverse, sb)
			append("left", canvas.left * densityInverse, sb)
			append("width", canvas.width * densityInverse, sb)
			append("height", canvas.height * densityInverse, sb)
			append("x", canvas.x * densityInverse, sb)
			append("y", canvas.y * densityInverse, sb, true)

			sb.append("}")

			return sb.toString()
		}

		private fun append(key: String, value: Float, sb: StringBuilder, isLast: Boolean = false) {
			sb.append("\"${key}\": ${value}${if (isLast) "" else ","}")
		}

		@JvmStatic
		@FastNative
		external fun nativeInitGL(
			surface: Surface,
			alpha: Boolean,
			antialias: Boolean,
			depth: Boolean,
			failIfMajorPerformanceCaveat: Boolean,
			powerPreference: String,
			premultipliedAlpha: Boolean,
			preserveDrawingBuffer: Boolean,
			stencil: Boolean,
			desynchronized: Boolean,
			xrCompatible: Boolean,
			version: Int,
			isCanvas: Boolean,
		): Long

		@JvmStatic
		@FastNative
		external fun nativeInitGLNoSurface(
			width: Int,
			height: Int,
			alpha: Boolean,
			antialias: Boolean,
			depth: Boolean,
			failIfMajorPerformanceCaveat: Boolean,
			powerPreference: String,
			premultipliedAlpha: Boolean,
			preserveDrawingBuffer: Boolean,
			stencil: Boolean,
			desynchronized: Boolean,
			xrCompatible: Boolean,
			version: Int,
			isCanvas: Boolean,
		): Long


		@JvmStatic
		@CriticalNative
		external fun nativeCreate2DContext(
			context: Long,
			width: Int,
			height: Int,
			alpha: Boolean,
			density: Float,
			samples: Int,
			fontColor: Int,
			ppi: Float,
			direction: Int
		): Long

		@JvmStatic
		@FastNative
		external fun nativeUpdateGLSurface(
			surface: Surface,
			context: Long
		)

		@JvmStatic
		@FastNative
		external fun nativeUpdate2DSurface(
			surface: Surface,
			context: Long
		)

		@JvmStatic
		@CriticalNative
		external fun nativeUpdate2DSurfaceNoSurface(
			width: Int,
			height: Int,
			context: Long
		)


		@JvmStatic
		@CriticalNative
		external fun nativeUpdateGLNoSurface(
			width: Int,
			height: Int,
			context: Long
		)


		@JvmStatic
		@CriticalNative
		external fun nativeReleaseGL(
			context: Long
		)

		@JvmStatic
		@CriticalNative
		external fun nativeMakeGLCurrent(
			context: Long
		): Boolean

		@JvmStatic
		@CriticalNative
		external fun nativeGLPointerRefCount(
			context: Long
		): Long

		@JvmStatic
		@CriticalNative
		external fun nativeGetGLPointer(
			context: Long
		): Long

		@JvmStatic
		@CriticalNative
		external fun nativeReleaseGLPointer(
			gl: Long
		)

		@JvmStatic
		@FastNative
		external fun nativeWriteCurrentGLContextToBitmap(context: Long, bitmap: Bitmap)

		@JvmStatic
		@CriticalNative
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
		@CriticalNative
		external fun nativeResizeCustomSurface(
			context: Long,
			width: Float,
			height: Float,
			density: Float,
			alpha: Boolean,
			ppi: Int,
		)


		@JvmStatic
		@FastNative
		external fun nativeCustomWithBitmapFlush(context: Long, view: Bitmap)


		@JvmStatic
		@CriticalNative
		external fun nativeContext2DTest(context: Long)

		@JvmStatic
		@CriticalNative
		external fun nativeContext2DPathTest(context: Long)

		@JvmStatic
		fun context2DTest(context: Long) {
			nativeContext2DTest(context)
		}

		@JvmStatic
		fun context2DPathTest(context: Long) {
			nativeContext2DPathTest(context)
		}


		@JvmStatic
		val direction: Int
			get() {
				var direction = 0
				if (TextUtilsCompat.getLayoutDirectionFromLocale(Locale.getDefault()) == ViewCompat.LAYOUT_DIRECTION_RTL) {
					direction = 1
				}
				return direction
			}
	}
}
