package org.nativescript.canvas.svg

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Matrix
import android.util.AttributeSet
import android.util.Log
import android.view.View
import java.util.concurrent.Executors


class NSCSVG : View {
	internal var bitmap: Bitmap? = null
	internal val lock = Any()
	private var pendingInvalidate: Boolean = false
	private val executor = Executors.newSingleThreadExecutor()
	private var src: String = ""
	private var srcPath: String = ""
	private var mMatrix = Matrix()

	var sync: Boolean = false

	constructor(context: Context) : super(context, null) {
		init(context)
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init(context)
	}

	private fun init(context: Context) {
		if (isInEditMode) {
			return
		}
	}

	private var currentTask: java.util.concurrent.Future<*>? = null
	private fun resize(w: Int, h: Int) {
		doDraw()
	}


	override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
		super.onSizeChanged(w, h, oldw, oldh)
		if (w != 0 && h != 0) {

			bitmap = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888)

			resize(w, h)
		}
	}

	override fun onDraw(canvas: Canvas) {
		bitmap?.let {
			canvas.drawBitmap(it, mMatrix, null)
		}
	}

	private fun doDraw() {
		bitmap?.let {
			currentTask?.cancel(true)
			if (sync) {
				if (srcPath.isNotEmpty()) {
					nativeDrawSVGFromPath(it, srcPath)
					pendingInvalidate = false
					invalidate()
				} else {
					nativeDrawSVG(it, src)
					pendingInvalidate = false
					invalidate()
				}
				return
			}

			synchronized(lock) {
				currentTask = executor.submit {
					if (srcPath.isNotEmpty()) {
						nativeDrawSVGFromPath(it, srcPath)
						handler?.post {
							pendingInvalidate = false
							invalidate()
						}
					} else {
						nativeDrawSVG(it, src)
						handler?.post {
							pendingInvalidate = false
							invalidate()
						}
					}
					currentTask = null
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
		init {
			try {
				System.loadLibrary("canvassvg")
			} catch (_: Exception) {
			}
		}

		@JvmStatic
		private external fun nativeDrawSVG(bitmap: Bitmap, svg: String)

		@JvmStatic
		private external fun nativeDrawSVGFromPath(bitmap: Bitmap, svg: String)
	}
}
