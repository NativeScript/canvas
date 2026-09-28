package org.nativescript.canvas

import android.content.Context
import android.graphics.SurfaceTexture
import android.util.AttributeSet
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
		isCreated = true
		if (width == 0 || height == 0) {
			isCreatedWithZeroSized = true
			return
		}
		this.surface = Surface(surface)
		attach()
	}


	override fun onSurfaceTextureSizeChanged(surface: SurfaceTexture, width: Int, height: Int) {
		if (isCreatedWithZeroSized && (width != 0 || height != 0)) {
			this.surface = Surface(surface)
			isCreatedWithZeroSized = false
			attach()
			return
		}
		resize()
	}

	private fun attach() {
		canvas?.let {
			if (!isReady) {
				isReady = true
				postOnAnimation {
					it.listener?.contextReady()
				}
			}
			it.resize()
		}
	}


	override fun onSurfaceTextureDestroyed(surface: SurfaceTexture): Boolean {
		isCreated = false
		// Mirror GLViewSV.surfaceDestroyed(): let the canvas rebind its EGL
		// context to an offscreen surface before the window surface actually
		// goes away, so a render loop that's still firing (Choreographer/
		// requestAnimationFrame) doesn't run against a torn-down surface.
		// Previously this only released the local Surface wrapper, leaving the
		// native context bound to a dead surface until finalize() eventually
		// ran — the TextureView path never got the offscreen-rebind protection
		// the SurfaceView path already has.
		canvas?.surfaceDestroyed()
		this.surface?.release()
		this.surface = null
		return false
	}


	override fun onSurfaceTextureUpdated(surface: SurfaceTexture) {}

}
