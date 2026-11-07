import { knownFolders, path } from '@nativescript/core';
import { MediaBase, currentTimeProperty, durationProperty } from './common';

declare const __non_webpack_require__: (specifier: string) => any;

/** The GStreamer player in canvasmedia.node (crates/canvas-media-napi/src/linux.rs). */
export interface NativeMediaPlayer {
	readonly widget: number;
	readonly videoWidth: number;
	readonly videoHeight: number;
	readonly frameId: number;
	currentTime: number;
	readonly duration: number;
	volume: number;
	muted: boolean;
	loop: boolean;
	setSource(source: string): void;
	play(): boolean;
	pause(): void;
	readPixelsInto(out: Uint8Array): boolean;
	close(): void;
}

// Loaded on import, so a missing module makes canvas-polyfill's probes fall back.
const Native: { NSCMediaPlayer: new (onEvent: (type: string, detail?: string) => void, video?: boolean) => NativeMediaPlayer } = __non_webpack_require__('system_lib://canvasmedia.node');

/** MediaPlaybackState, as the native player reports it. */
const enum PlaybackState {
	None = 0,
	Opening = 1,
	Buffering = 2,
	Playing = 3,
	Paused = 4,
}

const HAVE_NOTHING = 0;
const HAVE_METADATA = 1;
const HAVE_ENOUGH_DATA = 4;

const PLAYABLE = /^(video\/(mp4|x-m4v|quicktime|3gpp|3gpp2|mp2t|webm|x-ms-wmv|x-ms-asf)|audio\/(mpeg|mp3|mp4|x-m4a|aac|aacp|3gpp|3gpp2|wav|wave|x-wav|vnd\.wave|flac|x-flac|ogg|opus|webm|x-ms-wma|amr))$/;

/** What GStreamer's usual plugin sets play; the installed decoders decide in the end. */
export function canPlayType(type: string): '' | 'maybe' {
	const mime = String(type ?? '')
		.split(';')[0]
		.trim()
		.toLowerCase();
	return PLAYABLE.test(mime) ? 'maybe' : '';
}

export function resolveSrc(src: string): string {
	if (typeof src === 'string' && src.startsWith('~/')) {
		return path.join(knownFolders.currentApp().path, src.replace('~', ''));
	}
	return src;
}

type MediaOwner = MediaBase & {
	_notifyListener(type: string): void;
	_onFrame?(): void;
	/** `true` when the owner loads another source instead (its next `<Source>`). */
	_onLoadError?(): boolean;
};

export class LinuxMediaPlayer {
	readonly player: NativeMediaPlayer;
	readyState = HAVE_NOTHING;
	private _src = '';
	private _playing = false;
	private _play: { promise: Promise<void>; resolve: () => void; reject: (reason?: any) => void } | null = null;
	private _timeupdate: any = null;
	private _owner: WeakRef<MediaOwner>;
	private _autoplay = false;

	constructor(owner: MediaOwner, video: boolean) {
		this._owner = new WeakRef(owner);
		const ref = new WeakRef(this);
		this.player = new Native.NSCMediaPlayer((type, detail) => ref.deref()?._onEvent(type, detail), video);
	}

	/** The native player stands in for the Windows bridge's frame access. */
	get bridge() {
		return this.player;
	}

	private _notify(type: string) {
		this._owner.deref()?._notifyListener(type);
	}

	private _onEvent(type: string, detail?: string) {
		const owner = this._owner.deref();
		if (!owner) {
			return;
		}
		switch (type) {
			case 'opened':
				if (this.readyState !== HAVE_NOTHING) {
					break;
				}
				this.readyState = HAVE_METADATA;
				durationProperty.nativeValueChange(owner, this.duration);
				this._notify('durationchange');
				this._notify('loadedmetadata');
				this.readyState = HAVE_ENOUGH_DATA;
				this._notify('loadeddata');
				this._notify('canplay');
				this._notify('canplaythrough');
				if (this._autoplay) {
					this.play().catch(() => {});
				}
				break;
			case 'state':
				this._onState(Number(detail));
				break;
			case 'frame':
				owner._onFrame?.();
				break;
			case 'seeked':
				currentTimeProperty.nativeValueChange(owner, this.currentTime);
				this._notify('seeked');
				this._notify('timeupdate');
				break;
			case 'durationchange':
				if (this.readyState >= HAVE_METADATA) {
					durationProperty.nativeValueChange(owner, this.duration);
					this._notify('durationchange');
				}
				break;
			case 'ended':
				this._setPlaying(false);
				currentTimeProperty.nativeValueChange(owner, this.currentTime);
				this._notify('timeupdate');
				this._notify('pause');
				this._notify('ended');
				break;
			case 'error': {
				if (this.readyState === HAVE_NOTHING && owner._onLoadError?.()) {
					break;
				}
				this._setPlaying(false);
				this._rejectPlay(Object.assign(new Error(detail || 'Media playback failed'), { name: 'NotSupportedError' }));
				this._notify('error');
				break;
			}
			case 'resize':
			case 'waiting':
				this._notify(type);
				break;
		}
	}

	private _onState(state: PlaybackState) {
		if (state === PlaybackState.Playing) {
			if (this._playing) {
				return;
			}
			this._setPlaying(true);
			const play = this._play;
			this._play = null;
			if (play) {
				play.resolve();
			} else {
				// Started by autoplay.
				this._notify('play');
			}
			this._notify('playing');
		} else if (state === PlaybackState.Paused || state === PlaybackState.None) {
			this._setPlaying(false);
		}
	}

	private _setPlaying(playing: boolean) {
		this._playing = playing;
		if (playing && !this._timeupdate) {
			// The web fires timeupdate every 15-250 ms while playing.
			this._timeupdate = setInterval(() => {
				const owner = this._owner.deref();
				if (owner) {
					currentTimeProperty.nativeValueChange(owner, this.currentTime);
					this._notify('timeupdate');
				}
			}, 250);
		} else if (!playing && this._timeupdate) {
			clearInterval(this._timeupdate);
			this._timeupdate = null;
		}
	}

	private _rejectPlay(error: Error) {
		const play = this._play;
		this._play = null;
		play?.reject(error);
	}

	get src() {
		return this._src;
	}

	set src(value: string) {
		this._src = value ?? '';
		this.load();
	}

	load() {
		this._setPlaying(false);
		this._rejectPlay(Object.assign(new Error('The play() request was interrupted by a new load request.'), { name: 'AbortError' }));
		this.readyState = HAVE_NOTHING;
		const src = resolveSrc(this._src);
		try {
			this.player.setSource(src);
		} catch (error) {
			console.warn(`canvas-media: cannot load ${src}:`, error);
			this._notify('error');
		}
	}

	play(): Promise<void> {
		if (this._playing) {
			return Promise.resolve();
		}
		if (this._play) {
			return this._play.promise;
		}
		if (!this._src) {
			return Promise.reject(Object.assign(new Error('The element has no supported sources.'), { name: 'NotSupportedError' }));
		}
		let resolve: () => void, reject: (reason?: any) => void;
		const promise = new Promise<void>((res, rej) => {
			resolve = res;
			reject = rej;
		});
		this._play = { promise, resolve, reject };
		this._notify('play');
		if (!this.player.play()) {
			this._rejectPlay(Object.assign(new Error('The media could not be played.'), { name: 'NotSupportedError' }));
		}
		return promise;
	}

	pause() {
		const wasPlaying = this._playing || !!this._play;
		this.player.pause();
		this._setPlaying(false);
		this._rejectPlay(Object.assign(new Error('The play() request was interrupted by a call to pause().'), { name: 'AbortError' }));
		if (wasPlaying) {
			this._notify('pause');
		}
	}

	get paused() {
		return !this._playing && !this._play;
	}

	/** No audio tap on Linux yet: an AudioContext cannot take the element's audio. */
	attachAudioTap(): null {
		return null;
	}

	detachAudioTap() {}

	get currentTime() {
		return this.player.currentTime || 0;
	}

	set currentTime(value: number) {
		value = Number(value);
		if (!Number.isFinite(value) || value < 0) {
			return;
		}
		this.player.currentTime = value;
		this._notify('seeking');
	}

	get duration() {
		return this.readyState < HAVE_METADATA ? NaN : this.player.duration;
	}

	get muted(): boolean {
		return this.player.muted;
	}

	set muted(value: boolean) {
		value = !!value;
		if (value !== this.muted) {
			this.player.muted = value;
			this._notify('volumechange');
		}
	}

	get volume(): number {
		return this.player.volume;
	}

	set volume(value: number) {
		value = Math.min(1, Math.max(0, Number(value) || 0));
		if (value !== this.volume) {
			this.player.volume = value;
			this._notify('volumechange');
		}
	}

	get loop(): boolean {
		return this.player.loop;
	}

	set loop(value: boolean) {
		this.player.loop = !!value;
	}

	get autoplay(): boolean {
		return this._autoplay;
	}

	set autoplay(value: boolean) {
		this._autoplay = !!value;
	}
}
