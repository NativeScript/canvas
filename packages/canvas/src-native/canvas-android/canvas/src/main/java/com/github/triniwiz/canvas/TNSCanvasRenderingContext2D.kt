package com.github.triniwiz.canvas

import android.graphics.Bitmap
import android.util.Log
import kotlinx.coroutines.runBlocking
import org.json.JSONException
import org.json.JSONObject
import kotlin.coroutines.resume
import kotlin.coroutines.suspendCoroutine

/**
 * Created by triniwiz on 2019-07-06
 */
class TNSCanvasRenderingContext2D internal constructor(val canvas: TNSCanvas) :
	TNSCanvasRenderingContext {

	@Throws(Throwable::class)
	protected fun finalize() {
		nativeDestroy(canvas.nativeContext)
	}

	var direction: TNSTextDirection
		get() {
			return runBlocking {
				val result = suspendCoroutine<TNSTextDirection> {
					canvas.queueEvent(Runnable {
						val value = if (nativeGetDirection(canvas.nativeContext) == 1) {
							TNSTextDirection.Rtl
						} else {
							TNSTextDirection.Ltr
						}
						it.resume(value)
					})
				}
				result
			}
		}
		set(direction) {
			canvas.queueEvent(Runnable { nativeSetDirection(canvas.nativeContext, direction.toNative()) })
		}

	var fillStyle: TNSColorStyle
		get() {
			return runBlocking {
				suspendCoroutine<TNSColorStyle> {
					canvas.queueEvent(Runnable {
						val style = nativeGetFillStyle(
							canvas.nativeContext
						)
						var value: TNSColorStyle = TNSColor("black")
						try {
							val styleValue = style.getLong("value")
							value = when (style.getInt("value_type")) {
								0 -> TNSColor(styleValue)
								1 -> TNSCanvasGradient(styleValue)
								2 -> TNSPattern(styleValue)
								else -> TNSColor("black")
							}
						} catch (e: JSONException) {
						}
						it.resume(value)
					})
				}
			}
		}
		set(value) {
			printLog("setFillStyle")
			canvas.queueEvent(Runnable {
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
			})
		}

	var strokeStyle: TNSColorStyle
		get() {
			return runBlocking {
				suspendCoroutine<TNSColorStyle> {
					canvas.queueEvent(Runnable {
						val style = nativeGetStrokeStyle(
							canvas.nativeContext
						)
						var value: TNSColorStyle = TNSColor("black")
						try {
							val styleValue = style.getLong("value")
							when (style.getInt("value_type")) {
								0 -> value = TNSColor(styleValue)
								1 -> value = TNSCanvasGradient(styleValue)
								2 -> value = TNSPattern(styleValue)
								else -> value = TNSColor("black")
							}
						} catch (ignore: JSONException) {
						}
						it.resume(value)
					})
				}
			}
		}
		set(value) {
			printLog("setFillStyle")
			canvas.queueEvent(Runnable {
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
			})
		}

	var lineWidth: Float
		get() {
			return runBlocking {
				suspendCoroutine<Float> {
					canvas.queueEvent(Runnable {
						val value = nativeGetLineWidth(canvas.nativeContext)
						it.resume(value)
					})
				}
			}
		}
		set(lineWidth) {
			printLog("setLineWidth")
			canvas.queueEvent(Runnable { nativeSetLineWidth(canvas.nativeContext, lineWidth) })
		}

	var lineCap: TNSLineCap
		get() {
			return runBlocking {
				suspendCoroutine<TNSLineCap> {
					canvas.queueEvent(Runnable {
						val value = nativeGetLineCap(canvas.nativeContext)
						it.resume(TNSLineCap.fromNative(value))
					})
				}
			}
		}
		set(value) {
			printLog("setLineCap")
			canvas.queueEvent(Runnable { nativeSetLineCap(canvas.nativeContext, value.toNative()) })
		}

	var lineJoin: TNSLineJoin
		get() {
			return runBlocking {
				suspendCoroutine<TNSLineJoin> {
					canvas.queueEvent(Runnable {
						val value = nativeGetLineJoin(canvas.nativeContext)
						it.resume(TNSLineJoin.fromNative(value))
					})
				}
			}
		}
		set(value) {
			printLog("setLineJoin")
			canvas.queueEvent(Runnable { nativeSetLineJoin(canvas.nativeContext, value.toNative()) })
		}


	var miterLimit: Float
		get() {
			return runBlocking {
				suspendCoroutine<Float> {
					canvas.queueEvent(Runnable {
						val value = nativeGetMiterLimit(canvas.nativeContext)
						it.resume(value)
					})
				}
			}
		}
		set(limit) {
			printLog("setMiterLimit")
			canvas.queueEvent(Runnable { nativeSetMiterLimit(canvas.nativeContext, limit) })
		}

	var lineDashOffset: Float
		get() {
			return runBlocking {
				suspendCoroutine<Float> {
					canvas.queueEvent(Runnable {
						val value = nativeGetLineDashOffset(canvas.nativeContext)
						it.resume(value)
					})
				}
			}
		}
		set(offset) {
			printLog("setLineDashOffset")
			canvas.queueEvent(Runnable { nativeSetLineDashOffset(canvas.nativeContext, offset) })
		}


	var lineDash: FloatArray
		get() {
			return runBlocking {
				suspendCoroutine<FloatArray> {
					canvas.queueEvent(Runnable {
						val value = nativeGetLineDash(canvas.nativeContext)
						it.resume(value)
					})
				}
			}
		}
		set(value) {
			printLog("setLineDash")
			canvas.queueEvent(Runnable { nativeSetLineDash(canvas.nativeContext, value) })
		}

	var globalCompositeOperation: TNSCompositeOperationType
		get() {
			return runBlocking {
				suspendCoroutine<TNSCompositeOperationType> {
					canvas.queueEvent(Runnable {
						val value = nativeGetGlobalCompositeOperation(canvas.nativeContext)
						it.resume(TNSCompositeOperationType.fromNative(value))
					})
				}
			}
		}
		set(value) {
			printLog("setGlobalCompositeOperation")
			canvas.queueEvent(Runnable {
				nativeSetGlobalCompositeOperation(canvas.nativeContext, value.toNative())
			})
		}


	var globalAlpha: Float
		get() {
			return runBlocking {
				suspendCoroutine<Float> {
					canvas.queueEvent(Runnable {
						val value = nativeGetGlobalAlpha(canvas.nativeContext)
						it.resume(value)
					})
				}
			}
		}
		set(alpha) {
			printLog("setGlobalAlpha")
			canvas.queueEvent(Runnable {
				nativeSetGlobalAlpha(canvas.nativeContext, globalAlpha)
			})
		}

	var textAlign: TNSTextAlignment
		get() {
			return runBlocking {
				suspendCoroutine<TNSTextAlignment> {
					canvas.queueEvent(Runnable {
						val value = nativeGetTextAlign(canvas.nativeContext)
						it.resume(TNSTextAlignment.fromNative(value))
					})
				}
			}
		}
		set(textAlign) {
			printLog("setTextAlign")
			canvas.queueEvent(Runnable {
				nativeSetTextAlign(canvas.nativeContext, textAlign.toNative())
			})
		}


	var shadowBlur: Float
		get() {
			return runBlocking {
				suspendCoroutine<Float> {
					canvas.queueEvent(Runnable {
						val value = nativeGetShadowBlur(canvas.nativeContext)
						it.resume(value)
					})
				}
			}
		}
		set(blur) {
			printLog("setShadowBlur")
			canvas.queueEvent(Runnable { nativeSetShadowBlur(canvas.nativeContext, blur) })
		}

	var shadowColor: String
		get() {
			return runBlocking {
				suspendCoroutine<String> {
					canvas.queueEvent(Runnable {
						val value = nativeGetShadowColor(canvas.nativeContext)
						it.resume(value)
					})
				}
			}
		}
		set(color) {
			printLog("setShadowColor")
			canvas.queueEvent(Runnable { nativeSetShadowColor(canvas.nativeContext, color) })
		}

	var shadowOffsetX: Float
		get() {
			return runBlocking {
				suspendCoroutine<Float> {
					canvas.queueEvent(Runnable {
						val value = nativeGetShadowOffsetX(canvas.nativeContext)
						it.resume(value)
					})
				}
			}
		}
		set(x) {
			printLog("setShadowOffsetX")
			canvas.queueEvent(Runnable { nativeSetShadowOffsetX(canvas.nativeContext, x) })
		}
	var shadowOffsetY: Float
		get() {
			return runBlocking {
				suspendCoroutine<Float> {
					canvas.queueEvent(Runnable {
						val value = nativeGetShadowOffsetY(canvas.nativeContext)
						it.resume(value)
					})
				}
			}
		}
		set(y) {
			printLog("setShadowOffsetY")
			canvas.queueEvent(Runnable { nativeSetShadowOffsetY(canvas.nativeContext, y) })
		}

	var font: String
		get() {
			return runBlocking {
				suspendCoroutine<String> {
					canvas.queueEvent(Runnable {
						val value = nativeGetFont(canvas.nativeContext)
						it.resume(value)
					})
				}
			}
		}
		set(font) {
			printLog("setFont")
			canvas.queueEvent(Runnable { nativeSetFont(canvas.nativeContext, font) })
		}


	var imageSmoothingEnabled: Boolean
		get() {
			return runBlocking {
				suspendCoroutine<Boolean> {
					canvas.queueEvent(Runnable {
						val value = nativeGetImageSmoothingEnabled(canvas.nativeContext)
						it.resume(value)
					})
				}
			}
		}
		set(enabled) {
			printLog("setImageSmoothingEnabled")
			canvas.queueEvent(Runnable {
				nativeSetImageSmoothingEnabled(canvas.nativeContext, enabled)
			})
		}

	var imageSmoothingQuality: TNSImageSmoothingQuality
		get() {
			return runBlocking {
				suspendCoroutine<TNSImageSmoothingQuality> {
					canvas.queueEvent(Runnable {
						val value = nativeGetImageSmoothingQuality(canvas.nativeContext)
						it.resume(TNSImageSmoothingQuality.fromNative(value))
					})
				}
			}
		}
		set(quality) {
			printLog("setImageSmoothingQuality")
			canvas.queueEvent(Runnable {
				nativeSetImageSmoothingQuality(canvas.nativeContext, quality.toNative())
			})
		}


	var currentTransform: TNSDOMMatrix
		get() {
			printLog("getCurrentTransform")

			return runBlocking {
				suspendCoroutine<TNSDOMMatrix> {
					canvas.queueEvent(Runnable {
						val id = nativeGetCurrentTransform(canvas.nativeContext)
						it.resume(
							if (id == 0L) {
								TNSDOMMatrix()
							} else TNSDOMMatrix(id)
						)
					})
				}
			}
		}
		set(matrix) {
			printLog("setCurrentTransform")
			canvas.queueEvent(Runnable {
				nativeSetCurrentTransform(canvas.nativeContext, matrix.matrix)
			})
		}

	private fun updateCanvas() {
		// synchronized (canvasView.lock) {
		canvas.pendingInvalidate = true
		//}
	}

	fun clearRect(x: Float, y: Float, width: Float, height: Float) {
		printLog("clearRect")
		canvas.queueEvent(Runnable {
			nativeClearRect(canvas.nativeContext, x, y, width, height)
			updateCanvas()
		})
	}

	fun fillRect(x: Float, y: Float, width: Float, height: Float) {
		printLog("fillRect")
		canvas.queueEvent(Runnable {
			nativeFillRect(canvas.nativeContext, x, y, width, height)
			updateCanvas()
		})
	}

	fun strokeRect(x: Float, y: Float, width: Float, height: Float) {
		printLog("strokeRect")
		canvas.queueEvent(Runnable {
			nativeStrokeRect(canvas.nativeContext, x, y, width, height)
			updateCanvas()
		})
	}

	@JvmOverloads
	fun fillText(text: String, x: Float, y: Float, width: Float = 0f) {
		printLog("fillText")
		canvas.queueEvent(Runnable {
			nativeFillText(canvas.nativeContext, text, x, y, width)
			updateCanvas()
		})
	}

	@JvmOverloads
	fun strokeText(text: String, x: Float, y: Float, width: Float = 0f) {
		printLog("strokeText")
		canvas.queueEvent(Runnable {
			nativeStrokeText(canvas.nativeContext, text, x, y, width)
			updateCanvas()
		})
	}

	fun rect(x: Float, y: Float, width: Float, height: Float) {
		printLog("rect")
		canvas.queueEvent(Runnable { nativeRect(canvas.nativeContext, x, y, width, height) })
	}

	@JvmOverloads
	fun fill(rule: TNSFillRule = TNSFillRule.NonZero) {
		fill(0, rule.toNative())
	}

	@JvmOverloads
	fun fill(path: TNSPath2D, rule: TNSFillRule = TNSFillRule.NonZero) {
		fill(path.path, rule.toNative())
	}

	private fun fill(path: Long, rule: Int) {
		printLog("fill: path rule")
		canvas.queueEvent(Runnable {
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
		canvas.queueEvent(Runnable {
			nativeStroke(canvas.nativeContext, path)
			updateCanvas()
		})
	}

	fun beginPath() {
		printLog("beginPath")
		canvas.queueEvent(Runnable { nativeBeginPath(canvas.nativeContext) })
	}

	fun moveTo(x: Float, y: Float) {
		printLog("moveTo")
		canvas.queueEvent(Runnable { nativeMoveTo(canvas.nativeContext, x, y) })
	}

	fun lineTo(x: Float, y: Float) {
		printLog("lineTo")
		canvas.queueEvent(Runnable { nativeLineTo(canvas.nativeContext, x, y) })
	}

	fun closePath() {
		printLog("closePath")
		canvas.queueEvent(Runnable { nativeClosePath(canvas.nativeContext) })
	}

	@JvmOverloads
	fun arc(
		x: Float,
		y: Float,
		radius: Float,
		startAngle: Float,
		endAngle: Float,
		anticlockwise: Boolean = false
	) {
		printLog("arc")
		canvas.queueEvent(Runnable {
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
		canvas.queueEvent(Runnable { nativeArcTo(canvas.nativeContext, x1, y1, x2, y2, radius) })
	}

	fun bezierCurveTo(cp1x: Float, cp1y: Float, cp2x: Float, cp2y: Float, x: Float, y: Float) {
		printLog("bezierCurveTo")
		canvas.queueEvent(Runnable {
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

	@JvmOverloads
	fun ellipse(
		x: Float,
		y: Float,
		radiusX: Float,
		radiusY: Float,
		rotation: Float,
		startAngle: Float,
		endAngle: Float,
		anticlockwise: Boolean = false
	) {
		printLog("ellipse")
		canvas.queueEvent(Runnable {
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

	fun clip(path: TNSPath2D) {
		printLog("clip: path")
		clip(path, TNSFillRule.NonZero)
	}

	fun clip(path: TNSPath2D, rule: TNSFillRule) {
		printLog("clip: path rule")
		clip(path.path, rule.toNative())
	}

	private fun clip(path: Long, rule: Int) {
		printLog("clip: path rule")
		canvas.queueEvent(Runnable { nativeClip(canvas.nativeContext, path, rule) })
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

	fun createPattern(
		src: TNSCanvas?,
		repetition: TNSPatternRepetition = TNSPatternRepetition.Repeat
	): TNSPattern? {
		printLog("createPattern: canvas")
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
		printLog("createPattern: bitmap")
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
		return runBlocking {
			suspendCoroutine<TNSPattern?> {
				canvas.queueEvent(Runnable {
					val value = nativeCreatePattern(
						canvas.nativeContext,
						data, width, height, repetition
					)
					if (value == 0L) {
						it.resume(null)
					} else {
						it.resume(TNSPattern(value))
					}
				})
			}
		}
	}

	fun createPattern(
		src: TNSImageAsset?,
		repetition: TNSPatternRepetition = TNSPatternRepetition.Repeat
	): TNSPattern? {
		printLog("createPattern: asset")
		if (src == null) return null
		return runBlocking {
			suspendCoroutine<TNSPattern?> {
				canvas.queueEvent(Runnable {
					val value = nativeCreatePatternWithAsset(
						canvas.nativeContext,
						src.nativeImageAsset, repetition.toNative()
					)
					if (value == 0L) {
						it.resume(null)
					} else {
						it.resume(TNSPattern(value))
					}
				})
			}
		}
	}


	fun save() {
		printLog("save")
		canvas.queueEvent(Runnable {
			nativeSave(canvas.nativeContext)
		})
	}

	fun restore() {
		printLog("restore")
		canvas.queueEvent(Runnable {
			nativeRestore(canvas.nativeContext)
		})
	}

	fun setTransform(a: Float, b: Float, c: Float, d: Float, e: Float, f: Float) {
		printLog("setTransform")
		canvas.queueEvent(Runnable {
			nativeSetTransform(canvas.nativeContext, a, b, c, d, e, f)
		})
	}

	fun transform(a: Float, b: Float, c: Float, d: Float, e: Float, f: Float) {
		printLog("transform")
		canvas.queueEvent(Runnable { nativeTransform(canvas.nativeContext, a, b, c, d, e, f) })
	}

	fun scale(x: Float, y: Float) {
		printLog("scale")
		canvas.queueEvent(Runnable { nativeScale(canvas.nativeContext, x, y) })
	}

	fun rotate(angle: Float) {
		printLog("rotate")
		canvas.queueEvent(Runnable { nativeRotate(canvas.nativeContext, angle) })
	}

	fun translate(x: Float, y: Float) {
		printLog("translate")
		canvas.queueEvent(Runnable { nativeTranslate(canvas.nativeContext, x, y) })
	}

	fun quadraticCurveTo(cpx: Float, cpy: Float, x: Float, y: Float) {
		printLog("quadraticCurveTo")
		canvas.queueEvent(Runnable {
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
		canvas.queueEvent(Runnable {
			nativeDrawImageDxDy(
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
			canvas.queueEvent(Runnable {
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
		canvas.queueEvent(Runnable {
			nativeDrawImageDxDyWithAsset(
				canvas.nativeContext,
				asset.nativeImageAsset,
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
		canvas.queueEvent(Runnable {
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
		})
	}

	fun drawImage(image: Bitmap?, dx: Float, dy: Float, dWidth: Float, dHeight: Float) {
		printLog("drawImage: bitmap")
		image?.let {
			canvas.queueEvent(Runnable {
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
		canvas.queueEvent(Runnable {
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
		canvas.queueEvent(Runnable {
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
			canvas.queueEvent(Runnable {
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
		canvas.queueEvent(Runnable {
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

	@JvmOverloads
	fun putImageData(
		data: TNSImageData,
		x: Float,
		y: Float,
		dirtyX: Float = 0f,
		dirtyY: Float = 0f,
		dirtyWidth: Float = 0f,
		dirtyHeight: Float = 0f
	) {
		canvas.queueEvent(Runnable {
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
		})
	}

	fun getImageData(sx: Float, sy: Float, sw: Float, sh: Float): TNSImageData {
		printLog("getImageData")
		return runBlocking {
			suspendCoroutine<TNSImageData> {
				canvas.queueEvent(Runnable {
					val value = nativeGetImageData(canvas.nativeContext, sx, sy, sw, sh)
					it.resume(
						TNSImageData(sw.toInt(), sh.toInt(), value)
					)
				})
			}
		}
	}


	fun resetTransform() {
		printLog("resetTransform")
		canvas.queueEvent(Runnable { nativeResetTransform(canvas.nativeContext) })
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
		return runBlocking {
			suspendCoroutine<Boolean> {
				canvas.queueEvent(Runnable {
					val value = nativeIsPointInPath(canvas.nativeContext, path, x, y, fillRule)
					it.resume(value)
				})
			}
		}
	}

	fun isPointInStroke(x: Float, y: Float): Boolean {
		return isPointInStroke(null, x, y)
	}

	fun isPointInStroke(path: TNSPath2D? = null, x: Float, y: Float): Boolean {
		printLog("isPointInStroke")
		return runBlocking {
			suspendCoroutine<Boolean> {
				canvas.queueEvent(Runnable {
					val value = nativeIsPointInStroke(canvas.nativeContext, path?.path ?: 0L, x, y)
					it.resume(value)
				})
			}
		}
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
		private external fun nativeGetFillStyle(context: Long): JSONObject

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
		private external fun nativeSetShadowColor(context: Long, color: String)

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

		var isDebug = true
	}
}
