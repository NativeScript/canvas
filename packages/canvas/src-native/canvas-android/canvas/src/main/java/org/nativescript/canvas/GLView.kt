package org.nativescript.canvas

import android.content.Context
import android.graphics.Matrix
import android.graphics.SurfaceTexture
import android.util.AttributeSet
import android.util.Log
import android.view.TextureView
import android.view.TextureView.SurfaceTextureListener
import java.lang.Exception
import java.lang.ref.WeakReference
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

/**
 * Created by triniwiz on 6/9/20
 */
internal class GLView : TextureView, SurfaceTextureListener {
	internal var isCreated = false
	private var isCreatedWithZeroSized = false

	internal var renderer: WeakReference<GLRenderer>? = null

	private var mListener: TNSCanvas.Listener? = null

	constructor(context: Context?) : super(context!!) {
		init()
	}

	private fun setScaling() {
		val matrix = Matrix()
		val density = resources.displayMetrics.density
		if (ignorePixelScaling) {
			matrix.postScale(density, density)
		}
		setTransform(matrix)
	}

	var ignorePixelScaling: Boolean = false
		set(value) {
			field = value
			setScaling()
		}

	private var isReady = false


	fun resize(width: Int, height: Int) {
		renderer?.get()?.drawingBufferWidth = width
		renderer?.get()?.drawingBufferHeight = height
	}

	constructor(context: Context?, attrs: AttributeSet?) : super(
		context!!, attrs
	) {
		init()
	}

	fun init() {
		isOpaque = false
		surfaceTextureListener = this
	}

	fun setListener(listener: TNSCanvas.Listener?) {
		mListener = listener
	}

	@Synchronized
	override fun onSurfaceTextureAvailable(surface: SurfaceTexture, width: Int, height: Int) {
		renderer?.get()?.drawingBufferHeight = height
		renderer?.get()?.drawingBufferWidth = width
		if (!isCreated) {
			if (width == 0 || height == 0) {
				isCreatedWithZeroSized = true
			}
			if (!isCreatedWithZeroSized) {
				renderer?.get()?.setTexture(surface)
				if (mListener != null && !isReady) {
					mListener!!.contextReady()
					isReady = true
				}
			}
			isCreated = true
		}
	}

	@Synchronized
	override fun onSurfaceTextureSizeChanged(surface: SurfaceTexture, width: Int, height: Int) {
		renderer?.get()?.drawingBufferHeight = height
		renderer?.get()?.drawingBufferWidth = width
		if (!isCreatedWithZeroSized) {
			// resize
		}
		if (isCreatedWithZeroSized && (width != 0 || height != 0)) {
			renderer?.get()?.setTexture(surface)
			isCreatedWithZeroSized = false
			if (mListener != null && !isReady) {
				mListener!!.contextReady()
				isReady = true
			}
		}
	}

	override fun onSurfaceTextureDestroyed(surface: SurfaceTexture): Boolean {
		//mGLContext.destroy();
		isCreated = false
		return true
	}

	override fun onSurfaceTextureUpdated(surface: SurfaceTexture) {}
}
