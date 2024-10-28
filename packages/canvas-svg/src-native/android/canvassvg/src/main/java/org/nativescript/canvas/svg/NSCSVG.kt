package org.nativescript.canvas.svg

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Matrix
import android.os.Build
import android.util.AttributeSet
import android.view.View
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


class NSCSVGData {
	internal var mData: ByteBuffer? = null
	internal var mWidth: Int = 0
	internal var mHeight: Int = 0
	internal var mSize: Int = 0
	internal var mBitmap: Bitmap? = null


	fun resize(width: Int, height: Int) {
		if (width > 0 && height > 0) {
			mData = null
			mBitmap = null
			val size = (width * height * 4)
			mData = ByteBuffer.allocateDirect(size)
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

	val width: Int
		get() {
			return this.mWidth
		}

	val height: Int
		get() {
			return this.mHeight
		}

	fun toImage(): Bitmap? {
		if (mBitmap == null) {
			if (mWidth > 0 && mHeight > 0) {
				mData?.let { data ->
					val bitmap = Bitmap.createBitmap(mWidth.toInt(), mHeight.toInt(), Bitmap.Config.ARGB_8888)
					data.rewind()
					bitmap.copyPixelsFromBuffer(data)
				}
			}
		}
		return mBitmap
	}
}


class NSCSVG : View {
	var data: NSCSVGData? = null
		internal set
	internal val lock = Any()
	private var pendingInvalidate: Boolean = false
	private val executor = Executors.newSingleThreadExecutor()
	internal var src: String = ""
	internal var srcPath: String = ""
	private var mMatrix = Matrix()

	var sync: Boolean = false

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
		setBackgroundColor(Color.TRANSPARENT)
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

	override fun onDraw(canvas: Canvas) {
		data?.toImage()?.let {
			canvas.drawBitmap(it, mMatrix, null)
		}
	}

	private fun doDraw() {
		data?.toImage()?.let {
			currentTask?.cancel(true)
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


	interface Callback {
		fun onSuccess(view: NSCSVGData?)
	}

	companion object {
		init {
			try {
				System.loadLibrary("canvassvg")
			} catch (_: Exception) {
			}
		}

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
