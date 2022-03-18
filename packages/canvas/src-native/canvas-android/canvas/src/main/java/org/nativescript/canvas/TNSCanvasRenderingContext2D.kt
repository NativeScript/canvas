package org.nativescript.canvas

import android.graphics.Bitmap
import android.util.Log
import org.json.JSONException
import org.json.JSONObject
import java.nio.ByteBuffer
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

/**
 * Created by triniwiz on 2019-07-06
 */
class TNSCanvasRenderingContext2D internal constructor(val canvas: TNSCanvas) :
	TNSCanvasRenderingContext {

	@Throws(Throwable::class)
	protected fun finalize() {
		TNSCanvas.nativeDestroyContext(canvas.nativeContext)
	}

	private fun runOnGLThread(runnable: Runnable?) {
		canvas.queueEvent(runnable)
	}


	var direction: TNSTextDirection
		get() {
			printLog("getDirection")
			val lock = CountDownLatch(1)
			var value = TNSTextDirection.Ltr
			runOnGLThread {
				value = if (nativeGetDirection(canvas.nativeContext) == 1) {
					TNSTextDirection.Rtl
				} else {
					TNSTextDirection.Ltr
				}
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
			return value
		}
		set(direction) {
			printLog("setDirection")
			runOnGLThread { nativeSetDirection(canvas.nativeContext, direction.toNative()) }
		}

	fun setDirection(direction: Int) {
		if (direction in 0..1) {
			printLog("setDirection")
			runOnGLThread { nativeSetDirection(canvas.nativeContext, direction) }
		}
	}

	var fillStyle: TNSColorStyle
		get() {
			printLog("getFillStyle")
			val lock = CountDownLatch(1)
			var value: TNSColorStyle = TNSColor("black")
			runOnGLThread {
				val style = nativeGetFillStyle(
					canvas.nativeContext
				)

				try {
					val styleValue = style.getLong("value")
					value = when (style.getInt("value_type")) {
						0 -> TNSColor(styleValue)
						1 -> TNSCanvasGradient(styleValue)
						2 -> TNSPattern(styleValue)
						else -> TNSColor("black")
					}
				} catch (e: JSONException) {
					e.printStackTrace()
				}
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			printLog("setFillStyle")
			runOnGLThread {
				when (value.styleType) {
					TNSColorStyleType.Color -> {
						val color = value as TNSColor
						nativeSetFillColorWithString(canvas.nativeContext, color.color)
					}
					TNSColorStyleType.Pattern -> {
						val pattern = value as TNSPattern
						nativeSetFillStyle(canvas.nativeContext, pattern.style)
					}
					TNSColorStyleType.Gradient -> {
						val gradient = value as TNSCanvasGradient
						nativeSetFillStyle(canvas.nativeContext, gradient.style)
					}
				}
			}
		}

	fun setFillStyle(color: String) {
		runOnGLThread {
			nativeSetFillColorWithString(canvas.nativeContext, color)
		}
	}

	var filter: String
		get() {
			printLog("getFilter")
			val lock = CountDownLatch(1)
			var value = "none"
			runOnGLThread {
				value = nativeGetFilter(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			printLog("setFilter")
			runOnGLThread { nativeSetFilter(canvas.nativeContext, value) }
		}

	var strokeStyle: TNSColorStyle
		get() {
			printLog("getStrokeStyle")
			val lock = CountDownLatch(1)
			var value: TNSColorStyle = TNSColor("black")
			runOnGLThread {
				val style = nativeGetStrokeStyle(
					canvas.nativeContext
				)
				try {
					val styleValue = style.getLong("value")
					value = when (style.getInt("value_type")) {
						0 -> TNSColor(styleValue)
						1 -> TNSCanvasGradient(styleValue)
						2 -> TNSPattern(styleValue)
						else -> TNSColor("black")
					}
				} catch (ignore: JSONException) {
				}
				lock.countDown()
			}

			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			printLog("setStrokeStyle")
			runOnGLThread {
				when (value.styleType) {
					TNSColorStyleType.Color -> {
						val color = value as TNSColor
						nativeSetStrokeColorWithString(canvas.nativeContext, color.color)
					}
					TNSColorStyleType.Pattern -> {
						val pattern = value as TNSPattern
						nativeSetStrokeStyle(canvas.nativeContext, pattern.style)
					}
					TNSColorStyleType.Gradient -> {
						val gradient = value as TNSCanvasGradient
						nativeSetStrokeStyle(canvas.nativeContext, gradient.style)
					}
				}
			}
		}

	fun setStrokeStyle(color: String) {
		runOnGLThread {
			nativeSetStrokeColorWithString(canvas.nativeContext, color)
		}
	}

	var lineWidth: Float
		get() {
			printLog("getLineWidth")
			val lock = CountDownLatch(1)
			var value = 1f
			runOnGLThread {
				value = nativeGetLineWidth(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
			return value
		}
		set(lineWidth) {
			printLog("setLineWidth")
			runOnGLThread { nativeSetLineWidth(canvas.nativeContext, lineWidth) }
		}

	var lineCap: TNSLineCap
		get() {
			printLog("getLineCap")
			val lock = CountDownLatch(1)
			var value = TNSLineCap.Butt
			runOnGLThread {
				value = TNSLineCap.fromNative(nativeGetLineCap(canvas.nativeContext))
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			printLog("setLineCap")
			runOnGLThread { nativeSetLineCap(canvas.nativeContext, value.toNative()) }
		}

	fun setLineCap(cap: Int) {
		if (cap in 0..2) {
			printLog("setLineCap")
			runOnGLThread { nativeSetLineCap(canvas.nativeContext, cap) }
		}
	}

	var lineJoin: TNSLineJoin
		get() {
			printLog("getLineJoin")
			val lock = CountDownLatch(1)
			var value = TNSLineJoin.Bevel
			runOnGLThread {
				value = TNSLineJoin.fromNative(nativeGetLineJoin(canvas.nativeContext))
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			printLog("setLineJoin")
			runOnGLThread { nativeSetLineJoin(canvas.nativeContext, value.toNative()) }
		}

	fun setLineJoin(join: Int) {
		if (join in 0..2) {
			printLog("setLineJoin")
			runOnGLThread { nativeSetLineJoin(canvas.nativeContext, join) }
		}
	}


	var miterLimit: Float
		get() {
			printLog("getMiterLimit")
			val lock = CountDownLatch(1)
			var value = 10f
			runOnGLThread {
				value = nativeGetMiterLimit(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
			return value
		}
		set(limit) {
			printLog("setMiterLimit")
			runOnGLThread { nativeSetMiterLimit(canvas.nativeContext, limit) }
		}

	var lineDashOffset: Float
		get() {
			printLog("getLineDashOffset")
			val lock = CountDownLatch(1)
			var value = 0f
			runOnGLThread {
				value = nativeGetLineDashOffset(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
			return value
		}
		set(offset) {
			printLog("setLineDashOffset")
			runOnGLThread { nativeSetLineDashOffset(canvas.nativeContext, offset) }
		}


	var lineDash: FloatArray
		get() {
			printLog("getLineDash")
			val lock = CountDownLatch(1)
			var value = FloatArray(0)
			runOnGLThread {
				value = nativeGetLineDash(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			printLog("setLineDash")
			runOnGLThread { nativeSetLineDash(canvas.nativeContext, value) }
		}

	var globalCompositeOperation: TNSCompositeOperationType
		get() {
			printLog("getGlobalCompositeOperation")
			val lock = CountDownLatch(1)
			var value = TNSCompositeOperationType.SourceOver
			runOnGLThread {
				value =
					TNSCompositeOperationType.fromNative(nativeGetGlobalCompositeOperation(canvas.nativeContext))
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			printLog("setGlobalCompositeOperation")
			runOnGLThread {
				nativeSetGlobalCompositeOperation(canvas.nativeContext, value.toNative())
			}
		}

	fun setGlobalCompositeOperation(type: Int) {
		if (type in 0..25) {
			printLog("setGlobalCompositeOperation")
			runOnGLThread {
				nativeSetGlobalCompositeOperation(canvas.nativeContext, type)
			}
		}
	}


	var globalAlpha: Float
		get() {
			printLog("getGlobalAlpha")
			val lock = CountDownLatch(1)
			var value = 1f
			runOnGLThread {
				value = nativeGetGlobalAlpha(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(alpha) {
			printLog("setGlobalAlpha")
			runOnGLThread(Runnable {
				nativeSetGlobalAlpha(canvas.nativeContext, alpha)
			})
		}

	var textAlign: TNSTextAlignment
		get() {
			printLog("getTextAlign")
			val lock = CountDownLatch(1)
			var value = TNSTextAlignment.Start
			runOnGLThread {
				value = TNSTextAlignment.fromNative(nativeGetTextAlign(canvas.nativeContext))
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(textAlign) {
			printLog("setTextAlign")
			runOnGLThread {
				nativeSetTextAlign(canvas.nativeContext, textAlign.toNative())
			}
		}


	fun setTextAlign(alignment: Int) {
		if (alignment in 0..4) {
			printLog("setTextAlign")
			runOnGLThread {
				nativeSetTextAlign(canvas.nativeContext, alignment)
			}
		}
	}


	var shadowBlur: Float
		get() {
			printLog("getShadowBlur")
			val lock = CountDownLatch(1)
			var value = 0f
			runOnGLThread {
				value = nativeGetShadowBlur(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(blur) {
			printLog("setShadowBlur")
			runOnGLThread { nativeSetShadowBlur(canvas.nativeContext, blur) }
		}

	var shadowColor: String
		get() {
			printLog("getShadowColor")
			val lock = CountDownLatch(1)
			var value = "rgba(0,0,0,0)"
			runOnGLThread {
				value = nativeGetShadowColor(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(color) {
			printLog("setShadowColor")
			runOnGLThread { nativeSetShadowColorString(canvas.nativeContext, color) }
		}

	var shadowOffsetX: Float
		get() {
			printLog("getShadowOffsetX")
			val lock = CountDownLatch(1)
			var value = 0f
			runOnGLThread {
				value = nativeGetShadowOffsetX(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(x) {
			printLog("setShadowOffsetX")
			runOnGLThread { nativeSetShadowOffsetX(canvas.nativeContext, x) }
		}
	var shadowOffsetY: Float
		get() {
			printLog("getShadowOffsetY")
			val lock = CountDownLatch(1)
			var value = 0f
			runOnGLThread {
				value = nativeGetShadowOffsetY(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(y) {
			printLog("setShadowOffsetY")
			runOnGLThread { nativeSetShadowOffsetY(canvas.nativeContext, y) }
		}

	var font: String
		get() {
			printLog("getFont")
			val lock = CountDownLatch(1)
			var value = "10px sans-serif"
			runOnGLThread {
				value = nativeGetFont(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(font) {
			printLog("setFont")
			runOnGLThread { nativeSetFont(canvas.nativeContext, font) }
		}


	var imageSmoothingEnabled: Boolean
		get() {
			printLog("getImageSmoothingEnabled")
			val lock = CountDownLatch(1)
			var value = false
			runOnGLThread {
				value = nativeGetImageSmoothingEnabled(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(enabled) {
			printLog("setImageSmoothingEnabled")
			runOnGLThread {
				nativeSetImageSmoothingEnabled(canvas.nativeContext, enabled)
			}
		}

	var imageSmoothingQuality: TNSImageSmoothingQuality
		get() {
			printLog("getImageSmoothingQuality")
			val lock = CountDownLatch(1)
			var value = TNSImageSmoothingQuality.Low
			runOnGLThread {
				value =
					TNSImageSmoothingQuality.fromNative(nativeGetImageSmoothingQuality(canvas.nativeContext))
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(quality) {
			printLog("setImageSmoothingQuality")
			runOnGLThread {
				nativeSetImageSmoothingQuality(canvas.nativeContext, quality.toNative())
			}
		}


	var currentTransform: TNSDOMMatrix
		get() {
			printLog("getCurrentTransform")
			val lock = CountDownLatch(1)
			var value: TNSDOMMatrix? = null
			runOnGLThread {
				val id = nativeGetCurrentTransform(canvas.nativeContext)
				value = if (id == 0L) {
					TNSDOMMatrix()
				} else TNSDOMMatrix(id)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
			return value ?: TNSDOMMatrix()
		}
		set(matrix) {
			printLog("setCurrentTransform")
			runOnGLThread {
				nativeSetCurrentTransform(canvas.nativeContext, matrix.matrix)
			}
		}

	private fun updateCanvas() {
		// synchronized (canvasView.lock) {
		canvas.invalidateState = TNSCanvas.InvalidateState.PENDING
		//}
	}

	fun clearRect(x: Float, y: Float, width: Float, height: Float) {
		printLog("clearRect")
		runOnGLThread(Runnable {
			nativeClearRect(canvas.nativeContext, x, y, width, height)
			updateCanvas()
		})
	}

	fun fillRect(x: Float, y: Float, width: Float, height: Float) {
		printLog("fillRect")
		runOnGLThread(Runnable {
			nativeFillRect(canvas.nativeContext, x, y, width, height)
			updateCanvas()
		})
	}

	fun strokeRect(x: Float, y: Float, width: Float, height: Float) {
		printLog("strokeRect")
		runOnGLThread(Runnable {
			nativeStrokeRect(canvas.nativeContext, x, y, width, height)
			updateCanvas()
		})
	}

	fun fillText(text: String, x: Float, y: Float) {
		fillText(text, x, y, 0f)
	}

	fun fillText(text: String, x: Float, y: Float, width: Float) {
		printLog("fillText")
		runOnGLThread(Runnable {
			nativeFillText(canvas.nativeContext, text, x, y, width)
			updateCanvas()
		})
	}


	fun strokeText(text: String, x: Float, y: Float) {
		strokeText(text, x, y, 0f)
	}

	fun strokeText(text: String, x: Float, y: Float, width: Float) {
		printLog("strokeText")
		runOnGLThread(Runnable {
			nativeStrokeText(canvas.nativeContext, text, x, y, width)
			updateCanvas()
		})
	}

	fun rect(x: Float, y: Float, width: Float, height: Float) {
		printLog("rect")
		runOnGLThread(Runnable { nativeRect(canvas.nativeContext, x, y, width, height) })
	}


	fun fill() {
		fill(TNSFillRule.NonZero)
	}

	fun fill(rule: TNSFillRule) {
		fill(0, rule.toNative())
	}

	fun fill(rule: Int) {
		if (rule in 0..1) {
			fill(0, rule)
		}
	}

	fun fill(path: TNSPath2D) {
		fill(path, TNSFillRule.NonZero)
	}

	fun fill(path: TNSPath2D, rule: TNSFillRule) {
		fill(path.path, rule.toNative())
	}

	fun fill(path: TNSPath2D, rule: Int) {
		if (rule in 0..1) {
			fill(path.path, rule)
		}
	}

	private fun fill(path: Long, rule: Int) {
		printLog("fill: path rule")
		runOnGLThread(Runnable {
			nativeFill(canvas.nativeContext, path, rule)
			updateCanvas()
		})
	}

	fun stroke() {
		stroke(0)
	}

	fun stroke(path: TNSPath2D) {
		stroke(path.path)
	}

	private fun stroke(path: Long) {
		printLog("stroke: path")
		runOnGLThread(Runnable {
			nativeStroke(canvas.nativeContext, path)
			updateCanvas()
		})
	}

	fun beginPath() {
		printLog("beginPath")
		runOnGLThread(Runnable { nativeBeginPath(canvas.nativeContext) })
	}

	fun moveTo(x: Float, y: Float) {
		printLog("moveTo")
		runOnGLThread(Runnable { nativeMoveTo(canvas.nativeContext, x, y) })
	}

	fun lineTo(x: Float, y: Float) {
		printLog("lineTo")
		runOnGLThread(Runnable { nativeLineTo(canvas.nativeContext, x, y) })
	}

	fun closePath() {
		printLog("closePath")
		runOnGLThread(Runnable { nativeClosePath(canvas.nativeContext) })
	}


	fun arc(
		x: Float,
		y: Float,
		radius: Float,
		startAngle: Float,
		endAngle: Float
	) {
		arc(x, y, radius, startAngle, endAngle, false)
	}


	fun arc(
		x: Float,
		y: Float,
		radius: Float,
		startAngle: Float,
		endAngle: Float,
		anticlockwise: Boolean = false
	) {
		printLog("arc")
		runOnGLThread(Runnable {
			nativeArc(
				canvas.nativeContext,
				x,
				y,
				radius,
				startAngle,
				endAngle,
				anticlockwise
			)
		})
	}

	fun arcTo(x1: Float, y1: Float, x2: Float, y2: Float, radius: Float) {
		printLog("arcTo")
		runOnGLThread(Runnable { nativeArcTo(canvas.nativeContext, x1, y1, x2, y2, radius) })
	}

	fun bezierCurveTo(cp1x: Float, cp1y: Float, cp2x: Float, cp2y: Float, x: Float, y: Float) {
		printLog("bezierCurveTo")
		runOnGLThread(Runnable {
			nativeBezierCurveTo(
				canvas.nativeContext,
				cp1x,
				cp1y,
				cp2x,
				cp2y,
				x,
				y
			)
		})
	}


	fun ellipse(
		x: Float,
		y: Float,
		radiusX: Float,
		radiusY: Float,
		rotation: Float,
		startAngle: Float,
		endAngle: Float
	) {
		ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, false)
	}


	fun ellipse(
		x: Float,
		y: Float,
		radiusX: Float,
		radiusY: Float,
		rotation: Float,
		startAngle: Float,
		endAngle: Float,
		anticlockwise: Boolean
	) {
		printLog("ellipse")
		runOnGLThread(Runnable {
			nativeEllipse(
				canvas.nativeContext,
				x,
				y,
				radiusX,
				radiusY,
				rotation,
				startAngle,
				endAngle,
				anticlockwise
			)
		})
	}

	fun clip() {
		clip(0, TNSFillRule.NonZero.toNative())
	}

	fun clip(rule: TNSFillRule) {
		clip(0, rule.toNative())
	}

	fun clip(rule: Int) {
		if (rule in 0..1) {
			clip(0, rule)
		}
	}

	fun clip(path: TNSPath2D) {
		clip(path, TNSFillRule.NonZero)
	}

	fun clip(path: TNSPath2D, rule: TNSFillRule) {
		clip(path.path, rule.toNative())
	}

	fun clip(path: TNSPath2D, rule: Int) {
		if (rule in 0..1) {
			clip(path.path, rule)
		}
	}

	private fun clip(path: Long, rule: Int) {
		printLog("clip: path rule")
		runOnGLThread(Runnable { nativeClip(canvas.nativeContext, path, rule) })
	}

	fun createLinearGradient(x0: Float, y0: Float, x1: Float, y1: Float): TNSCanvasGradient {
		printLog("createLinearGradient")
		return TNSCanvasGradient(nativeCreateLinearGradient(canvas.nativeContext, x0, y0, x1, y1))
	}

	fun createRadialGradient(
		x0: Float,
		y0: Float,
		r0: Float,
		x1: Float,
		y1: Float,
		r1: Float
	): TNSCanvasGradient {
		printLog("createRadialGradient")
		val style = nativeCreateRadialGradient(
			canvas.nativeContext,
			x0,
			y0,
			r0,
			x1,
			y1,
			r1
		)
		return TNSCanvasGradient(style)
	}

	fun createPattern(
		src: TNSCanvas?,
		repetition: TNSPatternRepetition = TNSPatternRepetition.Repeat
	): TNSPattern? {
		return createPattern(src, repetition.toNative())
	}

	fun createPattern(
		src: TNSCanvas?,
		repetition: Int
	): TNSPattern? {
		printLog("createPattern: canvas")
		if (src == null) return null
		if (repetition in 0..3) {
			val ss = src.snapshot()
			return createPattern(
				ss,
				src.width,
				src.height,
				repetition
			)
		}
		return null
	}

	fun createPattern(
		src: Bitmap?,
		repetition: TNSPatternRepetition = TNSPatternRepetition.Repeat
	): TNSPattern? {
		return createPattern(src, repetition.toNative())
	}

	fun createPattern(
		src: Bitmap?,
		repetition: Int
	): TNSPattern? {
		printLog("createPattern: bitmap")
		if (src == null) return null
		if (repetition in 0..3) {
			return createPattern(
				Utils.getBytesFromBitmap(src),
				src.width,
				src.height,
				repetition
			)
		}
		return null
	}

	fun createPattern(
		data: ByteArray,
		width: Int,
		height: Int,
		repetition: Int
	): TNSPattern? {
		printLog("createPattern: imagebitmap")
		var value: TNSPattern? = null
		if (repetition in 0..3) {
			val lock = CountDownLatch(1)
			runOnGLThread {
				val id = nativeCreatePattern(
					canvas.nativeContext,
					data, width, height, repetition
				)

				if (id != 0L) {
					value = TNSPattern(id)
				}
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
		}
		return value
	}


	fun createPattern(
		data: ByteBuffer,
		width: Int,
		height: Int,
		repetition: Int
	): TNSPattern? {
		printLog("createPattern: imagebitmap")
		var value: TNSPattern? = null
		if (repetition in 0..3) {
			val lock = CountDownLatch(1)
			runOnGLThread {
				val id = if (data.isDirect) {
					nativeCreatePatternWithBuffer(
						canvas.nativeContext,
						data, width, height, repetition
					)
				} else {
					nativeCreatePattern(
						canvas.nativeContext,
						data.array(), width, height, repetition
					)
				}

				if (id != 0L) {
					value = TNSPattern(id)
				}
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: Exception) {
			}
		}
		return value
	}


	private fun createPatternEncoded(
		data: ByteArray,
		repetition: Int
	): TNSPattern? {
		printLog("createPattern: imagebitmap")
		val lock = CountDownLatch(1)
		var value: TNSPattern? = null
		runOnGLThread {
			val id = nativeCreatePatternEncoded(
				canvas.nativeContext,
				data, repetition
			)

			if (id != 0L) {
				value = TNSPattern(id)
			}
			lock.countDown()
		}
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (e: Exception) {
		}
		return value
	}


	private fun createPatternEncoded(
		data: ByteBuffer,
		repetition: Int
	): TNSPattern? {
		printLog("createPattern: imagebitmap")
		val lock = CountDownLatch(1)
		var value: TNSPattern? = null
		runOnGLThread {
			val id = if (data.isDirect) {
				nativeCreatePatternEncodedWithBuffer(
					canvas.nativeContext,
					data, repetition
				)
			} else {
				nativeCreatePatternEncoded(
					canvas.nativeContext,
					data.array(), repetition
				)
			}

			if (id != 0L) {
				value = TNSPattern(id)
			}
			lock.countDown()
		}
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (e: Exception) {
		}
		return value
	}


	fun createPattern(
		src: TNSImageAsset?,
		repetition: TNSPatternRepetition = TNSPatternRepetition.Repeat
	): TNSPattern? {
		printLog("createPattern: asset")
		if (src == null) return null
		val lock = CountDownLatch(1)
		var value: TNSPattern? = null
		runOnGLThread {
			val id = nativeCreatePatternWithAsset(
				canvas.nativeContext,
				src.nativeImageAsset, repetition.toNative()
			)
			if (id != 0L) {
				value = TNSPattern(id)
			}
			lock.countDown()
		}
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (e: java.lang.Exception) {
		}
		return value
	}

	fun createPattern(
		src: TNSImageBitmap?,
		repetition: TNSPatternRepetition = TNSPatternRepetition.Repeat
	): TNSPattern? {
		return createPattern(src, repetition.toNative())
	}

	fun createPattern(
		src: TNSImageBitmap?,
		repetition: Int
	): TNSPattern? {
		printLog("createPattern: imagebitmap")
		if (src == null) return null
		var value: TNSPattern? = null
		if (repetition in 0..3) {
			val lock = CountDownLatch(1)
			runOnGLThread {
				val id = nativeCreatePatternWithAsset(
					canvas.nativeContext,
					src.nativeImageAsset, repetition
				)
				if (id != 0L) {
					value = TNSPattern(id)
				}
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
			} catch (e: java.lang.Exception) {
			}
		}
		return value
	}


	fun save() {
		printLog("save")
		runOnGLThread(Runnable {
			nativeSave(canvas.nativeContext)
		})
	}

	fun restore() {
		printLog("restore")
		runOnGLThread(Runnable {
			nativeRestore(canvas.nativeContext)
		})
	}

	fun setTransform(a: Float, b: Float, c: Float, d: Float, e: Float, f: Float) {
		printLog("setTransform")
		runOnGLThread(Runnable {
			nativeSetTransform(canvas.nativeContext, a, b, c, d, e, f)
		})
	}

	fun transform(a: Float, b: Float, c: Float, d: Float, e: Float, f: Float) {
		printLog("transform")
		runOnGLThread(Runnable { nativeTransform(canvas.nativeContext, a, b, c, d, e, f) })
	}

	fun scale(x: Float, y: Float) {
		printLog("scale")
		runOnGLThread(Runnable { nativeScale(canvas.nativeContext, x, y) })
	}

	fun rotate(angle: Float) {
		printLog("rotate")
		runOnGLThread(Runnable { nativeRotate(canvas.nativeContext, angle) })
	}

	fun translate(x: Float, y: Float) {
		printLog("translate")
		runOnGLThread(Runnable { nativeTranslate(canvas.nativeContext, x, y) })
	}

	fun quadraticCurveTo(cpx: Float, cpy: Float, x: Float, y: Float) {
		printLog("quadraticCurveTo")
		runOnGLThread(Runnable {
			nativeQuadraticCurveTo(canvas.nativeContext, cpx, cpy, x, y)
		})
	}

	fun drawImage(image: TNSCanvas, dx: Float, dy: Float) {
		printLog("drawImage: canvas")
		val ss = image.snapshot()
		var width = image.width
		var height = image.height
		if (width == 0) {
			val params = image.layoutParams
			if (params != null) {
				width = params.width
			}
		}
		if (height == 0) {
			val params = image.layoutParams
			if (params != null) {
				height = params.height
			}
		}
		if (width < 1) {
			width = 1
		}
		if (height < 1) {
			height = 1
		}
		val finalWidth = width.toFloat()
		val finalHeight = height.toFloat()
		runOnGLThread(Runnable {
			nativeDrawImageDxDyWithBuffer(
				canvas.nativeContext,
				ss,
				finalWidth,
				finalHeight,
				dx,
				dy
			)
			updateCanvas()
		})
	}

	fun drawImage(image: Bitmap?, dx: Float, dy: Float) {
		printLog("drawImage: bitmap")
		image?.let {
			runOnGLThread(Runnable {
				val width = image.width.toFloat()
				val height = image.height.toFloat()
				nativeDrawImageDxDyWithBitmap(
					canvas.nativeContext,
					image,
					width,
					height,
					dx,
					dy
				)
				updateCanvas()
			})
		}
	}

	fun drawImage(asset: TNSImageAsset, dx: Float, dy: Float) {
		printLog("drawImage: asset")
		runOnGLThread {
			nativeDrawImageDxDyWithAsset(
				canvas.nativeContext,
				asset.nativeImageAsset,
				dx,
				dy
			)
			updateCanvas()
		}
	}

	fun drawImage(bitmap: TNSImageBitmap, dx: Float, dy: Float) {
		printLog("drawImage: bitmap")
		runOnGLThread(Runnable {
			nativeDrawImageDxDyWithAsset(
				canvas.nativeContext,
				bitmap.nativeImageAsset,
				dx,
				dy
			)
			updateCanvas()
		})
	}

	fun drawImage(image: TNSCanvas, dx: Float, dy: Float, dWidth: Float, dHeight: Float) {
		printLog("drawImage: canvas")
		val ss = image.snapshot()
		var width = image.width
		var height = image.height
		if (width == 0) {
			val params = image.layoutParams
			if (params != null) {
				width = params.width
			}
		}
		if (height == 0) {
			val params = image.layoutParams
			if (params != null) {
				height = params.height
			}
		}
		if (width < 1) {
			width = 1
		}
		if (height < 1) {
			height = 1
		}
		val finalWidth = width.toFloat()
		val finalHeight = height.toFloat()
		runOnGLThread(Runnable {
			nativeDrawImageDxDyDwDyWithBuffer(
				canvas.nativeContext,
				ss,
				finalWidth,
				finalHeight,
				dx,
				dy,
				dWidth,
				dHeight
			)
			updateCanvas()
		})
	}

	fun drawImage(image: Bitmap?, dx: Float, dy: Float, dWidth: Float, dHeight: Float) {
		printLog("drawImage: bitmap")
		image?.let {
			runOnGLThread(Runnable {
				nativeDrawImageDxDyDwDhWithBitmap(
					canvas.nativeContext,
					image,
					image.width.toFloat(),
					image.height.toFloat(),
					dx,
					dy,
					dWidth,
					dHeight
				)
				updateCanvas()
			})
		}
	}

	fun drawImage(asset: TNSImageAsset, dx: Float, dy: Float, dWidth: Float, dHeight: Float) {
		printLog("drawImage: asset")
		runOnGLThread(Runnable {
			nativeDrawImageDxDyDwDhWithAsset(
				canvas.nativeContext,
				asset.nativeImageAsset,
				dx,
				dy,
				dWidth,
				dHeight
			)
			updateCanvas()
		})
	}

	fun drawImage(bitmap: TNSImageBitmap, dx: Float, dy: Float, dWidth: Float, dHeight: Float) {
		printLog("drawImage: bitmap")
		runOnGLThread {
			nativeDrawImageDxDyDwDhWithAsset(
				canvas.nativeContext,
				bitmap.nativeImageAsset,
				dx,
				dy,
				dWidth,
				dHeight
			)
			updateCanvas()
		}
	}

	fun drawImage(
		image: TNSCanvas,
		sx: Float,
		sy: Float,
		sWidth: Float,
		sHeight: Float,
		dx: Float,
		dy: Float,
		dWidth: Float,
		dHeight: Float
	) {
		printLog("drawImage: canvas")
		val ss = image.snapshot()
		var width = image.width
		var height = image.height
		if (image.width == 0) {
			val params = image.layoutParams
			if (params != null) {
				width = params.width
			}
		}
		if (image.height == 0) {
			val params = image.layoutParams
			if (params != null) {
				height = params.height
			}
		}
		if (width < 1) {
			width = 1
		}
		if (height < 1) {
			height = 1
		}
		val finalWidth = width.toFloat()
		val finalHeight = height.toFloat()
		runOnGLThread(Runnable {
			nativeDrawImageWithBuffer(
				canvas.nativeContext,
				ss,
				finalWidth,
				finalHeight,
				sx,
				sy,
				sWidth,
				sHeight,
				dx,
				dy,
				dWidth,
				dHeight
			)
			updateCanvas()
		})
	}

	fun drawImage(
		image: Bitmap?,
		sx: Float,
		sy: Float,
		sWidth: Float,
		sHeight: Float,
		dx: Float,
		dy: Float,
		dWidth: Float,
		dHeight: Float
	) {
		printLog("drawImage: bitmap")
		image?.let {
			runOnGLThread(Runnable {
				nativeDrawImageWithBitmap(
					canvas.nativeContext,
					image,
					image.width.toFloat(),
					image.height.toFloat(),
					sx,
					sy,
					sWidth,
					sHeight,
					dx,
					dy,
					dWidth,
					dHeight
				)
				updateCanvas()
			})
		}
	}

	fun drawImage(
		asset: TNSImageAsset,
		sx: Float,
		sy: Float,
		sWidth: Float,
		sHeight: Float,
		dx: Float,
		dy: Float,
		dWidth: Float,
		dHeight: Float
	) {
		printLog("drawImage: asset")
		runOnGLThread(Runnable {
			nativeDrawImageWithAsset(
				canvas.nativeContext,
				asset.nativeImageAsset,
				sx,
				sy,
				sWidth,
				sHeight,
				dx,
				dy,
				dWidth,
				dHeight
			)
			updateCanvas()
		})
	}


	fun drawImage(
		bitmap: TNSImageBitmap,
		sx: Float,
		sy: Float,
		sWidth: Float,
		sHeight: Float,
		dx: Float,
		dy: Float,
		dWidth: Float,
		dHeight: Float
	) {
		printLog("drawImage: bitmap")
		runOnGLThread(Runnable {
			nativeDrawImageWithAsset(
				canvas.nativeContext,
				bitmap.nativeImageAsset,
				sx,
				sy,
				sWidth,
				sHeight,
				dx,
				dy,
				dWidth,
				dHeight
			)
			updateCanvas()
		})
	}


	fun measureText(text: String?): TNSTextMetrics {
		printLog("measureText")
		return TNSTextMetrics(nativeMeasureText(canvas.nativeContext, text ?: ""))
	}

	fun createImageData(width: Int, height: Int): TNSImageData {
		printLog("createImageData")
		return TNSImageData(width, height, nativeCreateImageData(canvas.nativeContext, width, height))
	}

	fun createImageData(imageData: TNSImageData): TNSImageData {
		printLog("createImageData")
		val width = imageData.width
		val height = imageData.height
		return TNSImageData(
			width, height, nativeCreateImageData(canvas.nativeContext, width, height)
		)
	}

	fun putImageData(
		data: TNSImageData,
		x: Float,
		y: Float
	) {
		putImageData(data, x, y, 0f, 0f, 0f, 0f)
	}


	fun putImageData(
		data: TNSImageData,
		x: Float,
		y: Float,
		dirtyX: Float,
		dirtyY: Float,
		dirtyWidth: Float,
		dirtyHeight: Float
	) {
		runOnGLThread {
			nativePutImageData(
				canvas.nativeContext,
				data.nativeImageData,
				x,
				y,
				dirtyX,
				dirtyY,
				dirtyWidth,
				dirtyHeight
			)
			updateCanvas()
		}
	}

	fun getImageData(sx: Float, sy: Float, sw: Float, sh: Float): TNSImageData {
		printLog("getImageData")
		val lock = CountDownLatch(1)
		var value: TNSImageData? = null
		runOnGLThread {
			value = TNSImageData(
				sw.toInt(),
				sh.toInt(),
				nativeGetImageData(canvas.nativeContext, sx, sy, sw, sh)
			)
			lock.countDown()
		}
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (e: Exception) {
		}
		return value ?: TNSImageData(sw.toInt(), sh.toInt(), -1)
	}


	fun resetTransform() {
		printLog("resetTransform")
		runOnGLThread(Runnable { nativeResetTransform(canvas.nativeContext) })
	}

	fun isPointInPath(x: Float, y: Float): Boolean {
		return isPointInPath(0, x, y, TNSFillRule.NonZero.toNative())
	}

	fun isPointInPath(x: Float, y: Float, fillRule: TNSFillRule): Boolean {
		return isPointInPath(0, x, y, fillRule.toNative())
	}


	fun isPointInPath(
		path: TNSPath2D,
		x: Float,
		y: Float
	): Boolean {
		return isPointInPath(path.path, x, y, TNSFillRule.NonZero.toNative())
	}

	fun isPointInPath(
		path: TNSPath2D,
		x: Float,
		y: Float,
		fillRule: TNSFillRule
	): Boolean {
		return isPointInPath(path.path, x, y, fillRule.toNative())
	}

	private fun isPointInPath(path: Long, x: Float, y: Float, fillRule: Int): Boolean {
		printLog("isPointInPath")
		val lock = CountDownLatch(1)
		var value = false
		runOnGLThread {
			value = nativeIsPointInPath(canvas.nativeContext, path, x, y, fillRule)
			lock.countDown()
		}
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (e: java.lang.Exception) {
		}
		return value
	}

	fun isPointInStroke(x: Float, y: Float): Boolean {
		return isPointInStroke(null, x, y)
	}

	fun isPointInStroke(path: TNSPath2D? = null, x: Float, y: Float): Boolean {
		printLog("isPointInStroke")
		val lock = CountDownLatch(1)
		var value = false
		runOnGLThread {
			value = nativeIsPointInStroke(canvas.nativeContext, path?.path ?: 0L, x, y)
			lock.countDown()
		}
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (e: java.lang.Exception) {
		}
		return value
	}

	companion object {
		const val TAG = "CanvasRenderingContext"

		@JvmStatic
		private external fun nativeSetDirection(context: Long, direction: Int)

		@JvmStatic
		private external fun nativeGetDirection(context: Long): Int

		@JvmStatic
		private external fun nativeSetFillColorWithString(context: Long, color: String)

		@JvmStatic
		private external fun nativeSetFillStyle(context: Long, style: Long)

		@JvmStatic
		private external fun nativeGetFillStyle(context: Long): JSONObject

		@JvmStatic
		private external fun nativeSetFilter(context: Long, filter: String)

		@JvmStatic
		private external fun nativeGetFilter(context: Long): String

		@JvmStatic
		private external fun nativeSetStrokeColorWithString(context: Long, color: String)

		@JvmStatic
		private external fun nativeSetStrokeStyle(context: Long, style: Long)

		@JvmStatic
		private external fun nativeGetStrokeStyle(context: Long): JSONObject

		@JvmStatic
		private external fun nativeGetLineWidth(context: Long): Float

		@JvmStatic
		private external fun nativeSetLineWidth(context: Long, width: Float)

		@JvmStatic
		private external fun nativeSetLineCap(context: Long, cap: Int)

		@JvmStatic
		private external fun nativeGetLineCap(context: Long): Int

		@JvmStatic
		private external fun nativeSetLineJoin(context: Long, join: Int)

		@JvmStatic
		private external fun nativeGetLineJoin(context: Long): Int

		@JvmStatic
		private external fun nativeSetMiterLimit(context: Long, limit: Float)

		@JvmStatic
		private external fun nativeGetMiterLimit(context: Long): Float

		@JvmStatic
		private external fun nativeSetLineDashOffset(context: Long, offset: Float)

		@JvmStatic
		private external fun nativeGetLineDashOffset(context: Long): Float

		@JvmStatic
		private external fun nativeSetLineDash(context: Long, dash: FloatArray)

		@JvmStatic
		private external fun nativeGetLineDash(context: Long): FloatArray

		@JvmStatic
		private external fun nativeClearRect(
			context: Long,
			x: Float,
			y: Float,
			width: Float,
			height: Float
		)

		@JvmStatic
		private external fun nativeFillRect(
			context: Long,
			x: Float,
			y: Float,
			width: Float,
			height: Float
		)

		@JvmStatic
		private external fun nativeStrokeRect(
			context: Long,
			x: Float,
			y: Float,
			width: Float,
			height: Float
		)

		@JvmStatic
		private external fun nativeFillText(
			context: Long,
			text: String,
			x: Float,
			y: Float,
			width: Float
		)

		@JvmStatic
		private external fun nativeStrokeText(
			context: Long,
			text: String,
			x: Float,
			y: Float,
			width: Float
		)

		@JvmStatic
		private external fun nativeRect(context: Long, x: Float, y: Float, width: Float, height: Float)

		@JvmStatic
		private external fun nativeFill(context: Long, path: Long, fill: Int)

		@JvmStatic
		private external fun nativeStroke(context: Long, path: Long)

		@JvmStatic
		private external fun nativeBeginPath(context: Long)

		@JvmStatic
		private external fun nativeMoveTo(context: Long, x: Float, y: Float)

		@JvmStatic
		private external fun nativeLineTo(context: Long, x: Float, y: Float)

		@JvmStatic
		private external fun nativeClosePath(context: Long)

		@JvmStatic
		private external fun nativeArc(
			context: Long,
			x: Float,
			y: Float,
			radius: Float,
			startAngle: Float,
			endAngle: Float,
			anticlockwise: Boolean
		)

		@JvmStatic
		private external fun nativeArcTo(
			context: Long,
			x1: Float,
			y1: Float,
			x2: Float,
			y2: Float,
			radius: Float
		)

		@JvmStatic
		private external fun nativeBezierCurveTo(
			context: Long,
			cp1x: Float,
			cp1y: Float,
			cp2x: Float,
			cp2y: Float,
			x: Float,
			y: Float
		)

		@JvmStatic
		private external fun nativeEllipse(
			context: Long,
			x: Float,
			y: Float,
			radiusX: Float,
			radiusY: Float,
			rotation: Float,
			startAngle: Float,
			endAngle: Float,
			anticlockwise: Boolean
		)

		@JvmStatic
		private external fun nativeClip(context: Long, path: Long, rule: Int)

		@JvmStatic
		private external fun nativeCreateLinearGradient(
			context: Long,
			x0: Float,
			y0: Float,
			x1: Float,
			y1: Float
		): Long

		@JvmStatic
		private external fun nativeCreateRadialGradient(
			context: Long,
			x0: Float,
			y0: Float,
			r0: Float,
			x1: Float,
			y1: Float,
			r1: Float
		): Long

		@JvmStatic
		private external fun nativeCreatePattern(
			context: Long,
			data: ByteArray,
			width: Int,
			height: Int,
			repetition: Int
		): Long

		@JvmStatic
		private external fun nativeCreatePatternWithBuffer(
			context: Long,
			data: ByteBuffer,
			width: Int,
			height: Int,
			repetition: Int
		): Long


		@JvmStatic
		private external fun nativeCreatePatternEncoded(
			context: Long,
			data: ByteArray,
			repetition: Int
		): Long


		@JvmStatic
		private external fun nativeCreatePatternEncodedWithBuffer(
			context: Long,
			data: ByteBuffer,
			repetition: Int
		): Long

		@JvmStatic
		private external fun nativeCreatePatternWithAsset(
			context: Long,
			asset: Long,
			repetition: Int
		): Long

		@JvmStatic
		private external fun nativeGetGlobalCompositeOperation(context: Long): Int

		@JvmStatic
		private external fun nativeSetGlobalCompositeOperation(context: Long, operation: Int)

		@JvmStatic
		private external fun nativeSetGlobalAlpha(context: Long, globalAlpha: Float)

		@JvmStatic
		private external fun nativeGetGlobalAlpha(context: Long): Float

		@JvmStatic
		private external fun nativeSetTextAlign(context: Long, alignment: Int)

		@JvmStatic
		private external fun nativeGetTextAlign(context: Long): Int

		@JvmStatic
		private external fun nativeSave(context: Long)

		@JvmStatic
		private external fun nativeRestore(context: Long)

		@JvmStatic
		private external fun nativeSetTransform(
			context: Long,
			a: Float,
			b: Float,
			c: Float,
			d: Float,
			e: Float,
			f: Float
		)

		@JvmStatic
		private external fun nativeTransform(
			context: Long,
			a: Float,
			b: Float,
			c: Float,
			d: Float,
			e: Float,
			f: Float
		)

		@JvmStatic
		private external fun nativeScale(context: Long, x: Float, y: Float)

		@JvmStatic
		private external fun nativeRotate(context: Long, angle: Float)

		@JvmStatic
		private external fun nativeTranslate(context: Long, x: Float, y: Float)

		@JvmStatic
		private external fun nativeQuadraticCurveTo(
			context: Long,
			cpx: Float,
			cpy: Float,
			x: Float,
			y: Float
		)

		@JvmStatic
		private external fun nativeDrawImageDxDy(
			context: Long,
			data: ByteArray,
			width: Float,
			height: Float,
			dx: Float,
			dy: Float
		)


		@JvmStatic
		private external fun nativeDrawImageDxDyWithBuffer(
			context: Long,
			data: ByteBuffer,
			width: Float,
			height: Float,
			dx: Float,
			dy: Float
		)

		@JvmStatic
		private external fun nativeDrawImageDxDyWithBitmap(
			context: Long,
			bitmap: Bitmap,
			width: Float,
			height: Float,
			dx: Float,
			dy: Float
		)

		@JvmStatic
		private external fun nativeDrawImageDxDyWithAsset(
			context: Long,
			asset: Long,
			dx: Float,
			dy: Float
		)


		@JvmStatic
		private external fun nativeDrawImageDxDyDwDy(
			context: Long,
			data: ByteArray,
			width: Float,
			height: Float,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		)


		@JvmStatic
		private external fun nativeDrawImageDxDyDwDyWithBuffer(
			context: Long,
			data: ByteBuffer,
			width: Float,
			height: Float,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		)


		@JvmStatic
		private external fun nativeDrawImageDxDyDwDhWithBitmap(
			context: Long,
			bitmap: Bitmap,
			width: Float,
			height: Float,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		)

		@JvmStatic
		private external fun nativeDrawImageDxDyDwDhWithAsset(
			context: Long,
			asset: Long,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		)


		@JvmStatic
		private external fun nativeDrawImage(
			context: Long,
			data: ByteArray,
			width: Float,
			height: Float,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		)


		@JvmStatic
		private external fun nativeDrawImageWithBuffer(
			context: Long,
			data: ByteBuffer,
			width: Float,
			height: Float,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		)

		@JvmStatic
		private external fun nativeDrawImageWithBitmap(
			context: Long,
			bitmap: Bitmap,
			width: Float,
			height: Float,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		)

		@JvmStatic
		private external fun nativeDrawImageWithAsset(
			context: Long,
			asset: Long,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		)


		@JvmStatic
		private external fun nativeGetShadowBlur(context: Long): Float

		@JvmStatic
		private external fun nativeSetShadowBlur(context: Long, blur: Float)

		@JvmStatic
		private external fun nativeGetShadowColor(context: Long): String

		@JvmStatic
		private external fun nativeSetShadowColor(context: Long, r: Int, g: Int, b: Int, a: Int): String

		@JvmStatic
		private external fun nativeSetShadowColorString(context: Long, color: String)

		@JvmStatic
		private external fun nativeGetShadowOffsetX(context: Long): Float

		@JvmStatic
		private external fun nativeSetShadowOffsetX(context: Long, x: Float)

		@JvmStatic
		private external fun nativeGetShadowOffsetY(context: Long): Float

		@JvmStatic
		private external fun nativeSetShadowOffsetY(context: Long, y: Float)

		@JvmStatic
		private external fun nativeGetFont(context: Long): String

		@JvmStatic
		private external fun nativeSetFont(context: Long, font: String)

		@JvmStatic
		private external fun nativeGetImageSmoothingEnabled(context: Long): Boolean

		@JvmStatic
		private external fun nativeSetImageSmoothingEnabled(context: Long, enabled: Boolean)

		@JvmStatic
		private external fun nativeGetImageSmoothingQuality(context: Long): Int

		@JvmStatic
		private external fun nativeSetImageSmoothingQuality(context: Long, quality: Int)

		@JvmStatic
		private external fun nativeGetCurrentTransform(context: Long): Long

		@JvmStatic
		private external fun nativeSetCurrentTransform(context: Long, transform: Long)

		@JvmStatic
		private external fun nativeMeasureText(context: Long, text: String): Long

		@JvmStatic
		private external fun nativeCreateImageData(context: Long, width: Int, height: Int): Long

		@JvmStatic
		private external fun nativePutImageData(
			context: Long,
			imageData: Long,
			dx: Float,
			dy: Float,
			dirtyX: Float,
			dirtyY: Float,
			dirtyWidth: Float,
			dirtyHeight: Float
		)

		@JvmStatic
		private external fun nativeGetImageData(
			context: Long,
			sx: Float,
			sy: Float,
			sw: Float,
			sh: Float
		): Long

		@JvmStatic
		private external fun nativeResetTransform(context: Long)

		@JvmStatic
		private external fun nativeIsPointInPath(
			context: Long,
			path: Long,
			x: Float,
			y: Float,
			rule: Int
		): Boolean

		@JvmStatic
		private external fun nativeIsPointInStroke(
			context: Long,
			path: Long,
			x: Float,
			y: Float
		): Boolean

		private fun printLog(msg: String) {
			if (isDebug) {
				Log.d(TAG, msg)
			}
		}

		var isDebug = false
	}
}
