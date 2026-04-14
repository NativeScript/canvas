import { Canvas } from '../Canvas';
import { ImageAsset } from '../ImageAsset';
import { native_ } from './Constants';
import { GPUBuffer } from './GPUBuffer';
import { GPUCommandBuffer } from './GPUCommandBuffer';
import { GPUImageCopyExternalImage, GPUImageCopyTexture, GPUImageCopyTextureTagged, GPUImageDataLayout } from './Interfaces';
import { GPUExtent3D } from './Types';
export class GPUQueue {
	[native_]: any;

	get label() {
		return this[native_]?.label ?? '';
	}

	static fromNative(value: any) {
		if (value) {
			const ret = new GPUQueue();
			ret[native_] = value;
			return ret;
		}
		return null;
	}

	copyExternalImageToTexture(source: GPUImageCopyExternalImage, destination: GPUImageCopyTextureTagged, copySize: GPUExtent3D) {
		const src: GPUImageCopyExternalImage = {
			source: undefined,
		};

		// Hold explicit GC roots for all JS wrapper objects until after the native call.
		// Extracting .native (a raw pointer) drops the only reference V8 sees, making the
		// wrapper eligible for collection mid-call in aggressive GC environments.
		let _keepAlive: unknown;

		if (source.source) {
			if (source.source instanceof ImageBitmap) {
				_keepAlive = source.source;
				src.source = (source.source as any).native;
			} else if (source.source instanceof ImageData) {
				_keepAlive = source.source;
				src.source = (source.source as any).native;
			} else if (source.source instanceof ImageAsset) {
				_keepAlive = source.source;
				src.source = source.source.native;
			} else if (typeof source.source.tagName === 'string' && (source.source.tagName === 'IMG' || source.source.tagName === 'IMAGE')) {
				if (source.source._asset instanceof ImageAsset) {
					_keepAlive = source.source._asset;
					src.source = source.source._asset.native;
				}
			} else if (typeof source.source.tagName === 'string' && (source.source.tagName === 'VID' || source.source.tagName === 'VIDEO') && source.source._video && typeof source.source._video.getVideoFrameData === 'function') {
				const frame = source.source._video.getVideoFrameData();
				if (frame) {
					src.source = frame;
				}
			} else if (source.source && typeof source.source.getVideoFrameData === 'function') {
				const frame = source.source.getVideoFrameData();
				if (frame) {
					src.source = frame;
				}
			} else if (source.source?._type === '2d' || source.source?._type?.indexOf('webgl') > -1 || source.source?._type === 'webgpu') {
				_keepAlive = source.source;
				src.source = (source.source as any).native;
			} else if (source.source instanceof Canvas) {
				_keepAlive = source.source;
				src.source = source.source.native;
			} else if (typeof source.source.tagName === 'string' && source.source.tagName === 'CANVAS') {
				if (source.source._canvas instanceof Canvas) {
					_keepAlive = source.source._canvas;
					src.source = source.source._canvas.native;
				}
			}
		}

		if (typeof source.flipY === 'boolean') {
			src.flipY = source.flipY;
		}

		if (source.origin) {
			src.origin = { ...source.origin };
		}

		const dst: GPUImageCopyTextureTagged = {
			texture: destination.texture[native_],
		};

		if (destination.aspect) {
			dst.aspect = destination.aspect;
		}

		if (destination.colorSpace) {
			dst.colorSpace = destination.colorSpace;
		}

		if (typeof destination.mipLevel === 'number') {
			dst.mipLevel = destination.mipLevel;
		}

		if (destination.origin) {
			dst.origin = { ...destination.origin };
		}

		if (typeof destination.premultipliedAlpha === 'boolean') {
			dst.premultipliedAlpha = destination.premultipliedAlpha;
		}

		let size: GPUExtent3D;

		if (Array.isArray(copySize)) {
			size = {
				width: copySize[0],
				height: copySize[1] ?? 1,
				depthOrArrayLayers: copySize[2] ?? 1,
			};
		} else {
			size = {
				//@ts-ignore Allow spreading GPUExtent3D object, which may have optional properties.
				width: 0,
				height: 1,
				depthOrArrayLayers: 1,
				...copySize,
			};
		}

		this[native_].copyExternalImageToTexture(src, dst, size);
		// Prevent dead-variable elimination from hoisting the GC root drop before the call.
		void _keepAlive;
	}

	submit(commands: GPUCommandBuffer[]) {
		if (Array.isArray(commands)) {
			this[native_].submit(
				commands.map((command) => {
					return command[native_];
				}),
			);
		}
	}

	onSubmittedWorkDone() {
		return new Promise<void>((resolve, reject) => {
			this[native_].onSubmittedWorkDone(() => {
				resolve();
			});
		});
	}

	private alignTo4(n: number) {
		return (n + 3) & ~3;
	}

	private padDataForWriteBuffer(data: ArrayBuffer | Uint8Array, size: number, dataOffset: number = 0): Uint8Array {
		// Normalize input to Uint8Array
		let src: Uint8Array;
		if (data instanceof Uint8Array) {
			src = data;
		} else {
			src = new Uint8Array(data as ArrayBuffer);
		}
		// Clamp size to available data
		const available = src.byteLength - dataOffset;
		const copySize = Math.min(size, available);
		const padded = new Uint8Array(this.alignTo4(size));
		padded.set(src.subarray(dataOffset, dataOffset + copySize));
		// The extra bytes are already zero-initialized
		return padded;
	}

	writeBuffer(buffer: GPUBuffer, bufferOffset: number, data: BufferSource | Array<number>, dataOffset: number = 0, size?: number) {
		// Only Array<number> needs conversion
		let nativeData: Uint8Array | ArrayBuffer | BufferSource;
		if (Array.isArray(data)) {
			nativeData = new Uint8Array(data);
		} else {
			nativeData = data as BufferSource;
		}

		// Default size to available bytes from dataOffset
		let actualSize = size;
		let totalLength = 0;
		if (nativeData instanceof Uint8Array) {
			totalLength = nativeData.byteLength;
		} else if (nativeData instanceof ArrayBuffer) {
			totalLength = nativeData.byteLength;
		} else if (ArrayBuffer.isView(nativeData)) {
			totalLength = (nativeData as ArrayBufferView).byteLength;
		}
		if (actualSize == null) {
			actualSize = totalLength - dataOffset;
		}
		// Defensive: clamp size
		if (actualSize < 0) actualSize = 0;
		if (dataOffset < 0) dataOffset = 0;

		if (actualSize % 4 !== 0 || bufferOffset % 4 !== 0 || dataOffset % 4 !== 0) {
			// Pad/correct as needed
			const paddedData = this.padDataForWriteBuffer(nativeData as any, actualSize, dataOffset);
			this[native_].writeBuffer(buffer?.[native_], bufferOffset ?? 0, paddedData, 0, this.alignTo4(actualSize));
		} else {
			this[native_].writeBuffer(buffer?.[native_], bufferOffset ?? 0, nativeData, dataOffset ?? 0, actualSize);
		}
		// Keep nativeData alive past the call
		void nativeData;
	}

	writeTexture(destination: GPUImageCopyTexture, data: BufferSource, dataLayout: GPUImageDataLayout, size: GPUExtent3D) {
		const dst: GPUImageCopyTexture = {
			...destination,
			texture: destination.texture[native_],
		};
		// Must check Array.isArray on `size` before spreading — spreading an array produces
		// a plain object with numeric keys, so the check would always be false afterwards.
		let ext: GPUExtent3D;
		if (Array.isArray(size)) {
			ext = {
				width: size[0],
				height: size[1] ?? 1,
				depthOrArrayLayers: size[2] ?? 1,
			};
		} else {
			ext = { ...size };
		}
		this[native_].writeTexture(dst, data, dataLayout, ext);
	}
}
