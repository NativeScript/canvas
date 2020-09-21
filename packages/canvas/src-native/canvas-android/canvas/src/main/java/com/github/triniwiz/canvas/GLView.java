package com.github.triniwiz.canvas;

import android.content.Context;
import android.graphics.PixelFormat;
import android.graphics.SurfaceTexture;
import android.util.AttributeSet;
import android.util.Log;
import android.view.SurfaceHolder;
import android.view.SurfaceView;
import android.view.TextureView;

import androidx.annotation.NonNull;

import java.lang.ref.WeakReference;
import java.util.Map;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

/**
 * Created by triniwiz on 6/9/20
 */
public class GLView extends TextureView implements TextureView.SurfaceTextureListener {

  private boolean isCreated = false;
  private boolean isCreatedWithZeroSized = false;
  private GLContext mGLContext;
  private CanvasView.Listener mListener;
  int drawingBufferWidth = 0;
  int drawingBufferHeight = 0;

  public GLView(Context context) {
    super(context);
    init();
  }

  void resize(int width, int height) {
    drawingBufferWidth = width;
    drawingBufferHeight = height;
    mGLContext.resize(width, height);
  }

  public GLView(Context context, AttributeSet attrs) {
    super(context, attrs);
    init();
  }

  void init() {
    // setZOrderOnTop(true);
    // getHolder().setFormat(PixelFormat.TRANSPARENT);
    // getHolder().addCallback(this);
    setOpaque(false);
    setSurfaceTextureListener(this);
    mGLContext = new GLContext();
    mGLContext.glView = new WeakReference<>(this);
  }

  boolean starting = false;
  CountDownLatch startupLock;

  void setupContext() {
    if (mGLContext.mGLThread != null && mGLContext.mGLThread.isStarted) {
      return;
    }
    if (mGLContext.mGLThread == null) {
      mGLContext.init(null);
    }
    if (!starting) {
      starting = true;
      startupLock = new CountDownLatch(1);
      mGLContext.startGLThread();
      try {
        startupLock.await(1500, TimeUnit.MILLISECONDS);
      } catch (InterruptedException ignore) {
      } finally {
        starting = false;
        startupLock = null;
      }
    }
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
  public void onSurfaceTextureAvailable(SurfaceTexture surface, int width, int height) {
    drawingBufferHeight = height;
    drawingBufferWidth = width;
    if (!isCreated) {
      if (width == 0 || height == 0) {
        isCreatedWithZeroSized = true;
      }
      if (!isCreatedWithZeroSized) {
        mGLContext.init(surface);
        if (mListener != null) {
          mListener.contextReady();
        }
      }
      isCreated = true;
    }
  }

  @Override
  public void onSurfaceTextureSizeChanged(SurfaceTexture surface, int width, int height) {
    drawingBufferHeight = height;
    drawingBufferWidth = width;
    if (!isCreatedWithZeroSized) {
      // resize
    }
    if (isCreatedWithZeroSized && (width != 0 || height != 0)) {
      mGLContext.init(surface);
      isCreatedWithZeroSized = false;
      if (mListener != null) {
        mListener.contextReady();
      }
    }
  }

  @Override
  public boolean onSurfaceTextureDestroyed(SurfaceTexture surface) {
    //mGLContext.destroy();
    isCreated = false;
    return true;
  }

  @Override
  public void onSurfaceTextureUpdated(SurfaceTexture surface) {
  }

    /*@Override
    public void surfaceCreated(@NonNull SurfaceHolder surfaceHolder) {
        int width = surfaceHolder.getSurfaceFrame().width();
        int height = surfaceHolder.getSurfaceFrame().height();
        drawingBufferHeight = height;
        drawingBufferWidth = width;
        if (!isCreated) {
            if (width == 0 || height == 0) {
                isCreatedWithZeroSized = true;
            }
            if (!isCreatedWithZeroSized) {
                mGLContext.init(surfaceHolder.getSurface());
            }
            isCreated = true;
        }
    }


    @Override
    public void surfaceChanged(@NonNull SurfaceHolder surfaceHolder, int format, int width, int height) {
        drawingBufferHeight = height;
        drawingBufferWidth = width;
        if (!isCreatedWithZeroSized) {
            // resize
        }
        if (isCreatedWithZeroSized && (width != 0 || height != 0)) {
            mGLContext.init(surfaceHolder.getSurface());
            isCreatedWithZeroSized = false;
            if (mListener != null) {
                mListener.contextReady();
            }
        }
    }

    @Override
    public void surfaceDestroyed(@NonNull SurfaceHolder surfaceHolder) {
        isCreated = false;
    }*/

//    public void setOpaque(boolean b) {
//        if(b){
//            getHolder().setFormat(PixelFormat.OPAQUE);
//        }else {
//            getHolder().setFormat(PixelFormat.TRANSPARENT);
//        }
//    }
}
