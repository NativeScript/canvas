package com.github.triniwiz.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.util.AttributeSet
import android.view.View
import java.lang.ref.WeakReference

class CPUView @JvmOverloads constructor(
	context: Context, attrs: AttributeSet? = null, defStyleAttr: Int = 0
) : View(context, attrs, defStyleAttr) {
	private var view: Bitmap? = null
	internal var canvasView: WeakReference<TNSCanvas>? = null
	internal val lock = Any()
	override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
		super.onSizeChanged(w, h, oldw, oldh)
		if (w != 0 && h != 0) {
			view = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888)
			canvasView?.get()?.let { canvas ->
				synchronized(lock) {
					if (canvas.nativeContext == 0L) {
						canvas.queueEvent {
							val metrics = resources.displayMetrics
							canvas.nativeContext = TNSCanvas.nativeInitContextWithCustomSurface(
								w.toFloat(),
								h.toFloat(),
								metrics.density,
								true,
								Color.BLACK,
								metrics.densityDpi * 160,
								TNSCanvas.direction.toNative()
							)
							handler?.post {
								canvas.listener?.contextReady()
							}
						}
					} else {
						canvas.queueEvent {
							val metrics = resources.displayMetrics
							TNSCanvas.nativeResizeCustomSurface(
								canvas.nativeContext,
								w.toFloat(),
								h.toFloat(),
								metrics.density,
								true,
								metrics.densityDpi * 160,
							)
							handler?.post {
								invalidate()
							}
						}
					}
				}
			}
		}
	}

	override fun onDraw(canvas: Canvas) {
		view?.let {
			canvas.drawBitmap(it, 0f, 0f, null)
		}
	}

	fun flush() {
		view?.let {
			synchronized(lock) {
				canvasView?.get()?.let { canvas ->
					if (canvas.nativeContext != 0L) {
						canvas.queueEvent {
							TNSCanvas.nativeCustomWithBitmapFlush(canvas.nativeContext, it)
							handler!!.post {
								canvas.pendingInvalidate = false
								invalidate()
							}
						}
					}
				}
			}
		}
	}
}
