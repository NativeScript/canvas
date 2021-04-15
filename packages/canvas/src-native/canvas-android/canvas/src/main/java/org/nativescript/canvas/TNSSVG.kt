package org.nativescript.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.util.AttributeSet
import android.util.Log
import android.view.View
import java.util.concurrent.Executors

/**
 * Created by triniwiz on 2019-08-05
 */
class TNSSVG : View {
	internal var bitmap: Bitmap? = null
	private var svgCanvas: Long = 0
	private var isInit = false
	internal val lock = Any()
	private var pendingInvalidate: Boolean = false
	private val executor = Executors.newSingleThreadExecutor()
	private var src: String = ""
	private var srcPath: String = ""

	constructor(context: Context) : super(context, null) {
		init(context)
	}

	constructor(context: Context, useCpu: Boolean) : super(context, null) {
		init(context)
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init(context)
	}

	private fun init(context: Context) {
		if (isInEditMode) {
			return
		}
		if (!TNSCanvas.isLibraryLoaded) {
			System.loadLibrary("canvasnative")
			TNSCanvas.isLibraryLoaded = true
		}
	}


	override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
		super.onSizeChanged(w, h, oldw, oldh)
		if (w != 0 && h != 0) {
			val metrics = resources.displayMetrics
			bitmap = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888)
			Log.d("com.test", "bm ${Canvas(bitmap!!).maximumBitmapWidth} x ${Canvas(bitmap!!).maximumBitmapHeight}")
			if (svgCanvas == 0L) {
				executor.execute {
					synchronized(lock) {
						svgCanvas = TNSCanvas.nativeInitContextWithCustomSurface(
							w.toFloat(),
							h.toFloat(),
							metrics.density,
							true,
							Color.BLACK,
							metrics.densityDpi.toFloat(),
							TNSCanvas.direction.toNative()
						)

						if (srcPath.isNotEmpty() || src.isNotEmpty()) {

							if (srcPath.isNotEmpty()) {
								nativeDrawSVGFromPath(svgCanvas, srcPath)
							} else {
								nativeDrawSVG(svgCanvas, src)
							}
							bitmap?.let {
								TNSCanvas.nativeCustomWithBitmapFlush(svgCanvas, it)
								handler!!.post {
									pendingInvalidate = false
									invalidate()
								}
							}

						}
					}
				}
			} else {
				executor.execute {
					val metrics = resources.displayMetrics
					TNSCanvas.nativeResizeCustomSurface(
						svgCanvas,
						w.toFloat(),
						h.toFloat(),
						metrics.density,
						true,
						metrics.densityDpi,
					)

					if (srcPath.isNotEmpty() || src.isNotEmpty()) {
						if (srcPath.isNotEmpty()) {
							nativeDrawSVGFromPath(svgCanvas, srcPath)
						} else {
							nativeDrawSVG(svgCanvas, src)
						}
						bitmap?.let {
							TNSCanvas.nativeCustomWithBitmapFlush(svgCanvas, it)
							handler!!.post {
								pendingInvalidate = false
								invalidate()
							}
						}
					}
				}
			}
		}
	}

	override fun onDraw(canvas: Canvas) {
		bitmap?.let {
			canvas.drawBitmap(it, 0f, 0f, null)
		}
	}

	fun flush() {
		bitmap?.let {
			synchronized(lock) {
				if (svgCanvas != 0L) {
					executor.execute {
						TNSCanvas.nativeCustomWithBitmapFlush(svgCanvas, it)
						handler!!.post {
							pendingInvalidate = false
							invalidate()
						}
					}
				}
			}
		}
	}

	private fun doDraw() {
		synchronized(lock) {
			if (svgCanvas != 0L) {
				executor.execute {
					if (srcPath.isNotEmpty()) {
						nativeDrawSVGFromPath(svgCanvas, srcPath)
						bitmap?.let {
							if (svgCanvas != 0L) {
								executor.execute {
									TNSCanvas.nativeCustomWithBitmapFlush(svgCanvas, it)
									handler!!.post {
										pendingInvalidate = false
										invalidate()
									}
								}
							}
						}
					} else {
						nativeDrawSVG(svgCanvas, src)
						bitmap?.let {
							if (svgCanvas != 0L) {
								executor.execute {
									TNSCanvas.nativeCustomWithBitmapFlush(svgCanvas, it)
									handler!!.post {
										pendingInvalidate = false
										invalidate()
									}
								}
							}
						}
					}
				}
			}
		}
	}

	fun setSrc(src: String) {
		this.src = src
		doDraw()
	}

	fun setSrcPath(path: String) {
		this.srcPath = path
		doDraw()
	}

	companion object {
		@JvmStatic
		private external fun nativeDrawSVG(svgCanvas: Long, svg: String)

		@JvmStatic
		private external fun nativeDrawSVGFromPath(svgCanvas: Long, svg: String)
	}
}
