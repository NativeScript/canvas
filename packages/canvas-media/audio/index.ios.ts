import { AudioBase } from './common';
import { Source } from '../source';
import { controlsProperty, autoplayProperty, loopProperty, currentTimeProperty, durationProperty } from '../common';
import { booleanConverter, knownFolders, path, Utils } from '@nativescript/core';

declare const NSCAudioHelper;
declare var NSCAudioHelperListener: any;

interface NSCAudioHelperListener {}

@ObjCClass(NSCAudioHelperListener)
@NativeClass()
class NSCAudioHelperListenerImpl extends NSObject implements NSCAudioHelperListener {
	_owner: WeakRef<Audio>;
	public static initWithOwner(owner: WeakRef<Audio>): NSCAudioHelperListenerImpl {
		const obj = NSCAudioHelperListenerImpl.alloc().init() as NSCAudioHelperListenerImpl;
		obj._owner = owner;
		return obj;
	}

	public onStateChange(state) {
		const owner = this._owner.deref();
		if (owner) {
			// NSCPlayerState: 1 === playing
			if (state === 1) {
				if (owner._playResolve) {
					owner._playResolve();
					owner._playResolve = null;
					owner._playReject = null;
					owner._playPromise = null;
				}
				owner._notifyListener(Audio.playingEvent);
			}
		}
	}

	public onTimeUpdate(time) {
		const owner = this._owner.deref();
		if (owner) {
			currentTimeProperty.nativeValueChange(owner, time);
			owner._notifyListener(Audio.timeupdateEvent);
		}
	}

	public onLoadedData() {
		const owner = this._owner.deref();
		if (owner) {
			durationProperty.nativeValueChange(owner, owner.duration);
			owner._notifyListener(Audio.durationchangeEvent);
			owner._notifyListener(Audio.loadedmetadataEvent);
			owner._notifyListener(Audio.loadeddataEvent);
		}
	}

	public onCanPlay() {
		const owner = this._owner.deref();
		if (owner) {
			owner._notifyListener(Audio.canplayEvent);
		}
	}

	public onCanPlayThrough() {
		const owner = this._owner.deref();
		if (owner) {
			owner._notifyListener(Audio.canplaythroughEvent);
		}
	}
}

export class Audio extends AudioBase {
	helper: any;
	_sourceView: Source[] = [];
	private listener: NSCAudioHelperListenerImpl;

	_playPromise: Promise<void> | null = null;
	_playResolve: (() => void) | null = null;
	_playReject: ((err?: any) => void) | null = null;

	_isCustom: boolean = false;

	static createCustomView() {
		const audio = new Audio();
		audio._isCustom = true;
		audio.width = 300;
		audio.height = 40;
		return audio;
	}

	public onMeasure(widthMeasureSpec: number, heightMeasureSpec: number): void {
		const nativeView = this.helper.view;
		if (nativeView) {
			const width = Utils.layout.makeMeasureSpec(Utils.layout.toDevicePixels(300), Utils.layout.EXACTLY);
			const height = Utils.layout.makeMeasureSpec(Utils.layout.toDevicePixels(40), Utils.layout.EXACTLY);

			this.setMeasuredDimension(width, height);
		}
	}

	constructor() {
		super();
		this.helper = NSCAudioHelper.new();
		try {
			AVAudioSession.sharedInstance().setCategoryError(AVAudioSessionCategoryPlayback);
		} catch (e) {}
		this.listener = NSCAudioHelperListenerImpl.initWithOwner(new WeakRef(this));
		this.helper.listener = this.listener;
	}

	createNativeView() {
		return this.helper.view;
	}

	initNativeView() {
		super.initNativeView();
	}

	[controlsProperty.setNative](enable: boolean) {
		this.helper.controls = enable;
	}

	[autoplayProperty.setNative](autoplay: boolean) {
		this.helper.autoplay = autoplay;
	}

	get autoplay() {
		try {
			return this.helper.autoplay;
		} catch (e) {
			return false;
		}
	}

	_addChildFromBuilder(name: string, value: any) {
		if (value instanceof Source) {
			this._sourceView.push(value);
		}
	}

	onLoaded() {
		super.onLoaded();
		const item = this._sourceView[0];
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
		if (isNaN(value) || value < 0) {
			return;
		}
		this.helper.currentTime = value;
	}

	get muted() {
		return this.helper.muted;
	}

	set muted(value: boolean) {
		this.helper.muted = booleanConverter(value);
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

	canPlayType(type: string) {
		if (!this.helper) {
			return '';
		}

		return this.helper.canPlayType(type);
	}

	get controls() {
		return this.helper.controls;
	}

	set controls(enabled: boolean) {
		this.helper.controls = booleanConverter(enabled);
	}

	play() {
		if (this.helper.state === 1) {
			return Promise.resolve();
		}
		if (this._playPromise) {
			return this._playPromise;
		}

		this._notifyListener(Audio.playEvent);
		this._playPromise = new Promise<void>((resolve, reject) => {
			this._playResolve = resolve;
			this._playReject = reject;
			this.helper.play();
		});

		return this._playPromise;
	}

	pause() {
		this.helper.pause();
		this._notifyListener(Audio.pauseEvent);
	}

	// @ts-ignore
	get loop() {
		return this.helper.loop;
	}

	// @ts-ignore
	set loop(value: boolean | string) {
		if (value === undefined || value === null) {
			return;
		}
		this.helper.loop = booleanConverter(value);
	}

	private _activeTapNode: any = null;

	attachAudioContextTap(contextNative: any): any {
		if (!contextNative || typeof contextNative.createSourceNodeFromMediaPlayer !== 'function') {
			return null;
		}
		const player: any = this.helper?.player;
		if (!player) return null;
		try {
			const node = contextNative.createSourceNodeFromMediaPlayer(player);
			if (node) this._activeTapNode = node;
			return node;
		} catch (e) {
			console.warn('Audio.attachAudioContextTap: native call failed:', e);
			return null;
		}
	}

	detachAudioContextTap(): void {
		this._activeTapNode = null;
	}
}
