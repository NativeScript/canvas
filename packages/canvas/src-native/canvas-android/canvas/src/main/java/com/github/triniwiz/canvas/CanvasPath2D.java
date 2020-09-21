package com.github.triniwiz.canvas;

import android.util.Log;

import androidx.annotation.Nullable;

/**
 * Created by triniwiz on 2019-08-11
 */
public class CanvasPath2D {
    long path;

    private static native long nativeInit();

    private static native long nativeInitWithPath(long path);

    private static native long nativeInitWithData(String data);

    private static native long nativeAddPath(long path_ptr, long path, long matrix);

    private static native long nativeClosePath(long path_ptr);

    private static native long nativeRect(long path, float x, float y, float width, float height);

    private static native long nativeMoveTo(long path, float x, float y);

    private static native long nativeLineTo(long path_ptr, float x, float y);

    private static native long nativeArc(long path_ptr, float x, float y, float radius, float startAngle, float endAngle, boolean anticlockwise);

    private static native long nativeArcTo(long path_ptr, float x1, float y1, float x2, float y2, float radius);

    private static native long nativeBezierCurveTo(long path_ptr, float cp1x, float cp1y, float cp2x, float cp2y, float x, float y);

    private static native long nativeEllipse(long path_ptr, float x, float y, float radiusX, float radiusY, float rotation, float startAngle, float endAngle, boolean anticlockwise);

    private static native long nativeQuadraticCurveTo(long path_ptr, float cpx, float cpy, float x, float y);

    private static native void nativeFreePath(long path_ptr);


    public CanvasPath2D() {
        path = nativeInit();
    }

    public CanvasPath2D(CanvasPath2D path2D) {
        path = nativeInitWithPath(path2D.path);
    }

    public CanvasPath2D(String data) {
        path = nativeInitWithData(data);
    }

    public void addPath(CanvasPath2D path2D, @Nullable CanvasDOMMatrix matrix) {
        long m = 0;
        if (matrix != null) {
            m = matrix.matrix;
        }
        path = nativeAddPath(path, path2D.path, m);
    }

    public void closePath() {
        path = nativeClosePath(path);
    }

    public void moveTo(float x, float y) {
        path = nativeMoveTo(path, x, y);
    }

    public void rect(float x, float y, float width, float height) {
        path = nativeRect(path, x, y, width, height);
    }

    public void lineTo(float x, float y) {
        path = nativeLineTo(path, x, y);
    }

    public void arc(float x, float y, float radius, float startAngle, float endAngle, boolean anticlockwise) {
        path = nativeArc(path, x, y, radius, startAngle, endAngle, anticlockwise);
    }

    public void arcTo(float x1, float y1, float x2, float y2, float radius) {
        path = nativeArcTo(path, x1, y1, x2, y2, radius);
    }

    public void bezierCurveTo(float cp1x, float cp1y, float cp2x, float cp2y, float x, float y) {
        path = nativeBezierCurveTo(path, cp1x, cp1y, cp2x, cp2y, x, y);
    }

    public void ellipse(float x, float y, float radiusX, float radiusY, float rotation, float startAngle, float endAngle, boolean anticlockwise) {
        path = nativeEllipse(path, x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise);
    }

    public void quadraticCurveTo(float cpx, float cpy, float x, float y) {
        path = nativeQuadraticCurveTo(path, cpx, cpy, x, y);
    }

    @Override
    protected void finalize() throws Throwable {
        super.finalize();
        nativeFreePath(path);
        path = 0;
    }
}
