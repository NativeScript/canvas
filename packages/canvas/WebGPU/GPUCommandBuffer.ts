import { native_ } from './Constants';

export class GPUCommandBuffer {
	[native_];
	static fromNative(commandBuffer) {
		if (commandBuffer) {
			const ret = new GPUCommandBuffer();
			ret[native_] = commandBuffer;
			return ret;
		}
		return null;
	}
}
