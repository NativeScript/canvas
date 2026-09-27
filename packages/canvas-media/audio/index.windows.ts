import { booleanConverter } from '@nativescript/core';
import { AudioBase } from './common';
import { Source } from '../source';
import { WindowsMediaPlayer, canPlayType } from '../player-windows';

declare const Microsoft: any;

export class Audio extends AudioBase {
	_media: WindowsMediaPlayer;
	_sourceView: Source[] = [];
	/** `<Source>`s not tried yet, in order. */
	private _sources: string[] = [];
	_isCustom = false;
	private _controls = false;
	private _grid: any;
	/** The transport controls, while `controls` is set. */
	private _element: any;

	constructor() {
		super();
		this._media = new WindowsMediaPlayer(this, false);
	}

	static createCustomView() {
		const audio = new Audio();
		audio._isCustom = true;
		audio.width = 300;
		audio.height = 40;
		return audio;
	}

	get _player() {
		return this._media.player;
	}

	createNativeView() {
		this._grid = new Microsoft.UI.Xaml.Controls.Grid();
		this._syncControls();
		return this._grid;
	}

	disposeNativeView() {
		try {
			this._element?.SetMediaPlayer(null);
		} catch (e) {}
		this._grid = this._element = null;
		super.disposeNativeView();
	}

	private _syncControls() {
		const grid = this._grid;
		if (!grid) {
			return;
		}
		if (this._controls && !this._element) {
			const element = new Microsoft.UI.Xaml.Controls.MediaPlayerElement();
			element.SetMediaPlayer(this._player);
			element.AreTransportControlsEnabled = true;
			try {
				const controls = element.TransportControls;
				controls.IsCompact = true;
				controls.IsFullWindowButtonVisible = false;
				controls.IsZoomButtonVisible = false;
			} catch (e) {}
			grid.Children.Append(element);
			this._element = element;
		} else if (!this._controls && this._element) {
			const element = this._element;
			this._element = null;
			try {
				element.SetMediaPlayer(null);
				grid.Children.Clear();
			} catch (e) {}
		}
	}

	get readyState() {
		return this._media.readyState;
	}

	canPlayType(type: string) {
		return canPlayType(type);
	}

	_addChildFromBuilder(name: string, value: any) {
		if (value instanceof Source) {
			this._sourceView.push(value);
		}
	}

	onLoaded() {
		super.onLoaded();
		if (!this._media.src) {
			this._sources = this._sourceView.filter((item) => item.src && (!item.type || canPlayType(item.type) !== '')).map((item) => item.src);
			this._onLoadError();
		}
	}

	_onLoadError() {
		const next = this._sources.shift();
		if (next === undefined) {
			return false;
		}
		this._media.src = next;
		return true;
	}

	get duration() {
		return this._media.duration;
	}

	get currentTime() {
		return this._media.currentTime;
	}

	set currentTime(value: number) {
		this._media.currentTime = value;
	}

	get muted() {
		return this._media.muted;
	}

	set muted(value: boolean) {
		this._media.muted = booleanConverter(value as any);
	}

	get volume() {
		return this._media.volume;
	}

	set volume(value: number) {
		this._media.volume = value;
	}

	get paused() {
		return this._media.paused;
	}

	get src() {
		return this._media.src;
	}

	set src(value: string) {
		this._sources = [];
		this._media.src = value;
	}

	load() {
		this._media.load();
	}

	play() {
		return this._media.play();
	}

	pause() {
		this._media.pause();
	}

	get controls() {
		return this._controls;
	}

	set controls(value: boolean) {
		this._controls = booleanConverter(value as any);
		this._syncControls();
	}

	get autoplay() {
		return this._media.autoplay;
	}

	set autoplay(value: boolean) {
		this._media.autoplay = booleanConverter(value as any);
	}

	get loop() {
		return this._media.loop;
	}

	set loop(value: boolean) {
		this._media.loop = booleanConverter(value as any);
	}

	/** For audio-context's MediaElementAudioSourceNode: the element's audio, routed into the graph. */
	attachAudioContextTap(): any {
		return this._media.attachAudioTap();
	}

	detachAudioContextTap(): void {
		this._media.detachAudioTap();
	}
}
