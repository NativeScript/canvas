import { VideoBase } from './common';
import { Source, srcProperty } from '../source';
import { controlsProperty, playsinlineProperty, mutedProperty, currentTimeProperty, loopProperty, autoplayProperty } from '../common';
import { booleanConverter, knownFolders, path } from '@nativescript/core';
declare const NSCCanvasUtils, NSCVideoHelper, NSCRender;

interface NSCVideoHelperListener {}

//@ts-ignore
@ObjCClass(NSCVideoHelperListener)
@NativeClass()
class NSCVideoHelperListenerImpl extends NSObject implements NSCVideoHelperListener {
	_owner: WeakRef<Video>;
	public static initWithOwner(owner: WeakRef<Video>): NSCVideoHelperListenerImpl {
		const obj = NSCVideoHelperListenerImpl.alloc().init() as NSCVideoHelperListenerImpl;
		obj._owner = owner;
		return obj;
	}

	public onStateChange(state) {
		const owner = this._owner.deref();
		if (owner) {
			if (state === 1) {
				if (owner._playResolve) {
					owner._playResolve();
					owner._playResolve = null;
					owner._playReject = null;
					owner._playPromise = null;
				}
				owner._notifyListener(Video.playingEvent);
			}
		}
	}

	public onTimeUpdate(time) {
		const owner = this._owner.deref();
		if (owner) {
			owner._notifyListener(Video.timeupdateEvent);
		}
	}

	public onVideoFrameCallback() {
		const owner = this._owner.deref();
		if (owner) {
			owner._notifyVideoFrameCallbacks();
		}
	}

	public onLoadedData() {
		const owner = this._owner.deref();
		if (owner) {
			owner._notifyListener(Video.durationchangeEvent);
			owner._notifyListener(Video.loadedmetadataEvent);
			owner._notifyListener(Video.loadeddataEvent);
		}
	}

	public onCanPlay() {
		const owner = this._owner.deref();
		if (owner) {
			owner._notifyListener(Video.canplayEvent);
		}
	}

	public onCanPlayThrough() {
		const owner = this._owner.deref();
		if (owner) {
			owner._notifyListener(Video.canplaythroughEvent);
		}
	}
}

export class Video extends VideoBase {
	helper;
	_sourceView: Source[];
	private listener: NSCVideoHelperListenerImpl;
	_isCustom: boolean = false;
	_isInForground = true;
	_ctx: any;
	static IS_DEBUG = false;
	_renderer: NSCRender;
	get _player() {
		return this.helper.player;
	}
	constructor() {
		super();
		this.helper = NSCVideoHelper.new();
		try {
			AVAudioSession.sharedInstance().setCategoryError(AVAudioSessionCategoryPlayback);
		} catch (e) {}
		this._sourceView = [];
		this.listener = NSCVideoHelperListenerImpl.initWithOwner(new WeakRef(this));
		this.helper.listener = this.listener;
		this.setNativeView(this.helper.controller.view);
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
		return this.helper.readyState;
	}

	getCurrentFrame(context) {
		//@ts-ignore
		const flipY = context?.__flipY ?? false;
		if (!this.helper.isInForeground) {
			return;
		}
		if (this.helper.assetOutput) {
			try {
				if (!this._renderer) {
					this._renderer = NSCRender.alloc().init();
				}
				this._renderer.drawFrame(this.helper.player, this.helper.assetOutput, this.helper.videoSize, arguments[4], arguments[5], flipY);
				//	NSCCanvasUtils.drawFrame(this.helper.player, this.helper.assetOutput, this.helper.videoSize, arguments[4], arguments[5], flipY);
			} catch (e) {
				console.error('getCurrentFrame error:', e);
			}
		}
	}

	getFrameForTexImage3D(nativeCtx: any, ctx: any, target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number) {
		//@ts-ignore
		const flipY = nativeCtx?.__flipY ?? false;
		if (!this.helper.isInForeground) {
			return;
		}
		if (this.helper.assetOutput && this.helper.player) {
			if (!this._renderer) {
				this._renderer = NSCRender.alloc().init();
			}
			try {
				//@ts-ignore
				this._renderer.drawFrameTexImage3D(this.helper.player, this.helper.assetOutput, this.helper.videoSize, target, level, internalformat, width, height, depth, border, format, type, flipY);
				return;
			} catch (e) {
				console.error('getFrameForTexImage3D error:', e);
			}
			// Simulator / CPU fallback: read raw BGRA pixel data and upload via texImage3D
			try {
				const frameData = NSCRender.getVideoFrameData(this.helper.player, this.helper.assetOutput, this.helper.videoSize);
				if (frameData) {
					const nsData = frameData.objectForKey('data') as NSData;
					const fw = (frameData.objectForKey('width') as NSNumber).intValue;
					const fh = (frameData.objectForKey('height') as NSNumber).intValue;
					if (nsData && fw > 0 && fh > 0) {
						const pixels = new Uint8Array(interop.bufferFromData(nsData));
						// NSCRender returns BGRA; swap B↔R to get RGBA
						for (let i = 0; i < pixels.length; i += 4) {
							const b = pixels[i];
							pixels[i] = pixels[i + 2];
							pixels[i + 2] = b;
						}
						ctx.native.texImage3D(target, level, internalformat, fw, fh, depth, border, format, type, pixels);
					}
				}
			} catch (fe) {
				console.error('getFrameForTexImage3D fallback error:', fe);
			}
		}
	}

	getFrameForTexSubImage3D(nativeCtx: any, ctx: any, target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number) {
		//@ts-ignore
		const flipY = nativeCtx?.__flipY ?? false;
		if (!this.helper.isInForeground) {
			return;
		}
		if (this.helper.assetOutput && this.helper.player) {
			if (!this._renderer) {
				this._renderer = NSCRender.alloc().init();
			}
			try {
				//@ts-ignore
				this._renderer.drawFrameTexSubImage3D(this.helper.player, this.helper.assetOutput, this.helper.videoSize, target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, flipY);
				return;
			} catch (e) {
				console.error('getFrameForTexSubImage3D error:', e);
			}
			// Simulator / CPU fallback: read raw BGRA pixel data and upload via texSubImage3D
			try {
				const frameData = NSCRender.getVideoFrameData(this.helper.player, this.helper.assetOutput, this.helper.videoSize);
				if (frameData) {
					const nsData = frameData.objectForKey('data') as NSData;
					const fw = (frameData.objectForKey('width') as NSNumber).intValue;
					const fh = (frameData.objectForKey('height') as NSNumber).intValue;
					if (nsData && fw > 0 && fh > 0) {
						const pixels = new Uint8Array(interop.bufferFromData(nsData));
						// NSCRender returns BGRA; swap B↔R to get RGBA
						for (let i = 0; i < pixels.length; i += 4) {
							const b = pixels[i];
							pixels[i] = pixels[i + 2];
							pixels[i + 2] = b;
						}
						ctx.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, fw, fh, depth, format, type, pixels);
					}
				}
			} catch (fe) {
				console.error('getFrameForTexSubImage3D fallback error:', fe);
			}
		}
	}

	drawImageFrame(context2d: any, args: any[]) {
		if (!this.helper.isInForeground) {
			return;
		}
		if (this.helper.assetOutput && this.helper.player) {
			try {
				const ptr = context2d.context.__getPointer();
				let dirty = false;
				if (args.length === 3) {
					dirty = NSCRender.drawVideoFrame(this.helper.player, this.helper.assetOutput, this.helper.videoSize, ptr, args[1], args[2]);
				} else if (args.length === 5) {
					dirty = NSCRender.drawVideoFrame(this.helper.player, this.helper.assetOutput, this.helper.videoSize, ptr, args[1], args[2], args[3], args[4]);
				} else if (args.length === 9) {
					dirty = NSCRender.drawVideoFrame(this.helper.player, this.helper.assetOutput, this.helper.videoSize, ptr, args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]);
				}
				if (dirty) {
					context2d.context.__makeDirty();
				}
			} catch (e) {
				console.error('drawImageFrame error:', e);
			}
		}
	}

	getVideoFrameData(): any {
		if (!this.helper.isInForeground) {
			return null;
		}
		if (this.helper.assetOutput && this.helper.player) {
			try {
				return NSCRender.getVideoFrameData(this.helper.player, this.helper.assetOutput, this.helper.videoSize);
			} catch (e) {
				console.error('getVideoFrameData error:', e);
				return null;
			}
		}
		return null;
	}

	createNativeView(): Object {
		const ctrl = this.topViewController;
		if (ctrl) {
			ctrl.addChildViewController(this.helper.controller);
			this.helper.controller.didMoveToParentViewController(ctrl);
		}
		return this.helper.controller.view;
	}

	initNativeView() {
		super.initNativeView();
	}

	canPlayType(type: string) {
		try {
			return this.helper.canPlayType(type) || '';
		} catch (e) {
			return '';
		}
	}

	[controlsProperty.setNative](enable: boolean) {
		this.helper.controls = enable;
	}

	[playsinlineProperty.setNative](inline: boolean) {
		this.helper.playsinline = inline;
	}

	[autoplayProperty.setNative](autoplay: boolean) {
		this.helper.autoplay = autoplay;
	}

	_addChildFromBuilder(name: string, value: any) {
		if (value instanceof Source) {
			this._sourceView.push(value);
		}
	}

	onLoaded() {
		super.onLoaded();
		const item = this._sourceView.filter((item) => {
			if (item.type.indexOf('video/webm') === -1) {
				return item;
			}
		})[0];
		if (item) {
			this.helper.src = item.src;
		}
	}

	get duration() {
		return this.helper.duration;
	}

	get currentTime() {
		return this.helper.currentTime;
	}

	set currentTime(value: number) {
		this.helper.currentTime = value;
	}

	get muted() {
		return this.helper.muted;
	}

	set muted(value: boolean) {
		this.helper.muted = value;
	}

	get src() {
		return this.helper.src;
	}

	set src(value: string) {
		let src = value;
		if (typeof src === 'string' && src.startsWith('~/')) {
			src = path.join(knownFolders.currentApp().path, src.replace('~', ''));
		}
		this.helper.src = src;
	}

	load() {
		if (!this.helper) {
			return;
		}
		this.helper.load();
	}

	get controls() {
		return this.helper.controls;
	}

	set controls(enabled: boolean) {
		this.helper.controls = enabled;
	}

	_playPromise: Promise<void> | null = null;
	_playResolve: (() => void) | null = null;
	_playReject: (() => void) | null = null;

	play() {
		if (this.helper.state === 1) {
			return Promise.resolve();
		}
		if (this._playPromise) {
			return this._playPromise;
		}
		this._playPromise = new Promise<void>((resolve, reject) => {
			this._playResolve = resolve;
			this._playReject = reject;
			this.helper.play();
		});

		return this._playPromise;
	}

	pause() {
		this.helper.pause();
	}

	// @ts-ignore
	get loop() {
		return this.helper.isLoop;
	}

	// @ts-ignore
	set loop(value: boolean | string) {
		let loopValue: boolean;
		switch (typeof value) {
			case 'string':
				loopValue = value.toLowerCase() === 'true';
				break;
			case 'boolean':
				loopValue = value as boolean;
				break;
			default:
				loopValue = Boolean(value);
		}
		this.helper.loop = loopValue;
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
