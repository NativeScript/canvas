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

export declare class VideoColorSpace {
	readonly primaries: VideoColorPrimaries | null;
	readonly transfer: VideoTransferCharacteristics | null;
	readonly matrix: VideoMatrixCoefficients | null;
	readonly fullRange: boolean | null;
	constructor(init?: VideoColorSpaceInit);
	toJSON(): VideoColorSpaceInit;
}

export interface RectInit {
	x?: number;
	y?: number;
	width?: number;
	height?: number;
}

export declare class DOMRectReadOnly {
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
	readonly top: number;
	readonly right: number;
	readonly bottom: number;
	readonly left: number;
	constructor(x?: number, y?: number, width?: number, height?: number);
	toJSON(): { x: number; y: number; width: number; height: number; top: number; right: number; bottom: number; left: number };
}

export interface PlaneLayout {
	offset: number;
	stride: number;
}

export interface VideoFrameInit {
	duration?: number;
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

export declare class VideoFrame {
	readonly format: VideoPixelFormat | null;
	readonly codedWidth: number;
	readonly codedHeight: number;
	readonly codedRect: DOMRectReadOnly;
	readonly visibleRect: DOMRectReadOnly;
	readonly displayWidth: number;
	readonly displayHeight: number;
	readonly duration: number | null;
	readonly timestamp: number;
	readonly colorSpace: VideoColorSpace;

	constructor(data: ArrayBuffer | ArrayBufferView, init: VideoFrameBufferInit);

	constructor(source: object, init?: VideoFrameInit);

	static fromVideo(source: object, init?: VideoFrameInit): VideoFrame | null;

	allocationSize(options?: VideoFrameCopyToOptions): number;

	copyTo(destination: ArrayBuffer | ArrayBufferView, options?: VideoFrameCopyToOptions): Promise<PlaneLayout[]>;

	clone(): VideoFrame;

	close(): void;
}
