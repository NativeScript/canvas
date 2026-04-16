package org.nativescript.canvas.media;

import android.content.Context;
import android.graphics.ImageFormat;
import android.media.Image;
import android.media.ImageReader;
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
import java.util.concurrent.atomic.AtomicBoolean;


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

import android.media.MediaCodecInfo;
import android.media.MediaCodecList;

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

	// SimpleCache is provided centrally to avoid duplicate instances locking the same folder.
	private LeastRecentlyUsedCacheEvictor _leastRecentlyUsedCacheEvictor;
	private StandaloneDatabaseProvider _exoDatabaseProvider;
	private static final long _exoPlayerCacheSize = 100 * 1024 * 1024;
	private DefaultMediaSourceFactory _msf;
	private CacheDataSource.Factory _cdsf;
	boolean _isCustom = false;
	boolean _playing = false;
	private final AtomicBoolean _loadedDataFired = new AtomicBoolean(false);
	private boolean _canPlayFired = false;
	private boolean _canPlayThroughFired = false;
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

	volatile boolean _hasFrame = false;
	static int BUFFER_MS = 500;
	public static boolean IS_DEBUG = true;

	SurfaceTexture _glSt;
	Surface _glSurface;
	int _glTextureId = -1;
	volatile boolean _glHasFrame = false;
	Object _glRender; // TextureRender — kept alive to own the OES texture
	private boolean _glInitFailed = false;
	// Tracks which path currently owns the player's video surface to prevent conflicts.
	private enum SurfaceOwner { NONE, WEBGL, GL2D, BITMAP }
	private SurfaceOwner _surfaceOwner = SurfaceOwner.NONE;

	// ImageReader-backed surface for the CPU/Vulkan bitmap path.
	// Provides video frames without requiring any view hierarchy or window attachment —
	// the Android equivalent of a detached <video> element on the web.
	private ImageReader _imageReader;
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

	// Dimensions of the most-recently returned getRgbaBytes() frame
	private int _lastRgbaWidth = 0;
	private int _lastRgbaHeight = 0;

	public interface Callback {
		void onDurationChange(long duration);

		void onPlaying();

		void onVideoSizeChanged(int width, int height);

		void onCurrentTimeChanged(long time);

		void onVideoFrame();

		void onLoadedData();

		void onCanPlay();

		void onCanPlayThrough();

		void onError(String message);
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
		if (this._callback != null) {
			this._callback.onError(error.getMessage());
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

		private Method canvasCreate2DContextSurfaceTexture;
		private Method canvasDrawVideoFrameGL;

		private Method canvasUpdateTexImageFor3D;
		private Method canvasUpdateTexSubImageFor3D;

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
	
			try {
				Class<?> TextureRenderClass = Class.forName("org.nativescript.canvas.TextureRender");
				canvasUpdateTexImageFor3D = canvasUtils.getDeclaredMethod(
						"updateTexImageFor3D",
						Long.TYPE,           // state
						Boolean.TYPE,        // flipY
						SurfaceTexture.class,
						TextureRenderClass,
						Integer.TYPE,        // videoWidth
						Integer.TYPE,        // videoHeight
						Integer.TYPE,        // target
						Integer.TYPE,        // level
						Integer.TYPE,        // internalformat
						Integer.TYPE,        // width
						Integer.TYPE,        // height
						Integer.TYPE,        // depth
						Integer.TYPE,        // border
						Integer.TYPE         // zoffset
				);
				canvasUpdateTexSubImageFor3D = canvasUtils.getDeclaredMethod(
						"updateTexSubImageFor3D",
						Long.TYPE,           // state
						Boolean.TYPE,        // flipY
						SurfaceTexture.class,
						TextureRenderClass,
						Integer.TYPE,        // target
						Integer.TYPE,        // level
						Integer.TYPE,        // xoffset
						Integer.TYPE,        // yoffset
						Integer.TYPE,        // zoffset
						Integer.TYPE,        // width
						Integer.TYPE         // height
				);
			} catch (Exception e) {
				if (IS_DEBUG) {
					android.util.Log.d("JS", "3D texture fast-path unavailable: " + e);
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

		
		public boolean updateTexImageFor3D(long state, boolean flipY, SurfaceTexture st, Object render,
		                                    int videoWidth, int videoHeight,
		                                    int target, int level, int internalformat,
		                                    int width, int height, int depth, int border, int zoffset) {
			if (canvasUpdateTexImageFor3D == null) return false;
			try {
				Class<?> TextureRender = Class.forName("org.nativescript.canvas.TextureRender");
				canvasUpdateTexImageFor3D.invoke(null, state, flipY, st, TextureRender.cast(render),
						videoWidth, videoHeight, target, level, internalformat,
						width, height, depth, border, zoffset);
				return true;
			} catch (Exception e) {
				if (IS_DEBUG) android.util.Log.d("JS", "updateTexImageFor3D: " + e);
				return false;
			}
		}

		public boolean updateTexSubImageFor3D(long state, boolean flipY, SurfaceTexture st, Object render,
		                                       int target, int level,
		                                       int xoffset, int yoffset, int zoffset,
		                                       int width, int height) {
			if (canvasUpdateTexSubImageFor3D == null) return false;
			try {
				Class<?> TextureRender = Class.forName("org.nativescript.canvas.TextureRender");
				canvasUpdateTexSubImageFor3D.invoke(null, state, flipY, st, TextureRender.cast(render),
						target, level, xoffset, yoffset, zoffset, width, height);
				return true;
			} catch (Exception e) {
				if (IS_DEBUG) android.util.Log.d("JS", "updateTexSubImageFor3D: " + e);
				return false;
			}
		}


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
			// Release ImageReader if the bitmap path claimed the surface first.
			if (_surfaceOwner == SurfaceOwner.BITMAP) {
				if (_imageReader != null) { _imageReader.close(); _imageReader = null; }
				_surfaceOwner = SurfaceOwner.NONE;
			}
			// Claim the surface for the WebGL path; GL2D path must not override this.
			this._player.setVideoSurface(this._surface);
			_surfaceOwner = SurfaceOwner.WEBGL;
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


	public boolean getFrameForTexImage3D(boolean isLoaded, long state, boolean flipY,
	                                      int target, int level, int internalformat,
	                                      int width, int height, int depth, int border) {
		ensureGLResources3D(state, width, height);
		if (this._st == null || !this._hasFrame) return false;

		boolean drawn = _utils.updateTexImageFor3D(state, flipY, this._st, this._render,
				this._videoWidth, this._videoHeight,
				target, level, internalformat, width, height, depth, border, 0);
		this._hasFrame = false;
		return drawn;
	}

	public boolean getFrameForTexSubImage3D(boolean isLoaded, long state, boolean flipY,
	                                         int target, int level,
	                                         int xoffset, int yoffset, int zoffset,
	                                         int width, int height) {
		ensureGLResources3D(state, width, height);
		if (this._st == null || !this._hasFrame) return false;

		boolean drawn = _utils.updateTexSubImageFor3D(state, flipY, this._st, this._render,
				target, level, xoffset, yoffset, zoffset, width, height);
		this._hasFrame = false;
		return drawn;
	}


	private void ensureGLResources3D(long state, int width, int height) {
		if (this._st != null && this._render != null) return; // already set up

		Object[] result = _utils.createSurfaceTexture(state);
		if (result == null) return;
		this._st = (SurfaceTexture) result[0];
		this._st.setDefaultBufferSize(width, height);
		this._st.setOnFrameAvailableListener(this);
		this._surface = new Surface(this._st);
		// Release the ImageReader if the bitmap path previously claimed the surface.
		if (_surfaceOwner == SurfaceOwner.BITMAP) {
			if (_imageReader != null) { _imageReader.close(); _imageReader = null; }
			_surfaceOwner = SurfaceOwner.NONE;
		}
		this._player.setVideoSurface(this._surface);
		_surfaceOwner = SurfaceOwner.WEBGL;
		this._render = result[1];
	}

	public PlayerView getPlayerView() {
		return this._playerView;
	}

	/**
	 * Initialise the ImageReader-based video surface for the CPU/Vulkan bitmap path.
	 * Called lazily once video dimensions are known. Works without any window or
	 * view-hierarchy attachment — the Android equivalent of a detached web <video>.
	 */
	private void setupBitmapSurface() {
		if (IS_DEBUG) android.util.Log.d("JS", "setupBitmapSurface: owner=" + _surfaceOwner + " vw=" + _videoWidth + " vh=" + _videoHeight + " reader=" + (_imageReader != null));
		if (_surfaceOwner != SurfaceOwner.NONE) return;
		if (_videoWidth <= 0 || _videoHeight <= 0) return;
		if (_imageReader != null) return;

		try {
			// PRIVATE format lets the hardware decoder keep frames in GPU memory.
			// On API 33+ we can wrap the HardwareBuffer as a Bitmap at zero copy.
			// On older APIs we fall back to RGBA_8888 (may force software decode on
			// some devices, but keeps things readable without a GL readback pass).
			int format = (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU)
					? ImageFormat.PRIVATE
					: android.graphics.PixelFormat.RGBA_8888;

			_imageReader = ImageReader.newInstance(_videoWidth, _videoHeight, format, 2);
			_imageReader.setOnImageAvailableListener(reader -> {
				_hasFrame = true;
				if (_callback != null) _callback.onVideoFrame();
			}, handler);
			_player.setVideoSurface(_imageReader.getSurface());
			_surfaceOwner = SurfaceOwner.BITMAP;
		} catch (Exception e) {
			if (IS_DEBUG) android.util.Log.d("JS", "setupBitmapSurface failed: " + e);
		}
	}

	public android.graphics.Bitmap getCurrentBitmap() {
		// Lazily set up the ImageReader surface when dimensions are first known.
		if (_imageReader == null && _surfaceOwner == SurfaceOwner.NONE) {
			setupBitmapSurface();
		}

		if (_imageReader != null) {
			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
				// API 33+: PRIVATE format → HardwareBuffer → blit into software Bitmap.
				// wrapHardwareBuffer() gives a Config.HARDWARE bitmap whose pixels are
				// not CPU-accessible. Blitting it via Canvas copies it to the software
				// _reusableBitmap so callers (texSubImage3D, drawImage…) can lockPixels.
				try (Image image = _imageReader.acquireLatestImage()) {
					if (image == null) return null;
					android.hardware.HardwareBuffer hb = image.getHardwareBuffer();
					if (hb == null) return null;
					android.graphics.Bitmap hwBmp = android.graphics.Bitmap.wrapHardwareBuffer(hb, null);
					hb.close();
					int w = hwBmp.getWidth(), h = hwBmp.getHeight();
					if (_reusableBitmap == null
							|| _reusableBitmap.getWidth() != w
							|| _reusableBitmap.getHeight() != h) {
						if (_reusableBitmap != null) _reusableBitmap.recycle();
						_reusableBitmap = android.graphics.Bitmap.createBitmap(
								w, h, android.graphics.Bitmap.Config.ARGB_8888);
					}
					new android.graphics.Canvas(_reusableBitmap).drawBitmap(hwBmp, 0, 0, null);
					hwBmp.recycle();
					return _reusableBitmap;
				} catch (Exception e) {
					if (IS_DEBUG) android.util.Log.d("JS", "getCurrentBitmap HW: " + e);
				}
			} else {
				// API < 33: RGBA_8888 format → read pixels from plane[0].
				try (Image image = _imageReader.acquireLatestImage()) {
					if (image == null) return null;
					Image.Plane plane = image.getPlanes()[0];
					java.nio.ByteBuffer buf = plane.getBuffer();
					int w = image.getWidth();
					int h = image.getHeight();
					if (_reusableBitmap == null
							|| _reusableBitmap.getWidth() != w
							|| _reusableBitmap.getHeight() != h) {
						if (_reusableBitmap != null) _reusableBitmap.recycle();
						_reusableBitmap = android.graphics.Bitmap.createBitmap(
								w, h, android.graphics.Bitmap.Config.ARGB_8888);
					}
					_reusableBitmap.copyPixelsFromBuffer(buf);
					return _reusableBitmap;
				} catch (Exception e) {
					if (IS_DEBUG) android.util.Log.d("JS", "getCurrentBitmap SW: " + e);
				}
			}
		}

		// Last-resort fallback: PlayerView's TextureView, only reachable when the
		// player IS in the window (createNativeView() was called and view is visible).
		View surfaceView = this._playerView.getVideoSurfaceView();
		if (surfaceView instanceof TextureView && ((TextureView) surfaceView).isAvailable()) {
			TextureView tv = (TextureView) surfaceView;
			int w = _videoWidth > 0 ? _videoWidth : tv.getWidth();
			int h = _videoHeight > 0 ? _videoHeight : tv.getHeight();
			if (w <= 0 || h <= 0) return null;
			if (_reusableBitmap == null || _reusableBitmap.getWidth() != w || _reusableBitmap.getHeight() != h) {
				if (_reusableBitmap != null) _reusableBitmap.recycle();
				_reusableBitmap = android.graphics.Bitmap.createBitmap(w, h, android.graphics.Bitmap.Config.ARGB_8888);
			}
			return tv.getBitmap(_reusableBitmap);
		}

		return null;
	}

	public java.nio.ByteBuffer getRgbaBuffer() {
		if (_imageReader == null && _surfaceOwner == SurfaceOwner.NONE) {
			setupBitmapSurface();
		}

		if (_imageReader != null) {
			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
				// ── API 33+: PRIVATE → HardwareBuffer → software Bitmap → RGBA ──────────
				try (Image image = _imageReader.acquireLatestImage()) {
					if (image == null) return null;
					android.hardware.HardwareBuffer hb = image.getHardwareBuffer();
					if (hb == null) return null;
					android.graphics.Bitmap hwBmp = android.graphics.Bitmap.wrapHardwareBuffer(hb, null);
					hb.close();
					int w = hwBmp.getWidth(), h = hwBmp.getHeight();
					android.graphics.Bitmap softBmp = android.graphics.Bitmap.createBitmap(
							w, h, android.graphics.Bitmap.Config.ARGB_8888);
					new android.graphics.Canvas(softBmp).drawBitmap(hwBmp, 0, 0, null);
					hwBmp.recycle();

					// copyPixelsToBuffer requires native byte order; on ARM (little-endian)
					// ARGB_8888 int 0xAARRGGBB → bytes [B, G, R, A].  Swap B↔R → RGBA.
					java.nio.ByteBuffer buf = java.nio.ByteBuffer
							.allocateDirect(w * h * 4)
							.order(java.nio.ByteOrder.nativeOrder());
					softBmp.copyPixelsToBuffer(buf);
					softBmp.recycle();

					for (int i = 0, n = w * h * 4; i < n; i += 4) {
						byte b = buf.get(i);
						buf.put(i,     buf.get(i + 2)); // R → position 0
						buf.put(i + 2, b);              // B → position 2
						// G (i+1) and A (i+3) are already correct
					}
					buf.rewind();
					_lastRgbaWidth  = w;
					_lastRgbaHeight = h;
					return buf;
				} catch (Exception e) {
					if (IS_DEBUG) android.util.Log.d("JS", "getRgbaBuffer HW: " + e);
				}
			} else {
				// API < 33: RGBA_8888 plane → direct buffer, strip row padding
				try (Image image = _imageReader.acquireLatestImage()) {
					if (image == null) return null;
					Image.Plane plane   = image.getPlanes()[0];
					java.nio.ByteBuffer srcBuf    = plane.getBuffer();
					int w           = image.getWidth();
					int h           = image.getHeight();
					int pixelStride = plane.getPixelStride(); // typically 4 for RGBA_8888
					int rowStride   = plane.getRowStride();   // may include alignment padding
					int rowBytes    = w * 4;

					java.nio.ByteBuffer dst = java.nio.ByteBuffer.allocateDirect(rowBytes * h);
					if (pixelStride == 4 && rowStride == rowBytes) {
						// No padding — single bulk copy
						srcBuf.rewind();
						dst.put(srcBuf);
					} else {
						// Padding present — strip it row by row using absolute-position reads,
						// which work correctly when writing into a direct ByteBuffer on older APIs.
						for (int row = 0; row < h; row++) {
							for (int col = 0; col < w; col++) {
								int srcOff = row * rowStride + col * pixelStride;
								dst.put(srcBuf.get(srcOff));
								dst.put(srcBuf.get(srcOff + 1));
								dst.put(srcBuf.get(srcOff + 2));
								dst.put(srcBuf.get(srcOff + 3));
							}
						}
					}
					dst.rewind();
					_lastRgbaWidth  = w;
					_lastRgbaHeight = h;
					return dst;
				} catch (Exception e) {
					if (IS_DEBUG) android.util.Log.d("JS", "getRgbaBuffer SW: " + e);
				}
			}
		}

		View surfaceView = this._playerView.getVideoSurfaceView();
		if (surfaceView instanceof TextureView && ((TextureView) surfaceView).isAvailable()) {
			TextureView tv = (TextureView) surfaceView;
			int w = _videoWidth  > 0 ? _videoWidth  : tv.getWidth();
			int h = _videoHeight > 0 ? _videoHeight : tv.getHeight();
			if (w <= 0 || h <= 0) return null;
			android.graphics.Bitmap bmp = android.graphics.Bitmap.createBitmap(
					w, h, android.graphics.Bitmap.Config.ARGB_8888);
			tv.getBitmap(bmp);

			java.nio.ByteBuffer buf = java.nio.ByteBuffer
					.allocateDirect(w * h * 4)
					.order(java.nio.ByteOrder.nativeOrder());
			bmp.copyPixelsToBuffer(buf);
			bmp.recycle();

			// BGRA → RGBA
			for (int i = 0, n = w * h * 4; i < n; i += 4) {
				byte b = buf.get(i);
				buf.put(i,     buf.get(i + 2));
				buf.put(i + 2, b);
			}
			buf.rewind();
			_lastRgbaWidth  = w;
			_lastRgbaHeight = h;
			return buf;
		}

		return null;
	}

	public int getLastRgbaWidth()  { return _lastRgbaWidth; }
	public int getLastRgbaHeight() { return _lastRgbaHeight; }

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

	public boolean drawVideoFrame2D(int backendType, long context, float dx, float dy) {
		if (context == 0) return false;
		// Ensure the player surface is set up even before dimensions are known.
		// On some devices onVideoSizeChanged only fires after the first rendered frame,
		// so we must give ExoPlayer a surface first to break the deadlock.
		if (backendType == 2) ensureGL2DSurface(context);
		float w = (float) _videoWidth;
		float h = (float) _videoHeight;
		if (w <= 0 || h <= 0) return false;
		if (backendType == 2) {
			return drawFrame2DGL(context, 0, 0, w, h, dx, dy, w, h);
		}
		return drawFrame2DBitmap(context, 0, 0, w, h, dx, dy, w, h);
	}

	public boolean drawVideoFrame2D(int backendType, long context, float dx, float dy, float dw, float dh) {
		if (context == 0) return false;
		if (backendType == 2) ensureGL2DSurface(context);
		float w = (float) _videoWidth;
		float h = (float) _videoHeight;
		if (w <= 0 || h <= 0) return false;
		if (backendType == 2) {
			return drawFrame2DGL(context, 0, 0, w, h, dx, dy, dw, dh);
		}
		return drawFrame2DBitmap(context, 0, 0, w, h, dx, dy, dw, dh);
	}

	public boolean drawVideoFrame2D(int backendType, long context, float sx, float sy, float sw, float sh, float dx, float dy, float dw, float dh) {
		if (context == 0) return false;
		if (backendType == 2) ensureGL2DSurface(context);
		if (_videoWidth <= 0 || _videoHeight <= 0) return false;
		if (backendType == 2) {
			return drawFrame2DGL(context, sx, sy, sw, sh, dx, dy, dw, dh);
		}
		return drawFrame2DBitmap(context, sx, sy, sw, sh, dx, dy, dw, dh);
	}


	private void ensureGL2DSurface(long context) {
		if (_glInitFailed || _glSt != null) return;
		if (IS_DEBUG) android.util.Log.d("JS", "ensureGL2DSurface: context=" + context);
		Object[] result = _utils.create2DContextSurfaceTexture(context);
		if (result == null) {
			if (IS_DEBUG) android.util.Log.d("JS", "ensureGL2DSurface: create2DContextSurfaceTexture returned null → bitmap fallback");
			_glInitFailed = true;
			return;
		}
		if (IS_DEBUG) android.util.Log.d("JS", "ensureGL2DSurface: created ok, textureId=" + result[1]);
		_glSt = (SurfaceTexture) result[0];
		_glTextureId = (Integer) result[1];
		_glRender = result[2];
		_glSt.setOnFrameAvailableListener(st -> {
			_glHasFrame = true;
			if (_callback != null) _callback.onVideoFrame();
		});
		_glSurface = new Surface(_glSt);
		if (_surfaceOwner == SurfaceOwner.BITMAP) {
			if (_imageReader != null) { _imageReader.close(); _imageReader = null; }
			_surfaceOwner = SurfaceOwner.NONE;
		}
		if (_surfaceOwner != SurfaceOwner.WEBGL) {
			_player.setVideoSurface(_glSurface);
			_surfaceOwner = SurfaceOwner.GL2D;
		}
	}

	private boolean drawFrame2DGL(long context, float sx, float sy, float sw, float sh, float dx, float dy, float dw, float dh) {
		// Permanently fall back to CPU path if GL init previously failed.
		if (_glInitFailed) {
			return drawFrame2DBitmap(context, sx, sy, sw, sh, dx, dy, dw, dh);
		}

		// Surface should already be set up by ensureGL2DSurface() called from
		// drawVideoFrame2D. If it somehow isn't (e.g. called directly), init now.
		if (_glSt == null) {
			ensureGL2DSurface(context);
			if (_glSt == null) {
				return drawFrame2DBitmap(context, sx, sy, sw, sh, dx, dy, dw, dh);
			}
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
			return true;
		}

		// drawVideoFrameGL failed with a live frame (Skia OES path not supported —
		// common on emulators and some Vulkan-only devices). Permanently fall back to
		// the CPU bitmap path via ImageReader so subsequent frames actually render.
		if (IS_DEBUG) android.util.Log.d("JS", "drawVideoFrameGL: OES draw failed, switching to bitmap path");
		_glInitFailed = true;
		_glHasFrame = false;
		// Release the GL surface so the player can be redirected to ImageReader.
		if (_glSurface != null) { _glSurface.release(); _glSurface = null; }
		if (_glSt != null) { _glSt.release(); _glSt = null; }
		_surfaceOwner = SurfaceOwner.NONE;
		// Set up ImageReader and redirect the player.
		setupBitmapSurface();
		// First frame after transition may be null (decoder just switched surfaces).
		return drawFrame2DBitmap(context, sx, sy, sw, sh, dx, dy, dw, dh);
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

	public void setSrc(String value) {
		this._src = value;
		this._loadedDataFired.set(false);
		this._canPlayFired = false;
		this._canPlayThroughFired = false;
		// Video dimensions will change — release the ImageReader so setupBitmapSurface()
		// recreates it at the new size once onVideoSizeChanged fires.
		if (_surfaceOwner == SurfaceOwner.BITMAP) {
			if (_imageReader != null) { _imageReader.close(); _imageReader = null; }
			_surfaceOwner = SurfaceOwner.NONE;
		}
		this._setSrc(value);
	}

	public void load() {
		try {
			this.pause();

			if (_timer != null) { _timer.cancel(); _timer.purge(); _timer = null; }
			if (_frameTimer != null) { _frameTimer.cancel(); _frameTimer.purge(); _frameTimer = null; }

			if (_glSurface != null) { _glSurface.release(); _glSurface = null; }
			if (_glSt != null) { _glSt.release(); _glSt = null; }

			if (_imageReader != null) { try { _imageReader.close(); } catch (Exception ignored) {} _imageReader = null; }

			_surfaceOwner = SurfaceOwner.NONE;
			_hasFrame = false;
			_glHasFrame = false;
			_loadedDataFired.set(false);

			if (this._src != null && !this._src.isEmpty()) {
				this._setSrc(this._src);
			}
		} catch (Exception e) {
			if (IS_DEBUG) android.util.Log.d("JS", "VideoHelper.load() error: " + e);
		}
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
		
		if (t.contains("mp4") || t.contains("mpeg") || t.contains("h264") || t.contains("video/mp4")) return "probably";
		if (t.contains("webm") || t.contains("vp8") || t.contains("vp9") || t.contains("video/webm")) return "maybe";
		if (t.contains("ogg") || t.contains("theora") || t.contains("video/ogg")) return "maybe";
		return "";
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
	public void  onPlaybackStateChanged(@Player.State int playbackState) {
		if (playbackState == Player.STATE_READY) {
			this._readyState = 2; // HAVE_CURRENT_DATA
			if (!this._canPlayFired) {
				this._canPlayFired = true;
				if (this._callback != null) {
					this._callback.onCanPlay();
				}
			}

			if (_loadedDataFired.compareAndSet(false, true)) {
				if (this._callback != null) {
					this._callback.onLoadedData();
				}
			}

			long duration = this._player.getDuration();
			long buffered = this._player.getBufferedPosition();
			if (duration > 0 && buffered >= duration - 1000 && !this._canPlayThroughFired) {
				this._canPlayThroughFired = true;
				if (this._callback != null) {
					this._callback.onCanPlayThrough();
				}
			}
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
	public void  onRenderedFirstFrame() {
		if (_loadedDataFired.compareAndSet(false, true)) {
			if (this._callback != null) {
				this._callback.onLoadedData();
			}
		}
	}

	
	@Deprecated
	@UnstableApi
	@Override
	public void  onCues(List<Cue> cues) {}
	
	@Override
	public void  onCues(CueGroup cueGroup) {}


	@UnstableApi
	public void onMetadata(Metadata metadata) {}
}
