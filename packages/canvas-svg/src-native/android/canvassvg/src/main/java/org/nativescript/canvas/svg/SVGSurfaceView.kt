package org.nativescript.canvas.svg

import android.content.Context
import android.graphics.PixelFormat
import android.util.AttributeSet
import android.view.SurfaceHolder
import android.view.SurfaceView

/**
 * SurfaceView host for the GPU path. Cheaper than [SVGTextureView] (the compositor scans the
 * buffer out directly), but it cannot be transformed or overlapped by other views, which is
 * why both exist.
 */
class SVGSurfaceView : SurfaceView, SurfaceHolder.Callback {
	internal var svg: NSCSVG? = null
	internal var isCreated = false

	constructor(context: Context) : super(context) {
		init()
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init()
	}

	private fun init() {
		holder.setFormat(PixelFormat.TRANSLUCENT)
		holder.addCallback(this)
	}

	override fun surfaceCreated(holder: SurfaceHolder) {
		isCreated = true
		// Size arrives with surfaceChanged; creating against a zero-sized surface fails.
	}

	override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
		if (width == 0 || height == 0) {
			return
		}
		svg?.onSurfaceReady(holder.surface, width, height)
	}

	override fun surfaceDestroyed(holder: SurfaceHolder) {
		isCreated = false
		svg?.onSurfaceLost()
	}
}
