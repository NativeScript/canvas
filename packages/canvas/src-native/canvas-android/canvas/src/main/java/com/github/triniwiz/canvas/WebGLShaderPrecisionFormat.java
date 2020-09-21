package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 4/22/20
 */
public class WebGLShaderPrecisionFormat {
    int rangeMin = 0;
    int rangeMax = 0;
    int precision = 0;
    public WebGLShaderPrecisionFormat() {}

    public WebGLShaderPrecisionFormat(int rangeMin, int rangeMax, int precision) {
        this.rangeMin = rangeMin;
        this.rangeMax = rangeMax;
        this.precision = precision;
    }

    public int getPrecision() {
        return precision;
    }

    public int getRangeMax() {
        return rangeMax;
    }

    public int getRangeMin() {
        return rangeMin;
    }
}
