package com.github.triniwiz.canvas;

import android.graphics.Bitmap;
import android.opengl.GLES20;
import android.os.Build;
import android.util.Log;

import androidx.annotation.Nullable;

import com.github.triniwiz.canvas.extensions.ANGLE_instanced_arrays;
import com.github.triniwiz.canvas.extensions.EXT_blend_minmax;
import com.github.triniwiz.canvas.extensions.EXT_color_buffer_half_float;
import com.github.triniwiz.canvas.extensions.EXT_disjoint_timer_query;
import com.github.triniwiz.canvas.extensions.EXT_sRGB;
import com.github.triniwiz.canvas.extensions.EXT_shader_texture_lod;
import com.github.triniwiz.canvas.extensions.EXT_texture_filter_anisotropic;
import com.github.triniwiz.canvas.extensions.OES_element_index_uint;
import com.github.triniwiz.canvas.extensions.OES_standard_derivatives;
import com.github.triniwiz.canvas.extensions.OES_texture_float;
import com.github.triniwiz.canvas.extensions.OES_texture_float_linear;
import com.github.triniwiz.canvas.extensions.OES_texture_half_float;
import com.github.triniwiz.canvas.extensions.OES_texture_half_float_linear;
import com.github.triniwiz.canvas.extensions.OES_vertex_array_object;
import com.github.triniwiz.canvas.extensions.WEBGL_color_buffer_float;
import com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_atc;
import com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_etc;
import com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_etc1;
import com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_pvrtc;
import com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_s3tc;
import com.github.triniwiz.canvas.extensions.WEBGL_depth_texture;
import com.github.triniwiz.canvas.extensions.WEBGL_draw_buffers;
import com.github.triniwiz.canvas.extensions.WEBGL_lose_context;

import java.io.UnsupportedEncodingException;
import java.lang.reflect.Array;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.nio.LongBuffer;
import java.nio.ShortBuffer;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.CountDownLatch;

import static java.nio.charset.StandardCharsets.UTF_8;

/**
 * Created by triniwiz on 4/21/20
 */
public class WebGLRenderingContext implements CanvasRenderingContext {

  private CanvasView canvas;
  final static int SIZE_OF_BYTE = 1;
  final static int SIZE_OF_SHORT = 2;
  final static int SIZE_OF_INT = 4;
  final static int SIZE_OF_LONG = 8;
  final static int SIZE_OF_FLOAT = 4;
  final static int SIZE_OF_DOUBLE = 2;
  final static int SIZE_OF_CHAR = 2;

  native ByteBuffer nativeBufferFromBitmap(Bitmap bitmap, int bytesPerPixel, boolean flipY);

  native byte[] nativeBytesFromBitmap(Bitmap bitmap, int bytesPerPixel, boolean flipY);

  native void nativeFlipInPlace(byte[] storage, int bytesPerPixel, int height);

  native void nativeVertexAttribPointer(int index, int size, int type, boolean normalized, int stride, int offset);

  native void nativeGetVertexAttribOffset(int index, int pname, ByteBuffer buffer);

  native void nativeTexImage2DAsset(int target, int level, int internalformat, int width, int height, int border, int format, int type, long asset, boolean flipY);

  native void nativeTexSubImage2DAsset(int target,
                                       int level,
                                       int xoffset,
                                       int yoffset,
                                       int width,
                                       int height,
                                       int format,
                                       int image_type,
                                       long asset,
                                       boolean flipY);

  public WebGLRenderingContext(CanvasView canvas) {
    this.canvas = canvas;
  }

  public WebGLRenderingContext(CanvasView canvas, Map<String, Object> attrs) {
    this.canvas = canvas;
  }

  public CanvasView getCanvas() {
    return canvas;
  }

  public int getDrawingBufferWidth() {
    return canvas.getDrawingBufferWidth();
  }

  public int getDrawingBufferHeight() {
    return canvas.getDrawingBufferHeight();
  }

  void runOnGLThread(final Runnable runnable) {
    canvas.queueEvent(runnable);
  }

  void updateCanvas() {
    // synchronized (canvasView.lock) {
    canvas.pendingInvalidate = true;
    //}
  }

  final int GL_UNSIGNED_BYTE = 0x1401;
  final int GL_FLOAT = 0x1406;
  final int GL_HALF_FLOAT = 0x140B;
  final int GL_UNSIGNED_SHORT_5_6_5 = 0x8363;
  final int GL_UNSIGNED_SHORT_4_4_4_4 = 0x8033;
  final int GL_UNSIGNED_SHORT_5_5_5_1 = 0x8034;
  final int GL_LUMINANCE = 0x1909;
  final int GL_ALPHA = 0x1906;
  final int GL_LUMINANCE_ALPHA = 0x190A;
  final int GL_RGB = 0x1907;
  final int GL_RGBA = 0x1908;

  int bytesPerPixel(int pixel_type, int format) {
    int bytes_per_component = 0;
    switch (pixel_type) {
      case GL_UNSIGNED_BYTE:
        bytes_per_component = 1;
        break;
      case GL_FLOAT:
        bytes_per_component = 4;
        break;
      case GL_HALF_FLOAT:
        bytes_per_component = 2;
        break;
      case GL_UNSIGNED_SHORT_5_6_5:
      case GL_UNSIGNED_SHORT_4_4_4_4:
      case GL_UNSIGNED_SHORT_5_5_5_1:
        return 2;
    }

    switch (format) {
      case GL_LUMINANCE:
      case GL_ALPHA:
        return bytes_per_component;
      case GL_LUMINANCE_ALPHA:
        return 2 * bytes_per_component;
      case GL_RGB:
        return 3 * bytes_per_component;
      case GL_RGBA:
        return 4 * bytes_per_component;
    }
    return 0;
  }

  ByteBuffer bufferFromBitmap(Bitmap bitmap, int bytesPerPixel, boolean flipY) {
    return nativeBufferFromBitmap(bitmap, bytesPerPixel, flipY);
  }

  byte[] bytesFromBitmap(Bitmap bitmap, int bytesPerPixel, boolean flipY) {
    return nativeBytesFromBitmap(bitmap, bytesPerPixel, flipY);
  }

  void flipInPlace(byte[] storage, int bytesPerPixel, int height) {
    nativeFlipInPlace(storage, bytesPerPixel, height);
  }

  public void activeTexture(final int texture) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glActiveTexture(texture);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void attachShader(final int program, final int shader) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glAttachShader(program, shader);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bindAttribLocation(final int program, final int index, final String name) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBindAttribLocation(program, index, name);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bindBuffer(final int target, final int buffer) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBindBuffer(target, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bindBuffer(final int target, @Nullable Object buffer) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBindBuffer(target, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bindFramebuffer(final int target, final int framebuffer) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBindFramebuffer(target, framebuffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bindRenderbuffer(final int target, final int renderbuffer) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBindRenderbuffer(target, renderbuffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bindTexture(final int target, final int texture) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBindTexture(target, texture);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void blendColor(final float red, final float green, final float blue, final float alpha) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBlendColor(red, green, blue, alpha);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void blendEquation(final int mode) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBlendEquation(mode);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void blendEquationSeparate(final int modeRGB, final int modeAlpha) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBlendEquationSeparate(modeRGB, modeAlpha);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void blendFunc(final int sfactor, final int dfactor) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBlendFunc(sfactor, dfactor);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void blendFuncSeparate(final int srcRGB, final int dstRGB, final int srcAlpha, final int dstAlpha) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBlendFuncSeparate(srcRGB, dstRGB, srcAlpha, dstAlpha);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bufferData(final int target, final int size, final int usage) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBufferData(target, size, null, usage);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void bufferData(final int target, @Nullable Object srcData, final int usage) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glBufferData(target, 0, null, usage);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bufferData(final int target, final byte[] srcData, final int usage) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final ByteBuffer buffer = ByteBuffer.wrap(srcData);
        GLES20.glBufferData(target, srcData.length, buffer, usage);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bufferData(final int target, final short[] srcData, final int usage) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final ShortBuffer buffer = ShortBuffer.wrap(srcData);
        GLES20.glBufferData(target, srcData.length * SIZE_OF_SHORT, buffer, usage);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bufferData(final int target, final float[] srcData, final int usage) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final FloatBuffer buffer = FloatBuffer.wrap(srcData);
        GLES20.glBufferData(target, srcData.length * SIZE_OF_FLOAT, buffer, usage);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bufferData(final int target, final int[] srcData, final int usage) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final IntBuffer buffer = IntBuffer.wrap(srcData);
        GLES20.glBufferData(target, srcData.length * SIZE_OF_INT, buffer, usage);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bufferSubData(final int target, final int offset, final byte[] srcData) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final ByteBuffer buffer = ByteBuffer.wrap(srcData);
        int os = SIZE_OF_BYTE * offset;
        GLES20.glBufferSubData(target, os, srcData.length * SIZE_OF_BYTE, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bufferSubData(final int target, final int offset, final short[] srcData) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final ShortBuffer buffer = ShortBuffer.wrap(srcData);
        int os = SIZE_OF_SHORT * offset;
        GLES20.glBufferSubData(target, os, srcData.length * SIZE_OF_SHORT, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bufferSubData(final int target, final int offset, final int[] srcData) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final IntBuffer buffer = IntBuffer.wrap(srcData);
        int os = SIZE_OF_INT * offset;
        GLES20.glBufferSubData(target, os, srcData.length * SIZE_OF_INT, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void bufferSubData(final int target, final int offset, final float[] srcData) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final FloatBuffer buffer = FloatBuffer.wrap(srcData);
        int os = SIZE_OF_FLOAT * offset;
        GLES20.glBufferSubData(target, os, srcData.length * SIZE_OF_FLOAT, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public int checkFramebufferStatus(final int target) {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] status = new int[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        status[0] = GLES20.glCheckFramebufferStatus(target);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return status[0];
  }

  public void clear(final int mask) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        if (clearIfComposited(mask) != HowToClear.CombinedClear) {
          GLES20.glClear(mask);
        }
        GLES20.glClear(mask);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void clearColor(final float red, final float green, final float blue, final float alpha) {
    final CountDownLatch lock = new CountDownLatch(1);
    canvas.mClearColor[0] = red;
    canvas.mClearColor[1] = green;
    canvas.mClearColor[2] = blue;
    canvas.mClearColor[3] = alpha;
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glClearColor(red, green, blue, alpha);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void clearDepth(final float depth) {
    final CountDownLatch lock = new CountDownLatch(1);
    canvas.mClearDepth = depth;
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glClearDepthf(depth);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void clearStencil(final int stencil) {
    final CountDownLatch lock = new CountDownLatch(1);
    canvas.mClearStencil = stencil;
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glClearStencil(stencil);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void colorMask(final boolean red, final boolean green, final boolean blue, final boolean alpha) {
    final CountDownLatch lock = new CountDownLatch(1);
    canvas.mColorMask[0] = red;
    canvas.mColorMask[1] = green;
    canvas.mColorMask[2] = blue;
    canvas.mColorMask[3] = alpha;
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glColorMask(red, green, blue, alpha);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  void reset() {
    GLES20.glDisable(GLES20.GL_SCISSOR_TEST);
    GLES20.glClearColor(0, 0, 0, 0);
    GLES20.glColorMask(true, true, true, true);
    int clearMask = GLES20.GL_COLOR_BUFFER_BIT;
    if (canvas.contextDepth) {
      GLES20.glClearDepthf(1);
      clearMask |= GLES20.GL_DEPTH_BUFFER_BIT;
      GLES20.glDepthMask(true);
    }
    if (canvas.contextStencil) {
      GLES20.glClearStencil(0);
      clearMask |= GLES20.GL_STENCIL_BUFFER_BIT;
      GLES20.glStencilMaskSeparate(GLES20.GL_FRONT, 0xFFFFFFFF);
    }
  }

  void restoreStateAfterClear() {

    // Restore the state that the context set.
    if (canvas.mScissorEnabled) {
      GLES20.glEnable(GLES20.GL_SCISSOR_TEST);
    }
    GLES20.glClearColor(canvas.mClearColor[0], canvas.mClearColor[1], canvas.mClearColor[2], canvas.mClearColor[3]);
    GLES20.glColorMask(canvas.mColorMask[0], canvas.mColorMask[1],
      canvas.mColorMask[2], canvas.mColorMask[3]);
    GLES20.glClearDepthf(canvas.mClearDepth);
    GLES20.glClearStencil(canvas.mClearStencil);
    GLES20.glStencilMaskSeparate(GLES20.GL_FRONT, canvas.mStencilMask);
    GLES20.glDepthMask(canvas.mDepthMask);
  }

  HowToClear clearIfComposited() {
    return clearIfComposited(0);
  }

  HowToClear clearIfComposited(int mask) {
    boolean combinedClear = (mask > 0) && !canvas.mScissorEnabled;
    int m = mask & GLES20.GL_COLOR_BUFFER_BIT;
    GLES20.glDisable(GLES20.GL_SCISSOR_TEST);
    if (combinedClear && (m == GLES20.GL_COLOR_BUFFER_BIT)) {
      GLES20.glClearColor(canvas.mColorMask[0] ? canvas.mClearColor[0] : 0,
        canvas.mColorMask[1] ? canvas.mClearColor[1] : 0,
        canvas.mColorMask[2] ? canvas.mClearColor[2] : 0,
        canvas.mColorMask[3] ? canvas.mClearColor[3] : 0);
    } else {
      GLES20.glClearColor(0, 0, 0, 0);
    }


    GLES20.glColorMask(true, true, true, true);
    int clearMask = GLES20.GL_COLOR_BUFFER_BIT;
    if (canvas.contextDepth) {
      if (!combinedClear || !canvas.mDepthMask || (mask & GLES20.GL_DEPTH_BUFFER_BIT) == 0) {
        GLES20.glClearDepthf(1);
        clearMask |= GLES20.GL_DEPTH_BUFFER_BIT;
        GLES20.glDepthMask(true);
      }
    }
    if (canvas.contextStencil) {
      if (combinedClear && (mask & GLES20.GL_STENCIL_BUFFER_BIT) != 0) {
        GLES20.glClearStencil(canvas.mClearStencil & canvas.mStencilMask);
      } else {
        GLES20.glClearStencil(0);
        clearMask |= GLES20.GL_STENCIL_BUFFER_BIT;
        GLES20.glStencilMaskSeparate(GLES20.GL_FRONT, 0xFFFFFFFF);
      }
    }
    // mask
    //  GLES20.glBindFramebuffer(GLES20.GL_FRAMEBUFFER, 0);
    GLES20.glClear(mask);
    restoreStateAfterClear();
    return combinedClear ? HowToClear.CombinedClear : HowToClear.JustClear;

  }

  public void commit() {
    // NOOP
  }

  public void compileShader(final int shader) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glCompileShader(shader);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void compressedTexImage2D(final int target, final int level, final int internalformat, final int width, final int height, final int border, final byte[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final ByteBuffer buffer = ByteBuffer.wrap(pixels);
        GLES20.glCompressedTexImage2D(target, level, internalformat, width, height, border, pixels.length * SIZE_OF_BYTE, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void compressedTexImage2D(final int target, final int level, final int internalformat, final int width, final int height, final int border, final short[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final ShortBuffer buffer = ShortBuffer.wrap(pixels);
        GLES20.glCompressedTexImage2D(target, level, internalformat, width, height, border, pixels.length * SIZE_OF_SHORT, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void compressedTexImage2D(final int target, final int level, final int internalformat, final int width, final int height, final int border, final int[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final IntBuffer buffer = IntBuffer.wrap(pixels);
        GLES20.glCompressedTexImage2D(target, level, internalformat, width, height, border, pixels.length * SIZE_OF_INT, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void compressedTexImage2D(final int target, final int level, final int internalformat, final int width, final int height, final int border, final float[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final FloatBuffer buffer = FloatBuffer.wrap(pixels);
        GLES20.glCompressedTexImage2D(target, level, internalformat, width, height, border, pixels.length * SIZE_OF_FLOAT, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void compressedTexSubImage2D(final int target, final int level, final int xoffset, final int yoffset, final int width, final int height, final int format, final byte[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final ByteBuffer buffer = ByteBuffer.wrap(pixels);
        GLES20.glCompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels.length * SIZE_OF_BYTE, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void compressedTexSubImage2D(final int target, final int level, final int xoffset, final int yoffset, final int width, final int height, final int format, final short[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final ShortBuffer buffer = ShortBuffer.wrap(pixels);
        GLES20.glCompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels.length * SIZE_OF_SHORT, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void compressedTexSubImage2D(final int target, final int level, final int xoffset, final int yoffset, final int width, final int height, final int format, final int[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final IntBuffer buffer = IntBuffer.wrap(pixels);
        GLES20.glCompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels.length * SIZE_OF_INT, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void compressedTexSubImage2D(final int target, final int level, final int xoffset, final int yoffset, final int width, final int height, final int format, final float[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        final FloatBuffer buffer = FloatBuffer.wrap(pixels);
        GLES20.glCompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels.length * SIZE_OF_FLOAT, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void copyTexImage2D(final int target, final int level, final int internalformat, final int x, final int y, final int width, final int height, final int border) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        clearIfComposited();
        GLES20.glCopyTexImage2D(target, level, internalformat, x, y, width, height, border);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void copyTexSubImage2D(final int target, final int level, final int xoffset, final int yoffset, final int x, final int y, final int width, final int height) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        clearIfComposited();
        GLES20.glCopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public int createBuffer() {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] bufferId = new int[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glGenBuffers(1, bufferId, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return bufferId[0];
  }

  public int createFramebuffer() {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] frameBufferId = new int[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glGenFramebuffers(1, frameBufferId, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return frameBufferId[0];
  }

  public int createProgram() {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] program = new int[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        program[0] = GLES20.glCreateProgram();
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return program[0];
  }

  public int createRenderbuffer() {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] renderBufferId = new int[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glGenRenderbuffers(1, renderBufferId, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return renderBufferId[0];
  }

  public int createShader(final int type) {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] shader = new int[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        shader[0] = GLES20.glCreateShader(type);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return shader[0];
  }

  public int createTexture() {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] textureId = new int[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glGenTextures(1, textureId, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return textureId[0];
  }

  public void cullFace(final int mode) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glCullFace(mode);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void deleteBuffer(final int buffer) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        int[] id = {buffer};
        GLES20.glDeleteBuffers(1, id, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void deleteFramebuffer(final int frameBuffer) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        int[] id = {frameBuffer};
        GLES20.glDeleteFramebuffers(1, id, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void deleteProgram(final int program) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glDeleteProgram(program);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void deleteRenderbuffer(final int renderbuffer) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        int[] id = {renderbuffer};
        GLES20.glDeleteRenderbuffers(1, id, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void deleteShader(final int shader) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glDeleteShader(shader);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void deleteTexture(final int texture) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        int[] id = {texture};
        GLES20.glDeleteTextures(1, id, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void depthFunc(final int func) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glDepthFunc(func);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void depthMask(final boolean flag) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glDepthMask(flag);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void depthRange(final float zNear, final float zFar) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glDepthRangef(zNear, zFar);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void detachShader(final int program, final int shader) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glDetachShader(program, shader);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void disable(final int cap) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glDisable(cap);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void disableVertexAttribArray(final int index) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glDisableVertexAttribArray(index);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void drawArrays(final int mode, final int first, final int count) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        clearIfComposited();
        GLES20.glDrawArrays(mode, first, count);
        updateCanvas();
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void drawElements(final int mode, final int count, final int type, final int offset) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        clearIfComposited();
        GLES20.glDrawElements(mode, count, type, offset);
        updateCanvas();
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void enable(final int cap) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glEnable(cap);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void enableVertexAttribArray(final int index) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glEnableVertexAttribArray(index);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void finish() {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glFinish();
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void flush() {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glFlush();
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void framebufferRenderbuffer(final int target, final int attachment, final int renderbuffertarget, final int renderbuffer) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glFramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void framebufferTexture2D(final int target, final int attachment, final int textarget, final int texture, final int level) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glFramebufferTexture2D(target, attachment, textarget, texture, level);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void frontFace(final int mode) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glFrontFace(mode);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void generateMipmap(final int target) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glGenerateMipmap(target);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public WebGLActiveInfo getActiveAttrib(final int program, final int index) {
    final CountDownLatch lock = new CountDownLatch(1);
    final WebGLActiveInfo info = new WebGLActiveInfo();
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        //IntBuffer length = IntBuffer.allocate(1);
        //GLES20.glGetProgramiv(program, GLES20.GL_ACTIVE_ATTRIBUTE_MAX_LENGTH, length);
        int[] length = new int[1];
        GLES20.glGetProgramiv(program, GLES20.GL_ACTIVE_ATTRIBUTE_MAX_LENGTH, length, 0);
        byte[] name = new byte[length[0]];
        int[] size = new int[1];
        int[] type = new int[1];
        int[] nameLength = new int[1];
        GLES20.glGetActiveAttrib(program, index, length[0], nameLength, 0, size, 0, type, 0, name, 0);
        String result;
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.KITKAT) {
          result = new String(name, 0, nameLength[0], UTF_8);
        } else {
          try {
            result = new String(name, 0, nameLength[0], "UTF-8");
          } catch (UnsupportedEncodingException e) {
            result = new String(name, 0, nameLength[0]);
          }
        }
        info.name = result;
        info.size = size[0];
        info.type = type[0];
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return info;
  }


  public WebGLActiveInfo getActiveUniform(final int program, final int index) {
    final CountDownLatch lock = new CountDownLatch(1);
    final WebGLActiveInfo info = new WebGLActiveInfo();
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        int[] length = new int[1];
        int[] size = new int[1];
        int[] type = new int[1];
        GLES20.glGetProgramiv(program, GLES20.GL_ACTIVE_UNIFORM_MAX_LENGTH, length, 0);
        byte[] name = new byte[length[0]];
        int[] nameLength = new int[1];
        GLES20.glGetActiveUniform(program, index, length[0], nameLength, 0, size, 0, type, 0, name, 0);
        String result;
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.KITKAT) {
          result = new String(name, 0, nameLength[0], UTF_8);
        } else {
          try {
            result = new String(name, 0, nameLength[0], "UTF-8");
          } catch (UnsupportedEncodingException e) {
            result = new String(name, 0, nameLength[0]);
          }
        }
        info.name = result;
        info.size = size[0];
        info.type = type[0];
        lock.countDown();
      }
    });

    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return info;
  }

  public int[] getAttachedShaders(final int program) {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[][] shadersList = new int[1][];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        int[] count = new int[1];
        GLES20.glGetProgramiv(program, GLES20.GL_ATTACHED_SHADERS, count, 0);
        int[] shaders = new int[count[0]];
        int[] written = new int[1];
        GLES20.glGetAttachedShaders(program, count[0], written, 0, shaders, 0);
        shadersList[0] = shaders;
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return shadersList[0];
  }

  public int getAttribLocation(final int program, final String name) {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] location = new int[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        location[0] = GLES20.glGetAttribLocation(program, name);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }

    return location[0];
  }

  public int getBufferParameter(final int target, final int pname) {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] parameter = new int[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
//                IntBuffer params = IntBuffer.allocate(1);
//                GLES20.glGetBufferParameteriv(target, pname, params);
        int[] params = new int[1];
        GLES20.glGetBufferParameteriv(target, pname, params, 0);
        parameter[0] = params[0];
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return parameter[0];
  }


  private boolean alpha = true;
  private boolean antialias = true;
  private boolean depth = true;
  private boolean failIfMajorPerformanceCaveat = false;
  private String powerPreference = "default";
  private boolean premultipliedAlpha = true;
  private boolean preserveDrawingBuffer = false;
  private boolean stencil = false;
  private boolean desynchronized = false;

  public Map<String, Object> getContextAttributes() {
    // Return nil if context is lost
    Map<String, Object> attrib = new HashMap<>();
    attrib.put("alpha", canvas.contextAlpha);
    attrib.put("antialias", canvas.contextAntialias);
    attrib.put("depth", canvas.contextDepth);
    attrib.put("failIfMajorPerformanceCaveat", canvas.contextFailIfMajorPerformanceCaveat);
    attrib.put("powerPreference", canvas.contextPowerPreference);
    attrib.put("premultipliedAlpha", canvas.contextPremultipliedAlpha);
    attrib.put("preserveDrawingBuffer", canvas.contextPreserveDrawingBuffer);
    attrib.put("stencil", canvas.contextStencil);
    attrib.put("desynchronized", canvas.contextDesynchronized);
    return attrib;
  }

  public int getError() {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] error = new int[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        error[0] = GLES20.glGetError();
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return error[0];
  }

  public Object getExtension(final String name) {
    final CountDownLatch lock = new CountDownLatch(1);
    final Object[] value = new Object[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        String extensions = GLES20.glGetString(GLES20.GL_EXTENSIONS);
        if (name.equals("EXT_blend_minmax") && extensions.contains("GL_EXT_blend_minmax")) {
          value[0] = new EXT_blend_minmax();
        } else if (name.equals("EXT_color_buffer_half_float") && extensions.contains("GL_EXT_color_buffer_half_float")) {
          value[0] = new EXT_color_buffer_half_float();
        } else if (name.equals("EXT_disjoint_timer_query") && extensions.contains("GL_EXT_disjoint_timer_query")) {
          if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2) {
            value[0] = new EXT_disjoint_timer_query(canvas);
          } else {
            value[0] = null;
          }
        } else if (name.equals("EXT_sRGB") && extensions.contains("GL_EXT_sRGB")) {
          value[0] = new EXT_sRGB();
        } else if (name.equals("EXT_shader_texture_lod")) {
          if (extensions.contains("GL_EXT_shader_texture_lod")) {
            value[0] = new EXT_shader_texture_lod();
          }
        } else if (name.equals("EXT_texture_filter_anisotropic") && extensions.contains("GL_EXT_texture_filter_anisotropic")) {
          value[0] = new EXT_texture_filter_anisotropic();
        } else if (name.equals("OES_element_index_uint") && extensions.contains("GL_OES_element_index_uint")) {
          value[0] = new OES_element_index_uint();
        } else if (name.equals("OES_standard_derivatives") && extensions.contains("GL_OES_standard_derivatives")) {
          value[0] = new OES_standard_derivatives();
        } else if (name.equals("OES_texture_float") && (extensions.contains("GL_OES_texture_float") || canvas.glVersion > 2)) {
          value[0] = new OES_texture_float();
        } else if (name.equals("OES_texture_float_linear") && (extensions.contains("GL_OES_texture_float_linear") || canvas.glVersion > 2)) {
          value[0] = new OES_texture_float_linear();
        } else if (name.equals("OES_texture_half_float") && extensions.contains("GL_OES_texture_half_float")) {
          value[0] = new OES_texture_half_float();
        } else if (name.equals("OES_texture_half_float_linear") && extensions.contains("GL_OES_texture_half_float_linear")) {
          value[0] = new OES_texture_half_float_linear();
        } else if (name.equals("OES_vertex_array_object") && extensions.contains("GL_OES_vertex_array_object")) {
          if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2) {
            value[0] = new OES_vertex_array_object(canvas);
          } else {
            value[0] = null;
          }
        } else if (name.equals("WEBGL_color_buffer_float") && extensions.contains("GL_OES_packed_depth_stencil")) {
          value[0] = new WEBGL_color_buffer_float();
        } else if (name.equals("WEBGL_compressed_texture_atc") && extensions.contains("GL_AMD_compressed_ATC_texture")) {
          value[0] = new WEBGL_compressed_texture_atc();
        } else if (name.equals("WEBGL_compressed_texture_etc1") && extensions.contains("GL_OES_compressed_ETC1_RGB8_texture")) {
          value[0] = new WEBGL_compressed_texture_etc1();
        } else if (name.equals("WEBGL_compressed_texture_s3tc") && extensions.contains("GL_EXT_texture_compression_dxt1") && extensions.contains("GL_EXT_texture_compression_s3tc")) {
          value[0] = new WEBGL_compressed_texture_s3tc();
        } else if (name.equals("WEBGL_compressed_texture_etc")) {
          if (canvas.glVersion > 2) {
            value[0] = new WEBGL_compressed_texture_etc();
          } else {
            value[0] = null;
          }
        } else if (name.equals("WEBGL_compressed_texture_pvrtc") && extensions.contains("GL_IMG_texture_compression_pvrtc")) {
          value[0] = new WEBGL_compressed_texture_pvrtc();
        } else if (name.equals("WEBGL_lose_context")) {
          value[0] = new WEBGL_lose_context(canvas);
        } else if (name.equals("ANGLE_instanced_arrays")) {
          if (canvas.glVersion > 2) {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2) {
              value[0] = new ANGLE_instanced_arrays();
            } else {
              value[0] = null;
            }
          }
        } else if (name.equals("WEBGL_depth_texture") && extensions.contains("GL_OES_depth_texture")) {
          value[0] = new WEBGL_depth_texture();
        } else if (name.equals("WEBGL_draw_buffers")) {
          if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN_MR2) {
            if (canvas.glVersion > 2 || extensions.contains("GL_EXT_draw_buffers")) {
              value[0] = new WEBGL_draw_buffers();
            }
          } else {
            value[0] = null;
          }
        } else {
          value[0] = null;
        }
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return value[0];
  }

  public FramebufferAttachmentParameter getFramebufferAttachmentParameter(final int target, final int attachment, final int pname) {
    final CountDownLatch lock = new CountDownLatch(1);
    final FramebufferAttachmentParameter result = new FramebufferAttachmentParameter();
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
//                IntBuffer params = IntBuffer.allocate(1);
//                GLES20.glGetFramebufferAttachmentParameteriv(target, attachment, pname, params);

        int[] params = new int[1];
        GLES20.glGetFramebufferAttachmentParameteriv(target, attachment, pname, params, 0);
        if (attachment == FRAMEBUFFER_ATTACHMENT_OBJECT_NAME) {
//                    IntBuffer name = IntBuffer.allocate(1);
//                    GLES20.glGetFramebufferAttachmentParameteriv(target, attachment, GLES20.GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE, name);
          int[] name = new int[0];
          GLES20.glGetFramebufferAttachmentParameteriv(target, attachment, GLES20.GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE, name, 0);
          switch (name[0]) {
            case GLES20.GL_RENDERBUFFER:
              result.isRenderbuffer = true;
              result.value = params[0];
              break;
            case GLES20.GL_TEXTURE:
              result.isTexture = true;
              result.value = params[0];
              break;
            default:
              result.value = params[0];
              break;
          }
        } else {
          result.value = params[0];
        }
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return result;
  }


  private boolean boolConverter(int value) {
    return value == GLES20.GL_TRUE;
  }

  public Object getParameter(final int pname) {
    final CountDownLatch lock = new CountDownLatch(1);
    final Object[] parameter = new Object[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        switch (pname) {
          case ACTIVE_TEXTURE:
          case ALPHA_BITS:
          case ARRAY_BUFFER_BINDING:
          case BLEND_DST_ALPHA:
          case BLEND_DST_RGB:
          case BLEND_EQUATION:
          case BLEND_EQUATION_ALPHA:
            //case BLEND_EQUATION_ RGB: same as BLEND_EQUATION
          case BLEND_SRC_ALPHA:
          case BLEND_SRC_RGB:
          case BLUE_BITS:
          case CULL_FACE_MODE:
          case CURRENT_PROGRAM:
          case DEPTH_BITS:
          case DEPTH_FUNC:
          case ELEMENT_ARRAY_BUFFER_BINDING:
          case FRAMEBUFFER_BINDING:
          case FRONT_FACE:
          case GENERATE_MIPMAP_HINT:
          case GREEN_BITS:
          case IMPLEMENTATION_COLOR_READ_FORMAT:
          case IMPLEMENTATION_COLOR_READ_TYPE:
          case MAX_COMBINED_TEXTURE_IMAGE_UNITS:
          case MAX_CUBE_MAP_TEXTURE_SIZE:
          case MAX_FRAGMENT_UNIFORM_VECTORS:
          case MAX_RENDERBUFFER_SIZE:
          case MAX_TEXTURE_IMAGE_UNITS:
          case MAX_TEXTURE_SIZE:
          case MAX_VARYING_VECTORS:
          case MAX_VERTEX_ATTRIBS:
          case MAX_VERTEX_TEXTURE_IMAGE_UNITS:
          case MAX_VERTEX_UNIFORM_VECTORS:
          case PACK_ALIGNMENT:
          case RED_BITS:
          case RENDERBUFFER_BINDING:
          case SAMPLE_BUFFERS:
          case SAMPLES:
          case STENCIL_BACK_FAIL:
          case STENCIL_BACK_FUNC:
          case STENCIL_BACK_PASS_DEPTH_FAIL:
          case STENCIL_BACK_PASS_DEPTH_PASS:
          case STENCIL_BACK_REF:
          case STENCIL_BACK_VALUE_MASK:
          case STENCIL_BACK_WRITEMASK:
          case STENCIL_BITS:
          case STENCIL_CLEAR_VALUE:
          case STENCIL_FAIL:
          case STENCIL_FUNC:
          case STENCIL_PASS_DEPTH_FAIL:
          case STENCIL_PASS_DEPTH_PASS:
          case STENCIL_REF:
          case STENCIL_VALUE_MASK:
          case STENCIL_WRITEMASK:
          case SUBPIXEL_BITS:
          case TEXTURE_BINDING_2D:
          case TEXTURE_BINDING_CUBE_MAP:
          case UNPACK_ALIGNMENT:
//                        IntBuffer param = IntBuffer.allocate(1);
//                        GLES20.glGetIntegerv(pname, param);
            int[] param = new int[1];
            GLES20.glGetIntegerv(pname, param, 0);
            if ((pname == TEXTURE_BINDING_2D || pname == TEXTURE_BINDING_CUBE_MAP || pname == RENDERBUFFER_BINDING || pname == FRAMEBUFFER_BINDING) && param[0] == 0) {
              parameter[0] = null;
            } else {
              parameter[0] = param[0];
            }
            break;
          case UNPACK_COLORSPACE_CONVERSION_WEBGL:
            int cs = colorSpaceConversionWebGL;
            if (cs == -1) {
              cs = BROWSER_DEFAULT_WEBGL;
            }
            parameter[0] = cs;
            break;
          case ALIASED_LINE_WIDTH_RANGE:
          case ALIASED_POINT_SIZE_RANGE:
          case DEPTH_RANGE:
            float[] param2 = new float[2];
            GLES20.glGetFloatv(pname, param2, 0);
            parameter[0] = param2;
            break;
          case BLEND_COLOR:
          case COLOR_CLEAR_VALUE:
            float[] param3 = new float[4];
            GLES20.glGetFloatv(pname, param3, 0);
            parameter[0] = param3;
            break;
          case UNPACK_FLIP_Y_WEBGL:
            parameter[0] = flipYWebGL;
            break;
          case UNPACK_PREMULTIPLY_ALPHA_WEBGL:
            parameter[0] = premultiplyAlphaWebGL;
            break;
          case BLEND:
          case CULL_FACE:
          case DEPTH_TEST:
          case DEPTH_WRITEMASK:
          case DITHER:
          case POLYGON_OFFSET_FILL:
          case SAMPLE_COVERAGE_INVERT:
          case SCISSOR_TEST:
          case STENCIL_TEST:
            boolean[] param4 = new boolean[1];
            GLES20.glGetBooleanv(pname, param4, 0);
            parameter[0] = param4[0];
            break;
          case COLOR_WRITEMASK:
            boolean[] param5 = new boolean[4];
            GLES20.glGetBooleanv(pname, param5, 0);
            parameter[0] = param5;
            break;
          case COMPRESSED_TEXTURE_FORMATS:
            int[] count = new int[1];
            GLES20.glGetIntegerv(GLES20.GL_NUM_COMPRESSED_TEXTURE_FORMATS, count, 0);
            int[] formats = new int[count[0]];
            GLES20.glGetIntegerv(GLES20.GL_COMPRESSED_TEXTURE_FORMATS, formats, 0);
            parameter[0] = formats;
            break;
          case DEPTH_CLEAR_VALUE:
          case LINE_WIDTH:
          case POLYGON_OFFSET_FACTOR:
          case POLYGON_OFFSET_UNITS:
          case SAMPLE_COVERAGE_VALUE:
            float[] param6 = new float[1];
            GLES20.glGetFloatv(pname, param6, 0);
            parameter[0] = param6[0];
            break;
          case MAX_VIEWPORT_DIMS:
            int[] dims = new int[2];
            GLES20.glGetIntegerv(pname, dims, 0);
            parameter[0] = dims;
            break;
          case SCISSOR_BOX:
          case VIEWPORT:
            int[] params7 = new int[4];
            GLES20.glGetIntegerv(pname, params7, 0);
            parameter[0] = params7;
            break;
          case RENDERER:
          case SHADING_LANGUAGE_VERSION:
          case VENDOR:
          case VERSION:
            parameter[0] = GLES20.glGetString(pname);
            break;
          default:
            parameter[0] = null;
            break;
        }
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return parameter[0];
  }

  public String getProgramInfoLog(final int program) {
    final CountDownLatch lock = new CountDownLatch(1);
    final String[] infoLog = new String[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        infoLog[0] = GLES20.glGetProgramInfoLog(program);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return infoLog[0];
  }

  public Object getProgramParameter(final int program, final int pname) {
    final CountDownLatch lock = new CountDownLatch(1);
    final Object[] parameter = new Object[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        int[] param = new int[1];
        GLES20.glGetProgramiv(program, pname, param, 0);
        switch (pname) {
          case DELETE_STATUS:
          case LINK_STATUS:
          case VALIDATE_STATUS:
            parameter[0] = boolConverter(param[0]);
            break;
          default:
            parameter[0] = param[0];
            break;
        }
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return parameter[0];
  }

  public int getRenderbufferParameter(final int target, final int pname) {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] parameter = new int[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        int[] params = new int[1];
        GLES20.glGetRenderbufferParameteriv(target, pname, params, 0);
        parameter[0] = params[0];
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return parameter[0];
  }

  public String getShaderInfoLog(final int shader) {
    final CountDownLatch lock = new CountDownLatch(1);
    final String[] infoLog = new String[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        infoLog[0] = GLES20.glGetShaderInfoLog(shader);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return infoLog[0];
  }

  public Object getShaderParameter(final int shader, final int pname) {
    final CountDownLatch lock = new CountDownLatch(1);
    final Object[] parameter = new Object[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        int[] params = new int[1];
        GLES20.glGetShaderiv(shader, pname, params, 0);
        switch (pname) {
          case DELETE_STATUS:
          case COMPILE_STATUS:
            parameter[0] = boolConverter(params[0]);
            break;
          default:
            parameter[0] = params[0];
            break;
        }
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return parameter[0];
  }

  public WebGLShaderPrecisionFormat getShaderPrecisionFormat(final int shaderType, final int precisionType) {
    final CountDownLatch lock = new CountDownLatch(1);
    final WebGLShaderPrecisionFormat precisionFormat = new WebGLShaderPrecisionFormat();
    //final boolean[] hasError = new boolean[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        int[] range = new int[2];
        int[] precision = new int[1];
        GLES20.glGetShaderPrecisionFormat(shaderType, precisionType, range, 0, precision, 0);
//                int error = GLES20.glGetError();
//                if (error == GLES20.GL_INVALID_ENUM || error == GLES20.GL_INVALID_OPERATION) {
//                    hasError[0] = true;
//                    lock.countDown();
//                    return;
//                }
        precisionFormat.rangeMin = range[0];
        precisionFormat.rangeMax = range[1];
        precisionFormat.precision = precision[0];
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
        /*if (hasError[0]) {
            return null;
        }*/
    return precisionFormat;
  }

  public String getShaderSource(final int shader) {
    final CountDownLatch lock = new CountDownLatch(1);
    final String[] source = new String[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        source[0] = GLES20.glGetShaderSource(shader);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return source[0];
  }

  public String[] getSupportedExtensions() {
    final CountDownLatch lock = new CountDownLatch(1);
    final ArrayList<String[]> extensions = new ArrayList<>();
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        String glExtensions = GLES20.glGetString(GLES20.GL_EXTENSIONS);
        extensions.add(glExtensions.split(" "));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return extensions.get(0);
  }

  public int getTexParameter(final int target, final int pname) {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] parameters = new int[1];
    final boolean[] hasError = new boolean[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        int[] params = new int[1];
        GLES20.glGetTexParameteriv(target, pname, params, 0);
        parameters[0] = params[0];
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }

    return parameters[0];
  }

  private float[] getFloatSlice(int count) {
    return new float[count];
  }

  private int[] getIntSlice(int count) {
    return new int[count];
  }

  public Object getUniform(final int program, final int location) {
    final CountDownLatch lock = new CountDownLatch(1);
    final Object[] uniform = new Object[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        int[] type = new int[1];
        GLES20.glGetActiveUniform(program, location, 0, null, 0, null, 0, type, 0, null, 0);
        switch (type[0]) {
          case GLES20.GL_FLOAT:
            float[] single = getFloatSlice(1);
            GLES20.glGetUniformfv(program, location, single, 0);
            uniform[0] = single[0];
            break;
          case GLES20.GL_FLOAT_VEC2:
            float[] vec2 = getFloatSlice(2);
            GLES20.glGetUniformfv(program, location, vec2, 0);
            uniform[0] = vec2;
            break;
          case GLES20.GL_FLOAT_VEC3:
            float[] vec3 = getFloatSlice(3);
            GLES20.glGetUniformfv(program, location, vec3, 0);
            uniform[0] = vec3;
            break;
          case GLES20.GL_FLOAT_VEC4:
            float[] vec4 = getFloatSlice(4);
            GLES20.glGetUniformfv(program, location, vec4, 0);
            uniform[0] = vec4;
            break;
          case GLES20.GL_SAMPLER_2D:
          case GLES20.GL_SAMPLER_CUBE:
          case GLES20.GL_INT:
            int[] singleInt = getIntSlice(1);
            GLES20.glGetUniformiv(program, location, singleInt, 0);
            uniform[0] = singleInt[0];
            break;
          case GLES20.GL_INT_VEC2:
            int[] intVec2 = getIntSlice(2);
            GLES20.glGetUniformiv(program, location, intVec2, 0);
            uniform[0] = intVec2;
            break;
          case GLES20.GL_INT_VEC3:
            int[] intVec3 = getIntSlice(3);
            GLES20.glGetUniformiv(program, location, intVec3, 0);
            uniform[0] = intVec3;
            break;
          case GLES20.GL_INT_VEC4:
            int[] intVec4 = getIntSlice(4);
            GLES20.glGetUniformiv(program, location, intVec4, 0);
            uniform[0] = intVec4;
            break;
          case GLES20.GL_BOOL:
            int[] singleBool = getIntSlice(1);
            GLES20.glGetUniformiv(program, location, singleBool, 0);
            uniform[0] = boolConverter(singleBool[0]);
            break;
          case GLES20.GL_BOOL_VEC2:
            int[] boolVec2 = getIntSlice(2);
            boolean[] boolVec2Result = new boolean[2];
            GLES20.glGetUniformiv(program, location, boolVec2, 0);
            boolVec2Result[0] = boolConverter(boolVec2[0]);
            boolVec2Result[1] = boolConverter(boolVec2[1]);
            uniform[0] = boolVec2Result;
            break;
          case GLES20.GL_BOOL_VEC3:
            int[] boolVec3 = getIntSlice(3);
            boolean[] boolVec3Result = new boolean[3];
            GLES20.glGetUniformiv(program, location, boolVec3, 0);
            boolVec3Result[0] = boolConverter(boolVec3[0]);
            boolVec3Result[1] = boolConverter(boolVec3[1]);
            boolVec3Result[2] = boolConverter(boolVec3[2]);
            uniform[0] = boolVec3Result;
            break;
          case GLES20.GL_BOOL_VEC4:
            int[] boolVec4 = getIntSlice(4);
            boolean[] boolVec4Result = new boolean[4];
            GLES20.glGetUniformiv(program, location, boolVec4, 0);
            boolVec4Result[0] = boolConverter(boolVec4[0]);
            boolVec4Result[1] = boolConverter(boolVec4[1]);
            boolVec4Result[2] = boolConverter(boolVec4[2]);
            boolVec4Result[3] = boolConverter(boolVec4[3]);
            uniform[0] = boolVec4Result;
            break;
          case GLES20.GL_FLOAT_MAT2:
            float[] mat2 = getFloatSlice(2);
            GLES20.glGetUniformfv(program, location, mat2, 0);
            uniform[0] = mat2;
            break;
          case GLES20.GL_FLOAT_MAT3:
            float[] mat3 = getFloatSlice(3);
            GLES20.glGetUniformfv(program, location, mat3, 0);
            uniform[0] = mat3;
            break;
          case GLES20.GL_FLOAT_MAT4:
            float[] mat4 = getFloatSlice(4);
            GLES20.glGetUniformfv(program, location, mat4, 0);
            uniform[0] = mat4;
            break;
        }
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return uniform[0];
  }

  public int getUniformLocation(final int program, final String name) {
    final CountDownLatch lock = new CountDownLatch(1);
    final int[] location = new int[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        location[0] = GLES20.glGetUniformLocation(program, name);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return location[0];
  }

  public Object getVertexAttrib(final int index, final int pname) {
    final CountDownLatch lock = new CountDownLatch(1);
    final Object[] attrib = new Object[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        if (pname == CURRENT_VERTEX_ATTRIB) {
          float[] params = new float[4];
          GLES20.glGetVertexAttribfv(index, pname, params, 0);
          attrib[0] = params;
        } else {
          int[] params = new int[1];
          GLES20.glGetVertexAttribiv(index, pname, params, 0);
          switch (pname) {
            case VERTEX_ATTRIB_ARRAY_ENABLED:
            case VERTEX_ATTRIB_ARRAY_NORMALIZED:
              attrib[0] = boolConverter(params[0]);
              break;
            default:
              attrib[0] = params[0];
              break;
          }
        }
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return attrib[0];
  }

  public long getVertexAttribOffset(final int index, final int pname) {
    final CountDownLatch lock = new CountDownLatch(1);
    final long[] offset = new long[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        ByteBuffer buffer = ByteBuffer.allocateDirect(SIZE_OF_LONG).order(ByteOrder.nativeOrder());
        // LongBuffer buffer = LongBuffer.allocate(1);
        nativeGetVertexAttribOffset(index, pname, buffer);
        offset[0] = buffer.get(0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return offset[0];
  }

  public void hint(final int target, final int mode) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glHint(target, mode);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public boolean isBuffer(final int buffer) {
    final CountDownLatch lock = new CountDownLatch(1);
    final boolean[] value = new boolean[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        value[0] = GLES20.glIsBuffer(buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return value[0];
  }

  public boolean isContextLost() {

//        if let renderer = canvas.renderer as? GLRenderer {
//            return EAGLContext.current() != renderer.glkView.context
//        }
    return false;
  }


  public boolean isEnabled(final int cap) {
    final CountDownLatch lock = new CountDownLatch(1);
    final boolean[] value = new boolean[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        value[0] = GLES20.glIsEnabled(cap);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return value[0];
  }

  public boolean isFramebuffer(final int framebuffer) {
    final CountDownLatch lock = new CountDownLatch(1);
    final boolean[] value = new boolean[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        value[0] = GLES20.glIsFramebuffer(framebuffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return value[0];
  }

  public boolean isProgram(final int program) {
    final CountDownLatch lock = new CountDownLatch(1);
    final boolean[] value = new boolean[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        value[0] = GLES20.glIsProgram(program);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return value[0];
  }

  public boolean isRenderbuffer(final int renderbuffer) {
    final CountDownLatch lock = new CountDownLatch(1);
    final boolean[] value = new boolean[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        value[0] = GLES20.glIsRenderbuffer(renderbuffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return value[0];
  }

  public boolean isShader(final int shader) {
    final CountDownLatch lock = new CountDownLatch(1);
    final boolean[] value = new boolean[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        value[0] = GLES20.glIsShader(shader);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return value[0];
  }

  public boolean isTexture(final int texture) {
    final CountDownLatch lock = new CountDownLatch(1);
    final boolean[] value = new boolean[1];
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        value[0] = GLES20.glIsTexture(texture);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    return value[0];
  }

  public void lineWidth(final float width) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glLineWidth(width);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void linkProgram(final int program) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glLinkProgram(program);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  private int objectToInt(Object value, int defaultValue) {
    if (value != null) {
      try {
        return Integer.parseInt(value.toString());
      } catch (NumberFormatException e) {
        return defaultValue;
      }
    }
    return defaultValue;
  }

  private boolean objectToBoolean(Object value, boolean defaultValue) {
    if (value != null) {
      return Boolean.parseBoolean(value.toString());
    }
    return defaultValue;
  }

  private int objectToColorSpace(Object value, int defaultValue) {
    if (value != null) {
      try {
        int val = Integer.parseInt(value.toString());
        if (val == BROWSER_DEFAULT_WEBGL || val == GLES20.GL_NONE) {
          return val;
        }
        return BROWSER_DEFAULT_WEBGL;
      } catch (NumberFormatException e) {
        return defaultValue;
      }
    }
    return defaultValue;
  }

  boolean flipYWebGL = false;
  private boolean premultiplyAlphaWebGL = false;
  private int colorSpaceConversionWebGL = -1;

  public void pixelStorei(final int pname, final Object param) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        switch (pname) {
          case GLES20.GL_PACK_ALIGNMENT:
          case GLES20.GL_UNPACK_ALIGNMENT:
            GLES20.glPixelStorei(pname, objectToInt(param, 4));
            break;
          case UNPACK_FLIP_Y_WEBGL:
            flipYWebGL = objectToBoolean(param, false);
            break;
          case UNPACK_PREMULTIPLY_ALPHA_WEBGL:
            premultiplyAlphaWebGL = objectToBoolean(param, false);
            break;
          case UNPACK_COLORSPACE_CONVERSION_WEBGL:
            colorSpaceConversionWebGL = objectToColorSpace(param, BROWSER_DEFAULT_WEBGL);
            break;
          default:
            break;
        }
        lock.countDown();
      }
    });

    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void polygonOffset(final float factor, final float units) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glPolygonOffset(factor, units);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  // Does it need a lock ??
  public void readPixels(final int x, final int y, final int width, final int height, final int format, final int type, final byte[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        clearIfComposited();
        GLES20.glReadPixels(x, y, width, height, format, type, ByteBuffer.wrap(pixels));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void readPixels(final int x, final int y, final int width, final int height, final int format, final int type, final short[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        clearIfComposited();
        GLES20.glReadPixels(x, y, width, height, format, type, ShortBuffer.wrap(pixels));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void readPixels(final int x, final int y, final int width, final int height, final int format, final int type, final float[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        clearIfComposited();
        GLES20.glReadPixels(x, y, width, height, format, type, FloatBuffer.wrap(pixels));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void readPixels(final int x, final int y, final int width, final int height, final int format, final int type, final int[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        clearIfComposited();
        GLES20.glReadPixels(x, y, width, height, format, type, IntBuffer.wrap(pixels));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void renderbufferStorage(final int target, final int internalFormat, final int width, final int height) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glRenderbufferStorage(target, internalFormat, width, height);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void sampleCoverage(final float value, final boolean invert) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glSampleCoverage(value, invert);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void scissor(final int x, final int y, final int width, final int height) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glScissor(x, y, width, height);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void shaderSource(final int shader, final String source) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glShaderSource(shader, source);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void stencilFunc(final int func, final int ref, final int mask) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glStencilFunc(func, ref, mask);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void stencilFuncSeparate(final int face, final int func, final int ref, final int mask) {
    final CountDownLatch lock = new CountDownLatch(1);
    switch (face) {
      case GLES20.GL_FRONT_AND_BACK:
        canvas.mStencilFuncRef = ref;
        canvas.mStencilFuncRefBack = ref;
        canvas.mStencilFuncMask = mask;
        canvas.mStencilFuncMaskBack = mask;
        break;
      case GLES20.GL_FRONT:
        canvas.mStencilFuncRef = ref;
        canvas.mStencilFuncMask = mask;
        ;
        break;
      case GLES20.GL_BACK:
        canvas.mStencilFuncRefBack = ref;
        canvas.mStencilFuncMaskBack = mask;
        break;
      default:
        // TODO emit error
        break;
    }
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glStencilFuncSeparate(face, func, ref, mask);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void stencilMask(final int mask) {
    final CountDownLatch lock = new CountDownLatch(1);
    canvas.mStencilMask = mask;
    canvas.mStencilMaskBack = mask;
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glStencilMask(mask);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void stencilMaskSeparate(final int face, final int mask) {
    final CountDownLatch lock = new CountDownLatch(1);
    switch (face) {
      case GLES20.GL_FRONT_AND_BACK:
        canvas.mStencilMask = mask;
        canvas.mStencilMaskBack = mask;
      case GLES20.GL_FRONT:
        canvas.mStencilMask = mask;
      case GLES20.GL_BACK:
        canvas.mStencilMaskBack = mask;
      default:
        // TODO emit error
        break;
    }
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glStencilMaskSeparate(face, mask);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void stencilOp(final int fail, final int zfail, final int zpass) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glStencilOp(fail, zfail, zpass);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void stencilOpSeparate(final int face, final int fail, final int zfail, final int zpass) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glStencilOpSeparate(face, fail, zfail, zpass);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void texImage2D(final int target, final int level, final int internalformat, final int width, final int height, final int border, final int format, final int type, @Nullable final byte[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        if (pixels != null) {
          if (flipYWebGL) {
            flipInPlace(pixels, bytesPerPixel(type, format) * width, height);
          }
          GLES20.glTexImage2D(target, level, internalformat, width, height, border, format, type, ByteBuffer.wrap(pixels));
        } else {
          GLES20.glTexImage2D(target, level, internalformat, width, height, border, format, type, null);
        }
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texImage2D(final int target, final int level, final int internalformat, final int width, final int height, final int border, final int format, final int type, final short[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glTexImage2D(target, level, internalformat, width, height, border, format, type, ShortBuffer.wrap(pixels));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texImage2D(final int target, final int level, final int internalformat, final int width, final int height, final int border, final int format, final int type, final int[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glTexImage2D(target, level, internalformat, width, height, border, format, type, IntBuffer.wrap(pixels));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texImage2D(final int target, final int level, final int internalformat, final int width, final int height, final int border, final int format, final int type, final float[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glTexImage2D(target, level, internalformat, width, height, border, format, type, FloatBuffer.wrap(pixels));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texImage2D(final int target, final int level, final int internalformat, final int format, final int type, final CanvasView canvas) {
    final CountDownLatch lock = new CountDownLatch(1);
    final byte[] raw = canvas.snapshot();
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        if (flipYWebGL) {
          flipInPlace(raw, bytesPerPixel(type, format) * canvas.getWidth(), canvas.getHeight());
        }
        GLES20.glTexImage2D(target, level, internalformat, canvas.getWidth(), canvas.getHeight(), 0, format, type, ByteBuffer.wrap(raw));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texImage2D(final int target, final int level, final int internalformat, final int format, final int type, final ImageAsset asset) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        nativeTexImage2DAsset(target, level, internalformat, asset.getWidth(), asset.getHeight(), 0, format, type, asset.nativeImageAsset, flipYWebGL);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texImage2D(final int target, final int level, final int internalformat, final int format, final int type, final Bitmap pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        ByteBuffer storage = bufferFromBitmap(pixels, bytesPerPixel(type, format) * pixels.getWidth(), flipYWebGL);
        // GLUtils.texImage2D(target, level, internalformat, pixels, type, 0);
        //GLUtils.texImage2D(target, level, pixels, 0);
        GLES20.glTexImage2D(target, level, internalformat, pixels.getWidth(), pixels.getHeight(), 0, format, type, storage);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texParameterf(final int target, final int pname, final float param) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glTexParameterf(target, pname, param);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texParameteri(final int target, final int pname, final int param) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glTexParameteri(target, pname, param);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texSubImage2D(final int target, final int level, final int xoffset, final int yoffset, final int width, final int height, final int format, final int type, @Nullable final byte[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        if (pixels != null) {
          if (flipYWebGL) {
            flipInPlace(pixels, bytesPerPixel(type, format) * width, height);
          }
          GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, ByteBuffer.wrap(pixels));
        } else {
          GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, null);
        }
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texSubImage2D(final int target, final int level, final int xoffset, final int yoffset, final int width, final int height, final int format, final int type, final short[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, ShortBuffer.wrap(pixels));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texSubImage2D(final int target, final int level, final int xoffset, final int yoffset, final int width, final int height, final int format, final int type, final int[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, IntBuffer.wrap(pixels));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texSubImage2D(final int target, final int level, final int xoffset, final int yoffset, final int width, final int height, final int format, final int type, final float[] pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, FloatBuffer.wrap(pixels));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texSubImage2D(final int target, final int level, final int xoffset, final int yoffset, final int format, final int type, final CanvasView canvas) {
    final CountDownLatch lock = new CountDownLatch(1);
    final byte[] raw = canvas.snapshot();
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        ByteBuffer buffer = ByteBuffer.wrap(raw);
        GLES20.glTexSubImage2D(target, level, xoffset, yoffset, canvas.getWidth(), canvas.getHeight(), format, type, buffer);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texSubImage2D(final int target, final int level, final int xoffset, final int yoffset, final int format, final int type, final Bitmap pixels) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        ByteBuffer storage = bufferFromBitmap(pixels, bytesPerPixel(type, format) * pixels.getWidth(), flipYWebGL);
        GLES20.glTexSubImage2D(target, level, xoffset, yoffset, pixels.getWidth(), pixels.getHeight(), format, type, storage);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void texSubImage2D(final int target, final int level, final int xoffset, final int yoffset, final int format, final int type, final ImageAsset asset) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        nativeTexSubImage2DAsset(target, level, xoffset, yoffset, asset.getWidth(), asset.getHeight(), format, type, asset.nativeImageAsset, flipYWebGL);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void uniform1f(final int location, final float v0) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform1f(location, v0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void uniform1fv(final int location, final float[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform1fv(location, 1, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void uniform1i(final int location, final int v0) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform1i(location, v0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void uniform1iv(final int location, final int[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform1iv(location, 1, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void uniform2f(final int location, final float v0, final float v1) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform2f(location, v0, v1);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void uniform2fv(final int location, final float[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform1fv(location, 1, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void uniform2i(final int location, final int v0, final int v1) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform2i(location, v0, v1);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void uniform2iv(final int location, final int[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform2iv(location, 1, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void uniform3f(final int location, final float v0, final float v1, final float v2) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform3f(location, v0, v1, v2);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void uniform3fv(final int location, final float[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform3fv(location, 1, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }


  public void uniform3i(final int location, final int v0, final int v1, final int v2) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform3i(location, v0, v1, v2);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void uniform3iv(final int location, final int[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform3iv(location, 1, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void uniform4f(final int location, final float v0, final float v1, final float v2, final float v3) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform4f(location, v0, v1, v2, v3);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void uniform4fv(final int location, final float[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform4fv(location, 1, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void uniform4i(final int location, final int v0, final int v1, final int v2, final int v3) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform4i(location, v0, v1, v2, v3);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void uniform4iv(final int location, final int[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniform4iv(location, 1, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void uniformMatrix2fv(final int location, final boolean transpose, final float[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniformMatrix2fv(location, 1, transpose, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void uniformMatrix3fv(final int location, final boolean transpose, final float[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniformMatrix3fv(location, 1, transpose, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void uniformMatrix4fv(final int location, final boolean transpose, final float[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUniformMatrix4fv(location, 1, transpose, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void useProgram(final int program) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glUseProgram(program);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void validateProgram(final int program) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glValidateProgram(program);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void vertexAttrib1f(final int index, final float v0) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glVertexAttrib1f(index, v0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void vertexAttrib2f(final int index, final float v0, final float v1) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glVertexAttrib2f(index, v0, v1);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void vertexAttrib3f(final int index, final float v0, final float v1, final float v2) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glVertexAttrib3f(index, v0, v1, v2);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void vertexAttrib4f(final int index, final float v0, final float v1, final float v2, final float v3) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glVertexAttrib4f(index, v0, v1, v2, v3);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void vertexAttrib1fv(final int index, final float[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glVertexAttrib1fv(index, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void vertexAttrib2fv(final int index, final float[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glVertexAttrib2fv(index, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void vertexAttrib3fv(final int index, final float[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glVertexAttrib3fv(index, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void vertexAttrib4fv(final int index, final float[] value) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glVertexAttrib4fv(index, value, 0);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void vertexAttribPointer(final int index, final int size, final int type, final boolean normalized, final int stride, final int offset) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        // GLES20.glVertexAttribPointer(index, size, type, normalized, stride, offset);
        nativeVertexAttribPointer(index, size, type, normalized, stride, offset);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  public void viewport(final int x, final int y, final int width, final int height) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES20.glViewport(x, y, width, height);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

  /* Clearing buffers */

  public final int DEPTH_BUFFER_BIT = GLES20.GL_DEPTH_BUFFER_BIT;

  public final int COLOR_BUFFER_BIT = GLES20.GL_COLOR_BUFFER_BIT;

  public final int STENCIL_BUFFER_BIT = GLES20.GL_STENCIL_BUFFER_BIT;

  /* Clearing buffers */

  /* Rendering primitives */

  public final int POINTS = GLES20.GL_POINTS;

  public final int LINES = GLES20.GL_LINES;

  public final int LINE_LOOP = GLES20.GL_LINE_LOOP;

  public final int LINE_STRIP = GLES20.GL_LINE_STRIP;

  public final int TRIANGLES = GLES20.GL_TRIANGLES;

  public final int TRIANGLE_STRIP = GLES20.GL_TRIANGLE_STRIP;

  public final int TRIANGLE_FAN = GLES20.GL_TRIANGLE_FAN;

  /* Rendering primitives */

  /* Blending modes */


  public final int ONE = GLES20.GL_ONE;

  public final int ZERO = GLES20.GL_ZERO;
  public final int SRC_COLOR = GLES20.GL_SRC_COLOR;

  public final int ONE_MINUS_SRC_COLOR = GLES20.GL_ONE_MINUS_SRC_COLOR;

  public final int SRC_ALPHA = GLES20.GL_SRC_ALPHA;

  public final int ONE_MINUS_SRC_ALPHA = GLES20.GL_ONE_MINUS_SRC_ALPHA;

  public final int DST_ALPHA = GLES20.GL_DST_ALPHA;

  public final int ONE_MINUS_DST_ALPHA = GLES20.GL_ONE_MINUS_DST_ALPHA;

  public final int DST_COLOR = GLES20.GL_DST_COLOR;

  public final int ONE_MINUS_DST_COLOR = GLES20.GL_ONE_MINUS_DST_COLOR;

  public final int SRC_ALPHA_SATURATE = GLES20.GL_SRC_ALPHA_SATURATE;

  public final int CONSTANT_COLOR = GLES20.GL_CONSTANT_COLOR;
  public final int ONE_MINUS_CONSTANT_COLOR = GLES20.GL_ONE_MINUS_CONSTANT_COLOR;

  public final int CONSTANT_ALPHA = GLES20.GL_CONSTANT_ALPHA;
  public final int ONE_MINUS_CONSTANT_ALPHA = GLES20.GL_ONE_MINUS_CONSTANT_ALPHA;

  /* Blending modes */

  /* Blending equations */
  public final int FUNC_ADD = GLES20.GL_FUNC_ADD;

  public final int FUNC_SUBTRACT = GLES20.GL_FUNC_SUBTRACT;

  public final int FUNC_REVERSE_SUBTRACT = GLES20.GL_FUNC_REVERSE_SUBTRACT;

  /* Blending equations */


  /* Getting GL parameter information */

  public final int BLEND_EQUATION = GLES20.GL_BLEND_EQUATION;

  public final int BLEND_EQUATION_RGB = GLES20.GL_BLEND_EQUATION_RGB;

  public final int BLEND_EQUATION_ALPHA = GLES20.GL_BLEND_EQUATION_ALPHA;

  public final int BLEND_DST_RGB = GLES20.GL_BLEND_DST_RGB;

  public final int BLEND_SRC_RGB = GLES20.GL_BLEND_SRC_RGB;

  public final int BLEND_DST_ALPHA = GLES20.GL_BLEND_DST_ALPHA;

  public final int BLEND_SRC_ALPHA = GLES20.GL_BLEND_SRC_ALPHA;

  public final int BLEND_COLOR = GLES20.GL_BLEND_COLOR;

  public final int ARRAY_BUFFER_BINDING = GLES20.GL_ARRAY_BUFFER_BINDING;

  public final int ELEMENT_ARRAY_BUFFER_BINDING = GLES20.GL_ELEMENT_ARRAY_BUFFER_BINDING;

  public final int LINE_WIDTH = GLES20.GL_LINE_WIDTH;

  public final int ALIASED_POINT_SIZE_RANGE = GLES20.GL_ALIASED_POINT_SIZE_RANGE;

  public final int ALIASED_LINE_WIDTH_RANGE = GLES20.GL_ALIASED_LINE_WIDTH_RANGE;

  public final int CULL_FACE_MODE = GLES20.GL_CULL_FACE_MODE;

  public final int FRONT_FACE = GLES20.GL_FRONT_FACE;

  public final int DEPTH_RANGE = GLES20.GL_DEPTH_RANGE;

  public final int DEPTH_WRITEMASK = GLES20.GL_DEPTH_WRITEMASK;

  public final int DEPTH_CLEAR_VALUE = GLES20.GL_DEPTH_CLEAR_VALUE;

  public final int DEPTH_FUNC = GLES20.GL_DEPTH_FUNC;

  public final int STENCIL_CLEAR_VALUE = GLES20.GL_STENCIL_CLEAR_VALUE;

  public final int STENCIL_FUNC = GLES20.GL_STENCIL_FUNC;

  public final int STENCIL_FAIL = GLES20.GL_STENCIL_FAIL;

  public final int STENCIL_PASS_DEPTH_FAIL = GLES20.GL_STENCIL_PASS_DEPTH_FAIL;

  public final int STENCIL_PASS_DEPTH_PASS = GLES20.GL_STENCIL_PASS_DEPTH_PASS;

  public final int STENCIL_REF = GLES20.GL_STENCIL_REF;

  public final int STENCIL_VALUE_MASK = GLES20.GL_STENCIL_VALUE_MASK;

  public final int STENCIL_WRITEMASK = GLES20.GL_STENCIL_WRITEMASK;

  public final int STENCIL_BACK_FUNC = GLES20.GL_STENCIL_BACK_FUNC;

  public final int STENCIL_BACK_FAIL = GLES20.GL_STENCIL_BACK_FAIL;

  public final int STENCIL_BACK_PASS_DEPTH_FAIL = GLES20.GL_STENCIL_BACK_PASS_DEPTH_FAIL;

  public final int STENCIL_BACK_PASS_DEPTH_PASS = GLES20.GL_STENCIL_BACK_PASS_DEPTH_PASS;

  public final int STENCIL_BACK_REF = GLES20.GL_STENCIL_BACK_REF;

  public final int STENCIL_BACK_VALUE_MASK = GLES20.GL_STENCIL_BACK_VALUE_MASK;

  public final int STENCIL_BACK_WRITEMASK = GLES20.GL_STENCIL_BACK_WRITEMASK;

  public final int VIEWPORT = GLES20.GL_VIEWPORT;

  public final int SCISSOR_BOX = GLES20.GL_SCISSOR_BOX;

  public final int COLOR_CLEAR_VALUE = GLES20.GL_COLOR_CLEAR_VALUE;

  public final int COLOR_WRITEMASK = GLES20.GL_COLOR_WRITEMASK;

  public final int UNPACK_ALIGNMENT = GLES20.GL_UNPACK_ALIGNMENT;

  public final int PACK_ALIGNMENT = GLES20.GL_PACK_ALIGNMENT;

  public final int MAX_TEXTURE_SIZE = GLES20.GL_MAX_TEXTURE_SIZE;

  public final int MAX_VIEWPORT_DIMS = GLES20.GL_MAX_VIEWPORT_DIMS;

  public final int SUBPIXEL_BITS = GLES20.GL_SUBPIXEL_BITS;

  public final int RED_BITS = GLES20.GL_RED_BITS;

  public final int GREEN_BITS = GLES20.GL_GREEN_BITS;

  public final int BLUE_BITS = GLES20.GL_BLUE_BITS;

  public final int ALPHA_BITS = GLES20.GL_ALPHA_BITS;

  public final int DEPTH_BITS = GLES20.GL_DEPTH_BITS;

  public final int STENCIL_BITS = GLES20.GL_STENCIL_BITS;

  public final int POLYGON_OFFSET_UNITS = GLES20.GL_POLYGON_OFFSET_UNITS;

  public final int POLYGON_OFFSET_FACTOR = GLES20.GL_POLYGON_OFFSET_FACTOR;

  public final int TEXTURE_BINDING_2D = GLES20.GL_TEXTURE_BINDING_2D;

  public final int SAMPLE_BUFFERS = GLES20.GL_SAMPLE_BUFFERS;

  public final int SAMPLES = GLES20.GL_SAMPLES;

  public final int SAMPLE_COVERAGE_VALUE = GLES20.GL_SAMPLE_COVERAGE_VALUE;

  public final int SAMPLE_COVERAGE_INVERT = GLES20.GL_SAMPLE_COVERAGE_INVERT;

  public final int COMPRESSED_TEXTURE_FORMATS = GLES20.GL_COMPRESSED_TEXTURE_FORMATS;

  public final int VENDOR = GLES20.GL_VENDOR;

  public final int RENDERER = GLES20.GL_RENDERER;

  public final int VERSION = GLES20.GL_VERSION;

  public final int IMPLEMENTATION_COLOR_READ_TYPE = GLES20.GL_IMPLEMENTATION_COLOR_READ_TYPE;

  public final int IMPLEMENTATION_COLOR_READ_FORMAT = GLES20.GL_IMPLEMENTATION_COLOR_READ_FORMAT;

  public final int BROWSER_DEFAULT_WEBGL = 0x9244;

  /* Getting GL parameter information */

  /* Buffers */

  public final int STATIC_DRAW = GLES20.GL_STATIC_DRAW;

  public final int STREAM_DRAW = GLES20.GL_STREAM_DRAW;

  public final int DYNAMIC_DRAW = GLES20.GL_DYNAMIC_DRAW;

  public final int ARRAY_BUFFER = GLES20.GL_ARRAY_BUFFER;

  public final int ELEMENT_ARRAY_BUFFER = GLES20.GL_ELEMENT_ARRAY_BUFFER;

  public final int BUFFER_SIZE = GLES20.GL_BUFFER_SIZE;

  public final int BUFFER_USAGE = GLES20.GL_BUFFER_USAGE;

  /* Buffers */

  /* Vertex attributes */

  public final int CURRENT_VERTEX_ATTRIB = GLES20.GL_CURRENT_VERTEX_ATTRIB;

  public final int VERTEX_ATTRIB_ARRAY_ENABLED = GLES20.GL_VERTEX_ATTRIB_ARRAY_ENABLED;

  public final int VERTEX_ATTRIB_ARRAY_SIZE = GLES20.GL_VERTEX_ATTRIB_ARRAY_SIZE;

  public final int VERTEX_ATTRIB_ARRAY_STRIDE = GLES20.GL_VERTEX_ATTRIB_ARRAY_STRIDE;

  public final int VERTEX_ATTRIB_ARRAY_TYPE = GLES20.GL_VERTEX_ATTRIB_ARRAY_TYPE;

  public final int VERTEX_ATTRIB_ARRAY_NORMALIZED = GLES20.GL_VERTEX_ATTRIB_ARRAY_NORMALIZED;

  public final int VERTEX_ATTRIB_ARRAY_POINTER = GLES20.GL_VERTEX_ATTRIB_ARRAY_POINTER;

  public final int VERTEX_ATTRIB_ARRAY_BUFFER_BINDING = GLES20.GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING;

  /* Vertex attributes */

  /* Culling */

  public final int CULL_FACE = GLES20.GL_CULL_FACE;

  public final int FRONT = GLES20.GL_FRONT;

  public final int BACK = GLES20.GL_BACK;

  public final int FRONT_AND_BACK = GLES20.GL_FRONT_AND_BACK;

  /* Culling */

  /* Enabling and disabling */

  public final int BLEND = GLES20.GL_BLEND;

  public final int DEPTH_TEST = GLES20.GL_DEPTH_TEST;

  public final int DITHER = GLES20.GL_DITHER;

  public final int POLYGON_OFFSET_FILL = GLES20.GL_POLYGON_OFFSET_FILL;

  public final int SAMPLE_ALPHA_TO_COVERAGE = GLES20.GL_SAMPLE_ALPHA_TO_COVERAGE;

  public final int SAMPLE_COVERAGE = GLES20.GL_SAMPLE_COVERAGE;

  public final int SCISSOR_TEST = GLES20.GL_SCISSOR_TEST;

  public final int STENCIL_TEST = GLES20.GL_STENCIL_TEST;

  /* Enabling and disabling */

  /* Errors */
  public final int NO_ERROR = GLES20.GL_NO_ERROR;

  public final int INVALID_ENUM = GLES20.GL_INVALID_ENUM;

  public final int INVALID_VALUE = GLES20.GL_INVALID_VALUE;

  public final int INVALID_OPERATION = GLES20.GL_INVALID_OPERATION;

  public final int INVALID_FRAMEBUFFER_OPERATION = GLES20.GL_INVALID_FRAMEBUFFER_OPERATION;

  public final int OUT_OF_MEMORY = GLES20.GL_OUT_OF_MEMORY;

  public final int CONTEXT_LOST_WEBGL = 0x9242;
  /* Errors */

  /* Front face directions */

  public final int CW = GLES20.GL_CW;

  public final int CCW = GLES20.GL_CCW;

  /* Front face directions */


  /* Hints */

  public final int DONT_CARE = GLES20.GL_DONT_CARE;

  public final int FASTEST = GLES20.GL_FASTEST;

  public final int NICEST = GLES20.GL_NICEST;

  public final int GENERATE_MIPMAP_HINT = GLES20.GL_GENERATE_MIPMAP_HINT;

  /* Hints */


  /* Data types */

  public final int BYTE = GLES20.GL_BYTE;

  public final int UNSIGNED_BYTE = GLES20.GL_UNSIGNED_BYTE;

  public final int UNSIGNED_SHORT = GLES20.GL_UNSIGNED_SHORT;

  public final int SHORT = GLES20.GL_SHORT;

  public final int UNSIGNED_INT = GLES20.GL_UNSIGNED_INT;

  public final int INT = GLES20.GL_INT;

  public final int FLOAT = GLES20.GL_FLOAT;

  /* Data types */


  /* Pixel formats */

  public final int DEPTH_COMPONENT = GLES20.GL_DEPTH_COMPONENT;

  public final int ALPHA = GLES20.GL_ALPHA;

  public final int RGB = GLES20.GL_RGB;

  public final int RGBA = GLES20.GL_RGBA;

  public final int LUMINANCE = GLES20.GL_LUMINANCE;

  public final int LUMINANCE_ALPHA = GLES20.GL_LUMINANCE_ALPHA;

  /* Pixel formats */

  /* Pixel types */

  // public final int UNSIGNED_BYTE = GLES20.GL_UNSIGNED_BYTE;

  public final int UNSIGNED_SHORT_4_4_4_4 = GLES20.GL_UNSIGNED_SHORT_4_4_4_4;

  public final int UNSIGNED_SHORT_5_5_5_1 = GLES20.GL_UNSIGNED_SHORT_5_5_5_1;

  public final int UNSIGNED_SHORT_5_6_5 = GLES20.GL_UNSIGNED_SHORT_5_6_5;

  /* Pixel types */

  /* Shaders */

  public final int FRAGMENT_SHADER = GLES20.GL_FRAGMENT_SHADER;

  public final int VERTEX_SHADER = GLES20.GL_VERTEX_SHADER;

  public final int COMPILE_STATUS = GLES20.GL_COMPILE_STATUS;

  public final int DELETE_STATUS = GLES20.GL_DELETE_STATUS;

  public final int LINK_STATUS = GLES20.GL_LINK_STATUS;

  public final int VALIDATE_STATUS = GLES20.GL_VALIDATE_STATUS;

  public final int ATTACHED_SHADERS = GLES20.GL_ATTACHED_SHADERS;

  public final int ACTIVE_ATTRIBUTES = GLES20.GL_ACTIVE_ATTRIBUTES;

  public final int ACTIVE_UNIFORMS = GLES20.GL_ACTIVE_UNIFORMS;

  public final int MAX_VERTEX_UNIFORM_VECTORS = GLES20.GL_MAX_VERTEX_UNIFORM_VECTORS;

  public final int MAX_VARYING_VECTORS = GLES20.GL_MAX_VARYING_VECTORS;

  public final int MAX_COMBINED_TEXTURE_IMAGE_UNITS = GLES20.GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS;

  public final int MAX_VERTEX_TEXTURE_IMAGE_UNITS = GLES20.GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS;

  public final int MAX_TEXTURE_IMAGE_UNITS = GLES20.GL_MAX_TEXTURE_IMAGE_UNITS;

  public final int MAX_VERTEX_ATTRIBS = GLES20.GL_MAX_VERTEX_ATTRIBS;

  public final int MAX_FRAGMENT_UNIFORM_VECTORS = GLES20.GL_MAX_FRAGMENT_UNIFORM_VECTORS;

  public final int SHADER_TYPE = GLES20.GL_SHADER_TYPE;

  public final int SHADING_LANGUAGE_VERSION = GLES20.GL_SHADING_LANGUAGE_VERSION;

  public final int CURRENT_PROGRAM = GLES20.GL_CURRENT_PROGRAM;

  /* Shaders */

  /* Depth or stencil tests */

  public final int NEVER = GLES20.GL_NEVER;

  public final int LESS = GLES20.GL_LESS;

  public final int EQUAL = GLES20.GL_EQUAL;

  public final int LEQUAL = GLES20.GL_LEQUAL;

  public final int GREATER = GLES20.GL_GREATER;

  public final int NOTEQUAL = GLES20.GL_NOTEQUAL;

  public final int GEQUAL = GLES20.GL_GEQUAL;

  public final int ALWAYS = GLES20.GL_ALWAYS;

  /* Depth or stencil tests */

  /* Stencil actions */

  public final int KEEP = GLES20.GL_KEEP;

  public final int REPLACE = GLES20.GL_REPLACE;

  public final int INCR = GLES20.GL_INCR;

  public final int DECR = GLES20.GL_DECR;

  public final int INVERT = GLES20.GL_INVERT;

  public final int INCR_WRAP = GLES20.GL_INCR_WRAP;

  public final int DECR_WRAP = GLES20.GL_DECR_WRAP;

  /* Stencil actions */

  /* Textures */

  public final int NEAREST = GLES20.GL_NEAREST;

  public final int LINEAR = GLES20.GL_LINEAR;

  public final int NEAREST_MIPMAP_NEAREST = GLES20.GL_NEAREST_MIPMAP_NEAREST;

  public final int LINEAR_MIPMAP_NEAREST = GLES20.GL_LINEAR_MIPMAP_NEAREST;

  public final int NEAREST_MIPMAP_LINEAR = GLES20.GL_NEAREST_MIPMAP_LINEAR;

  public final int LINEAR_MIPMAP_LINEAR = GLES20.GL_LINEAR_MIPMAP_LINEAR;

  public final int TEXTURE_MAG_FILTER = GLES20.GL_TEXTURE_MAG_FILTER;

  public final int TEXTURE_MIN_FILTER = GLES20.GL_TEXTURE_MIN_FILTER;

  public final int TEXTURE_WRAP_S = GLES20.GL_TEXTURE_WRAP_S;

  public final int TEXTURE_WRAP_T = GLES20.GL_TEXTURE_WRAP_T;

  public final int TEXTURE_2D = GLES20.GL_TEXTURE_2D;

  public final int TEXTURE = GLES20.GL_TEXTURE;

  public final int TEXTURE_CUBE_MAP = GLES20.GL_TEXTURE_CUBE_MAP;

  public final int TEXTURE_BINDING_CUBE_MAP = GLES20.GL_TEXTURE_BINDING_CUBE_MAP;

  public final int TEXTURE_CUBE_MAP_POSITIVE_X = GLES20.GL_TEXTURE_CUBE_MAP_POSITIVE_X;

  public final int TEXTURE_CUBE_MAP_NEGATIVE_X = GLES20.GL_TEXTURE_CUBE_MAP_NEGATIVE_X;

  public final int TEXTURE_CUBE_MAP_POSITIVE_Y = GLES20.GL_TEXTURE_CUBE_MAP_POSITIVE_Y;

  public final int TEXTURE_CUBE_MAP_NEGATIVE_Y = GLES20.GL_TEXTURE_CUBE_MAP_NEGATIVE_Y;

  public final int TEXTURE_CUBE_MAP_POSITIVE_Z = GLES20.GL_TEXTURE_CUBE_MAP_POSITIVE_Z;

  public final int TEXTURE_CUBE_MAP_NEGATIVE_Z = GLES20.GL_TEXTURE_CUBE_MAP_NEGATIVE_Z;

  public final int MAX_CUBE_MAP_TEXTURE_SIZE = GLES20.GL_MAX_CUBE_MAP_TEXTURE_SIZE;

  public final int TEXTURE0 = GLES20.GL_TEXTURE0;

  public final int TEXTURE1 = GLES20.GL_TEXTURE1;

  public final int TEXTURE2 = GLES20.GL_TEXTURE2;

  public final int TEXTURE3 = GLES20.GL_TEXTURE3;

  public final int TEXTURE4 = GLES20.GL_TEXTURE4;

  public final int TEXTURE5 = GLES20.GL_TEXTURE5;

  public final int TEXTURE6 = GLES20.GL_TEXTURE6;

  public final int TEXTURE7 = GLES20.GL_TEXTURE7;

  public final int TEXTURE8 = GLES20.GL_TEXTURE8;

  public final int TEXTURE9 = GLES20.GL_TEXTURE9;

  public final int TEXTURE10 = GLES20.GL_TEXTURE10;

  public final int TEXTURE11 = GLES20.GL_TEXTURE11;

  public final int TEXTURE12 = GLES20.GL_TEXTURE12;

  public final int TEXTURE13 = GLES20.GL_TEXTURE13;

  public final int TEXTURE14 = GLES20.GL_TEXTURE14;

  public final int TEXTURE15 = GLES20.GL_TEXTURE15;

  public final int TEXTURE16 = GLES20.GL_TEXTURE16;

  public final int TEXTURE17 = GLES20.GL_TEXTURE17;

  public final int TEXTURE18 = GLES20.GL_TEXTURE18;

  public final int TEXTURE19 = GLES20.GL_TEXTURE19;

  public final int TEXTURE20 = GLES20.GL_TEXTURE20;

  public final int TEXTURE21 = GLES20.GL_TEXTURE21;

  public final int TEXTURE22 = GLES20.GL_TEXTURE22;

  public final int TEXTURE23 = GLES20.GL_TEXTURE23;

  public final int TEXTURE24 = GLES20.GL_TEXTURE24;

  public final int TEXTURE25 = GLES20.GL_TEXTURE25;

  public final int TEXTURE26 = GLES20.GL_TEXTURE26;

  public final int TEXTURE27 = GLES20.GL_TEXTURE27;

  public final int TEXTURE28 = GLES20.GL_TEXTURE28;

  public final int TEXTURE29 = GLES20.GL_TEXTURE29;

  public final int TEXTURE30 = GLES20.GL_TEXTURE30;

  public final int TEXTURE31 = GLES20.GL_TEXTURE31;

  public final int ACTIVE_TEXTURE = GLES20.GL_ACTIVE_TEXTURE;

  public final int REPEAT = GLES20.GL_REPEAT;

  public final int CLAMP_TO_EDGE = GLES20.GL_CLAMP_TO_EDGE;

  public final int MIRRORED_REPEAT = GLES20.GL_MIRRORED_REPEAT;

  /* Textures */



  /* Uniform types */

  public final int FLOAT_VEC2 = GLES20.GL_FLOAT_VEC2;

  public final int FLOAT_VEC3 = GLES20.GL_FLOAT_VEC3;

  public final int FLOAT_VEC4 = GLES20.GL_FLOAT_VEC4;

  public final int INT_VEC2 = GLES20.GL_INT_VEC2;

  public final int INT_VEC3 = GLES20.GL_INT_VEC3;

  public final int INT_VEC4 = GLES20.GL_INT_VEC4;


  public final int BOOL = GLES20.GL_BOOL;


  public final int BOOL_VEC2 = GLES20.GL_BOOL_VEC2;

  public final int BOOL_VEC3 = GLES20.GL_BOOL_VEC3;

  public final int BOOL_VEC4 = GLES20.GL_BOOL_VEC4;


  public final int FLOAT_MAT2 = GLES20.GL_FLOAT_MAT2;


  public final int FLOAT_MAT3 = GLES20.GL_FLOAT_MAT3;


  public final int FLOAT_MAT4 = GLES20.GL_FLOAT_MAT4;

  public final int SAMPLER_2D = GLES20.GL_SAMPLER_2D;

  public final int SAMPLER_CUBE = GLES20.GL_SAMPLER_CUBE;

  /* Uniform types */

  /* Shader precision-specified types */

  public final int LOW_FLOAT = GLES20.GL_LOW_FLOAT;
  public final int MEDIUM_FLOAT = GLES20.GL_MEDIUM_FLOAT;
  public final int HIGH_FLOAT = GLES20.GL_HIGH_FLOAT;
  public final int LOW_INT = GLES20.GL_LOW_INT;
  public final int MEDIUM_INT = GLES20.GL_MEDIUM_INT;
  public final int HIGH_INT = GLES20.GL_HIGH_INT;

  /* Shader precision-specified types */


  /* Framebuffers and renderbuffers */

  public final int FRAMEBUFFER = GLES20.GL_FRAMEBUFFER;

  public final int RENDERBUFFER = GLES20.GL_RENDERBUFFER;

  public final int RGBA4 = GLES20.GL_RGBA4;

  public final int RGB565 = GLES20.GL_RGB565;

  public final int RGB5_A1 = GLES20.GL_RGB5_A1;

  public final int DEPTH_COMPONENT16 = GLES20.GL_DEPTH_COMPONENT16;

  public final int STENCIL_INDEX8 = GLES20.GL_STENCIL_INDEX8;

  public final int DEPTH_STENCIL = 0x84F9;

  public final int RENDERBUFFER_WIDTH = GLES20.GL_RENDERBUFFER_WIDTH;

  public final int RENDERBUFFER_HEIGHT = GLES20.GL_RENDERBUFFER_HEIGHT;

  public final int RENDERBUFFER_INTERNAL_FORMAT = GLES20.GL_RENDERBUFFER_INTERNAL_FORMAT;

  public final int RENDERBUFFER_RED_SIZE = GLES20.GL_RENDERBUFFER_RED_SIZE;

  public final int RENDERBUFFER_GREEN_SIZE = GLES20.GL_RENDERBUFFER_GREEN_SIZE;

  public final int RENDERBUFFER_BLUE_SIZE = GLES20.GL_RENDERBUFFER_BLUE_SIZE;

  public final int RENDERBUFFER_ALPHA_SIZE = GLES20.GL_RENDERBUFFER_ALPHA_SIZE;

  public final int RENDERBUFFER_DEPTH_SIZE = GLES20.GL_RENDERBUFFER_DEPTH_SIZE;

  public final int RENDERBUFFER_STENCIL_SIZE = GLES20.GL_RENDERBUFFER_STENCIL_SIZE;

  public final int FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE = GLES20.GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE;

  public final int FRAMEBUFFER_ATTACHMENT_OBJECT_NAME = GLES20.GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME;

  public final int FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL = GLES20.GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL;

  public final int FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE = GLES20.GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE;

  public final int COLOR_ATTACHMENT0 = GLES20.GL_COLOR_ATTACHMENT0;

  public final int DEPTH_ATTACHMENT = GLES20.GL_DEPTH_ATTACHMENT;

  public final int STENCIL_ATTACHMENT = GLES20.GL_STENCIL_ATTACHMENT;

  public final int DEPTH_STENCIL_ATTACHMENT = 0x821A;

  public final int NONE = GLES20.GL_NONE;

  public final int FRAMEBUFFER_COMPLETE = GLES20.GL_FRAMEBUFFER_COMPLETE;

  public final int FRAMEBUFFER_INCOMPLETE_ATTACHMENT = GLES20.GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT;

  public final int FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT = GLES20.GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT;

  public final int FRAMEBUFFER_INCOMPLETE_DIMENSIONS = GLES20.GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS;

  public final int FRAMEBUFFER_UNSUPPORTED = GLES20.GL_FRAMEBUFFER_UNSUPPORTED;

  public final int FRAMEBUFFER_BINDING = GLES20.GL_FRAMEBUFFER_BINDING;

  public final int RENDERBUFFER_BINDING = GLES20.GL_RENDERBUFFER_BINDING;

  public final int MAX_RENDERBUFFER_SIZE = GLES20.GL_MAX_RENDERBUFFER_SIZE;

  //public final int INVALID_FRAMEBUFFER_OPERATION = GLES20.GL_INVALID_FRAMEBUFFER_OPERATION;

  /* Framebuffers and renderbuffers */

  /* Pixel storage modes */

  public final int UNPACK_COLORSPACE_CONVERSION_WEBGL = 0x9243;

  public final int UNPACK_FLIP_Y_WEBGL = 0x9240;

  public final int UNPACK_PREMULTIPLY_ALPHA_WEBGL = 0x9241;

  /* Pixel storage modes */
}
