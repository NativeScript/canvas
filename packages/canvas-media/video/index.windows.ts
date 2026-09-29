import { booleanConverter } from '@nativescript/core';
import { VideoBase } from './common';
import { Source } from '../source';
import { WindowsMediaPlayer, canPlayType } from '../player-windows';

declare const Microsoft: any, NSWinRT: any;

const GL_RGBA = 0x1908;
const GL_UNSIGNED_BYTE = 0x1401;
const GL_RGBA8 = 0x8058;
const GL_SRGB8_ALPHA8 = 0x8c43;

/** Frames are RGBA8: sized RGBA formats (WebGL 2) are kept, anything else uploads as RGBA. */
function rgbaInternalFormat(internalformat: number) {
	return internalformat === GL_RGBA8 || internalformat === GL_SRGB8_ALPHA8 ? internalformat : GL_RGBA;
}

/**
 * The stock MediaPlayerElement template (Windows App SDK 1.6) with its presenter kept for layout (the
 * element measures itself from it) but invisible: in frame-server mode it only paints black.
 */
const CONTROLS_TEMPLATE = `<ControlTemplate xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml" TargetType="MediaPlayerElement">
	<Grid x:Name="LayoutRoot">
		<Border Background="Transparent" />
		<Image x:Name="PosterImage" Visibility="Collapsed" Source="{TemplateBinding PosterSource}" Stretch="{TemplateBinding Stretch}" />
		<MediaPlayerPresenter x:Name="MediaPlayerPresenter" Opacity="0" Stretch="{TemplateBinding Stretch}" MediaPlayer="{TemplateBinding MediaPlayer}" />
		<ContentPresenter x:Name="TransportControlsPresenter" Visibility="{TemplateBinding AreTransportControlsEnabled}" />
		<Grid x:Name="TimedTextSourcePresenter" />
	</Grid>
</ControlTemplate>`;
let controlsTemplate: any;

interface Frame {
	id: number;
	pixels: Uint8Array;
	width: number;
	height: number;
}

export class Video extends VideoBase {
	_media: WindowsMediaPlayer;
	_sourceView: Source[] = [];
	private _sources: string[] = [];
	_isCustom = false;
	private _controls = false;
	private _grid: any;
	private _image: any;
	/** Only for its transport controls: the frames are the Image's. */
	private _element: any;
	private _surface: any;
	private _surfaceWidth = 0;
	private _surfaceHeight = 0;
	private _frame: Frame | null = null;
	private _asset: any;
	private _assetFrameId = 0;

	constructor() {
		super();
		this._media = new WindowsMediaPlayer(this, true);
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

	createNativeView() {
		const { Controls, Media } = Microsoft.UI.Xaml;
		const grid = new Controls.Grid();
		const image = new Controls.Image();
		image.Stretch = Media.Stretch.Uniform;
		grid.Children.Append(image);
		this._grid = grid;
		this._image = image;
		this._surfaceWidth = this._surfaceHeight = 0;
		this._syncControls();
		return grid;
	}

	disposeNativeView() {
		try {
			this._element?.SetMediaPlayer(null);
		} catch (e) {}
		this._bridge.detachSurfaceImageSource();
		this._grid = this._image = this._element = this._surface = null;
		super.disposeNativeView();
	}

	private _syncControls() {
		const grid = this._grid;
		if (!grid) {
			return;
		}
		if (this._controls && !this._element) {
			const element = new Microsoft.UI.Xaml.Controls.MediaPlayerElement();
			controlsTemplate ??= Microsoft.UI.Xaml.Markup.XamlReader.Load(CONTROLS_TEMPLATE);
			element.Template = controlsTemplate;
			element.SetMediaPlayer(this._player);
			element.AreTransportControlsEnabled = true;
			try {
				// Full window and zoom act on the invisible presenter.
				element.TransportControls.IsFullWindowButtonVisible = false;
				element.TransportControls.IsZoomButtonVisible = false;
			} catch (e) {}
			grid.Children.Append(element);
			this._element = element;
		} else if (!this._controls && this._element) {
			const element = this._element;
			this._element = null;
			try {
				element.SetMediaPlayer(null);
				// Always the last child, over the image.
				grid.Children.RemoveAtEnd();
			} catch (e) {}
		}
	}

	_onFrame() {
		this._present();
		this._notifyVideoFrameCallbacks();
	}

	private _present() {
		const image = this._image;
		if (!image) {
			return;
		}
		const bridge = this._bridge;
		const width = bridge.videoWidth;
		const height = bridge.videoHeight;
		if (!width || !height) {
			return;
		}
		if (width !== this._surfaceWidth || height !== this._surfaceHeight) {
			const surface = new Microsoft.UI.Xaml.Media.Imaging.SurfaceImageSource(width, height, true);
			if (!bridge.attachSurfaceImageSource(NSWinRT.interop.pointerKey(surface), width, height)) {
				return;
			}
			image.Source = surface;
			this._surface = surface;
			this._surfaceWidth = width;
			this._surfaceHeight = height;
		}
		bridge.present();
	}

	_currentFrame(): Frame | null {
		const bridge = this._bridge;
		const id = bridge.frameId;
		if (!id) {
			return null;
		}
		if (this._frame?.id !== id) {
			const width = bridge.videoWidth;
			const height = bridge.videoHeight;
			const pixels = bridge.readPixels();
			if (!pixels || pixels.length !== width * height * 4) {
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

	/** WebGL `texImage2D` / `texSubImage2D` with this video: `(native, context, target, level, internalformat, ...)`. */
	getCurrentFrame(context?: any) {
		const native = arguments[0];
		const frame = this._currentFrame();
		if (!frame || typeof native?.texImage2D !== 'function') {
			return;
		}
		native.texImage2D(arguments[2], arguments[3], rgbaInternalFormat(arguments[4]), frame.width, frame.height, 0, GL_RGBA, GL_UNSIGNED_BYTE, frame.pixels);
	}

	getFrameForTexImage3D(nativeCtx: any, ctx: any, target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number) {
		const frame = this._currentFrame();
		// One frame is one slice.
		if (!frame || depth !== 1) {
			return;
		}
		nativeCtx.texImage3D(target, level, rgbaInternalFormat(internalformat), frame.width, frame.height, 1, border, GL_RGBA, GL_UNSIGNED_BYTE, frame.pixels);
	}

	getFrameForTexSubImage3D(nativeCtx: any, ctx: any, target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number) {
		const frame = this._currentFrame();
		if (!frame || depth !== 1) {
			return;
		}
		nativeCtx.texSubImage3D(target, level, xoffset, yoffset, zoffset, frame.width, frame.height, 1, GL_RGBA, GL_UNSIGNED_BYTE, frame.pixels);
	}

	/** 2D `drawImage(video, ...)`; `args` are drawImage's arguments. */
	drawImageFrame(context2d: any, args: any[]) {
		if (this._drawGPUFrame(context2d, args)) {
			return;
		}
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

	/** Without a readback: the frame is drawn on the canvas's (or its render thread's) D3D12 device. */
	private _drawGPUFrame(context2d: any, args: any[]): boolean {
		const native = context2d?.context;
		const bridge = this._bridge;
		if (typeof native?.__drawD3DSharedFrame !== 'function' || !bridge.sharesFrames || !bridge.frameId) {
			return false;
		}
		const frame = bridge.gpuFrame();
		if (!frame) {
			return false;
		}
		const { width, height } = frame;
		let rect: number[];
		if (args.length === 3) {
			rect = [0, 0, width, height, args[1], args[2], width, height];
		} else if (args.length === 5) {
			rect = [0, 0, width, height, args[1], args[2], args[3], args[4]];
		} else if (args.length === 9) {
			rect = args.slice(1, 9);
		} else {
			frame.close();
			return false;
		}
		try {
			return native.__drawD3DSharedFrame(frame.address, width, height, ...rect);
		} finally {
			frame.close();
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

	/** `device`: the WebGPU device's adapter LUID (`__frameDevice`). */
	supportsGPUFrames(device: number): boolean {
		const bridge = this._bridge;
		return !!device && bridge.sharesFrames && bridge.adapterLuid === device;
	}

	/** null when no frame was decoded since the last one handed out; `close()` it once the upload is issued. */
	getGPUFrameTexture(device: number): any {
		const bridge = this._bridge;
		const id = bridge.frameId;
		if (!id || id === this._gpuFrameId || !this.supportsGPUFrames(device)) {
			return null;
		}
		const frame = bridge.gpuFrame();
		if (!frame) {
			return null;
		}
		this._gpuFrameId = id;
		return {
			texturePointer: frame.address,
			width: frame.width,
			height: frame.height,
			close: () => frame.close(),
		};
	}

	//@ts-ignore
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

	set controls(value: boolean) {
		this._controls = booleanConverter(value as any);
		this._syncControls();
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
