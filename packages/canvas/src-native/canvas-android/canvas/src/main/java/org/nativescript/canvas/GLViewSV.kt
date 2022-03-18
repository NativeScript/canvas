package org.nativescript.canvas

import android.content.Context
import android.util.AttributeSet
import android.view.SurfaceHolder
import android.view.SurfaceView

internal class GLViewSV : SurfaceView, SurfaceHolder.Callback {
	private var isCreated = false
	private var isCreatedWithZeroSized = false
	var gLContext: GLContext? = null
		private set
	private var mListener: TNSCanvas.Listener? = null

	constructor(context: Context?) : super(context) {
		init()
	}

	constructor(context: Context?, attrs: AttributeSet?) : super(context, attrs) {
		init()
	}

	fun init() {
		gLContext = GLContext()
		holder.addCallback(this)
	}

	fun flush() {
		gLContext!!.flush()
	}

	fun queueEvent(runnable: Runnable?) {
		gLContext!!.queueEvent(runnable!!)
	}

	fun setListener(listener: TNSCanvas.Listener?) {
		mListener = listener
	}

	override fun surfaceCreated(holder: SurfaceHolder) {
		if (!isCreated) {
			isCreatedWithZeroSized = true
			isCreated = true
		}
	}

	override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
		if (isCreatedWithZeroSized && (width != 0 || height != 0)) {
			gLContext!!.setTexture(holder.surface)
			isCreatedWithZeroSized = false
			if (mListener != null) {
				mListener!!.contextReady()
			}
		}
	}

	override fun surfaceDestroyed(holder: SurfaceHolder) {
		isCreated = false
	}
}
