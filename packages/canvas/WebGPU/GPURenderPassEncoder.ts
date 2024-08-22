import { native_ } from './Constants';
import { GPUBindGroup } from './GPUBindGroup';
import { GPUBuffer } from './GPUBuffer';
import { GPURenderBundle } from './GPURenderBundle';
import { GPURenderPipeline } from './GPURenderPipeline';
import { GPUColor } from './Types';

export class GPURenderPassEncoder {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	beginOcclusionQuery(queryIndex: number) {
		this[native_].beginOcclusionQuery(queryIndex);
	}

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

	end() {
		this[native_].end();
	}

	endOcclusionQuery() {
		this[native_].endOcclusionQuery();
	}

	executeBundles(bundles: GPURenderBundle[]) {
		if (Array.isArray(bundles)) {
			this[native_].executeBundles(bundles.map((bundle) => bundle[native_]));
		}
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

	setBlendConstant(color: GPUColor) {
		if (Array.isArray(color)) {
			this[native_].setBlendConstant({
				r: color[0],
				g: color[1],
				b: color[2],
				a: color[3],
			});
		} else {
			this[native_].setBlendConstant(color);
		}
	}

	setIndexBuffer(buffer: GPUBuffer, indexFormat: 'uint16' | 'uint32', offset?: number, size?: number) {
		this[native_].setIndexBuffer(buffer[native_], indexFormat, offset ?? 0, size ?? buffer.size - (offset ?? 0));
	}

	setPipeline(renderPipeline: GPURenderPipeline) {
		this[native_].setPipeline(renderPipeline[native_]);
	}

	setScissorRect(x: number, y: number, width: number, height: number) {
		this[native_].setScissorRect(x, y, width, height);
	}

	setStencilReference(reference: number) {
		this[native_].setStencilReference(reference);
	}

	setVertexBuffer(slot: number, buffer: GPUBuffer, offset?: number, size?: number) {
		this[native_].setVertexBuffer(slot, buffer[native_], offset ?? 0, size ?? buffer.size - (offset ?? 0));
	}

	setViewport(x, y, width, height, minDepth, maxDepth) {
		this[native_].setViewport(x, y, width, height, minDepth, maxDepth);
	}

	static fromNative(pass) {
		if (pass) {
			const ret = new GPURenderPassEncoder();
			ret[native_] = pass;
			return ret;
		}
		return null;
	}
}
