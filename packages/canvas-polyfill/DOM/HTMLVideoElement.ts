import { Element } from './Element';
import { Video } from '@nativescript/canvas-media';
export class HTMLVideoElement extends Element {
	_video: Video;
	constructor() {
		super('video');
		// @ts-ignore
		this._video = Video.createCustomView();
	}

	addEventListener(type: string, listener: Function, useCapture: boolean | any) {
		this._video.addEventListener(type, listener, useCapture);
	}
	removeEventListener(type: string, listener: Function, useCapture: boolean | any) {
		this._video.removeEventListener(type, listener, useCapture);
	}

	requestVideoFrameCallback(callback: Function){
		this._video.requestVideoFrameCallback(callback);
	}

	get autoplay() {
		return this._video.autoplay;
	}

	set autoplay(value: boolean) {
		this._video.autoplay = value;
	}

	get muted() {
		return this._video.muted;
	}

	set muted(value: boolean) {
		this._video.muted = value;
	}

	get controls() {
		return this._video.controls;
	}

	set controls(value: boolean) {
		this._video.controls = value;
	}

	get loop() {
		return this._video.muted;
	}

	set loop(value: boolean) {
		this._video.loop = value;
	}

	get src() {
		return this._video.src;
	}

	set src(value: string) {
		this._video.src = value;
	}

	set width(value) {
		if (this._video) {
			this._video.width = value as any;
		}
	}

	get width() {
		if (this._video) {
			return this._video.width;
		}
		return this._video;
	}

	set height(value) {
		if (this._video) {
			this._video.height = value as any;
		}
	}

	get height() {
		if (this._video) {
			return this._video.height;
		}
		return this._video;
	}

	play() {
		this._video.play();
	}

	pause() {
		this._video.pause();
	}
}
