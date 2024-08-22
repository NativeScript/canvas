package org.nativescript.canvas.svg

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Matrix
import android.util.AttributeSet
import android.util.Log
import android.view.View
import android.view.ViewGroup
import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader
import java.net.HttpURLConnection
import java.net.URL
import java.nio.ByteBuffer
import java.util.concurrent.CountDownLatch
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit
import java.util.regex.Pattern
import kotlin.math.min


class NSCSVG : View {
	var bitmap: Bitmap? = null
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
		setBackgroundColor(Color.TRANSPARENT)
	}

	private var currentTask: java.util.concurrent.Future<*>? = null
	private fun resize(w: Int, h: Int) {
		doDraw()
	}

	override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
		super.onSizeChanged(w, h, oldw, oldh)

		if ((w != 0 && h != 0) && w != layoutParams.width && h != layoutParams.height) {

			bitmap = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888)

			resize(w, h)
		}
	}

	override fun onDraw(canvas: Canvas) {
		bitmap?.let {
			canvas.drawBitmap(it, mMatrix, null)
		}
	}

	private fun doDraw() {
		bitmap?.let {
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
		fun onSuccess(view: NSCSVG?)
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
		fun fromPathSync(context: Context, path: String): NSCSVG? {
			val view = NSCSVG(context)
			val file = File(path)
			if (file.length() == 0L) {
				return null
			}
			val text = file.readText()
			val dim = parseSVGDimensions(context, text)
			view.layoutParams = ViewGroup.LayoutParams(dim.width, dim.height)
			val bitmap = Bitmap.createBitmap(dim.width, dim.height, Bitmap.Config.ARGB_8888)
			nativeDrawSVGFromPath(bitmap, context.resources.displayMetrics.density, path)
			return view
		}

		@JvmStatic
		fun fromPath(context: Context, path: String, callback: Callback) {
			val executor = Executors.newSingleThreadExecutor()
			executor.execute {
				val view = NSCSVG(context)
				val file = File(path)
				if (file.length() == 0L) {
					callback.onSuccess(null)
					return@execute
				}
				val text = file.readText()
				val dim = parseSVGDimensions(context, text)
				view.layoutParams = ViewGroup.LayoutParams(dim.width, dim.height)
				val bitmap = Bitmap.createBitmap(dim.width, dim.height, Bitmap.Config.ARGB_8888)
				nativeDrawSVGFromPath(bitmap, context.resources.displayMetrics.density, path)
				callback.onSuccess(view)
			}
		}

		@JvmStatic
		fun fromStringSync(context: Context, width: Int, height: Int, string: String): NSCSVG {
			val view = NSCSVG(context)
			view.layoutParams = ViewGroup.LayoutParams(width, height)
			val bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
			nativeDrawSVG(bitmap, context.resources.displayMetrics.density, string)
			view.bitmap = bitmap
			return view
		}

		@JvmStatic
		fun fromString(context: Context, width: Int, height: Int, string: String, callback: Callback) {
			val executor = Executors.newSingleThreadExecutor()
			executor.execute {
				val view = NSCSVG(context)
				view.layoutParams = ViewGroup.LayoutParams(width, height)
				val bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
				nativeDrawSVG(bitmap, context.resources.displayMetrics.density, string)
				view.bitmap = bitmap
				callback.onSuccess(view)
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
					val view = NSCSVG(context)
					val dim = parseSVGDimensions(context, svg)
					view.layoutParams = ViewGroup.LayoutParams(dim.width, dim.height)
					val bitmap = Bitmap.createBitmap(dim.width, dim.height, Bitmap.Config.ARGB_8888)
					nativeDrawSVG(bitmap, context.resources.displayMetrics.density, svg)
					view.bitmap = bitmap
					callback.onSuccess(view)
				} catch (e: Exception) {
					e.printStackTrace()
					callback.onSuccess(null)
				}
			}
		}

		@JvmStatic
		fun fromRemoteSync(
			context: Context,
			path: String
		): NSCSVG? {
			var view: NSCSVG? = null
			val thread = Thread {
				view = NSCSVG(context)
				val url = URL(path)
				val connection = url.openConnection() as HttpURLConnection
				val br = BufferedReader(InputStreamReader(connection.inputStream))
				val svg = br.readText()
				val dim = parseSVGDimensions(context, svg)
				view?.layoutParams = ViewGroup.LayoutParams(dim.width, dim.height)
				val bitmap = Bitmap.createBitmap(dim.width, dim.height, Bitmap.Config.ARGB_8888)
				nativeDrawSVG(bitmap, context.resources.displayMetrics.density, svg)
				view?.bitmap = bitmap
			}
			thread.start()
			thread.join()

			return view
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
					attributePair?.split("=")?.let { parts ->
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
