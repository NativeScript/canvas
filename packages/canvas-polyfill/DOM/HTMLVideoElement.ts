import { HTMLElement } from './HTMLElement';

let Video: any;
let _VideoFrame: any;

export class HTMLVideoElement extends HTMLElement {
	_video;

	constructor() {
		super('video');
		if (!Video) {
			try {
				// @ts-ignore
				const media = require('@nativescript/canvas-media');
				Video = media.Video;
				_VideoFrame = media.VideoFrame;
			} catch (e) {}
		}
		if (Video) {
			this._video = Video.createCustomView();
			this.nativeElement = this._video;

			if (!this.nativeElement.__domElement) {
				this.nativeElement.__domElement = new DOMParser().parseFromString('<video></video>', 'text/html').documentElement as never;
			}
		}
	}

	canPlayType(type: string): '' | 'probably' | 'maybe' {
		if (!this._video) return '';
		return this._video.canPlayType(type);
	}

	requestVideoFrameCallback(callback: Function) {
		this._video?.requestVideoFrameCallback?.(callback);
	}

	cancelVideoFrameCallback(callback: Function) {
		this._video?.cancelVideoFrameCallback?.(callback);
	}

	captureFrame(init?: { timestamp?: number; duration?: number }): any | null {
		if (!_VideoFrame || !this._video) return null;
		try {
			return _VideoFrame.fromVideo(this, init);
		} catch {
			return null;
		}
	}

	get readyState() {
		return this._video?.readyState ?? 0;
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

	get currentTime() {
		return this._video?.currentTime ?? 0;
	}

	set currentTime(value: number) {
		if (this._video) {
			this._video.currentTime = value;
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

	load() {
		if (this._video) {
			this._video.load();
		}
	}

	play() {
		if (this._video) {
			return this._video.play();
		}
		return Promise.reject('Video element is not supported');
	}

	pause() {
		if (this._video) {
			this._video.pause();
		}
	}
}
