package com.github.triniwiz.canvas;

import android.content.Context;
import android.graphics.Bitmap;
import android.graphics.Canvas;
import android.util.AttributeSet;
import android.util.Log;
import android.view.View;
import androidx.annotation.Nullable;

/**
 * Created by triniwiz on 2019-08-05
 */
public class SVGView extends View {
    private static native long nativeInit(long svgCanvas, Bitmap bitmap);

    private static native long drawSVG(long svgCanvas, Bitmap bitmap, String svg);

    public Bitmap bitmap;
    public long svgCanvas = 0;
    boolean isInit;

    public SVGView(Context context) {
        super(context);
    }

    public SVGView(Context context, @Nullable AttributeSet attrs) {
        super(context, attrs);
    }

    @Override
    protected void onSizeChanged(int w, int h, int oldw, int oldh) {
        if (!isInit) {
            bitmap = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888);
            svgCanvas = SVGView.nativeInit(svgCanvas, bitmap);
            isInit = true;
        } else {
            // scale (w / oldw) (h / oldh)
        }
    }

    @Override
    protected void onDraw(Canvas canvas) {
        canvas.drawBitmap(bitmap, 0, 0, null);
    }

    public void setSrc(String src) {
        svgCanvas = SVGView.drawSVG(svgCanvas, bitmap, src);
        invalidate();
    }
}
