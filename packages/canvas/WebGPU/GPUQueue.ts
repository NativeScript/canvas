import { native_ } from './Constants';
import { GPUBuffer } from './GPUBuffer';
import { GPUCommandBuffer } from './GPUCommandBuffer';

export class GPUQueue {
	[native_];

	static fromNative(value) {
		if (value) {
			const ret = new GPUQueue();
			ret[native_] = value;
			return ret;
		}
		return null;
	}

	writeBuffer(buffer: GPUBuffer, bufferOffset: number, data: Uint8Array | Array<number>, dataOffset: number, size?: number) {
		if (Array.isArray(data)) {
			data = new Uint8Array(data);
		}

		this[native_].writeBuffer(buffer?.[native_], bufferOffset ?? 0, data, dataOffset ?? 0, size ?? -1);
	}

	submit(commands: GPUCommandBuffer[]) {
		console.log('submit', commands);
	}
}
