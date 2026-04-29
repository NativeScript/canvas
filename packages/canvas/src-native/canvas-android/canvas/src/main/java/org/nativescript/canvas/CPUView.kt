package org.nativescript.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Matrix
import android.util.AttributeSet
import android.util.Log
import android.view.View

class CPUView @JvmOverloads constructor(
	context: Context, attrs: AttributeSet? = null, defStyleAttr: Int = 0
) : View(context, attrs, defStyleAttr) {
	internal var bitmap: Bitmap? = null
	internal var canvas: NSCCanvas? = null
	internal var matrix = Matrix()
	override fun onDraw(canvas: Canvas) {
		bitmap?.let {
			canvas.drawBitmap(it, matrix, null)
		}
		super.onDraw(canvas)
	}

	fun render() {
		post {
			bitmap?.let { bitmap ->
				canvas?.let {
					NSCCanvas.nativeCustomWithBitmapFlush(it.nativeContext, bitmap)
				}
			}
			invalidate()
		}
	}
}
