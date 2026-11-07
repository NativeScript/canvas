import { booleanConverter } from '@nativescript/core';
import { VideoBase } from './common';
import { Source } from '../source';
import { LinuxMediaPlayer, canPlayType } from '../player-linux';

declare const interop: any;

const GL_RGBA = 0x1908;
const GL_UNSIGNED_BYTE = 0x1401;
const GL_RGBA8 = 0x8058;
const GL_SRGB8_ALPHA8 = 0x8c43;

/** Frames are RGBA8: sized RGBA formats (WebGL 2) are kept, anything else uploads as RGBA. */
function rgbaInternalFormat(internalformat: number) {
	return internalformat === GL_RGBA8 || internalformat === GL_SRGB8_ALPHA8 ? internalformat : GL_RGBA;
}

interface Frame {
	id: number;
	pixels: Uint8Array;
	width: number;
	height: number;
}

export class Video extends VideoBase {
	_media: LinuxMediaPlayer;
	_sourceView: Source[] = [];
	private _sources: string[] = [];
	_isCustom = false;
	private _controls = false;
	private _frame: Frame | null = null;
	private _asset: any;
	private _assetFrameId = 0;

	constructor() {
		super();
		this._media = new LinuxMediaPlayer(this, true);
	}

	static createCustomView() {
		const video = new Video();
		video._isCustom = true;
		video.width = 300;
		video.height = 150;
		return video;
	}

	get _player() {
		return this._media.player;
	}

	get _bridge() {
		return this._media.bridge;
	}

	/** The native player's GtkPicture: it shows every decoded frame itself. */
	createNativeView() {
		return new interop.Pointer(this._bridge.widget);
	}

	disposeNativeView() {
		this._media.pause();
		super.disposeNativeView();
	}

	_onFrame() {
		this._notifyVideoFrameCallbacks();
	}

	/** The latest frame, copied into a buffer kept across frames of the same size. */
	_currentFrame(): Frame | null {
		const bridge = this._bridge;
		const id = bridge.frameId;
		if (!id) {
			return null;
		}
		if (this._frame?.id !== id) {
			const width = bridge.videoWidth;
			const height = bridge.videoHeight;
			let pixels = this._frame?.pixels;
			if (!pixels || pixels.length !== width * height * 4) {
				pixels = new Uint8Array(width * height * 4);
			}
			if (!bridge.readPixelsInto(pixels)) {
				return null;
			}
			this._frame = { id, pixels, width, height };
		}
		return this._frame;
	}

	private _currentAsset(): any {
		const frame = this._currentFrame();
		if (!frame) {
			return null;
		}
		if (this._assetFrameId !== frame.id) {
			this._asset ??= new global.CanvasModule.ImageAsset();
			// Video frames are opaque: premultiplied as they are.
			if (!this._asset.fromBytesSync(frame.width, frame.height, frame.pixels, true)) {
				return null;
			}
			this._assetFrameId = frame.id;
		}
		return this._asset;
	}

	/** Natively: a GPU copy for GL frames, one upload for memory ones. `false` if it could not. */
	private _texImageNative(native: any, target: number, level: number, internalformat: number, threeD: boolean, sub: boolean, x = 0, y = 0, z = 0): boolean {
		const bridge = this._bridge;
		if (typeof native?.__texImageVideoFrame !== 'function' || !bridge.frameId) {
			return false;
		}
		const frame = bridge.gpuFrame();
		if (!frame) {
			return false;
		}
		try {
			return native.__texImageVideoFrame(frame.address, target, level, internalformat, threeD, sub, x, y, z);
		} finally {
			frame.close();
		}
	}

	/** WebGL `texImage2D` / `texSubImage2D` with this video: `(native, context, target, level, internalformat, ...)`. */
	getCurrentFrame(context?: any) {
		const native = arguments[0];
		if (this._texImageNative(native, arguments[2], arguments[3], arguments[4], false, false)) {
			return;
		}
		const frame = this._currentFrame();
		if (!frame || typeof native?.texImage2D !== 'function') {
			return;
		}
		native.texImage2D(arguments[2], arguments[3], rgbaInternalFormat(arguments[4]), frame.width, frame.height, 0, GL_RGBA, GL_UNSIGNED_BYTE, frame.pixels);
	}

	getFrameForTexImage3D(nativeCtx: any, ctx: any, target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number) {
		// One frame is one slice.
		if (depth !== 1 || this._texImageNative(nativeCtx, target, level, internalformat, true, false)) {
			return;
		}
		const frame = this._currentFrame();
		if (!frame) {
			return;
		}
		nativeCtx.texImage3D(target, level, rgbaInternalFormat(internalformat), frame.width, frame.height, 1, border, GL_RGBA, GL_UNSIGNED_BYTE, frame.pixels);
	}

	getFrameForTexSubImage3D(nativeCtx: any, ctx: any, target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number) {
		if (depth !== 1 || this._texImageNative(nativeCtx, target, level, format, true, true, xoffset, yoffset, zoffset)) {
			return;
		}
		const frame = this._currentFrame();
		if (!frame) {
			return;
		}
		nativeCtx.texSubImage3D(target, level, xoffset, yoffset, zoffset, frame.width, frame.height, 1, GL_RGBA, GL_UNSIGNED_BYTE, frame.pixels);
	}

	/** 2D `drawImage(video, ...)`; `args` are drawImage's arguments. */
	drawImageFrame(context2d: any, args: any[]) {
		const asset = this._currentAsset();
		if (!asset || !context2d?.context) {
			return;
		}
		if (args.length === 3) {
			context2d.context.drawImage(asset, args[1], args[2]);
		} else if (args.length === 5) {
			context2d.context.drawImage(asset, args[1], args[2], args[3], args[4]);
		} else if (args.length === 9) {
			context2d.context.drawImage(asset, args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]);
		}
	}

	attachAudioContextTap(): any {
		return this._media.attachAudioTap();
	}

	detachAudioContextTap(): void {
		this._media.detachAudioTap();
	}

	getVideoFrameData(): any {
		return this._currentAsset();
	}

	private _gpuFrameId = 0;

	/**
	 * `device`: the WebGPU device's UUID key (`__frameDevice`). Every frame goes to WebGPU natively:
	 * GL frames on the GPU when on the same device, else as mapped memory (one upload).
	 */
	supportsGPUFrames(device: number): boolean {
		return !!device;
	}

	/** null when no frame was decoded since the last one handed out; `close()` it once the upload is issued. */
	getGPUFrameTexture(device: number): any {
		const bridge = this._bridge;
		const id = bridge.frameId;
		if (!id || id === this._gpuFrameId || !device) {
			return null;
		}
		const frame = bridge.gpuFrame(bridge.gpuFrames && bridge.deviceKey === device);
		if (!frame) {
			return null;
		}
		this._gpuFrameId = id;
		return {
			texturePointer: frame.address,
			width: frame.width,
			height: frame.height,
			// importExternalTexture draws it into a plane of its own.
			copiedOnImport: true,
			close: () => frame.close(),
		};
	}

	get readyState() {
		return this._media.readyState;
	}

	get videoWidth() {
		return this._bridge.videoWidth;
	}

	get videoHeight() {
		return this._bridge.videoHeight;
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
		this._frame = null;
		this._assetFrameId = 0;
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

	/** No transport controls on Linux yet. */
	set controls(value: boolean) {
		this._controls = booleanConverter(value as any);
	}

	//@ts-ignore
	get autoplay() {
		return this._media.autoplay;
	}

	set autoplay(value: boolean) {
		this._media.autoplay = booleanConverter(value as any);
	}

	// @ts-ignore
	get loop() {
		return this._media.loop;
	}

	// @ts-ignore
	set loop(value: boolean | string) {
		this._media.loop = booleanConverter(value as any);
	}
}
