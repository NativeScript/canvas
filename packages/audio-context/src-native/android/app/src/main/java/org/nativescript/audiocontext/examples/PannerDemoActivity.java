package org.nativescript.audiocontext.examples;

import android.os.Bundle;
import android.util.Log;
import android.widget.Button;
import android.widget.SeekBar;
import android.widget.TextView;

import androidx.appcompat.app.AppCompatActivity;

import org.nativescript.audiocontext.AudioContext;
import org.nativescript.audiocontext.AudioContextInstance;
import org.nativescript.audiocontext.AudioOscillatorNode;
import org.nativescript.audiocontext.AudioPannerNode;

public class PannerDemoActivity extends AppCompatActivity {
    private static final String TAG = "PannerDemoActivity";

    private AudioContextInstance ctx;
    private AudioOscillatorNode osc;
    private AudioPannerNode panner;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        int layoutId = getResources().getIdentifier("activity_panner_demo", "layout", getPackageName());
        if (layoutId != 0) setContentView(layoutId);

        Button btnStart = findViewById(getResources().getIdentifier("btn_start", "id", getPackageName()));
        Button btnStop = findViewById(getResources().getIdentifier("btn_stop", "id", getPackageName()));
        Button btnSweepPan = findViewById(getResources().getIdentifier("btn_sweep_pan", "id", getPackageName()));
        Button btnSweepPos = findViewById(getResources().getIdentifier("btn_sweep_pos", "id", getPackageName()));
        SeekBar seek = findViewById(getResources().getIdentifier("seek_pan", "id", getPackageName()));
        TextView tvValue = findViewById(getResources().getIdentifier("tv_pan_value", "id", getPackageName()));

        btnStart.setOnClickListener(v -> startOsc());
        btnStop.setOnClickListener(v -> stopOsc());
        btnSweepPan.setOnClickListener(v -> PanningExamples.panSweepOscillator(4000));
        btnSweepPos.setOnClickListener(v -> PanningExamples.positionSweepOscillator(4000));

        seek.setMax(200);
        seek.setProgress(100);
        seek.setOnSeekBarChangeListener(new SeekBar.OnSeekBarChangeListener() {
            @Override
            public void onProgressChanged(SeekBar seekBar, int progress, boolean fromUser) {
                double pan = (progress - 100) / 100.0;
                if (panner != null) panner.setPan(pan);
                tvValue.setText(String.format("Pan: %.2f", pan));
            }

            @Override
            public void onStartTrackingTouch(SeekBar seekBar) {}

            @Override
            public void onStopTrackingTouch(SeekBar seekBar) {}
        });
    }

    private void startOsc() {
        try {
            if (ctx != null) return;
            ctx = new AudioContextInstance();
            osc = new AudioOscillatorNode(ctx, "sine", 440.0);
            panner = AudioContext.getInstance().createPanner(ctx);
            osc.connect(panner);
            panner.connect(ctx.getDestination());
            osc.start();
            Log.i(TAG, "Started oscillator");
        } catch (Throwable t) {
            Log.w(TAG, "startOsc failed", t);
        }
    }

    private void stopOsc() {
        try {
            if (osc != null) {
                try { osc.stop(); } catch (Throwable ignored) {}
            }
            if (panner != null) {
                try { panner.disconnect(); } catch (Throwable ignored) {}
            }
            if (osc != null) {
                try { osc.release(); } catch (Throwable ignored) {}
                osc = null;
            }
            if (panner != null) {
                try { panner.release(); } catch (Throwable ignored) {}
                panner = null;
            }
            if (ctx != null) {
                try { ctx.release(); } catch (Throwable ignored) {}
                ctx = null;
            }
            Log.i(TAG, "Stopped and cleaned up");
        } catch (Throwable t) {
            Log.w(TAG, "stopOsc failed", t);
        }
    }

    @Override
    protected void onDestroy() {
        stopOsc();
        super.onDestroy();
    }
}
