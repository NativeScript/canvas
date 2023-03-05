package org.nativescript.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Color
import android.os.*
import android.util.AttributeSet
import android.view.Surface
import android.widget.FrameLayout
import androidx.core.text.TextUtilsCompat
import androidx.core.view.ViewCompat
import org.json.JSONObject
import java.nio.ByteBuffer
import java.util.*
import java.util.concurrent.ConcurrentHashMap


/**
 * Created by triniwiz on 3/29/20
 */
class NSCCanvas : FrameLayout {

	internal var nativeGL: Long = 0

	var nativeContext: Long = 0
		private set

	enum class SurfaceType {
		Texture,
		Surface
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
	private lateinit var textureView: GLView
	private lateinit var surfaceView: GLViewSV

	constructor(context: Context) : super(context, null) {
		init(context, SurfaceType.Texture)
	}

	constructor(context: Context, type: SurfaceType) : super(context, null) {
		init(context, type)
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init(context, SurfaceType.Texture)
		textureView = GLView(context, attrs)
		surfaceView = GLViewSV(context, attrs)
	}

	private fun init(context: Context, type: SurfaceType) {
		surfaceType = type
		setBackgroundColor(Color.TRANSPARENT)
	}

	val drawingBufferWidth: Int
		get() {
			if (surfaceType == SurfaceType.Surface) {
				return surfaceView.width
			}
			return textureView.width
		}

	val drawingBufferHeight: Int
		get() {
			if (surfaceType == SurfaceType.Surface) {
				return surfaceView.height
			}
			return textureView.height
		}

	@Synchronized
	@Throws(Throwable::class)
	protected fun finalize() {
		if (nativeGL != 0L) {
			nativeReleaseGL(nativeGL)
			nativeGL = 0
		}
		if (nativeContext != 0L) {
			nativeReleaseGLPointer(nativeContext)
			nativeContext = 0
		}
	}

	fun initContext(type: String) {
		initContext(type, null)
	}

	fun initContext(type: String, contextAttributes: String?) {

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
	}

	fun initContext(
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
		var version = -1
		var isCanvas = false
		when (type) {
			"2d" -> {
				version = 0
				isCanvas = true
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
		if (nativeGL == 0L) {
			val surface = if (surfaceType == SurfaceType.Surface) {
				surfaceView.holder.surface
			} else {
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
					isCanvas
				)
				nativeContext = nativeGetGLPointer(nativeGL)
			}
		}
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

	interface Listener {
		fun contextReady()
		fun surfaceResize(width: Int, height: Int)
	}

	var listener: Listener? = null

	companion object {
		var views: ConcurrentHashMap<*, *> = ConcurrentHashMap<Any?, Any?>()
		internal var isLibraryLoaded = false
		const val TAG = "CanvasView"

		init {
			loadLib()
		}

		@JvmStatic
		fun loadLib() {
			if (!isLibraryLoaded) {
				System.loadLibrary("canvasnativev8")
				isLibraryLoaded = true
			}
		}

		@JvmStatic
		var enableDebug = false

		@JvmStatic
		fun layoutView(width: Float, height: Float, NSCCanvas: NSCCanvas) {
			layoutView(width.toInt(), height.toInt(), NSCCanvas)
		}

		@JvmStatic
		fun layoutView(width: Int, height: Int, NSCCanvas: NSCCanvas) {
			var rootParams = NSCCanvas.layoutParams

			if (rootParams != null && width == rootParams.width && height == rootParams.height) {
				return;
			}

			if (width != 0 && height != 0) {
				if (rootParams == null) {
					rootParams = LayoutParams(0, 0)
				}
				rootParams.width = width
				rootParams.height = height

				NSCCanvas.layoutParams = rootParams

				val w = MeasureSpec.makeMeasureSpec(width, MeasureSpec.EXACTLY)
				val h = MeasureSpec.makeMeasureSpec(height, MeasureSpec.EXACTLY)
				NSCCanvas.measure(w, h)
				NSCCanvas.layout(0, 0, width, height)
			}
		}

		@JvmStatic
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
		external fun nativeUpdateGLSurface(
			surface: Surface,
			context: Long
		)

		@JvmStatic
		external fun nativeReleaseGL(
			context: Long
		)

		@JvmStatic
		external fun nativeGetGLPointer(
			context: Long
		): Long

		@JvmStatic
		external fun nativeReleaseGLPointer(
			gl: Long
		)

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
		external fun nativeCustomWithBitmapFlush(context: Long, view: Bitmap)

		@JvmStatic
		internal val direction: Int
			get() {
				var direction = 0
				if (TextUtilsCompat.getLayoutDirectionFromLocale(Locale.getDefault()) == ViewCompat.LAYOUT_DIRECTION_RTL) {
					direction = 1
				}
				return direction
			}
	}
}
