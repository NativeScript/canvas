import { HTMLElement } from './HTMLElement';

let Audio: any;

export class HTMLAudioElement extends HTMLElement {
	_audio;

	constructor() {
		super('audio');
		if (!Audio) {
			try {
				// @ts-ignore
				const media = require('@nativescript/canvas-media');
				Audio = media.Audio;
			} catch (e) {}
		}
		if (Audio) {
			this._audio = Audio.createCustomView();
			this.nativeElement = this._audio;

			if (!this.nativeElement.__domElement) {
				this.nativeElement.__domElement = new DOMParser().parseFromString('<audio></audio>', 'text/html').documentElement as never;
			}
		}
	}

	attachAudioContextTap(ctx: any) {
		if (this._audio) {
			try {
				return this._audio.attachAudioContextTap(ctx);
			} catch (error) {
				return null;
			}
		}
		return null;
	}

	detachAudioContextTap(ctx: any) {
		if (this._audio) {
			try {
				this._audio.detachAudioContextTap(ctx);
			} catch (error) {}
		}
		return null;
	}

	canPlayType(type: string): '' | 'probably' | 'maybe' {
		if (!this._audio) return '';
		return this._audio.canPlayType(type);
	}

	get readyState() {
		return this._audio?.readyState ?? 0;
	}

	get autoplay() {
		return this._audio?.autoplay ?? false;
	}

	set autoplay(value: boolean) {
		if (this._audio) {
			this._audio.autoplay = value;
		}
	}

	get muted() {
		return this._audio?.muted ?? false;
	}

	set muted(value: boolean) {
		if (this._audio) {
			this._audio.muted = value;
		}
	}

	get controls() {
		return this._audio?.controls ?? false;
	}

	set controls(value: boolean) {
		if (this._audio) {
			this._audio.controls = value;
		}
	}

	get currentTime() {
		return this._audio?.currentTime ?? 0;
	}

	set currentTime(value: number) {
		if (this._audio) {
			this._audio.currentTime = value;
		}
	}

	get loop() {
		return this._audio?.loop ?? false;
	}

	set loop(value: boolean) {
		if (this._audio) {
			this._audio.loop = value;
		}
	}

	get src() {
		return this._audio?.src ?? '';
	}

	set src(value: string) {
		if (this._audio) {
			this._audio.src = value;
		}
	}

	set width(value) {
		if (this._audio) {
			this._audio.width = value as any;
		}
	}

	get width() {
		return (this._audio?.width as any) ?? 0;
	}

	set height(value) {
		if (this._audio) {
			this._audio.height = value as any;
		}
	}

	get height() {
		return (this._audio?.height as any) ?? 0;
	}

	load() {
		if (this._audio) {
			this._audio.load();
		}
	}

	play() {
		if (this._audio) {
			return this._audio.play();
		}
		return Promise.reject('Audio element is not supported');
	}

	pause() {
		if (this._audio) {
			this._audio.pause();
		}
	}
}
