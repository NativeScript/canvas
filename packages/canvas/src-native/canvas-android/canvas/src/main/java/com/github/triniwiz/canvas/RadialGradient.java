package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 5/30/20
 */
public class RadialGradient extends Gradient {
    float x0;
    float y0;
    float r0;
    float x1;
    float y1;
    float r1;

    public RadialGradient(float x0, float y0, float r0, float x1, float y1, float r1) {
        this.x0 = x0;
        this.y0 = y0;
        this.r0 = r0;
        this.r1 = r1;
        this.x1 = x1;
        this.y1 = y1;
    }
}
