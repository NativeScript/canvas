package org.nativescript.canvas

import android.graphics.Bitmap
import android.util.Log
import java.util.concurrent.TimeUnit

/**
 * Created by triniwiz on 2019-07-06
 */
class TNSCanvasRenderingContext2D internal constructor(val canvas: TNSCanvas) :
	TNSCanvasRenderingContext {

	private val lock = ResettableCountDownLatch(1)

	@Throws(Throwable::class)
	protected fun finalize() {
		nativeDestroy(canvas.nativeContext)
	}

	var direction: TNSTextDirection
		get() {
			var value = TNSTextDirection.Ltr
			canvas.queueEvent {
				value = if (nativeGetDirection(canvas.nativeContext) == 1) {
					TNSTextDirection.Rtl
				} else {
					TNSTextDirection.Ltr
				}
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: Exception) {
			}
			return value
		}
		set(direction) {
			canvas.queueEvent { nativeSetDirection(canvas.nativeContext, direction.toNative()) }
		}

	fun setFillStyleWithString(color: String) {
		canvas.queueEvent {
			nativeSetFillColorWithString(canvas.nativeContext, color)
		}
	}

	var fillStyle: TNSColorStyle
		get() {
			var value: TNSColorStyle = TNSColor.black
			canvas.queueEvent {
				val style = nativeGetFillStyle(
					canvas.nativeContext
				)
				val styleValue = style.value
				value = when (style.type) {
					0 -> TNSColor(styleValue)
					1 -> TNSCanvasGradient(styleValue)
					2 -> TNSPattern(styleValue)
					else -> TNSColor.black
				}
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			canvas.queueEvent {
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
					else -> {}
				}
			}
		}

	var filter: String
		get() {
			var value = "none"
			canvas.queueEvent {
				value = nativeGetFilter(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			canvas.queueEvent { nativeSetFilter(canvas.nativeContext, value) }
		}

	fun setStrokeStyleWithString(color: String) {
		canvas.queueEvent {
			nativeSetStrokeColorWithString(canvas.nativeContext, color)
		}
	}

	var strokeStyle: TNSColorStyle
		get() {
			var value: TNSColorStyle = TNSColor.black
			canvas.queueEvent {
				val style = nativeGetStrokeStyle(
					canvas.nativeContext
				)

				val styleValue = style.value
				value = when (style.type) {
					0 -> TNSColor(styleValue)
					1 -> TNSCanvasGradient(styleValue)
					2 -> TNSPattern(styleValue)
					else -> TNSColor.black
				}
				lock.countDown()
			}

			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			canvas.queueEvent {
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
					else -> {}
				}
			}
		}

	var lineWidth: Float
		get() {
			var value = 1f
			canvas.queueEvent {
				value = nativeGetLineWidth(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: Exception) {
			}
			return value
		}
		set(lineWidth) {
			canvas.queueEvent { nativeSetLineWidth(canvas.nativeContext, lineWidth) }
		}

	var lineCap: TNSLineCap
		get() {
			var value = TNSLineCap.Butt
			canvas.queueEvent {
				value = TNSLineCap.fromNative(nativeGetLineCap(canvas.nativeContext))
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			canvas.queueEvent { nativeSetLineCap(canvas.nativeContext, value.toNative()) }
		}

	var lineJoin: TNSLineJoin
		get() {
			var value = TNSLineJoin.Bevel
			canvas.queueEvent {
				value = TNSLineJoin.fromNative(nativeGetLineJoin(canvas.nativeContext))
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			canvas.queueEvent { nativeSetLineJoin(canvas.nativeContext, value.toNative()) }
		}


	var miterLimit: Float
		get() {
			var value = 10f
			canvas.queueEvent {
				value = nativeGetMiterLimit(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: Exception) {
			}
			return value
		}
		set(limit) {
			canvas.queueEvent { nativeSetMiterLimit(canvas.nativeContext, limit) }
		}

	var lineDashOffset: Float
		get() {
			var value = 0f
			canvas.queueEvent {
				value = nativeGetLineDashOffset(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: Exception) {
			}
			return value
		}
		set(offset) {
			canvas.queueEvent { nativeSetLineDashOffset(canvas.nativeContext, offset) }
		}


	var lineDash: FloatArray
		get() {
			var value = FloatArray(0)
			canvas.queueEvent {
				value = nativeGetLineDash(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			canvas.queueEvent { nativeSetLineDash(canvas.nativeContext, value) }
		}

	var globalCompositeOperation: TNSCompositeOperationType
		get() {
			var value = TNSCompositeOperationType.SourceOver
			canvas.queueEvent {
				value =
					TNSCompositeOperationType.fromNative(nativeGetGlobalCompositeOperation(canvas.nativeContext))
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: Exception) {
			}
			return value
		}
		set(value) {
			canvas.queueEvent {
				nativeSetGlobalCompositeOperation(canvas.nativeContext, value.toNative())
			}
		}


	var globalAlpha: Float
		get() {
			var value = 1f
			canvas.queueEvent {
				value = nativeGetGlobalAlpha(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(alpha) {
			canvas.queueEvent {
				nativeSetGlobalAlpha(canvas.nativeContext, alpha)
			}
		}

	var textAlign: TNSTextAlignment
		get() {
			var value = TNSTextAlignment.Start
			canvas.queueEvent {
				value = TNSTextAlignment.fromNative(nativeGetTextAlign(canvas.nativeContext))
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(textAlign) {
			canvas.queueEvent {
				nativeSetTextAlign(canvas.nativeContext, textAlign.toNative())
			}
		}


	var shadowBlur: Float
		get() {
			var value = 0f
			canvas.queueEvent {
				value = nativeGetShadowBlur(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(blur) {
			canvas.queueEvent { nativeSetShadowBlur(canvas.nativeContext, blur) }
		}

	var shadowColor: String
		get() {
			var value = "rgba(0,0,0,0)"
			canvas.queueEvent {
				value = nativeGetShadowColor(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(color) {
			canvas.queueEvent { nativeSetShadowColorString(canvas.nativeContext, color) }
		}

	var shadowOffsetX: Float
		get() {
			var value = 0f
			canvas.queueEvent {
				value = nativeGetShadowOffsetX(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(x) {
			canvas.queueEvent { nativeSetShadowOffsetX(canvas.nativeContext, x) }
		}
	var shadowOffsetY: Float
		get() {
			var value = 0f
			canvas.queueEvent {
				value = nativeGetShadowOffsetY(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(y) {
			canvas.queueEvent { nativeSetShadowOffsetY(canvas.nativeContext, y) }
		}

	var font: String
		get() {
			var value = "10px sans-serif"
			canvas.queueEvent {
				value = nativeGetFont(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(font) {
			canvas.queueEvent { nativeSetFont(canvas.nativeContext, font) }
		}


	var imageSmoothingEnabled: Boolean
		get() {
			var value = false
			canvas.queueEvent {
				value = nativeGetImageSmoothingEnabled(canvas.nativeContext)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(enabled) {
			canvas.queueEvent {
				nativeSetImageSmoothingEnabled(canvas.nativeContext, enabled)
			}
		}

	var imageSmoothingQuality: TNSImageSmoothingQuality
		get() {
			var value = TNSImageSmoothingQuality.Low
			canvas.queueEvent {
				value =
					TNSImageSmoothingQuality.fromNative(nativeGetImageSmoothingQuality(canvas.nativeContext))
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: java.lang.Exception) {
			}
			return value
		}
		set(quality) {
			canvas.queueEvent {
				nativeSetImageSmoothingQuality(canvas.nativeContext, quality.toNative())
			}
		}


	var currentTransform: TNSDOMMatrix
		get() {
			var value: TNSDOMMatrix? = null
			canvas.queueEvent {
				val id = nativeGetCurrentTransform(canvas.nativeContext)
				value = if (id == 0L) {
					TNSDOMMatrix()
				} else TNSDOMMatrix(id)
				lock.countDown()
			}
			try {
				lock.await(2, TimeUnit.SECONDS)
				lock.reset()
			} catch (e: Exception) {
			}
			return value ?: TNSDOMMatrix()
		}
		set(matrix) {
			canvas.queueEvent {
				nativeSetCurrentTransform(canvas.nativeContext, matrix.matrix)
			}
		}

	private fun updateCanvas() {
		// synchronized (canvasView.lock) {
		canvas.invalidateState = canvas.invalidateState or TNSCanvas.INVALIDATE_STATE_PENDING
		//}
	}

	fun clearRect(x: Float, y: Float, width: Float, height: Float) {
		canvas.queueEvent {
			nativeClearRect(canvas.nativeContext, x, y, width, height)
			updateCanvas()
		}
	}

	fun fillRect(x: Float, y: Float, width: Float, height: Float) {
		canvas.queueEvent {
			nativeFillRect(canvas.nativeContext, x, y, width, height)
			updateCanvas()
		}
	}

	fun strokeRect(x: Float, y: Float, width: Float, height: Float) {
		canvas.queueEvent {
			nativeStrokeRect(canvas.nativeContext, x, y, width, height)
			updateCanvas()
		}
	}

	fun fillText(text: String, x: Float, y: Float) {
		fillText(text, x, y, 0f)
	}

	fun fillText(text: String, x: Float, y: Float, width: Float) {
		canvas.queueEvent {
			nativeFillText(canvas.nativeContext, text, x, y, width)
			updateCanvas()
		}
	}


	fun strokeText(text: String, x: Float, y: Float) {
		strokeText(text, x, y, 0f)
	}

	fun strokeText(text: String, x: Float, y: Float, width: Float) {
		canvas.queueEvent {
			nativeStrokeText(canvas.nativeContext, text, x, y, width)
			updateCanvas()
		}
	}

	fun rect(x: Float, y: Float, width: Float, height: Float) {
		canvas.queueEvent { nativeRect(canvas.nativeContext, x, y, width, height) }
	}

	fun roundRect(
		x: Float, y: Float, width: Float, height: Float,
		topLeft: Float,
		topRight: Float,
		bottomRight: Float,
		bottomLeft: Float
	) {
		canvas.queueEvent {
			nativeRoundRect(
				canvas.nativeContext,
				x,
				y,
				width,
				height,
				topLeft,
				topRight,
				bottomRight,
				bottomLeft
			)
		}
	}

	fun roundRect(
		x: Float, y: Float, width: Float, height: Float, radii: Float
	) {
		canvas.queueEvent {
			nativeRoundRect(
				canvas.nativeContext,
				x,
				y,
				width,
				height,
				radii,
				radii,
				radii,
				radii
			)
		}
	}


	fun roundRect(
		x: Float, y: Float, width: Float, height: Float, radii: FloatArray
	) {
		val size = radii.size
		if (size == 0) {
			return
		}
		canvas.queueEvent {
			/*
			[all-corners]
			[top-left-and-bottom-right, top-right-and-bottom-left]
			[top-left, top-right-and-bottom-left, bottom-right]
			[top-left, top-right, bottom-right, bottom-left]
			 */
			var topLeft = 0f
			var topRight = 0f
			var bottomRight = 0f
			var bottomLeft = 0f

			when (size) {
				1 -> {
					topLeft = radii[0]
					topRight = topLeft
					bottomRight = topLeft
					bottomLeft = topLeft
				}

				2 -> {
					topLeft = radii[0]
					topRight = radii[1]
					bottomRight = topLeft
					bottomLeft = topRight
				}
				3 -> {
					topLeft = radii[0]
					topRight = radii[1]
					bottomRight = radii[2]
					bottomLeft = topRight
				}
				4 -> {
					topLeft = radii[0]
					topRight = radii[1]
					bottomRight = radii[2]
					bottomLeft = radii[3]
				}
			}

			nativeRoundRect(
				canvas.nativeContext,
				x,
				y,
				width,
				height,
				topLeft,
				topRight,
				bottomRight,
				bottomLeft
			)
		}
	}


	fun fill() {
		fill(TNSFillRule.NonZero)
	}


	fun fill(rule: TNSFillRule) {
		fill(0, rule.toNative())
	}

	fun fill(path: TNSPath2D) {
		fill(path, TNSFillRule.NonZero)
	}

	fun fill(path: TNSPath2D, rule: TNSFillRule) {
		fill(path.path, rule.toNative())
	}

	private fun fill(path: Long, rule: Int) {
		canvas.queueEvent {
			nativeFill(canvas.nativeContext, path, rule)
			updateCanvas()
		}
	}

	fun stroke() {
		stroke(0)
	}

	fun stroke(path: TNSPath2D) {
		stroke(path.path)
	}

	private fun stroke(path: Long) {
		canvas.queueEvent {
			nativeStroke(canvas.nativeContext, path)
			updateCanvas()
		}
	}

	fun beginPath() {
		canvas.queueEvent { nativeBeginPath(canvas.nativeContext) }
	}

	fun moveTo(x: Float, y: Float) {
		canvas.queueEvent { nativeMoveTo(canvas.nativeContext, x, y) }
	}

	fun lineTo(x: Float, y: Float) {
		canvas.queueEvent { nativeLineTo(canvas.nativeContext, x, y) }
	}

	fun closePath() {
		canvas.queueEvent { nativeClosePath(canvas.nativeContext) }
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
		canvas.queueEvent {
			nativeArc(
				canvas.nativeContext,
				x,
				y,
				radius,
				startAngle,
				endAngle,
				anticlockwise
			)
		}
	}

	fun arcTo(x1: Float, y1: Float, x2: Float, y2: Float, radius: Float) {
		canvas.queueEvent { nativeArcTo(canvas.nativeContext, x1, y1, x2, y2, radius) }
	}

	fun bezierCurveTo(cp1x: Float, cp1y: Float, cp2x: Float, cp2y: Float, x: Float, y: Float) {
		canvas.queueEvent {
			nativeBezierCurveTo(
				canvas.nativeContext,
				cp1x,
				cp1y,
				cp2x,
				cp2y,
				x,
				y
			)
		}
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
		canvas.queueEvent {
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
		}
	}

	fun clip() {
		clip(TNSFillRule.NonZero)
	}

	fun clip(rule: TNSFillRule) {
		canvas.queueEvent { nativeClipRule(canvas.nativeContext, rule.toNative()) }
	}

	fun clip(path: TNSPath2D) {
		clip(path, TNSFillRule.NonZero)
	}

	fun clip(path: TNSPath2D, rule: TNSFillRule) {
		clip(path.path, rule.toNative())
	}

	private fun clip(path: Long, rule: Int) {
		canvas.queueEvent { nativeClip(canvas.nativeContext, path, rule) }
	}

	fun createLinearGradient(x0: Float, y0: Float, x1: Float, y1: Float): TNSCanvasGradient {
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
		return TNSCanvasGradient(
			nativeCreateRadialGradient(
				canvas.nativeContext,
				x0,
				y0,
				r0,
				x1,
				y1,
				r1
			)
		)
	}

	fun createConicGradient(startAngle: Float, x: Float, y: Float): TNSCanvasGradient {
		return TNSCanvasGradient(
			nativeCreateConicGradient(
				canvas.nativeContext,
				startAngle,
				x,
				y
			)
		)
	}

	fun createPattern(
		src: TNSCanvas?,
		repetition: TNSPatternRepetition = TNSPatternRepetition.Repeat
	): TNSPattern? {
		if (src == null) return null
		val ss = src.snapshot()
		return createPattern(
			ss,
			src.width,
			src.height,
			repetition.toNative()
		)
	}

	fun createPattern(
		src: Bitmap?,
		repetition: TNSPatternRepetition = TNSPatternRepetition.Repeat
	): TNSPattern? {
		if (src == null) return null
		return createPattern(
			Utils.getBytesFromBitmap(src),
			src.width,
			src.height,
			repetition.toNative()
		)
	}

	private fun createPattern(
		data: ByteArray,
		width: Int,
		height: Int,
		repetition: Int
	): TNSPattern? {
		var value: TNSPattern? = null
		canvas.queueEvent {
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
			lock.reset()
		} catch (e: Exception) {
		}
		return value
	}


	private fun createPatternEncoded(
		data: ByteArray,
		repetition: Int
	): TNSPattern? {
		var value: TNSPattern? = null
		canvas.queueEvent {
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
			lock.reset()
		} catch (e: Exception) {
		}
		return value
	}

	fun createPattern(
		src: TNSImageAsset?,
		repetition: TNSPatternRepetition = TNSPatternRepetition.Repeat
	): TNSPattern? {
		if (src == null) return null

		var value: TNSPattern? = null
		canvas.queueEvent {
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
			lock.reset()
		} catch (e: java.lang.Exception) {
		}
		return value
	}

	fun createPattern(
		src: TNSImageBitmap?,
		repetition: TNSPatternRepetition = TNSPatternRepetition.Repeat
	): TNSPattern? {
		if (src == null) return null
		var value: TNSPattern? = null
		canvas.queueEvent {
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
			lock.reset()
		} catch (e: java.lang.Exception) {
		}
		return value
	}


	fun save() {
		canvas.queueEvent {
			nativeSave(canvas.nativeContext)
		}
	}

	fun restore() {
		canvas.queueEvent {
			nativeRestore(canvas.nativeContext)
		}
	}

	fun setTransform(a: Float, b: Float, c: Float, d: Float, e: Float, f: Float) {
		canvas.queueEvent {
			nativeSetTransform(canvas.nativeContext, a, b, c, d, e, f)
		}
	}

	fun transform(a: Float, b: Float, c: Float, d: Float, e: Float, f: Float) {
		canvas.queueEvent { nativeTransform(canvas.nativeContext, a, b, c, d, e, f) }
	}

	fun scale(x: Float, y: Float) {
		canvas.queueEvent { nativeScale(canvas.nativeContext, x, y) }
	}

	fun rotate(angle: Float) {
		canvas.queueEvent { nativeRotate(canvas.nativeContext, angle) }
	}

	fun translate(x: Float, y: Float) {
		canvas.queueEvent { nativeTranslate(canvas.nativeContext, x, y) }
	}

	fun quadraticCurveTo(cpx: Float, cpy: Float, x: Float, y: Float) {
		canvas.queueEvent {
			nativeQuadraticCurveTo(canvas.nativeContext, cpx, cpy, x, y)
		}
	}

	fun drawImage(image: TNSCanvas, dx: Float, dy: Float) {
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
		canvas.queueEvent {
			nativeDrawImageDxDy(
				canvas.nativeContext,
				ss,
				finalWidth,
				finalHeight,
				dx,
				dy
			)
			updateCanvas()
		}
	}

	fun drawImage(image: Bitmap?, dx: Float, dy: Float) {
		image?.let {
			canvas.queueEvent {
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
			}
		}
	}

	fun drawImage(asset: TNSImageAsset, dx: Float, dy: Float) {
		canvas.queueEvent {
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
		canvas.queueEvent {
			nativeDrawImageDxDyWithAsset(
				canvas.nativeContext,
				bitmap.nativeImageAsset,
				dx,
				dy
			)
			updateCanvas()
		}
	}

	fun drawImage(image: TNSCanvas, dx: Float, dy: Float, dWidth: Float, dHeight: Float) {
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
		canvas.queueEvent {
			nativeDrawImageDxDyDwDy(
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
		}
	}

	fun drawImage(image: Bitmap?, dx: Float, dy: Float, dWidth: Float, dHeight: Float) {
		image?.let {
			canvas.queueEvent {
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
			}
		}
	}

	fun drawImage(asset: TNSImageAsset, dx: Float, dy: Float, dWidth: Float, dHeight: Float) {
		canvas.queueEvent {
			nativeDrawImageDxDyDwDhWithAsset(
				canvas.nativeContext,
				asset.nativeImageAsset,
				dx,
				dy,
				dWidth,
				dHeight
			)
			updateCanvas()
		}
	}

	fun drawImage(bitmap: TNSImageBitmap, dx: Float, dy: Float, dWidth: Float, dHeight: Float) {
		canvas.queueEvent {
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
		canvas.queueEvent {
			nativeDrawImage(
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
		}
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
		image?.let {
			canvas.queueEvent {
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
			}
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
		canvas.queueEvent {
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
		}
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
		canvas.queueEvent {
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
		}
	}


	fun measureText(text: String?): TNSTextMetrics {
		return TNSTextMetrics(nativeMeasureText(canvas.nativeContext, text ?: ""))
	}

	fun createImageData(width: Int, height: Int): TNSImageData {
		return TNSImageData(width, height, nativeCreateImageData(canvas.nativeContext, width, height))
	}

	fun createImageData(imageData: TNSImageData): TNSImageData {
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
		canvas.queueEvent {
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
		var value: TNSImageData? = null
		canvas.queueEvent {
			value = TNSImageData(
				sw.toInt(),
				sh.toInt(),
				nativeGetImageData(canvas.nativeContext, sx, sy, sw, sh)
			)
			lock.countDown()
		}
		try {
			lock.await(2, TimeUnit.SECONDS)
			lock.reset()
		} catch (e: Exception) {
		}
		return value ?: TNSImageData(sw.toInt(), sh.toInt(), -1)
	}


	fun resetTransform() {
		canvas.queueEvent { nativeResetTransform(canvas.nativeContext) }
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
		var value = false
		canvas.queueEvent {
			value = nativeIsPointInPath(canvas.nativeContext, path, x, y, fillRule)
			lock.countDown()
		}
		try {
			lock.await(2, TimeUnit.SECONDS)
			lock.reset()
		} catch (e: java.lang.Exception) {
		}
		return value
	}

	fun isPointInStroke(x: Float, y: Float): Boolean {
		return isPointInStroke(null, x, y)
	}

	fun isPointInStroke(path: TNSPath2D? = null, x: Float, y: Float): Boolean {
		var value = false
		canvas.queueEvent {
			value = nativeIsPointInStroke(canvas.nativeContext, path?.path ?: 0L, x, y)
			lock.countDown()
		}
		try {
			lock.await(2, TimeUnit.SECONDS)
			lock.reset()
		} catch (e: java.lang.Exception) {
		}
		return value
	}

	companion object {
		const val TAG = "CanvasRenderingContext"

		@JvmStatic
		private external fun nativeDestroy(context: Long)

		@JvmStatic
		private external fun nativeSetDirection(context: Long, direction: Int)

		@JvmStatic
		private external fun nativeGetDirection(context: Long): Int

		@JvmStatic
		private external fun nativeSetFillColorWithString(context: Long, color: String)

		@JvmStatic
		private external fun nativeSetFillStyle(context: Long, style: Long)

		@JvmStatic
		private external fun nativeGetFillStyle(context: Long): TNSColorStyleRef

		@JvmStatic
		private external fun nativeSetFilter(context: Long, filter: String)

		@JvmStatic
		private external fun nativeGetFilter(context: Long): String

		@JvmStatic
		private external fun nativeSetStrokeColorWithString(context: Long, color: String)

		@JvmStatic
		private external fun nativeSetStrokeStyle(context: Long, style: Long)

		@JvmStatic
		private external fun nativeGetStrokeStyle(context: Long): TNSColorStyleRef

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
		private external fun nativeRoundRect(
			context: Long,
			x: Float,
			y: Float,
			width: Float,
			height: Float,
			topLeft: Float,
			topRight: Float,
			bottomRight: Float,
			bottomLeft: Float
		)

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
		private external fun nativeClipRule(context: Long, rule: Int)

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
		private external fun nativeCreateConicGradient(
			context: Long,
			startAngle: Float,
			x: Float,
			y: Float,
		): Long


		@JvmStatic
		private external fun nativeCreatePatternFromContext(
			context: Long,
			repetition: Int
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
		private external fun nativeCreatePatternEncoded(
			context: Long,
			data: ByteArray,
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
	}
}
