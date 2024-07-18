import { native_ } from './Constants';
import { GPUBindGroup } from './GPUBindGroup';
import { GPUBuffer } from './GPUBuffer';
import { GPURenderPipeline } from './GPURenderPipeline';

export class GPURenderBundleEncoder {
	[native_];
	draw(vertexCount: number, instanceCount: number = 1, firstVertex: number = 0, firstInstance: number = 0) {
		this[native_].draw(vertexCount, instanceCount ?? 1, firstVertex ?? 0, firstInstance ?? 0);
	}

	drawIndexed(indexCount: number, instanceCount: number = 1, firstVertex: number = 0, firstInstance: number = 0) {
		this[native_].drawIndexed(indexCount, instanceCount ?? 1, firstVertex ?? 0, firstInstance ?? 0);
	}

	drawIndexedIndirect(indirectBuffer: GPUBuffer, indirectOffset: number) {
		this[native_].drawIndexedIndirect(indirectBuffer[native_], indirectOffset);
	}

	drawIndirect(indirectBuffer: GPUBuffer, indirectOffset: number) {
		this[native_].drawIndirect(indirectBuffer[native_], indirectOffset);
	}

	finish() {
		this[native_].finish();
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

	setBindGroup(index: number, bindGroup: GPUBindGroup, dynamicOffsetsData?: number[] | Uint32Array, dynamicOffsetsDataStart?: number, dynamicOffsetsDataLength?: number) {
		const group = bindGroup?.[native_];
		if (!group) {
			return;
		}
		if (Array.isArray(dynamicOffsetsData)) {
			const data = new Uint32Array(dynamicOffsetsData);
			this[native_].setBindGroup(index, group, data, 0, data.length);
		} else if (dynamicOffsetsData instanceof Uint32Array) {
			this[native_].setBindGroup(index, group, dynamicOffsetsData, dynamicOffsetsDataStart, dynamicOffsetsDataLength);
		} else {
			this[native_].setBindGroup(index, group);
		}
	}

	setIndexBuffer(buffer: GPUBindGroup, indexFormat: 'uint16' | 'uint32', offset?: number, size?: number) {
		this[native_].setIndexBuffer(buffer[native_], indexFormat, offset, size);
	}

	setPipeline(renderPipeline: GPURenderPipeline) {
		this[native_].setPipeline(renderPipeline[native_]);
	}

	setVertexBuffer(slot: number, buffer: GPUBuffer, offset?: number, size?: number) {
		this[native_].setVertexBuffer(slot, buffer[native_], offset ?? -1, size ?? -1);
	}

	static fromNative(encoder) {
		if (encoder) {
			const ret = new GPURenderBundleEncoder();
			ret[native_] = encoder;
			return ret;
		}
		return null;
	}
}
