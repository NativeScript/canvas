package com.github.triniwiz.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.os.Handler
import android.os.Looper
import android.util.AttributeSet
import android.view.View
import com.github.triniwiz.canvas.TNSCanvas
import java.lang.ref.WeakReference

class CPUView @JvmOverloads constructor(
	context: Context, attrs: AttributeSet? = null, defStyleAttr: Int = 0
) : View(context, attrs, defStyleAttr) {
	private var view: Bitmap? = null

	internal var canvasView: WeakReference<TNSCanvas>? = null

	override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
		super.onSizeChanged(w, h, oldw, oldh)
		if (w != 0 && h != 0) {
			view = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888)
			val canvas = canvasView!!.get()
			if (canvas != null && canvas.nativeContext == 0L) {
				val metrics = resources.displayMetrics
				canvas.nativeContext =
					TNSCanvas.nativeInitContextWithCustomSurface(
						h.toFloat(),
						w.toFloat(),
						metrics.density,
						true,
						0,
						metrics.densityDpi * 160,
						TNSCanvas.direction.toNative()
					)
				canvas.listener?.contextReady()
			}
		}
	}

	override fun onDraw(canvas: Canvas) {
		if (view != null) {
			canvas.drawBitmap(view!!, 0f, 0f, null)
		}
	}

	fun flush() {
		canvasView?.get()?.let { canvas ->
			if (canvas.nativeContext != 0L) {
				TNSCanvas.nativeCustomWithBitmapFlush(canvas.nativeContext, view)
				handler!!.post {
					invalidate()
					canvas.pendingInvalidate = false
				}
			}
		}
	}
}
