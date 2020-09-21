package com.github.triniwiz.canvas;

import android.graphics.Bitmap;
import android.util.Log;
import android.view.ViewGroup;

import java.lang.reflect.Array;
import java.nio.ByteBuffer;
import java.util.Arrays;
import java.util.concurrent.CountDownLatch;

/**
 * Created by triniwiz on 5/30/20
 */

public class Pattern implements ICanvasColorStyle {
    private static native long nativeCreatePattern(Bitmap image, String repetition);

    private static native long nativeCreatePatternRaw(byte[] image, int width, int height, String repetition);

    private static native long nativeCreatePatternRawBuffer(ByteBuffer image, int width, int height, String repetition);

    private static native long nativeCreatePatternCanvas(byte[] image, String repetition);

    private static native void nativeFree(long pattern);

    private static native long nativeSetPatternTransform(long pattern, long matrix);

    long nativePattern;

    public enum PatternRepetition {
        Repeat("repeat"),
        RepeatX("repeat-x"),
        RepeatY("repeat-y"),
        NoRepeat("no-repeat");
        private String pattern;

        PatternRepetition(String pattern) {
            this.pattern = pattern;
        }

        @Override
        public String toString() {
            return pattern;
        }
    }

    public Pattern(CanvasView parent, CanvasView src, final PatternRepetition repetition) {
        Log.d("com.test", "parent " + parent.canvas + " src " + src.canvas);
        final byte[] ss = src.snapshot();
        final CountDownLatch lock = new CountDownLatch(1);
        final long[] ptr = new long[1];
        parent.queueEvent(new Runnable() {
            @Override
            public void run() {
                ptr[0] = nativeCreatePatternCanvas(ss, repetition.pattern);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignore) {
        }

        this.nativePattern = ptr[0];
    }

    public Pattern(CanvasView parent, final Bitmap src, final PatternRepetition repetition) {
        final CountDownLatch lock = new CountDownLatch(1);
        final long[] ptr = new long[1];
        parent.queueEvent(new Runnable() {
            @Override
            public void run() {
                ptr[0] = nativeCreatePattern(src, repetition.pattern);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignore) {
        }
        this.nativePattern = ptr[0];
    }

    public Pattern(CanvasView parent, final ImageAsset src, final PatternRepetition repetition) {
        final CountDownLatch lock = new CountDownLatch(1);
        final long[] ptr = new long[1];
        parent.queueEvent(new Runnable() {
            @Override
            public void run() {
                ptr[0] = nativeCreatePatternRawBuffer(src.getBuffer(), src.getWidth(), src.getHeight(), repetition.pattern);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignore) {
        }
        this.nativePattern = ptr[0];
    }

    private void destroy() {
        if (this.nativePattern != 0) {
            nativeFree(this.nativePattern);
        }
    }

    public void setTransform(CanvasDOMMatrix matrix) {
        this.nativePattern = nativeSetPatternTransform(this.nativePattern, matrix.matrix);
    }

    @Override
    protected void finalize() throws Throwable {
        super.finalize();
        destroy();
    }

    @Override
    public CanvasColorStyleType getStyleType() {
        return CanvasColorStyleType.Pattern;
    }
}