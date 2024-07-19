import { native_ } from './Constants';
import { GPUBindGroup } from './GPUBindGroup';
import { GPUBuffer } from './GPUBuffer';
import { GPUComputePipeline } from './GPUComputePipeline';

export class GPUComputePassEncoder {
	[native_];

	dispatchWorkgroups(workgroupCountX: number, workgroupCountY: number = 1, workgroupCountZ: number = 1) {
		this[native_].dispatchWorkgroups(workgroupCountX, workgroupCountY ?? 1, workgroupCountZ ?? 1);
	}

	dispatchWorkgroupsIndirect(indirectBuffer: GPUBuffer, indirectOffset: number) {
		this[native_].dispatchWorkgroupsIndirect(indirectBuffer[native_], indirectOffset);
	}

	end() {
		this[native_].end();
		this[native_] = null;
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

	setPipeline(renderPipeline: GPUComputePipeline) {
		this[native_].setPipeline(renderPipeline[native_]);
	}

	static fromNative(pass) {
		if (pass) {
			const ret = new GPUComputePassEncoder();
			ret[native_] = pass;
			return ret;
		}
		return null;
	}
}
