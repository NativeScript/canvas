import { native_ } from './Constants';
import { GPUBindGroup } from './GPUBindGroup';

export class GPURenderPass {
	[native_];

	setBindGroup(index: number, bindGroup: GPUBindGroup, dynamicOffsetsData?: number[] | Uint8Array, dynamicOffsetsDataStart?: number, dynamicOffsetsDataLength?: number) {
		const group = bindGroup?.[native_];
		if (!group) {
			return;
		}
		if (Array.isArray(dynamicOffsetsData)) {
			const data = new Uint8Array(dynamicOffsetsData);
			this[native_].setBindGroup(index, group, data, 0, data.length);
		} else if (dynamicOffsetsData instanceof Uint8Array) {
			this[native_].setBindGroup(index, group, dynamicOffsetsData, dynamicOffsetsDataStart, dynamicOffsetsDataLength);
		} else {
			this[native_].setBindGroup(index, group);
		}
	}
}
