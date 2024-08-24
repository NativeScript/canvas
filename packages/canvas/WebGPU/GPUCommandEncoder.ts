import { native_ } from './Constants';
import { GPUBuffer } from './GPUBuffer';
import { GPUCommandBuffer } from './GPUCommandBuffer';
import { GPUComputePassEncoder } from './GPUComputePassEncoder';
import { GPUQuerySet } from './GPUQuerySet';
import { GPURenderPassEncoder } from './GPURenderPassEncoder';
import { GPUImageCopyBuffer, GPUImageCopyTexture, GPURenderPassColorAttachment, GPURenderPassDepthStencilAttachment, GPURenderPassDescriptor, GPURenderPassTimestampWrites } from './Interfaces';
import { GPUExtent3D } from './Types';
import { parseComputePassDescriptor, parseRenderPassDescriptor } from './Utils';

export class GPUCommandEncoder {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	static fromNative(encoder) {
		if (encoder) {
			const ret = new GPUCommandEncoder();
			ret[native_] = encoder;
			return ret;
		}
		return null;
	}

	beginComputePass(descriptor?: {
		label?: string;
		timestampWrites?: {
			beginningOfPassWriteIndex: number;
			endOfPassWriteIndex: number;
			querySet: GPUQuerySet;
		};
	}) {
		const desc = parseComputePassDescriptor(descriptor);
		return GPUComputePassEncoder.fromNative(this[native_].beginComputePass(desc));
	}

	beginRenderPass(descriptor: GPURenderPassDescriptor) {
		const desc = parseRenderPassDescriptor(descriptor);
		const passEncoder = this[native_].beginRenderPass(desc);
		return GPURenderPassEncoder.fromNative(passEncoder);
	}

	clearBuffer(buffer: GPUBuffer, offset?: number, size?: number) {
		this[native_].clearBuffer(buffer[native_], offset ?? -1, size ?? -1);
	}

	copyBufferToBuffer(source: GPUBuffer, sourceOffset: number, destination: GPUBuffer, destinationOffset: number, size: number) {
		this[native_].copyBufferToBuffer(source[native_], sourceOffset, destination[native_], destinationOffset, size);
	}

	copyBufferToTexture(source: GPUImageCopyBuffer, destination: GPUImageCopyTexture, copySize: GPUExtent3D) {
		const src: GPUImageCopyBuffer = {
			buffer: source.buffer[native_],
			bytesPerRow: source.bytesPerRow,
		};

		if (typeof source.offset === 'number') {
			src.offset = source.offset;
		}

		if (typeof source.rowsPerImage === 'number') {
			src.rowsPerImage = source.rowsPerImage;
		}

		const dst: GPUImageCopyTexture = {
			texture: destination.texture[native_],
		};

		if (destination.aspect) {
			dst.aspect = destination.aspect;
		}

		if (typeof destination.mipLevel === 'number') {
			dst.mipLevel = destination.mipLevel;
		}

		if (destination.origin) {
			dst.origin = destination.origin;
		}

		let size: GPUExtent3D;

		if (Array.isArray(copySize)) {
			size = {
				width: copySize[0],
				height: copySize[1] ?? 1,
				depthOrArrayLayers: copySize[2] ?? 1,
			};
		} else {
			size = { ...copySize };
		}

		this[native_].copyBufferToTexture(src, dst, size);
	}

	copyTextureToBuffer(source: GPUImageCopyTexture, destination: GPUImageCopyBuffer, copySize: GPUExtent3D) {
		const src: GPUImageCopyTexture = {
			texture: source.texture[native_],
		};

		if (source.aspect) {
			src.aspect = source.aspect;
		}

		if (typeof source.mipLevel === 'number') {
			src.mipLevel = source.mipLevel;
		}

		if (source.origin) {
			src.origin = source.origin;
		}

		const dst: GPUImageCopyBuffer = {
			buffer: destination.buffer[native_],
			bytesPerRow: destination.bytesPerRow,
		};

		if (typeof destination.offset === 'number') {
			dst.offset = destination.offset;
		}

		if (typeof destination.rowsPerImage === 'number') {
			dst.rowsPerImage = destination.rowsPerImage;
		}

		let size: GPUExtent3D;

		if (Array.isArray(copySize)) {
			size = {
				width: copySize[0],
				height: copySize[1] ?? 1,
				depthOrArrayLayers: copySize[2] ?? 1,
			};
		} else {
			size = { ...copySize };
		}

		this[native_].copyTextureToBuffer(src, dst, size);
	}

	copyTextureToTexture(source: GPUImageCopyTexture, destination: GPUImageCopyTexture, copySize: GPUExtent3D) {
		const src: GPUImageCopyTexture = {
			texture: source.texture[native_],
		};

		if (source.aspect) {
			src.aspect = source.aspect;
		}

		if (typeof source.mipLevel === 'number') {
			src.mipLevel = source.mipLevel;
		}

		if (source.origin) {
			src.origin = source.origin;
		}

		const dst: GPUImageCopyTexture = {
			texture: destination.texture[native_],
		};

		if (destination.aspect) {
			dst.aspect = destination.aspect;
		}

		if (typeof destination.mipLevel === 'number') {
			dst.mipLevel = destination.mipLevel;
		}

		if (destination.origin) {
			dst.origin = destination.origin;
		}

		let size: GPUExtent3D;

		if (Array.isArray(copySize)) {
			size = {
				width: copySize[0],
				height: copySize[1] ?? 1,
				depthOrArrayLayers: copySize[2] ?? 1,
			};
		} else {
			size = { ...copySize };
		}

		this[native_].copyTextureToTexture(src, dst, size);
	}

	finish(descriptor?: { label?: string }) {
		const ret = this[native_].finish(descriptor);
		return GPUCommandBuffer.fromNative(ret);
	}

	insertDebugMarker(markerLabel: string) {
		this[native_].insertDebugMarker(markerLabel);
	}

	popDebugGroup() {
		this[native_].popDebugGroup();
	}

	pushDebugGroup(groupLabel: string) {
		this[native_].pushDebugGroup(groupLabel);
	}

	resolveQuerySet(querySet: GPUQuerySet, firstQuery: number, queryCount: number, destination: GPUBuffer, destinationOffset: number) {
		this[native_].resolveQuerySet(querySet[native_], firstQuery, queryCount, destination[native_], destinationOffset);
	}

	writeTimestamp(querySet: GPUQuerySet, queryIndex: number) {
		this[native_].writeTimestamp(querySet[native_], queryIndex);
	}
}
