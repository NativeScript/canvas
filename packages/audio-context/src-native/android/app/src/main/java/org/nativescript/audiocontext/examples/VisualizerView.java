package org.nativescript.audiocontext.examples;

import android.content.Context;
import android.graphics.Canvas;
import android.graphics.Color;
import android.graphics.Paint;
import android.util.AttributeSet;
import android.view.View;

public class VisualizerView extends View {
    private float[] data;
    private final Paint paint = new Paint(Paint.ANTI_ALIAS_FLAG);

    public VisualizerView(Context c) { super(c); init(); }
    public VisualizerView(Context c, AttributeSet a) { super(c, a); init(); }
    public VisualizerView(Context c, AttributeSet a, int s) { super(c, a, s); init(); }

    private void init() {
        paint.setColor(Color.rgb(50, 200, 120));
    }

    public void setData(float[] d) {
        this.data = d;
        invalidate();
    }

    @Override
    protected void onDraw(Canvas canvas) {
        super.onDraw(canvas);
        int w = getWidth();
        int h = getHeight();
        canvas.drawColor(Color.rgb(8, 12, 26));
        if (data == null || data.length == 0) return;

        int barCount = Math.min(64, data.length);
        int binsPerBar = Math.max(1, data.length / barCount);
        float barWidth = (float) w / (float) barCount;

        final float minDb = -100f;
        final float maxDb = -30f;
        final float range = Math.max(1.0f, maxDb - minDb);

        for (int i = 0; i < barCount; ++i) {
            float sum = 0f;
            int base = i * binsPerBar;
            int end = Math.min(data.length, base + binsPerBar);
            for (int j = base; j < end; ++j) sum += data[j];
            float avg = sum / (end - base);
            float norm = (avg - minDb) / range;
            if (norm < 0f) norm = 0f;
            if (norm > 1f) norm = 1f;
            float barH = norm * h;
            float left = i * barWidth + 2f;
            float right = left + barWidth - 4f;
            canvas.drawRect(left, h - barH, right, h, paint);
        }
    }
}
