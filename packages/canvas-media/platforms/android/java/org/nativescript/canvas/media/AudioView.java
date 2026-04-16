package org.nativescript.canvas.media;

import android.content.Context;
import android.widget.LinearLayout;
import android.widget.ImageButton;
import android.widget.SeekBar;
import android.widget.TextView;
import android.view.View;
import android.view.Gravity;
import android.util.TypedValue;
import android.view.ViewGroup.LayoutParams;
import java.lang.ref.WeakReference;

public class AudioView extends LinearLayout {
    private ImageButton _playButton;
    private SeekBar _seekBar;
    private TextView _timeLabel;
    private long _durationMs = 0;
    private WeakReference<AudioHelper> _helperRef;
    private boolean _isSeeking = false;

    public AudioView(Context context, AudioHelper helper) {
        super(context);
        this._helperRef = new WeakReference<>(helper);

        setOrientation(HORIZONTAL);
        setGravity(Gravity.CENTER_VERTICAL);
        int pad = (int) TypedValue.applyDimension(TypedValue.COMPLEX_UNIT_DIP, 8, getResources().getDisplayMetrics());
        setPadding(pad, pad, pad, pad);


        int minWidthPx = (int) TypedValue.applyDimension(TypedValue.COMPLEX_UNIT_DIP, 300, getResources().getDisplayMetrics());
        int minHeightPx = (int) TypedValue.applyDimension(TypedValue.COMPLEX_UNIT_DIP, 40, getResources().getDisplayMetrics());
        setMinimumWidth(minWidthPx);
        setMinimumHeight(minHeightPx);

        _playButton = new ImageButton(context);
        _playButton.setImageResource(android.R.drawable.ic_media_play);
        _playButton.setBackgroundResource(0);
        int iconSize = (int) TypedValue.applyDimension(TypedValue.COMPLEX_UNIT_DIP, 40, getResources().getDisplayMetrics());
        LayoutParams btnParams = new LayoutParams(iconSize, iconSize);
        addView(_playButton, btnParams);

        _seekBar = new SeekBar(context);
        LayoutParams sbParams = new LayoutParams(0, LayoutParams.WRAP_CONTENT, 1f);
        sbParams.leftMargin = pad;
        sbParams.rightMargin = pad;
        addView(_seekBar, sbParams);
        _seekBar.setEnabled(false);

        _timeLabel = new TextView(context);
        _timeLabel.setText("0:00 / 0:00");
        addView(_timeLabel, new LayoutParams(LayoutParams.WRAP_CONTENT, LayoutParams.WRAP_CONTENT));

        _playButton.setOnClickListener(new OnClickListener() {
            @Override
            public void onClick(View v) {
                AudioHelper h = _helperRef.get();
                if (h == null) return;
                  boolean playing = h.isPlaying();
                    if (playing) {
                        h.pause();
                    } else {
                        h.play();
                    }
            }
        });

        _seekBar.setOnSeekBarChangeListener(new SeekBar.OnSeekBarChangeListener() {
            @Override
            public void onProgressChanged(SeekBar seekBar, int progress, boolean fromUser) {
                if (fromUser) {
                    updateTimeLabel(progress);
                    AudioHelper h = _helperRef.get();
                    if (h == null) return;
                      float seconds = progress / 1000.0f;
                        h.setCurrentTime(seconds);
                }
            }

            @Override
            public void onStartTrackingTouch(SeekBar seekBar) {
                _isSeeking = true;
            }

            @Override
            public void onStopTrackingTouch(SeekBar seekBar) {
                _isSeeking = false;
                AudioHelper h = _helperRef.get();
                if (h == null) return;
                float seconds = seekBar.getProgress() / 1000.0f;
                h.setCurrentTime(seconds);
            }
        });
    }

    public void setPlaying(boolean playing) {
        _playButton.setImageResource(playing ? android.R.drawable.ic_media_pause : android.R.drawable.ic_media_play);
    }

    public void setDuration(long durationMs) {
        this._durationMs = durationMs;
        if (durationMs > 0) {
            _seekBar.setMax((int)Math.max(durationMs, 1000));
        }
       updateTimeLabel((int)_seekBar.getProgress()); 
       _seekBar.setEnabled(durationMs > 0);
    }

    public void setCurrentTime(long ms) {
        if (_isSeeking) return;
        _seekBar.setProgress((int)ms);
        updateTimeLabel((int)ms);
    }

    private void updateTimeLabel(int ms) {
        int totalSec = ms / 1000;
        int mins = totalSec / 60;
        int secs = totalSec % 60;
        int durationSec = (int)(this._durationMs / 1000);
        int dmins = durationSec / 60;
        int dsecs = durationSec % 60;
        _timeLabel.setText(String.format("%d:%02d / %d:%02d", mins, secs, dmins, dsecs));
    }
}
