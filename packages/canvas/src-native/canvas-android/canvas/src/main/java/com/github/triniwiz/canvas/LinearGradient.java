package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 5/30/20
 */
public class LinearGradient extends Gradient {
    float x0;
    float y0;
    float x1;
    float y1;

    public LinearGradient(float x0, float y0, float x1, float y1) {
        this.x0 = x0;
        this.y0 = y0;
        this.x1 = x1;
        this.y1 = y1;
    }
}
