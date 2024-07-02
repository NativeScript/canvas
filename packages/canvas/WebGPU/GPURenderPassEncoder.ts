import { native_ } from './Constants';
import { GPUBuffer } from './GPUBuffer';
import { GPURenderPipeline } from './GPURenderPipeline';

export class GPURenderPassEncoder {
	[native_];

	static fromNative(pass) {
		if (pass) {
			const ret = new GPURenderPassEncoder();
			ret[native_] = pass;
			return ret;
		}
		return null;
	}

	setPipeline(renderPipeline: GPURenderPipeline) {
		this[native_].setPipeline(renderPipeline[native_]);
	}

	setVertexBuffer(slot: number, buffer: GPUBuffer, offset?: number, size?: number) {
		this[native_].setVertexBuffer(slot, buffer[native_], offset, size);
	}

	draw(vertexCount: number, instanceCount: number = 1, firstVertex: number = 0, firstInstance: number = 0) {
		this[native_].draw(vertexCount, instanceCount, firstVertex, firstInstance);
	}

	end() {
		this[native_].end();
	}
}
