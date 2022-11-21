package org.nativescript.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Matrix
import android.graphics.Rect
import android.util.AttributeSet
import android.view.View
import android.widget.ImageView.ScaleType
import java.lang.ref.WeakReference

class CPUView @JvmOverloads constructor(
	context: Context, attrs: AttributeSet? = null, defStyleAttr: Int = 0
) : View(context, attrs, defStyleAttr) {
	private var view: Bitmap? = null
	internal var canvasView: WeakReference<TNSCanvas>? = null
	internal val lock = Any()

	var scaleType = TNSCanvas.ScaleType.None

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
		view?.let {
			synchronized(lock) {
				canvasView?.get()?.let { canvas ->
					if (canvas.nativeContext != 0L) {
						canvas.queueEvent {
							// We don't want pending flags that were set up to this point
							canvas.invalidateState = canvas.invalidateState and TNSCanvas.INVALIDATE_STATE_PENDING.inv()
							TNSCanvas.nativeCustomWithBitmapFlush(canvas.nativeContext, it)
							handler!!.post {
								canvas.invalidateState = canvas.invalidateState and TNSCanvas.INVALIDATE_STATE_INVALIDATING.inv()
								invalidate()
							}
						}
					}
				}
			}
		}
	}

//	private fun intoMatrix(): Matrix? {
//		if (scaleType == TNSCanvas.ScaleType.None){return null}
//		val matrix = Matrix()
//		val rect = Rect(left, top, right, bottom)
//
//		return when(scaleType){
//			TNSCanvas.ScaleType.AspectFill -> TODO()
//			TNSCanvas.ScaleType.AspectFit -> TODO()
//			TNSCanvas.ScaleType.Fill -> TODO()
//			else -> null
//		}
//	}
}
