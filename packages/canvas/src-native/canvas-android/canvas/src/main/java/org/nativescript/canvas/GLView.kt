package org.nativescript.canvas

import android.content.Context
import android.graphics.Matrix
import android.graphics.SurfaceTexture
import android.util.AttributeSet
import android.util.Log
import android.view.Surface
import android.view.TextureView
import android.view.TextureView.SurfaceTextureListener

/**
 * Created by triniwiz on 6/9/20
 */
class GLView : TextureView, SurfaceTextureListener {
	internal var isCreated = false
	internal var isCreatedWithZeroSized = false
	internal var canvas: NSCCanvas? = null
	internal var surface: Surface? = null

	constructor(context: Context) : super(context) {
		init()
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


	private fun resize() {
		canvas?.resize()
	}


	override fun onSurfaceTextureAvailable(surface: SurfaceTexture, width: Int, height: Int) {
	//	Log.d("JS", "onSurfaceTextureAvailable $surface $width $height")
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
						isReady = true
						it.listener?.contextReady()
					} else {
						resize()
					}
				}
			}
			isCreated = true
		}
	}


	override fun onSurfaceTextureSizeChanged(surface: SurfaceTexture, width: Int, height: Int) {
	//	Log.d("JS", "onSurfaceTextureSizeChanged $surface $width $height")
		if (isReady || surface == this.surface) {
			return
		}
		if (!isCreatedWithZeroSized) {
			// resize
			resize()
			return
		}
		if (isCreatedWithZeroSized && (width != 0 || height != 0)) {
			this.surface = Surface(surface)
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


	override fun onSurfaceTextureDestroyed(surface: SurfaceTexture): Boolean {
		//Log.d("JS", "onSurfaceTextureDestroyed $surface")
		isCreated = false
		return false
	}


	override fun onSurfaceTextureUpdated(surface: SurfaceTexture) {}

}
