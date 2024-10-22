package org.nativescript.canvas

import android.content.Context
import android.graphics.Color
import android.graphics.PixelFormat
import android.graphics.SurfaceTexture
import android.os.Build
import android.util.AttributeSet
import android.util.Log
import android.view.Surface
import android.view.SurfaceControl
import android.view.SurfaceControl.Transaction
import android.view.SurfaceHolder
import android.view.SurfaceView
import java.util.concurrent.Executors

class GLViewSV : SurfaceView, SurfaceHolder.Callback {
	var isCreated = false
	internal var isCreatedWithZeroSized = false
	internal var canvas: NSCCanvas? = null
	private var wasDestroyed = false
	internal var isReady = false
	internal lateinit var control: SurfaceControl

	constructor(context: Context) : super(context) {
		init()
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init()
	}

	fun init() {
		holder.setFormat(PixelFormat.RGBA_8888)
		setZOrderOnTop(true)
		holder.addCallback(this)

	}

	private fun resize() {
		canvas?.resize()
	}

	override fun surfaceCreated(holder: SurfaceHolder) {
		if (!isCreated) {
			isCreatedWithZeroSized = true
			isCreated = true
		}
		wasDestroyed = false
		canvas?.listener?.surfaceCreated()
	}

	override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
		if (isReady || !isCreatedWithZeroSized) {
			resize()
			return
		}

		if (width != 0 || height != 0) {
			isCreatedWithZeroSized = false
			canvas?.let {
				if (!isReady) {
					isReady = true
					it.listener?.contextReady()
				} else {
					resize()
				}
			}
		}
	}

	override fun surfaceDestroyed(holder: SurfaceHolder) {
		isCreated = false
		wasDestroyed = true
		canvas?.surfaceDestroyed()
	}
}
