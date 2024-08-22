import { ImageAsset } from '../ImageAsset';
import { native_ } from './Constants';
import { GPUBuffer } from './GPUBuffer';
import { GPUCommandBuffer } from './GPUCommandBuffer';
import { GPUImageCopyExternalImage, GPUImageCopyTexture, GPUImageCopyTextureTagged, GPUImageDataLayout } from './Interfaces';
import { GPUExtent3D } from './Types';
export class GPUQueue {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	static fromNative(value) {
		if (value) {
			const ret = new GPUQueue();
			ret[native_] = value;
			return ret;
		}
		return null;
	}

	copyExternalImageToTexture(source: GPUImageCopyExternalImage, destination: GPUImageCopyTextureTagged, copySize: GPUExtent3D) {
		destination.texture = destination.texture[native_];
		if (source.source) {
			if (source.source instanceof ImageBitmap) {
				source.source = (source.source as any).native;
			} else if (source.source instanceof ImageData) {
				source.source = (source.source as any).native;
			} else if (source.source instanceof ImageAsset) {
				source.source = source.source.native;
			} else if (typeof source.source.tagName === 'string' && (source.source.tagName === 'IMG' || source.source.tagName === 'IMAGE')) {
				if (source.source._asset instanceof ImageAsset) {
					source.source = source.source._asset.native;
				}
			}
		}

		if (Array.isArray(copySize)) {
			copySize = {
				width: copySize[0],
				height: copySize[1] ?? 1,
				depthOrArrayLayers: copySize[2] ?? 1,
			};
		} else {
			copySize = {
				width: 0,
				height: 1,
				depthOrArrayLayers: 1,
				...copySize,
			};
		}

		this[native_].copyExternalImageToTexture(source, destination, copySize);
	}

	submit(commands: GPUCommandBuffer[]) {
		if (Array.isArray(commands)) {
			this[native_].submit(
				commands.map((command) => {
					return command[native_];
				})
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

	writeBuffer(buffer: GPUBuffer, bufferOffset: number, data: BufferSource | Array<number>, dataOffset?: number, size?: number) {
		if (Array.isArray(data)) {
			data = new Uint8Array(data);
		} else if (ArrayBuffer.isView(data)) {
			data = new Uint8Array(data.buffer, data.byteOffset, data.byteLength);
		} else if ((data as any) instanceof ArrayBuffer) {
			data = new Uint8Array(data);
		}

		this[native_].writeBuffer(buffer?.[native_], bufferOffset ?? 0, data, dataOffset ?? 0, size ?? -1);
	}

	writeTexture(destination: GPUImageCopyTexture, data: BufferSource, dataLayout: GPUImageDataLayout, size: GPUExtent3D) {
		destination.texture = destination.texture[native_];
		if (Array.isArray(size)) {
			size = {
				width: size[0],
				height: size[1] ?? 1,
				depthOrArrayLayers: size[2] ?? 1,
			};
		}
		this[native_].writeTexture(destination, data, dataLayout, size);
	}
}
