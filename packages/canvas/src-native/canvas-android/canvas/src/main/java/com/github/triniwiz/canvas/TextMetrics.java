package com.github.triniwiz.canvas;

import android.graphics.Paint;

/**
 * Created by triniwiz on 2019-07-10
 */
public class TextMetrics {
    Paint paint;
    String text;

    TextMetrics(Paint paint, String text) {
        this.paint = paint;
        this.text = text;
    }

    float getWidth() {
        return paint.measureText(text);
    }
}
