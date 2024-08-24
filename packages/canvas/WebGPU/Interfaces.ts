import type { GPUBindGroupLayout } from './GPUBindGroupLayout';
import type { GPUBuffer } from './GPUBuffer';
import type { GPUPipelineLayout } from './GPUPipelineLayout';
import type { GPUQuerySet } from './GPUQuerySet';
import type { GPUShaderModule } from './GPUShaderModule';
import type { GPUTexture } from './GPUTexture';
import type { GPUTextureView } from './GPUTextureView';
import type { GPUBindingResource, GPUBlendFactor, GPUBlendOperation, GPUBufferBindingType, GPUColor, GPUCompareFunction, GPUCullMode, GPUFeatureName, GPUFrontFace, GPUIndexFormat, GPULoadOp, GPUOrigin2D, GPUOrigin3D, GPUPrimitiveTopology, GPUSamplerBindingType, GPUStencilOperation, GPUStorageTextureAccess, GPUStoreOp, GPUTextureAspect, GPUTextureFormat, GPUTextureSampleType, GPUTextureViewDimension, GPUVertexFormat, GPUVertexStepMode } from './Types';
export interface GPUExternalTextureBindingLayout {}

export interface GPUProgrammableStage {
	constants?: {
		[name: string]: number;
	};
	entryPoint?: string | Uint32Array;
	module: GPUShaderModule;
}

export interface GPUStencilFaceState {
	compare?: GPUCompareFunction;
	depthFailOp?: GPUStencilOperation;
	failOp?: GPUStencilOperation;
	passOp?: GPUStencilOperation;
}

export interface GPUDepthStencilState {
	depthBias?: number;
	depthBiasClamp?: number;
	depthBiasSlopeScale?: number;
	depthCompare?: GPUCompareFunction;
	depthWriteEnabled?: boolean;
	format: GPUTextureFormat;
	stencilBack?: GPUStencilFaceState;
	stencilFront?: GPUStencilFaceState;
	stencilReadMask?: number;
	stencilWriteMask?: number;
}

export interface GPUBlendComponent {
	dstFactor?: GPUBlendFactor;
	operation?: GPUBlendOperation;
	srcFactor?: GPUBlendFactor;
}

export interface GPUBlendState {
	alpha: GPUBlendComponent;
	color: GPUBlendComponent;
}

export interface GPUColorTargetState {
	blend?: GPUBlendState;
	format: GPUTextureFormat;
	writeMask?: number;
}

export interface GPUFragmentState {
	constants?: {
		[name: string]: number;
	};
	entryPoint?: string | Uint32Array;
	module: GPUShaderModule;
	targets: (null | GPUColorTargetState)[];
}

export interface GPUMultisampleState {
	alphaToCoverageEnabled?: boolean;
	count?: number;
	mask?: number;
}

export interface GPUPrimitiveState {
	cullMode?: GPUCullMode;
	frontFace?: GPUFrontFace;
	stripIndexFormat?: GPUIndexFormat;
	topology?: GPUPrimitiveTopology;
	unclippedDepth?: boolean;
}

export interface GPUVertexAttribute {
	format: GPUVertexFormat;
	offset: number;
	shaderLocation: number;
}

export interface GPUVertexBufferLayout {
	arrayStride: number;
	attributes: GPUVertexAttribute[];
	stepMode?: GPUVertexStepMode;
}

export interface GPUVertexState {
	buffers?: GPUVertexBufferLayout[];
	constants?: {
		[name: string]: number;
	};
	entryPoint?: string | Uint32Array;
	module: GPUShaderModule;
}

export interface GPUColorDict {
	a: number;
	b: number;
	g: number;
	r: number;
}

export interface GPURenderPassColorAttachment {
	clearValue?: GPUColor;
	depthSlice?: number;
	loadOp: GPULoadOp;
	resolveTarget?: GPUTextureView;
	storeOp: GPUStoreOp;
	view: GPUTextureView;
}

export interface GPURenderPassDepthStencilAttachment {
	depthClearValue?: number;
	depthLoadOp: GPULoadOp;
	depthReadOnly?: boolean;
	depthStoreOp: GPUStoreOp;
	stencilClearValue?: number;
	stencilLoadOp?: GPULoadOp;
	stencilReadOnly?: boolean;
	stencilStoreOp?: GPUStoreOp;
	view: GPUTextureView;
}

export interface GPURenderPassTimestampWrites {
	beginningOfPassWriteIndex: number;
	endOfPassWriteIndex: number;
	querySet: GPUQuerySet;
}

export interface GPUImageCopyBuffer {
	buffer: GPUBuffer;
	bytesPerRow: number;
	offset?: number;
	rowsPerImage?: number;
}

export interface GPUImageCopyTexture {
	aspect?: GPUTextureAspect;
	mipLevel?: number;
	origin?: GPUOrigin3D;
	texture: GPUTexture;
}

export interface GPUImageCopyExternalImage {
	flipY?: boolean;
	origin?: GPUOrigin2D;
	source: any;
}

export interface GPUImageCopyTextureTagged {
	aspect?: GPUTextureAspect;
	colorSpace?: PredefinedColorSpace;
	mipLevel?: number;
	origin?: GPUOrigin3D;
	premultipliedAlpha?: boolean;
	texture: GPUTexture;
}

export interface GPUImageDataLayout {
	bytesPerRow: number;
	offset?: number;
	rowsPerImage?: number;
}

export interface GPUTextureViewDescriptor {
	arrayLayerCount?: number;
	aspect?: GPUTextureAspect;
	baseArrayLayer?: number;
	baseMipLevel?: number;
	dimension: GPUTextureViewDimension;
	format?: GPUTextureFormat;
	label?: string;
	mipLevelCount?: number;
}

export interface GPUDeviceDescriptor {
	defaultQueue?: {
		label?: string;
	};
	label?: string;
	requiredFeatures?: GPUFeatureName[];
	requiredLimits?: {
		[name: string]: number;
	};
}

export interface GPUComputePassTimestampWrites {
	beginningOfPassWriteIndex: number;
	endOfPassWriteIndex: number;
	querySet: GPUQuerySet;
}

export interface GPUComputePassDescriptor {
	label?: string;
	timestampWrites?: GPUComputePassTimestampWrites;
}

export interface GPURenderPassDescriptor {
	colorAttachments: (null | GPURenderPassColorAttachment)[];
	depthStencilAttachment?: GPURenderPassDepthStencilAttachment;
	label?: string;
	maxDrawCount?: number;
	occlusionQuerySet?: GPUQuerySet;
	timestampWrites?: GPURenderPassTimestampWrites;
}

export interface GPUBufferBinding {
	buffer: GPUBuffer;
	offset?: number;
	size?: number;
}

export interface GPUBindGroupEntry {
	binding: number;
	resource: GPUBindingResource;
}

export interface GPUBindGroupDescriptor {
	entries: GPUBindGroupEntry[];
	label?: string;
	layout: GPUBindGroupLayout;
}

export interface GPUBufferBindingLayout {
	hasDynamicOffset?: boolean;
	minBindingSize?: number;
	type?: GPUBufferBindingType;
}

export interface GPUExternalTextureBindingLayout {}

export interface GPUStorageTextureBindingLayout {
	access?: GPUStorageTextureAccess;
	format: GPUTextureFormat;
	viewDimension?: GPUTextureViewDimension;
}

export interface GPUSamplerBindingLayout {
	type?: GPUSamplerBindingType;
}

export interface GPUTextureBindingLayout {
	multisampled?: boolean;
	sampleType?: GPUTextureSampleType;
	viewDimension?: GPUTextureViewDimension;
}

export interface GPUBindGroupLayoutEntry {
	label?: string;
	binding: number;
	buffer?: GPUBufferBindingLayout;
	externalTexture?: GPUExternalTextureBindingLayout;
	sampler?: GPUSamplerBindingLayout;
	storageTexture?: GPUStorageTextureBindingLayout;
	texture?: GPUTextureBindingLayout;
	visibility: number;
}

export interface GPUBindGroupLayoutDescriptor {
	entries: GPUBindGroupLayoutEntry[];
	label?: string;
}

export interface GPUComputePipelineDescriptor {
	compute: GPUProgrammableStage;
	label?: string;
	layout: GPUPipelineLayout | 'auto';
}

export interface GPURenderPipelineDescriptor {
	depthStencil?: GPUDepthStencilState;
	fragment?: GPUFragmentState;
	label?: string;
	layout: GPUPipelineLayout | 'auto';
	multisample?: GPUMultisampleState;
	primitive?: GPUPrimitiveState;
	vertex: GPUVertexState;
}
