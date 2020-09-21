package com.github.triniwiz.canvas;

import android.content.Context;
import android.graphics.Bitmap;
import android.graphics.Canvas;
import android.os.Handler;
import android.os.Looper;
import android.util.AttributeSet;
import android.view.View;

import androidx.annotation.Nullable;

import java.lang.ref.WeakReference;

public class CPUView extends View {
    Bitmap view;
    WeakReference<CanvasView> canvasView;
    Handler handler;

    public CPUView(Context context) {
        super(context);
        init();
    }

    public CPUView(Context context, @Nullable AttributeSet attrs) {
        super(context, attrs);
        init();
    }

    private void init() {
        handler = new Handler(Looper.getMainLooper());
    }

    @Override
    protected void onSizeChanged(int w, int h, int oldw, int oldh) {
        super.onSizeChanged(w, h, oldw, oldh);
        if (w != 0 && h != 0) {
            view = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888);
            CanvasView canvas = canvasView.get();
            if (canvas != null && canvas.canvas == 0) {
                canvas.canvas = CanvasView.nativeInit(true, 0, w, h, 0, CanvasView.getDirection());
                if (canvas.listener != null) {
                    canvas.listener.contextReady();
                }
            }
        }
    }

    @Override
    protected void onDraw(Canvas canvas) {
        if (view != null) {
            canvas.drawBitmap(view, 0, 0, null);
        }
    }

    public void flush() {
        final CanvasView canvas = canvasView.get();
        if (view != null && canvas != null && canvas.canvas != 0) {
            canvas.canvas = CanvasView.nativeCpuFlush(canvas.canvas, view);
            handler.post(new Runnable() {
                @Override
                public void run() {
                    invalidate();
                    canvas.pendingInvalidate = false;
                }
            });
        }
    }
}
