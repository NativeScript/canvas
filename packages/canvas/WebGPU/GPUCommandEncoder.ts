import { native_ } from './Constants';
import { GPUCommandBuffer } from './GPUCommandBuffer';
import { GPURenderPassEncoder } from './GPURenderPassEncoder';

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

	beginRenderPass(desc) {
		const passEncoder = this[native_].beginRenderPass(desc);
		return GPURenderPassEncoder.fromNative(passEncoder);
	}

	finish(descriptor?: { label?: string }) {
		const ret = this[native_].finish(descriptor);
		console.log('finish', ret);
		return GPUCommandBuffer.fromNative(ret);
	}
}
