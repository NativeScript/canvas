package org.nativescript.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.os.Handler
import android.os.Looper
import android.util.AttributeSet
import android.view.View
import java.lang.ref.WeakReference
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

class CPUView @JvmOverloads constructor(
	context: Context, attrs: AttributeSet? = null, defStyleAttr: Int = 0
) : View(context, attrs, defStyleAttr) {
	internal var view: Bitmap? = null
	internal var glRenderer: GLRenderer? = null

	fun resize(width: Int, height: Int) {
		if (width != 0 && height != 0 && (width != view?.width && height != view?.height)) {
			view = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
			glRenderer?.let { glRenderer ->
				if (glRenderer.nativeContext == 0L) {
					val metrics = resources.displayMetrics
					glRenderer.nativeContext = TNSCanvas.nativeInitContextWithCustomSurface(
						width.toFloat(),
						height.toFloat(),
						metrics.density,
						true,
						Color.BLACK,
						metrics.densityDpi * 160F,
						TNSCanvas.direction.toNative()
					)

					if (Looper.myLooper() != Looper.getMainLooper()) {
						glRenderer.mainHandler.post {
							glRenderer.listener?.contextReady()
						}
					} else {
						glRenderer.listener?.contextReady()
					}

				} else {
					val metrics = resources.displayMetrics
					TNSCanvas.nativeResizeCustomSurface(
						glRenderer.nativeContext,
						width.toFloat(),
						height.toFloat(),
						metrics.density,
						true,
						metrics.densityDpi * 160,
					)

					if (Looper.myLooper() != Looper.getMainLooper()) {
						glRenderer.mainHandler.post {
							invalidate()
						}
					} else {
						invalidate()
					}
				}
			}
		} else {
			view = null
		}
	}

	override fun onDraw(canvas: Canvas) {
		view?.let {
			canvas.drawBitmap(it, 0f, 0f, null)
		}
	}
}
