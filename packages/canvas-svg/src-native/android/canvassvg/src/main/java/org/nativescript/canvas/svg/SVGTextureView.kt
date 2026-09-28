package org.nativescript.canvas.svg

import android.content.Context
import android.graphics.SurfaceTexture
import android.util.AttributeSet
import android.view.Surface
import android.view.TextureView

/**
 * TextureView host for the GPU path, the default, because it composites like a normal view
 * (transformable, overlappable, respects z-order) at the cost of an extra copy.
 */
class SVGTextureView : TextureView, TextureView.SurfaceTextureListener {
	internal var svg: NSCSVG? = null
	private var surface: Surface? = null

	constructor(context: Context) : super(context) {
		init()
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init()
	}

	private fun init() {
		isOpaque = false
		surfaceTextureListener = this
	}

	override fun onSurfaceTextureAvailable(texture: SurfaceTexture, width: Int, height: Int) {
		if (width == 0 || height == 0) {
			return
		}
		surface = Surface(texture)
		svg?.onSurfaceReady(surface, width, height)
	}

	override fun onSurfaceTextureSizeChanged(texture: SurfaceTexture, width: Int, height: Int) {
		if (width == 0 || height == 0) {
			return
		}
		if (surface == null) {
			surface = Surface(texture)
			svg?.onSurfaceReady(surface, width, height)
			return
		}
		svg?.onSurfaceResized(width, height)
	}

	override fun onSurfaceTextureDestroyed(texture: SurfaceTexture): Boolean {
		// Tear the GPU context down before the surface goes away, or a render still in flight
		// draws into freed memory.
		svg?.onSurfaceLost()
		surface?.release()
		surface = null
		return true
	}

	override fun onSurfaceTextureUpdated(texture: SurfaceTexture) {}
}
