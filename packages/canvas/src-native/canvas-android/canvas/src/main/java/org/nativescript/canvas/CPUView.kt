package org.nativescript.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.util.AttributeSet
import android.view.View
import java.lang.ref.WeakReference
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

class CPUView @JvmOverloads constructor(
	context: Context, attrs: AttributeSet? = null, defStyleAttr: Int = 0
) : View(context, attrs, defStyleAttr) {
	internal var view: Bitmap? = null
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
								metrics.densityDpi * 160F,
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
		flush(false)
	}

	fun flush(wait: Boolean) {
		view?.let {
			synchronized(lock) {
				canvasView?.get()?.let { canvas ->
					if (canvas.nativeContext != 0L) {
						val lock = if (wait) {
							CountDownLatch(1)
						} else {
							null
						}
						canvas.queueEvent {
							TNSCanvas.nativeCustomWithBitmapFlush(canvas.nativeContext, it)
							handler!!.post {
								canvas.invalidateState = TNSCanvas.InvalidateState.NONE
								invalidate()
							}
							lock?.countDown()
						}
						lock?.let {
							try {
								it.await(2, TimeUnit.SECONDS)
							} catch (ignored: InterruptedException) {
							}
						}
					}
				}
			}
		}
	}
}
