import { controlsProperty, VideoBase, playsinlineProperty, mutedProperty, srcProperty, currentTimeProperty } from './common';
import { Source } from '../common';
import { knownFolders, path } from '@nativescript/core';
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
					console.log('ready');
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
	_previousTS: any;
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
				const currentTime = this.#player.currentTime();
				if (!this._assetOutput.hasNewPixelBufferForItemTime(currentTime)) {
					return;
				}
				const sampleBuffer = this._assetOutput.copyPixelBufferForItemTimeItemTimeForDisplay(currentTime, null);
				if (sampleBuffer !== 0) {
					/*	const currentSampleTime = CMSampleBufferGetOutputPresentationTimeStamp(sampleBuffer);
								const differenceFromLastFrame = CMTimeSubtract(currentSampleTime, this['previousFrameTime']);
								const currentActualTime = CFAbsoluteTimeGetCurrent();
	
								const frameTimeDifference = CMTimeGetSeconds(differenceFromLastFrame);
								const actualTimeDifference = currentActualTime - this['previousActualFrameTime'];
								
			
								if (frameTimeDifference > actualTimeDifference) {
									usleep(1000000.0 * (frameTimeDifference - actualTimeDifference));
								}
	
								this['previousFrameTime'] = currentSampleTime;
								this['previousActualFrameTime'] = CFAbsoluteTimeGetCurrent();
								*/

					//const pixel_buffer = CMSampleBufferGetImageBuffer(sampleBuffer);

					//const startTime = CFAbsoluteTimeGetCurrent();

					const GL_TEXTURE_2D = 3553;
					const GL_RGBA = 6408;
					const GL_BGRA_EXT = 32993;
					const GL_UNSIGNED_BYTE = 5121;

					const pixel_buffer = sampleBuffer; //CMSampleBufferGetImageBuffer(sampleBuffer);

					//const startTime = CFAbsoluteTimeGetCurrent();
					CVPixelBufferLockBaseAddress(pixel_buffer, 0);
					//const bpr = CVPixelBufferGetBytesPerRow(pixel_buffer);
					const width = CVPixelBufferGetWidth(pixel_buffer);
					const height = CVPixelBufferGetHeight(pixel_buffer);
					const line_base = CVPixelBufferGetBaseAddress(pixel_buffer);
					glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_BGRA_EXT, GL_UNSIGNED_BYTE, line_base);

					CVPixelBufferUnlockBaseAddress(pixel_buffer, 0);
					// CMSampleBufferInvalidate(sampleBuffer);

					/*const currentFrameTime = CFAbsoluteTimeGetCurrent() - startTime;
							console.log('fps', currentFrameTime * 1000);
							*/
					//this._previousTS = currentTS;
					// may not need to release
					/*
								 Unlike regular Core Foundation objects, toll-free bridged types are automatically memory managed by NativeScript,
								  so there is no need to retain or release them using CFRetain and CFRelease.
								*/
					// https://docs.nativescript.org/core-concepts/ios-runtime/marshalling-overview#corefoundation-objects
					//CFRelease(sampleBuffer);
				}
			} catch (e) {
				console.log('getCurrentFrame error:', e);
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
		this.#src = value;
		this._loadSrc(value);
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
				const fps = this._asset.tracks.firstObject?.nominalFrameRate ?? 30;

				const _interval = CMTimeMake(1, fps);
				this._playbackFramesObserver = this.#player.addPeriodicTimeObserverForIntervalQueueUsingBlock(_interval, null, (currentTime) => {
					if (this._isPlaying) {
						this._notifyVideoFrameCallbacks();
					}
				});

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
				this._previousTS = undefined;
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
