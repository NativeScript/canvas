package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 3/27/20
 */
public class CanvasDOMMatrix {
    long matrix;

    private static native long nativeInit();

    private static native long nativeSetMatrix(long matrix, float[] values);

    private static native float[] nativeGetMatrix(long matrix);

    private static native void nativeFreeMatrix(long matrix);

    private float a;
    private float b;
    private float c;
    private float d;
    private float e;
    private float f;

    public CanvasDOMMatrix() {
        matrix = nativeInit();
        refreshMatrix();
    }

    CanvasDOMMatrix(long matrix) {
        this.matrix = matrix;
        refreshMatrix();
    }

    private void updateMatrix() {
        float[] values = {a, b, c, d, e, f};
        nativeSetMatrix(matrix, values);
    }


    private void refreshMatrix() {
        float[] values = nativeGetMatrix(matrix);
        a = values[0];
        b = values[1];
        c = values[2];
        d = values[3];
        e = values[4];
        f = values[5];
    }

    private void reload() {
        updateMatrix();
        refreshMatrix();
    }


    public float getA() {
        return a;
    }

    public void setA(float a) {
        this.a = a;
        reload();
    }

    public float getB() {
        return b;
    }

    public void setB(float b) {
        this.b = b;
        reload();
    }

    public float getC() {
        return c;
    }

    public void setC(float c) {
        this.c = c;
        reload();
    }

    public float getD() {
        return d;
    }

    public void setD(float d) {
        this.d = d;
        reload();
    }

    public float getE() {
        return e;
    }

    public void setE(float e) {
        this.e = e;
        reload();
    }

    public float getF() {
        return f;
    }

    public void setF(float f) {
        this.f = f;
        reload();
    }

    @Override
    protected void finalize() throws Throwable {
        super.finalize();
        nativeFreeMatrix(matrix);
        matrix = 0;
    }
}
