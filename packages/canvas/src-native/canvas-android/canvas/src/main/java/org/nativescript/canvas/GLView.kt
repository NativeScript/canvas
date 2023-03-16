package org.nativescript.canvas

import android.content.Context
import android.graphics.Matrix
import android.graphics.SurfaceTexture
import android.util.AttributeSet
import android.view.Surface
import android.view.TextureView
import android.view.TextureView.SurfaceTextureListener

/**
 * Created by triniwiz on 6/9/20
 */
internal class GLView : TextureView, SurfaceTextureListener {
	internal var isCreated = false
	internal var isCreatedWithZeroSized = false
	internal var canvas: NSCCanvas? = null
	internal var surface: Surface? = null
	internal var st: SurfaceTexture? = null

	constructor(context: Context) : super(context) {
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

	internal var isReady = false

	constructor(context: Context, attrs: AttributeSet?) : super(
		context, attrs
	) {
		init()
	}

	fun init() {
		isOpaque = false
		surfaceTextureListener = this
	}


	override fun onSurfaceTextureAvailable(surface: SurfaceTexture, width: Int, height: Int) {
		if (isReady) {
			return
		}
		if (!isCreated) {
			if (width == 0 || height == 0) {
				isCreatedWithZeroSized = true
			}
			if (!isCreatedWithZeroSized) {
				this.surface = Surface(surface)
				canvas?.let {
					if (!isReady) {
						it.listener?.contextReady()
						isReady = true
					}
				}
			}
			isCreated = true
		}
	}

	override fun onSurfaceTextureSizeChanged(surface: SurfaceTexture, width: Int, height: Int) {
		if (isReady) {
			return
		}
		if (!isCreatedWithZeroSized) {
			// resize
		}
		if (isCreatedWithZeroSized && (width != 0 || height != 0)) {
			if (surface == st) {
				return
			}
			this.surface = Surface(surface)
			isCreatedWithZeroSized = false
			canvas?.let {
				if (!isReady) {
					it.listener?.contextReady()
					isReady = true
				}
			}
		}
	}

	override fun onSurfaceTextureDestroyed(surface: SurfaceTexture): Boolean {
		isCreated = false
		return true
	}

	override fun onSurfaceTextureUpdated(surface: SurfaceTexture) {}
}
