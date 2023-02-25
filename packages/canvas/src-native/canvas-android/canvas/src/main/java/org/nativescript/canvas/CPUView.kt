package org.nativescript.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.os.Looper
import android.util.AttributeSet
import android.view.View

class CPUView @JvmOverloads constructor(
	context: Context, attrs: AttributeSet? = null, defStyleAttr: Int = 0
) : View(context, attrs, defStyleAttr) {
	internal var view: Bitmap? = null

	fun resize(width: Int, height: Int) {
		val previousWidth = view?.width ?: 0
		val previousHeight = view?.height ?: 0
		if (width != 0 && height != 0 && (width != previousWidth && height != previousHeight)) {
			view = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)

			if (previousWidth == 0 && previousHeight == 0) {
				glRenderer?.listener?.contextReady()
				return
			}

		} else {
			view = null
		}

		glRenderer?.listener?.surfaceResize(width, height)
	}

	override fun onDraw(canvas: Canvas) {
		view?.let {
			canvas.drawBitmap(it, 0f, 0f, null)
		}
	}
}
