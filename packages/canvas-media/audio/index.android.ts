import { AudioBase } from './common';
import { Application, booleanConverter, knownFolders, path } from '@nativescript/core';
import { Source } from '..';
import { durationProperty, currentTimeProperty } from '../common';

declare var org: any;

export class Audio extends AudioBase {
	_instance: any;
	_playResolve: Function | null = null;
	_playReject: Function | null = null;
	_playPromise: Promise<void> | null = null;
	_sourceView: Source[] = [];
	_playing: boolean = false;
	_readyState: number = 0;
	_isCustom: boolean = false;

	static createCustomView() {
		const audio = new Audio();
		audio._isCustom = true;
		audio.width = 300;
		audio.height = 40;
		return audio;
	}

	constructor() {
		super();

		const activity: any = Application.android.foregroundActivity || Application.android.startActivity;
		this._instance = new org.nativescript.canvas.media.AudioHelper(activity, knownFolders.documents().path);

		const ref = new WeakRef(this);

		this._instance.setCallback(
			new org.nativescript.canvas.media.AudioHelper.Callback({
				onDurationChange(duration: number) {
					const owner = ref.get();
					if (owner) {
						let secs = Number(duration) > 0 ? Number(duration) / 1000.0 : NaN;
						durationProperty.nativeValueChange(owner, secs);
						owner._notifyListener(Audio.durationchangeEvent);
					}
				},

				onPlaying() {
					const owner = ref.get();
					if (owner) {
						owner._playing = true;
						if (owner._playResolve) {
							owner._playResolve();
							owner._playResolve = null;
							owner._playReject = null;
							owner._playPromise = null;
						}
						owner._notifyListener(Audio.playingEvent);
					}
				},

				onCurrentTimeChanged(time: number) {
					const owner = ref.get();
					if (owner) {
						let secs = Number(time) / 1000.0;
						currentTimeProperty.nativeValueChange(owner, secs);
						owner._notifyListener(Audio.timeupdateEvent);
					}
				},

				onLoadedData() {
					const owner = ref.get();
					if (owner) {
						owner._readyState = Audio.HAVE_CURRENT_DATA;
						owner._notifyListener(Audio.durationchangeEvent);
						owner._notifyListener(Audio.loadedmetadataEvent);
						owner._notifyListener(Audio.loadeddataEvent);
					}
				},

				onCanPlay() {
					const owner = ref.get();
					if (owner) {
						owner._notifyListener(Audio.canplayEvent);
					}
				},

				onCanPlayThrough() {
					const owner = ref.get();
					if (owner) {
						owner._notifyListener(Audio.canplaythroughEvent);
					}
				},

				onError(message: string) {
					const owner = ref.get();
					if (owner) {
						owner._playing = false;
						if (owner._playReject) {
							owner._playReject(new Error(message ?? 'Playback error'));
						}
						owner._playResolve = null;
						owner._playReject = null;
						owner._playPromise = null;
						owner._notifyListener(Audio.errorEvent);
					}
				},
			}),
		);
	}

	createNativeView() {
		return this._instance.getContainer();
	}

	play() {
		if (this._playing) {
			return Promise.resolve();
		}

		if (this._playPromise) {
			return this._playPromise;
		}

		this._notifyListener(Audio.playEvent);

		this._playPromise = new Promise<void>((resolve, reject) => {
			this._playResolve = resolve;
			this._playReject = reject;
			try {
				this._instance.play();
			} catch (e) {
				this._playResolve = null;
				this._playReject = null;
				this._playPromise = null;
				reject(e);
			}
		});
		return this._playPromise;
	}

	pause() {
		if (!this._instance) {
			return;
		}
		this._instance.pause();
		this._notifyListener(Audio.pauseEvent);
	}

	get muted() {
		return this._instance.getMuted();
	}

	set muted(value: boolean) {
		this._instance.setMuted(booleanConverter(value));
	}

	get duration() {
		const d = this._instance.getDuration();
		if (d == null || Number(d) <= 0) return NaN;
		return Number(d) / 1000.0;
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

	load() {
		if (!this._instance) {
			return;
		}
		this._instance.load();
	}

	canPlayType(type: string) {
		if (!this._instance) {
			return '';
		}
		return this._instance.canPlayType(type);
	}

	get autoplay() {
		try {
			return this._instance.getAutoplay();
		} catch (e) {
			return false;
		}
	}

	set autoplay(value: boolean) {
		this._instance.setAutoplay(booleanConverter(value));
	}

	get controls() {
		try {
			return this._instance.getControls();
		} catch (e) {
			return false;
		}
	}

	set controls(enabled: boolean) {
		this._instance.setControls(booleanConverter(enabled));
	}

	get loop() {
		return this._instance.getLoop();
	}

	set loop(value: boolean) {
		this._instance.setLoop(booleanConverter(value));
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

	attachAudioContextTap(contextNative: any): any {
		const inst: any = (this as any)._instance;
		if (!inst || typeof inst.attachAudioContextTap !== 'function') {
			return null;
		}
		try {
			return inst.attachAudioContextTap(contextNative);
		} catch (e) {
			console.warn('Audio.attachAudioContextTap: native call failed:', e);
			return null;
		}
	}

	detachAudioContextTap(): void {
		const inst: any = (this as any)._instance;
		if (!inst || typeof inst.detachAudioContextTap !== 'function') return;
		try {
			inst.detachAudioContextTap();
		} catch (e) {
			console.warn('Audio.detachAudioContextTap: native call failed:', e);
		}
	}
}
