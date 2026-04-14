export type VideoPixelFormat = 'I420' | 'I420A' | 'I422' | 'I444' | 'NV12' | 'RGBA' | 'RGBX' | 'BGRA' | 'BGRX';
export type VideoColorPrimaries = 'bt709' | 'bt470bg' | 'smpte170m' | 'bt2020' | 'smpte432';
export type VideoTransferCharacteristics = 'bt709' | 'smpte170m' | 'iec61966-2-1' | 'linear' | 'pq' | 'hlg';
export type VideoMatrixCoefficients = 'rgb' | 'bt709' | 'bt470bg' | 'smpte170m' | 'bt2020-ncl';

export interface VideoColorSpaceInit {
	primaries?: VideoColorPrimaries | null;
	transfer?: VideoTransferCharacteristics | null;
	matrix?: VideoMatrixCoefficients | null;
	fullRange?: boolean | null;
}

export class VideoColorSpace {
	readonly primaries: VideoColorPrimaries | null;
	readonly transfer: VideoTransferCharacteristics | null;
	readonly matrix: VideoMatrixCoefficients | null;
	readonly fullRange: boolean | null;

	constructor(init: VideoColorSpaceInit = {}) {
		this.primaries = init.primaries ?? null;
		this.transfer = init.transfer ?? null;
		this.matrix = init.matrix ?? null;
		this.fullRange = init.fullRange ?? null;
	}

	toJSON(): VideoColorSpaceInit {
		return {
			primaries: this.primaries,
			transfer: this.transfer,
			matrix: this.matrix,
			fullRange: this.fullRange,
		};
	}
}

export class DOMRectReadOnly {
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;

	constructor(x = 0, y = 0, width = 0, height = 0) {
		this.x = x;
		this.y = y;
		this.width = width;
		this.height = height;
	}

	get top(): number {
		return this.y;
	}
	get right(): number {
		return this.x + this.width;
	}
	get bottom(): number {
		return this.y + this.height;
	}
	get left(): number {
		return this.x;
	}

	toJSON() {
		return {
			x: this.x,
			y: this.y,
			width: this.width,
			height: this.height,
			top: this.top,
			right: this.right,
			bottom: this.bottom,
			left: this.left,
		};
	}
}

export class DOMException extends Error {
	constructor(message = '', name = 'Error') {
		super(message);
		this.name = name;
	}
}

export interface VideoFrameInit {
	duration?: number | null;
	timestamp?: number;
	alpha?: 'keep' | 'discard';
	visibleRect?: RectInit;
	displayWidth?: number;
	displayHeight?: number;
	colorSpace?: VideoColorSpaceInit;
}

export interface VideoFrameBufferInit {
	format: VideoPixelFormat;
	codedWidth: number;
	codedHeight: number;
	timestamp: number;
	duration?: number;
	layout?: PlaneLayout[];
	visibleRect?: RectInit;
	displayWidth?: number;
	displayHeight?: number;
	colorSpace?: VideoColorSpaceInit;
}

export interface VideoFrameCopyToOptions {
	rect?: RectInit;
	layout?: PlaneLayout[];
}

export interface PlaneLayout {
	offset: number;
	stride: number;
}

export interface RectInit {
	x?: number;
	y?: number;
	width?: number;
	height?: number;
}

export interface _VideoFrameData {
	pixelData: Uint8Array | null;
	codedWidth: number;
	codedHeight: number;
	displayWidth: number;
	displayHeight: number;
	timestamp: number;
	duration?: number | null;
	format: VideoPixelFormat;
	colorSpace?: VideoColorSpaceInit;
}

export const _INTERNAL = Symbol('VideoFrame.internal');

const SRGB_COLOR_SPACE: VideoColorSpaceInit = {
	primaries: 'bt709',
	transfer: 'iec61966-2-1',
	matrix: 'rgb',
	fullRange: true,
};

export abstract class VideoFrameBase {
	protected _closed = false;
	protected _pixelData: Uint8Array | null;

	readonly format: VideoPixelFormat | null;
	readonly codedWidth: number;
	readonly codedHeight: number;
	readonly codedRect: DOMRectReadOnly;
	readonly visibleRect: DOMRectReadOnly;
	readonly displayWidth: number;
	readonly displayHeight: number;
	readonly duration?: number | null;
	readonly timestamp: number;
	readonly colorSpace: VideoColorSpace;

	protected constructor(data: _VideoFrameData) {
		this.format = data.format;
		this.codedWidth = data.codedWidth;
		this.codedHeight = data.codedHeight;
		this.codedRect = new DOMRectReadOnly(0, 0, data.codedWidth, data.codedHeight);
		this.visibleRect = new DOMRectReadOnly(0, 0, data.codedWidth, data.codedHeight);
		this.displayWidth = data.displayWidth;
		this.displayHeight = data.displayHeight;
		this.duration = data.duration;
		this.timestamp = data.timestamp;
		this.colorSpace = new VideoColorSpace(data.colorSpace ?? SRGB_COLOR_SPACE);
		this._pixelData = data.pixelData;
	}

	protected abstract _doClone(): VideoFrameBase;

	/**
	 * Direct read-only access to the underlying RGBA pixel buffer.
	 *
	 * This is a non-standard extension for performance-critical consumers such as
	 * WebGL uploaders that can pass the Uint8Array straight to `texImage2D` without
	 * the extra copy that `copyTo()` would incur.  The returned array is a live view
	 * of the frame's backing store — do not mutate it, and do not hold a reference
	 * after calling `close()`.
	 */
	get pixelData(): Uint8Array | null {
		this._assertOpen();
		return this._pixelData;
	}

	allocationSize(options?: VideoFrameCopyToOptions): number {
		this._assertOpen();
		const rect = this._resolveRect(options?.rect);
		return rect.width * rect.height * 4; // RGBA = 4 bytes/pixel
	}

	copyTo(destination: ArrayBuffer | ArrayBufferView, options?: VideoFrameCopyToOptions): Promise<PlaneLayout[]> {
		try {
			this._assertOpen();

			if (!this._pixelData) {
				return Promise.reject(new DOMException('No pixel data available — frame may not have been captured yet', 'InvalidStateError'));
			}

			const rect = this._resolveRect(options?.rect);
			const stride = rect.width * 4;
			const needed = stride * rect.height;

			let dest: Uint8Array;
			if (destination instanceof ArrayBuffer) {
				if (destination.byteLength < needed) {
					return Promise.reject(new RangeError(`Destination buffer too small: need ${needed} bytes, have ${destination.byteLength}`));
				}
				dest = new Uint8Array(destination);
			} else {
				const view = destination as ArrayBufferView;
				if (view.byteLength < needed) {
					return Promise.reject(new RangeError(`Destination buffer too small: need ${needed} bytes, have ${view.byteLength}`));
				}
				dest = new Uint8Array(view.buffer, view.byteOffset, view.byteLength);
			}

			const srcStride = this.codedWidth * 4;
			const isFullFrame = rect.x === 0 && rect.y === 0 && rect.width === this.codedWidth && rect.height === this.codedHeight;

			if (isFullFrame) {
				dest.set(this._pixelData.subarray(0, needed));
			} else {
				for (let row = 0; row < rect.height; row++) {
					const srcOff = (rect.y + row) * srcStride + rect.x * 4;
					const dstOff = row * stride;
					dest.set(this._pixelData.subarray(srcOff, srcOff + stride), dstOff);
				}
			}

			return Promise.resolve([{ offset: 0, stride }]);
		} catch (err) {
			return Promise.reject(err);
		}
	}

	clone(): this {
		this._assertOpen();
		return this._doClone() as this;
	}

	close(): void {
		if (this._closed) return;
		this._closed = true;
		this._pixelData = null;
	}

	protected _assertOpen(): void {
		if (this._closed) {
			throw new DOMException('VideoFrame is closed', 'InvalidStateError');
		}
	}

	private _resolveRect(rect?: RectInit | null): { x: number; y: number; width: number; height: number } {
		if (!rect) {
			return { x: 0, y: 0, width: this.codedWidth, height: this.codedHeight };
		}
		const x = Math.max(0, Math.floor(rect.x ?? 0));
		const y = Math.max(0, Math.floor(rect.y ?? 0));
		const width = Math.min(this.codedWidth - x, Math.max(0, Math.floor(rect.width ?? this.codedWidth)));
		const height = Math.min(this.codedHeight - y, Math.max(0, Math.floor(rect.height ?? this.codedHeight)));
		return { x, y, width, height };
	}
}

export function bgraToRgba(src: Uint8Array): Uint8Array {
	const dst = new Uint8Array(src.length);
	const len = src.length;
	for (let i = 0; i < len; i += 4) {
		dst[i] = src[i + 2]; // R ← B
		dst[i + 1] = src[i + 1]; // G
		dst[i + 2] = src[i]; // B ← R
		dst[i + 3] = src[i + 3]; // A
	}
	return dst;
}
