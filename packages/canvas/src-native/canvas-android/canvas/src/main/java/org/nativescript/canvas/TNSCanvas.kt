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
import android.view.Choreographer
import android.view.Choreographer.FrameCallback
import android.view.View
import android.view.ViewGroup
import android.widget.FrameLayout
import androidx.core.text.TextUtilsCompat
import androidx.core.view.ViewCompat
import org.json.JSONObject
import java.io.ByteArrayOutputStream
import java.lang.ref.WeakReference
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.util.*
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit


/**
 * Created by triniwiz on 3/29/20
 */
class TNSCanvas : FrameLayout {
	internal lateinit var renderer: GLRenderer


	val nativeContext: Long
		get() {
			return renderer.nativeContext
		}

	internal var invalidateState: Int
	get() {
		return renderer.invalidateState
	}

	set(value) {
		renderer.invalidateState = value
	}

	var isHandleInvalidationManually: Boolean
		set(value) {
			renderer.isHandleInvalidationManually = value
		}
		get() {
			return renderer.isHandleInvalidationManually
		}

	var ignorePixelScaling: Boolean
		set(value) {
			renderer.ignorePixelScaling = value
		}
		get() {
			return renderer.ignorePixelScaling
		}


	constructor(context: Context) : super(context, null) {
		init(context, false)
	}

	constructor(context: Context, softwareRenderer: Boolean) : super(context, null) {
		init(context, softwareRenderer)
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init(context, false)
	}

	private fun init(context: Context, softwareRenderer: Boolean) {
		setBackgroundColor(Color.TRANSPARENT)

		if (isInEditMode) {
			return
		}

		renderer = GLRenderer(this, context, softwareRenderer)

	}

	val drawingBufferWidth: Int
		get() {
			return renderer.drawingBufferWidth
		}
	val drawingBufferHeight: Int
		get() {
			return renderer.drawingBufferHeight
		}

	@Synchronized
	@Throws(Throwable::class)
	protected fun finalize() {
		if (renderer.nativeContext != 0L) {
			nativeDestroyContext(renderer.nativeContext)
			renderer.nativeContext = 0
		}
	}

	fun toData(): ByteArray? {
		return renderer.toData()
	}

	fun snapshot(): ByteBuffer {
		return renderer.snapshot()
	}

	@JvmOverloads
	fun toDataURL(type: String = "image/png", quality: Float = 0.92f): String? {
		return renderer.toDataURL(type, quality)
	}

	enum class ContextType {
		NONE, CANVAS, WEBGL;
		companion object {
			@JvmStatic
			fun fromString(value: String): ContextType {
				return when(value){
					"2d" -> CANVAS
					"experimental-webgl", "webgl",  "webgl2" -> WEBGL
					else -> NONE
				}
			}
		}
	}


	fun getContextAttributesJson(): String {
		return renderer.contextAttributes.toJSON()
	}

	fun getContext(type: String) {
		getContext(type, GLRenderer.ContextAttributes.default)
	}

	fun getContext(type: String, contextAttributes: String?){

		val attrs = contextAttributes?.let {
			try {
				val attrsJSON = JSONObject(contextAttributes)
				GLRenderer.ContextAttributes.fromJSON(attrsJSON)
			}catch (e: Exception) {
				GLRenderer.ContextAttributes.default
			}
		} ?: GLRenderer.ContextAttributes.default
		return renderer.getContext(type, attrs)
	}

	internal fun getContext(type: String, contextAttributes: GLRenderer.ContextAttributes) {
		return renderer.getContext(type, contextAttributes)
	}

	override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
		renderer.resize(w, h)
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

	var listener: Listener?
		set(value) {
			renderer.listener = value
		}
		get() {
			return renderer.listener
		}

	fun flush() {
		renderer.flush()
	}


	companion object {
		var views: ConcurrentHashMap<*, *> = ConcurrentHashMap<Any?, Any?>()
		internal var isLibraryLoaded = false
		const val TAG = "CanvasView"

		init {
			loadLib()
		}

		@JvmStatic
		fun loadLib(){
			if (!isLibraryLoaded) {
				System.loadLibrary("canvasnativev8")
				isLibraryLoaded = true
			}
		}

		@JvmStatic
		var enableDebug = false

		@JvmStatic
		fun layoutView(width: Float, height: Float, tnsCanvas: TNSCanvas) {
			layoutView(width.toInt(), height.toInt(), tnsCanvas)
		}

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
		external fun nativeDestroyContext(context: Long)

		@JvmStatic
		external fun nativeCustomWithBitmapFlush(context: Long, view: Bitmap)

		@JvmStatic
		internal fun nativeDataURLImpl(context: Long, type: String?, quality: Float): String? {
			return nativeDataURL(context, type, quality)
		}

		@JvmStatic
		private external fun nativeDataURL(context: Long, type: String?, quality: Float): String?

		@JvmStatic
		internal fun nativeToDataImpl(context: Long): ByteArray? {
			return nativeToData(context)
		}

		@JvmStatic
		private external fun nativeToData(context: Long): ByteArray?

		@JvmStatic
		internal fun nativeSnapshotCanvasImpl(context: Long): ByteBuffer {
			return nativeSnapshotCanvas(context)
		}

		@JvmStatic
		private external fun nativeSnapshotCanvas(context: Long): ByteBuffer


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
