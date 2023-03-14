import { VideoBase, controlsProperty, autoplayProperty, loopProperty, srcProperty, currentTimeProperty, durationProperty } from './common';
import { Screen, Application, Utils, knownFolders, path } from '@nativescript/core';
import { Source } from '..';
import { isString } from '@nativescript/core/utils/types';
declare var com;
const STATE_IDLE: number = 1;
const STATE_BUFFERING: number = 2;
const STATE_READY: number = 3;
const STATE_ENDED: number = 4;
const MATCH_PARENT = 0xffffffff;
const TYPE = { DETECT: 0, SS: 1, DASH: 2, HLS: 3, OTHER: 4 };
declare const org;
export class Video extends VideoBase {
	_container: android.widget.LinearLayout;
	_sourceView: Source[] = [];
	_playerView: com.google.android.exoplayer2.ui.PlayerView;
	_player: com.google.android.exoplayer2.SimpleExoPlayer;
	_playerListener; //com.google.android.exoplayer2.Player.Listener;
	_src: string;
	_autoplay: boolean;
	_loop: boolean;
	_textureView: android.view.TextureView;

	private static _cache: com.google.android.exoplayer2.upstream.cache.SimpleCache;
	private static _leastRecentlyUsedCacheEvictor: com.google.android.exoplayer2.upstream.cache.LeastRecentlyUsedCacheEvictor;
	private static _exoDatabaseProvider; // com.google.android.exoplayer2.database.ExoDatabaseProvider;
	private static _exoPlayerCacheSize = 100 * 1024 * 1024;
	private static _dsf: com.google.android.exoplayer2.upstream.DefaultDataSourceFactory;
	private static _msf: com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
	private static _cdsf: com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory;
	_isCustom: boolean = false;
	_playing: boolean = false;
	_timer: any;
	_st: android.graphics.SurfaceTexture;
	_surface: android.view.Surface;
	_render: any;
	_frameListener: any;
	_canvas: any;
	_frameTimer: any;
	_readyState: number = 0;
	_videoWidth = 0;
	_videoHeight = 0;
	private static _didInit = false;
	static BUFFER_MS = 500;
	static IS_DEBUG = false;
	_instance;
	constructor() {
		super();
		try {
			java.lang.System.loadLibrary('canvasnative');
		} catch (ex) {}

		const activity: androidx.appcompat.app.AppCompatActivity = Application.android.foregroundActivity || Application.android.startActivity;

		const cacheDir = new java.io.File(path.join(knownFolders.documents().path, 'MEDIA_PLAYER_CACHE'));

		this._instance = new org.nativescript.canvas.media.VideoHelper(activity, knownFolders.documents().path);

		const ref = new WeakRef(this);

		this._instance.setCallback(
			new org.nativescript.canvas.media.VideoHelper.Callback({
				onDurationChange(duration) {
					const owner = ref.get();
					if (owner) {
						durationProperty.nativeValueChange(owner, duration);
						owner._notifyListener(Video.durationchangeEvent);
					}
				},

				onPlaying() {
					const owner = ref.get();
					if (owner) {
						owner._notifyListener(Video.playingEvent);
					}
				},

				onVideoSizeChanged(width, height) {
					const owner = ref.get();
					if (owner) {
						owner._videoWidth = width;
						owner._videoHeight = height;
					}
				},

				onCurrentTimeChanged(time) {
					const owner = ref.get();
					if (owner) {
						currentTimeProperty.nativeValueChange(owner, time);
						owner._notifyListener(Video.timeupdateEvent);
					}
				},

				onVideoFrame() {
					const owner = ref.get();
					if (owner) {
						owner._notifyVideoFrameCallbacks();
					}
				},
			})
		);

		/*
		if (!Video._didInit) {
			const packageName = activity.getPackageName();
			const cacheDir = new java.io.File(path.join(knownFolders.documents().path, 'MEDIA_PLAYER_CACHE'));
			if (!cacheDir.exists()) {
				cacheDir.mkdirs();
			}

			Video._leastRecentlyUsedCacheEvictor = new com.google.android.exoplayer2.upstream.cache.LeastRecentlyUsedCacheEvictor(Video._exoPlayerCacheSize);

			Video._exoDatabaseProvider = new com.google.android.exoplayer2.database.ExoDatabaseProvider(activity);
			Video._cache = new com.google.android.exoplayer2.upstream.cache.SimpleCache(cacheDir, Video._leastRecentlyUsedCacheEvictor, Video._exoDatabaseProvider);
			Video._dsf = new com.google.android.exoplayer2.upstream.DefaultDataSourceFactory(activity, com.google.android.exoplayer2.util.Util.getUserAgent(activity, packageName));
			Video._cdsf = new com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory();
			Video._cdsf.setCache(Video._cache);
			Video._cdsf.setUpstreamDataSourceFactory(Video._dsf);
			Video._msf = new com.google.android.exoplayer2.source.DefaultMediaSourceFactory(Video._cdsf);
			Video._didInit = true;
		}
		const builder = new com.google.android.exoplayer2.SimpleExoPlayer.Builder(activity);
		builder.setMediaSourceFactory(Video._msf);
		const loadControl = new com.google.android.exoplayer2.DefaultLoadControl.Builder();
		loadControl.setBufferDurationsMs(Video.BUFFER_MS, com.google.android.exoplayer2.DefaultLoadControl.DEFAULT_MAX_BUFFER_MS, Video.BUFFER_MS, Video.BUFFER_MS);
		builder.setLoadControl(loadControl.build());
		this._player = builder.build();
		const ref = new WeakRef(this);
		this._playerListener = new com.google.android.exoplayer2.Player.Listener({
			onIsPlayingChanged: function (isPlaying) {
				const owner = ref.get();
				if (owner) {
					const duration = owner._player.getDuration();
					if (owner.duration != duration) {
						durationProperty.nativeValueChange(owner, duration);
						owner._notifyListener(Video.durationchangeEvent);
					}
					owner._playing = isPlaying;
					if (isPlaying) {
						owner._notifyListener(Video.playingEvent);
						owner._notifyVideoFrameCallbacks();
						if (owner._timer) {
							clearInterval(owner._timer);
						}
						if (owner._frameTimer) {
							clearInterval(owner._frameTimer);
						}

						owner._timer = setInterval(function () {
							const current = owner._player.getCurrentPosition();
							if (current != owner.currentTime) {
								currentTimeProperty.nativeValueChange(owner, current);
								owner._notifyListener(Video.timeupdateEvent);
								owner._notifyVideoFrameCallbacks();
							}
						}, 1000);
					} else {
						clearInterval(owner._timer);
						clearInterval(owner._frameTimer);
						currentTimeProperty.nativeValueChange(owner, owner._player.getCurrentPosition());
					}
				}
			},
			onLoadingChanged: function (isLoading) {},
			onPlaybackParametersChanged: function (playbackParameters) {},
			onPlaybackSuppressionReasonChanged: function (playbackSuppressionReason) {},
			onPlayerError: function (error) {},
			onPlayerStateChanged: function (playWhenReady, playbackState) {},
			onPositionDiscontinuity: function (reason) {},
			onRepeatModeChanged: function (repeatMode) {},
			onSeekProcessed: function () {},
			onShuffleModeEnabledChanged: function (shuffleModeEnabled) {},
			//@ts-ignore
			onTimelineChanged(timeline: com.google.android.exoplayer2.Timeline, manifest: any, reason: number) {},
			onTracksChanged: function (trackGroups, trackSelections) {},

			onPlayWhenReadyChanged(playWhenReady: boolean, reason: number) {},
			onEvents(player, events) {},
			onMediaItemTransition(mediaItem, reason: number) {},
			onAvailableCommandsChanged(availableCommands) {},
			onPlaybackStateChanged(playbackState: number) {},
			onIsLoadingChanged(isLoading: boolean) {},
			onPlayerErrorChanged(error) {
				if (Video.IS_DEBUG) {
					console.log(error);
				}
			},
			onTracksInfoChanged(tracksInfo) {},
			onSurfaceSizeChanged(width: number, height: number) {},
			onRenderedFirstFrame() {},
			onDeviceVolumeChanged(volume: number, muted: boolean) {},
			onVideoSizeChanged(videoSize) {
				const owner = ref.get();
				if (owner) {
					owner._videoWidth = videoSize.width;
					owner._videoHeight = videoSize.height;
				}
			},
		});
		const inflator = activity.getLayoutInflater();
		const layout = Video.getResourceId(Application.android.foregroundActivity || Application.android.startActivity, 'player');
		this._player.addListener(this._playerListener);
		this._playerView = inflator.inflate(layout, null, false) as any; //new com.google.android.exoplayer2.ui.PlayerView(Application.android.foregroundActivity || Application.android.startActivity);
		this._container = new android.widget.LinearLayout(Application.android.foregroundActivity || Application.android.startActivity);
		const params = new android.widget.LinearLayout.LayoutParams(MATCH_PARENT, MATCH_PARENT);
		this._textureView = new android.view.TextureView(Application.android.foregroundActivity || Application.android.startActivity);
		this._container.addView(this._textureView as any, params);
		this.setNativeView(this._container);
		*/
		this.setNativeView(this._instance.getContainer());
	}

	get readyState() {
		return this._readyState;
	}

	_setSurface(surface) {
		this._player.setVideoSurface(surface);
	}

	private static getResourceId(context: any, res: string = '') {
		if (!context) return 0;
		if (isString(res)) {
			const packageName = context.getPackageName();
			try {
				const className = java.lang.Class.forName(`${packageName}.R$layout`);
				return parseInt(String(className.getDeclaredField(res).get(null)));
			} catch (e) {
				return 0;
			}
		}
		return 0;
	}
	static st_count = 0;
	static createCustomView() {
		const video = new Video();
		video._isCustom = true;
		video.width = 300;
		video.height = 150;
		return video;
	}

	// private _setSrc(value: string) {
	// 	try {
	// 		if (typeof value === 'string' && value.startsWith('~/')) {
	// 			value = path.join(knownFolders.currentApp().path, value.replace('~', ''));
	// 		}
	// 		this._player.setMediaItems(java.util.Arrays.asList([com.google.android.exoplayer2.MediaItem.fromUri(android.net.Uri.parse(value))]), true);
	// 		this._player.prepare();
	// 		if (this._autoplay) {
	// 			this._player.setPlayWhenReady(true);
	// 		}
	// 	} catch (e) {
	// 		if (Video.IS_DEBUG) {
	// 			console.log(e);
	// 		}
	// 	}
	// }

	_hasFrame = false;
	getCurrentFrame(context?: WebGLRenderingContext) {
		const ctx = arguments[1] as any;
		const flipY = ctx._flipY;
		const ptr = ctx._canvas._canvas.getNativeGL();

		this._instance.getCurrentFrame(!!this.isLoaded, ptr, flipY, arguments[4], arguments[5]);

		/*	if (this.isLoaded) {
			const surfaceView = this._playerView.getVideoSurfaceView();
			if (surfaceView instanceof android.view.TextureView) {
				const st = surfaceView.getSurfaceTexture();
				if (st) {
					// @ts-ignore
					this._render = org.nativescript.canvas.Utils.createRenderAndAttachToGLContext(ptr, st);
					this._st = st;
				}
			}
		}


		try {
			if (!this._st) {
				// @ts-ignore
				const result = org.nativescript.canvas.Utils.createSurfaceTexture(ptr);
				console.log(result);
				this._st = result[0];
				const ref = new WeakRef(this);
				this._frameListener = new android.graphics.SurfaceTexture.OnFrameAvailableListener({
					onFrameAvailable(param0: android.graphics.SurfaceTexture) {
						const owner = ref.get();
						if (owner) {
							owner._hasFrame = true;
							owner._notifyVideoFrameCallbacks();
						}
					},
				});
	
				this._st.setOnFrameAvailableListener(this._frameListener);
	
				this._surface = new android.view.Surface(this._st);
				this._player.setVideoSurface(this._surface);
				this._render = result[1];
			}
	
			if (this._st) {
				if (!this._hasFrame) {
					return;
				}
				// @ts-ignore
				console.log(ptr, flipY, this._st, this._render, this._videoWidth, this._videoHeight, arguments[4], arguments[5]);
	
				org.nativescript.canvas.Utils.updateTexImage(ptr, flipY, this._st, this._render, this._videoWidth, this._videoHeight, arguments[4], arguments[5]);
				this._hasFrame = false;
			}
		} catch (error) {
			console.log(error);
		}
		*/
	}

	play() {
		this._instance.play();
	}

	pause() {
		this._instance.pause();
	}

	get muted() {
		return this._instance.getMuted();
	}

	set muted(value: boolean) {
		this._instance.setMuted(value);
	}

	get duration() {
		return this._instance.getDuration();
	}

	get currentTime() {
		return this._instance.getCurrentTime();
	}

	set currentTime(value: number) {
		this._instance.setCurrentTime(value);
	}

	get src() {
		return this._instance.getSrc();
	}

	set src(value: string) {
		if (typeof value === 'string' && value.startsWith('~/')) {
			value = path.join(knownFolders.currentApp().path, value.replace('~', ''));
		}
		this._instance.setSrc(value);
	}

	//@ts-ignore
	get autoplay() {
		return this._instance.getAutoplay();
	}

	set autoplay(value: boolean) {
		this._instance.setAutoplay(value);
	}

	get controls() {
		return this._instance.getControls();
	}

	set controls(enabled: boolean) {
		this._instance.setControls(enabled);
	}

	// @ts-ignore
	get loop() {
		return this._instance.getLoop();
	}

	set loop(value: boolean) {
		this._instance.setLoop(value);
	}

	createNativeView(): Object {
		if (!this._playerView) {
			this._playerView = new com.google.android.exoplayer2.ui.PlayerView(this._context);
		}
		this._playerView.setPlayer(this._player);
		return this._playerView;
	}

	initNativeView() {
		super.initNativeView();
	}

	_addChildFromBuilder(name: string, value: any) {
		if (value instanceof Source) {
			this._sourceView.push(value);
		}
	}

	onLoaded() {
		super.onLoaded();
		if (this._sourceView.length > 0) {
			this.src = this._sourceView[0].src;
		}
	}

	// @ts-ignore
	get width() {
		if (this.getMeasuredWidth() > 0) {
			return this.getMeasuredWidth();
		}
		return this._instance.getWidth();
	}

	set width(value) {
		this.style.width = value;
		if (this._isCustom) {
			this._layoutNative();
		}
	}

	// @ts-ignore
	get height() {
		if (this.getMeasuredHeight() > 0) {
			return this.getMeasuredHeight();
		}
		return this._instance.getHeight();
	}

	set height(value) {
		this.style.height = value;
		if (this._isCustom) {
			this._layoutNative();
		}
	}

	_layoutNative() {
		if (!this.parent) {
			//@ts-ignore
			if ((typeof this.width === 'string' && this.width.indexOf('%')) || (typeof this.height === 'string' && this.height.indexOf('%'))) {
				return;
			}
			if (!this._isCustom) {
				return;
			}

			const size = this._realSize;
			size.width = size.width * Screen.mainScreen.scale;
			size.height = size.height * Screen.mainScreen.scale;
			this._instance.layoutNative(size.width, size.height);
			/*	//@ts-ignore
			let rootParams = this._container.getLayoutParams();

			if (rootParams && (size.width || 0) === rootParams.width && (size.height || 0) === rootParams.height) {
				return;
			}

			if ((size.width || 0) !== 0 && (size.height || 0) !== 0) {
				if (!rootParams) {
					rootParams = new android.widget.FrameLayout.LayoutParams(0, 0);
				}
				rootParams.width = size.width;
				rootParams.height = size.height;
				let surfaceParams; // = this._canvas.getSurface().getLayoutParams();
				if (!surfaceParams) {
					surfaceParams = new android.widget.FrameLayout.LayoutParams(0, 0);
				}
				//	surfaceParams.width = size.width;
				//	surfaceParams.height = size.height;

				//@ts-ignore
				this._container.setLayoutParams(rootParams);

				const w = android.view.View.MeasureSpec.makeMeasureSpec(size.width, android.view.View.MeasureSpec.EXACTLY);
				const h = android.view.View.MeasureSpec.makeMeasureSpec(size.height, android.view.View.MeasureSpec.EXACTLY);
				//@ts-ignore
				this._container.measure(w, h);
				//@ts-ignore
				this._container.layout(0, 0, size.width || 0, size.height || 0);

				if (this._st) {
					this._st.setDefaultBufferSize(size.width || 0, size.height || 0);
				}
			}
			*/
		}
	}
}
