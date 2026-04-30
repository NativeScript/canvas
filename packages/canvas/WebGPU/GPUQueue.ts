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
				//@ts-ignore
				width: 0,
				height: 1,
				depthOrArrayLayers: 1,
				...copySize,
			};
		}

		this[native_].copyExternalImageToTexture(src, dst, size);
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

	writeBuffer(buffer: GPUBuffer, bufferOffset: number, data: BufferSource | Array<number>, dataOffset: number = 0, size?: number) {
		// WebGPU spec: when `data` is a TypedArray, `dataOffset` and `size` are in
		// elements. When `data` is an ArrayBuffer or DataView, they are in bytes.
		// Native expects bytes — so we normalize here.
		let view: ArrayBufferView;
		if (Array.isArray(data)) {
			view = new Uint8Array(data);
		} else if (data instanceof ArrayBuffer) {
			view = new Uint8Array(data);
		} else {
			view = data as ArrayBufferView;
		}

		const isTyped = ArrayBuffer.isView(view) && !(view instanceof DataView);
		const elemSize = isTyped ? ((view as any).BYTES_PER_ELEMENT ?? 1) : 1;

		if (dataOffset < 0) dataOffset = 0;
		const dataByteOffset = dataOffset * elemSize;
		const availableBytes = Math.max(0, view.byteLength - dataByteOffset);
		let writeBytes = size === undefined ? availableBytes : size * elemSize;
		if (writeBytes < 0) writeBytes = 0;
		if (writeBytes > availableBytes) writeBytes = availableBytes;

		if (writeBytes === 0) return;

		// Build a contiguous byte view at the requested offset (no copy).
		const u8 = new Uint8Array(view.buffer, view.byteOffset + dataByteOffset, writeBytes);

		// wgpu requires 4-byte alignment of bufferOffset and total size.
		const needsPad = (bufferOffset & 3) !== 0 || (writeBytes & 3) !== 0;
		if (needsPad) {
			const padded = new Uint8Array(this.alignTo4(writeBytes));
			padded.set(u8);
			this[native_].writeBuffer(buffer?.[native_], bufferOffset ?? 0, padded, 0, padded.byteLength);
		} else {
			this[native_].writeBuffer(buffer?.[native_], bufferOffset ?? 0, u8, 0, writeBytes);
		}

		void view;
	}

	writeTexture(destination: GPUImageCopyTexture, data: BufferSource, dataLayout: GPUImageDataLayout, size: GPUExtent3D) {
		const dst: GPUImageCopyTexture = {
			...destination,
			texture: destination.texture[native_],
		};
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
