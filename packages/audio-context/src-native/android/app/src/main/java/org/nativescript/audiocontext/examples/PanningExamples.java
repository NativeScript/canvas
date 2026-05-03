package org.nativescript.audiocontext.examples;

import android.os.Handler;
import android.os.Looper;
import android.util.Log;

import org.nativescript.audiocontext.AudioContext;
import org.nativescript.audiocontext.AudioContextInstance;
import org.nativescript.audiocontext.AudioOscillatorNode;
import org.nativescript.audiocontext.AudioPannerNode;
import org.nativescript.audiocontext.AudioParam;


public final class PanningExamples {
    private static final String TAG = "PanningExamples";

    private PanningExamples() {}

 
    public static void panSweepOscillator(long durationMs) {
        try {
            final AudioContextInstance ctx = new AudioContextInstance();
            final AudioOscillatorNode osc = new AudioOscillatorNode(ctx, "sine", 440.0);
            final AudioPannerNode panner = AudioContext.getInstance().createPanner(ctx);

            osc.connect(panner);
            panner.connect(ctx.getDestination());

            double now = ctx.getCurrentTime();
            AudioParam pan = panner.getPan();
            pan.setValue(-1.0); // start fully left
            pan.linearRampToValueAtTime(1.0, now + (durationMs / 1000.0));

            osc.start();
            Log.i(TAG, "Started pan sweep for " + durationMs + "ms");

            Handler h = new Handler(Looper.getMainLooper());
            h.postDelayed(() -> {
                try {
                    osc.stop();
                    panner.disconnect();
                    osc.release();
                    panner.release();
                    ctx.release();
                    Log.i(TAG, "Pan sweep finished and cleaned up");
                } catch (Throwable t) {
                    Log.w(TAG, "Error cleaning up after pan sweep", t);
                }
            }, durationMs + 500);
        } catch (Throwable t) {
            Log.w(TAG, "panSweepOscillator failed", t);
        }
    }


    public static void positionSweepOscillator(long durationMs) {
        try {
            final AudioContextInstance ctx = new AudioContextInstance();
            final AudioOscillatorNode osc = new AudioOscillatorNode(ctx, "sine", 440.0);
            final AudioPannerNode panner = AudioContext.getInstance().createPanner(ctx);

            osc.connect(panner);
            panner.connect(ctx.getDestination());

            double now = ctx.getCurrentTime();
            // animate position X from -2 (left) to +2 (right)
            panner.getPositionXParam().setValue(-2.0);
            panner.getPositionXParam().linearRampToValueAtTime(2.0, now + (durationMs / 1000.0));

            osc.start();
            Log.i(TAG, "Started position sweep for " + durationMs + "ms");

            Handler h = new Handler(Looper.getMainLooper());
            h.postDelayed(() -> {
                try {
                    osc.stop();
                    panner.disconnect();
                    osc.release();
                    panner.release();
                    ctx.release();
                    Log.i(TAG, "Position sweep finished and cleaned up");
                } catch (Throwable t) {
                    Log.w(TAG, "Error cleaning up after position sweep", t);
                }
            }, durationMs + 500);
        } catch (Throwable t) {
            Log.w(TAG, "positionSweepOscillator failed", t);
        }
    }
}
