package com.github.triniwiz.canvas;

import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.graphics.Color;
import android.util.Log;
import android.view.ViewGroup;

import java.nio.ByteBuffer;
import java.util.concurrent.CountDownLatch;


/**
 * Created by triniwiz on 2019-07-06
 */
public class CanvasRenderingContext2D implements CanvasRenderingContext {
  private static native long nativeRect(long canvas_ptr, float x, float y, float width, float height);

  private static native long nativeStroke(long canvas_ptr);

  private static native long nativeStrokePath(long canvas_ptr, long path);

  private static native long nativeFill(long canvas_ptr);

  private static native long nativeFillRule(long canvas_ptr, String rule);

  private static native long nativeFillPathRule(long canvas_ptr, long path, String rule);

  private static native long nativeFillRect(long canvas_ptr, float x, float y, float width, float height);

  private static native long nativeStrokeRect(long canvas_ptr, float x, float y, float width, float height);

  private static native long nativeFillText(long canvas_ptr, String text, float x, float y, float width);

  private static native long nativeStrokeText(long canvas_ptr, String text, float x, float y, float width);

  private static native long nativeBeginPath(long canvas_ptr);

  private static native long nativeMoveTo(long canvas_ptr, float x, float y);

  private static native long nativeLineTo(long canvas_ptr, float x, float y);

  private static native long nativeClosePath(long canvas_ptr);

  private static native long nativeArc(long canvas_ptr, float x, float y, float radius, float startAngle, float endAngle, boolean anticlockwise);

  private static native long nativeArcTo(long canvas_ptr, float x1, float y1, float x2, float y2, float radius);

  private static native long nativeBezierCurveTo(long canvas_ptr, float cp1x, float cp1y, float cp2x, float cp2y, float x, float y);

  private static native long nativeEllipse(long canvas_ptr, float x, float y, float radiusX, float radiusY, float rotation, float startAngle, float endAngle, boolean anticlockwise);

  private static native long nativeClip(long canvas_ptr);

  private static native long nativeClipRule(long canvas_ptr, String rule);

  private static native long nativeClipPathRule(long canvas_ptr, long path, String rule);

  private static native long nativeSetLineWidth(long canvas_ptr, float lineWidth);

  private static native long nativeSetGlobalCompositeOperation(long canvas_ptr, String composite);

  private static native long nativeSetGlobalAlpha(long canvas_ptr, int alpha);

  private static native long nativeSetTextAlignment(long canvas_ptr, String alignment);

  private static native long nativeSave(long canvas_ptr);

  private static native long nativeRestore(long canvas_ptr);

  private static native long nativeSetTransform(long canvas_ptr, float a, float b, float c, float d, float e, float f);

  private static native long nativeTransform(long canvas, float a, float b, float c, float d, float e, float f);

  private static native long nativeScale(long canvas, float x, float y);

  private static native long nativeRotate(long canvas, float angle);

  private static native long nativeTranslate(long canvas, float x, float y);

  private static native long nativeQuadraticCurveTo(long canvas, float cpx, float cpy, float x, float y);

  private static native long nativeDrawImageCanvas(long canvas, byte[] image, int width, int height, float dx, float dy);

  private static native long nativeDrawImage(long canvas, Bitmap image, float dx, float dy);

  private static native long nativeDrawImageRaw(long canvas, byte[] pixels, int originalWidth, int originalHeight, float dx, float dy);

  private static native long nativeDrawImageCanvasDw(long canvas, byte[] image, int width, int height, float dx, float dy, float dWidth, float dHeight);

  private static native long nativeDrawImageDw(long canvas, Bitmap image, float dx, float dy, float dWidth, float dHeight);

  private static native long nativeDrawImageDwRaw(long canvas, byte[] pixels, int originalWidth, int originalHeight, float dx, float dy, float dWidth, float dHeight);

  private static native long nativeDrawImageCanvasSw(long canvas, byte[] image, int width, int height, float sx, float sy, float sWidth, float sHeight, float dx, float dy, float dWidth, float dHeight);

  private static native long nativeDrawImageSw(long canvas, Bitmap image, float sx, float sy, float sWidth, float sHeight, float dx, float dy, float dWidth, float dHeight);

  private static native long nativeDrawImageSwRaw(long canvas, byte[] pixels, int originalWidth, int originalHeight, float sx, float sy, float sWidth, float sHeight, float dx, float dy, float dWidth, float dHeight);

  private static native long nativeClearRect(long canvas_ptr, float x, float y, float width, float height);

  private static native long nativeSetFillColorRgba(long canvas_ptr, int r, int g, int b, int a);

  private static native long nativeSetFillColor(long canvas_ptr, int color);

  private static native long nativeSetStrokeColorRgba(long canvas_ptr, int r, int g, int b, int a);

  private static native long nativeSetStrokeColor(long canvas_ptr, int color);

  private static native long nativeSetShadowBlur(long canvas, float blur);

  private static native long nativeSetShadowColor(long canvas, int color);

  private static native long nativeSetShadowOffsetX(long canvas, float x);

  private static native long nativeSetShadowOffsetY(long canvas, float y);

  private static native long nativeSetFont(long canvas, String font);

  private static native ByteBuffer nativeCreateImageData(int width, int height);

  private static native long nativePutImageData(long canvas, int width, int height, ByteBuffer data, float x, float y, float dirtyX, float dirtyY, int dirtyWidth, int dirtyHeight);

  private static native long nativeSetImageSmoothingEnabled(long canvas, boolean enabled);

  private static native long nativeSetImageSmoothingQuality(long canvas, String quality);

  private static native long nativeSetLineCap(long canvas, String toString);

  private static native long nativeSetLineJoin(long canvas, String toString);

  private static native long nativeSetFillGradientRadial(long canvas, float x0, float y0, float r0, float x1, float y1, float r1, int[] rawValues, float[] rawKeys);

  private static native long nativeSetFillGradientLinear(long canvas, float x0, float y0, float x1, float y1, int[] rawValues, float[] rawKeys);

  private static native long nativeSetStrokeGradientRadial(long canvas, float x0, float y0, float r0, float x1, float y1, float r1, int[] rawValues, float[] rawKeys);

  private static native long nativeSetStrokeGradientLinear(long canvas, float x0, float y0, float x1, float y1, int[] rawValues, float[] rawKeys);

  private static native long nativeSetLineDashOffset(long canvas, float offset);

  private static native long nativeSetLineDash(long canvas, float[] dash);

  private static native long nativeResetTransform(long canvas);

  private static native long nativeSetMiterLimit(long canvas, float limit);

  private static native CanvasTextMetrics nativeMeasureText(long canvas, String text);

  private static native long nativeSetCurrentTransform(long canvas, long matrix);

  private static native long nativeGetCurrentTransform(long canvas);

  private static native boolean nativeIsPointInPath(long canvas, float x, float y);

  private static native boolean nativeIsPointInPathWithRule(long canvas, float x, float y, String fillRule);

  private static native boolean nativeIsPointInPathWithPathRule(long canvas, long path, float x, float y, String fillRule);

  private static native boolean nativeIsPointInStroke(long canvas, float x, float y);

  private static native boolean nativeIsPointInStrokeWithPath(long canvas, long path, float x, float y);

  private static native ByteBuffer nativeGetImageData(long canvas, float sx, float sy, int width, int height);

  private static native long nativeSetFillPattern(long canvas, long pattern);

  private static native long nativeSetStrokePattern(long canvas, long pattern);

  private static native String nativeGetDirection(long canvas);

  private static native long nativeSetDirection(long canvas, String direction);

  private CanvasView canvasView;

  CanvasRenderingContext2D(CanvasView view) {
    this.canvasView = view;
  }

  public static boolean isDebug = true;

  public enum LineCap {
    Butt("butt"),
    Round("round"),
    Square("square");
    private String lineCap;

    LineCap(String lineCap) {
      this.lineCap = lineCap;
    }

    @Override
    public String toString() {
      return lineCap;
    }
  }

  public enum LineJoin {
    Bevel("bevel"),
    Round("round"),
    Miter("miter");
    private String lineJoin;

    LineJoin(String lineJoin) {
      this.lineJoin = lineJoin;
    }

    @Override
    public String toString() {
      return lineJoin;
    }
  }

  public enum ImageSmoothingQuality {
    Low("low"),
    Medium("medium"),
    High("high");

    private String quality;

    ImageSmoothingQuality(String quality) {
      this.quality = quality;
    }

    @Override
    public String toString() {
      return quality;
    }
  }

  final Object lock = new Object();
  long state = 0;
  long currentPath = 0;
  private boolean imageSmoothingEnabled = false;
  private ImageSmoothingQuality imageSmoothingQuality = ImageSmoothingQuality.Low;
  private float lineWidth = 1;
  private Object fillStyle = Color.BLACK;
  private Object strokeStyle = Color.BLACK;
  private CanvasCompositeOperationType globalCompositeOperation = CanvasCompositeOperationType.SourceOver;
  private CanvasTextAlignment textAlign = CanvasTextAlignment.Start;
  private float globalAlpha = 1;
  private String font = "10px sans-serif";
  private LineCap lineCap = LineCap.Butt;
  private LineJoin lineJoin = LineJoin.Miter;
  private float lineDashOffset = 0f;
  private float miterLimit = 10;
  private String direction = "ltr";
  private float shadowBlur = 0;
  private int shadowColor = Color.TRANSPARENT;
  private float shadowOffsetX = 0;
  private float shadowOffsetY = 0;
  private float[] lineDash = new float[0];
  public boolean block = false;

  public void setDirection(final String direction) {
    if (direction.equals("ltr") || direction.equals("rtl")) {
      canvasView.canvas = nativeSetDirection(canvasView.canvas, direction);
      this.direction = direction;
    }
  }

  static final String TAG = "CanvasRenderingContext";

  private static void printLog(String msg) {
    if (isDebug) {
      Log.d(TAG, msg);
    }
  }

  public String getDirection() {
    return direction;
  }

  public Object getFillStyle() {
    return fillStyle;
  }

  public void setFillStyle(final int fillStyle) {
    printLog("setFillStyle int");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetFillColor(canvasView.canvas, fillStyle);
      }
    });

    this.fillStyle = fillStyle;
  }

  public void setFillStyle(final Object fillStyle) {
    printLog("setFillStyle obj");
    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        if (fillStyle instanceof ICanvasColorStyle) {
          switch (((ICanvasColorStyle) fillStyle).getStyleType()) {
            case Color:
              com.github.triniwiz.canvas.Color color = (com.github.triniwiz.canvas.Color) fillStyle;
              canvasView.canvas = nativeSetFillColorRgba(canvasView.canvas, Color.red(color.color), Color.green(color.color), Color.blue(color.color), Color.alpha(color.color));
              break;
            case Pattern:
              Pattern pattern = (Pattern) fillStyle;
              canvasView.canvas = nativeSetFillPattern(canvasView.canvas, pattern.nativePattern);
              break;
            case Gradient:
              Gradient gradient = (Gradient) fillStyle;
              if (gradient instanceof LinearGradient) {
                LinearGradient g = (LinearGradient) gradient;
                canvasView.canvas = nativeSetFillGradientLinear(canvasView.canvas, g.x0, g.y0, g.x1, g.y1, gradient.getColors(), gradient.getPositions());
              } else if (gradient instanceof RadialGradient) {
                RadialGradient g = (RadialGradient) gradient;
                canvasView.canvas = nativeSetFillGradientRadial(canvasView.canvas, g.x0, g.y0, g.r0, g.x1, g.y1, g.r1, gradient.getColors(), gradient.getPositions());
              }
              break;
          }
        } else {
          if (fillStyle instanceof String) {
            com.github.triniwiz.canvas.Color color = new com.github.triniwiz.canvas.Color((String) fillStyle);
            canvasView.canvas = nativeSetFillColorRgba(canvasView.canvas, Color.red(color.color), Color.green(color.color), Color.blue(color.color), Color.alpha(color.color));
          } else if (fillStyle != null) {
            canvasView.canvas = nativeSetFillColor(canvasView.canvas, (int) fillStyle);
          }
        }

      }
    });

    if (fillStyle != null) {
      this.fillStyle = fillStyle;
    }
  }

  public Object getStrokeStyle() {
    return strokeStyle;
  }

  public void setStrokeStyle(final int strokeStyle) {
    printLog("setStrokeStyle int");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetStrokeColor(canvasView.canvas, strokeStyle);
      }
    });

    this.strokeStyle = strokeStyle;
  }

  public void setStrokeStyle(final Object strokeStyle) {
    printLog("setStrokeStyle obj");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        if (strokeStyle instanceof ICanvasColorStyle) {
          switch (((ICanvasColorStyle) strokeStyle).getStyleType()) {
            case Color:
              com.github.triniwiz.canvas.Color color = (com.github.triniwiz.canvas.Color) strokeStyle;
              canvasView.canvas = CanvasRenderingContext2D.nativeSetStrokeColorRgba(canvasView.canvas, Color.red(color.color), Color.green(color.color), Color.blue(color.color), Color.alpha(color.color));
              break;
            case Pattern:
              Pattern pattern = (Pattern) strokeStyle;
              canvasView.canvas = nativeSetStrokePattern(canvasView.canvas, pattern.nativePattern);
              break;
            case Gradient:
              Gradient gradient = (Gradient) strokeStyle;
              if (gradient instanceof LinearGradient) {
                LinearGradient g = (LinearGradient) gradient;
                canvasView.canvas = CanvasRenderingContext2D.nativeSetStrokeGradientLinear(canvasView.canvas, g.x0, g.y0, g.x1, g.y1, gradient.getColors(), gradient.getPositions());
              } else if (gradient instanceof RadialGradient) {
                RadialGradient g = (RadialGradient) gradient;
                canvasView.canvas = CanvasRenderingContext2D.nativeSetStrokeGradientRadial(canvasView.canvas, g.x0, g.y0, g.r0, g.x1, g.y1, g.r1, gradient.getColors(), gradient.getPositions());
              }
              break;
          }
        } else {
          if (strokeStyle instanceof String) {
            com.github.triniwiz.canvas.Color color = new com.github.triniwiz.canvas.Color((String) strokeStyle);
            canvasView.canvas = nativeSetStrokeColorRgba(canvasView.canvas, Color.red(color.color), Color.green(color.color), Color.blue(color.color), Color.alpha(color.color));
          } else if (strokeStyle != null) {
            canvasView.canvas = nativeSetStrokeColor(canvasView.canvas, (int) strokeStyle);
          }
        }

      }
    });

    if (strokeStyle != null) {
      this.strokeStyle = strokeStyle;
    }
  }

  public float getLineWidth() {
    return lineWidth;
  }

  public void setLineWidth(final float lineWidth) {
    printLog("setLineWidth");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetLineWidth(canvasView.canvas, lineWidth);
      }
    });

    this.lineWidth = lineWidth;
  }

  public void setLineCap(final LineCap lineCap) {
    printLog("setLineCap");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetLineCap(canvasView.canvas, lineCap.toString());

      }
    });

    this.lineCap = lineCap;
  }

  public LineCap getLineCap() {
    return lineCap;
  }

  public void setLineJoin(final LineJoin lineJoin) {
    printLog("setLineJoin");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetLineJoin(canvasView.canvas, lineJoin.toString());

      }
    });

    this.lineJoin = lineJoin;
  }

  public LineJoin getLineJoin() {
    return lineJoin;
  }

  public void setMiterLimit(final float limit) {
    printLog("setMiterLimit");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetMiterLimit(canvasView.canvas, limit);

      }
    });

    this.miterLimit = limit;
  }

  public float getMiterLimit() {
    return miterLimit;
  }

  public void setLineDashOffset(final float offset) {
    printLog("setLineDashOffset");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetLineDashOffset(canvasView.canvas, offset);

      }
    });

    this.lineDashOffset = offset;
  }

  public float getLineDashOffset() {
    return lineDashOffset;
  }

  public float[] getLineDash() {
    return lineDash;
  }

  public void setLineDash(final float[] dash) {
    printLog("setLineDash");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetLineDash(canvasView.canvas, dash);

      }
    });

    this.lineDash = dash;
  }


  private void updateCanvas() {
    // synchronized (canvasView.lock) {
    canvasView.pendingInvalidate = true;
    //}
  }

  public void clearRect(final float x, final float y, final float width, final float height) {
    printLog("clearRect");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeClearRect(canvasView.canvas, x, y, width, height);
        updateCanvas();

      }
    });

  }

  public void fillRect(final float x, final float y, final float width, final float height) {
    printLog("fillRect");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeFillRect(canvasView.canvas, x, y, width, height);
        updateCanvas();

      }
    });

  }

  public void strokeRect(final float x, final float y, final float width, final float height) {
    printLog("strokeRect");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeStrokeRect(canvasView.canvas, x, y, width, height);
        updateCanvas();

      }
    });

  }

  public void fillText(String text, float x, float y) {
    fillText(text, x, y, 0);
  }

  public void fillText(final String text, final float x, final float y, final float width) {
    printLog("fillText");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeFillText(canvasView.canvas, text, x, y, width);
        updateCanvas();

      }
    });

  }

  public void strokeText(String text, float x, float y) {
    strokeText(text, x, y, 0);
  }

  public void strokeText(final String text, final float x, final float y, final float width) {
    printLog("strokeText");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeStrokeText(canvasView.canvas, text, x, y, width);
        updateCanvas();

      }
    });

  }

  synchronized long getCanvasPointer() {
    return canvasView.canvas;
  }

  synchronized void setCanvasPointer(long canvas) {
    canvasView.canvas = canvas;
  }

  public void rect(final float x, final float y, final float width, final float height) {
    printLog("rect");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeRect(canvasView.canvas, x, y, width, height);

      }
    });

  }

  public void fill() {
    printLog("fill");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeFill(canvasView.canvas);
        updateCanvas();

      }
    });

  }

  public void fill(final String rule) {
    printLog("fill: rule");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeFillRule(canvasView.canvas, rule);
        updateCanvas();

      }
    });

  }

  public void fill(final CanvasPath2D path, final String rule) {
    printLog("fill: path rule");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeFillPathRule(canvasView.canvas, path.path, rule);
        updateCanvas();
      }
    });

  }

  public void fill(final CanvasPath2D path) {
    printLog("fill: path");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeFillPathRule(canvasView.canvas, path.path, "nonzero");
        updateCanvas();

      }
    });

  }

  public void stroke() {
    printLog("stroke");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeStroke(canvasView.canvas);
        updateCanvas();

      }
    });

  }

  public void stroke(final CanvasPath2D path) {
    printLog("stroke: path");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeStrokePath(canvasView.canvas, path.path);
        updateCanvas();

      }
    });

  }

  public void beginPath() {
    printLog("beginPath");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeBeginPath(canvasView.canvas);

      }
    });

  }

  public void moveTo(final float x, final float y) {
    printLog("moveTo");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeMoveTo(canvasView.canvas, x, y);

      }
    });

  }

  public void lineTo(final float x, final float y) {
    printLog("lineTo");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeLineTo(canvasView.canvas, x, y);

      }
    });

  }

  public void closePath() {
    printLog("closePath");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeClosePath(canvasView.canvas);

      }
    });

  }

  public void arc(float x, float y, float radius, float startAngle, float endAngle) {
    arc(x, y, radius, startAngle, endAngle, false);
  }

  public void arc(final float x, final float y, final float radius, final float startAngle, final float endAngle, final boolean anticlockwise) {
    printLog("arc");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeArc(canvasView.canvas, x, y, radius, startAngle, endAngle, anticlockwise);

      }
    });

  }

  public void arcTo(final float x1, final float y1, final float x2, final float y2, final float radius) {
    printLog("arcTo");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeArcTo(canvasView.canvas, x1, y1, x2, y2, radius);

      }
    });

  }

  public void bezierCurveTo(final float cp1x, final float cp1y, final float cp2x, final float cp2y, final float x, final float y) {
    printLog("bezierCurveTo");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeBezierCurveTo(canvasView.canvas, cp1x, cp1y, cp2x, cp2y, x, y);

      }
    });

  }

  public void ellipse(float x, float y, float radiusX, float radiusY, float rotation, float startAngle, float endAngle) {
    ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, false);
  }

  public void ellipse(final float x, final float y, final float radiusX, final float radiusY, final float rotation, final float startAngle, final float endAngle, final boolean anticlockwise) {
    printLog("ellipse");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeEllipse(canvasView.canvas, x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise);

      }
    });

  }

  public void clip() {
    printLog("clip");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeClip(canvasView.canvas);

      }
    });

  }

  public void clip(final String rule) {
    printLog("clip: rule");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeClipRule(canvasView.canvas, rule);

      }
    });

  }

  public void clip(final CanvasPath2D path) {
    printLog("clip: path");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeClipPathRule(canvasView.canvas, path.path, "nonzero");

      }
    });

  }

  public void clip(final CanvasPath2D path, final String rule) {
    printLog("clip: path rule");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeClipPathRule(canvasView.canvas, path.path, rule);

      }
    });

  }

  public CanvasView getCanvas() {
    return canvasView;
  }

  public LinearGradient createLinearGradient(float x0, float y0, float x1, float y1) {
    printLog("createLinearGradient");
    return new LinearGradient(x0, y0, x1, y1);
  }

  public RadialGradient createRadialGradient(float x0, float y0, float r0, float x1, float y1, float r1) {
    printLog("createRadialGradient");
    return new RadialGradient(x0, y0, r0, x1, y1, r1);
  }

  public Pattern createPattern(CanvasView src, Pattern.PatternRepetition repetition) {
    printLog("createPattern: canvas");
    return new Pattern(canvasView, src, repetition);
  }

  public Pattern createPattern(Bitmap src, Pattern.PatternRepetition repetition) {
    printLog("createPattern: bitmap");
    return new Pattern(canvasView, src, repetition);
  }

  public Pattern createPattern(ImageAsset src, Pattern.PatternRepetition repetition) {
    printLog("createPattern: asset");
    return new Pattern(canvasView, src, repetition);
  }

  public CanvasCompositeOperationType getGlobalCompositeOperation() {
    return globalCompositeOperation;
  }

  public void setGlobalCompositeOperation(final CanvasCompositeOperationType globalCompositeOperation) {
    printLog("setGlobalCompositeOperation");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetGlobalCompositeOperation(canvasView.canvas, globalCompositeOperation.type);

      }
    });

    this.globalCompositeOperation = globalCompositeOperation;
  }


  public float getGlobalAlpha() {
    return globalAlpha;
  }

  public void setGlobalAlpha(float alpha) {
    printLog("setGlobalAlpha");
    if (alpha == 0 || alpha > 1) {
      alpha = 1;
    }
    final int globalAlpha = (int) (alpha * 255);

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetGlobalAlpha(canvasView.canvas, globalAlpha);

      }
    });

    this.globalAlpha = globalAlpha;
  }

  public CanvasTextAlignment getTextAlign() {
    return textAlign;
  }


  public void setTextAlign(final CanvasTextAlignment textAlign) {
    printLog("setTextAlign");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetTextAlignment(canvasView.canvas, textAlign.toString());

      }
    });

  }

  public void save() {
    printLog("save");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSave(canvasView.canvas);
        updateCanvas();

      }
    });

  }

  public void restore() {
    printLog("restore");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeRestore(canvasView.canvas);
        updateCanvas();

      }
    });

  }


  public void setTransform(final float a, final float b, final float c, final float d, final float e, final float f) {
    printLog("setTransform");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetTransform(canvasView.canvas, a, b, c, d, e, f);

      }
    });

  }

  public void transform(final float a, final float b, final float c, final float d, final float e, final float f) {
    printLog("transform");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeTransform(canvasView.canvas, a, b, c, d, e, f);

      }
    });

  }

  public void scale(final float x, final float y) {
    printLog("scale");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeScale(canvasView.canvas, x, y);

      }
    });

  }

  public void rotate(final float angle) {
    printLog("rotate");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeRotate(canvasView.canvas, angle);

      }
    });

  }

  public void translate(final float x, final float y) {
    printLog("translate");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeTranslate(canvasView.canvas, x, y);

      }
    });

  }

  public void quadraticCurveTo(final float cpx, final float cpy, final float x, final float y) {
    printLog("quadraticCurveTo");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeQuadraticCurveTo(canvasView.canvas, cpx, cpy, x, y);

      }
    });

  }


  public void drawImage(final CanvasView image, final float dx, final float dy) {
    printLog("drawImage: canvas");

    final byte[] ss = image.snapshot();
    int width = image.getWidth();
    int height = image.getHeight();
    if (width == 0) {
      ViewGroup.LayoutParams params = image.getLayoutParams();
      if (params != null) {
        width = params.width;
      }
    }

    if (height == 0) {
      ViewGroup.LayoutParams params = image.getLayoutParams();
      if (params != null) {
        height = params.height;
      }
    }
    if (width < 1) {
      width = 1;
    }
    if (height < 1) {
      height = 1;
    }
    final int finalWidth = width;
    final int finalHeight = height;
    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeDrawImageCanvas(canvasView.canvas, ss, finalWidth, finalHeight, dx, dy);
        updateCanvas();

      }
    });

  }

  public void drawImage(final Bitmap image, final float dx, final float dy) {
    printLog("drawImage: bitmap");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeDrawImage(canvasView.canvas, image, dx, dy);
        updateCanvas();

      }
    });

  }


  public void drawImage(final ImageAsset asset, final float dx, final float dy) {
    printLog("drawImage: asset");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeDrawImageRaw(canvasView.canvas, asset.getBytes(), asset.getWidth(), asset.getHeight(), dx, dy);
        updateCanvas();

      }
    });

  }

  public void drawImage(final CanvasView image, final float dx, final float dy, final float dWidth, final float dHeight) {
    printLog("drawImage: canvas");

    final byte[] ss = image.snapshot();
    int width = image.getWidth();
    int height = image.getHeight();
    if (width == 0) {
      ViewGroup.LayoutParams params = image.getLayoutParams();
      if (params != null) {
        width = params.width;
      }
    }

    if (height == 0) {
      ViewGroup.LayoutParams params = image.getLayoutParams();
      if (params != null) {
        height = params.height;
      }
    }
    if (width < 1) {
      width = 1;
    }
    if (height < 1) {
      height = 1;
    }
    final int finalWidth = width;
    final int finalHeight = height;
    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeDrawImageCanvasDw(canvasView.canvas, ss, finalWidth, finalHeight, dx, dy, dWidth, dHeight);
        updateCanvas();

      }
    });

  }

  public void drawImage(final Bitmap image, final float dx, final float dy, final float dWidth, final float dHeight) {
    printLog("drawImage: bitmap");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeDrawImageDw(canvasView.canvas, image, dx, dy, dWidth, dHeight);
        updateCanvas();

      }
    });

  }

  public void drawImage(final ImageAsset asset, final float dx, final float dy, final float dWidth, final float dHeight) {
    printLog("drawImage: asset");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeDrawImageDwRaw(canvasView.canvas, asset.getBytes(), asset.getWidth(), asset.getHeight(), dx, dy, dWidth, dHeight);
        updateCanvas();

      }
    });

  }

  public void drawImage(final CanvasView image, final float sx, final float sy, final float sWidth, final float sHeight, final float dx, final float dy, final float dWidth, final float dHeight) {
    printLog("drawImage: canvas");

    final byte[] ss = image.snapshot();
    int width = image.getWidth();
    int height = image.getHeight();
    if (image.getWidth() == 0) {
      ViewGroup.LayoutParams params = image.getLayoutParams();
      if (params != null) {
        width = params.width;
      }
    }

    if (image.getHeight() == 0) {
      ViewGroup.LayoutParams params = image.getLayoutParams();
      if (params != null) {
        height = params.height;
      }
    }
    if (width < 1) {
      width = 1;
    }
    if (height < 1) {
      height = 1;
    }

    final int finalWidth = width;
    final int finalHeight = height;
    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeDrawImageCanvasSw(canvasView.canvas, ss, finalWidth, finalHeight, sx, sy, sWidth, sHeight, dx, dy, dWidth, dHeight);
        updateCanvas();

      }
    });

  }

  public void drawImage(final Bitmap image, final float sx, final float sy, final float sWidth, final float sHeight, final float dx, final float dy, final float dWidth, final float dHeight) {
    printLog("drawImage: bitmap");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeDrawImageSw(canvasView.canvas, image, sx, sy, sWidth, sHeight, dx, dy, dWidth, dHeight);
        updateCanvas();

      }
    });

  }

  public void drawImage(final ImageAsset asset, final float sx, final float sy, final float sWidth, final float sHeight, final float dx, final float dy, final float dWidth, final float dHeight) {
    printLog("drawImage: asset");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = CanvasRenderingContext2D.nativeDrawImageSwRaw(canvasView.canvas, asset.getBytes(), asset.getWidth(), asset.getHeight(), sx, sy, sWidth, sHeight, dx, dy, dWidth, dHeight);
        updateCanvas();

      }
    });

  }

  public void setShadowBlur(final float blur) {
    printLog("setShadowBlur");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetShadowBlur(canvasView.canvas, blur);

      }
    });

    shadowBlur = blur;
  }

  public float getShadowBlur() {
    return shadowBlur;
  }

  public void setShadowColor(final int color) {
    printLog("setShadowColor");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetShadowColor(canvasView.canvas, color);

      }
    });

    shadowColor = color;
  }

  public int getShadowColor() {
    return shadowColor;
  }

  public void setShadowOffsetX(final float x) {
    printLog("setShadowOffsetX");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetShadowOffsetX(canvasView.canvas, x);

      }
    });

    shadowOffsetX = x;
  }

  public float getShadowOffsetX() {
    return shadowOffsetX;
  }

  public void setShadowOffsetY(final float y) {
    printLog("setShadowOffsetY");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetShadowOffsetY(canvasView.canvas, y);

      }
    });

    this.shadowOffsetY = y;
  }

  public float getShadowOffsetY() {
    return shadowOffsetY;
  }

  public void setFont(final String font) {
    printLog("setFont");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetFont(canvasView.canvas, font);

      }
    });

  }

  public String getFont() {
    return font;
  }

  public CanvasTextMetrics measureText(String text) {
    printLog("measureText");
    return nativeMeasureText(canvasView.canvas, text);
  }

  public ImageData createImageData(int width, int height) {
    printLog("createImageData");
    return new ImageData(width, height, nativeCreateImageData(width, height));
  }

  public ImageData createImageData(ImageData imageData) {
    printLog("createImageData");
    return new ImageData(imageData.getWidth(), imageData.getHeight(), nativeCreateImageData(imageData.getWidth(), imageData.getHeight()));
  }

  public void putImageData(ImageData data) {
    putImageData(data, 0, 0);
  }

  public void putImageData(ImageData data, float x, float y) {
    putImageData(data, x, y, 0, 0, -1, -1);
  }

  public void putImageData(final ImageData data, final float x, final float y, final float dirtyX, final float dirtyY, final int dirtyWidth, final int dirtyHeight) {
    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativePutImageData(canvasView.canvas, data.getWidth(), data.getHeight(), data.getData(), x, y, dirtyX, dirtyY, dirtyWidth, dirtyHeight);
        updateCanvas();

      }
    });

  }

  public ImageData getImageData(final float sx, final float sy, final int sw, final int sh) {
    printLog("getImageData");
    final CountDownLatch lock = new CountDownLatch(1);
    final ByteBuffer[] data = new ByteBuffer[1];
    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        data[0] = nativeGetImageData(canvasView.canvas, sx, sy, sw, sh);
        lock.countDown();
      }
    });

    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }

    return new ImageData(sw, sh, data[0]);
  }

  public boolean getImageSmoothingEnabled() {
    return imageSmoothingEnabled;
  }


  public void setImageSmoothingEnabled(final boolean enabled) {
    printLog("setImageSmoothingEnabled");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetImageSmoothingEnabled(canvasView.canvas, enabled);

      }
    });

    imageSmoothingEnabled = enabled;
  }

  public ImageSmoothingQuality getImageSmoothingQuality() {
    return imageSmoothingQuality;
  }

  public void setImageSmoothingQuality(final ImageSmoothingQuality quality) {
    printLog("setImageSmoothingQuality");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetImageSmoothingQuality(canvasView.canvas, quality.quality);

      }
    });

  }

  public void resetTransform() {
    printLog("resetTransform");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeResetTransform(canvasView.canvas);

      }
    });

  }

  public void setCurrentTransform(final CanvasDOMMatrix matrix) {
    printLog("setCurrentTransform");

    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        canvasView.canvas = nativeSetCurrentTransform(canvasView.canvas, matrix.matrix);

      }
    });

  }

  public CanvasDOMMatrix getCurrentTransform() {
    printLog("getCurrentTransform");
    final CountDownLatch lock = new CountDownLatch(1);
    final long[] id = new long[1];
    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        id[0] = nativeGetCurrentTransform(canvasView.canvas);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
    if (id[0] == 0) {
      return new CanvasDOMMatrix();
    }
    return new CanvasDOMMatrix(id[0]);
  }

  public boolean isPointInPath(final float x, final float y) {
    printLog("isPointInPath");
    final CountDownLatch lock = new CountDownLatch(1);
    final boolean[] value = new boolean[1];
    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        value[0] = nativeIsPointInPath(canvasView.canvas, x, y);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignore) {
    }
    return value[0];
  }

  public boolean isPointInPath(final float x, final float y, final String fillRule) {
    printLog("isPointInPath");
    final CountDownLatch lock = new CountDownLatch(1);
    final boolean[] value = new boolean[1];
    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        value[0] = nativeIsPointInPathWithRule(canvasView.canvas, x, y, fillRule);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignore) {
    }
    return value[0];
  }

  public boolean isPointInPath(final CanvasPath2D path, final float x, final float y, final String fillRule) {
    printLog("isPointInPath");
    final CountDownLatch lock = new CountDownLatch(1);
    final boolean[] value = new boolean[1];
    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        value[0] = nativeIsPointInPathWithPathRule(canvasView.canvas, path.path, x, y, fillRule);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignore) {
    }
    return value[0];
  }

  public boolean isPointInStroke(final float x, final float y) {
    printLog("isPointInStroke");
    final CountDownLatch lock = new CountDownLatch(1);
    final boolean[] value = new boolean[1];
    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        value[0] = nativeIsPointInStroke(canvasView.canvas, x, y);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignore) {
    }
    return value[0];
  }

  public boolean isPointInStroke(final CanvasPath2D path, final float x, final float y) {
    printLog("isPointInStroke");
    final CountDownLatch lock = new CountDownLatch(1);
    final boolean[] value = new boolean[1];
    canvasView.queueEvent(new Runnable() {
      @Override
      public void run() {
        value[0] = nativeIsPointInStrokeWithPath(canvasView.canvas, path.path, x, y);
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignore) {
    }
    return value[0];
  }

}
