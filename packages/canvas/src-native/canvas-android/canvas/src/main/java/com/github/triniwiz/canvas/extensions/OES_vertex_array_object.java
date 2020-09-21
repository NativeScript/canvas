package com.github.triniwiz.canvas.extensions;

import android.opengl.GLES20;
import android.opengl.GLES30;
import android.os.Build;

import androidx.annotation.RequiresApi;

import com.github.triniwiz.canvas.CanvasView;
import com.github.triniwiz.canvas.Constants;

import java.util.concurrent.CountDownLatch;

/**
 * Created by triniwiz on 5/1/20
 */
@RequiresApi(api = Build.VERSION_CODES.JELLY_BEAN_MR2)
public class OES_vertex_array_object {
    public int VERTEX_ARRAY_BINDING_OES = Constants.GL_VERTEX_ARRAY_BINDING_OES;
    CanvasView canvas;

    public OES_vertex_array_object(CanvasView canvas) {
        this.canvas = canvas;
    }

    public int createVertexArrayOES() {
        final int[] array = new int[1];
        final CountDownLatch lock = new CountDownLatch(1);
        canvas.queueEvent(new Runnable() {
            @Override
            public void run() {
                GLES30.glGenVertexArrays(1, array, 0);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return array[0];
    }

    public void deleteVertexArrayOES(int arrayObject) {
        final int[] array = {arrayObject};
        final CountDownLatch lock = new CountDownLatch(1);
        canvas.queueEvent(new Runnable() {
            @Override
            public void run() {
                GLES30.glDeleteVertexArrays(1, array, 0);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public boolean isVertexArrayOES(final int arrayObject) {
        final boolean[] value = new boolean[1];
        final CountDownLatch lock = new CountDownLatch(1);
        canvas.queueEvent(new Runnable() {
            @Override
            public void run() {
                value[0] = GLES30.glIsVertexArray(arrayObject);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public void bindVertexArrayOES(final int arrayObject) {
        final int[] array = {arrayObject};
        final CountDownLatch lock = new CountDownLatch(1);
        canvas.queueEvent(new Runnable() {
            @Override
            public void run() {
                GLES30.glBindVertexArray(arrayObject);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

}
