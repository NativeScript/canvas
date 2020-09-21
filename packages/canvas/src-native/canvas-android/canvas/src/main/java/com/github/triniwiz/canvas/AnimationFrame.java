package com.github.triniwiz.canvas;

import android.animation.TimeAnimator;
import android.os.Handler;
import android.os.HandlerThread;
import android.os.Looper;
import android.os.SystemClock;
import android.util.Log;
import android.view.Choreographer;

import java.nio.LongBuffer;
import java.sql.Array;
import java.time.Instant;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.Executor;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

/**
 * Created by triniwiz on 2019-08-13
 */
public class AnimationFrame implements Choreographer.FrameCallback {
    private static HashMap<Long, Callback> callbacks;
    static long lastCall = 0;
    static int count = 0;
    static Timer timer;
    static AnimationFrame instance;
    static ExecutorService executorService = Executors.newCachedThreadPool();
    static final Object lock = new Object();
    static Choreographer choreographer;
    static Handler handler;
    static long animationId = 0;

    static {
        callbacks = new HashMap<>();
        instance = new AnimationFrame();
        handler = new Handler(Looper.getMainLooper());
    }

    static int _minFps = 1000;
    static int framesRendered = 0;
    static long frameStartTime = 0;
    static boolean inAnimationFrame = false;

    void reset() {
        _minFps = 1000;
        frameStartTime = 0;
        framesRendered = 0;
    }

    void raf(final long fps){
        executorService.submit(new Runnable() {
            @Override
            public void run() {
                HashMap<Long, Callback> cbs = (HashMap<Long, Callback>) callbacks.clone();
                callbacks.clear();
                inAnimationFrame = true;
                Set<Map.Entry<Long, Callback>> set = cbs.entrySet();
                for (final Map.Entry<Long, Callback> cb : set) {
                    //if(cb.getKey() == animationId){
                    handler.post(new Runnable() {
                        @Override
                        public void run() {
                            cb.getValue().onFrame(fps);
                        }
                    });
                    // }
                }
                inAnimationFrame = false;
                reset();
            }
        });
    }

    void fps(long currentTimeMillis) {
        int fps;
        if (frameStartTime > 0) {
            // take the span in milliseconds
            long timeSpan = (currentTimeMillis - frameStartTime);
            framesRendered++;

            if (timeSpan > 1000) {
                fps = (int) (framesRendered * 1000 / timeSpan);
                if (fps < _minFps) {
                    _minFps = fps;
                }
                raf(fps);
                frameStartTime = currentTimeMillis;
                framesRendered = 0;
            }
        } else {
            frameStartTime = currentTimeMillis;
        }

    }


    @Override
    public void doFrame(long frameTimeNanos) {
        fps(frameTimeNanos / 1000000);
        Choreographer.getInstance().postFrameCallback(instance);
    }

    public interface Callback {
        void onFrame(long called);
    }

    private static long getTimeInFrameBase() {
        return System.nanoTime() / 1000000;
    }

    private static long getNewId() {
        animationId++;
        return animationId;
    }

    public synchronized static long requestAnimationFrame(Callback callback) {
        if (!inAnimationFrame) {
            inAnimationFrame = true;
            callback.onFrame(getTimeInFrameBase());
            inAnimationFrame = false;
            return getNewId();
        }
        long id = getNewId();
        callbacks.put(id, callback);
        Choreographer.getInstance().postFrameCallback(instance);
        return id;
    }


    public static void cancelAnimationFrame(long id) {
        callbacks.remove(id);
    }
}
