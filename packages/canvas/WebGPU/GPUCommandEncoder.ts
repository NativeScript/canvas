import { native_ } from './Constants';
import { GPUBuffer } from './GPUBuffer';
import { GPUCommandBuffer } from './GPUCommandBuffer';
import { GPUComputePassEncoder } from './GPUComputePassEncoder';
import { GPUQuerySet } from './GPUQuerySet';
import { GPURenderPassEncoder } from './GPURenderPassEncoder';
import { GPUImageCopyBuffer, GPUImageCopyTexture, GPURenderPassColorAttachment, GPURenderPassDepthStencilAttachment, GPURenderPassTimestampWrites } from './Interfaces';
import { GPUExtent3D } from './Types';

export class GPUCommandEncoder {
	[native_];

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
		if (descriptor?.timestampWrites) {
			descriptor.timestampWrites.querySet = descriptor.timestampWrites.querySet[native_];
		}

		return GPUComputePassEncoder.fromNative(this[native_].beginComputePass(descriptor));
	}

	beginRenderPass(descriptor: { colorAttachments: (null | GPURenderPassColorAttachment)[]; depthStencilAttachment?: GPURenderPassDepthStencilAttachment; label?: string; maxDrawCount?: number; occlusionQuerySet?: GPUQuerySet; timestampWrites?: GPURenderPassTimestampWrites }) {
		descriptor.colorAttachments = descriptor.colorAttachments.map((attachment) => {
			if (Array.isArray(attachment.clearValue)) {
				attachment.clearValue = { r: attachment.clearValue[0], g: attachment.clearValue[1], b: attachment.clearValue[2], a: attachment.clearValue[3] };
			}
			attachment.view = attachment.view[native_];

			if (attachment.resolveTarget) {
				attachment.resolveTarget = attachment.resolveTarget[native_];
			}
			return attachment;
		});

		if (descriptor?.depthStencilAttachment?.view?.[native_]) {
			descriptor.depthStencilAttachment.view = descriptor.depthStencilAttachment.view[native_];
		}

		if (descriptor?.occlusionQuerySet) {
			descriptor.occlusionQuerySet = descriptor.occlusionQuerySet[native_];
		}

		if (descriptor?.timestampWrites?.querySet) {
			descriptor.timestampWrites.querySet = descriptor.timestampWrites.querySet[native_];
		}

		const passEncoder = this[native_].beginRenderPass(descriptor);
		return GPURenderPassEncoder.fromNative(passEncoder);
	}

	clearBuffer(buffer: GPUBuffer, offset?: number, size?: number) {
		this[native_].clearBuffer(buffer[native_], offset ?? -1, size ?? -1);
	}

	copyBufferToBuffer(source: GPUBuffer, sourceOffset: number, destination: GPUBuffer, destinationOffset: number, size: number) {
		this[native_].copyBufferToBuffer(source[native_], sourceOffset, destination[native_], destinationOffset, size);
	}

	copyBufferToTexture(source: GPUImageCopyBuffer, destination: GPUImageCopyTexture, copySize: GPUExtent3D) {
		source.buffer = source.buffer[native_];
		destination.texture = destination.texture[native_];
		this[native_].copyBufferToTexture(source, destination, copySize);
	}

	copyTextureToBuffer(source: GPUImageCopyTexture, destination: GPUImageCopyBuffer, copySize: GPUExtent3D) {
		source.texture = source.texture[native_];
		destination.buffer = destination.buffer[native_];
		this[native_].copyTextureToBuffer(source, destination, copySize);
	}

	copyTextureToTexture(source: GPUImageCopyTexture, destination: GPUImageCopyTexture, copySize: GPUExtent3D) {
		source.texture = source.texture[native_];
		destination.texture = destination.texture[native_];
		this[native_].copyTextureToTexture(source, destination, copySize);
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
