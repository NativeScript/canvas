package com.github.triniwiz.canvas;

import android.app.Activity;
import android.app.ActivityManager;
import android.app.Application;
import android.content.Context;
import android.content.pm.ConfigurationInfo;
import android.graphics.Bitmap;
import android.graphics.Color;
import android.graphics.drawable.ColorDrawable;
import android.opengl.GLES20;
import android.os.Build;
import android.os.Bundle;
import android.os.Environment;
import android.os.Handler;
import android.os.HandlerThread;
import android.os.Looper;
import android.util.AttributeSet;
import android.util.Base64;
import android.util.Log;
import android.view.Choreographer;
import android.view.SurfaceHolder;
import android.view.ViewGroup;
import android.widget.FrameLayout;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.core.text.TextUtilsCompat;
import androidx.core.view.ViewCompat;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.lang.ref.WeakReference;
import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Locale;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.CountDownLatch;

import javax.microedition.khronos.opengles.GL10;


/**
 * Created by triniwiz on 3/29/20
 */
public class CanvasView extends FrameLayout implements Choreographer.FrameCallback, Application.ActivityLifecycleCallbacks {

  static ConcurrentHashMap views = new ConcurrentHashMap();

  public static ConcurrentHashMap getViews() {
    return views;
  }

  static native long nativeInit(boolean useCpu, int id, int width, int height, float scale, String direction);

  static native long nativeResize(long canvas_ptr, int id, int width, int height, float scale);

  static native long nativeRecreate(long canvas_ptr, int id, int width, int height, float scale);

  private static native long nativeDeInit(long canvas_ptr);

  private static native long nativeDestroy(long canvas_ptr);

  static native long nativeFlush(long canvas_ptr);

  static native long nativeCpuFlush(long canvas_ptr, Bitmap view);

  private static native String nativeToDataUrl(long canvas_ptr, String type, float quality);

  private static native byte[] nativeToData(long canvas_ptr);

  private static native byte[] nativeSnapshotCanvas(long canvas_ptr);

  private static native ByteBuffer nativeSnapshotCanvasBuffer(long canvas_ptr);


  private HandlerThread handlerThread = new HandlerThread("CanvasViewThread");
  private Handler handler;

  GLView glView;
  CPUView cpuView;
  private boolean handleInvalidationManually = false;
  volatile long canvas = 0;
  CanvasRenderingContext renderingContext2d = null;
  float scale = 0;
  Context ctx;
  boolean pendingInvalidate;
  final Object lock = new Object();
  final static long ONE_MILLISECOND_NS = 1000000;
  static final long ONE_S_IN_NS = 1000 * ONE_MILLISECOND_NS;
  Handler mainHandler;
  boolean wasPendingDraw = false;
  static long lastCall = 0;
  ContextType contextType = ContextType.NONE;
  String actualContextType = "";
  boolean useCpu = false;
  Map<String, Object> contextAttributes = new HashMap<>();


  boolean contextAlpha = true;
  boolean contextAntialias = true;
  boolean contextDepth = true;
  boolean contextFailIfMajorPerformanceCaveat = false;
  String contextPowerPreference = "default";
  boolean contextPremultipliedAlpha = true;
  boolean contextPreserveDrawingBuffer = false;
  boolean contextStencil = false;
  boolean contextDesynchronized = false;
  boolean contextXrCompatible = false;

  int mClearStencil;
  float[] mClearColor = new float[]{0, 0, 0, 0};
  boolean mScissorEnabled = false;
  float mClearDepth = 1;
  boolean[] mColorMask = new boolean[]{true, true, true, true};
  int mStencilMask = 0xFFFFFFFF;
  int mStencilMaskBack = 0xFFFFFFFF;
  int mStencilFuncRef = 0;
  int mStencilFuncRefBack = 0;
  int mStencilFuncMask = 0xFFFFFFFF;
  int mStencilFuncMaskBack = 0xFFFFFFFF;
  boolean mDepthMask = true;

  void clear() {
    queueEvent(new Runnable() {
      @Override
      public void run() {
        GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT);
      }
    });
  }

  Bitmap view;

  @Override
  public void doFrame(long frameTimeNanos) {
    if (!handleInvalidationManually) {
      if (pendingInvalidate) {
        flush();
      }
    }
    Choreographer.getInstance().postFrameCallback(this);
  }

  public CanvasView(Context context) {
    super(context, null);
    init(context, false);
  }

  public CanvasView(Context context, boolean useCpu) {
    super(context, null);
    init(context, useCpu);
  }

  private static boolean isLibraryLoaded = false;
  int glVersion;

  int previousOrientation = 0;

  public CanvasView(Context context, @Nullable AttributeSet attrs) {
    super(context, attrs);
    init(context, false);
  }

  private void init(Context context, boolean useCpu) {
    if (isInEditMode()) {
      return;
    }
    this.useCpu = useCpu;
    if (!isLibraryLoaded) {
      System.loadLibrary("canvasnative");
      isLibraryLoaded = true;
    }
    setBackgroundColor(Color.TRANSPARENT);
    glView = new GLView(context);
    glView.getGLContext().reference = new WeakReference<>(this);
    handlerThread.start();
    handler = new Handler(handlerThread.getLooper());
    mainHandler = new Handler(Looper.getMainLooper());
    ctx = context;
    scale = context.getResources().getDisplayMetrics().density;
    if (detectOpenGLES30() && !Utils.isEmulator()) {
      glVersion = 3;
    } else {
      glVersion = 2;
    }

    glView.setLayoutParams(
      new LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.MATCH_PARENT)
    );
    if (useCpu) {
      cpuView = new CPUView(context);
      cpuView.canvasView = new WeakReference<>(this);
      cpuView.setLayoutParams(
        new LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.MATCH_PARENT)
      );
      initCPUThread();
      addView(cpuView);
    } else {
      addView(glView);
    }

  }

  private void initCPUThread() {
    cpuHandlerThread = new HandlerThread("CanvasViewCpuThread");
    cpuHandlerThread.start();
    cpuHandler = new Handler(cpuHandlerThread.getLooper());
  }

  int getDrawingBufferWidth() {
    if (useCpu) {
      return cpuView.getWidth();
    }
    return glView.drawingBufferWidth;
  }

  int getDrawingBufferHeight() {
    if (useCpu) {
      return cpuView.getHeight();
    }
    return glView.drawingBufferHeight;
  }


  static final String TAG = "CanvasView";

  private boolean detectOpenGLES30() {
    ActivityManager am =
      (ActivityManager) getContext().getSystemService(Context.ACTIVITY_SERVICE);
    if (am == null) {
      return false;
    }
    ConfigurationInfo info = am.getDeviceConfigurationInfo();
    return (info.reqGlEsVersion >= 0x30000);
  }

  public void onPause() {
    if (glView != null) {
      glView.getGLContext().onPause();
    }
  }

  public void onResume() {
    if (glView != null) {
      glView.getGLContext().onResume();
    }
  }

  public void destroy() {
    nativeDestroy(canvas);
    canvas = 0;
  }

  @Override
  protected void finalize() throws Throwable {
    super.finalize();
    if (canvas != 0) {
      destroy();
    }
  }

  public void setHandleInvalidationManually(boolean handleInvalidationManually) {
    this.handleInvalidationManually = handleInvalidationManually;
  }

  public boolean isHandleInvalidationManually() {
    return handleInvalidationManually;
  }

  public GLView getSurface() {
    return glView;
  }

  Handler cpuHandler;
  HandlerThread cpuHandlerThread;

  public void queueEvent(final Runnable runnable) {
    if (useCpu) {
      if (!cpuHandlerThread.isAlive() || cpuHandlerThread.isInterrupted()) {
        cpuHandlerThread = null;
        cpuHandler = null;
        initCPUThread();
      }
      cpuHandler.post(runnable);
    } else {
      glView.queueEvent(runnable);
    }
  }

  public void setupActivityHandler(Application app) {
    app.unregisterActivityLifecycleCallbacks(this);
    app.registerActivityLifecycleCallbacks(this);
  }

  @Override
  public void onActivityCreated(@NonNull Activity activity, @Nullable Bundle savedInstanceState) {

  }

  @Override
  public void onActivityStarted(@NonNull Activity activity) {

  }

  @Override
  public void onActivityResumed(@NonNull Activity activity) {
    //onResume();
  }

  @Override
  public void onActivityPaused(@NonNull Activity activity) {
  }

  @Override
  public void onActivityStopped(@NonNull Activity activity) {
    //onPause();
  }

  @Override
  public void onActivitySaveInstanceState(@NonNull Activity activity, @NonNull Bundle outState) {

  }

  @Override
  public void onActivityDestroyed(@NonNull Activity activity) {

  }

  public interface DataURLListener {
    void onResult(String data);
  }


  public byte[] toData() {
    if (contextType == ContextType.CANVAS) {
      final CountDownLatch lock = new CountDownLatch(1);
      final byte[][] data = new byte[1][];
      queueEvent(new Runnable() {
        @Override
        public void run() {
          data[0] = nativeToData(canvas);
          lock.countDown();
        }
      });
      try {
        lock.await();
      } catch (InterruptedException ignore) {
      }
      return data[0];
    } else if (contextType == ContextType.WEBGL) {
      Bitmap bm = glView.getBitmap(getWidth(), getHeight());
      byte[] data = new byte[bm.getWidth() * bm.getHeight() * 4];
      ByteBuffer buffer = ByteBuffer.wrap(data);
      bm.copyPixelsToBuffer(buffer);
      return data;
    }

    return new byte[0];
  }

  byte[] snapshot() {
    if (contextType == ContextType.CANVAS) {
      final CountDownLatch lock = new CountDownLatch(1);
      final ArrayList<byte[]> ss = new ArrayList<>();
      // initCanvas();
      queueEvent(new Runnable() {
        @Override
        public void run() {
          ss.add(nativeSnapshotCanvas(canvas));
          lock.countDown();
        }
      });
      try {
        lock.await();
      } catch (InterruptedException ignore) {

      }
      return ss.get(0);
    } else if (contextType == ContextType.WEBGL) {
      Bitmap bm = glView.getBitmap(getWidth(), getHeight());
      ByteArrayOutputStream os = new ByteArrayOutputStream();
      bm.compress(Bitmap.CompressFormat.PNG, 100, os);
      return os.toByteArray();
    }

    return new byte[0];
  }

  public String toDataURL() {
    return toDataURL("image/png");
  }


  public void toDataURLAsync(DataURLListener listener) {
    toDataURLAsync("image/png", listener);
  }

  public void toDataURLAsync(String type, DataURLListener listener) {
    toDataURLAsync(type, 0.92f, listener);
  }

  public void toDataURLAsync(final String type, final float quality, final DataURLListener listener) {
    queueEvent(new Runnable() {
      @Override
      public void run() {
        listener.onResult(nativeToDataUrl(canvas, type, quality));
      }
    });
  }

  public String toDataURL(String type) {
    return toDataURL(type, 0.92f);
  }

  public String toDataURL(final String type, final float quality) {
    if (contextType == ContextType.WEBGL) {
      Bitmap bm = glView.getBitmap(getWidth(), getHeight());
      ByteArrayOutputStream os = new ByteArrayOutputStream();
      String dataType = "image/png";
      switch (type) {
        case "image/jpeg":
          dataType = "image/jpeg";
          break;
        case "image/jpg":
          dataType = "image/jpg";
          break;
        default:
          break;
      }

      if (bm != null) {
        if (type.equals("image/jpeg") || type.equals("image/jpg")) {
          bm.compress(Bitmap.CompressFormat.JPEG, (int) (quality * 100), os);
        } else {
          bm.compress(Bitmap.CompressFormat.PNG, (int) (quality * 100), os);
        }
      }
      return String.format("data:%s;base64,%s", dataType, Base64.encodeToString(os.toByteArray(), Base64.NO_WRAP));
    }
    final CountDownLatch lock = new CountDownLatch(1);
    final String[] data = new String[1];
    queueEvent(new Runnable() {
      @Override
      public void run() {
        data[0] = nativeToDataUrl(canvas, type, quality);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignore) {
    }
    return data[0];
  }


  public static CanvasDOMMatrix createSVGMatrix() {
    return new CanvasDOMMatrix();
  }

  WebGLRenderingContext webGLRenderingContext;
  WebGL2RenderingContext webGL2RenderingContext;

  enum ContextType {
    NONE,
    CANVAS,
    WEBGL
  }

  int needRenderRequest = 0;

  public void resizeViewPort() {
    queueEvent(new Runnable() {
      @Override
      public void run() {
        GLES20.glViewport(0, 0, getWidth(), getHeight());
      }
    });
  }

  static String getDirection() {
    String direction = "ltr";
    if (TextUtilsCompat.getLayoutDirectionFromLocale(Locale.getDefault()) == ViewCompat.LAYOUT_DIRECTION_RTL) {
      direction = "rtl";
    }
    return direction;
  }

  void initCanvas() {
    if (glView != null && canvas == 0) {
      int width = glView.getWidth();
      int height = glView.getHeight();
      // dynamically created view w/o layout
      ViewGroup.LayoutParams params = glView.getLayoutParams();
      if (width == 0 && params != null) {
        width = params.width;
        needRenderRequest += 1;
      }

      if (height == 0 && params != null) {
        height = params.height;
        needRenderRequest += 1;
      }
      // size was not set set to 1
      if (width == 0) {
        width = 1;
        needRenderRequest += 1;
      }
      if (height == 0) {
        height = 1;
        needRenderRequest += 1;
      }

      if (newSize == null || newSize.width == 0 && newSize.height == 0) {
        newSize = new Size(width, height);
      }


      final int finalWidth = width;
      final int finalHeight = height;

      glView.queueEvent(new Runnable() {
        @Override
        public void run() {
          if (canvas == 0 && finalWidth > 0 && finalHeight > 0) {
            // GLES20.glClearColor(1F, 1F, 1F, 1F);
            // GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT);
            int[] frameBuffers = new int[1];
            GLES20.glViewport(0, 0, finalWidth, finalHeight);
            GLES20.glGetIntegerv(GLES20.GL_FRAMEBUFFER_BINDING, frameBuffers, 0);
            canvas = nativeInit(useCpu, frameBuffers[0], finalWidth, finalHeight, scale, getDirection());
          }
        }
      });
    }
  }

  public @Nullable
  CanvasRenderingContext getContext(String type) {
    HashMap<String, Object> attributes = new HashMap<>();
    if (type.equals("2d")) {
      attributes.put("alpha", true);
      attributes.put("desynchronized", false);
    } else if (type.contains("webgl")) {
      attributes.put("alpha", true);
      attributes.put("depth", true);
      attributes.put("failIfMajorPerformanceCaveat", false);
      attributes.put("powerPreference", "default");
      attributes.put("premultipliedAlpha", true);
      attributes.put("preserveDrawingBuffer", false);
      attributes.put("stencil", false);
      attributes.put("xrCompatible", false);
      attributes.put("desynchronized", false);
    }
    return getContext(type, attributes);
  }


  private void handleAttributes(@Nullable Map<String, Object> contextAttributes) {
    if (contextAttributes != null) {
      Set<String> keys = contextAttributes.keySet();
      for (String key : keys) {
        Object value = contextAttributes.get(key);
        switch (key) {
          case "alpha":
            contextAlpha = (boolean) value;
          case "depth":
            contextDepth = (boolean) value;
          case "failIfMajorPerformanceCaveat":
            contextFailIfMajorPerformanceCaveat = (boolean) value;
          case "premultipliedAlpha":
            contextPremultipliedAlpha = (boolean) value;
          case "preserveDrawingBuffer":
            contextPreserveDrawingBuffer = (boolean) value;
          case "stencil":
            contextStencil = (boolean) value;
          case "xrCompatible":
            contextXrCompatible = (boolean) value;
          case "desynchronized":
            contextDesynchronized = (boolean) value;
            break;
          case "powerPreference":
            contextPowerPreference = (String) value;
            break;
          default:
            // NOOP
        }
      }
    }
  }

  public @Nullable
  CanvasRenderingContext getContext(String type, @Nullable Map<String, Object> contextAttributes) {
    handleAttributes(contextAttributes);
    if (type.equals("2d") || type.equals("experimental-webgl") || type.equals("webgl") || (type.equals("webgl2") && Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2)) {
      handler.post(new Runnable() {
        @Override
        public void run() {
          glView.setOpaque(!contextAlpha);
        }
      });
      if (type.equals("2d")) {
        contextType = ContextType.CANVAS;
      }
      if (renderingContext2d == null && webGLRenderingContext == null && webGL2RenderingContext == null) {
        glView.setupContext();
      }
    }
    switch (type) {
      case "2d":
        actualContextType = "2d";
        if (renderingContext2d == null) {
          renderingContext2d = new CanvasRenderingContext2D(this);
        }
        contextType = ContextType.CANVAS;
        return renderingContext2d;
      case "webgl":
      case "experimental-webgl":
        actualContextType = "webgl";
        if (webGLRenderingContext == null) {
          webGLRenderingContext = new WebGLRenderingContext(this);
        }
        contextType = ContextType.WEBGL;
        return webGLRenderingContext;
      case "webgl2":
        if (webGL2RenderingContext == null) {
          if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2) {
            actualContextType = "webgl";
            webGL2RenderingContext = new WebGL2RenderingContext(this);
            isWebGL = true;
            contextType = ContextType.WEBGL;
          } else {
            isWebGL = false;
            contextType = ContextType.NONE;
            return null;
          }
        }

        return webGL2RenderingContext;
    }
    contextType = ContextType.NONE;
    return null;
  }


  @Override
  protected void onSizeChanged(int w, int h, int oldw, int oldh) {
    glView.resize(w, h);
  }

  boolean isPaused;

  @Override
  protected void onDetachedFromWindow() {
    super.onDetachedFromWindow();
    isPaused = true;
    Choreographer.getInstance().removeFrameCallback(this);
  }

  @Override
  protected void onAttachedToWindow() {
    super.onAttachedToWindow();
    isPaused = false;
    Choreographer.getInstance().postFrameCallback(this);
  }

  SurfaceHolder holder;

  public interface Listener {
    public void contextReady();
  }

  Listener listener;

  public void setListener(Listener listener) {
    this.listener = listener;
    if (glView != null) {
      glView.setListener(listener);
    }
  }

  int mWidth = -1;
  int mHeight = -1;
  int textureId = 0;
  int renderCount = 0;
  boolean wasDestroyed = false;
  boolean isWebGL = false;

  static class Size {
    int width;
    int height;

    Size(int width, int height) {
      this.width = width;
      this.height = height;
    }

    public int getHeight() {
      return height;
    }

    public int getWidth() {
      return width;
    }
  }


  Size lastSize;
  Size newSize;

  public void flush() {
    if (useCpu) {
      cpuView.flush();
    } else {
      glView.flush();
    }
  }

  void showForeground() {
    mainHandler.post(new Runnable() {
      @Override
      public void run() {
        CanvasView.this.setForeground(new ColorDrawable(Color.WHITE));
      }
    });
  }

  void hideForeground() {
    mainHandler.post(new Runnable() {
      @Override
      public void run() {
        CanvasView.this.setForeground(null);
      }
    });
  }

  boolean readyEventSent = false;

}
