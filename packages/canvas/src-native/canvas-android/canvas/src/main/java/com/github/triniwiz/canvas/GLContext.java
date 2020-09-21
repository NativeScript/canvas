package com.github.triniwiz.canvas;

import android.annotation.SuppressLint;
import android.graphics.SurfaceTexture;
import android.opengl.EGL14;
import android.opengl.EGLExt;
import android.opengl.GLES20;
import android.opengl.GLUtils;
import android.util.Log;
import android.view.Choreographer;

import java.lang.ref.WeakReference;
import java.util.Arrays;
import java.util.HashMap;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.LinkedBlockingQueue;

import javax.microedition.khronos.egl.EGL10;
import javax.microedition.khronos.egl.EGLConfig;
import javax.microedition.khronos.egl.EGLContext;
import javax.microedition.khronos.egl.EGLDisplay;
import javax.microedition.khronos.egl.EGLSurface;

import static android.os.Process.THREAD_PRIORITY_DEFAULT;
import static android.os.Process.THREAD_PRIORITY_FOREGROUND;

/**
 * Created by triniwiz on 6/9/20
 */
public class GLContext {
  public WeakReference<GLView> glView;
  private BlockingQueue<Runnable> mQueue = new LinkedBlockingQueue<>();
  GLThread mGLThread;
  final static String TAG = "GLContext";
  private EGLDisplay mEGLDisplay;
  private EGLSurface mEGLSurface;
  private SurfaceTexture offscreenTexture;
  private int textureId = 0;
  private EGLContext mEGLContext;
  private EGLConfig mEGLConfig;
  private EGL10 mEGL;

  boolean alpha = true;
  boolean antialias = true;
  boolean depth = true;
  boolean failIfMajorPerformanceCaveat = false;
  String powerPreference = "default";
  boolean premultipliedAlpha = true;
  boolean preserveDrawingBuffer = false;
  boolean stencil = false;
  boolean desynchronized = false;
  boolean xrCompatible = false;

  WeakReference<CanvasView> reference;

  public boolean isHeadless() {
    if (mGLThread != null) {
      return mGLThread.mSurface == null;
    }
    return true;
  }

  public void queueEvent(Runnable runnable) {
    mQueue.add(runnable);
  }

  public void init(Object texture) {
    if (mGLThread != null) {
      return;
    }
    mGLThread = new GLThread(texture);
    mGLThread.setPriority(Thread.MAX_PRIORITY);
  }

  void startGLThread() {
    mGLThread.start();
  }

  void resize(final int width, final int height) {
    if (reference != null) {
      final CanvasView canvasView = reference.get();
      if (canvasView != null) {
        queueEvent(new Runnable() {
          @Override
          public void run() {
            if (offscreenTexture != null) {
              GLES20.glClearColor(0, 0, 0, 0);
              GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT);
              if (!swapBuffers(mEGLSurface)) {
                Log.e("GLContext", "Cannot swap buffers!");
              }
              offscreenTexture.release();
              offscreenTexture = null;
              textureId += 1;
              offscreenTexture = new SurfaceTexture(textureId);
              offscreenTexture.setDefaultBufferSize(width, height);
              mGLThread.mSurface = offscreenTexture;

              mEGLSurface = createSurface(mEGLConfig, offscreenTexture);

              if (mEGLSurface == null || mEGLSurface == EGL10.EGL_NO_SURFACE) {
                int error = mEGL.eglGetError();
                throw new RuntimeException("eglCreateWindowSurface failed " + GLUtils.getEGLErrorString(error));
              }

              if (canvasView.canvas > 0) {
                int[] frameBuffers = new int[1];
                GLES20.glGetIntegerv(GLES20.GL_FRAMEBUFFER_BINDING, frameBuffers, 0);
                canvasView.canvas = CanvasView.nativeResize(canvasView.canvas, frameBuffers[0], canvasView.getDrawingBufferWidth(), canvasView.getDrawingBufferHeight(), canvasView.scale);
                GLES20.glViewport(0, 0, width, height);
              }
            } else {
              GLES20.glClearColor(0, 0, 0, 0);
              GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT);
              if (!swapBuffers(mEGLSurface)) {
                Log.e("GLContext", "Cannot swap buffers!");
              }
              if (canvasView.canvas > 0) {
                GLES20.glViewport(0, 0, width, height);
                int[] frameBuffers = new int[1];
                GLES20.glGetIntegerv(GLES20.GL_FRAMEBUFFER_BINDING, frameBuffers, 0);
                canvasView.canvas = CanvasView.nativeResize(canvasView.canvas, frameBuffers[0], canvasView.getDrawingBufferWidth(), canvasView.getDrawingBufferHeight(), canvasView.scale);
              }
            }
          }
        });

      }
    }
  }

  public void flush() {
    queueEvent(new Runnable() {
      @Override
      public void run() {
        if (reference != null) {
          CanvasView canvasView = reference.get();
          if (canvasView != null && canvasView.canvas != 0 && canvasView.pendingInvalidate) {
            canvasView.canvas = CanvasView.nativeFlush(canvasView.canvas);
            if (!swapBuffers(mEGLSurface)) {
              Log.e("GLContext", "Cannot swap buffers!");
            }
            canvasView.pendingInvalidate = false;
          } else {
            // WebGL
            if (!swapBuffers(mEGLSurface)) {
              Log.e("GLContext", "Cannot swap buffers!");
            }
            if (canvasView != null) {
              canvasView.pendingInvalidate = false;
            }
          }
        }
      }
    });
  }

  public EGLSurface createSurface(EGLConfig config, Object surface) {
    if (surface == null) {
      int width = 1;
      int height = 1;
      if (reference != null) {
        CanvasView view = reference.get();
        if (view != null) {
          width = view.getWidth();
          height = view.getHeight();
        }

        if (width == 0) {
          width = 1;
        }

        if (height == 0) {
          height = 1;
        }

      }
      int[] surfaceAttribs = {
        EGL10.EGL_WIDTH, width,
        EGL10.EGL_HEIGHT, height,
        EGL10.EGL_NONE
      };
      textureId += 1;
      offscreenTexture = new SurfaceTexture(textureId);
      // return mEGL.eglCreatePbufferSurface(mEGLDisplay, config, surfaceAttribs);
      offscreenTexture.setDefaultBufferSize(width, height);
      return mEGL.eglCreateWindowSurface(mEGLDisplay, config, offscreenTexture, null);
    }

    return mEGL.eglCreateWindowSurface(mEGLDisplay, config, surface, null);
  }

  public void onPause() {
    queueEvent(new Runnable() {
      @Override
      public void run() {
        if (mEGL != null) {
          mEGL.eglMakeCurrent(mEGLDisplay, EGL10.EGL_NO_SURFACE, EGL10.EGL_NO_SURFACE, EGL10.EGL_NO_CONTEXT);
        }
        if (mGLThread != null) {
          mGLThread.setPaused(false);
        }
      }
    });
  }

  public void onResume() {
    queueEvent(new Runnable() {
      @Override
      public void run() {
        if (mGLThread != null) {
          mGLThread.setPaused(false);
        }
      }
    });
  }

  public boolean makeCurrent(EGLSurface surface) {
    return mEGL.eglMakeCurrent(mEGLDisplay, surface, surface, mEGLContext);
  }

  public boolean destroySurface(EGLSurface surface) {
    return mEGL.eglDestroySurface(mEGLDisplay, surface);
  }

  public boolean swapBuffers(EGLSurface surface) {
    return mEGL.eglSwapBuffers(mEGLDisplay, surface);
  }

  public boolean isGLThreadStarted() {
    if (mGLThread == null) {
      return false;
    }
    return mGLThread.isStarted;
  }

  public void destroy() {
    if (mGLThread != null) {
      try {
        mGLThread.interrupt();
        mGLThread.join();
      } catch (InterruptedException e) {
        Log.e("GLContext", "Can't interrupt GL thread.", e);
      }
      mGLThread = null;
    }
  }

  class GLThread extends Thread {
    boolean isStarted = false;
    boolean isPaused = false;
    CanvasView.ContextType type = CanvasView.ContextType.NONE;

    public synchronized void setPaused(boolean paused) {
      isPaused = paused;
    }

    @Override
    public synchronized void start() {
      super.start();
      isStarted = true;
    }

    @Override
    public void interrupt() {
      super.interrupt();
      isStarted = false;
    }


    private Object mSurface;

    private static final int EGL_CONTEXT_CLIENT_VERSION = 0x3098;
    private static final int EGL_CONTEXT_CLIENT_MINOR_VERSION = 0x30FB;

    public GLThread() {
      mSurface = null;
    }

    public GLThread(Object texture) {
      mSurface = texture;
    }

    @SuppressLint("InlinedApi")
    private void initEGL() {
      CanvasView view = reference.get();
      mEGL = (EGL10) EGLContext.getEGL();
      mEGLDisplay = mEGL.eglGetDisplay(EGL10.EGL_DEFAULT_DISPLAY);
      if (mEGLDisplay == EGL10.EGL_NO_DISPLAY) {
        throw new RuntimeException("eglGetDisplay failed " + GLUtils.getEGLErrorString(mEGL.eglGetError()));
      }

      int[] version = new int[2];
      if (!mEGL.eglInitialize(mEGLDisplay, version)) {
        throw new RuntimeException("eglInitialize failed " + GLUtils.getEGLErrorString(mEGL.eglGetError()));
      }

      // Find a compatible EGLConfig
      int[] configsCount = new int[1];
      EGLConfig[] configs = new EGLConfig[1];
      int type = EGL14.EGL_OPENGL_ES2_BIT;
      int depthSize = 16;
      int stencilSize = 0;
      boolean useAlpha = alpha;
      boolean enableBufferPreservation = preserveDrawingBuffer;
      if (view != null) {
        if (view.glVersion > 2 && view.actualContextType.equals("webgl2")) {
          type = EGLExt.EGL_OPENGL_ES3_BIT_KHR;
        }

        if (view.contextStencil && view.contextType != CanvasView.ContextType.CANVAS) {
          stencilSize = 8;
          stencil = true;
        }

        if (!view.contextDepth || view.contextType == CanvasView.ContextType.CANVAS) {
          depth = false;
          depthSize = 0;
        }

        enableBufferPreservation = view.contextPreserveDrawingBuffer;
        useAlpha = view.contextAlpha;
        if (view.contextType == CanvasView.ContextType.CANVAS) {
          antialias = false;
        } else {
          antialias = view.contextAntialias;
        }

      }

      int[] configSpec = {
        EGL10.EGL_RENDERABLE_TYPE, type,
        EGL10.EGL_RED_SIZE, 8,
        EGL10.EGL_GREEN_SIZE, 8,
        EGL10.EGL_BLUE_SIZE, 8,
        EGL10.EGL_ALPHA_SIZE, 8,
        EGL10.EGL_DEPTH_SIZE, depthSize,
        EGL10.EGL_STENCIL_SIZE, stencilSize,
      };


      if (antialias) {
        configSpec = Arrays.copyOf(configSpec, configSpec.length + 5);
        int last = configSpec.length - 1;
        configSpec[last - 4] = EGL10.EGL_SAMPLE_BUFFERS;
        configSpec[last - 3] = 1;
        configSpec[last - 2] = EGL10.EGL_SAMPLES;
        configSpec[last - 1] = 4;
        configSpec[last] = EGL10.EGL_NONE;
      } else {
        configSpec = Arrays.copyOf(configSpec, configSpec.length + 1);
        int last = configSpec.length - 1;
        configSpec[last] = EGL10.EGL_NONE;
      }


      if (!mEGL.eglChooseConfig(mEGLDisplay, configSpec, configs, 1, configsCount)) {
        if (antialias) {
          if (view != null) {
            view.contextAntialias = false;
          }
          configSpec = new int[]{
            EGL10.EGL_RENDERABLE_TYPE, type,
            EGL10.EGL_RED_SIZE, 8,
            EGL10.EGL_GREEN_SIZE, 8,
            EGL10.EGL_BLUE_SIZE, 8,
            EGL10.EGL_ALPHA_SIZE, 8,
            EGL10.EGL_DEPTH_SIZE, depthSize,
            EGL10.EGL_STENCIL_SIZE, stencilSize,
            EGL10.EGL_NONE
          };
          boolean config = mEGL.eglChooseConfig(mEGLDisplay, configSpec, configs, 1, configsCount);
          if (config) {
            mEGLConfig = configs[0];
          }

        } else {
          throw new IllegalArgumentException("eglChooseConfig failed " + GLUtils.getEGLErrorString(mEGL.eglGetError()));
        }

      } else if (configsCount[0] > 0) {
        mEGLConfig = configs[0];
      }
      if (mEGLConfig == null) {
        throw new RuntimeException("eglConfig not initialized");
      }

      if (Utils.isEmulator()) {
        // default to es2.0 fo
        mEGLContext = createGLContext(2, 0, mEGLConfig);
      } else {

        if (view != null) {
          if (view.glVersion > 2 && view.actualContextType.equals("webgl2")) {
            if (mEGLContext == null || mEGLContext == EGL10.EGL_NO_CONTEXT) {
              mEGLContext = createGLContext(3, 0, mEGLConfig);
            }
          }
        }

        if (mEGLContext == null || mEGLContext == EGL10.EGL_NO_CONTEXT) {
          mEGLContext = createGLContext(2, 0, mEGLConfig);
        }
      }


      mEGLSurface = createSurface(mEGLConfig, mSurface);

      if (mEGLSurface == null || mEGLSurface == EGL10.EGL_NO_SURFACE) {
        int error = mEGL.eglGetError();
        throw new RuntimeException("eglCreateWindowSurface failed " + GLUtils.getEGLErrorString(error));
      }

      // Switch to our EGLContext
      makeEGLContextCurrent();

      EGL14.eglSwapInterval(EGL14.eglGetCurrentDisplay(), 0);

      if (enableBufferPreservation) {
        // Enable buffer preservation -- allows app to draw over previous frames without clearing
        boolean didEnableBufferPreservation = EGL14.eglSurfaceAttrib(EGL14.eglGetCurrentDisplay(), EGL14.eglGetCurrentSurface(EGL14.EGL_DRAW),
          EGL14.EGL_SWAP_BEHAVIOR, EGL14.EGL_BUFFER_PRESERVED);
        if (!didEnableBufferPreservation) {
          if (view != null) {
            view.contextPreserveDrawingBuffer = false;
          }
        }
      }

      GLES20.glClearColor(0, 0, 0, 0);

      int bit = GLES20.GL_COLOR_BUFFER_BIT;
      if (depth) {
        GLES20.glClearDepthf(1);
        bit |= GLES20.GL_DEPTH_BUFFER_BIT;
      }

      if (stencil) {
        GLES20.glClearStencil(0);
        bit |= GLES20.GL_STENCIL_BUFFER_BIT;
      }

      GLES20.glClear(bit);

    }

    private void deInitEGL() {
      makeEGLContextCurrent();
      destroySurface(mEGLSurface);
      mEGL.eglDestroyContext(mEGLDisplay, mEGLContext);
      mEGL.eglTerminate(mEGLDisplay);
    }

    private EGLContext createGLContext(int contextVersion, int minorVersion, EGLConfig eglConfig) {
      int[] attribs = {EGL_CONTEXT_CLIENT_VERSION, contextVersion, EGL_CONTEXT_CLIENT_MINOR_VERSION, minorVersion, EGL10.EGL_NONE};
      return mEGL.eglCreateContext(mEGLDisplay, eglConfig, EGL10.EGL_NO_CONTEXT, attribs);
    }

    private void makeEGLContextCurrent() {
      if (!mEGLContext.equals(mEGL.eglGetCurrentContext()) ||
        !mEGLSurface.equals(mEGL.eglGetCurrentSurface(EGL10.EGL_DRAW))) {
        if (!makeCurrent(mEGLSurface)) {
          //  throw new RuntimeException("eglMakeCurrent failed " + GLUtils.getEGLErrorString(mEGL.eglGetError()));
          if (BuildConfig.DEBUG) {
            Log.d(TAG, "eglMakeCurrent failed " + GLUtils.getEGLErrorString(mEGL.eglGetError()));
          }
        }
      }
    }

    @Override
    public void run() {
      initEGL();
      GLView view = glView.get();
      if (view != null && view.startupLock != null) {
        CanvasView ref = reference.get();
        if (ref != null) {
          type = ref.contextType;
          makeEGLContextCurrent();
          if (type == CanvasView.ContextType.CANVAS && ref.canvas == 0) {
            int[] frameBuffers = new int[1];
            if (view.drawingBufferWidth == 0) {
              view.drawingBufferWidth = view.getWidth();
            }

            if (view.drawingBufferHeight == 0) {
              view.drawingBufferHeight = view.getHeight();
            }

            if (view.drawingBufferWidth == 0) {
              view.drawingBufferWidth = 1;
            }

            if (view.drawingBufferHeight == 0) {
              view.drawingBufferHeight = 1;
            }

            GLES20.glViewport(0, 0, view.drawingBufferWidth, view.drawingBufferHeight);
            GLES20.glGetIntegerv(GLES20.GL_FRAMEBUFFER_BINDING, frameBuffers, 0);
            ref.canvas = CanvasView.nativeInit(false, frameBuffers[0], view.drawingBufferWidth, view.drawingBufferHeight, ref.scale, CanvasView.getDirection());
          }
        }
        view.startupLock.countDown();
      }
      while (true) {
        try {
          if (!isPaused) {
            makeEGLContextCurrent();
            mQueue.take().run();
          }
        } catch (InterruptedException e) {
          break;
        }
      }
      deInitEGL();
    }
  }
}
