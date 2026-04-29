import { native_ } from './Constants';
import { GPUBindGroupLayout } from './GPUBindGroupLayout';
import { GPUBuffer } from './GPUBuffer';
import { GPUExternalTexture } from './GPUExternalTexture';
import { GPUPipelineLayout } from './GPUPipelineLayout';
import { GPUQuerySet } from './GPUQuerySet';
import { GPUSampler } from './GPUSampler';
import { GPUTexture } from './GPUTexture';
import { GPUTextureView } from './GPUTextureView';
import { GPUBindGroupDescriptor, GPUComputePassDescriptor, GPUComputePipelineDescriptor, GPUDepthStencilState, GPUFragmentState, GPUMultisampleState, GPUPrimitiveState, GPURenderPassColorAttachment, GPURenderPassDepthStencilAttachment, GPURenderPassDescriptor, GPURenderPassTimestampWrites, GPURenderPipelineDescriptor, GPUVertexState } from './Interfaces';

export function parseVertexFormat(value: string) {
	switch (value) {
		case 'uint8':
			return 0;
		case 'uint8x2':
			return 1;
		case 'uint8x4':
			return 2;
		case 'sint8':
			return 3;
		case 'sint8x2':
			return 4;
		case 'sint8x4':
			return 5;
		case 'unorm8':
			return 6;
		case 'unorm8x2':
			return 7;
		case 'unorm8x4':
			return 8;
		case 'snorm8':
			return 9;
		case 'snorm8x2':
			return 10;
		case 'snorm8x4':
			return 11;
		case 'uint16':
			return 12;
		case 'uint16x2':
			return 13;
		case 'uint16x4':
			return 14;
		case 'sint16':
			return 15;
		case 'sint16x2':
			return 16;
		case 'sint16x4':
			return 17;
		case 'unorm16':
			return 18;
		case 'unorm16x2':
			return 19;
		case 'unorm16x4':
			return 20;
		case 'snorm16':
			return 21;
		case 'snorm16x2':
			return 22;
		case 'snorm16x4':
			return 23;
		case 'float16':
			return 24;
		case 'float16x2':
			return 25;
		case 'float16x4':
			return 26;
		case 'float32':
			return 27;
		case 'float32x2':
			return 28;
		case 'float32x3':
			return 29;
		case 'float32x4':
			return 30;
		case 'uint32':
			return 31;
		case 'uint32x2':
			return 32;
		case 'uint32x3':
			return 33;
		case 'uint32x4':
			return 34;
		case 'sint32':
			return 35;
		case 'sint32x2':
			return 36;
		case 'sint32x3':
			return 37;
		case 'sint32x4':
			return 38;
		case 'float64':
			return 39;
		case 'float64x2':
			return 40;
		case 'float64x3':
			return 41;
		case 'float64x4':
			return 42;
		case 'unorm10-10-10-2':
			return 43;
		case 'unorm8x4-bgra':
			return 44;
	}
}

export function parseComputePassDescriptor(value?: GPUComputePassDescriptor) {
	if (!value) {
		return value;
	}
	const desc: GPUComputePassDescriptor = {};
	if (value.label) {
		desc.label = value.label;
	}
	if (value.timestampWrites) {
		desc.timestampWrites = { beginningOfPassWriteIndex: value.timestampWrites.beginningOfPassWriteIndex, endOfPassWriteIndex: value.timestampWrites.endOfPassWriteIndex, querySet: value.timestampWrites.querySet[native_] };
	}
	return desc;
}

export function parseRenderPassDescriptor(value: GPURenderPassDescriptor) {
	const desc: GPURenderPassDescriptor = { colorAttachments: new Array(value?.colorAttachments?.length ?? 0) };

	if (typeof value.maxDrawCount === 'number') {
		desc.maxDrawCount = value.maxDrawCount;
	}

	if (typeof value.label === 'string') {
		desc.label = value.label;
	}

	value.colorAttachments.forEach((attachment, i) => {
		if (attachment) {
			const newAttachment: Omit<GPURenderPassColorAttachment, 'view'> & { view: GPUTexture | GPUTextureView | null } = { loadOp: 'load', storeOp: 'store', view: null };

			if (Array.isArray(attachment.clearValue)) {
				newAttachment.clearValue = { r: attachment.clearValue[0], g: attachment.clearValue[1], b: attachment.clearValue[2], a: attachment.clearValue[3] };
			} else {
				newAttachment.clearValue = attachment.clearValue;
			}
			if (attachment.view) {
				if (attachment.view?.[native_]) {
					newAttachment.view = attachment.view[native_];
				} else if ('createView' in attachment.view && typeof attachment.view.createView === 'function') {
					try {
						const v = attachment.view.createView();
						newAttachment.view = v ? (v[native_] ?? v) : null;
					} catch (e) {}
				} else if (attachment.view instanceof GPUTextureView) {
					newAttachment.view = attachment.view[native_];
				} else if (attachment.view instanceof GPUTexture && typeof attachment.view.createView === 'function') {
					const v = attachment.view.createView();
					newAttachment.view = v ? v[native_] : null;
				} else if (attachment.view?.[native_]) {
					newAttachment.view = attachment.view[native_];
				} else {
					newAttachment.view = attachment.view as any;
				}
			}

			if (attachment.resolveTarget) {
				if (attachment.resolveTarget instanceof GPUTextureView) {
					newAttachment.resolveTarget = attachment.resolveTarget[native_];
				} else if (attachment.resolveTarget instanceof GPUTexture && typeof attachment.resolveTarget.createView === 'function') {
					const v = attachment.resolveTarget.createView();
					newAttachment.resolveTarget = v ? v[native_] : null;
				} else if (attachment.resolveTarget?.[native_]) {
					newAttachment.resolveTarget = attachment.resolveTarget[native_];
				} else {
					newAttachment.resolveTarget = attachment.resolveTarget as any;
				}
			}

			if (typeof attachment?.depthSlice === 'number') {
				newAttachment.depthSlice = attachment.depthSlice;
			}

			if (attachment.loadOp) {
				newAttachment.loadOp = attachment.loadOp;
			}

			if (attachment.storeOp) {
				newAttachment.storeOp = attachment.storeOp;
			}

			if (newAttachment.view) {
				desc.colorAttachments[i] = newAttachment as never;
			}
		} else {
			desc.colorAttachments[i] = null;
		}
	});

	if (value?.depthStencilAttachment) {
		let dsView: any = null;
		const ds = value.depthStencilAttachment as any;
		if (ds.view) {
			if (ds.view instanceof GPUTextureView) {
				dsView = ds.view[native_];
			} else if (ds.view instanceof GPUTexture && typeof ds.view.createView === 'function') {
				const v = ds.view.createView();
				dsView = v ? v[native_] : null;
			} else if (ds.view?.[native_]) {
				dsView = ds.view[native_];
			} else {
				dsView = ds.view as any;
			}
		}

		desc.depthStencilAttachment = {
			depthLoadOp: value.depthStencilAttachment.depthLoadOp ?? 'load',
			depthStoreOp: value.depthStencilAttachment.depthStoreOp ?? 'store',
			view: dsView,
			depthClearValue: value.depthStencilAttachment.depthClearValue ?? 0,
			depthReadOnly: value.depthStencilAttachment.depthReadOnly ?? false,
			stencilLoadOp: value.depthStencilAttachment.stencilLoadOp ?? 'load',
			stencilStoreOp: value.depthStencilAttachment.stencilStoreOp ?? 'store',
			stencilClearValue: value.depthStencilAttachment.stencilClearValue ?? 0,
			stencilReadOnly: value.depthStencilAttachment.stencilReadOnly ?? false,
		};
	}

	if (value?.occlusionQuerySet) {
		desc.occlusionQuerySet = value.occlusionQuerySet[native_];
	}

	if (value?.timestampWrites) {
		desc.timestampWrites = { beginningOfPassWriteIndex: value.timestampWrites.beginningOfPassWriteIndex, endOfPassWriteIndex: value.timestampWrites.endOfPassWriteIndex, querySet: value.timestampWrites.querySet[native_] };
	}

	return desc;
}

export function parseBindGroupDescriptor(value: GPUBindGroupDescriptor) {
	const desc: GPUBindGroupDescriptor = { layout: value.layout[native_], entries: [] };

	if (typeof value.label === 'string') {
		desc.label = value?.label;
	}

	if (Array.isArray(value.entries)) {
		for (const entry of value.entries) {
			if (entry.resource instanceof GPUTextureView) {
				desc.entries.push({ binding: entry.binding, resource: entry.resource[native_] });
			} else if (entry.resource instanceof GPUSampler) {
				desc.entries.push({ binding: entry.binding, resource: entry.resource[native_] });
			} else if (entry.resource instanceof GPUExternalTexture) {
				desc.entries.push({ binding: entry.binding, resource: entry.resource[native_] });
			} else if (entry?.resource instanceof GPUBuffer) {
				desc.entries.push({ binding: entry.binding, resource: { buffer: entry.resource[native_], offset: 0, size: entry.resource.size } });
			} else if (entry?.resource?.buffer && entry?.resource?.buffer instanceof GPUBuffer) {
				const offset = entry.resource.offset ?? 0;
				desc.entries.push({ binding: entry.binding, resource: { buffer: entry.resource.buffer[native_], offset, size: entry.resource.size ?? entry.resource.buffer.size - offset } });
			}
		}
	}

	return desc;
}

export function parseComputePipelineDescriptor(value: GPUComputePipelineDescriptor) {
	const desc: GPUComputePipelineDescriptor = { compute: { module: value.compute.module[native_] }, layout: 'auto' };

	if (value.compute?.constants) {
		desc.compute.constants = value.compute.constants;
	}

	if (value.compute?.entryPoint) {
		desc.compute.entryPoint = value.compute.entryPoint;
	}

	if (typeof value?.label === 'string') {
		desc.label = value.label;
	}

	if (value.layout instanceof GPUPipelineLayout) {
		desc.layout = value.layout[native_];
	}
	return desc;
}

function handleUnsupportedPlatformFormat(fragment: GPUFragmentState, method: string) {
	// falls back to platform supported format ... maybe this can be removed once frameworks use the getPreferredCanvasFormat
	if (__ANDROID__) {
		let hasBrga = false;
		fragment.targets = fragment.targets.map((target) => {
			switch (target?.format) {
				case 'bgra8unorm':
					target.format = 'rgba8unorm';
					hasBrga = true;
					break;
				case 'bgra8unorm-srgb':
					target.format = 'rgba8unorm-srgb';
					hasBrga = true;
					break;
			}
			return target;
		});
		if (hasBrga) {
			console.warn(`GPUDevice:${method} using unsupported brga format falling back to rgba counterpart.`);
		}
	}
}

export function parseRenderPipelineDescriptor(value: GPURenderPipelineDescriptor, method: string) {
	const desc: GPURenderPipelineDescriptor = { vertex: { module: value.vertex?.module?.[native_] }, layout: 'auto' };

	if (Array.isArray(value.vertex?.buffers)) {
		desc.vertex.buffers = [...value.vertex.buffers].map((source) => {
			const buffer = { ...source };
			buffer.attributes = [...buffer.attributes].map((attr) => {
				const ret = { ...attr };
				ret.format = parseVertexFormat(ret.format) as never;
				return ret;
			});

			return buffer;
		});
	}

	if (value.vertex.constants) {
		desc.vertex.constants = value.vertex.constants;
	}

	if (value.vertex.entryPoint) {
		desc.vertex.entryPoint = value.vertex.entryPoint;
	}

	if (value.fragment) {
		desc.fragment = { module: value.fragment?.module?.[native_], targets: value.fragment.targets };

		if (value.fragment.constants) {
			desc.fragment.constants = value.fragment.constants;
		}

		if (value.fragment.entryPoint) {
			desc.fragment.entryPoint = value.fragment.entryPoint;
		}

		handleUnsupportedPlatformFormat(desc.fragment, method);
	}

	if (value.depthStencil) {
		desc.depthStencil = value.depthStencil;
	}

	if (value.multisample) {
		desc.multisample = value.multisample;
	}

	if (value.primitive) {
		desc.primitive = value.primitive;
	}

	if (value.layout instanceof GPUPipelineLayout) {
		desc.layout = value.layout[native_];
	}

	return desc;
}
