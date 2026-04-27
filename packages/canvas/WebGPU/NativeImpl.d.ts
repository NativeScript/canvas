export const enum GPUErrorTypeImpl {
	None = 0,
	Lost,
	OutOfMemory,
	Validation,
	Internal,
}

export const enum GPUErrorFilterImpl {
	OutOfMemory = 0,
	Validation,
	Internal,
}

export const enum GPUPowerPreferenceImpl {
	None = 0,
	LowPower,
	HighPerformance,
}

export const enum SurfaceGetCurrentTextureStatusImpl {
	Success = 0x00000000,
	Timeout = 0x00000001,
	Outdated = 0x00000002,
	Lost = 0x00000003,
	OutOfMemory = 0x00000004,
	DeviceLost = 0x00000005,
	Occluded = 0x00000007,
	Validation = 0x00000008,
}

export const enum GPUTextureDimensionImpl {
	D1 = 0,
	D2,
	D3,
}

export const enum GPUTextureViewDimensionImpl {
	D1 = 0,
	D2,
	D2Array,
	Cube,
	CubeArray,
	D3,
}

export const enum GPUTextureAspectImpl {
	All = 0,
	StencilOnly,
	DepthOnly,
	Plane0,
	Plane1,
	Plane2,
}

export const enum GPUAstcBlockImpl {
	B4x4 = 0,
	B5x4,
	B5x5,
	B6x5,
	B6x6,
	B8x5,
	B8x6,
	B8x8,
	B10x5,
	B10x6,
	B10x8,
	B10x10,
	B12x10,
	B12x12,
}

export const enum GPUAstcChannelImpl {
	Unorm = 0,
	UnormSrgb,
	Hdr,
}

export interface GPUOrigin2dImpl {
	x: number;
	y: number;
}

export interface GPUOrigin3dImpl {
	x: number;
	y: number;
	z: number;
}

export interface GPUExtent3dImpl {
	width: number;
	height: number;
	depthOrArrayLayers: number;
}

export interface GPUColorImpl {
	r: number;
	g: number;
	b: number;
	a: number;
}

export interface GPUImageDataLayoutImpl {
	offset: number;
	bytesPerRow: number;
	rowsPerImage: number;
}

export interface GPUSupportedLimitsImpl {
	maxTextureDimension1d: number;
	maxTextureDimension2d: number;
	maxTextureDimension3d: number;
	maxTextureArrayLayers: number;
	maxBindGroups: number;
	maxBindingsPerBindGroup: number;
	maxDynamicUniformBuffersPerPipelineLayout: number;
	maxDynamicStorageBuffersPerPipelineLayout: number;
	maxSampledTexturesPerShaderStage: number;
	maxSamplersPerShaderStage: number;
	maxStorageBuffersPerShaderStage: number;
	maxStorageTexturesPerShaderStage: number;
	maxUniformBuffersPerShaderStage: number;
	maxBindingArrayElementsPerShaderStage: number;
	maxBindingArraySamplerElementsPerShaderStage: number;
	maxUniformBufferBindingSize: number;
	maxStorageBufferBindingSize: number;
	maxVertexBuffers: number;
	maxBufferSize: number;
	maxVertexAttributes: number;
	maxVertexBufferArrayStride: number;
	maxInterStageShaderVariables: number;
	minUniformBufferOffsetAlignment: number;
	minStorageBufferOffsetAlignment: number;
	maxColorAttachments: number;
	maxColorAttachmentBytesPerSample: number;
	maxComputeWorkgroupStorageSize: number;
	maxComputeInvocationsPerWorkgroup: number;
	maxComputeWorkgroupSizeX: number;
	maxComputeWorkgroupSizeY: number;
	maxComputeWorkgroupSizeZ: number;
	maxComputeWorkgroupsPerDimension: number;
	maxImmediateSize: number;
	maxNonSamplerBindings: number;
	// Mesh shading
	maxTaskInvocationsPerWorkgroup: number;
	maxTaskInvocationsPerDimension: number;
	maxMeshInvocationsPerWorkgroup: number;
	maxMeshInvocationsPerDimension: number;
	maxTaskPayloadSize: number;
	maxMeshOutputVertices: number;
	maxMeshOutputPrimitives: number;
	maxMeshOutputLayers: number;
	maxMeshMultiviewViewCount: number;
	// Ray tracing
	maxBlasPrimitiveCount: number;
	maxBlasGeometryCount: number;
	maxTlasInstanceCount: number;
	maxAccelerationStructuresPerShaderStage: number;
	maxMultiviewViewCount: number;
}

export interface GPUAdapterInfoImpl {
	readonly vendor: string;
	readonly architecture: string;
	readonly device: string;
	readonly description: string;
}

export interface GPUAdapterImpl {
	readonly label: string;
	readonly features: string[];
	readonly isFallbackAdapter: boolean;
	readonly limits: GPUSupportedLimitsImpl;

	requestAdapterInfo(): GPUAdapterInfoImpl;

	requestDevice(options: GPUDeviceDescriptorImpl, callback: (error: string | null, device: GPUDeviceImpl | null) => void): void;
}

export interface GPURequestAdapterOptionsImpl {
	powerPreference?: GPUPowerPreferenceImpl;
	forceFallbackAdapter?: boolean;
	featureLevel?: 'core' | 'compatibility';
}

export interface GPUDeviceDescriptorImpl {
	label?: string;
	requiredFeatures?: string[];
	requiredLimits?: GPUSupportedLimitsImpl | Record<string, number>;
}

export interface GPUBufferImpl {
	readonly label: string;
	readonly size: number;
	readonly usage: number;

	destroy(): void;
	getMappedRange(offset?: number, size?: number): ArrayBuffer;
	mapAsync(mode: number, offset?: number, size?: number): Promise<void>;
	unmap(): void;
}

export interface GPUTextureImpl {
	readonly label: string;
	readonly width: number;
	readonly height: number;
	readonly depthOrArrayLayers: number;
	readonly mipLevelCount: number;
	readonly sampleCount: number;
	readonly dimension: string;
	readonly format: string;
	readonly usage: number;

	createView(descriptor?: GPUTextureViewDescriptorImpl): GPUTextureViewImpl;
	destroy(): void;
}

export interface GPUTextureViewImpl {
	readonly label: string;
}

export interface GPUSamplerImpl {
	readonly label: string;
}

export interface GPUBindGroupImpl {
	readonly label: string;
}

export interface GPUBindGroupLayoutImpl {
	readonly label: string;
}

export interface GPUPipelineLayoutImpl {
	readonly label: string;
}

export interface GPUShaderModuleImpl {
	readonly label: string;
	getCompilationInfo(): Promise<GPUCompilationInfoImpl>;
}

export interface GPUCompilationMessageImpl {
	readonly message: string;
	readonly type: 'error' | 'warning' | 'info';
	readonly lineNum: number;
	readonly linePos: number;
	readonly offset: number;
	readonly length: number;
}

export interface GPUCompilationInfoImpl {
	readonly messages: GPUCompilationMessageImpl[];
}

export interface GPUCommandEncoderImpl {
	readonly label: string;

	beginRenderPass(descriptor: GPURenderPassDescriptorImpl): GPURenderPassEncoderImpl;
	beginComputePass(descriptor?: GPUComputePassDescriptorImpl): GPUComputePassEncoderImpl;
	copyBufferToBuffer(source: GPUBufferImpl, sourceOffset: number, destination: GPUBufferImpl, destinationOffset: number, size: number): void;
	copyBufferToTexture(source: GPUImageCopyBufferImpl, destination: GPUImageCopyTextureImpl, copySize: GPUExtent3dImpl): void;
	copyTextureToBuffer(source: GPUImageCopyTextureImpl, destination: GPUImageCopyBufferImpl, copySize: GPUExtent3dImpl): void;
	copyTextureToTexture(source: GPUImageCopyTextureImpl, destination: GPUImageCopyTextureImpl, copySize: GPUExtent3dImpl): void;
	clearBuffer(buffer: GPUBufferImpl, offset?: number, size?: number): void;
	resolveQuerySet(querySet: GPUQuerySetImpl, firstQuery: number, queryCount: number, destination: GPUBufferImpl, destinationOffset: number): void;
	finish(descriptor?: GPUCommandBufferDescriptorImpl): GPUCommandBufferImpl;
	pushDebugGroup(groupLabel: string): void;
	popDebugGroup(): void;
	insertDebugMarker(markerLabel: string): void;
}

export interface GPUCommandBufferImpl {
	readonly label: string;
}

export interface GPURenderPassEncoderImpl {
	readonly label: string;

	setPipeline(pipeline: GPURenderPipelineImpl): void;
	setBindGroup(index: number, bindGroup: GPUBindGroupImpl | null, dynamicOffsets?: number[]): void;
	setVertexBuffer(slot: number, buffer: GPUBufferImpl | null, offset?: number, size?: number): void;
	setIndexBuffer(buffer: GPUBufferImpl, indexFormat: 'uint16' | 'uint32', offset?: number, size?: number): void;
	setViewport(x: number, y: number, width: number, height: number, minDepth: number, maxDepth: number): void;
	setScissorRect(x: number, y: number, width: number, height: number): void;
	setBlendConstant(color: GPUColorImpl): void;
	setStencilReference(reference: number): void;

	draw(vertexCount: number, instanceCount?: number, firstVertex?: number, firstInstance?: number): void;
	drawIndexed(indexCount: number, instanceCount?: number, firstIndex?: number, baseVertex?: number, firstInstance?: number): void;
	drawIndirect(indirectBuffer: GPUBufferImpl, indirectOffset: number): void;
	drawIndexedIndirect(indirectBuffer: GPUBufferImpl, indirectOffset: number): void;

	executeBundles(bundles: GPURenderBundleImpl[]): void;
	beginOcclusionQuery(queryIndex: number): void;
	endOcclusionQuery(): void;

	pushDebugGroup(groupLabel: string): void;
	popDebugGroup(): void;
	insertDebugMarker(markerLabel: string): void;

	end(): void;
}

export interface GPUComputePassEncoderImpl {
	readonly label: string;

	setPipeline(pipeline: GPUComputePipelineImpl): void;
	setBindGroup(index: number, bindGroup: GPUBindGroupImpl | null, dynamicOffsets?: number[]): void;

	dispatchWorkgroups(workgroupCountX: number, workgroupCountY?: number, workgroupCountZ?: number): void;
	dispatchWorkgroupsIndirect(indirectBuffer: GPUBufferImpl, indirectOffset: number): void;

	pushDebugGroup(groupLabel: string): void;
	popDebugGroup(): void;
	insertDebugMarker(markerLabel: string): void;

	end(): void;
}

export interface GPURenderPipelineImpl {
	readonly label: string;
	getBindGroupLayout(index: number): GPUBindGroupLayoutImpl;
}

export interface GPUComputePipelineImpl {
	readonly label: string;
	getBindGroupLayout(index: number): GPUBindGroupLayoutImpl;
}

export interface GPURenderBundleImpl {
	readonly label: string;
}

export interface GPURenderBundleEncoderImpl {
	readonly label: string;

	setPipeline(pipeline: GPURenderPipelineImpl): void;
	setBindGroup(index: number, bindGroup: GPUBindGroupImpl | null, dynamicOffsets?: number[]): void;
	setVertexBuffer(slot: number, buffer: GPUBufferImpl | null, offset?: number, size?: number): void;
	setIndexBuffer(buffer: GPUBufferImpl, indexFormat: 'uint16' | 'uint32', offset?: number, size?: number): void;
	draw(vertexCount: number, instanceCount?: number, firstVertex?: number, firstInstance?: number): void;
	drawIndexed(indexCount: number, instanceCount?: number, firstIndex?: number, baseVertex?: number, firstInstance?: number): void;
	drawIndirect(indirectBuffer: GPUBufferImpl, indirectOffset: number): void;
	drawIndexedIndirect(indirectBuffer: GPUBufferImpl, indirectOffset: number): void;
	pushDebugGroup(groupLabel: string): void;
	popDebugGroup(): void;
	insertDebugMarker(markerLabel: string): void;
	finish(descriptor?: GPURenderBundleDescriptorImpl): GPURenderBundleImpl;
}

export interface GPUQuerySetImpl {
	readonly label: string;
	readonly type: string;
	readonly count: number;
	destroy(): void;
}

export interface GPUQueueImpl {
	readonly label: string;

	submit(commandBuffers: GPUCommandBufferImpl[]): void;

	writeBuffer(buffer: GPUBufferImpl, bufferOffset: number, data: ArrayBuffer | ArrayBufferView, dataOffset?: number, size?: number): void;

	writeTexture(destination: GPUImageCopyTextureImpl, data: ArrayBuffer | ArrayBufferView, dataLayout: GPUImageDataLayoutImpl, size: GPUExtent3dImpl): void;

	copyExternalImageToTexture(source: any, destination: GPUImageCopyTextureTaggedImpl, copySize: GPUExtent3dImpl): void;

	onSubmittedWorkDone(): Promise<void>;
}

export interface GPUDeviceImpl {
	readonly label: string;
	readonly features: string[];
	readonly limits: GPUSupportedLimitsImpl;
	readonly queue: GPUQueueImpl;

	destroy(): void;

	createBuffer(descriptor: GPUBufferDescriptorImpl): GPUBufferImpl;

	createTexture(descriptor: GPUTextureDescriptorImpl): GPUTextureImpl;

	createSampler(descriptor?: GPUSamplerDescriptorImpl): GPUSamplerImpl;

	createBindGroupLayout(descriptor: GPUBindGroupLayoutDescriptorImpl): GPUBindGroupLayoutImpl;

	createBindGroup(descriptor: GPUBindGroupDescriptorImpl): GPUBindGroupImpl;

	createPipelineLayout(descriptor: GPUPipelineLayoutDescriptorImpl): GPUPipelineLayoutImpl;

	createShaderModule(descriptor: GPUShaderModuleDescriptorImpl): GPUShaderModuleImpl;

	createCommandEncoder(descriptor?: GPUCommandEncoderDescriptorImpl): GPUCommandEncoderImpl;

	createRenderPipeline(descriptor: GPURenderPipelineDescriptorImpl): GPURenderPipelineImpl;

	createRenderPipelineAsync(descriptor: GPURenderPipelineDescriptorImpl, callback: (pipeline: GPURenderPipelineImpl | null, errorType: GPUErrorTypeImpl, message: string | null) => void): void;

	createComputePipeline(descriptor: GPUComputePipelineDescriptorImpl): GPUComputePipelineImpl;

	createComputePipelineAsync(descriptor: GPUComputePipelineDescriptorImpl, callback: (pipeline: GPUComputePipelineImpl | null, errorType: GPUErrorTypeImpl, message: string | null) => void): void;

	createRenderBundleEncoder(descriptor: GPURenderBundleEncoderDescriptorImpl): GPURenderBundleEncoderImpl;

	createQuerySet(descriptor: GPUQuerySetDescriptorImpl): GPUQuerySetImpl;

	pushErrorScope(filter: 'out-of-memory' | 'validation' | 'internal'): void;

	popErrorScope(callback: (errorType: GPUErrorTypeImpl, message: string | null) => void): void;

	setUncapturedErrorCallback(callback: ((errorType: GPUErrorTypeImpl, message: string | null) => void) | null): void;

	setLostCallback(callback: ((reason: number, message: string | null) => void) | null): void;
}

export interface GPUBufferDescriptorImpl {
	label?: string;
	size: number;
	usage: number;
	mappedAtCreation?: boolean;
}

export interface GPUTextureDescriptorImpl {
	label?: string;
	size: GPUExtent3dImpl | [number, number?, number?];
	mipLevelCount?: number;
	sampleCount?: number;
	dimension?: '1d' | '2d' | '3d';
	format: string;
	usage: number;
	viewFormats?: string[];
}

export interface GPUTextureViewDescriptorImpl {
	label?: string;
	format?: string;
	dimension?: string;
	aspect?: string;
	baseMipLevel?: number;
	mipLevelCount?: number;
	baseArrayLayer?: number;
	arrayLayerCount?: number;
}

export interface GPUSamplerDescriptorImpl {
	label?: string;
	addressModeU?: string;
	addressModeV?: string;
	addressModeW?: string;
	magFilter?: string;
	minFilter?: string;
	mipmapFilter?: string;
	lodMinClamp?: number;
	lodMaxClamp?: number;
	compare?: string;
	maxAnisotropy?: number;
}

export interface GPUBindGroupLayoutEntryImpl {
	binding: number;
	visibility: number;
	buffer?: {
		type?: string;
		hasDynamicOffset?: boolean;
		minBindingSize?: number;
	};
	sampler?: { type?: string };
	texture?: {
		sampleType?: string;
		viewDimension?: string;
		multisampled?: boolean;
	};
	storageTexture?: {
		access?: string;
		format: string;
		viewDimension?: string;
	};
	externalTexture?: Record<string, never>;
}

export interface GPUBindGroupLayoutDescriptorImpl {
	label?: string;
	entries: GPUBindGroupLayoutEntryImpl[];
}

export interface GPUBindGroupEntryImpl {
	binding: number;
	resource: GPUSamplerImpl | GPUTextureViewImpl | { buffer: GPUBufferImpl; offset?: number; size?: number };
}

export interface GPUBindGroupDescriptorImpl {
	label?: string;
	layout: GPUBindGroupLayoutImpl;
	entries: GPUBindGroupEntryImpl[];
}

export interface GPUPipelineLayoutDescriptorImpl {
	label?: string;
	bindGroupLayouts: GPUBindGroupLayoutImpl[];
}

export interface GPUShaderModuleDescriptorImpl {
	label?: string;
	code: string;
}

export interface GPUCommandEncoderDescriptorImpl {
	label?: string;
}

export interface GPUCommandBufferDescriptorImpl {
	label?: string;
}

export interface GPUProgrammableStageImpl {
	module: GPUShaderModuleImpl;
	entryPoint?: string;
	constants?: Record<string, number>;
}

export interface GPUComputePipelineDescriptorImpl {
	label?: string;
	layout: GPUPipelineLayoutImpl | 'auto';
	compute: GPUProgrammableStageImpl;
}

export interface GPUVertexAttributeImpl {
	format: string;
	offset: number;
	shaderLocation: number;
}

export interface GPUVertexBufferLayoutImpl {
	arrayStride: number;
	stepMode?: string;
	attributes: GPUVertexAttributeImpl[];
}

export interface GPUVertexStateImpl extends GPUProgrammableStageImpl {
	buffers?: (GPUVertexBufferLayoutImpl | null)[];
}

export interface GPUPrimitiveStateImpl {
	topology?: string;
	stripIndexFormat?: string;
	frontFace?: string;
	cullMode?: string;
	unclippedDepth?: boolean;
}

export interface GPUStencilFaceStateImpl {
	compare?: string;
	failOp?: string;
	depthFailOp?: string;
	passOp?: string;
}

export interface GPUDepthStencilStateImpl {
	format: string;
	depthWriteEnabled?: boolean;
	depthCompare?: string;
	stencilFront?: GPUStencilFaceStateImpl;
	stencilBack?: GPUStencilFaceStateImpl;
	stencilReadMask?: number;
	stencilWriteMask?: number;
	depthBias?: number;
	depthBiasSlopeScale?: number;
	depthBiasClamp?: number;
}

export interface GPUMultisampleStateImpl {
	count?: number;
	mask?: number;
	alphaToCoverageEnabled?: boolean;
}

export interface GPUBlendComponentImpl {
	operation?: string;
	srcFactor?: string;
	dstFactor?: string;
}

export interface GPUBlendStateImpl {
	color: GPUBlendComponentImpl;
	alpha: GPUBlendComponentImpl;
}

export interface GPUColorTargetStateImpl {
	format: string;
	blend?: GPUBlendStateImpl;
	writeMask?: number;
}

export interface GPUFragmentStateImpl extends GPUProgrammableStageImpl {
	targets: (GPUColorTargetStateImpl | null)[];
}

export interface GPURenderPipelineDescriptorImpl {
	label?: string;
	layout: GPUPipelineLayoutImpl | 'auto';
	vertex: GPUVertexStateImpl;
	primitive?: GPUPrimitiveStateImpl;
	depthStencil?: GPUDepthStencilStateImpl;
	multisample?: GPUMultisampleStateImpl;
	fragment?: GPUFragmentStateImpl;
}

export interface GPURenderBundleDescriptorImpl {
	label?: string;
}

export interface GPURenderBundleEncoderDescriptorImpl {
	label?: string;
	colorFormats: string[];
	depthStencilFormat?: string;
	sampleCount?: number;
	depthReadOnly?: boolean;
	stencilReadOnly?: boolean;
}

export interface GPUQuerySetDescriptorImpl {
	label?: string;
	type: 'occlusion' | 'timestamp';
	count: number;
}

export interface GPUImageCopyBufferImpl {
	buffer: GPUBufferImpl;
	offset?: number;
	bytesPerRow?: number;
	rowsPerImage?: number;
}

export interface GPUImageCopyTextureImpl {
	texture: GPUTextureImpl;
	mipLevel?: number;
	origin?: GPUOrigin3dImpl;
	aspect?: string;
}

export interface GPUImageCopyTextureTaggedImpl extends GPUImageCopyTextureImpl {
	colorSpace?: string;
	premultipliedAlpha?: boolean;
}

export interface GPURenderPassColorAttachmentImpl {
	view: GPUTextureViewImpl;
	resolveTarget?: GPUTextureViewImpl;
	clearValue?: GPUColorImpl;
	loadOp: 'load' | 'clear';
	storeOp: 'store' | 'discard';
}

export interface GPURenderPassDepthStencilAttachmentImpl {
	view: GPUTextureViewImpl;
	depthClearValue?: number;
	depthLoadOp?: 'load' | 'clear';
	depthStoreOp?: 'store' | 'discard';
	depthReadOnly?: boolean;
	stencilClearValue?: number;
	stencilLoadOp?: 'load' | 'clear';
	stencilStoreOp?: 'store' | 'discard';
	stencilReadOnly?: boolean;
}

export interface GPURenderPassTimestampWriteImpl {
	querySet: GPUQuerySetImpl;
	queryIndex: number;
	location: 'beginning' | 'end';
}

export interface GPURenderPassDescriptorImpl {
	label?: string;
	colorAttachments: (GPURenderPassColorAttachmentImpl | null)[];
	depthStencilAttachment?: GPURenderPassDepthStencilAttachmentImpl;
	occlusionQuerySet?: GPUQuerySetImpl;
	timestampWrites?: GPURenderPassTimestampWriteImpl;
	maxDrawCount?: number;
}

export interface GPUComputePassTimestampWriteImpl {
	querySet: GPUQuerySetImpl;
	queryIndex: number;
	location: 'beginning' | 'end';
}

export interface GPUComputePassDescriptorImpl {
	label?: string;
	timestampWrites?: GPUComputePassTimestampWriteImpl;
}

export interface GPUCanvasConfigurationImpl {
	device: GPUDeviceImpl;
	format: string;
	usage?: number;
	viewFormats?: string[];
	colorSpace?: string;
	alphaMode?: 'opaque' | 'premultiplied';
}

export interface GPUCurrentTextureImpl {
	texture: GPUTextureImpl;
	status: SurfaceGetCurrentTextureStatusImpl;
}

export interface GPUCanvasContextImpl {
	configure(configuration: GPUCanvasConfigurationImpl): void;
	unconfigure(): void;
	getCurrentTexture(): GPUTextureImpl | null;
	presentSurface(): void;
}

export interface GPUInstanceImpl {
	requestAdapter(options: GPURequestAdapterOptionsImpl | null, callback: (adapter: GPUAdapterImpl | null) => void): void;
}
