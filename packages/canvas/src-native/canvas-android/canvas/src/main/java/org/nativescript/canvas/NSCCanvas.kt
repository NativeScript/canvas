package org.nativescript.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Color
import android.graphics.Matrix
import android.opengl.EGL14
import android.os.*
import android.util.AttributeSet
import android.view.MotionEvent
import android.view.Surface
import android.view.View
import android.widget.FrameLayout
import androidx.core.text.TextUtilsCompat
import androidx.core.view.drawToBitmap
import dalvik.annotation.optimization.CriticalNative
import dalvik.annotation.optimization.FastNative
import org.json.JSONObject
import java.io.ByteArrayOutputStream
import java.nio.ByteBuffer
import java.nio.FloatBuffer
import java.util.*
import java.util.concurrent.ConcurrentHashMap
import kotlin.math.min


/**
 * Created by triniwiz on 3/29/20
 */
class NSCCanvas : FrameLayout {

	enum class Engine {
		None,
		CPU,
		GL,
		GPU
	}

	var fit = CanvasFit.FitX
		set(value) {
			field = value
			invalidate()
			//	scaleSurface()
		}


	override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
		super.onSizeChanged(w, h, oldw, oldh)
		resize()
	}

	private var engine = Engine.None
	var surfaceWidth: Int = 300
		set(value) {
			field = value
			layoutSurface(value, surfaceHeight, this)
		}

	var surfaceHeight: Int = 150
		set(value) {
			field = value
			layoutSurface(surfaceWidth, value, this)
		}


	var nativeContext: Long = 0
		private set

	val nativeContextString: String
		get() {
			return nativeContext.toString()
		}

	val surface: Surface?
		get() {
			return when (surfaceType) {
				SurfaceType.Texture -> textureView.surface
				SurfaceType.Surface -> {
					if (!surfaceView.isCreated && !surfaceView.isCreatedWithZeroSized) {
						return null
					}
					return surfaceView.holder.surface
				}
			}
		}

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

	private var surfaceType = SurfaceType.Texture
	lateinit var textureView: GLView
	lateinit var surfaceView: GLViewSV

	private var isAlpha = false

	constructor(context: Context) : super(context, null) {
		init(context, null, surfaceType)
	}

	constructor(context: Context, type: SurfaceType) : super(context, null) {
		init(context, null, type)
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init(context, attrs, surfaceType)
	}

	private fun init(context: Context, attrs: AttributeSet?, type: SurfaceType) {
		clipChildren = false
		textureView = GLView(context, attrs)
		textureView.canvas = this
		surfaceView = GLViewSV(context, attrs)
		surfaceView.canvas = this
		surfaceType = type
		setBackgroundColor(Color.TRANSPARENT)
		val internalWidth = 300
		val internalHeight = 150
		when (surfaceType) {
			SurfaceType.Texture -> {
				addView(
					textureView,
					internalWidth,
					internalHeight
				)
			}

			SurfaceType.Surface -> {
				addView(
					surfaceView,
					internalWidth,
					internalHeight
				)
			}
		}
	}

	val drawingBufferWidth: Int
		get() {
			return surfaceWidth
		}

	val drawingBufferHeight: Int
		get() {
			return surfaceHeight
		}

	internal fun surfaceDestroyed() {
		listener?.surfaceDestroyed()
		if (engine == Engine.GL && nativeContext != 0L) {
			makeContextCurrent()
			val display = EGL14.eglGetCurrentDisplay()
			EGL14.eglMakeCurrent(
				display,
				EGL14.EGL_NO_SURFACE,
				EGL14.EGL_NO_SURFACE,
				EGL14.EGL_NO_CONTEXT
			)
		}
	}

	@Synchronized
	@Throws(Throwable::class)
	protected fun finalize() {
		when (engine) {
			Engine.None -> {}
			Engine.CPU -> {}
			Engine.GL -> {
				if (nativeContext != 0L) {
					nativeReleaseWebGL(nativeContext)
					nativeContext = 0
				}
			}

			Engine.GPU -> {
			}
		}
		textureView.surfaceTexture?.release()
	}

	fun initWebGPUContext(instance: Long) {
		if (engine != Engine.None) {
			return
		}
		surface?.let {
			nativeContext = nativeInitWebGPU(instance, it, surfaceWidth, surfaceHeight)
		}
		if (nativeContext != 0L) {
			engine = Engine.GPU
		}

	}

	@JvmOverloads
	fun initContext(
		type: String,
		alpha: Boolean = true,
		antialias: Boolean = true,
		depth: Boolean = true,
		failIfMajorPerformanceCaveat: Boolean = false,
		powerPreference: Int = 0,
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
		if (nativeContext != 0L) {
			return
		}
		var alpha = true

		var antialias = true

		var depth = true

		var failIfMajorPerformanceCaveat = false

		var powerPreference = 0

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
							powerPreference = when (value as? String ?: "default") {
								"default" -> 0
								"high-performance" -> 1
								"low-power" -> 2
								else -> -1
							}
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
		powerPreference: Int,
		premultipliedAlpha: Boolean,
		preserveDrawingBuffer: Boolean,
		stencil: Boolean,
		desynchronized: Boolean,
		xrCompatible: Boolean
	) {
		if (nativeContext != 0L) {
			return
		}

		var version = -1
		when (type) {
			"2d" -> {
				version = 0
				is2D = true
			}

			"webgl" -> {
				version = 1
			}

			"experimental-webgl", "webgl2" -> {
				version = 2
			}
		}

		if (version == -1) {
			return
		}

		val surface = if (surfaceType == SurfaceType.Surface) {
			if (alpha) {
				surfaceView.setZOrderOnTop(true)
			} else {
				surfaceView.setZOrderOnTop(false)
			}
			if (!surfaceView.isCreated) {
				null
			} else {
				surfaceView.holder.surface
			}
		} else {
			textureView.isOpaque = !alpha
			textureView.surface
		}

		surface?.let {
			if (is2D) {
				val density = resources.displayMetrics.density
				if (!forceGL && Build.VERSION.SDK_INT >= Build.VERSION_CODES.N && vulkanVersion[0] > 1 && vulkanVersion[2] > 0) {
					nativeContext = nativeCreate2dContextVulkan(
						surfaceWidth,
						surfaceHeight,
						it,
						alpha,
						density,
						Color.BLACK,
						resources.displayMetrics.densityDpi.toFloat(),
						direction
					)
					engine = Engine.GPU
				} else {
					nativeContext = nativeCreate2DContext(
						surfaceWidth,
						surfaceHeight,
						it,
						alpha,
						density,
						Color.BLACK,
						resources.displayMetrics.densityDpi.toFloat(),
						direction
					)
					engine = Engine.GL
				}

			} else {
				nativeContext = nativeInitWebGL(
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
					version
				)
				engine = Engine.GL
			}

		} ?: run {
			if (surfaceWidth == 0 || surfaceHeight == 0) {
				var width = surfaceWidth
				var height = surfaceHeight

				if (width == 0) {
					width = 1
				}

				if (height == 0) {
					height = 1
				}
				if (is2D) {
					val density = resources.displayMetrics.density

					if (!forceGL && Build.VERSION.SDK_INT >= Build.VERSION_CODES.N && vulkanVersion[0] > 1 && vulkanVersion[2] > 0) {
						nativeContext = nativeCreate2dContextVulkan(
							width,
							height,
							null,
							alpha,
							density,
							Color.BLACK,
							resources.displayMetrics.densityDpi.toFloat(),
							direction
						)
						engine = Engine.GPU
					} else {
						nativeContext = nativeCreate2DContext(
							surfaceWidth,
							surfaceHeight,
							null,
							alpha,
							density,
							Color.BLACK,
							resources.displayMetrics.densityDpi.toFloat(),
							direction
						)
						engine = Engine.GL
					}
				} else {
					nativeContext = nativeInitWebGLNoSurface(
						width,
						height,
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
						version
					)
					engine = Engine.GL
				}

			} else {
				if (is2D) {
					val density = resources.displayMetrics.density
					if (!forceGL && Build.VERSION.SDK_INT >= Build.VERSION_CODES.N && vulkanVersion[0] > 1 && vulkanVersion[2] > 0) {
						nativeContext = nativeCreate2dContextVulkan(
							surfaceWidth,
							surfaceHeight,
							null,
							alpha,
							density,
							Color.BLACK,
							resources.displayMetrics.densityDpi.toFloat(),
							direction
						)
						engine = Engine.GPU
					} else {
						nativeContext = nativeCreate2DContext(
							surfaceWidth,
							surfaceHeight,
							null,
							alpha,
							density,
							Color.BLACK,
							resources.displayMetrics.densityDpi.toFloat(),
							direction
						)
						engine = Engine.GL
					}
				} else {
					nativeContext = nativeInitWebGLNoSurface(
						surfaceWidth,
						surfaceHeight,
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
						version
					)
					engine = Engine.GL
				}
			}
		}


		this.isAlpha = alpha
	}

	private var is2D = false

	fun create2DContext(
		alpha: Boolean,
		antialias: Boolean,
		depth: Boolean,
		failIfMajorPerformanceCaveat: Boolean,
		powerPreference: Int,
		premultipliedAlpha: Boolean,
		preserveDrawingBuffer: Boolean,
		stencil: Boolean,
		desynchronized: Boolean,
		xrCompatible: Boolean
	): Long {

		if (nativeContext != 0L) {
			return nativeContext
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

		return nativeContext
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

	private fun scaleSurface() {
		val frameWidth: Int = width
		val frameHeight: Int = height

		var scaleX: Float = (frameWidth.toFloat() / surfaceWidth.toFloat())
		var scaleY: Float = (frameHeight.toFloat() / surfaceHeight.toFloat())

		var newWidth = 0
		var newHeight = 0

		var dx = 0F
		var dy = 0F

		when (fit) {
			CanvasFit.None -> {
				newWidth = surfaceWidth
				newHeight = surfaceHeight
				scaleX = 1.0f
				scaleY = 1.0f
			}

			CanvasFit.Fill -> {
				newWidth = frameWidth
				newHeight = frameHeight
			}

			CanvasFit.FitX -> {
				newWidth = frameWidth
				newHeight = (surfaceHeight * scaleX).toInt()
				scaleY = scaleX
			}

			CanvasFit.FitY -> {
				newWidth = (surfaceWidth * scaleY).toInt()
				newHeight = frameHeight
				scaleX = scaleY
			}

			CanvasFit.ScaleDown -> {
				val scale = min(min(scaleX, scaleY), 1.0f)
				newWidth = (surfaceWidth * scale).toInt()
				newHeight = (surfaceHeight * scale).toInt()
				scaleX = scale
				scaleY = scale
			}
		}

		if (surfaceType == SurfaceType.Surface) {
			dx = (newWidth - surfaceWidth) / 2f
			dy = (newHeight - surfaceHeight) / 2f
			surfaceView.scaleX = scaleX
			surfaceView.scaleY = scaleY
			surfaceView.translationX = dx
			surfaceView.translationY = dy
		} else {
			val matrix = Matrix()
			matrix.preScale(scaleX, scaleY)
			matrix.postTranslate(dx, dy)
			textureView.setTransform(matrix)
		}
	}

	internal fun resize() {
		scaleSurface()
		if (nativeContext != 0L) {
			when (engine) {
				Engine.GL -> {
					surface?.let {
						if (is2D) {
							nativeUpdate2DSurface(
								it, surfaceWidth,
								surfaceHeight, nativeContext
							)
						} else {
							nativeUpdateWebGLSurface(it, nativeContext)
						}
					} ?: run {
						if (is2D) {
							//	GLES20.glViewport(0, 0, surfaceWidth, surfaceHeight)
							nativeUpdate2DSurfaceNoSurface(
								surfaceWidth,
								surfaceHeight,
								nativeContext
							)
						} else {
							nativeUpdateWebGLNoSurface(surfaceWidth, surfaceHeight, nativeContext)
						}
					}
				}

				Engine.GPU -> {
					if (nativeContext != 0L) {
						surface?.let {
							if (is2D) {
								nativeUpdate2DSurface(
									it, surfaceWidth,
									surfaceHeight, nativeContext
								)
							} else {
								nativeResizeWebGPU(nativeContext, it, surfaceWidth, surfaceHeight)
							}
						}
					}
				}

				else -> {}
			}

		}
		listener?.surfaceResize(surfaceWidth, surfaceHeight)
	}

	fun forceResize() {
		resize()
	}

	interface Listener {
		fun contextReady()
		fun surfaceResize(width: Int, height: Int)
		fun surfaceDestroyed()
		fun surfaceCreated()
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
		nativeMakeWebGLCurrent(nativeContext)
	}

	@JvmOverloads
	fun snapshot(flip: Boolean = false): Bitmap {
		var bitmap: Bitmap? = null
		var needsToFlip = false
		if (is2D) {
			bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
			nativeCustomWithBitmapFlush(nativeContext, bitmap)
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
				nativeWriteCurrentWebGLContextToBitmap(nativeContext, bitmap)
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

	fun toDataURL(type: String, quality: Float): String {
		var bitmap: Bitmap? = null
		var format = Bitmap.CompressFormat.PNG
		var retType = "image/png"
		when (type) {
			"image/jpg", "image/jpeg" -> {
				format = Bitmap.CompressFormat.JPEG
				retType = type
			}

			"image/webp" -> {
				format = Bitmap.CompressFormat.WEBP
				retType = type
			}
		}

		if (surfaceType == SurfaceType.Texture) {
			bitmap = textureView.bitmap
		}
		val bm = bitmap ?: Bitmap.createBitmap(surfaceWidth, surfaceHeight, Bitmap.Config.ARGB_8888)
		val byteArrayOutputStream = ByteArrayOutputStream()
		bm.compress(format, (quality * 100).toInt(), byteArrayOutputStream)
		val byteArray = byteArrayOutputStream.toByteArray()
		val encoded: String = android.util.Base64.encodeToString(byteArray, android.util.Base64.NO_WRAP)
		return "data:${retType};base64,${encoded}"
	}

	companion object {
		@JvmStatic
		var forceGL = true

		@JvmStatic
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

		internal var vulkanVersion: IntArray = intArrayOf(0, 0, 0)

		init {
			loadLib()
		}

		@JvmStatic
		fun loadLib() {
			if (!isLibraryLoaded) {
				try {
					System.loadLibrary("canvasnative")
					System.loadLibrary("canvasnativev8")
					isLibraryLoaded = true
					nativeGetVulkanVersion(vulkanVersion)
				} catch (_: Exception) {
				}
			}
		}

		@JvmStatic
		var enableDebug = false

		@JvmStatic
		fun layoutSurface(width: Float, height: Float, canvas: NSCCanvas) {
			layoutSurface(width.toInt(), height.toInt(), canvas)
		}

		@JvmStatic
		fun layoutSurface(width: Int, height: Int, canvas: NSCCanvas) {
			val view = when (canvas.surfaceType) {
				SurfaceType.Texture -> canvas.textureView
				SurfaceType.Surface -> {
					canvas.surfaceView
				}
			}

			var rootParams = view.layoutParams

			if (rootParams == null) {
				rootParams = LayoutParams(0, 0)
			}

			if (width == rootParams.width && height == rootParams.height) {
				return
			}

			rootParams.width = width
			rootParams.height = height

			if (rootParams.width <= 0) {
				rootParams.width = 1
			}

			if (rootParams.height <= 0) {
				rootParams.height = 1
			}

			val w = MeasureSpec.makeMeasureSpec(rootParams.width, MeasureSpec.EXACTLY)
			val h = MeasureSpec.makeMeasureSpec(rootParams.height, MeasureSpec.EXACTLY)

			view.measure(w, h)
			view.layout(0, 0, width, height)

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

		internal fun append(key: String, value: Float, sb: StringBuilder, isLast: Boolean = false) {
			sb.append("\"${key}\": ${value}${if (isLast) "" else ","}")
		}

		@JvmStatic
		external fun nativeGetVulkanVersion(array: IntArray)

		@JvmStatic
		@FastNative
		external fun nativeInitWebGPU(
			instance: Long,
			surface: Surface,
			width: Int,
			height: Int
		): Long

		@JvmStatic
		@FastNative
		external fun nativeResizeWebGPU(
			context: Long,
			surface: Surface,
			width: Int,
			height: Int
		)

		@JvmStatic
		@FastNative
		external fun nativeInitWebGL(
			surface: Surface?,
			alpha: Boolean,
			antialias: Boolean,
			depth: Boolean,
			failIfMajorPerformanceCaveat: Boolean,
			powerPreference: Int,
			premultipliedAlpha: Boolean,
			preserveDrawingBuffer: Boolean,
			stencil: Boolean,
			desynchronized: Boolean,
			xrCompatible: Boolean,
			version: Int
		): Long

		@JvmStatic
		@FastNative
		external fun nativeInitWebGLNoSurface(
			width: Int,
			height: Int,
			alpha: Boolean,
			antialias: Boolean,
			depth: Boolean,
			failIfMajorPerformanceCaveat: Boolean,
			powerPreference: Int,
			premultipliedAlpha: Boolean,
			preserveDrawingBuffer: Boolean,
			stencil: Boolean,
			desynchronized: Boolean,
			xrCompatible: Boolean,
			version: Int
		): Long

		@JvmStatic
		@FastNative
		external fun nativeCreate2DContext(
			width: Int,
			height: Int,
			surface: Surface?,
			alpha: Boolean,
			density: Float,
			fontColor: Int,
			ppi: Float,
			direction: Int
		): Long


		@JvmStatic
		@FastNative
		external fun nativeCreate2dContextVulkan(
			width: Int,
			height: Int,
			surface: Surface?,
			alpha: Boolean,
			density: Float,
			fontColor: Int,
			ppi: Float,
			direction: Int
		): Long

		@JvmStatic
		@FastNative
		external fun nativeUpdateWebGLSurface(
			surface: Surface,
			context: Long
		)

		@JvmStatic
		@FastNative
		external fun nativeUpdate2DSurface(
			surface: Surface,
			width: Int,
			height: Int,
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
		external fun nativeUpdateWebGLNoSurface(
			width: Int,
			height: Int,
			context: Long
		)

		@JvmStatic
		@CriticalNative
		external fun nativeReleaseWebGL(
			context: Long
		)

		@JvmStatic
		@CriticalNative
		external fun nativeMakeWebGLCurrent(
			context: Long
		): Boolean

		@JvmStatic
		@FastNative
		external fun nativeWriteCurrentWebGLContextToBitmap(context: Long, bitmap: Bitmap)

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
		external fun nativeContext2DConicTest(context: Long)

		@JvmStatic
		@CriticalNative
		external fun nativeContext2DRender(context: Long)

		@JvmStatic
		fun context2DTest(context: Long) {
			nativeContext2DTest(context)
		}

		@JvmStatic
		fun context2DPathTest(context: Long) {
			nativeContext2DPathTest(context)
		}

		@JvmStatic
		fun context2DRender(context: Long) {
			nativeContext2DRender(context)
		}

		@JvmStatic
		@FastNative
		external fun nativeWebGLC2DRender(context: Long, c2d: Long, internalFormat: Int, format: Int)


		@JvmStatic
		fun WebGLContextRender(gl: Long, context: Long, internalFormat: Int, format: Int) {
			nativeWebGLC2DRender(gl, context, internalFormat, format)
		}

		@JvmStatic
		val direction: Int
			get() {
				var direction = 0
				if (TextUtilsCompat.getLayoutDirectionFromLocale(Locale.getDefault()) == View.LAYOUT_DIRECTION_RTL) {
					direction = 1
				}
				return direction
			}
	}
}
