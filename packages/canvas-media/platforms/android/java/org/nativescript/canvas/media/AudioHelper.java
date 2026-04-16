package org.nativescript.canvas.media;

import android.content.Context;
import android.net.Uri;
import android.view.View;
import android.widget.LinearLayout;
import android.media.MediaCodecInfo;
import android.media.MediaCodecList;
import android.os.Handler;
import android.os.Looper;
import java.io.File;
import java.util.Collections;
import java.util.Timer;
import java.util.TimerTask;
import java.util.concurrent.atomic.AtomicBoolean;

import androidx.media3.common.MediaItem;
import androidx.media3.common.PlaybackException;
import androidx.media3.common.AudioAttributes;
import androidx.media3.common.Player;
import androidx.media3.common.util.UnstableApi;
import androidx.media3.common.util.Util;
import androidx.media3.datasource.DefaultDataSource;
import androidx.media3.datasource.DefaultHttpDataSource;
import androidx.media3.datasource.cache.CacheDataSource;
import androidx.media3.datasource.cache.LeastRecentlyUsedCacheEvictor;
import androidx.media3.datasource.cache.SimpleCache;
import androidx.media3.database.StandaloneDatabaseProvider;
import androidx.media3.exoplayer.DefaultLoadControl;
import androidx.media3.exoplayer.ExoPlayer;
import androidx.media3.exoplayer.source.DefaultMediaSourceFactory;

@UnstableApi
public class AudioHelper implements Player.Listener {
    private ExoPlayer _player;
    private DefaultDataSource.Factory _dsf;
    private CacheDataSource.Factory _cdsf;
    private DefaultMediaSourceFactory _msf;
    private boolean _autoplay;
    private boolean _loop;
    private String _src;
    private Timer _timer;
    private AtomicBoolean _loadedDataFired = new AtomicBoolean(false);
    private boolean _canPlayFired = false;
    private boolean _canPlayThroughFired = false;
    private Callback _callback;
    private LeastRecentlyUsedCacheEvictor _leastRecentlyUsedCacheEvictor;
    private StandaloneDatabaseProvider _exoDatabaseProvider;
    private static final long _exoPlayerCacheSize = 100 * 1024 * 1024;
    private boolean _playing = false;
    private AudioView _audioView;
    private LinearLayout _container;

    public interface Callback {
        void onDurationChange(long duration);
        void onPlaying();
        void onCurrentTimeChanged(long time);
        void onLoadedData();
        void onCanPlay();
        void onCanPlayThrough();
        void onError(String message);
    }

    public void setCallback(Callback callback) {
        this._callback = callback;
    }

    Handler handler;

    public AudioHelper(Context context, String cacheRoot) {
        File cacheDir = new File(cacheRoot, "MEDIA_PLAYER_CACHE");
        if (!cacheDir.exists()) {
            cacheDir.mkdirs();
        }

        Looper looper1 = Looper.myLooper();
        if (looper1 == null) {
            Looper.prepare();
            looper1 = Looper.myLooper();
        }

        handler = new Handler(looper1);

        _leastRecentlyUsedCacheEvictor = new LeastRecentlyUsedCacheEvictor(_exoPlayerCacheSize);
        String packageName = context.getPackageName();
        _exoDatabaseProvider = new StandaloneDatabaseProvider(context);
        SimpleCache cache = SimpleCacheProvider.getInstance(cacheDir, _leastRecentlyUsedCacheEvictor, _exoDatabaseProvider);
        DefaultHttpDataSource.Factory factory = new DefaultHttpDataSource.Factory();
        factory.setUserAgent(Util.getUserAgent(context, packageName));
        _dsf = new DefaultDataSource.Factory(context, factory);
        _cdsf = new CacheDataSource.Factory();
        if (cache != null) {
            _cdsf.setCache(cache);
        }
        _cdsf.setUpstreamDataSourceFactory(_dsf);
        _msf = new DefaultMediaSourceFactory(_cdsf);

        ExoPlayer.Builder builder = new ExoPlayer.Builder(context);
        builder.setMediaSourceFactory(_msf);

        DefaultLoadControl.Builder loadControl = new DefaultLoadControl.Builder();
        loadControl.setBufferDurationsMs(500, DefaultLoadControl.DEFAULT_MAX_BUFFER_MS, 500, 500);
        builder.setLoadControl(loadControl.build());
        _player = builder.build();
        _player.addListener(this);
       _player.setAudioAttributes(AudioAttributes.DEFAULT, true);

         _container = new LinearLayout(context);
            _container.setOrientation(LinearLayout.VERTICAL);
            _audioView = new AudioView(context, this);
            LinearLayout.LayoutParams params = new LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT);
            _container.addView(_audioView, params);
    }

    private void _setSrc(String value) {
        try {
            this._player.setMediaItems(Collections.singletonList(MediaItem.fromUri(Uri.parse(value))), true);
            this._player.prepare();
            if (this._autoplay) {
                this._player.setPlayWhenReady(true);
            }
        } catch (Exception e) {}
    }

    @Override
    public void onIsPlayingChanged(boolean isPlaying) {
        long duration = this._player.getDuration();
        if (this._callback != null) {
            this._callback.onDurationChange(duration);
        }
        this._playing = isPlaying;
        if (isPlaying) {
            if (this._callback != null) {
                this._callback.onPlaying();
                this._callback.onLoadedData();
            }
            if (this._audioView != null) {
                this._audioView.setPlaying(true);
                this._audioView.setDuration(duration);
            }
            if (_timer != null) {
                _timer.cancel();
                _timer = null;
            }
            _timer = new Timer();
            AudioHelper that = this;
            _timer.schedule(new TimerTask() {
                @Override
                public void run() {
                    handler.post(() -> {
                        long current = _player.getCurrentPosition();
                        if (that._callback != null) {
                            that._callback.onCurrentTimeChanged(current);
                        }
                        if (that._audioView != null) {
                            that._audioView.setCurrentTime(current);
                        }
                    });
                }
            }, 0, 1000);
        } else {
            if (_timer != null) {
                _timer.cancel();
                _timer = null;
            }
            if (this._callback != null) {
                this._callback.onCurrentTimeChanged(this._player.getCurrentPosition());
            }
            if (this._audioView != null) {
                this._audioView.setPlaying(false);
                this._audioView.setCurrentTime(this._player.getCurrentPosition());
            }
        }
    }

    @Override
    public void onPlayerError(PlaybackException error) {
        if (this._callback != null) {
            this._callback.onError(error == null ? "Playback error" : error.getMessage());
        }
    }

    @Override
    public void onEvents(Player player, Player.Events events) { }

    @Override
    public void onPlaybackStateChanged(@Player.State int playbackState) {
        if (playbackState == Player.STATE_READY) {
            if (!this._canPlayFired) {
                this._canPlayFired = true;
                if (this._callback != null) {
                    this._callback.onCanPlay();
                }
            }

            if (this._loadedDataFired.compareAndSet(false, true)) {
                if (this._callback != null) {
                    this._callback.onLoadedData();
                }
            }

            long duration = this._player.getDuration();
            long buffered = this._player.getBufferedPosition();
            if (this._audioView != null) {
                this._audioView.setDuration(duration);
                this._audioView.setCurrentTime(this._player.getCurrentPosition());
            }
            if (duration > 0 && buffered >= duration - 1000 && !this._canPlayThroughFired) {
                this._canPlayThroughFired = true;
                if (this._callback != null) {
                    this._callback.onCanPlayThrough();
                }
            }
        }
    }

    public void play() {
        try {
            this._player.setPlayWhenReady(true);
            this._player.play();
        } catch (Exception e) { }
    }

    public void pause() {
        try {
            this._player.setPlayWhenReady(false);
            this._player.pause();
        } catch (Exception e) { }
    }

    public boolean getMuted() {
        return this._player.getVolume() == 0f;
    }

    public void setMuted(boolean value) {
        this._player.setVolume(value ? 0f : 1f);
    }

    public long getDuration() {
        return this._player.getDuration();
    }

    public double getCurrentTime() {
        return this._player.getCurrentPosition() / 1000.0;
    }

    public void setCurrentTime(double value) {
        this._player.seekTo((long) (value * 1000.0));
    }

    public String getSrc() {
        return this._src;
    }

    public LinearLayout getContainer() {
        return this._container;
    }

    public boolean getControls() {
        return this._container != null && this._container.getVisibility() == View.VISIBLE;
    }

    public void setControls(boolean value) {
        if (this._container != null) {
            this._container.setVisibility(value ? View.VISIBLE : View.GONE);
        }
    }

    public boolean isPlaying() {
        return this._playing;
    }

    public void setSrc(String value) {
        this._src = value;
        this._loadedDataFired.set(false);
        this._canPlayFired = false;
        this._canPlayThroughFired = false;
        this._setSrc(value);
    }

    public void load() {
        this.pause();
        if (_timer != null) { _timer.cancel(); _timer = null; }
        this._loadedDataFired.set(false);
        if (this._src != null && !this._src.isEmpty()) {
            this._setSrc(this._src);
        }
    }

    public boolean getAutoplay() {
        return this._autoplay;
    }

    public void setAutoplay(boolean value) {
        this._autoplay = value;
        this._player.setPlayWhenReady(value);
    }

    public boolean getLoop() {
        return this._loop;
    }

    public void setLoop(boolean value) {
        this._loop = value;
        if (value) {
            this._player.setRepeatMode(Player.REPEAT_MODE_ALL);
        } else {
            this._player.setRepeatMode(Player.REPEAT_MODE_OFF);
        }
    }

    public String canPlayType(String type) {
        if (type == null || type.trim().isEmpty()) return "";
        String mime = type;
        int sc = mime.indexOf(';');
        if (sc >= 0) mime = mime.substring(0, sc).trim();
        String t = mime.toLowerCase();
        try {
            if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.LOLLIPOP) {
                MediaCodecList mcl = new MediaCodecList(MediaCodecList.ALL_CODECS);
                MediaCodecInfo[] infos = mcl.getCodecInfos();
                for (MediaCodecInfo info : infos) {
                    if (info.isEncoder()) continue;
                    String[] types = info.getSupportedTypes();
                    for (String s : types) {
                        if (s.equalsIgnoreCase(t)) {
                            return "probably";
                        }
                    }
                }
            }
        } catch (Throwable e) {}

        if (t.contains("mp3") || t.contains("mpeg") || t.contains("aac") || t.contains("wav") || t.contains("m4a") || t.contains("flac")) return "probably";
        if (t.contains("ogg") || t.contains("opus") || t.contains("webm")) return "maybe";
        return "";
    }
}
