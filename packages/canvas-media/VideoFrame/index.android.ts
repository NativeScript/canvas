import { VideoFrameBase, VideoColorSpace, DOMRectReadOnly, VideoFrameInit, VideoFrameBufferInit, VideoFrameCopyToOptions, PlaneLayout, RectInit, VideoPixelFormat, VideoColorSpaceInit, _VideoFrameData, _INTERNAL, bgraToRgba } from './common';

export { VideoColorSpace, DOMRectReadOnly, _INTERNAL };
export type { VideoPixelFormat, VideoFrameInit, VideoFrameBufferInit, VideoFrameCopyToOptions, PlaneLayout, RectInit, VideoColorSpaceInit };

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
			// setupBitmapSurface hasn't fired, or no frame has been decoded).
			// Callers can treat null as "not ready" and fall back to drawImage.
			if (!data.pixelData) return null;
			return new VideoFrame(_INTERNAL as any, data);
		} catch (err: any) {
			if (__ANDROID__) {
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
		const instance = video?._instance;

		if (!instance) {
			throw new TypeError('[VideoFrame] Source must be an HTMLVideoElement or a canvas-media Video instance');
		}

		const currentTimeSec: number = instance.getCurrentTime();
		const timestampUs = typeof init?.timestamp === 'number' ? init.timestamp : Math.round(currentTimeSec * 1_000_000);

		const rgbaBuffer = instance.getRgbaBuffer();

		let pixelData: Uint8Array | null = null;
		let codedWidth = 0;
		let codedHeight = 0;

		if (rgbaBuffer) {
			codedWidth = instance.getLastRgbaWidth() as number;
			codedHeight = instance.getLastRgbaHeight() as number;
			if (codedWidth > 0 && codedHeight > 0) {
				pixelData = new Uint8Array((ArrayBuffer as any).from(rgbaBuffer));
			}
		}

		if (codedWidth === 0 || codedHeight === 0) {
			codedWidth = (video._videoWidth as number) ?? 0;
			codedHeight = (video._videoHeight as number) ?? 0;
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
