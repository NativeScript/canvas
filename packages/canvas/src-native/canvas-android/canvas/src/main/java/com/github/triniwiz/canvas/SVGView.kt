package com.github.triniwiz.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.util.AttributeSet
import android.view.View

/**
 * Created by triniwiz on 2019-08-05
 */
class SVGView : View {
	internal var bitmap: Bitmap? = null
	private var svgCanvas: Long = 0
	private var isInit = false

	constructor(context: Context?) : super(context) {}
	constructor(context: Context?, attrs: AttributeSet?) : super(context, attrs) {
		handler
	}

	override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
		if (!isInit) {
			bitmap = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888)
			svgCanvas = nativeInit(svgCanvas, bitmap)
			isInit = true
		} else {
			// scale (w / oldw) (h / oldh)
		}
	}

	override fun onDraw(canvas: Canvas) {
		canvas.drawBitmap(bitmap!!, 0f, 0f, null)
	}

	fun setSrc(src: String) {
		svgCanvas = drawSVG(svgCanvas, bitmap, src)
		invalidate()
	}

	companion object {
		@JvmStatic
		private external fun nativeInit(svgCanvas: Long, bitmap: Bitmap?): Long
		@JvmStatic
		private external fun drawSVG(svgCanvas: Long, bitmap: Bitmap?, svg: String): Long
	}
}
