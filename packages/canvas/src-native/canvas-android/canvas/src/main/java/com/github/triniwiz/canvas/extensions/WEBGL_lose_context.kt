package com.github.triniwiz.canvas.extensions

import com.github.triniwiz.canvas.TNSCanvas
import javax.microedition.khronos.egl.EGL10
import javax.microedition.khronos.egl.EGLContext

/**
 * Created by triniwiz on 5/1/20
 */
class WEBGL_lose_context(var canvasView: TNSCanvas) {
	fun loseContext() {
		val egl = EGLContext.getEGL() as EGL10
		egl.eglMakeCurrent(
			EGL10.EGL_NO_DISPLAY,
			EGL10.EGL_NO_SURFACE,
			EGL10.EGL_NO_SURFACE,
			EGL10.EGL_NO_CONTEXT
		)
	}

	fun restoreContext() {
		// Force flush to make current?
		canvasView.flush()
	}
}
