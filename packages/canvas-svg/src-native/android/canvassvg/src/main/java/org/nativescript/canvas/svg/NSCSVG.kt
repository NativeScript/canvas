package org.nativescript.canvas.svg

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Matrix
import android.os.Build
import android.util.AttributeSet
import android.util.Log
import android.view.Surface
import android.view.View
import android.widget.FrameLayout
import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader
import java.net.HttpURLConnection
import java.net.URL
import java.nio.ByteBuffer
import java.nio.charset.Charset
import java.nio.charset.StandardCharsets
import java.util.concurrent.Executors
import java.util.regex.Pattern
import kotlin.math.min
import androidx.core.graphics.createBitmap
import java.nio.ByteOrder


class NSCSVGData {
	internal var mData: ByteBuffer? = null
	internal var mWidth: Int = 0
	internal var mHeight: Int = 0
	internal var mSize: Int = 0
	internal var mBitmap: Bitmap? = null
	private var mDirty = false

	fun resize(width: Int, height: Int) {
		if (width > 0 && height > 0) {
			mData = null
			mBitmap = null
			val size = (width * height * 4)
			mData = ByteBuffer.allocateDirect(size)
			mData?.order(ByteOrder.nativeOrder())
			mSize = size
			this.mWidth = width
			this.mHeight = height
		} else {
			mData = null
			mBitmap = null
			this.mWidth = 0
			this.mHeight = 0
		}
	}

	val data: ByteBuffer?
		get() {
			return this.mData
		}

	/** Bitmap of exactly this size, reused when the size is unchanged. */
	fun ensureBitmap(width: Int, height: Int): Bitmap? {
		if (width <= 0 || height <= 0) {
			return null
		}
		if (mBitmap == null || mWidth != width || mHeight != height) {
			mWidth = width
			mHeight = height
			mBitmap = createBitmap(width, height)
		}
		// Pixels come straight from the renderer, so there is nothing to copy in.
		mData = null
		mDirty = false
		return mBitmap
	}

	fun setPixels(buffer: ByteBuffer, width: Int, height: Int) {
		mData = buffer
		mSize = buffer.remaining()
		// Keep the bitmap unless the size changed. Reallocating it every frame is what
		// makes an animated svg churn the heap.
		if (mWidth != width || mHeight != height) {
			mBitmap = null
			mWidth = width
			mHeight = height
		}
		mDirty = true
	}

	val width: Int
		get() {
			return this.mWidth
		}

	val height: Int
		get() {
			return this.mHeight
		}

	fun toImage(): Bitmap? {
		if (mWidth <= 0 || mHeight <= 0) {
			return mBitmap
		}
		val data = mData ?: return mBitmap
		if (mBitmap == null) {
			mBitmap = createBitmap(mWidth, mHeight)
			mDirty = true
		}
		if (mDirty) {
			data.rewind()
			mBitmap?.copyPixelsFromBuffer(data)
			data.rewind()
			mDirty = false
		}
		return mBitmap
	}
}


/**
 * Hosts the SVG surface. A [FrameLayout] rather than a plain `View` so the GPU path can add a
 * TextureView or SurfaceView child at runtime. NativeScript sets properties in an order that
 * makes the surface type unknowable at construction time, so both have to remain possible.
 * With the GPU off, nothing is added and the layout draws the bitmap itself.
 */
class NSCSVG : FrameLayout {
	enum class SurfaceType { Texture, Surface }

	/** Mirrors `canvas_svg_c::gpu::Backend`. */
	enum class Backend(val value: Int) { Auto(0), Gl(1), Vulkan(2), Metal(3) }

	/**
	 * Told when the GPU context dies and when one comes back. A context outlives neither a
	 * driver reset nor the GPU being reclaimed, and the view keeps drawing either way; this
	 * exists so the JS side can say which path it is on.
	 */
	interface ContextListener {
		fun onContextLost()
		fun onContextRestored()
	}

	var contextListener: ContextListener? = null

	var data: NSCSVGData? = null
		internal set
	internal val lock = Any()
	private var pendingInvalidate: Boolean = false
	private val executor = Executors.newSingleThreadExecutor()
	internal var src: String = ""
	internal var srcPath: String = ""
	private var mMatrix = Matrix()
	internal var isPreloaded = false

	var sync: Boolean = false

	// -----------------------------------------------------------------------
	// GPU
	// -----------------------------------------------------------------------

	private var gpuContext: Long = 0
	private var renderThread: Long = 0
	private var hostView: View? = null

	/** Rasterize on the shared render thread so a heavy document doesn't block the UI. */
	var threaded: Boolean = true
		set(value) {
			if (field == value) return
			field = value
			applySurfaceMode()
		}

	/** The live surface, kept so a context that dies mid-frame can be rebuilt against it. */
	private var surface: Surface? = null
	private var surfaceWidth: Int = 0
	private var surfaceHeight: Int = 0

	/** Last frame's arguments, replayed once the surface becomes available. */
	private var pendingDocument: Long = 0
	private var pendingScale: Float = 1f
	private var pendingWidth: Int = 0
	private var pendingHeight: Int = 0

	var gpu: Boolean = true
		set(value) {
			if (field == value) return
			field = value
			applySurfaceMode()
		}

	var surfaceType: SurfaceType = SurfaceType.Texture
		set(value) {
			if (field == value) return
			field = value
			applySurfaceMode()
		}

	/** Forces a rasterizer. [Backend.Auto] tries Vulkan, then GL, then falls back to the bitmap. */
	var backend: Backend = Backend.Auto
		set(value) {
			if (field == value) return
			field = value
			applySurfaceMode()
		}

	/** Which backend is actually running, once a surface exists. */
	val activeBackend: Backend
		get() = when {
			gpuContext != 0L ->
				Backend.entries.firstOrNull { it.value == nativeGpuBackend(gpuContext) } ?: Backend.Auto
			// The threaded renderer keeps its context on its own thread and does not hand the
			// handle out; what it resolved `auto` to is not observable from here.
			renderThread != 0L -> backend
			else -> Backend.Auto
		}

	val isGpuActive: Boolean
		get() = gpuContext != 0L || renderThread != 0L

	constructor(context: Context) : super(context, null) {
		init(context)
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init(context)
	}

	private fun init(context: Context) {
		if (isInEditMode) {
			return
		}
		data = NSCSVGData()
		clipChildren = false
		setBackgroundColor(Color.TRANSPARENT)
		// A FrameLayout skips onDraw by default, and the bitmap path needs it.
		setWillNotDraw(false)
		applySurfaceMode()
	}

	/** Adds, swaps or removes the surface host to match [gpu]/[surfaceType]. */
	private fun applySurfaceMode() {
		if (isInEditMode) {
			return
		}
		destroyGpuContext()
		hostView?.let { removeView(it) }
		hostView = null

		if (!gpu) {
			invalidate()
			return
		}

		hostView = when (surfaceType) {
			SurfaceType.Texture -> SVGTextureView(context).also { it.svg = this }
			SurfaceType.Surface -> SVGSurfaceView(context).also { it.svg = this }
		}
		addView(hostView, LayoutParams.MATCH_PARENT, LayoutParams.MATCH_PARENT)
	}

	internal fun onSurfaceReady(surface: Surface?, width: Int, height: Int) {
		this.surface = surface
		surfaceWidth = width
		surfaceHeight = height
		if (surface == null || gpuContext != 0L || renderThread != 0L) {
			if (gpuContext != 0L || renderThread != 0L) onSurfaceResized(width, height)
			return
		}
		if (!createGpuContext()) {
			// No usable context: drop back to the bitmap, which always works.
			gpu = false
			return
		}
		// The document was very likely already rendered before the surface existed.
		replayLastFrame()
	}

	internal fun onSurfaceResized(width: Int, height: Int) {
		surfaceWidth = width
		surfaceHeight = height
		if (gpuContext == 0L && renderThread == 0L) {
			return
		}
		if (renderThread != 0L) {
			nativeRenderThreadResize(renderThread, width, height)
		} else {
			nativeGpuResize(gpuContext, width, height)
		}
		replayLastFrame()
	}

	/** Throws the GPU context away so the next frame exercises recovery. For testing. */
	fun debugLoseContext() {
		if (gpuContext != 0L) {
			nativeGpuDebugLoseContext(gpuContext)
		}
	}

	internal fun onSurfaceLost() {
		destroyGpuContext()
		surface = null
		surfaceWidth = 0
		surfaceHeight = 0
	}

	private fun createGpuContext(): Boolean {
		val created = createGpuContextImpl()
		// Otherwise Android keeps replaying the last onDraw, bitmap included, under the GPU frames.
		if (created) invalidate()
		return created
	}

	private fun createGpuContextImpl(): Boolean {
		val surface = this.surface ?: return false
		if (surfaceWidth <= 0 || surfaceHeight <= 0) {
			return false
		}
		if (threaded) {
			renderThread = nativeRenderThreadCreate(surface, surfaceWidth, surfaceHeight, backend.value)
			if (renderThread != 0L) {
				return true
			}
			// Starting the thread failed; the single-threaded context still might not.
		}
		gpuContext = nativeGpuCreate(surface, surfaceWidth, surfaceHeight, backend.value)
		return gpuContext != 0L
	}

	private fun replayLastFrame() {
		if (pendingDocument == 0L) {
			return
		}
		renderFrame(pendingDocument, pendingScale, pendingWidth, pendingHeight)
	}

	/**
	 * Draws one frame on whichever path is available, and deals with a context that died doing
	 * it. Native already rebuilds a lost context a few times itself and only reports
	 * [STATUS_LOST] once it has given up, but it rebuilds against the same window, so one
	 * more attempt through the surface is worth making before the view gives up on the GPU.
	 */
	private fun renderFrame(document: Long, scale: Float, width: Int, height: Int) {
		if (renderThread != 0L) {
			// Paint into a display list here and hand it over; the rasterizing happens on the
			// render thread, so this returns without waiting for it.
			nativeRenderThreadCommit(renderThread, document, width, height, scale)
			// The renderer reports asynchronously, so loss surfaces a frame or two late. That is
			// fine, since it only decides which path JS is told it is on.
			when (nativeRenderThreadStatus(renderThread)) {
				STATUS_RECOVERED -> contextListener?.onContextRestored()
				STATUS_LOST -> {
					Log.w("NSCSVG", "gpu render thread lost its context")
					destroyGpuContext()
					contextListener?.onContextLost()
					if (!createGpuContext()) {
						Log.w("NSCSVG", "gpu unusable, falling back to the cpu raster")
						gpu = false
						renderToBitmap(document, scale, width, height)
					}
				}
			}
			return
		}
		if (gpuContext != 0L) {
			val status = nativeGpuRender(gpuContext, document, scale)

			if (status == STATUS_RECOVERED) {
				contextListener?.onContextRestored()
			}
			if (status != STATUS_LOST) {
				return
			}

			Log.w("NSCSVG", "gpu context lost")
			destroyGpuContext()
			contextListener?.onContextLost()

			if (createGpuContext() && nativeGpuRender(gpuContext, document, scale) != STATUS_LOST) {
				contextListener?.onContextRestored()
				return
			}
			// Out of options. The bitmap always draws, so the view keeps working; turning `gpu`
			// off also drops the now-useless surface host and is visible from JS.
			Log.w("NSCSVG", "gpu unusable, falling back to the cpu raster")
			gpu = false
		}
		// Keyed off the context, not the `gpu` flag: until a surface actually exists there is
		// nothing to draw into, and the view would otherwise sit blank.
		renderToBitmap(document, scale, width, height)
	}

	private fun destroyGpuContext() {
		if (renderThread != 0L) {
			// Blocks until the thread has joined and released its surface.
			nativeRenderThreadDestroy(renderThread)
			renderThread = 0
		}
		if (gpuContext == 0L) {
			return
		}
		nativeGpuDestroy(gpuContext)
		gpuContext = 0
	}

	override fun onDetachedFromWindow() {
		super.onDetachedFromWindow()
		destroyGpuContext()
	}

	private var currentTask: java.util.concurrent.Future<*>? = null
	private fun resize(w: Int, h: Int) {
		doDraw()
	}

	override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
		super.onSizeChanged(w, h, oldw, oldh)

		if ((w != 0 && h != 0) && w != layoutParams.width && h != layoutParams.height) {

			data?.resize(w, h)

			resize(w, h)
		}
	}

	fun loadBuffer(buffer: ByteBuffer, width: Int, height: Int) {
		data?.setPixels(buffer, width, height)
		invalidate()
	}

	/**
	 * Renders a live document. On the GPU that draws straight into the window framebuffer; on
	 * the CPU it renders into the backing bitmap in place, so neither path stages the frame in
	 * an intermediate buffer (the copy an animated svg would otherwise pay every frame).
	 */
	fun renderDocument(document: Long, width: Int, height: Int, scale: Float) {
		if (document == 0L) {
			return
		}
		pendingDocument = document
		pendingScale = scale
		pendingWidth = width
		pendingHeight = height

		renderFrame(document, scale, width, height)
	}

	private fun renderToBitmap(document: Long, scale: Float, width: Int, height: Int) {
		data?.ensureBitmap(width, height)?.let { bitmap ->
			nativeRenderDocument(bitmap, document, scale)
			invalidate()
		}
	}

	override fun onDraw(canvas: Canvas) {
		if (isGpuActive) {
			return
		}
		data?.toImage()?.let {
			canvas.drawBitmap(it, mMatrix, null)
		}
	}

	/**
	 * The legacy one-shot rasterizer, for [src]/[srcPath]/preloaded data only.
	 *
	 * It must not run for a live document: it rasterizes on a background thread into the very
	 * bitmap [renderDocument] draws into on the main thread, and both lock the bitmap's pixels.
	 * Two threads locking the same bitmap is a hang, not a torn frame. It used to be dormant
	 * here by accident (with no src there was no bitmap for `toImage` to return) until the
	 * live path started creating one.
	 */
	private fun doDraw() {
		if (!isPreloaded && src.isEmpty() && srcPath.isEmpty()) {
			return
		}
		data?.toImage()?.let {
			currentTask?.cancel(true)
			if(isPreloaded){
				invalidate()
				return@let
			}
			if (sync) {
				if (srcPath.isNotEmpty()) {
					nativeDrawSVGFromPath(it, resources.displayMetrics.density, srcPath)
					pendingInvalidate = false
					invalidate()
				} else {
					nativeDrawSVG(it, resources.displayMetrics.density, src)
					pendingInvalidate = false
					invalidate()
				}
				return
			}

			synchronized(lock) {
				currentTask = executor.submit {
					if (srcPath.isNotEmpty()) {
						nativeDrawSVGFromPath(it, resources.displayMetrics.density, srcPath)
						handler?.post {
							pendingInvalidate = false
							invalidate()
						}
					} else {
						nativeDrawSVG(it, resources.displayMetrics.density, src)
						handler?.post {
							pendingInvalidate = false
							invalidate()
						}
					}
					currentTask = null
				}
			}
		}
	}

	fun setSrc(src: String) {
		this.src = src
		doDraw()
	}

	fun setSrcPath(path: String) {
		this.srcPath = path
		doDraw()
	}


	fun loadData(data: NSCSVGData){
		this.data = data
		this.src = ""
		this.srcPath = ""
		this.isPreloaded = true
		doDraw()
	}

	interface Callback {
		fun onSuccess(view: NSCSVGData?)
	}

	companion object {
		/** Mirrors `canvas_svg_c::gpu::FrameStatus`. */
		private const val STATUS_RECOVERED = 2
		private const val STATUS_LOST = 3

		init {
			try {
				System.loadLibrary("canvassvg")
			} catch (_: Exception) {
			}
		}

		@JvmStatic
		private external fun nativeRenderDocument(bitmap: Bitmap, document: Long, scale: Float)

		/** Returns 0 when no GPU context could be made, the cue to fall back to the bitmap. */
		@JvmStatic
		private external fun nativeGpuCreate(
			surface: Surface,
			width: Int,
			height: Int,
			backend: Int
		): Long

		@JvmStatic
		private external fun nativeGpuBackend(gpu: Long): Int

		@JvmStatic
		private external fun nativeGpuResize(gpu: Long, width: Int, height: Int)

		/** A `canvas_svg_c::gpu::FrameStatus`; see [STATUS_LOST]. */
		@JvmStatic
		private external fun nativeGpuRender(gpu: Long, document: Long, scale: Float): Int

		@JvmStatic
		private external fun nativeGpuDebugLoseContext(gpu: Long)

		@JvmStatic
		private external fun nativeGpuDestroy(gpu: Long)

		/** Returns 0 when no render thread could be started; fall back to the bitmap. */
		@JvmStatic
		private external fun nativeRenderThreadCreate(
			surface: Surface,
			width: Int,
			height: Int,
			backend: Int
		): Long

		/** Records on the calling thread; the rasterizing happens on the render thread. */
		@JvmStatic
		private external fun nativeRenderThreadCommit(
			handle: Long,
			document: Long,
			width: Int,
			height: Int,
			scale: Float
		): Boolean

		@JvmStatic
		private external fun nativeRenderThreadResize(handle: Long, width: Int, height: Int)

		/** The last present's [STATUS_LOST]-style status, or -1 if nothing presented since. */
		@JvmStatic
		private external fun nativeRenderThreadStatus(handle: Long): Int

		@JvmStatic
		private external fun nativeRenderThreadDestroy(handle: Long)

		@JvmStatic
		private external fun nativeDrawSVG(bitmap: Bitmap, scale: Float, svg: String)

		@JvmStatic
		private external fun nativeDrawSVGFromPath(bitmap: Bitmap, scale: Float, svg: String)

		@JvmStatic
		private external fun nativeDrawSVGFromBytes(bitmap: Bitmap, scale: Float, bytes: ByteBuffer)

		@JvmStatic
		private external fun nativeDrawSVGWithBuffer(
			buffer: ByteBuffer,
			width: Int,
			height: Int,
			scale: Float,
			svg: String
		)

		@JvmStatic
		private external fun nativeDrawSVGFromPathWithBuffer(
			buffer: ByteBuffer,
			width: Int,
			height: Int,
			scale: Float,
			svg: String
		)

		@JvmStatic
		private external fun nativeDrawSVGFromBytesWithBuffer(
			buffer: ByteBuffer,
			width: Int,
			height: Int,
			scale: Float,
			bytes: ByteBuffer
		)

		@JvmStatic
		fun fromPathSync(context: Context, path: String): NSCSVGData? {
			val ret = NSCSVGData()
			val file = File(path)
			if (file.length() == 0L) {
				return null
			}
			val text = file.readText()
			val dim = parseSVGDimensions(context, text)
			ret.resize(dim.width, dim.height)
			return ret.mData?.let { data ->
				nativeDrawSVGFromPathWithBuffer(
					data,
					ret.width,
					ret.height,
					context.resources.displayMetrics.density,
					path
				)
				ret
			}
		}

		@JvmStatic
		fun fromPath(context: Context, path: String, callback: Callback) {
			val executor = Executors.newSingleThreadExecutor()
			executor.execute {
				val ret = NSCSVGData()
				val file = File(path)
				if (file.length() == 0L) {
					callback.onSuccess(null)
					return@execute
				}
				val text = file.readText()
				val dim = parseSVGDimensions(context, text)
				ret.resize(dim.width, dim.height)
				ret.mData?.let { data ->
					nativeDrawSVGFromPathWithBuffer(
						data,
						dim.width,
						dim.height,
						context.resources.displayMetrics.density,
						path
					)
					callback.onSuccess(ret)
				} ?: callback.onSuccess(null)

			}
		}

		@JvmStatic
		fun fromStringSync(context: Context, width: Int, height: Int, string: String): NSCSVGData {
			val ret = NSCSVGData()
			ret.resize(width, height)
			ret.mData?.let { data ->
				nativeDrawSVGWithBuffer(
					data,
					width,
					height,
					context.resources.displayMetrics.density,
					string
				)
			}
			return ret
		}

		@JvmStatic
		fun fromString(context: Context, width: Int, height: Int, string: String, callback: Callback) {
			val executor = Executors.newSingleThreadExecutor()
			executor.execute {
				val ret = NSCSVGData()
				ret.resize(width, height)
				ret.mData?.let { data ->
					nativeDrawSVGWithBuffer(
						data,
						width,
						height,
						context.resources.displayMetrics.density,
						string
					)
					callback.onSuccess(ret)
				} ?: callback.onSuccess(null)
			}
		}

		@JvmStatic
		fun fromRemote(context: Context, path: String, callback: Callback) {
			val executor = Executors.newSingleThreadExecutor()
			executor.execute {
				try {
					val url = URL(path)
					val connection = url.openConnection() as HttpURLConnection
					val br = BufferedReader(InputStreamReader(connection.inputStream))
					val svg = br.readText()
					val ret = NSCSVGData()
					val dim = parseSVGDimensions(context, svg)
					ret.resize(dim.width, dim.height)
					ret.mData?.let { data ->
						nativeDrawSVGWithBuffer(
							data,
							dim.width,
							dim.height,
							context.resources.displayMetrics.density,
							svg
						)
						callback.onSuccess(ret)
					} ?: callback.onSuccess(null)
				} catch (e: Exception) {
					callback.onSuccess(null)
				}
			}
		}

		@JvmStatic
		fun fromRemoteSync(
			context: Context,
			path: String
		): NSCSVGData? {
			var ret: NSCSVGData? = null
			val thread = Thread {
				ret = NSCSVGData()
				val url = URL(path)
				val connection = url.openConnection() as HttpURLConnection
				val br = BufferedReader(InputStreamReader(connection.inputStream))
				val svg = br.readText()
				val dim = parseSVGDimensions(context, svg)
				ret?.resize(dim.width, dim.height)
				ret = ret?.mData?.let { data ->
					nativeDrawSVGWithBuffer(
						data,
						dim.width,
						dim.height,
						context.resources.displayMetrics.density,
						svg
					)
					ret
				}
			}
			thread.start()
			thread.join()

			return ret
		}


		@JvmStatic
		fun fromBytesSync(context: Context, bytes: ByteBuffer): NSCSVGData? {
			val ret = NSCSVGData()
			bytes.rewind()
			var svg: String? = null
			try {
				svg = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.KITKAT) {
					StandardCharsets.UTF_8.decode(bytes).toString()
				} else {
					Charset.forName("UTF-8").decode(bytes).toString()
				}
			} catch (_: Exception) {
			} finally {
				bytes.rewind()
			}
			if (svg == null) {
				return null
			}
			val dim = parseSVGDimensions(context, svg)
			ret.resize(dim.width, dim.height)
			return ret.mData?.let { data ->
				nativeDrawSVGWithBuffer(
					data,
					ret.width,
					ret.height,
					context.resources.displayMetrics.density,
					svg
				)
				ret
			}
		}

		@JvmStatic
		fun fromBytes(context: Context, bytes: ByteBuffer, callback: Callback) {
			val executor = Executors.newSingleThreadExecutor()
			executor.execute {
				val ret = NSCSVGData()
				bytes.rewind()

				var svg: String? = null
				try {
					svg = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.KITKAT) {
						StandardCharsets.UTF_8.decode(bytes).toString()
					} else {
						Charset.forName("UTF-8").decode(bytes).toString()
					}
				} catch (_: Exception) {
				} finally {
					bytes.rewind()
				}

				if (svg == null) {
					callback.onSuccess(null)
					return@execute
				}

				val dim = parseSVGDimensions(context, svg)
				ret.resize(dim.width, dim.height)
				ret.mData?.let { data ->
					nativeDrawSVGWithBuffer(
						data,
						dim.width,
						dim.height,
						context.resources.displayMetrics.density,
						svg
					)
					callback.onSuccess(ret)
				} ?: callback.onSuccess(null)

			}
		}


		internal class Dimensions {
			var width = 0
			var height = 0
		}


		internal fun parseSVGDimensions(context: Context, svgString: String): Dimensions {
			val pattern = Pattern.compile("<svg\\s*([^>]*)>", Pattern.CASE_INSENSITIVE)
			val matcher = pattern.matcher(svgString)

			val dimensions = Dimensions()

			if (matcher.find()) {
				val svgAttributes = matcher.group(1)

				val attributesPattern = Pattern.compile("(?:width|height|viewBox)\\s*=\\s*\"([^\"]+)\"")
				val attributesMatcher = attributesPattern.matcher(svgAttributes)

				var width = 0.0
				var height = 0.0
				var widthDefined = false
				var heightDefined = false
				val viewBox = DoubleArray(4)

				while (attributesMatcher.find()) {

					val attributePair = attributesMatcher.group(0)?.trim()
					attributePair?.split(" ")?.let { attributes ->
						for (attr in attributes) {
							attr.split("=").let { parts ->
								if (parts.size == 2) {
									val attributeName = parts[0].trim()
									var attributeValue = parts[1].trim().replace("\"", "")


									val stringLiteralPattern = Pattern.compile("\\\\\"(\\d*\\.?\\d+)\\\\\"")
									val stringLiteralMatcher = stringLiteralPattern.matcher(attributeValue)
									if (stringLiteralMatcher.matches()) {
										stringLiteralMatcher.group(1)?.let {
											attributeValue = it
										}
									}
									if (attributeName == "width") {
										width = attributeValue.toDouble() * context.resources.displayMetrics.density
										widthDefined = true
									} else if (attributeName == "height") {
										height = attributeValue.toDouble() * context.resources.displayMetrics.density
										heightDefined = true
									} else if (attributeName == "viewBox") {
										val viewBoxValues = parts[1].trim { it <= ' ' }
											.split(" ".toRegex()).dropLastWhile { it.isEmpty() }.toTypedArray()
										for (i in 0 until min(4.0, viewBoxValues.size.toDouble()).toInt()) {
											var value = viewBoxValues[i].trim().replace("\"", "");
											val slm = stringLiteralPattern.matcher(value)
											if (slm.matches()) {
												slm.group(1)?.let {
													value = it
												}
											}
											viewBox[i] = value.toDouble()
										}
									}
								}
							}
						}
					}
				}

				if (width == 0.0 && viewBox.size == 4) {
					val viewBoxWidth = viewBox[2]
					val viewBoxHeight = viewBox[3]
					val aspectRatio = viewBoxWidth / viewBoxHeight
					width = context.resources.displayMetrics.widthPixels * aspectRatio
				}

				if (height == 0.0 && viewBox.size == 4) {
					val viewBoxWidth = viewBox[2]
					val viewBoxHeight = viewBox[3]
					val aspectRatio = viewBoxWidth / viewBoxHeight
					height = context.resources.displayMetrics.heightPixels / aspectRatio
				}

				if ((width == 0.0 || width.isNaN()) && !widthDefined) {
					width = context.resources.displayMetrics.widthPixels.toDouble()
				}

				if ((height == 0.0 || height.isNaN()) && !heightDefined) {
					height = context.resources.displayMetrics.heightPixels.toDouble()
				}

				dimensions.width = width.toInt()
				dimensions.height = height.toInt()
			}

			return dimensions
		}
	}
}
