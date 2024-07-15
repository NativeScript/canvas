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
		desc.colorAttachments = desc.colorAttachments.map((attachment) => {
			if (Array.isArray(attachment.clearValue)) {
				attachment.clearValue = { r: attachment.clearValue[0], g: attachment.clearValue[1], b: attachment.clearValue[2], a: attachment.clearValue[3] };
			}
			attachment.view = attachment.view[native_];
			return attachment;
		});

		if (desc?.depthStencilAttachment?.view?.[native_]) {
			desc.depthStencilAttachment.view = desc.depthStencilAttachment.view[native_];
		}
		const passEncoder = this[native_].beginRenderPass(desc);
		return GPURenderPassEncoder.fromNative(passEncoder);
	}

	finish(descriptor?: { label?: string }) {
		const ret = this[native_].finish(descriptor);
		return GPUCommandBuffer.fromNative(ret);
	}
}
