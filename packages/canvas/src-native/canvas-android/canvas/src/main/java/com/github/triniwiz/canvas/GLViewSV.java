package com.github.triniwiz.canvas;

import android.content.Context;
import android.util.AttributeSet;
import android.view.SurfaceHolder;
import android.view.SurfaceView;

public class GLViewSV extends SurfaceView implements SurfaceHolder.Callback {
    private boolean isCreated = false;
    private boolean isCreatedWithZeroSized = false;
    private GLContext mGLContext;
    private CanvasView.Listener mListener;


    public GLViewSV(Context context) {
        super(context);
        init();
    }

    public GLViewSV(Context context, AttributeSet attrs) {
        super(context, attrs);
        init();
    }


    void init() {
        mGLContext = new GLContext();
        getHolder().addCallback(this);
    }

    public void flush() {
        mGLContext.flush();
    }

    public GLContext getGLContext() {
        return mGLContext;
    }

    public void queueEvent(Runnable runnable) {
        mGLContext.queueEvent(runnable);
    }

    public void setListener(CanvasView.Listener listener) {
        this.mListener = listener;
    }


    @Override
    public void surfaceCreated(SurfaceHolder holder) {
        if (!isCreated) {
            isCreatedWithZeroSized = true;
            isCreated = true;
        }

    }

    @Override
    public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
        if (isCreatedWithZeroSized && (width != 0 || height != 0)) {
            mGLContext.init(holder.getSurface());
            isCreatedWithZeroSized = false;
            if (mListener != null) {
                mListener.contextReady();
            }
        }

    }

    @Override
    public void surfaceDestroyed(SurfaceHolder holder) {
        isCreated = false;
    }
}
