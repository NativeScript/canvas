package org.nativescript.canvas

import android.content.Context
import android.util.AttributeSet
import android.view.SurfaceHolder
import android.view.SurfaceView

internal class GLViewSV : SurfaceView, SurfaceHolder.Callback {
	private var isCreated = false
	private var isCreatedWithZeroSized = false
	internal var canvas: NSCCanvas? = null

	private fun setScaling() {
		val density = resources.displayMetrics.density
		if (ignorePixelScaling) {
			scaleX = density
			scaleY = density
		}else {
			scaleX = 1f
			scaleY = 1f
		}
	}

	var ignorePixelScaling: Boolean = false
		set(value) {
			field = value
			setScaling()
		}

	constructor(context: Context) : super(context) {
		init()
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init()
	}

	fun init() {
		holder.addCallback(this)
	}

	override fun surfaceCreated(holder: SurfaceHolder) {
		if (!isCreated) {
			isCreatedWithZeroSized = true
			isCreated = true
		}
	}

	override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
		if (isCreatedWithZeroSized && (width != 0 || height != 0)) {
			isCreatedWithZeroSized = false
			canvas?.let {
				it.listener?.contextReady()
			}
		}
	}

	override fun surfaceDestroyed(holder: SurfaceHolder) {
		isCreated = false
	}
}
