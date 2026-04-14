import { Utils } from '@nativescript/core';
import { VideoFrameBase, VideoColorSpace, DOMRectReadOnly, VideoFrameInit, VideoFrameBufferInit, VideoFrameCopyToOptions, PlaneLayout, RectInit, VideoPixelFormat, VideoColorSpaceInit, _VideoFrameData, _INTERNAL, bgraToRgba } from './common';

export { VideoColorSpace, DOMRectReadOnly, _INTERNAL };
export type { VideoPixelFormat, VideoFrameInit, VideoFrameBufferInit, VideoFrameCopyToOptions, PlaneLayout, RectInit, VideoColorSpaceInit };

declare const NSCRender: any;
declare const interop: any;

export class VideoFrame extends VideoFrameBase {
	constructor(source: ArrayBuffer | ArrayBufferView, init: VideoFrameBufferInit);
	constructor(source: object, init?: VideoFrameInit);
	constructor(source: any, init?: any) {
		if (source === _INTERNAL) {
			super(init as _VideoFrameData);
			return;
		}

		if (source instanceof ArrayBuffer || ArrayBuffer.isView(source)) {
			super(VideoFrame._dataFromBuffer(source, init as VideoFrameBufferInit));
			return;
		}
		super(VideoFrame._dataFromVideoSource(source, init as VideoFrameInit));
	}

	static fromVideo(source: any, init?: VideoFrameInit): VideoFrame | null {
		try {
			const data = VideoFrame._dataFromVideoSource(source, init);
			// Return null when no pixel data is available yet (e.g. video not loaded,
			// AVPlayerItemVideoOutput has no pixel buffer at the current time).
			// Callers can treat null as "not ready" and fall back to drawImage.
			if (!data.pixelData) return null;
			return new VideoFrame(_INTERNAL as any, data);
		} catch (err: any) {
			if (__IOS__) {
				console.error('[VideoFrame] fromVideo:', err?.message ?? err);
			}
			return null;
		}
	}

	private static _dataFromBuffer(source: ArrayBuffer | ArrayBufferView, init: VideoFrameBufferInit): _VideoFrameData {
		VideoFrame._assertBufferInit(init);

		const buf = ArrayBuffer.isView(source) ? new Uint8Array((source as ArrayBufferView).buffer, (source as ArrayBufferView).byteOffset, (source as ArrayBufferView).byteLength) : new Uint8Array(source as ArrayBuffer);

		const pixelData = init.format === 'BGRA' || init.format === 'BGRX' ? bgraToRgba(buf) : new Uint8Array(buf);

		return {
			pixelData,
			codedWidth: init.codedWidth,
			codedHeight: init.codedHeight,
			displayWidth: init.displayWidth ?? init.codedWidth,
			displayHeight: init.displayHeight ?? init.codedHeight,
			timestamp: init.timestamp,
			duration: init.duration ?? null,
			format: 'RGBA',
			colorSpace: init.colorSpace,
		};
	}

	private static _dataFromVideoSource(source: any, init?: VideoFrameInit): _VideoFrameData {
		const video = source?._video ?? source;
		const helper = video?.helper;

		if (!helper) {
			throw new TypeError('[VideoFrame] Source must be an HTMLVideoElement or a canvas-media Video instance');
		}

		if (!helper.isInForeground) {
			throw new DOMException('[VideoFrame] Cannot capture a frame while the video is in the background', 'InvalidStateError');
		}

		if (!helper.player || !helper.assetOutput) {
			throw new DOMException('[VideoFrame] Video is not ready — wait for the loadeddata event before capturing frames', 'InvalidStateError');
		}

		const timestampUs = typeof init?.timestamp === 'number' ? init.timestamp : Math.round((helper.currentTime as number) * 1_000_000);

		const frameDict: NSDictionary<any, any> = NSCRender.getVideoFrameData(helper.player, helper.assetOutput, helper.videoSize);

		let pixelData: Uint8Array | null = null;
		let codedWidth = 0;
		let codedHeight = 0;

		if (frameDict) {
			const nsData = frameDict.objectForKey('data');
			const wNum = frameDict.objectForKey('width');
			const hNum = frameDict.objectForKey('height');

			codedWidth = typeof wNum === 'number' ? wNum : ((wNum?.intValue as number) ?? 0);
			codedHeight = typeof hNum === 'number' ? hNum : ((hNum?.intValue as number) ?? 0);

			if (nsData && codedWidth > 0 && codedHeight > 0) {
				pixelData = new Uint8Array(interop.bufferFromData(nsData));
			}
		}

		if (codedWidth === 0 || codedHeight === 0) {
			const size = helper.videoSize;
			codedWidth = size ? (size.width as number) | 0 : 0;
			codedHeight = size ? (size.height as number) | 0 : 0;
		}

		return {
			pixelData,
			codedWidth,
			codedHeight,
			displayWidth: init?.displayWidth ?? codedWidth,
			displayHeight: init?.displayHeight ?? codedHeight,
			timestamp: timestampUs,
			duration: init?.duration ?? null,
			format: 'RGBA',
			colorSpace: init?.colorSpace,
		};
	}

	private static _assertBufferInit(init: VideoFrameBufferInit): void {
		if (!init?.format) {
			throw new TypeError('VideoFrame: init.format is required for the buffer constructor');
		}
		if (!init?.codedWidth || init.codedWidth <= 0) {
			throw new TypeError('VideoFrame: init.codedWidth must be a positive integer');
		}
		if (!init?.codedHeight || init.codedHeight <= 0) {
			throw new TypeError('VideoFrame: init.codedHeight must be a positive integer');
		}
		if (typeof init?.timestamp !== 'number' || !isFinite(init.timestamp)) {
			throw new TypeError('VideoFrame: init.timestamp must be a finite number (microseconds)');
		}
	}

	protected _doClone(): VideoFrame {
		const pixelCopy = this._pixelData ? new Uint8Array(this._pixelData) : null;
		const data: _VideoFrameData = {
			pixelData: pixelCopy,
			codedWidth: this.codedWidth,
			codedHeight: this.codedHeight,
			displayWidth: this.displayWidth,
			displayHeight: this.displayHeight,
			timestamp: this.timestamp,
			duration: this.duration,
			format: this.format as VideoPixelFormat,
			colorSpace: this.colorSpace.toJSON(),
		};
		return new VideoFrame(_INTERNAL as any, data as any);
	}
}
