package org.nativescript.canvas

import android.content.Context
import android.graphics.Matrix
import android.graphics.SurfaceTexture
import android.util.AttributeSet
import android.util.Log
import android.view.TextureView
import android.view.TextureView.SurfaceTextureListener
import java.lang.ref.WeakReference
import java.time.LocalDate
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

/**
 * Created by triniwiz on 6/9/20
 */
internal class GLView : TextureView, SurfaceTextureListener {
	private var isCreated = false
	private var isCreatedWithZeroSized = false

	var gLContext: GLContext? = null
		private set
	private var mListener: TNSCanvas.Listener? = null
	@JvmField
    var drawingBufferWidth = 0
	@JvmField
    var drawingBufferHeight = 0

	constructor(context: Context?) : super(context!!) {
		init()
	}

	private fun setScaling(){
		val matrix = Matrix()
		val density = resources.displayMetrics.density
		if(ignorePixelScaling){
			matrix.postScale(density, density)
		}
		setTransform(matrix)
	}

	var ignorePixelScaling: Boolean = false
		set(value) {
			field = value
			setScaling()
		}

	fun resize(width: Int, height: Int) {
		drawingBufferWidth = width
		drawingBufferHeight = height
		gLContext!!.resize(width, height)
	}

	constructor(context: Context?, attrs: AttributeSet?) : super(
		context!!, attrs
	) {
		init()
	}

	fun init() {
		isOpaque = false
		surfaceTextureListener = this
		gLContext = GLContext()
		gLContext!!.glView = WeakReference(this)
	}

	var starting = false
	var startupLock: CountDownLatch? = null
	fun setupContext() {
		if (gLContext!!.mGLThread != null && gLContext!!.mGLThread!!.isStarted) {
			return
		}
		if (gLContext!!.mGLThread == null) {
			gLContext!!.init(null)
		}
		if (!starting) {
			starting = true
			startupLock = CountDownLatch(1)
			gLContext!!.startGLThread()
			try {
				startupLock!!.await(1500, TimeUnit.MILLISECONDS)
			} catch (ignore: InterruptedException) {
			} finally {
				starting = false
				startupLock = null
			}
		}
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

	@Synchronized
	override fun onSurfaceTextureAvailable(surface: SurfaceTexture, width: Int, height: Int) {
		drawingBufferHeight = height
		drawingBufferWidth = width
		if (!isCreated) {
			if (width == 0 || height == 0) {
				isCreatedWithZeroSized = true
			}
			if (!isCreatedWithZeroSized) {
				gLContext?.let {
					if(it.usingOffscreenTexture && it.didInit){
						it.usingOffscreenTexture = false
						it.mGLThread!!.mSurface = surface
						it.resize(width, height, true)
					}else {
						it.init(surface)
					}
					if (mListener != null) {
						mListener!!.contextReady()
					}
				}
			}
			isCreated = true
		}
	}

	@Synchronized
	override fun onSurfaceTextureSizeChanged(surface: SurfaceTexture, width: Int, height: Int) {
		drawingBufferHeight = height
		drawingBufferWidth = width
		if (!isCreatedWithZeroSized) {
			// resize
		}
		if (isCreatedWithZeroSized && (width != 0 || height != 0)) {
			gLContext!!.init(surface)
			isCreatedWithZeroSized = false
			if (mListener != null) {
				mListener!!.contextReady()
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
