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
public class EXT_disjoint_timer_query {
    public static int QUERY_COUNTER_BITS_EXT = Constants.GL_QUERY_COUNTER_BITS_EXT;
    public static int CURRENT_QUERY_EXT = Constants.GL_CURRENT_QUERY_EXT;
    public static int QUERY_RESULT_EXT = Constants.GL_QUERY_RESULT_EXT;
    public static int QUERY_RESULT_AVAILABLE_EXT = Constants.GL_QUERY_RESULT_AVAILABLE_EXT;
    public static int TIME_ELAPSED_EXT = Constants.GL_TIME_ELAPSED_EXT;
    public static int TIMESTAMP_EXT = Constants.GL_TIMESTAMP_EXT;
    public static int GPU_DISJOINT_EXT = Constants.GL_GPU_DISJOINT_EXT;
    CanvasView canvas;

    public EXT_disjoint_timer_query(CanvasView canvas) {
        this.canvas = canvas;
    }

    public int createQueryEXT() {
        final CountDownLatch lock = new CountDownLatch(1);
        final int[] query = new int[1];
        canvas.queueEvent(new Runnable() {
            @Override
            public void run() {
                GLES30.glGenQueries(1, query, 0);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return query[0];
    }

    public void deleteQueryEXT(int query) {
        final CountDownLatch lock = new CountDownLatch(1);
        final int[] id = {query};
        canvas.queueEvent(new Runnable() {
            @Override
            public void run() {
                GLES30.glDeleteQueries(1, id, 0);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public boolean isQueryEXT(final int query) {
        final CountDownLatch lock = new CountDownLatch(1);
        final boolean[] value = new boolean[1];
        canvas.queueEvent(new Runnable() {
            @Override
            public void run() {
                value[0] = GLES30.glIsQuery(query);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public void beginQueryEXT(final int target, final int query) {
        final CountDownLatch lock = new CountDownLatch(1);
        canvas.queueEvent(new Runnable() {
            @Override
            public void run() {
                GLES30.glBeginQuery(target, query);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void endQueryEXT(final int target) {
        final CountDownLatch lock = new CountDownLatch(1);
        canvas.queueEvent(new Runnable() {
            @Override
            public void run() {
                GLES30.glEndQuery(target);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void queryCounterEXT(int query, int target) {
        // NOOP
    }

    public int getQueryEXT(final int target, final int pname) {
        final CountDownLatch lock = new CountDownLatch(1);
        final int[] query = new int[1];
        canvas.queueEvent(new Runnable() {
            @Override
            public void run() {
                GLES30.glGetQueryiv(target, pname, query, 0);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return query[0];
    }

    public Object getQueryObjectEXT(final int query, final int pname) {
        final CountDownLatch lock = new CountDownLatch(1);
        final int[] value = new int[1];
        canvas.queueEvent(new Runnable() {
            @Override
            public void run() {
                GLES30.glGetQueryObjectuiv(query, pname, value, 0);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        if(pname ==  QUERY_RESULT_AVAILABLE_EXT){
            return value[0] == GLES20.GL_TRUE;
        }
        return value[0];
    }
}
