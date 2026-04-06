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


import android.view.TextureView;

import android.graphics.SurfaceTexture;


import androidx.annotation.Nullable;
import androidx.media3.common.AudioAttributes;
import androidx.media3.common.C;
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
import androidx.media3.database.StandaloneDatabaseProvider;
import androidx.media3.datasource.DefaultDataSource;
import androidx.media3.datasource.DefaultHttpDataSource;
import androidx.media3.datasource.cache.CacheDataSource;
import androidx.media3.datasource.cache.LeastRecentlyUsedCacheEvictor;
import androidx.media3.datasource.cache.SimpleCache;
import androidx.media3.exoplayer.DefaultLoadControl;
import androidx.media3.exoplayer.ExoPlayer;
import androidx.media3.exoplayer.source.DefaultMediaSourceFactory;
import androidx.media3.ui.PlayerView;

import org.nativescript.canvas_media.R;

@UnstableApi
public class VideoHelper implements Player.Listener, SurfaceTexture.OnFrameAvailableListener {
	public static class Source {
		String src;
		String type;
	}

	LinearLayout _container;

	ArrayList<Source> _sourceView;
	PlayerView _playerView;
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

	// --- GL fast-path state (2D canvas only) ------------------------------------
	// These fields are initialised lazily on the first drawVideoFrame2D call with
	// backendType == 1 (GL).  The OES texture lives in the 2D canvas's EGL context
	// so Skia can sample from it directly without a CPU round-trip.
	SurfaceTexture _glSt;
	Surface _glSurface;
	int _glTextureId = -1;
	volatile boolean _glHasFrame = false;
	Object _glRender; // TextureRender — kept alive to own the OES texture
	// ---------------------------------------------------------------------------

	// Cached reflection for drawVideoFrame2D — resolved once, reused per frame
	private static Class<?> _drawImageClass;
	private static Method _drawImageDxDy;
	private static Method _drawImageDxDyDwDh;
	private static Method _drawImageSrcDst;
	private static boolean _drawImageResolved = false;

	private static void ensureDrawImageMethods() {
		if (_drawImageResolved) return;
		try {
			_drawImageClass = Class.forName("org.nativescript.canvas.NSCCanvasRenderingContext2D");
			_drawImageDxDy = _drawImageClass.getMethod("drawImage", long.class, android.graphics.Bitmap.class, float.class, float.class);
			_drawImageDxDyDwDh = _drawImageClass.getMethod("drawImage", long.class, android.graphics.Bitmap.class, float.class, float.class, float.class, float.class);
			_drawImageSrcDst = _drawImageClass.getMethod("drawImage", long.class, android.graphics.Bitmap.class, float.class, float.class, float.class, float.class, float.class, float.class, float.class, float.class);
		} catch (Exception ignored) {}
		_drawImageResolved = true;
	}

	// Reusable bitmap for getCurrentBitmap to avoid per-frame allocation
	private android.graphics.Bitmap _reusableBitmap;

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
			_timer.schedule(new TimerTask() {
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
		if (_cache == null) {
			_cache = new SimpleCache(cacheDir, _leastRecentlyUsedCacheEvictor, _exoDatabaseProvider);
		}
		DefaultHttpDataSource.Factory factory = new DefaultHttpDataSource.Factory();
		factory.setUserAgent(Util.getUserAgent(context, packageName));
		_dsf = new DefaultDataSource.Factory(context, factory);
		_cdsf = new CacheDataSource.Factory();
		_cdsf.setCache(_cache);
		_cdsf.setUpstreamDataSourceFactory(_dsf);
		_msf = new DefaultMediaSourceFactory(_cdsf);

		ExoPlayer.Builder builder = new ExoPlayer.Builder(context);
		builder.setMediaSourceFactory(_msf);

		DefaultLoadControl.Builder loadControl = new DefaultLoadControl.Builder();
		loadControl.setBufferDurationsMs(BUFFER_MS, DefaultLoadControl.DEFAULT_MAX_BUFFER_MS, BUFFER_MS, BUFFER_MS);
		builder.setLoadControl(loadControl.build());
		_player = builder.build();

		LayoutInflater inflator = (LayoutInflater) context.getSystemService(Context.LAYOUT_INFLATER_SERVICE);
		this._player.addListener(this);
		this._playerView = (PlayerView) inflator.inflate(R.layout.player, null, false);
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

		// GL fast-path helpers (2D canvas only)
		private Method canvasCreate2DContextSurfaceTexture;
		private Method canvasDrawVideoFrameGL;

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
			// GL fast-path methods — resolved separately so a missing method
			// does not break the CPU path.
			try {
				canvasCreate2DContextSurfaceTexture = canvasUtils.getDeclaredMethod(
						"create2DContextSurfaceTexture", Long.TYPE);
				canvasDrawVideoFrameGL = canvasUtils.getDeclaredMethod(
						"drawVideoFrameGL",
						Long.TYPE,          // context
						Integer.TYPE,       // textureId
						SurfaceTexture.class,
						Integer.TYPE,       // videoWidth
						Integer.TYPE,       // videoHeight
						Float.TYPE,         // sx
						Float.TYPE,         // sy
						Float.TYPE,         // sw
						Float.TYPE,         // sh
						Float.TYPE,         // dx
						Float.TYPE,         // dy
						Float.TYPE,         // dw
						Float.TYPE          // dh
				);
			} catch (Exception e) {
				if (IS_DEBUG) {
					android.util.Log.d("JS", "GL fast-path unavailable: " + e);
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

		/** Create a GL OES texture + SurfaceTexture inside the 2D canvas EGL context.
		 *  Returns [SurfaceTexture, Integer textureId, TextureRender], or null on failure. */
		Object[] create2DContextSurfaceTexture(long context) {
			if (canvasCreate2DContextSurfaceTexture == null) return null;
			try {
				return (Object[]) canvasCreate2DContextSurfaceTexture.invoke(null, context);
			} catch (IllegalAccessException ignored) {
				return null;
			} catch (InvocationTargetException e) {
				if (IS_DEBUG) android.util.Log.d("JS", "create2DContextSurfaceTexture: " + e.getCause());
				return null;
			}
		}

		/** Draw a video frame via the GL fast-path (OES texture → Skia GPU, zero CPU copy).
		 *  Returns true when the frame was drawn. */
		boolean drawVideoFrameGL(
				long context, int textureId, SurfaceTexture surfaceTexture,
				int videoWidth, int videoHeight,
				float sx, float sy, float sw, float sh,
				float dx, float dy, float dw, float dh) {
			if (canvasDrawVideoFrameGL == null) return false;
			try {
				Object result = canvasDrawVideoFrameGL.invoke(
						null, context, textureId, surfaceTexture,
						videoWidth, videoHeight,
						sx, sy, sw, sh,
						dx, dy, dw, dh);
				return result instanceof Boolean && (Boolean) result;
			} catch (IllegalAccessException ignored) {
				return false;
			} catch (InvocationTargetException e) {
				if (IS_DEBUG) android.util.Log.d("JS", "drawVideoFrameGL: " + e.getCause());
				return false;
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

	public PlayerView getPlayerView() {
		return this._playerView;
	}

	public android.graphics.Bitmap getCurrentBitmap() {
		View surfaceView = this._playerView.getVideoSurfaceView();
		if (surfaceView instanceof TextureView) {
			TextureView tv = (TextureView) surfaceView;
			if (tv.isAvailable()) {
				int w = tv.getWidth();
				int h = tv.getHeight();
				if (w > 0 && h > 0) {
					if (_reusableBitmap == null || _reusableBitmap.getWidth() != w || _reusableBitmap.getHeight() != h) {
						if (_reusableBitmap != null) {
							_reusableBitmap.recycle();
						}
						_reusableBitmap = android.graphics.Bitmap.createBitmap(w, h, android.graphics.Bitmap.Config.ARGB_8888);
					}
					return tv.getBitmap(_reusableBitmap);
				}
			}
		}
		return null;
	}

	private boolean drawFrame2DBitmap(long context, float sx, float sy, float sw, float sh, float dx, float dy, float dw, float dh) {
		android.graphics.Bitmap bitmap = getCurrentBitmap();
		if (bitmap == null) return false;
		ensureDrawImageMethods();
		if (_drawImageSrcDst == null) return false;
		try {
			Object result = _drawImageSrcDst.invoke(null, context, bitmap, sx, sy, sw, sh, dx, dy, dw, dh);
			return result instanceof Boolean && (Boolean) result;
		} catch (Exception e) {
			return false;
		}
	}

	/**
	 * Draw the current video frame into a 2D canvas context.
	 *
	 * @param backendType  Rendering backend of the target context:
	 *                     0 = CPU  — CPU bitmap readback (TextureView → Bitmap → Skia).
	 *                     1 = GL   — GL fast-path: OES texture → Skia GPU image (zero CPU copy).
	 *                     2 = Vulkan — falls back to CPU bitmap path (TODO: AHardwareBuffer path).
	 *                     3 = Metal  — iOS only; not reached on Android.
	 * @param context      Native pointer to the CanvasRenderingContext2D Rust struct.
	 */
	public boolean drawVideoFrame2D(int backendType, long context, float dx, float dy) {
		float w = (float) _videoWidth;
		float h = (float) _videoHeight;
		if (backendType == 1) {
			return drawFrame2DGL(context, 0, 0, w, h, dx, dy, w, h);
		}
		return drawFrame2DBitmap(context, 0, 0, w, h, dx, dy, w, h);
	}

	public boolean drawVideoFrame2D(int backendType, long context, float dx, float dy, float dw, float dh) {
		float w = (float) _videoWidth;
		float h = (float) _videoHeight;
		if (backendType == 1) {
			return drawFrame2DGL(context, 0, 0, w, h, dx, dy, dw, dh);
		}
		return drawFrame2DBitmap(context, 0, 0, w, h, dx, dy, dw, dh);
	}

	public boolean drawVideoFrame2D(int backendType, long context, float sx, float sy, float sw, float sh, float dx, float dy, float dw, float dh) {
		if (backendType == 1) {
			return drawFrame2DGL(context, sx, sy, sw, sh, dx, dy, dw, dh);
		}
		return drawFrame2DBitmap(context, sx, sy, sw, sh, dx, dy, dw, dh);
	}

	/**
	 * GL fast-path: draw the current video frame by having Skia sample an OES texture
	 * directly, with no CPU round-trip.
	 *
	 * On the first call the method:
	 *  1. Calls into Rust to make the 2D canvas EGL context current.
	 *  2. Creates a TextureRender (generates an OES texture) inside that EGL context.
	 *  3. Wraps the OES texture in a SurfaceTexture and directs the player to it.
	 *
	 * On every subsequent call it delegates to Utils.drawVideoFrameGL which:
	 *  1. Makes the 2D context current.
	 *  2. Calls SurfaceTexture.updateTexImage() to latch the newest frame.
	 *  3. Calls into Rust/Skia to draw the OES texture directly (no Bitmap).
	 *
	 * Falls back to the CPU bitmap path if the GL resources cannot be initialised.
	 */
	private boolean drawFrame2DGL(long context, float sx, float sy, float sw, float sh, float dx, float dy, float dw, float dh) {
		// Lazy initialisation — runs only once per VideoHelper instance.
		if (_glSt == null) {
			Object[] result = _utils.create2DContextSurfaceTexture(context);
			if (result == null) {
				// GL init failed; fall back to CPU path.
				return drawFrame2DBitmap(context, sx, sy, sw, sh, dx, dy, dw, dh);
			}
			_glSt = (SurfaceTexture) result[0];
			_glTextureId = (Integer) result[1];
			_glRender = result[2];

			_glSt.setOnFrameAvailableListener(st -> {
				_glHasFrame = true;
				if (_callback != null) {
					_callback.onVideoFrame();
				}
			});

			_glSurface = new Surface(_glSt);
			_player.setVideoSurface(_glSurface);
		}

		if (!_glHasFrame) {
			// No new decoded frame yet; nothing to draw.
			return false;
		}

		boolean drawn = _utils.drawVideoFrameGL(
				context, _glTextureId, _glSt,
				_videoWidth, _videoHeight,
				sx, sy, sw, sh,
				dx, dy, dw, dh);

		if (drawn) {
			_glHasFrame = false;
		}
		return drawn;
	}

	public void setPlayer() {
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

	public double getCurrentTime() {
		return this._player.getCurrentPosition() / 1000.0;
	}

	public void setCurrentTime(int value) {
		this._player.seekTo((value * 1000), 0);
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
			this._player.setRepeatMode(Player.REPEAT_MODE_ALL);
		} else {
			this._player.setRepeatMode(Player.REPEAT_MODE_OFF);
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
	

	@Override
	public void onEvents(Player player, Player.Events events) {}

	
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
	public void  onPlaybackStateChanged(@Player.State int playbackState) {}
	
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
}
