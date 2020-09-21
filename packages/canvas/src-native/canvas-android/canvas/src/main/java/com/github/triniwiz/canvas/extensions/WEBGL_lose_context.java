package com.github.triniwiz.canvas.extensions;


import com.github.triniwiz.canvas.CanvasView;

import javax.microedition.khronos.egl.EGL10;
import javax.microedition.khronos.egl.EGLContext;

/**
 * Created by triniwiz on 5/1/20
 */
public class WEBGL_lose_context {
    CanvasView canvasView;

    public WEBGL_lose_context(CanvasView canvasView) {
        this.canvasView = canvasView;
    }

    public void loseContext() {
        EGL10 egl = (EGL10) EGLContext.getEGL();
        egl.eglMakeCurrent(EGL10.EGL_NO_DISPLAY, EGL10.EGL_NO_SURFACE, EGL10.EGL_NO_SURFACE, EGL10.EGL_NO_CONTEXT);
    }

    public void restoreContext() {
        // Force flush to make current?
        this.canvasView.flush();
    }
}
