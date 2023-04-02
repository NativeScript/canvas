import { controlsProperty, VideoBase, playsinlineProperty, mutedProperty, srcProperty, currentTimeProperty } from './common';
import { Source } from '../common';
import { knownFolders, path } from '@nativescript/core';
declare const Utils;
@NativeClass()
class NativeObject extends NSObject {
	_owner: WeakRef<Video>;
	public static initWithOwner(owner: WeakRef<Video>): NativeObject {
		const obj = NativeObject.alloc().init() as NativeObject;
		obj._owner = owner;
		return obj;
	}

	observeValueForKeyPathOfObjectChangeContext(path: string, obj: Object, change: NSDictionary<any, any>, context: any) {
		if (path === 'status') {
			const owner = this._owner.get();
			if (owner) {
				if (owner._player.currentItem.status === AVPlayerItemStatus.Failed) {
					const baseError = owner._player.currentItem.error.userInfo.objectForKey(NSUnderlyingErrorKey);
					const error = new Error();

					/*error: {
					code: baseError.code,
					domain: baseError.domain
				  },
				  stack: error.stack

				*/
				} else if (owner._player.currentItem.status === AVPlayerItemStatus.ReadyToPlay) {
					if (!owner._videoSize) {
						owner._videoSize = owner._asset.tracksWithMediaType(AVMediaTypeVideo)?.objectAtIndex(0).naturalSize ?? undefined;
					}
				}
			}
		} else if (path === 'loadedTimeRanges') {
			const playerItem = obj as AVPlayerItem;
			if (playerItem) {
				const ranges = playerItem.loadedTimeRanges;
				const first = ranges?.firstObject as NSValue;
				if (first) {
					let loaded = first.CMTimeRangeValue.duration.value / first.CMTimeRangeValue.duration.timescale;
					let total = playerItem.duration.value / playerItem.duration.timescale;
					//	console.log('loaded', loaded / total);
				}
			}
		}
	}

	dealloc() {
		// using this to clean up listeners
		const owner = this._owner.get();
		if (owner) {
			if (owner._playEndNotificationId) {
				NSNotificationCenter.defaultCenter.removeObserver(owner._playEndNotificationId);
				owner._playEndNotificationId = undefined;
				NSNotificationCenter.defaultCenter.removeObserver(owner._resumeListenerId);
				owner._resumeListenerId = undefined;
				NSNotificationCenter.defaultCenter.removeObserver(owner._suspendListenerId);
				owner._suspendListenerId = undefined;
				owner._destroy();
			}
		}
	}
}

export class Video extends VideoBase {
	#videoCtrl: AVPlayerViewController;
	#player: AVPlayer;
	#sourceView: Source[];
	#protocols: NativeObject;
	_isCustom: boolean = false;
	_readyState: number = 0;
	_resumeListenerId: any;
	_suspendListenerId: any;
	_isInForground = true;
	_assetOutput: AVPlayerItemVideoOutput;
	_isPlaying = false;
	#src: string;
	#muted: boolean;
	#controls: boolean;
	_playEndNotificationId: any;
	_playbackTimeObserver: any;
	_playbackFramesObserver: any;
	_currentUrl: string;
	_fps: number;
	_ctx: any;
	_asset: AVURLAsset;
	_videoSize: any;
	_render: any;
	static IS_DEBUG = false;
	get _player() {
		return this.#player;
	}
	constructor() {
		super();
		this.#videoCtrl = AVPlayerViewController.new();
		try {
			AVAudioSession.sharedInstance().setCategoryError(AVAudioSessionCategoryPlayback);
		} catch (e) {}
		this.#player = AVPlayer.new();
		this.#videoCtrl.player = this.#player;
		this.#sourceView = [];
		this.#protocols = NativeObject.initWithOwner(new WeakRef(this));
		this.setNativeView(this.#videoCtrl.view);
		this._suspendListenerId = NSNotificationCenter.defaultCenter.addObserverForNameObjectQueueUsingBlock(UIApplicationDidEnterBackgroundNotification, null, null, (noti) => {
			this._isInForground = false;
		});

		this._resumeListenerId = NSNotificationCenter.defaultCenter.addObserverForNameObjectQueueUsingBlock(UIApplicationDidBecomeActiveNotification, null, null, (noti) => {
			this._isInForground = true;
		});
	}

	static createCustomView() {
		const video = new Video();
		video._isCustom = true;
		video.width = 300;
		video.height = 150;
		return video;
	}

	//@ts-ignore
	get readyState() {
		return this._readyState;
	}

	getCurrentFrame(context) {
		if (!this._isInForground) {
			return;
		}
		if (this._assetOutput) {
			try {
				if (!this._render) {
					this._render = Utils.setupRender();
					this._render.createSurface();
				}
				Utils.drawFrame(this.#player, this._assetOutput, this._videoSize, this._render, arguments[4], arguments[5], false);
			} catch (e) {
				if (Video.IS_DEBUG) {
					console.log('getCurrentFrame error:', e);
				}
			}
		}
	}

	createNativeView(): Object {
		const ctrl = this.topViewController;
		if (ctrl) {
			ctrl.addChildViewController(this.#videoCtrl);
			this.#videoCtrl.didMoveToParentViewController(ctrl);
		}
		return this.#videoCtrl.view;
	}

	initNativeView() {
		super.initNativeView();
		this.#videoCtrl.showsPlaybackControls = this.controls;
		this.#videoCtrl.entersFullScreenWhenPlaybackBegins = !this.playsinline;
		this.#player.muted = this.muted;
	}

	[controlsProperty.setNative](enable: boolean) {
		this.#videoCtrl.showsPlaybackControls = enable;
	}

	[playsinlineProperty.setNative](inline: boolean) {
		this.#videoCtrl.entersFullScreenWhenPlaybackBegins = !inline;
	}

	_addChildFromBuilder(name: string, value: any) {
		if (value instanceof Source) {
			this.#sourceView.push(value);
		}
	}

	onLoaded() {
		super.onLoaded();
		const item = this.#sourceView.filter((item) => {
			if (item.type.indexOf('video/webm') === -1) {
				return item;
			}
		})[0];
		if (item) {
			this._loadSrc(item.src);
		}
	}

	get duration() {
		const currentItem = this.#player.currentItem;
		if (currentItem) {
			return CMTimeGetSeconds(currentItem.asset.duration);
		}
		return NaN;
	}

	get currentTime() {
		const ct = this.#player.currentTime();
		return ct.value / ct.timescale;
	}

	set currentTime(value: number) {
		const time = CMTimeMakeWithSeconds(value, this.#player.currentTime().timescale);
		this.#player.seekToTime(time);
	}

	get muted() {
		return this.#muted;
	}

	set muted(value: boolean) {
		this.#muted = value;
		this.#player.muted = value;
	}

	get src() {
		return this.#src;
	}

	set src(value: string) {
		if (value !== this._currentUrl) {
			this.#src = value;
			this._loadSrc(value);
		}
	}

	get controls() {
		return this.#controls;
	}

	set controls(enabled: boolean) {
		this.#controls = enabled;
		this.#videoCtrl.showsPlaybackControls = enabled;
	}

	private _addTimeObserver() {
		const _interval = CMTimeMake(1, 1);
		this._playbackTimeObserver = this.#player.addPeriodicTimeObserverForIntervalQueueUsingBlock(_interval, null, (currentTime) => {
			if (this._isPlaying) {
				const _seconds = CMTimeGetSeconds(currentTime);
				currentTimeProperty.nativeValueChange(this, _seconds);
				this._notifyListener(Video.timeupdateEvent);
			}
		});
	}

	private _loadSrc(value: string) {
		try {
			const src = value;
			let url;
			if (typeof value === 'string' && value.startsWith('~/')) {
				value = path.join(knownFolders.currentApp().path, value.replace('~', ''));
			}

			if (typeof value === 'string' && value.startsWith('/')) {
				url = NSURL.fileURLWithPath(value);
			}

			if (this._playEndNotificationId) {
				NSNotificationCenter.defaultCenter.removeObserver(this._playEndNotificationId);
				this._playEndNotificationId = undefined;
			}
			if (!url) {
				url = NSURL.URLWithString(value);
			}
			this._asset = AVURLAsset.assetWithURL(url);
			const keys = ['tracks', 'duration'];
			this._asset.loadValuesAsynchronouslyForKeysCompletionHandler(keys, () => {
				console.dir(this._asset.tracksWithMediaType(AVMediaTypeVideo));
				this._videoSize = this._asset.tracksWithMediaType(AVMediaTypeVideo)?.objectAtIndex(0).naturalSize ?? undefined;

				const fps = this._asset.tracks.firstObject?.nominalFrameRate ?? 30;

				const _interval = CMTimeMake(1, fps);
				this._playbackFramesObserver = this.#player.addPeriodicTimeObserverForIntervalQueueUsingBlock(_interval, null, (currentTime) => {
					if (this._isPlaying) {
						this._notifyVideoFrameCallbacks();
					}
				});
				this._render?.destroy();
				this._render = undefined;
				const item = AVPlayerItem.alloc().initWithAsset(this._asset);
				const settings: any = {};
				settings[kCVPixelBufferPixelFormatTypeKey] = NSNumber.numberWithUnsignedInt(kCVPixelFormatType_32BGRA);
				this._assetOutput = AVPlayerItemVideoOutput.alloc().initWithOutputSettings(settings);
				item.addOutput(this._assetOutput);
				item.addObserverForKeyPathOptionsContext(this.#protocols, 'status', 0, null);
				item.addObserverForKeyPathOptionsContext(this.#protocols, 'loadedTimeRanges', NSKeyValueObservingOptions.Initial | NSKeyValueObservingOptions.New, null);
				this._playEndNotificationId = NSNotificationCenter.defaultCenter.addObserverForNameObjectQueueUsingBlock(AVPlayerItemDidPlayToEndTimeNotification, item, null, (notfi) => {
					if (this.loop) {
						this.#player.seekToTime(kCMTimeZero);
						this.#player.play();
						this._isPlaying = true;
						this._notifyListener(Video.playingEvent);
					}
				});
				this.#player.replaceCurrentItemWithPlayerItem(item);
				this._readyState = Video.HAVE_METADATA;
				this._currentUrl = src;
				if (this.autoplay) {
					this.play();
				}
			});
		} catch (e) {
			console.log('_loadSrc', e);
		}
	}

	play() {
		if (this._isPlaying) {
			return;
		}
		this._addTimeObserver();
		this.#player.play();

		this._isPlaying = true;
		this._notifyListener(Video.playingEvent);
	}

	pause() {
		if (!this._isPlaying) {
			return;
		}
		if (this._playbackTimeObserver) {
			this.#player.removeTimeObserver(this._playbackTimeObserver);
			this._playbackTimeObserver = undefined;
		}

		/*
		if (this._playbackTimeObserver) {
			this.#player.removeTimeObserver(this._playbackTimeObserver);
			this._playbackTimeObserver = undefined;
		}
		*/

		this.#player.pause();
		this._isPlaying = false;
	}

	_destroy() {
		this.#player.pause();
	}

	private static get rootViewController(): UIViewController | undefined {
		const keyWindow = UIApplication.sharedApplication.keyWindow;
		return keyWindow != null ? keyWindow.rootViewController : undefined;
	}

	private get topViewController(): UIViewController | undefined {
		const root = Video.rootViewController;
		if (root == null) {
			return undefined;
		}
		return this.findTopViewController(root);
	}

	private findTopViewController(root: UIViewController): UIViewController | undefined {
		const presented = root.presentedViewController;
		if (presented != null) {
			return this.findTopViewController(presented);
		}
		if (root instanceof UISplitViewController) {
			const last = root.viewControllers.lastObject;
			if (last == null) {
				return root;
			}
			return this.findTopViewController(last);
		} else if (root instanceof UINavigationController) {
			const top = root.topViewController;
			if (top == null) {
				return root;
			}
			return this.findTopViewController(top);
		} else if (root instanceof UITabBarController) {
			const selected = root.selectedViewController;
			if (selected == null) {
				return root;
			}
			return this.findTopViewController(selected);
		} else {
			return root;
		}
	}
}
