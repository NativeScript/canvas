package org.nativescript.canvas.media;

import android.content.Context;
import android.net.Uri;
import android.os.Build;
import android.os.Handler;
import android.os.Looper;
import android.view.LayoutInflater;
import android.view.Surface;
import android.view.View;
import android.view.ViewGroup;
import android.widget.FrameLayout;
import android.widget.LinearLayout;

import java.io.File;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Timer;
import java.util.TimerTask;

import com.google.android.exoplayer2.DeviceInfo;
import com.google.android.exoplayer2.MediaItem;
import com.google.android.exoplayer2.MediaMetadata;
import com.google.android.exoplayer2.PlaybackException;
import com.google.android.exoplayer2.PlaybackParameters;
import com.google.android.exoplayer2.Player;
import com.google.android.exoplayer2.Timeline;
import com.google.android.exoplayer2.TracksInfo;
import com.google.android.exoplayer2.audio.AudioAttributes;
import com.google.android.exoplayer2.metadata.Metadata;
import com.google.android.exoplayer2.source.TrackGroupArray;
import com.google.android.exoplayer2.text.Cue;
import com.google.android.exoplayer2.trackselection.TrackSelectionArray;
import com.google.android.exoplayer2.trackselection.TrackSelectionParameters;
import com.google.android.exoplayer2.ui.StyledPlayerView;
import com.google.android.exoplayer2.ExoPlayer;

import android.view.TextureView;

import com.google.android.exoplayer2.upstream.DefaultDataSource;
import com.google.android.exoplayer2.upstream.DefaultHttpDataSource;
import com.google.android.exoplayer2.upstream.cache.SimpleCache;
import com.google.android.exoplayer2.upstream.cache.LeastRecentlyUsedCacheEvictor;
import com.google.android.exoplayer2.database.StandaloneDatabaseProvider;
import com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
import com.google.android.exoplayer2.upstream.cache.CacheDataSource;

import android.graphics.SurfaceTexture;

import androidx.annotation.Nullable;
import androidx.annotation.RequiresApi;

import com.google.android.exoplayer2.DefaultLoadControl;
import com.google.android.exoplayer2.util.Util;
import com.google.android.exoplayer2.video.VideoSize;

import org.nativescript.canvas_media.R;

@RequiresApi(api = Build.VERSION_CODES.ICE_CREAM_SANDWICH_MR1)
public class VideoHelper implements Player.Listener, SurfaceTexture.OnFrameAvailableListener {
    public static class Source {
        String src;
        String type;
    }

    LinearLayout _container;

    ArrayList<Source> _sourceView;
    StyledPlayerView _playerView;
    ExoPlayer _player;
    String _src;
    boolean _autoplay;
    boolean _loop;
    TextureView _textureView;
    private DefaultDataSource.Factory _dsf;

    private static SimpleCache _cache;
    private LeastRecentlyUsedCacheEvictor _leastRecentlyUsedCacheEvictor;
    private StandaloneDatabaseProvider _exoDatabaseProvider;
    private static final long _exoPlayerCacheSize = 100 * 1024 * 1024;
    private DefaultMediaSourceFactory _msf;
    private CacheDataSource.Factory _cdsf;
    boolean _isCustom = false;
    boolean _playing = false;
    Timer _timer;
    SurfaceTexture _st;
    Surface _surface;
    Object _render;
    Object _frameListener;
    Object _canvas;
    Timer _frameTimer;
    int _readyState = 0;
    int _videoWidth = 0;
    int _videoHeight = 0;

    boolean _hasFrame = false;
    static int BUFFER_MS = 500;
    public static boolean IS_DEBUG = true;

    public interface Callback {
        void onDurationChange(long duration);

        void onPlaying();

        void onVideoSizeChanged(int width, int height);

        void onCurrentTimeChanged(long time);

        void onVideoFrame();
    }

    private Callback _callback;

    public void setCallback(Callback callback) {
        this._callback = callback;
    }

    @Override
    public void onRepeatModeChanged(int repeatMode) {
    }

    @Override
    public void onPlaybackParametersChanged(PlaybackParameters playbackParameters) {
    }

    @Override
    public void onPlaybackSuppressionReasonChanged(int playbackSuppressionReason) {
    }

    @Override
    public void onPlaybackStateChanged(int playbackState) {
    }

    @Override
    public void onPositionDiscontinuity(Player.PositionInfo oldPosition, Player.PositionInfo newPosition, int reason) {
    }

    @Override
    public void onShuffleModeEnabledChanged(boolean shuffleModeEnabled) {
    }

    @Override
    public void onTimelineChanged(Timeline timeline, int reason) {

    }

    @Override
    public void onTracksInfoChanged(TracksInfo tracksInfo) {

    }

    @Override
    public void onIsLoadingChanged(boolean isLoading) {
    }

    @Override
    public void onDeviceVolumeChanged(int volume, boolean muted) {
    }

    @Override
    public void onDeviceInfoChanged(DeviceInfo deviceInfo) {
    }

    @Override
    public void onRenderedFirstFrame() {

    }

    @Override
    public void onMediaItemTransition(@Nullable MediaItem mediaItem, int reason) {

    }

    @Override
    public void onAudioAttributesChanged(AudioAttributes audioAttributes) {

    }

    @Override
    public void onAudioSessionIdChanged(int audioSessionId) {

    }

    @Override
    public void onAvailableCommandsChanged(Player.Commands availableCommands) {

    }

    @Override
    public void onCues(List<Cue> cues) {

    }

    @Override
    public void onEvents(Player player, Player.Events events) {

    }

    @Override
    public void onMaxSeekToPreviousPositionChanged(long maxSeekToPreviousPositionMs) {

    }

    @Override
    public void onMediaMetadataChanged(MediaMetadata mediaMetadata) {

    }

    @Override
    public void onMetadata(Metadata metadata) {

    }

    @Override
    public void onPlayerErrorChanged(@Nullable PlaybackException error) {

    }

    @Override
    public void onPlaylistMetadataChanged(MediaMetadata mediaMetadata) {

    }

    @Override
    public void onPlayWhenReadyChanged(boolean playWhenReady, int reason) {

    }

    @Override
    public void onSeekBackIncrementChanged(long seekBackIncrementMs) {

    }

    @Override
    public void onSeekForwardIncrementChanged(long seekForwardIncrementMs) {

    }

    @Override
    public void onSkipSilenceEnabledChanged(boolean skipSilenceEnabled) {
    }

    @Override
    public void onSurfaceSizeChanged(int width, int height) {

    }

    @Override
    public void onTrackSelectionParametersChanged(TrackSelectionParameters parameters) {

    }

    @Override
    public void onVolumeChanged(float volume) {

    }

    @Override
    public void onPlayerStateChanged(boolean playWhenReady, int playbackState) {

    }

    @Override
    public void onLoadingChanged(boolean isLoading) {

    }

    @Override
    public void onPositionDiscontinuity(int reason) {

    }

    @Override
    public void onSeekProcessed() {

    }

    @Override
    public void onTracksChanged(TrackGroupArray trackGroups, TrackSelectionArray trackSelections) {

    }

    @Override
    public void onIsPlayingChanged(boolean isPlaying) {
        long duration = this._player.getDuration();
        if (this.getDuration() != duration) {
            if (this._callback != null) {
                this._callback.onDurationChange(duration);
            }
        }
        this._playing = isPlaying;
        if (isPlaying) {
            if (this._callback != null) {
                this._callback.onPlaying();
                this._callback.onVideoFrame();
            }

            if (_timer != null) {
                _timer.cancel();
                _timer.purge();
                _timer = null;
            }

            if (_frameTimer != null) {
                _frameTimer.cancel();
                _frameTimer.purge();
                _frameTimer = null;
            }

            _timer = new Timer();
            VideoHelper that = this;
            _timer.scheduleAtFixedRate(new TimerTask() {
                @Override
                public void run() {
                    handler.post(() -> {
                        long current = _player.getCurrentPosition();
                        if (current != that.getCurrentTime()) {
                            if (that._callback != null) {
                                that._callback.onCurrentTimeChanged(current);
                                that._callback.onVideoFrame();
                            }
                        }
                    });
                }
            }, 0, 1000);


        } else {
            if (_timer != null) {
                _timer.cancel();
                _timer.purge();
                _timer = null;
            }

            if (_frameTimer != null) {
                _frameTimer.cancel();
                _frameTimer.purge();
                _frameTimer = null;
            }


            if (this._callback != null) {
                this._callback.onCurrentTimeChanged(this._player.getCurrentPosition());
            }
        }
    }

    @Override
    public void onPlayerError(PlaybackException error) {
        if (VideoHelper.IS_DEBUG) {
            android.util.Log.d("JS", "" + error);
        }
    }

    @Override
    public void onVideoSizeChanged(VideoSize videoSize) {
        this._videoWidth = videoSize.width;
        this._videoHeight = videoSize.height;
        if (this._callback != null) {
            this._callback.onVideoSizeChanged(videoSize.width, videoSize.height);
        }

    }

    @Override
    public void onFrameAvailable(SurfaceTexture surfaceTexture) {
        this._hasFrame = true;
        if (this._callback != null) {
            this._callback.onVideoFrame();
        }
    }

    Handler handler;

    public VideoHelper(Context context, String cacheRoot) {
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

        _leastRecentlyUsedCacheEvictor = new LeastRecentlyUsedCacheEvictor(VideoHelper._exoPlayerCacheSize);
        String packageName = context.getPackageName();
        _exoDatabaseProvider = new StandaloneDatabaseProvider(context);
        if(_cache == null){
_cache = new com.google.android.exoplayer2.upstream.cache.SimpleCache(cacheDir, _leastRecentlyUsedCacheEvictor, _exoDatabaseProvider);
        }
        DefaultHttpDataSource.Factory factory = new com.google.android.exoplayer2.upstream.DefaultHttpDataSource.Factory();
        factory.setUserAgent(Util.getUserAgent(context, packageName));
        _dsf = new DefaultDataSource.Factory(context, factory);
        _cdsf = new CacheDataSource.Factory();
        _cdsf.setCache(_cache);
        _cdsf.setUpstreamDataSourceFactory(_dsf);
        _msf = new DefaultMediaSourceFactory(_cdsf);

        ExoPlayer.Builder builder = new ExoPlayer.Builder(context);
        builder.setMediaSourceFactory(_msf);

        DefaultLoadControl.Builder loadControl = new DefaultLoadControl.Builder();
        loadControl.setBufferDurationsMs(BUFFER_MS, com.google.android.exoplayer2.DefaultLoadControl.DEFAULT_MAX_BUFFER_MS, BUFFER_MS, BUFFER_MS);
        builder.setLoadControl(loadControl.build());
        _player = builder.build();

        LayoutInflater inflator = (LayoutInflater) context.getSystemService(Context.LAYOUT_INFLATER_SERVICE);
        this._player.addListener(this);
        this._playerView = (StyledPlayerView) inflator.inflate(R.layout.player, null, false);
        this._container = new LinearLayout(context);
        LinearLayout.LayoutParams params = new LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.MATCH_PARENT);
        this._textureView = new TextureView(context);
        this._container.addView(this._textureView, params);
    }

    static class Utils {
        private Class<?> canvasUtils;
        private Method canvasCreateRenderAndAttachToGLContext;

        private Method canvasCreateSurfaceTexture;

        private Method canvasUpdateTexImage;

        Utils() {
            try {
                canvasUtils = Class.forName("org.nativescript.canvas.Utils");
                canvasCreateRenderAndAttachToGLContext = canvasUtils.getDeclaredMethod("createRenderAndAttachToGLContext", Long.TYPE, SurfaceTexture.class);
                canvasCreateSurfaceTexture = canvasUtils.getDeclaredMethod("createSurfaceTexture", Long.TYPE);
                canvasUpdateTexImage = canvasUtils.getDeclaredMethod("updateTexImage", Long.TYPE, Boolean.TYPE, SurfaceTexture.class, Class.forName("org.nativescript.canvas.TextureRender"), Integer.TYPE, Integer.TYPE, Integer.TYPE, Integer.TYPE);
            } catch (Exception e) {
                if (IS_DEBUG) {
                    android.util.Log.d("JS", "" + e);
                }
            }
        }

        Object createRenderAndAttachToGLContext(long state, SurfaceTexture st) {
            try {
                return canvasCreateRenderAndAttachToGLContext.invoke(null, state, st);
            } catch (IllegalAccessException ignored) {
                return null;
            } catch (InvocationTargetException ignored) {
                return null;
            }
        }

        Object[] createSurfaceTexture(long state) {
            try {
                return (Object[]) canvasCreateSurfaceTexture.invoke(null, state);
            } catch (IllegalAccessException ignored) {
                return null;
            } catch (InvocationTargetException ignored) {
                return null;
            }
        }

        public void updateTexImage(long state, boolean flipY, SurfaceTexture st, Object render, int videoWidth, int videoHeight, int internalFormat, int format) {
            try {
                Class<?> TextureRender = Class.forName("org.nativescript.canvas.TextureRender");
                canvasUpdateTexImage.invoke(null, state, flipY, st, TextureRender.cast(render), videoWidth, videoHeight, internalFormat, format);
            } catch (IllegalAccessException ignored) {
            } catch (InvocationTargetException ignored) {
            } catch (ClassNotFoundException ignored) {
            }
        }
    }


    private final Utils _utils = new Utils();


    public void getCurrentFrame(boolean isLoaded, long state, boolean flipY, int internalFormat,
                                int format) {
        if (isLoaded) {
            View surfaceView = this._playerView.getVideoSurfaceView();
            if (surfaceView instanceof TextureView) {
                SurfaceTexture st = ((TextureView) surfaceView).getSurfaceTexture();
                if (st != null) {
                    // @ts-ignore
                    this._render = _utils.createRenderAndAttachToGLContext(state, st);
                    this._st = st;
                }
            }
        }

        if (this._st == null) {
            Object[] result = _utils.createSurfaceTexture(state);
            assert result != null;
            this._st = (SurfaceTexture) result[0];

            this._st.setOnFrameAvailableListener(this);

            this._surface = new Surface(this._st);
            this._player.setVideoSurface(this._surface);
            this._render = result[1];
        }

        if (this._st != null) {
            if (!this._hasFrame) {
                return;
            }
            _utils.updateTexImage(state, flipY, this._st, this._render, this._videoWidth, this._videoHeight, internalFormat, format);
            this._hasFrame = false;
        }
    }

    public StyledPlayerView getPlayerView(){
        return this._playerView;
    }

    public void setPlayer(){
          this._playerView.setPlayer(this._player);
    }

    public LinearLayout getContainer() {
        return this._container;
    }

    public void setSurface(Surface surface) {
        this._player.setVideoSurface(surface);
    }

    private void _setSrc(String value) {
        try {

            this._player.setMediaItems(Collections.singletonList(MediaItem.fromUri(Uri.parse(value))), true);
            this._player.prepare();
            if (this._autoplay) {
                this._player.setPlayWhenReady(true);
            }
        } catch (Exception e) {
            if (IS_DEBUG) {
                android.util.Log.d("JS", "" + e);
            }
        }
    }

    public void play() {
        this._player.setPlayWhenReady(true);
    }

    public void pause() {
        this._player.setPlayWhenReady(false);
    }

    public boolean getMuted() {
        return this._player.isDeviceMuted();
    }

    public void setMuted(boolean value) {
        this._player.setDeviceMuted(value);
    }

    public long getDuration() {
        return this._player.getDuration();
    }

    public long getCurrentTime() {
        return this._player.getCurrentPosition() / 1000;
    }

    public void setCurrentTime(long value) {
        this._player.seekTo((int) (value * 1000), 0);
    }

    public String getSrc() {
        return this._src;
    }

    public void setSrc(String value) {
        this._src = value;
        this._setSrc(value);
    }

    public boolean getAutoplay() {
        return this._autoplay;
    }

    public void setAutoplay(boolean value) {
        this._player.setPlayWhenReady(value);
    }

    public boolean getControls() {
        return this._playerView.getUseController();
    }

    public void setControls(boolean enabled) {
        this._playerView.setUseController(enabled);
    }

    public boolean getLoop() {
        return this._loop;
    }

    public void setLoop(boolean value) {
        this._loop = value;
        if (value) {
            this._player.setRepeatMode(com.google.android.exoplayer2.Player.REPEAT_MODE_ALL);
        } else {
            this._player.setRepeatMode(com.google.android.exoplayer2.Player.REPEAT_MODE_OFF);
        }
    }


    public int getWidth() {
        int width = this._container.getWidth();
        if (width == 0) {
            ViewGroup.LayoutParams rootParams = this._container.getLayoutParams();
            if (rootParams != null) {
                return rootParams.width;
            }
        }
        return width;
    }

    public int getHeight() {
        int height = this._container.getHeight();
        if (height == 0) {
            ViewGroup.LayoutParams rootParams = this._container.getLayoutParams();
            if (rootParams != null) {
                return rootParams.height;
            }
        }
        return height;
    }

    public void layoutNative(int width, int height) {

        ViewGroup.LayoutParams rootParams = this._container.getLayoutParams();

        if (rootParams != null && width == rootParams.width && height == rootParams.height) {
            return;
        }

        if (width != 0 && height != 0) {
            if (rootParams == null) {
                rootParams = new android.widget.FrameLayout.LayoutParams(0, 0);
            }
            rootParams.width = width;
            rootParams.height = height;
            FrameLayout.LayoutParams surfaceParams = new android.widget.FrameLayout.LayoutParams(0, 0);

            this._container.setLayoutParams(rootParams);

            int w = android.view.View.MeasureSpec.makeMeasureSpec(width, android.view.View.MeasureSpec.EXACTLY);
            int h = android.view.View.MeasureSpec.makeMeasureSpec(height, android.view.View.MeasureSpec.EXACTLY);

            this._container.measure(w, h);

            this._container.layout(0, 0, width, height);

            if (this._st != null) {
                this._st.setDefaultBufferSize(width, height);
            }
        }
    }
}