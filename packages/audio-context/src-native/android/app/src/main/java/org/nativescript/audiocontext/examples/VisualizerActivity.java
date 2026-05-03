package org.nativescript.audiocontext.examples;

import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.util.Log;
import android.view.View;
import android.widget.Button;

import androidx.appcompat.app.AppCompatActivity;

import org.nativescript.audiocontext.AudioContext;
import org.nativescript.audiocontext.AudioContextInstance;
import org.nativescript.audiocontext.AudioOscillatorNode;
import org.nativescript.audiocontext.AnalyserNode;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;

public class VisualizerActivity extends AppCompatActivity {
    private static final String TAG = "VisualizerActivity";

    private AudioContextInstance ctx;
    private AudioOscillatorNode osc;
    private AnalyserNode analyser;
    private VisualizerView visualizerView;

    private final Handler handler = new Handler(Looper.getMainLooper());
    private final Runnable tick = new Runnable() {
        @Override
        public void run() {
            updateAndSchedule();
        }
    };

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        int layoutId = getResources().getIdentifier("activity_visualizer", "layout", getPackageName());
        if (layoutId != 0) setContentView(layoutId);

        visualizerView = findViewById(getResources().getIdentifier("visualizer_view", "id", getPackageName()));
        View btnStart = findViewById(getResources().getIdentifier("btn_start_vis", "id", getPackageName()));
        View btnStop = findViewById(getResources().getIdentifier("btn_stop_vis", "id", getPackageName()));

        if (btnStart != null) btnStart.setOnClickListener(v -> startOscAndAnalyser());
        if (btnStop != null) btnStop.setOnClickListener(v -> stopOscAndAnalyser());
    }

    @Override
    protected void onResume() {
        super.onResume();
        startOscAndAnalyser();
    }

    @Override
    protected void onPause() {
        stopOscAndAnalyser();
        super.onPause();
    }

    private void startOscAndAnalyser() {
        try {
            if (ctx != null) return;
            ctx = new AudioContextInstance();
            osc = new AudioOscillatorNode(ctx, "sine", 220.0);
            analyser = AudioContext.getInstance().createAnalyser(ctx);
            osc.connect(analyser);
            analyser.connect(ctx.getDestination());
            osc.start();
            handler.post(tick);
        } catch (Throwable t) {
            Log.w(TAG, "start failed", t);
        }
    }

    private void stopOscAndAnalyser() {
        try {
            handler.removeCallbacks(tick);
            if (osc != null) {
                try { osc.stop(); } catch (Throwable ignored) {}
                try { osc.release(); } catch (Throwable ignored) {}
                osc = null;
            }
            if (analyser != null) {
                try { analyser.release(); } catch (Throwable ignored) {}
                analyser = null;
            }
            if (ctx != null) {
                try { ctx.release(); } catch (Throwable ignored) {}
                ctx = null;
            }
        } catch (Throwable t) {
            Log.w(TAG, "stop failed", t);
        }
    }

    private void updateAndSchedule() {
        try {
            if (analyser == null || visualizerView == null) {
                handler.postDelayed(tick, 60);
                return;
            }
            int bins = analyser.getFrequencyBinCount();
            FloatBuffer fb = ByteBuffer.allocateDirect(bins * 4).order(ByteOrder.nativeOrder()).asFloatBuffer();
            analyser.getFloatFrequencyData(fb);
            fb.position(0);
            float[] a = new float[bins];
            fb.get(a);
            visualizerView.setData(a);
        } catch (Throwable t) {
            Log.w(TAG, "update failed", t);
        } finally {
            handler.postDelayed(tick, 60);
        }
    }
}
