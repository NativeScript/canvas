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
import java.util.List;
import java.util.concurrent.atomic.AtomicBoolean;
import androidx.annotation.Nullable;
import androidx.media3.common.AudioAttributes;
import androidx.media3.common.DeviceInfo;
import androidx.media3.common.MediaItem;
import androidx.media3.common.MediaMetadata;
import androidx.media3.common.Metadata;
import androidx.media3.common.PlaybackException;
import androidx.media3.common.PlaybackParameters;
import androidx.media3.common.Player;
import androidx.media3.common.Timeline;
import androidx.media3.common.TrackSelectionParameters;
import androidx.media3.common.Tracks;
import androidx.media3.common.VideoSize;
import androidx.media3.common.text.Cue;
import androidx.media3.common.text.CueGroup;
import androidx.media3.common.util.UnstableApi;
import androidx.media3.common.util.Util;



import androidx.media3.datasource.DefaultDataSource;
import androidx.media3.datasource.DefaultHttpDataSource;
import androidx.media3.datasource.cache.CacheDataSource;
import androidx.media3.datasource.cache.LeastRecentlyUsedCacheEvictor;
import androidx.media3.datasource.cache.SimpleCache;
import androidx.media3.database.StandaloneDatabaseProvider;
import androidx.media3.exoplayer.DefaultLoadControl;
import androidx.media3.exoplayer.DefaultRenderersFactory;
import androidx.media3.exoplayer.ExoPlayer;
import androidx.media3.exoplayer.audio.AudioSink;
import androidx.media3.exoplayer.audio.DefaultAudioSink;
import androidx.media3.exoplayer.source.DefaultMediaSourceFactory;
import androidx.media3.common.audio.AudioProcessor;

import java.lang.reflect.Method;

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
    private final AudioContextTapProcessor _tapProcessor = new AudioContextTapProcessor();
    private volatile Object _tapSourceNode;
    private volatile Object _attachedContext;
    private volatile int _tapNodeSampleRate;
    private volatile int _tapNodeChannels;
    private volatile Method _tapNodePushFrames;
    private volatile Method _tapNodeEndStream;
    private volatile Method _tapNodeConfigureFormat;
    private volatile Method _ctxCreateExternalPcm;
    private volatile boolean _tapSuppressesDirectOutput = false;
    private volatile boolean _muted = false;
    private static final int MAX_TAP_REFLECTION_FAILURES = 8;
    private final Object _tapStateLock = new Object();
    private volatile int _tapReflectionFailures = 0;

    private final AudioContextTapProcessor.Sink _tapSink = new AudioContextTapProcessor.Sink() {
        @Override
        public void onPcmFrames(float[] interleaved, int sampleCount, int sampleRate, int channels) {
            Object node = _tapSourceNode;
            Method push = _tapNodePushFrames;
            if (node == null || push == null || sampleCount == 0 || sampleRate <= 0 || channels <= 0) return;

            if (_tapNodeSampleRate != sampleRate || _tapNodeChannels != channels) {
                Method configure = _tapNodeConfigureFormat;
                if (configure == null) return;
                try {
                    configure.invoke(node, sampleRate, channels);
                    _tapNodeSampleRate = sampleRate;
                    _tapNodeChannels = channels;
                } catch (Throwable t) {
                    handleTapReflectionFailure("configureFormat reflection failed", t);
                    return;
                }
            }

            float[] payload = interleaved;
            if (interleaved.length != sampleCount) {
                payload = new float[sampleCount];
                System.arraycopy(interleaved, 0, payload, 0, sampleCount);
            }
            try {
                push.invoke(node, (Object) payload);
                _tapReflectionFailures = 0;
            } catch (Throwable t) {
                handleTapReflectionFailure("pushFrames reflection failed", t);
            }
        }

        @Override
        public boolean isActive() {
            return _tapSourceNode != null;
        }
    };

    private void handleTapReflectionFailure(String label, Throwable t) {
        int failures = ++_tapReflectionFailures;
        android.util.Log.w("AudioHelper", label, t);
        if (failures >= MAX_TAP_REFLECTION_FAILURES) {
            android.util.Log.w("AudioHelper", "disabling audio context tap after repeated failures");
            detachAudioContextTap();
        }
    }

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

    private void safeInvoke(String label, Runnable action) {
        try {
            action.run();
        } catch (Throwable t) {
            android.util.Log.w("AudioHelper", label + " failed", t);
        }
    }

    private void dispatchOnHandler(String label, Runnable action) {
        Handler currentHandler = handler;
        if (currentHandler == null) {
            safeInvoke(label, action);
            return;
        }

        Looper targetLooper = currentHandler.getLooper();
        if (targetLooper != null && targetLooper == Looper.myLooper()) {
            safeInvoke(label, action);
            return;
        }

        currentHandler.post(() -> safeInvoke(label, action));
    }

    private void updateEffectivePlayerVolume() {
        try {
            if (_player != null) {
                _player.setVolume((_muted || _tapSuppressesDirectOutput) ? 0f : 1f);
            }
        } catch (Throwable t) {
            android.util.Log.w("AudioHelper", "updateEffectivePlayerVolume failed", t);
        }
    }

    public AudioHelper(Context context, String cacheRoot) {
        File cacheDir = new File(cacheRoot, "MEDIA_PLAYER_CACHE");
        if (!cacheDir.exists()) { cacheDir.mkdirs(); }
        

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

        _tapProcessor.setSink(_tapSink);
        DefaultRenderersFactory rf = new DefaultRenderersFactory(context) {
            @Override
            protected AudioSink buildAudioSink(android.content.Context ctx,
                                               boolean enableFloatOutput,
                                               boolean enableAudioTrackPlaybackParams) {
                return new DefaultAudioSink.Builder(ctx)
                        .setEnableFloatOutput(enableFloatOutput)
                        .setEnableAudioTrackPlaybackParams(enableAudioTrackPlaybackParams)
                        .setAudioProcessors(new AudioProcessor[]{ _tapProcessor })
                        .build();
            }
        };

        ExoPlayer.Builder builder = new ExoPlayer.Builder(context);
        builder.setMediaSourceFactory(_msf);
        builder.setRenderersFactory(rf);

        DefaultLoadControl.Builder loadControl = new DefaultLoadControl.Builder();
        loadControl.setBufferDurationsMs(500, DefaultLoadControl.DEFAULT_MAX_BUFFER_MS, 500, 500);
        builder.setLoadControl(loadControl.build());
        _player = builder.build();
        _player.addListener(this);
       _player.setAudioAttributes(AudioAttributes.DEFAULT, true);
                updateEffectivePlayerVolume();

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
        dispatchOnHandler("onIsPlayingChanged", () -> {
            if (this._callback != null) {
                safeInvoke("onDurationChange callback", () -> this._callback.onDurationChange(duration));
            }
            this._playing = isPlaying;
            if (isPlaying) {
                if (this._callback != null) {
                    safeInvoke("onPlaying callback", () -> this._callback.onPlaying());
                    safeInvoke("onLoadedData callback", () -> this._callback.onLoadedData());
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
                        dispatchOnHandler("onCurrentTimeChanged", () -> {
                            long current = _player.getCurrentPosition();
                            if (that._callback != null) {
                                safeInvoke("onCurrentTimeChanged callback", () -> that._callback.onCurrentTimeChanged(current));
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
	            long currentTime = this._player.getCurrentPosition();
	            if (this._callback != null) {
	                safeInvoke("onCurrentTimeChanged callback", () -> this._callback.onCurrentTimeChanged(currentTime));
	            }
	            if (this._audioView != null) {
	                this._audioView.setPlaying(false);
	                this._audioView.setCurrentTime(currentTime);
	            }
	        }
        });
    }

    @Override
    public void onPlayerError(PlaybackException error) {
        final String message = error == null ? "Playback error" : error.getMessage();
        dispatchOnHandler("onPlayerError", () -> {
            if (this._callback != null) {
                safeInvoke("onError callback", () -> this._callback.onError(message));
            }
        });
    }

    @Override
    public void onEvents(Player player, Player.Events events) { }

    	@Override
	public void  onTimelineChanged(Timeline timeline, @Player.TimelineChangeReason int reason) {}


	@Override
	public void  onMediaItemTransition(
		@Nullable MediaItem mediaItem, @Player.MediaItemTransitionReason int reason) {}


	@Override
	public void onTracksChanged(Tracks tracks) {}
	
	@Override
	public void onMediaMetadataChanged(MediaMetadata mediaMetadata) {}
	
	@Override
	public void  onPlaylistMetadataChanged(MediaMetadata mediaMetadata) {}
	
	@Override
	public void  onIsLoadingChanged(boolean isLoading) {}
	
	@Deprecated
	@UnstableApi
	@Override
	public void  onLoadingChanged(boolean isLoading) {}
	
	@Override
	public void  onAvailableCommandsChanged(Player.Commands availableCommands) {}
	
	@Override
	public void  onTrackSelectionParametersChanged(TrackSelectionParameters parameters) {}
	
	@Deprecated
	@UnstableApi
	@Override
	public void  onPlayerStateChanged(boolean playWhenReady, @Player.State int playbackState) {}



    @Override
    public void onPlaybackStateChanged(@Player.State int playbackState) {
        if (playbackState == Player.STATE_READY) {
            dispatchOnHandler("onPlaybackStateChanged", () -> {
                if (!this._canPlayFired) {
                    this._canPlayFired = true;
                    if (this._callback != null) {
                        safeInvoke("onCanPlay callback", () -> this._callback.onCanPlay());
                    }
                }

                if (this._loadedDataFired.compareAndSet(false, true)) {
                    if (this._callback != null) {
                        safeInvoke("onLoadedData callback", () -> this._callback.onLoadedData());
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
                        safeInvoke("onCanPlayThrough callback", () -> this._callback.onCanPlayThrough());
                    }
                }
            });
        }
    }



@Override
	public void  onPlayWhenReadyChanged(
		boolean playWhenReady, @Player.PlayWhenReadyChangeReason int reason) {}
	
	@Override
	public void  onPlaybackSuppressionReasonChanged(
		@Player.PlaybackSuppressionReason int playbackSuppressionReason) {}
	
	@Override
	public void  onRepeatModeChanged(@Player.RepeatMode int repeatMode) {}
	
	@Override
	public void  onShuffleModeEnabledChanged(boolean shuffleModeEnabled) {}
	
	@Override
	public void  onPlayerErrorChanged(@Nullable PlaybackException error) {}
	
	@Deprecated
	@UnstableApi
	@Override
	public void  onPositionDiscontinuity(@Player.DiscontinuityReason int reason) {}
	
	@Override
	public void  onPositionDiscontinuity(
		Player.PositionInfo oldPosition, Player.PositionInfo newPosition, @Player.DiscontinuityReason int reason) {}
	
	@Override
	public void  onPlaybackParametersChanged(PlaybackParameters playbackParameters) {}
	
	@Override
	public void  onSeekBackIncrementChanged(long seekBackIncrementMs) {}
	
	@Override
	public void  onSeekForwardIncrementChanged(long seekForwardIncrementMs) {}
	
	@Override
	public void  onMaxSeekToPreviousPositionChanged(long maxSeekToPreviousPositionMs) {}
	
	@UnstableApi
	@Override
	public void  onAudioSessionIdChanged(int audioSessionId) {}
	
	@Override
	public void  onAudioAttributesChanged(AudioAttributes audioAttributes) {}
	
	@Override
	public void  onVolumeChanged(float volume) {}
	
	@Override
	public void  onSkipSilenceEnabledChanged(boolean skipSilenceEnabled) {}
	
	@Override
	public void  onDeviceInfoChanged(DeviceInfo deviceInfo) {}
	
	@Override
	public void  onDeviceVolumeChanged(int volume, boolean muted) {}
	
	@Override
	public void  onSurfaceSizeChanged(int width, int height) {}
	
	@Override
	public void  onRenderedFirstFrame() {}

	
	@Deprecated
	@UnstableApi
	@Override
	public void  onCues(List<Cue> cues) {}
	
	@Override
	public void  onCues(CueGroup cueGroup) {}


	@UnstableApi
	public void onMetadata(Metadata metadata) {}





    public void play() {
        try {
            this._player.setPlayWhenReady(true);
            this._player.play();
        } catch (Exception e) { }
    }

    public Object attachAudioContextTap(Object contextNative) {
        if (contextNative == null) return null;
        try {
            Method create = contextNative.getClass()
                    .getMethod("createExternalPcmSource", int.class, int.class);
            Object node = create.invoke(contextNative, 48000, 2);
            if (node == null) return null;

            Method push = node.getClass().getMethod("pushFrames", float[].class);
            Method end = node.getClass().getMethod("endStream");
            Method configure = node.getClass().getMethod("configureFormat", int.class, int.class);

            synchronized (_tapStateLock) {
                _attachedContext = contextNative;
                _ctxCreateExternalPcm = create;
                _tapNodePushFrames = push;
                _tapNodeEndStream = end;
                _tapNodeConfigureFormat = configure;
                _tapNodeSampleRate = 48000;
                _tapNodeChannels = 2;
                _tapReflectionFailures = 0;
                _tapSuppressesDirectOutput = true;
                _tapSourceNode = node;
            }
            updateEffectivePlayerVolume();
            return node;
        } catch (NoSuchMethodException e) {
            android.util.Log.w("AudioHelper", "attachAudioContextTap: contextNative missing expected surface", e);
            return null;
        } catch (Throwable t) {
            android.util.Log.w("AudioHelper", "attachAudioContextTap failed", t);
            return null;
        }
    }

    public void detachAudioContextTap() {
        Object node;
        Method end;
        synchronized (_tapStateLock) {
            node = _tapSourceNode;
            end = _tapNodeEndStream;
            _tapSourceNode = null;
            _attachedContext = null;
            _ctxCreateExternalPcm = null;
            _tapNodePushFrames = null;
            _tapNodeEndStream = null;
            _tapNodeConfigureFormat = null;
            _tapNodeSampleRate = 0;
            _tapNodeChannels = 0;
            _tapReflectionFailures = 0;
            _tapSuppressesDirectOutput = false;
        }
        updateEffectivePlayerVolume();
        if (node != null && end != null) {
            try { end.invoke(node); } catch (Throwable ignored) {}
        }
    }

    public void pause() {
        try {
            this._player.setPlayWhenReady(false);
            this._player.pause();
        } catch (Exception e) { }
    }

    public void release() {
        try { detachAudioContextTap(); } catch (Throwable ignored) {}
        try { if (_timer != null) { _timer.cancel(); _timer = null; } } catch (Throwable ignored) {}
        try { if (_player != null) { _player.release(); } } catch (Throwable ignored) {}
    }

    public boolean getMuted() {
        return this._muted;
    }

    public void setMuted(boolean value) {
        this._muted = value;
        updateEffectivePlayerVolume();
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
