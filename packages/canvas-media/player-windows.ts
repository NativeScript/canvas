import { knownFolders, path } from '@nativescript/core';
import { MediaBase, currentTimeProperty, durationProperty } from './common';

declare const Windows: any, NSWinRT: any, __non_webpack_require__: (specifier: string) => any;

export interface SharedFrame {
	/** A `CanvasD3DSharedFrame` for WebGPU's `nativeTexture`. */
	readonly address: number;
	readonly width: number;
	readonly height: number;
	close(): void;
}

/** The player's audio routed into a Web Audio graph (MediaElementAudioSourceNode). */
export interface AudioTap {
	/** An `AudioTapSource` for audiocontext.node's `createMediaElementSourceFromTap`. */
	readonly address: number;
	readonly framesTapped: number;
	setGain(gain: number): void;
	setRouted(routed: boolean): void;
}

export interface MediaPlayerBridge {
	readonly videoWidth: number;
	readonly videoHeight: number;
	/** Changes with every decoded frame; 0 before the first. */
	readonly frameId: number;
	readonly adapterLuid: number;
	readonly sharesFrames: boolean;
	gpuFrame(): SharedFrame | null;
	createAudioTap(): AudioTap;
	close(): void;
	attachSurfaceImageSource(key: string, width: number, height: number): boolean;
	detachSurfaceImageSource(): void;
	present(): boolean;
	readPixels(): Uint8Array | null;
}

// Loaded on import, so a missing module makes canvas-polyfill's probes fall back.
const Native: { NSCMediaPlayerBridge: new (playerKey: string, onEvent: (type: string, detail?: string) => void, frames?: boolean) => MediaPlayerBridge } = __non_webpack_require__('system_lib://canvasmedia.node');

/** Windows.Media.Playback.MediaPlaybackState */
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

/** Media Foundation's containers and codecs, plus WebM/Ogg (the Web Media Extensions Windows ships with). */
export function canPlayType(type: string): '' | 'maybe' {
	const mime = String(type ?? '')
		.split(';')[0]
		.trim()
		.toLowerCase();
	return PLAYABLE.test(mime) ? 'maybe' : '';
}

/** A WinRT TimeSpan (`{ Duration }` in 100 ns ticks) in seconds. */
function seconds(timeSpan: any): number {
	const value = Number(timeSpan?.Duration ?? timeSpan);
	return Number.isFinite(value) ? value / 1e7 : NaN;
}

export function resolveSrc(src: string): string {
	if (typeof src === 'string' && src.startsWith('~/')) {
		return path.join(knownFolders.currentApp().path, src.replace('~', ''));
	}
	return src;
}

function mediaSource(src: string) {
	// A scheme (http:, ms-appx:, file:, ...), not a drive letter.
	const uri = /^[a-z][a-z0-9+.-]+:/i.test(src) ? src : 'file:///' + encodeURI(src.replace(/\\/g, '/').replace(/^\/+/, '')).replace(/#/g, '%23').replace(/\?/g, '%3F');
	return Windows.Media.Core.MediaSource.CreateFromUri(new Windows.Foundation.Uri(uri));
}

type MediaOwner = MediaBase & {
	_notifyListener(type: string): void;
	_onFrame?(): void;
	/** A load failed: `true` when the owner loads another source instead (its next `<Source>`). */
	_onLoadError?(): boolean;
};

/**
 * A `Windows.Media.Playback.MediaPlayer` with HTMLMediaElement state and events, shared by the
 * Windows `Audio` and `Video`. Its events arrive through the native bridge (they are raised off the
 * JS thread); with `frames` the player runs in frame-server mode and decoded frames reach the owner's
 * `_onFrame`.
 */
export class WindowsMediaPlayer {
	readonly player: any;
	readonly bridge: MediaPlayerBridge;
	readyState = HAVE_NOTHING;
	private _src = '';
	private _playing = false;
	private _play: { promise: Promise<void>; resolve: () => void; reject: (reason?: any) => void } | null = null;
	private _timeupdate: any = null;
	private _owner: WeakRef<MediaOwner>;
	private _tap: AudioTap | null = null;

	constructor(owner: MediaOwner, frames: boolean) {
		this._owner = new WeakRef(owner);
		const player = new Windows.Media.Playback.MediaPlayer();
		player.AutoPlay = false;
		if (frames) {
			player.IsVideoFrameServerEnabled = true;
		}
		this.player = player;
		const ref = new WeakRef(this);
		this.bridge = new Native.NSCMediaPlayerBridge(NSWinRT.interop.pointerKey(player), (type, detail) => ref.deref()?._onEvent(type, detail), frames);
		// Before any source: an effect only applies to the sources set after it. Passes the audio
		// through until an AudioContext takes it (createMediaElementSource).
		try {
			this._tap = this.bridge.createAudioTap();
		} catch (e) {}
	}

	private get _session() {
		return this.player.PlaybackSession;
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
				this.readyState = HAVE_METADATA;
				durationProperty.nativeValueChange(owner, this.duration);
				this._notify('durationchange');
				this._notify('loadedmetadata');
				this.readyState = HAVE_ENOUGH_DATA;
				this._notify('loadeddata');
				this._notify('canplay');
				this._notify('canplaythrough');
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
			this.player.Source = src ? mediaSource(src) : null;
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
		try {
			this.player.Play();
		} catch (error) {
			this._rejectPlay(error);
		}
		return promise;
	}

	pause() {
		const wasPlaying = this._playing || !!this._play;
		try {
			this.player.Pause();
		} catch (e) {}
		this._setPlaying(false);
		this._rejectPlay(Object.assign(new Error('The play() request was interrupted by a call to pause().'), { name: 'AbortError' }));
		if (wasPlaying) {
			this._notify('pause');
		}
	}

	get paused() {
		return !this._playing && !this._play;
	}

	/**
	 * Routes the audio into a Web Audio graph instead of the speakers (createMediaElementSource):
	 * an audio effect in the player's pipeline hands the decoded samples over. null where the
	 * effect is unavailable.
	 */
	attachAudioTap(): AudioTap | null {
		if (!this._tap) {
			return null;
		}
		this._tap.setRouted(true);
		this._syncTapGain();
		return this._tap;
	}

	/** The player plays through the speakers again. */
	detachAudioTap() {
		this._tap?.setRouted(false);
	}

	/** The graph gets what the element would play: its volume, or silence when muted. */
	private _syncTapGain() {
		this._tap?.setGain(this.muted ? 0 : this.volume);
	}

	get currentTime() {
		try {
			return seconds(this._session.Position) || 0;
		} catch (e) {
			return 0;
		}
	}

	set currentTime(value: number) {
		value = Number(value);
		if (!Number.isFinite(value) || value < 0) {
			return;
		}
		try {
			this._session.Position = { Duration: Math.round(value * 1e7) };
			this._notify('seeking');
		} catch (e) {}
	}

	get duration() {
		if (this.readyState < HAVE_METADATA) {
			return NaN;
		}
		try {
			return seconds(this._session.NaturalDuration) || NaN;
		} catch (e) {
			return NaN;
		}
	}

	get muted(): boolean {
		return !!this.player.IsMuted;
	}

	set muted(value: boolean) {
		value = !!value;
		if (value !== this.muted) {
			this.player.IsMuted = value;
			this._syncTapGain();
			this._notify('volumechange');
		}
	}

	get volume(): number {
		return this.player.Volume;
	}

	set volume(value: number) {
		value = Math.min(1, Math.max(0, Number(value) || 0));
		if (value !== this.volume) {
			this.player.Volume = value;
			this._syncTapGain();
			this._notify('volumechange');
		}
	}

	get loop(): boolean {
		return !!this.player.IsLoopingEnabled;
	}

	set loop(value: boolean) {
		this.player.IsLoopingEnabled = !!value;
	}

	get autoplay(): boolean {
		return !!this.player.AutoPlay;
	}

	set autoplay(value: boolean) {
		this.player.AutoPlay = !!value;
	}
}
