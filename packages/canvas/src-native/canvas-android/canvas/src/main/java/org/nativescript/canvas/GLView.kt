package org.nativescript.canvas

import android.content.Context
import android.graphics.Matrix
import android.graphics.SurfaceTexture
import android.util.AttributeSet
import android.view.Surface
import android.view.TextureView
import android.view.TextureView.SurfaceTextureListener
import java.lang.ref.WeakReference

/**
 * Created by triniwiz on 6/9/20
 */
internal class GLView : TextureView, SurfaceTextureListener {
	internal var isCreated = false
	private var isCreatedWithZeroSized = false
	internal var canvas: NSCCanvas? = null
	internal var surface: Surface? = null

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

	private var isReady = false

	constructor(context: Context, attrs: AttributeSet?) : super(
		context, attrs
	) {
		init()
	}

	fun init() {
		isOpaque = false
		surfaceTextureListener = this
	}

	@Synchronized
	override fun onSurfaceTextureAvailable(surface: SurfaceTexture, width: Int, height: Int) {
		if (!isCreated) {
			if (width == 0 || height == 0) {
				isCreatedWithZeroSized = true
			}
			if (!isCreatedWithZeroSized) {
				this.surface = Surface(surfaceTexture)

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

	@Synchronized
	override fun onSurfaceTextureSizeChanged(surface: SurfaceTexture, width: Int, height: Int) {
		if (!isCreatedWithZeroSized) {
			// resize
		}
		if (isCreatedWithZeroSized && (width != 0 || height != 0)) {
			this.surface = Surface(surfaceTexture)
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
		//mGLContext.destroy();
		isCreated = false
		return true
	}

	override fun onSurfaceTextureUpdated(surface: SurfaceTexture) {}
}
