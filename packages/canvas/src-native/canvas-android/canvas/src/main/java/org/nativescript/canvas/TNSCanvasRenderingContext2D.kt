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

	private fun ensureContextIsCurrent() {
		canvas.renderer.makeEGLContextCurrent()
	}

	var direction: TNSTextDirection
		get() {
			printLog("getDirection")
			val value = if (nativeGetDirection(canvas.nativeContext) == 1) {
				TNSTextDirection.Rtl
			} else {
				TNSTextDirection.Ltr
			}
			return value
		}
		set(direction) {
			printLog("setDirection")
			nativeSetDirection(canvas.nativeContext, direction.toNative())
		}

	fun setDirection(direction: Int) {
		if (direction in 0..1) {
			printLog("setDirection")
			nativeSetDirection(canvas.nativeContext, direction)
		}
	}

	var fillStyle: TNSColorStyle
		get() {
			printLog("getFillStyle")
			val style = nativeGetFillStyle(
				canvas.nativeContext
			)

			val value = try {
				val styleValue = style.getLong("value")
				when (style.getInt("value_type")) {
					0 -> TNSColor(styleValue)
					1 -> TNSCanvasGradient(styleValue)
					2 -> TNSPattern(styleValue)
					else -> TNSColor("black")
				}
			} catch (e: JSONException) {
				TNSColor("black")
			}

			return value
		}
		set(value) {
			printLog("setFillStyle")
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

	fun setFillStyle(color: String) {
		nativeSetFillColorWithString(canvas.nativeContext, color)
	}

	var filter: String
		get() {
			printLog("getFilter")
			return nativeGetFilter(canvas.nativeContext)
		}
		set(value) {
			printLog("setFilter")
			nativeSetFilter(canvas.nativeContext, value)
		}

	var strokeStyle: TNSColorStyle
		get() {
			printLog("getStrokeStyle")
			val style = nativeGetStrokeStyle(
				canvas.nativeContext
			)
			return try {
				val styleValue = style.getLong("value")
				when (style.getInt("value_type")) {
					0 -> TNSColor(styleValue)
					1 -> TNSCanvasGradient(styleValue)
					2 -> TNSPattern(styleValue)
					else -> TNSColor("black")
				}
			} catch (ignore: JSONException) {
				TNSColor("black")
			}
		}
		set(value) {
			printLog("setStrokeStyle")
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

	fun setStrokeStyle(color: String) {
		nativeSetStrokeColorWithString(canvas.nativeContext, color)
	}

	var lineWidth: Float
		get() {
			printLog("getLineWidth")
			return nativeGetLineWidth(canvas.nativeContext)
		}
		set(lineWidth) {
			printLog("setLineWidth")
			nativeSetLineWidth(canvas.nativeContext, lineWidth)
		}

	var lineCap: TNSLineCap
		get() {
			printLog("getLineCap")
			return TNSLineCap.fromNative(nativeGetLineCap(canvas.nativeContext))
		}
		set(value) {
			printLog("setLineCap")
			nativeSetLineCap(canvas.nativeContext, value.toNative())
		}

	fun setLineCap(cap: Int) {
		if (cap in 0..2) {
			printLog("setLineCap")
			nativeSetLineCap(canvas.nativeContext, cap)
		}
	}

	var lineJoin: TNSLineJoin
		get() {
			printLog("getLineJoin")
			return TNSLineJoin.fromNative(nativeGetLineJoin(canvas.nativeContext))
		}
		set(value) {
			printLog("setLineJoin")
			nativeSetLineJoin(canvas.nativeContext, value.toNative())
		}

	fun setLineJoin(join: Int) {
		if (join in 0..2) {
			printLog("setLineJoin")
			nativeSetLineJoin(canvas.nativeContext, join)
		}
	}


	var miterLimit: Float
		get() {
			printLog("getMiterLimit")
			return nativeGetMiterLimit(canvas.nativeContext)
		}
		set(limit) {
			printLog("setMiterLimit")
			nativeSetMiterLimit(canvas.nativeContext, limit)
		}

	var lineDashOffset: Float
		get() {
			printLog("getLineDashOffset")
			return nativeGetLineDashOffset(canvas.nativeContext)
		}
		set(offset) {
			printLog("setLineDashOffset")
			nativeSetLineDashOffset(canvas.nativeContext, offset)
		}


	var lineDash: FloatArray
		get() {
			printLog("getLineDash")
			return nativeGetLineDash(canvas.nativeContext)
		}
		set(value) {
			printLog("setLineDash")
			nativeSetLineDash(canvas.nativeContext, value)
		}

	var globalCompositeOperation: TNSCompositeOperationType
		get() {
			printLog("getGlobalCompositeOperation")
			return TNSCompositeOperationType.fromNative(nativeGetGlobalCompositeOperation(canvas.nativeContext))
		}
		set(value) {
			printLog("setGlobalCompositeOperation")
			nativeSetGlobalCompositeOperation(canvas.nativeContext, value.toNative())
		}

	fun setGlobalCompositeOperation(type: Int) {
		if (type in 0..25) {
			printLog("setGlobalCompositeOperation")
			nativeSetGlobalCompositeOperation(canvas.nativeContext, type)
		}
	}


	var globalAlpha: Float
		get() {
			printLog("getGlobalAlpha")
			val lock = CountDownLatch(1)
			return nativeGetGlobalAlpha(canvas.nativeContext)
		}
		set(alpha) {
			printLog("setGlobalAlpha")
			return nativeSetGlobalAlpha(canvas.nativeContext, alpha)
		}

	var textAlign: TNSTextAlignment
		get() {
			printLog("getTextAlign")
			return TNSTextAlignment.fromNative(nativeGetTextAlign(canvas.nativeContext))
		}
		set(textAlign) {
			printLog("setTextAlign")
			nativeSetTextAlign(canvas.nativeContext, textAlign.toNative())
		}


	fun setTextAlign(alignment: Int) {
		if (alignment in 0..4) {
			printLog("setTextAlign")
			nativeSetTextAlign(canvas.nativeContext, alignment)
		}
	}


	var shadowBlur: Float
		get() {
			printLog("getShadowBlur")
			return nativeGetShadowBlur(canvas.nativeContext)
		}
		set(blur) {
			printLog("setShadowBlur")
			nativeSetShadowBlur(canvas.nativeContext, blur)
		}

	var shadowColor: String
		get() {
			printLog("getShadowColor")
			return nativeGetShadowColor(canvas.nativeContext)
		}
		set(color) {
			printLog("setShadowColor")
			nativeSetShadowColorString(canvas.nativeContext, color)
		}

	var shadowOffsetX: Float
		get() {
			printLog("getShadowOffsetX")
			return nativeGetShadowOffsetX(canvas.nativeContext)
		}
		set(x) {
			printLog("setShadowOffsetX")
			nativeSetShadowOffsetX(canvas.nativeContext, x)
		}
	var shadowOffsetY: Float
		get() {
			printLog("getShadowOffsetY")
			return nativeGetShadowOffsetY(canvas.nativeContext)
		}
		set(y) {
			printLog("setShadowOffsetY")
			nativeSetShadowOffsetY(canvas.nativeContext, y)
		}

	var font: String
		get() {
			printLog("getFont")
			return nativeGetFont(canvas.nativeContext)
		}
		set(font) {
			printLog("setFont")
			nativeSetFont(canvas.nativeContext, font)
		}


	var imageSmoothingEnabled: Boolean
		get() {
			printLog("getImageSmoothingEnabled")
			return nativeGetImageSmoothingEnabled(canvas.nativeContext)
		}
		set(enabled) {
			printLog("setImageSmoothingEnabled")
			nativeSetImageSmoothingEnabled(canvas.nativeContext, enabled)
		}

	var imageSmoothingQuality: TNSImageSmoothingQuality
		get() {
			printLog("getImageSmoothingQuality")
			return TNSImageSmoothingQuality.fromNative(nativeGetImageSmoothingQuality(canvas.nativeContext))
		}
		set(quality) {
			printLog("setImageSmoothingQuality")
			nativeSetImageSmoothingQuality(canvas.nativeContext, quality.toNative())
		}


	var currentTransform: TNSDOMMatrix
		get() {
			printLog("getCurrentTransform")
			val id = nativeGetCurrentTransform(canvas.nativeContext)
			return if (id == 0L) {
				TNSDOMMatrix()
			} else TNSDOMMatrix(id)
		}
		set(matrix) {
			printLog("setCurrentTransform")
			nativeSetCurrentTransform(canvas.nativeContext, matrix.matrix)
		}

	private fun updateCanvas() {
		// synchronized (canvasView.lock) {
		canvas.invalidateState = GLRenderer.InvalidateState.PENDING
		//}
	}

	fun clearRect(x: Float, y: Float, width: Float, height: Float) {
		printLog("clearRect")
		ensureContextIsCurrent()
		nativeClearRect(canvas.nativeContext, x, y, width, height)
		updateCanvas()
	}

	fun fillRect(x: Float, y: Float, width: Float, height: Float) {
		printLog("fillRect")
		ensureContextIsCurrent()
		nativeFillRect(canvas.nativeContext, x, y, width, height)
		updateCanvas()
	}

	fun strokeRect(x: Float, y: Float, width: Float, height: Float) {
		printLog("strokeRect")
		ensureContextIsCurrent()
		nativeStrokeRect(canvas.nativeContext, x, y, width, height)
		updateCanvas()
	}

	fun fillText(text: String, x: Float, y: Float) {
		fillText(text, x, y, 0f)
	}

	fun fillText(text: String, x: Float, y: Float, width: Float) {
		printLog("fillText")
		ensureContextIsCurrent()
		nativeFillText(canvas.nativeContext, text, x, y, width)
		updateCanvas()
	}


	fun strokeText(text: String, x: Float, y: Float) {
		strokeText(text, x, y, 0f)
	}

	fun strokeText(text: String, x: Float, y: Float, width: Float) {
		printLog("strokeText")
		ensureContextIsCurrent()
		nativeStrokeText(canvas.nativeContext, text, x, y, width)
		updateCanvas()
	}

	fun rect(x: Float, y: Float, width: Float, height: Float) {
		printLog("rect")
		ensureContextIsCurrent()
		nativeRect(canvas.nativeContext, x, y, width, height)
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
		ensureContextIsCurrent()
		nativeFill(canvas.nativeContext, path, rule)
		updateCanvas()
	}

	fun stroke() {
		stroke(0)
	}

	fun stroke(path: TNSPath2D) {
		stroke(path.path)
	}

	private fun stroke(path: Long) {
		printLog("stroke: path")
		ensureContextIsCurrent()
		nativeStroke(canvas.nativeContext, path)
		updateCanvas()
	}

	fun beginPath() {
		printLog("beginPath")
		nativeBeginPath(canvas.nativeContext)
	}

	fun moveTo(x: Float, y: Float) {
		printLog("moveTo")
		nativeMoveTo(canvas.nativeContext, x, y)
	}

	fun lineTo(x: Float, y: Float) {
		printLog("lineTo")
		nativeLineTo(canvas.nativeContext, x, y)
	}

	fun closePath() {
		printLog("closePath")
		nativeClosePath(canvas.nativeContext)
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

	fun arcTo(x1: Float, y1: Float, x2: Float, y2: Float, radius: Float) {
		printLog("arcTo")
		nativeArcTo(canvas.nativeContext, x1, y1, x2, y2, radius)
	}

	fun bezierCurveTo(cp1x: Float, cp1y: Float, cp2x: Float, cp2y: Float, x: Float, y: Float) {
		printLog("bezierCurveTo")
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
		nativeClip(canvas.nativeContext, path, rule)
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
			val id = nativeCreatePattern(
				canvas.nativeContext,
				data, width, height, repetition
			)

			if (id != 0L) {
				value = TNSPattern(id)
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
		return value
	}


	private fun createPatternEncoded(
		data: ByteArray,
		repetition: Int
	): TNSPattern? {
		printLog("createPattern: imagebitmap")
		var value: TNSPattern? = null

		val id = nativeCreatePatternEncoded(
			canvas.nativeContext,
			data, repetition
		)

		if (id != 0L) {
			value = TNSPattern(id)
		}
		return value
	}


	private fun createPatternEncoded(
		data: ByteBuffer,
		repetition: Int
	): TNSPattern? {
		printLog("createPattern: imagebitmap")
		var value: TNSPattern? = null

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
		return value
	}


	fun createPattern(
		src: TNSImageAsset?,
		repetition: TNSPatternRepetition = TNSPatternRepetition.Repeat
	): TNSPattern? {
		printLog("createPattern: asset")
		if (src == null) return null
		var value: TNSPattern? = null

		val id = nativeCreatePatternWithAsset(
			canvas.nativeContext,
			src.nativeImageAsset, repetition.toNative()
		)
		if (id != 0L) {
			value = TNSPattern(id)
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
			val id = nativeCreatePatternWithAsset(
				canvas.nativeContext,
				src.nativeImageAsset, repetition
			)
			if (id != 0L) {
				value = TNSPattern(id)
			}
		}
		return value
	}


	fun save() {
		printLog("save")
		nativeSave(canvas.nativeContext)
	}

	fun restore() {
		printLog("restore")
		nativeRestore(canvas.nativeContext)
	}

	fun setTransform(a: Float, b: Float, c: Float, d: Float, e: Float, f: Float) {
		printLog("setTransform")
		nativeSetTransform(canvas.nativeContext, a, b, c, d, e, f)
	}

	fun transform(a: Float, b: Float, c: Float, d: Float, e: Float, f: Float) {
		printLog("transform")
		nativeTransform(canvas.nativeContext, a, b, c, d, e, f)
	}

	fun scale(x: Float, y: Float) {
		printLog("scale")
		nativeScale(canvas.nativeContext, x, y)
	}

	fun rotate(angle: Float) {
		printLog("rotate")
		nativeRotate(canvas.nativeContext, angle)
	}

	fun translate(x: Float, y: Float) {
		printLog("translate")
		nativeTranslate(canvas.nativeContext, x, y)
	}

	fun quadraticCurveTo(cpx: Float, cpy: Float, x: Float, y: Float) {
		printLog("quadraticCurveTo")
		nativeQuadraticCurveTo(canvas.nativeContext, cpx, cpy, x, y)
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
		ensureContextIsCurrent()
		nativeDrawImageDxDyWithBuffer(
			canvas.nativeContext,
			ss,
			width.toFloat(),
			height.toFloat(),
			dx,
			dy
		)
		updateCanvas()
	}

	fun drawImage(image: Bitmap?, dx: Float, dy: Float) {
		printLog("drawImage: bitmap")
		image?.let {
			val width = image.width.toFloat()
			val height = image.height.toFloat()
			ensureContextIsCurrent()
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

	fun drawImage(asset: TNSImageAsset, dx: Float, dy: Float) {
		printLog("drawImage: asset")
		ensureContextIsCurrent()
		nativeDrawImageDxDyWithAsset(
			canvas.nativeContext,
			asset.nativeImageAsset,
			dx,
			dy
		)
		updateCanvas()
	}

	fun drawImage(bitmap: TNSImageBitmap, dx: Float, dy: Float) {
		printLog("drawImage: bitmap")
		ensureContextIsCurrent()
		nativeDrawImageDxDyWithAsset(
			canvas.nativeContext,
			bitmap.nativeImageAsset,
			dx,
			dy
		)
		updateCanvas()
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
		ensureContextIsCurrent()
		nativeDrawImageDxDyDwDyWithBuffer(
			canvas.nativeContext,
			ss,
			width.toFloat(),
			height.toFloat(),
			dx,
			dy,
			dWidth,
			dHeight
		)
		updateCanvas()
	}

	fun drawImage(image: Bitmap?, dx: Float, dy: Float, dWidth: Float, dHeight: Float) {
		printLog("drawImage: bitmap")
		image?.let {
			ensureContextIsCurrent()
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

	fun drawImage(asset: TNSImageAsset, dx: Float, dy: Float, dWidth: Float, dHeight: Float) {
		printLog("drawImage: asset")
		ensureContextIsCurrent()
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

	fun drawImage(bitmap: TNSImageBitmap, dx: Float, dy: Float, dWidth: Float, dHeight: Float) {
		printLog("drawImage: bitmap")
		ensureContextIsCurrent()
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

		ensureContextIsCurrent()
		nativeDrawImageWithBuffer(
			canvas.nativeContext,
			ss,
			width.toFloat(),
			height.toFloat(),
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
			ensureContextIsCurrent()
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
		ensureContextIsCurrent()
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
		ensureContextIsCurrent()
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
		ensureContextIsCurrent()
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

	fun getImageData(sx: Float, sy: Float, sw: Float, sh: Float): TNSImageData {
		printLog("getImageData")
		return TNSImageData(
			sw.toInt(),
			sh.toInt(),
			nativeGetImageData(canvas.nativeContext, sx, sy, sw, sh)
		)
	}


	fun resetTransform() {
		printLog("resetTransform")
		nativeResetTransform(canvas.nativeContext)
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
		return nativeIsPointInPath(canvas.nativeContext, path, x, y, fillRule)
	}

	fun isPointInStroke(x: Float, y: Float): Boolean {
		return isPointInStroke(null, x, y)
	}

	fun isPointInStroke(path: TNSPath2D? = null, x: Float, y: Float): Boolean {
		printLog("isPointInStroke")
		return nativeIsPointInStroke(canvas.nativeContext, path?.path ?: 0L, x, y)
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
