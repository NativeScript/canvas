import type { GPUBuffer } from './GPUBuffer';
import type { GPUQuerySet } from './GPUQuerySet';
import type { GPUShaderModule } from './GPUShaderModule';
import type { GPUTexture } from './GPUTexture';
import type { GPUTextureView } from './GPUTextureView';
import { GPUBlendFactor, GPUBlendOperation, GPUColor, GPUCompareFunction, GPUCullMode, GPUFrontFace, GPUIndexFormat, GPULoadOp, GPUOrigin3D, GPUPrimitiveTopology, GPUStencilOperation, GPUStoreOp, GPUTextureAspect, GPUTextureFormat, GPUVertexFormat, GPUVertexStepMode } from './Types';
export interface GPUExternalTextureBindingLayout {}

export interface GPUProgrammableStage {
	constants?: {
		[name: string]: number;
	};
	entryPoint: string | Uint32Array;
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
	entryPoint: string | Uint32Array;
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
	entryPoint: string | Uint32Array;
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
