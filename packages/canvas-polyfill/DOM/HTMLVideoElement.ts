import { HTMLElement } from './HTMLElement';

let Video: any;

export class HTMLVideoElement extends HTMLElement {
	_video;

	constructor() {
		super('video');
		if (!Video) {
			try {
				// @ts-ignore
				const video = require('@nativescript/canvas-media');
				Video = video.Video;
			} catch (e) {
			}
		}
		if (Video) {
			this._video = Video.createCustomView();
			this.nativeElement = this._video;
		}
	}

	canPlayType(type): '' | 'probably' | 'maybe' {
		// "video/webm"
		switch (type) {
			case 'video/mp4':
			case 'video/ogg':
				return 'probably';
			default:
				return '';
		}
	}

	requestVideoFrameCallback(callback: Function) {
		this._video?.requestVideoFrameCallback(callback);
	}

	get autoplay() {
		return this._video?.autoplay ?? false;
	}

	set autoplay(value: boolean) {
		if (this._video) {
			this._video.autoplay = value;
		}
	}

	get muted() {
		return this._video?.muted ?? false;
	}

	set muted(value: boolean) {
		if (this._video) {
			this._video.muted = value;
		}
	}

	get controls() {
		return this._video?.controls ?? false;
	}

	set controls(value: boolean) {
		if (this._video) {
			this._video.controls = value;
		}
	}

	get loop() {
		return this._video?.loop ?? false;
	}

	set loop(value: boolean) {
		if (this._video) {
			this._video.loop = value;
		}
	}

	get src() {
		return this._video?.src ?? '';
	}

	set src(value: string) {
		if (this._video) {
			this._video.src = value;
		}
	}

	set width(value) {
		if (this._video) {
			this._video.width = value as any;
		}
	}

	get width() {
		return (this._video?.width as any) ?? 0;
	}

	set height(value) {
		if (this._video) {
			this._video.height = value as any;
		}
	}

	get height() {
		return (this._video?.height as any) ?? 0;
	}

	play() {
		if (this._video) {
			this._video.play();
		}
	}

	pause() {
		if (this._video) {
			this._video.pause();
		}
	}
}
