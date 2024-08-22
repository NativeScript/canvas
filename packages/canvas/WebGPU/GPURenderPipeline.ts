import { native_ } from './Constants';
import { GPUBindGroupLayout } from './GPUBindGroupLayout';

export class GPURenderPipeline {
	[native_];

	get label() {
		return this[native_]?.label ?? '';
	}

	getBindGroupLayout(index: number) {
		return GPUBindGroupLayout.fromNative(this[native_].getBindGroupLayout(index));
	}

	static fromNative(pipeline) {
		if (pipeline) {
			const ret = new GPURenderPipeline();
			ret[native_] = pipeline;
			return ret;
		}
		return null;
	}
}
