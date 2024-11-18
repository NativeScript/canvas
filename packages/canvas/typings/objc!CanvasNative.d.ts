declare const enum CanvasAddressMode {
	ClampToEdge = 0,

	Repeat = 1,

	MirrorRepeat = 2,

	ClampToBorder = 3,
}

declare const enum CanvasAstcBlock {
	B4x4 = 0,

	B5x4 = 1,

	B5x5 = 2,

	B6x5 = 3,

	B6x6 = 4,

	B8x5 = 5,

	B8x6 = 6,

	B8x8 = 7,

	B10x5 = 8,

	B10x6 = 9,

	B10x8 = 10,

	B10x10 = 11,

	B12x10 = 12,

	B12x12 = 13,
}

declare const enum CanvasAstcChannel {
	Unorm = 0,

	UnormSrgb = 1,

	Hdr = 2,
}

declare const enum CanvasBindGroupEntryResource_Tag {
	Buffer = 0,

	Sampler = 1,

	TextureView = 2,
}

declare const enum CanvasBindingType_Tag {
	Buffer = 0,

	Sampler = 1,

	Texture = 2,

	StorageTexture = 3,
}

interface CanvasBlendComponent {
	src_factor: CanvasBlendFactor;
	dst_factor: CanvasBlendFactor;
	operation: CanvasBlendOperation;
}
declare var CanvasBlendComponent: interop.StructType<CanvasBlendComponent>;

declare const enum CanvasBlendFactor {
	Zero = 0,

	One = 1,

	Src = 2,

	OneMinusSrc = 3,

	SrcAlpha = 4,

	OneMinusSrcAlpha = 5,

	Dst = 6,

	OneMinusDst = 7,

	DstAlpha = 8,

	OneMinusDstAlpha = 9,

	SrcAlphaSaturated = 10,

	Constant = 11,

	OneMinusConstant = 12,

	Src1 = 13,

	OneMinusSrc1 = 14,

	Src1Alpha = 15,

	OneMinusSrc1Alpha = 16,
}

declare const enum CanvasBlendOperation {
	Add = 0,

	Subtract = 1,

	ReverseSubtract = 2,

	Min = 3,

	Max = 4,
}

interface CanvasBlendState {
	color: CanvasBlendComponent;
	alpha: CanvasBlendComponent;
}
declare var CanvasBlendState: interop.StructType<CanvasBlendState>;

interface CanvasBufferBinding {
	buffer: interop.Pointer | interop.Reference<any>;
	offset: number;
	size: number;
}
declare var CanvasBufferBinding: interop.StructType<CanvasBufferBinding>;

interface CanvasBufferBindingLayout {
	type_: CanvasBufferBindingType;
	has_dynamic_offset: boolean;
	min_binding_size: number;
}
declare var CanvasBufferBindingLayout: interop.StructType<CanvasBufferBindingLayout>;

declare const enum CanvasBufferBindingType {
	Uniform = 0,

	Storage = 1,

	ReadOnlyStorage = 2,
}

declare class CanvasCPUView extends UIView {
	static alloc(): CanvasCPUView; // inherited from NSObject

	static appearance(): CanvasCPUView; // inherited from UIAppearance

	/**
	 * @since 8.0
	 */
	static appearanceForTraitCollection(trait: UITraitCollection): CanvasCPUView; // inherited from UIAppearance

	/**
	 * @since 8.0
	 * @deprecated 9.0
	 */
	static appearanceForTraitCollectionWhenContainedIn(trait: UITraitCollection, ContainerClass: typeof NSObject): CanvasCPUView; // inherited from UIAppearance

	/**
	 * @since 9.0
	 */
	static appearanceForTraitCollectionWhenContainedInInstancesOfClasses(trait: UITraitCollection, containerTypes: NSArray<typeof NSObject> | (typeof NSObject)[]): CanvasCPUView; // inherited from UIAppearance

	/**
	 * @since 5.0
	 * @deprecated 9.0
	 */
	static appearanceWhenContainedIn(ContainerClass: typeof NSObject): CanvasCPUView; // inherited from UIAppearance

	/**
	 * @since 9.0
	 */
	static appearanceWhenContainedInInstancesOfClasses(containerTypes: NSArray<typeof NSObject> | (typeof NSObject)[]): CanvasCPUView; // inherited from UIAppearance

	static new(): CanvasCPUView; // inherited from NSObject

	ignorePixelScaling: boolean;
}

interface CanvasColor {
	r: number;
	g: number;
	b: number;
	a: number;
}
declare var CanvasColor: interop.StructType<CanvasColor>;

declare const enum CanvasCompareFunction {
	Never = 1,

	Less = 2,

	Equal = 3,

	LessEqual = 4,

	Greater = 5,

	NotEqual = 6,

	GreaterEqual = 7,

	Always = 8,
}

declare const enum CanvasCullMode {
	None = 0,

	Front = 1,

	Back = 2,
}

interface CanvasExtent3d {
	width: number;
	height: number;
	depth_or_array_layers: number;
}
declare var CanvasExtent3d: interop.StructType<CanvasExtent3d>;

declare const enum CanvasFillRule {
	NonZero = 0,

	EvenOdd = 1,
}

declare const enum CanvasFilterMode {
	Nearest = 0,

	Linear = 1,
}

declare const enum CanvasFit {
	None = 0,

	Fill = 1,

	FitX = 2,

	FitY = 3,

	ScaleDown = 4,
}

declare const enum CanvasFrontFace {
	Ccw = 0,

	Cw = 1,
}

declare class CanvasGLKView extends GLKView implements GLKViewDelegate {
	static alloc(): CanvasGLKView; // inherited from NSObject

	static appearance(): CanvasGLKView; // inherited from UIAppearance

	/**
	 * @since 8.0
	 */
	static appearanceForTraitCollection(trait: UITraitCollection): CanvasGLKView; // inherited from UIAppearance

	/**
	 * @since 8.0
	 * @deprecated 9.0
	 */
	static appearanceForTraitCollectionWhenContainedIn(trait: UITraitCollection, ContainerClass: typeof NSObject): CanvasGLKView; // inherited from UIAppearance

	/**
	 * @since 9.0
	 */
	static appearanceForTraitCollectionWhenContainedInInstancesOfClasses(trait: UITraitCollection, containerTypes: NSArray<typeof NSObject> | (typeof NSObject)[]): CanvasGLKView; // inherited from UIAppearance

	/**
	 * @since 5.0
	 * @deprecated 9.0
	 */
	static appearanceWhenContainedIn(ContainerClass: typeof NSObject): CanvasGLKView; // inherited from UIAppearance

	/**
	 * @since 9.0
	 */
	static appearanceWhenContainedInInstancesOfClasses(containerTypes: NSArray<typeof NSObject> | (typeof NSObject)[]): CanvasGLKView; // inherited from UIAppearance

	static new(): CanvasGLKView; // inherited from NSObject

	readonly canvas: NSCCanvas;

	readonly debugDescription: string; // inherited from NSObjectProtocol

	readonly description: string; // inherited from NSObjectProtocol

	readonly hash: number; // inherited from NSObjectProtocol

	readonly isProxy: boolean; // inherited from NSObjectProtocol

	readonly superclass: typeof NSObject; // inherited from NSObjectProtocol

	readonly; // inherited from NSObjectProtocol

	class(): typeof NSObject;

	conformsToProtocol(aProtocol: any /* Protocol */): boolean;

	glkViewDrawInRect(view: GLKView, rect: CGRect): void;

	isEqual(object: any): boolean;

	isKindOfClass(aClass: typeof NSObject): boolean;

	isMemberOfClass(aClass: typeof NSObject): boolean;

	performSelector(aSelector: string): any;

	performSelectorWithObject(aSelector: string, object: any): any;

	performSelectorWithObjectWithObject(aSelector: string, object1: any, object2: any): any;

	respondsToSelector(aSelector: string): boolean;

	retainCount(): number;

	self(): this;
}

declare const enum CanvasGPUAutoLayoutMode {
	Auto = 0,
}

declare const enum CanvasGPUErrorFilter {
	OutOfMemory = 0,

	Validation = 1,

	Internal = 2,
}

declare const enum CanvasGPUErrorType {
	None = 0,

	Lost = 1,

	OutOfMemory = 2,

	Validation = 3,

	Internal = 4,
}

declare const enum CanvasGPUPipelineLayoutOrGPUAutoLayoutMode_Tag {
	Layout = 0,

	Auto = 1,
}

declare const enum CanvasGPUPowerPreference {
	None = 0,

	LowPower = 1,

	HighPerformance = 2,
}

declare const enum CanvasGPUPresentMode {
	AutoVsync = 0,

	AutoNoVsync = 1,

	Fifo = 2,

	FifoRelaxed = 3,

	Immediate = 4,

	Mailbox = 5,
}

interface CanvasGPURequestAdapterOptions {
	power_preference: CanvasGPUPowerPreference;
	force_fallback_adapter: boolean;
}
declare var CanvasGPURequestAdapterOptions: interop.StructType<CanvasGPURequestAdapterOptions>;

interface CanvasGPUSupportedLimits {
	max_texture_dimension_1d: number;
	max_texture_dimension_2d: number;
	max_texture_dimension_3d: number;
	max_texture_array_layers: number;
	max_bind_groups: number;
	max_bindings_per_bind_group: number;
	max_dynamic_uniform_buffers_per_pipeline_layout: number;
	max_dynamic_storage_buffers_per_pipeline_layout: number;
	max_sampled_textures_per_shader_stage: number;
	max_samplers_per_shader_stage: number;
	max_storage_buffers_per_shader_stage: number;
	max_storage_textures_per_shader_stage: number;
	max_uniform_buffers_per_shader_stage: number;
	max_uniform_buffer_binding_size: number;
	max_storage_buffer_binding_size: number;
	max_vertex_buffers: number;
	max_buffer_size: number;
	max_vertex_attributes: number;
	max_vertex_buffer_array_stride: number;
	min_uniform_buffer_offset_alignment: number;
	min_storage_buffer_offset_alignment: number;
	max_inter_stage_shader_components: number;
	max_color_attachments: number;
	max_color_attachment_bytes_per_sample: number;
	max_compute_workgroup_storage_size: number;
	max_compute_invocations_per_workgroup: number;
	max_compute_workgroup_size_x: number;
	max_compute_workgroup_size_y: number;
	max_compute_workgroup_size_z: number;
	max_compute_workgroups_per_dimension: number;
	min_subgroup_size: number;
	max_subgroup_size: number;
	max_push_constant_size: number;
	max_non_sampler_bindings: number;
}
declare var CanvasGPUSupportedLimits: interop.StructType<CanvasGPUSupportedLimits>;

declare const enum CanvasGPUSurfaceAlphaMode {
	Auto = 0,

	Opaque = 1,

	PreMultiplied = 2,

	PostMultiplied = 3,

	Inherit = 4,
}

interface CanvasGPUTextureFormatAstc_Body {
	block: CanvasAstcBlock;
	channel: CanvasAstcChannel;
}
declare var CanvasGPUTextureFormatAstc_Body: interop.StructType<CanvasGPUTextureFormatAstc_Body>;

declare const enum CanvasGPUTextureFormat_Tag {
	R8Unorm = 0,

	R8Snorm = 1,

	R8Uint = 2,

	R8Sint = 3,

	R16Uint = 4,

	R16Sint = 5,

	R16Unorm = 6,

	R16Snorm = 7,

	R16Float = 8,

	Rg8Unorm = 9,

	Rg8Snorm = 10,

	Rg8Uint = 11,

	Rg8Sint = 12,

	R32Uint = 13,

	R32Sint = 14,

	R32Float = 15,

	Rg16Uint = 16,

	Rg16Sint = 17,

	Rg16Unorm = 18,

	Rg16Snorm = 19,

	Rg16Float = 20,

	Rgba8Unorm = 21,

	Rgba8UnormSrgb = 22,

	Rgba8Snorm = 23,

	Rgba8Uint = 24,

	Rgba8Sint = 25,

	Bgra8Unorm = 26,

	Bgra8UnormSrgb = 27,

	Rgb9e5Ufloat = 28,

	Rgb10a2Uint = 29,

	Rgb10a2Unorm = 30,

	Rg11b10UFloat = 31,

	Rg32Uint = 32,

	Rg32Sint = 33,

	Rg32Float = 34,

	Rgba16Uint = 35,

	Rgba16Sint = 36,

	Rgba16Unorm = 37,

	Rgba16Snorm = 38,

	Rgba16Float = 39,

	Rgba32Uint = 40,

	Rgba32Sint = 41,

	Rgba32Float = 42,

	Stencil8 = 43,

	Depth16Unorm = 44,

	Depth24Plus = 45,

	Depth24PlusStencil8 = 46,

	Depth32Float = 47,

	Depth32FloatStencil8 = 48,

	NV12 = 49,

	Bc1RgbaUnorm = 50,

	Bc1RgbaUnormSrgb = 51,

	Bc2RgbaUnorm = 52,

	Bc2RgbaUnormSrgb = 53,

	Bc3RgbaUnorm = 54,

	Bc3RgbaUnormSrgb = 55,

	Bc4RUnorm = 56,

	Bc4RSnorm = 57,

	Bc5RgUnorm = 58,

	Bc5RgSnorm = 59,

	Bc6hRgbUfloat = 60,

	Bc6hRgbFloat = 61,

	Bc7RgbaUnorm = 62,

	Bc7RgbaUnormSrgb = 63,

	Etc2Rgb8Unorm = 64,

	Etc2Rgb8UnormSrgb = 65,

	Etc2Rgb8A1Unorm = 66,

	Etc2Rgb8A1UnormSrgb = 67,

	Etc2Rgba8Unorm = 68,

	Etc2Rgba8UnormSrgb = 69,

	EacR11Unorm = 70,

	EacR11Snorm = 71,

	EacRg11Unorm = 72,

	EacRg11Snorm = 73,

	Astc = 74,
}

interface CanvasImageCopyBuffer {
	buffer: interop.Pointer | interop.Reference<any>;
	offset: number;
	bytes_per_row: number;
	rows_per_image: number;
}
declare var CanvasImageCopyBuffer: interop.StructType<CanvasImageCopyBuffer>;

interface CanvasImageCopyCanvasRenderingContext2D {
	source: interop.Pointer | interop.Reference<any>;
	origin: CanvasOrigin2d;
	flip_y: boolean;
}
declare var CanvasImageCopyCanvasRenderingContext2D: interop.StructType<CanvasImageCopyCanvasRenderingContext2D>;

interface CanvasImageCopyExternalImage {
	source: interop.Pointer | interop.Reference<any>;
	source_size: number;
	origin: CanvasOrigin2d;
	flip_y: boolean;
	width: number;
	height: number;
}
declare var CanvasImageCopyExternalImage: interop.StructType<CanvasImageCopyExternalImage>;

interface CanvasImageCopyImageAsset {
	source: interop.Pointer | interop.Reference<any>;
	origin: CanvasOrigin2d;
	flip_y: boolean;
}
declare var CanvasImageCopyImageAsset: interop.StructType<CanvasImageCopyImageAsset>;

interface CanvasImageCopyTexture {
	texture: interop.Pointer | interop.Reference<any>;
	mip_level: number;
	origin: CanvasOrigin3d;
	aspect: CanvasTextureAspect;
}
declare var CanvasImageCopyTexture: interop.StructType<CanvasImageCopyTexture>;

interface CanvasImageCopyWebGL {
	source: interop.Pointer | interop.Reference<any>;
	origin: CanvasOrigin2d;
	flip_y: boolean;
}
declare var CanvasImageCopyWebGL: interop.StructType<CanvasImageCopyWebGL>;

interface CanvasImageDataLayout {
	offset: number;
	bytes_per_row: number;
	rows_per_image: number;
}
declare var CanvasImageDataLayout: interop.StructType<CanvasImageDataLayout>;

interface CanvasImageSubresourceRange {
	aspect: CanvasTextureAspect;
	base_mip_level: number;
	mip_level_count: number;
	base_array_layer: number;
	array_layer_count: number;
}
declare var CanvasImageSubresourceRange: interop.StructType<CanvasImageSubresourceRange>;

declare const enum CanvasIndexFormat {
	Uint16 = 0,

	Uint32 = 1,
}

declare const enum CanvasLoadOp {
	Clear = 0,

	Load = 1,
}

interface CanvasMultisampleState {
	count: number;
	mask: number;
	alpha_to_coverage_enabled: boolean;
}
declare var CanvasMultisampleState: interop.StructType<CanvasMultisampleState>;

declare var CanvasNativeVersionNumber: number;

declare var CanvasNativeVersionString: interop.Reference<number>;

declare const enum CanvasOptionalBlendState_Tag {
	None = 0,

	Some = 1,
}

declare const enum CanvasOptionalCompareFunction_Tag {
	None = 0,

	Some = 1,
}

declare const enum CanvasOptionalGPUTextureFormat_Tag {
	None = 0,

	Some = 1,
}

declare const enum CanvasOptionalIndexFormat_Tag {
	None = 0,

	Some = 1,
}

declare const enum CanvasOptionalLoadOp_Tag {
	None = 0,

	Some = 1,
}

declare const enum CanvasOptionalStoreOp_Tag {
	None = 0,

	Some = 1,
}

declare const enum CanvasOptionalTextureViewDimension {
	None = 0,

	D1 = 1,

	D2 = 2,

	D2Array = 3,

	Cube = 4,

	CubeArray = 5,

	D3 = 6,
}

interface CanvasOrigin2d {
	x: number;
	y: number;
}
declare var CanvasOrigin2d: interop.StructType<CanvasOrigin2d>;

interface CanvasOrigin3d {
	x: number;
	y: number;
	z: number;
}
declare var CanvasOrigin3d: interop.StructType<CanvasOrigin3d>;

interface CanvasPassChannelColor {
	load_op: CanvasLoadOp;
	store_op: CanvasStoreOp;
	clear_value: CanvasColor;
	read_only: boolean;
}
declare var CanvasPassChannelColor: interop.StructType<CanvasPassChannelColor>;

declare const enum CanvasPrimitiveTopology {
	PointList = 0,

	LineList = 1,

	LineStrip = 2,

	TriangleList = 3,

	TriangleStrip = 4,
}

interface CanvasProgrammableStage {
	module: interop.Pointer | interop.Reference<any>;
	entry_point: interop.Pointer | interop.Reference<any>;
	constants: interop.Pointer | interop.Reference<any>;
}
declare var CanvasProgrammableStage: interop.StructType<CanvasProgrammableStage>;

declare const enum CanvasQueryType {
	Occlusion = 0,

	Timestamp = 1,
}

interface CanvasRenderPassColorAttachment {
	view: interop.Pointer | interop.Reference<any>;
	resolve_target: interop.Pointer | interop.Reference<any>;
	channel: CanvasPassChannelColor;
}
declare var CanvasRenderPassColorAttachment: interop.StructType<CanvasRenderPassColorAttachment>;

declare const enum CanvasRepetition {
	Repeat = 0,

	RepeatX = 1,

	RepeatY = 2,

	NoRepeat = 3,
}

interface CanvasSamplerBindingLayout {
	type_: CanvasSamplerBindingType;
}
declare var CanvasSamplerBindingLayout: interop.StructType<CanvasSamplerBindingLayout>;

declare const enum CanvasSamplerBindingType {
	Filtering = 0,

	NonFiltering = 1,

	Comparison = 2,
}

interface CanvasStencilFaceState {
	compare: CanvasCompareFunction;
	fail_op: CanvasStencilOperation;
	depth_fail_op: CanvasStencilOperation;
	pass_op: CanvasStencilOperation;
}
declare var CanvasStencilFaceState: interop.StructType<CanvasStencilFaceState>;

declare const enum CanvasStencilOperation {
	Keep = 0,

	Zero = 1,

	Replace = 2,

	Invert = 3,

	IncrementClamp = 4,

	DecrementClamp = 5,

	IncrementWrap = 6,

	DecrementWrap = 7,
}

declare const enum CanvasStorageTextureAccess {
	WriteOnly = 0,

	ReadOnly = 1,

	ReadWrite = 2,
}

declare const enum CanvasStoreOp {
	Discard = 0,

	Store = 1,
}

interface CanvasSurfaceCapabilities {
	formats: interop.Pointer | interop.Reference<any>;
	present_modes: interop.Pointer | interop.Reference<any>;
	alpha_modes: interop.Pointer | interop.Reference<any>;
	usages: number;
}
declare var CanvasSurfaceCapabilities: interop.StructType<CanvasSurfaceCapabilities>;

declare const enum CanvasTextureAspect {
	All = 0,

	StencilOnly = 1,

	DepthOnly = 2,

	Plane0 = 3,

	Plane1 = 4,

	Plane2 = 5,
}

interface CanvasTextureBindingLayout {
	sample_type: CanvasTextureSampleType;
	view_dimension: CanvasTextureViewDimension;
	multisampled: boolean;
}
declare var CanvasTextureBindingLayout: interop.StructType<CanvasTextureBindingLayout>;

declare const enum CanvasTextureDimension {
	D1 = 0,

	D2 = 1,

	D3 = 2,
}

declare const enum CanvasTextureSampleType {
	Float = 0,

	UnfilterableFloat = 1,

	Depth = 2,

	Sint = 3,

	Uint = 4,
}

declare const enum CanvasTextureViewDimension {
	D1 = 0,

	D2 = 1,

	D2Array = 2,

	Cube = 3,

	CubeArray = 4,

	D3 = 5,
}

interface CanvasVertexAttribute {
	format: CanvasVertexFormat;
	offset: number;
	shader_location: number;
}
declare var CanvasVertexAttribute: interop.StructType<CanvasVertexAttribute>;

interface CanvasVertexBufferLayout {
	array_stride: number;
	step_mode: CanvasVertexStepMode;
	attributes: interop.Pointer | interop.Reference<CanvasVertexAttribute>;
	attributes_size: number;
}
declare var CanvasVertexBufferLayout: interop.StructType<CanvasVertexBufferLayout>;

declare const enum CanvasVertexFormat {
	Uint8x2 = 0,

	Uint8x4 = 1,

	Sint8x2 = 2,

	Sint8x4 = 3,

	Unorm8x2 = 4,

	Unorm8x4 = 5,

	Snorm8x2 = 6,

	Snorm8x4 = 7,

	Uint16x2 = 8,

	Uint16x4 = 9,

	Sint16x2 = 10,

	Sint16x4 = 11,

	Unorm16x2 = 12,

	Unorm16x4 = 13,

	Snorm16x2 = 14,

	Snorm16x4 = 15,

	Float16x2 = 16,

	Float16x4 = 17,

	Float32 = 18,

	Float32x2 = 19,

	Float32x3 = 20,

	Float32x4 = 21,

	Uint32 = 22,

	Uint32x2 = 23,

	Uint32x3 = 24,

	Uint32x4 = 25,

	Sint32 = 26,

	Sint32x2 = 27,

	Sint32x3 = 28,

	Sint32x4 = 29,

	Float64 = 30,

	Float64x2 = 31,

	Float64x3 = 32,

	Float64x4 = 33,

	Unorm10_10_10_2 = 34,
}

interface CanvasVertexState {
	module: interop.Pointer | interop.Reference<any>;
	entry_point: interop.Pointer | interop.Reference<any>;
	constants: interop.Pointer | interop.Reference<any>;
	buffers: interop.Pointer | interop.Reference<CanvasVertexBufferLayout>;
	buffers_size: number;
}
declare var CanvasVertexState: interop.StructType<CanvasVertexState>;

declare const enum CanvasVertexStepMode {
	Vertex = 0,

	Instance = 1,
}

interface FileHelperMime {
	mime_type: interop.Pointer | interop.Reference<any>;
	extension: interop.Pointer | interop.Reference<any>;
}
declare var FileHelperMime: interop.StructType<FileHelperMime>;

declare const enum GLConstants {
	UnPackFlipYWebGL = 37440,

	UnpackPremultiplyAlphaWebGL = 37441,

	UnpackColorSpaceConversionWebGL = 37443,
}

declare const enum GPUMapMode {
	Read = 0,

	Write = 1,
}

declare const enum ImageBitmapColorSpaceConversion {
	Default = 0,

	None = 1,
}

declare const enum ImageBitmapPremultiplyAlpha {
	Default = 0,

	Premultiply = 1,

	AlphaNone = 2,
}

declare const enum ImageBitmapResizeQuality {
	Low = 0,

	Medium = 1,

	High = 2,

	Pixelated = 3,
}

declare const enum InvalidateState {
	None = 0,

	Pending = 1,

	Invalidating = 2,
}

declare class NSCCanvas extends UIView {
	static alloc(): NSCCanvas; // inherited from NSObject

	static appearance(): NSCCanvas; // inherited from UIAppearance

	/**
	 * @since 8.0
	 */
	static appearanceForTraitCollection(trait: UITraitCollection): NSCCanvas; // inherited from UIAppearance

	/**
	 * @since 8.0
	 * @deprecated 9.0
	 */
	static appearanceForTraitCollectionWhenContainedIn(trait: UITraitCollection, ContainerClass: typeof NSObject): NSCCanvas; // inherited from UIAppearance

	/**
	 * @since 9.0
	 */
	static appearanceForTraitCollectionWhenContainedInInstancesOfClasses(trait: UITraitCollection, containerTypes: NSArray<typeof NSObject> | (typeof NSObject)[]): NSCCanvas; // inherited from UIAppearance

	/**
	 * @since 5.0
	 * @deprecated 9.0
	 */
	static appearanceWhenContainedIn(ContainerClass: typeof NSObject): NSCCanvas; // inherited from UIAppearance

	/**
	 * @since 9.0
	 */
	static appearanceWhenContainedInInstancesOfClasses(containerTypes: NSArray<typeof NSObject> | (typeof NSObject)[]): NSCCanvas; // inherited from UIAppearance

	static getBoundingClientRect(view: UIView, buffer: interop.Pointer | interop.Reference<any>): void;

	static getViews(): NSMapTable<string, NSCCanvas>;

	static new(): NSCCanvas; // inherited from NSObject

	static setForceGL(value: boolean): void;

	autoScale: boolean;

	readonly drawingBufferHeight: number;

	readonly drawingBufferWidth: number;

	fit: CanvasFit;

	readonly height: number;

	ignoreTouchEvents: boolean;

	readonly is2D: boolean;

	readonly nativeContext: number;

	surfaceHeight: number;

	surfaceWidth: number;

	touchEventListener: (p1: string, p2: UIGestureRecognizer) => void;

	weblikeScale: boolean;

	readonly width: number;

	static forceGL: boolean;

	static readonly store: NSMutableDictionary<any, any>;

	context2DConic(context: number): void;

	context2DPathTest(context: number): void;

	context2DTest(context: number): void;

	create2DContext(alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number, premultipliedAlpha: boolean, preserveDrawingBuffer: boolean, stencil: boolean, desynchronized: boolean, xrCompatible: boolean, fontColor: number): number;

	forceLayout(width: number, height: number): void;

	getGlViewPtr(): interop.Pointer | interop.Reference<any>;

	getMtlLayerPtr(): interop.Pointer | interop.Reference<any>;

	getMtlViewPtr(): interop.Pointer | interop.Reference<any>;

	initContext(type: string, alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number, premultipliedAlpha: boolean, preserveDrawingBuffer: boolean, stencil: boolean, desynchronized: boolean, xrCompatible: boolean): void;

	initWebGPUContext(instance: number): void;

	render(): boolean;

	setListener(listener: NSCCanvasListener): void;

	snapshot(flip: boolean): UIImage;

	toDataURL(format: string, quality: number): string;
}

interface NSCCanvasListener {
	contextReady(): void;
}
declare var NSCCanvasListener: {
	prototype: NSCCanvasListener;
};

declare class NSCCanvasRenderingContext extends NSObject {
	static alloc(): NSCCanvasRenderingContext; // inherited from NSObject

	static new(): NSCCanvasRenderingContext; // inherited from NSObject
}

declare class NSCCanvasRenderingContext2D extends NSCCanvasRenderingContext {
	static alloc(): NSCCanvasRenderingContext2D; // inherited from NSObject

	static createPattern(context: number, src: UIImage, repetition: string): number;

	static drawImage(context: number, image: UIImage, dx: number, dy: number): boolean;

	static new(): NSCCanvasRenderingContext2D; // inherited from NSObject
}

declare class NSCCanvasUtils extends NSObject {
	static alloc(): NSCCanvasUtils; // inherited from NSObject

	static createImage(texturecache: any, buffer: any, textureAttributes: NSDictionary<any, any>, target: number, internalFormat: number, width: number, height: number, format: number, type: number, planeIndex: number): any;

	static createTextureCache(): any;

	static drawFrame(player: AVPlayer, output: AVPlayerItemVideoOutput, videoSize: CGSize, internalFormat: number, format: number, flipYWebGL: boolean): void;

	static new(): NSCCanvasUtils; // inherited from NSObject

	static setupRender(): NSCRender;

	static writeToFileError(data: NSData, path: string): boolean;
}

declare class NSCFontDescriptors extends NSObject {
	static alloc(): NSCFontDescriptors; // inherited from NSObject

	static new(): NSCFontDescriptors; // inherited from NSObject

	constructor(o: { family: string });

	initWithFamily(family: string): this;

	setFontStyle(value: string): void;

	setFontWeight(value: string): void;

	update(value: string): void;
}

declare const enum NSCFontDisplay {
	Auto = 0,

	Block = 1,

	Fallback = 2,

	Optional = 3,

	Swap = 4,
}

declare class NSCFontFace extends NSObject {
	static alloc(): NSCFontFace; // inherited from NSObject

	static importFromRemoteWithUrlLoadCallback(url: string, load: boolean, callback: (p1: NSArray<NSCFontFace>, p2: string) => void): void;

	static loadFromStyleWithStyle(style: string): NSCFontFace;

	static new(): NSCFontFace; // inherited from NSObject

	readonly ascentOverride: string;

	readonly descentOverride: string;

	display: NSCFontDisplay;

	readonly family: string;

	readonly font: any;

	readonly fontData: NSData;

	status: NSCFontFaceStatus;

	style: string;

	weight: NSCFontWeight;

	constructor();

	constructor(o: { data: string });

	constructor(o: { family: string });

	constructor(o: { family: string; data: NSData });

	constructor(o: { family: string; source: string });

	init(family: string, source: string, descriptors: NSCFontDescriptors): this;

	initData(family: string, data: NSData, descriptors: NSCFontDescriptors): this;

	initWithFamily(family: string): this;

	initWithFamilyData(family: string, source: NSData): this;

	initWithFamilySource(family: string, source: string): this;

	load(callback: (p1: string) => void): void;

	setFontDisplayWithValue(value: string): void;

	setFontStyleWithValueAngle(value: string, angle: string): void;

	setFontWeightWithValue(value: string): void;

	updateDescriptorWithValue(value: string): void;
}

declare class NSCFontFaceSet extends NSObject {
	static alloc(): NSCFontFaceSet; // inherited from NSObject

	static new(): NSCFontFaceSet; // inherited from NSObject

	onStatus: (p1: NSCFontFaceSetStatus) => void;

	readonly size: number;

	status: NSCFontFaceSetStatus;

	static readonly instance: NSCFontFaceSet;

	add(font: NSCFontFace): void;

	array(): NSArray<any>;

	check(font: string, text: string): boolean;

	clear(): void;

	delete(font: NSCFontFace): void;

	iter(): NSEnumerator<any>;

	load(font: string, text: string, callback: (p1: NSArray<NSCFontFace>, p2: string) => void): void;
}

declare const enum NSCFontFaceSetStatus {
	Loading = 0,

	Loaded = 1,
}

declare const enum NSCFontFaceStatus {
	Unloaded = 0,

	Loading = 1,

	Loaded = 2,

	Error = 3,
}

declare const enum NSCFontWeight {
	Thin = 0,

	ExtraLight = 1,

	Light = 2,

	Normal = 3,

	Medium = 4,

	SemiBold = 5,

	Bold = 6,

	ExtraBold = 7,

	Black = 8,
}

declare class NSCImageAsset extends NSObject {
	static alloc(): NSCImageAsset; // inherited from NSObject

	static loadImageFromImage(context: number, image: UIImage, callback: (p1: boolean) => void): void;

	static loadImageFromImageSync(context: number, image: UIImage): boolean;

	static loadImageFromPath(asset: number, path: string, callback: (p1: boolean) => void): void;

	static loadImageFromPathSync(asset: number, path: string): boolean;

	static new(): NSCImageAsset; // inherited from NSObject
}

declare class NSCImageBitmap extends NSObject {
	static alloc(): NSCImageBitmap; // inherited from NSObject

	static new(): NSCImageBitmap; // inherited from NSObject

	static readonly queue: NSObject & OS_dispatch_queue;
}

declare class NSCMTLView extends UIView {
	static alloc(): NSCMTLView; // inherited from NSObject

	static appearance(): NSCMTLView; // inherited from UIAppearance

	/**
	 * @since 8.0
	 */
	static appearanceForTraitCollection(trait: UITraitCollection): NSCMTLView; // inherited from UIAppearance

	/**
	 * @since 8.0
	 * @deprecated 9.0
	 */
	static appearanceForTraitCollectionWhenContainedIn(trait: UITraitCollection, ContainerClass: typeof NSObject): NSCMTLView; // inherited from UIAppearance

	/**
	 * @since 9.0
	 */
	static appearanceForTraitCollectionWhenContainedInInstancesOfClasses(trait: UITraitCollection, containerTypes: NSArray<typeof NSObject> | (typeof NSObject)[]): NSCMTLView; // inherited from UIAppearance

	/**
	 * @since 5.0
	 * @deprecated 9.0
	 */
	static appearanceWhenContainedIn(ContainerClass: typeof NSObject): NSCMTLView; // inherited from UIAppearance

	/**
	 * @since 9.0
	 */
	static appearanceWhenContainedInInstancesOfClasses(containerTypes: NSArray<typeof NSObject> | (typeof NSObject)[]): NSCMTLView; // inherited from UIAppearance

	static new(): NSCMTLView; // inherited from NSObject

	canvas: UIView;

	drawableSize: CGSize;

	queue: MTLCommandQueue;

	sampleCount: number;

	state: number;

	getDevicePtr(): interop.Pointer | interop.Reference<any>;

	getQueuePtr(): interop.Pointer | interop.Reference<any>;

	present(): void;

	setup(): void;
}

declare class NSCRender extends NSObject {
	static alloc(): NSCRender; // inherited from NSObject

	static new(): NSCRender; // inherited from NSObject
}

declare const enum NSCSurfaceState {
	kNone = 0,

	kPending = 1,

	kInvalidating = 2,
}

declare class NSCVideoFrame extends NSObject {
	static alloc(): NSCVideoFrame; // inherited from NSObject

	static new(): NSCVideoFrame; // inherited from NSObject
}

declare const enum NSCVideoFrameFormat {
	I420 = 0,

	I420A = 1,

	I422 = 2,

	I444 = 3,

	NV12 = 4,

	RGBA = 5,

	RGBX = 6,

	BGRA = 7,

	BGRX = 8,
}

declare class NSCWebGLRenderingContext extends NSObject {
	static alloc(): NSCWebGLRenderingContext; // inherited from NSObject

	static new(): NSCWebGLRenderingContext; // inherited from NSObject

	static texImage2D(context: number, target: number, level: number, internalformat: number, format: number, type: number, data: string | interop.Pointer | interop.Reference<any>, size: number, dimensions: CGSize, flipY: boolean): void;

	static texSubImage2D(context: number, target: number, level: number, xoffset: number, yoffset: number, format: number, type: number, data: string | interop.Pointer | interop.Reference<any>, size: number, dimensions: CGSize, flipY: boolean): void;
}

declare class NSSCanvasHelpers extends NSObject {
	static alloc(): NSSCanvasHelpers; // inherited from NSObject

	static create2DContext(view: NSCCanvas, width: number, height: number, alpha: boolean, density: number, fontColor: number, ppi: number, direction: number): number;

	static create2DContextMetal(view: NSCCanvas, alpha: boolean, density: number, fontColor: number, ppi: number, direction: number): number;

	static createPattern(context: number, image: UIImage, repetition: string): number;

	static deleteFile(path: string, callback: (p1: NSError, p2: boolean) => void): void;

	static drawImageWithContextImageDxDy(context: number, image: UIImage, dx: number, dy: number): boolean;

	static drawImageWithContextImageDxDyDwDh(context: number, image: UIImage, dx: number, dy: number, dw: number, dh: number): boolean;

	static drawImageWithContextImageSxSySwShDxDyDwDh(context: number, image: UIImage, sx: number, sy: number, sw: number, sh: number, dx: number, dy: number, dw: number, dh: number): boolean;

	static flush2DContext(context: number): void;

	static flush2DContextAndSyncCPU(context: number): void;

	static flushWebGL(context: number): boolean;

	static getBytesFromUIImage(image: UIImage): NSMutableData;

	static getPixelsPerInchForCurrentDevice(): string;

	static handleBase64Image(mime: string, dir: string, base64: string, callback: (p1: string, p2: string) => void): void;

	static initWebGLWithView(view: NSCCanvas, alpha: boolean, antialias: boolean, depth: boolean, fail_if_major_performance_caveat: boolean, power_preference: number, premultiplied_alpha: boolean, preserve_drawing_buffer: boolean, stencil: boolean, desynchronized: boolean, xr_compatible: boolean, version: number): number;

	static initWebGLWithWidthAndHeight(width: number, height: number, alpha: boolean, antialias: boolean, depth: boolean, fail_if_major_performance_caveat: boolean, power_preference: number, premultiplied_alpha: boolean, preserve_drawing_buffer: boolean, stencil: boolean, desynchronized: boolean, xr_compatible: boolean, version: number): number;

	static initWebGPUWithView(instance: number, view: NSCCanvas, width: number, height: number): number;

	static initWebGPUWithViewLayer(instance: number, view: NSCCanvas, width: number, height: number): number;

	static loadImageAssetWithContext(asset: number, image: UIImage): boolean;

	static loadImageAssetWithPath(asset: number, path: string): boolean;

	static new(): NSSCanvasHelpers; // inherited from NSObject

	static presentDrawable(context: number): void;

	static readFile(path: string, callback: (p1: string, p2: NSData) => void): void;

	static releaseWebGL(context: number): void;

	static resize2DContext(context: number, width: number, height: number): void;

	static resizeWebGPUWithView(context: number, view: NSCCanvas, width: number, height: number): void;

	static updateWebGLSurfaceWithView(view: number, width: number, height: number, context: number): void;

	static writeFile(data: NSData, path: string, callback: (p1: string, p2: string) => void): void;
}

declare const enum PaintStyleType {
	Color = 0,

	Gradient = 1,

	Pattern = 2,
}

declare const enum SurfaceGetCurrentTextureStatus {
	Success = 0,

	Timeout = 1,

	Outdated = 2,

	Lost = 3,

	OutOfMemory = 4,

	DeviceLost = 5,

	Force32 = 2147483647,
}

declare const enum TextBaseLine {
	TOP = 0,

	HANGING = 1,

	MIDDLE = 2,

	ALPHABETIC = 3,

	IDEOGRAPHIC = 4,

	BOTTOM = 5,
}

declare const enum WebGLExtensionType {
	WebGLExtensionTypeEXT_blend_minmax = 0,

	WebGLExtensionTypeEXT_color_buffer_half_float = 1,

	WebGLExtensionTypeEXT_disjoint_timer_query = 2,

	WebGLExtensionTypeEXT_sRGB = 3,

	WebGLExtensionTypeEXT_shader_texture_lod = 4,

	WebGLExtensionTypeEXT_texture_filter_anisotropic = 5,

	WebGLExtensionTypeOES_element_index_uint = 6,

	WebGLExtensionTypeOES_standard_derivatives = 7,

	WebGLExtensionTypeOES_texture_float = 8,

	WebGLExtensionTypeOES_texture_float_linear = 9,

	WebGLExtensionTypeOES_texture_half_float = 10,

	WebGLExtensionTypeOES_texture_half_float_linear = 11,

	WebGLExtensionTypeOES_vertex_array_object = 12,

	WebGLExtensionTypeWEBGL_color_buffer_float = 13,

	WebGLExtensionTypeWEBGL_compressed_texture_atc = 14,

	WebGLExtensionTypeWEBGL_compressed_texture_etc1 = 15,

	WebGLExtensionTypeWEBGL_compressed_texture_s3tc = 16,

	WebGLExtensionTypeWEBGL_compressed_texture_s3tc_srgb = 17,

	WebGLExtensionTypeWEBGL_compressed_texture_etc = 18,

	WebGLExtensionTypeWEBGL_compressed_texture_pvrtc = 19,

	WebGLExtensionTypeWEBGL_lose_context = 20,

	WebGLExtensionTypeANGLE_instanced_arrays = 21,

	WebGLExtensionTypeWEBGL_depth_texture = 22,

	WebGLExtensionTypeWEBGL_draw_buffers = 23,

	WebGLExtensionTypeOES_fbo_render_mipmap = 24,

	WebGLExtensionTypeNone = 25,
}

declare const enum WebGLResultType {
	Boolean = 0,

	I32Array = 1,

	U32Array = 2,

	F32Array = 3,

	BooleanArray = 4,

	U32 = 5,

	I32 = 6,

	F32 = 7,

	String = 8,

	None = 9,
}

declare function canvas_native_ccow_get_bytes(cow: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_ccow_get_length(cow: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_ccow_release(cow: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_2d_conic_test(context: number): void;

declare function canvas_native_context_2d_path_test(context: number): void;

declare function canvas_native_context_2d_test(context: number): void;

declare function canvas_native_context_arc(context: interop.Pointer | interop.Reference<any>, x: number, y: number, radius: number, start_angle: number, end_angle: number, anticlockwise: boolean): void;

declare function canvas_native_context_arc_to(context: interop.Pointer | interop.Reference<any>, x1: number, y1: number, x2: number, y2: number, radius: number): void;

declare function canvas_native_context_attributes_destroy(attr: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_begin_path(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_bezier_curve_to(context: interop.Pointer | interop.Reference<any>, cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void;

declare function canvas_native_context_clear_rect(context: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number): void;

declare function canvas_native_context_clip(context: interop.Pointer | interop.Reference<any>, path: interop.Pointer | interop.Reference<any>, rule: CanvasFillRule): void;

declare function canvas_native_context_clip_rule(context: interop.Pointer | interop.Reference<any>, rule: CanvasFillRule): void;

declare function canvas_native_context_close_path(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_create(width: number, height: number, density: number, alpha: boolean, font_color: number, ppi: number, direction: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_conic_gradient(context: interop.Pointer | interop.Reference<any>, start_angle: number, x: number, y: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_gl(view: interop.Pointer | interop.Reference<any>, width: number, height: number, density: number, alpha: boolean, font_color: number, ppi: number, direction: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_gl_no_window(width: number, height: number, density: number, font_color: number, ppi: number, direction: number, alpha: boolean): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_image_data(width: number, height: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_image_data_with_data(width: number, height: number, data: string | interop.Pointer | interop.Reference<any>, size: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_linear_gradient(context: interop.Pointer | interop.Reference<any>, x0: number, y0: number, x1: number, y1: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_pattern(context: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, repetition: CanvasRepetition): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_pattern_asset(context: interop.Pointer | interop.Reference<any>, asset: interop.Pointer | interop.Reference<any>, repetition: CanvasRepetition): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_pattern_canvas2d(source: interop.Pointer | interop.Reference<any>, context: interop.Pointer | interop.Reference<any>, repetition: CanvasRepetition): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_pattern_encoded(context: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number, repetition: CanvasRepetition): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_pattern_webgl(source: interop.Pointer | interop.Reference<any>, context: interop.Pointer | interop.Reference<any>, repetition: CanvasRepetition): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_radial_gradient(context: interop.Pointer | interop.Reference<any>, x0: number, y0: number, r0: number, x1: number, y1: number, r1: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_create_with_pointer(pointer: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_draw_atlas(context: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, xform: interop.Pointer | interop.Reference<number>, xform_size: number, tex: interop.Pointer | interop.Reference<number>, tex_size: number, colors: interop.Pointer | interop.Reference<interop.Pointer | interop.Reference<any>>, colors_size: number, blend_mode: number): void;

declare function canvas_native_context_draw_atlas_asset(context: interop.Pointer | interop.Reference<any>, asset: interop.Pointer | interop.Reference<any>, xform: interop.Pointer | interop.Reference<number>, xform_size: number, tex: interop.Pointer | interop.Reference<number>, tex_size: number, colors: interop.Pointer | interop.Reference<interop.Pointer | interop.Reference<any>>, colors_size: number, blend_mode: number): void;

declare function canvas_native_context_draw_atlas_encoded(context: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number, xform: interop.Pointer | interop.Reference<number>, xform_size: number, tex: interop.Pointer | interop.Reference<number>, tex_size: number, colors: interop.Pointer | interop.Reference<interop.Pointer | interop.Reference<any>>, colors_size: number, blend_mode: number): void;

declare function canvas_native_context_draw_image(context: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, sx: number, sy: number, s_width: number, s_height: number, dx: number, dy: number, d_width: number, d_height: number): void;

declare function canvas_native_context_draw_image_asset(context: interop.Pointer | interop.Reference<any>, asset: interop.Pointer | interop.Reference<any>, sx: number, sy: number, s_width: number, s_height: number, dx: number, dy: number, d_width: number, d_height: number): void;

declare function canvas_native_context_draw_image_context(context: interop.Pointer | interop.Reference<any>, source: interop.Pointer | interop.Reference<any>, sx: number, sy: number, s_width: number, s_height: number, dx: number, dy: number, d_width: number, d_height: number): void;

declare function canvas_native_context_draw_image_dx_dy(context: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, dx: number, dy: number): void;

declare function canvas_native_context_draw_image_dx_dy_asset(context: interop.Pointer | interop.Reference<any>, asset: interop.Pointer | interop.Reference<any>, dx: number, dy: number): void;

declare function canvas_native_context_draw_image_dx_dy_context(context: interop.Pointer | interop.Reference<any>, source: interop.Pointer | interop.Reference<any>, dx: number, dy: number): void;

declare function canvas_native_context_draw_image_dx_dy_dw_dh(context: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, dx: number, dy: number, d_width: number, d_height: number): void;

declare function canvas_native_context_draw_image_dx_dy_dw_dh_asset(context: interop.Pointer | interop.Reference<any>, asset: interop.Pointer | interop.Reference<any>, dx: number, dy: number, d_width: number, d_height: number): void;

declare function canvas_native_context_draw_image_dx_dy_dw_dh_context(context: interop.Pointer | interop.Reference<any>, source: interop.Pointer | interop.Reference<any>, dx: number, dy: number, d_width: number, d_height: number): void;

declare function canvas_native_context_draw_image_dx_dy_dw_dh_webgl(context: interop.Pointer | interop.Reference<any>, source: interop.Pointer | interop.Reference<any>, dx: number, dy: number, d_width: number, d_height: number): void;

declare function canvas_native_context_draw_image_dx_dy_webgl(context: interop.Pointer | interop.Reference<any>, source: interop.Pointer | interop.Reference<any>, dx: number, dy: number): void;

declare function canvas_native_context_draw_image_encoded(context: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number, sx: number, sy: number, s_width: number, s_height: number, dx: number, dy: number, d_width: number, d_height: number): void;

declare function canvas_native_context_draw_image_encoded_dx_dy(context: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number, dx: number, dy: number): void;

declare function canvas_native_context_draw_image_encoded_dx_dy_dw_dh(context: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number, dx: number, dy: number, d_width: number, d_height: number): void;

declare function canvas_native_context_draw_image_webgl(context: interop.Pointer | interop.Reference<any>, source: interop.Pointer | interop.Reference<any>, sx: number, sy: number, s_width: number, s_height: number, dx: number, dy: number, d_width: number, d_height: number): void;

declare function canvas_native_context_draw_paint(context: interop.Pointer | interop.Reference<any>, color: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_draw_point(context: interop.Pointer | interop.Reference<any>, x: number, y: number): void;

declare function canvas_native_context_draw_points(context: interop.Pointer | interop.Reference<any>, mode: number, points: interop.Pointer | interop.Reference<number>, size: number): void;

declare function canvas_native_context_ellipse(context: interop.Pointer | interop.Reference<any>, x: number, y: number, radius_x: number, radius_y: number, rotation: number, start_angle: number, end_angle: number, anticlockwise: boolean): void;

declare function canvas_native_context_fill(context: interop.Pointer | interop.Reference<any>, rule: CanvasFillRule): void;

declare function canvas_native_context_fill_oval(context: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number): void;

declare function canvas_native_context_fill_rect(context: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number): void;

declare function canvas_native_context_fill_text(context: interop.Pointer | interop.Reference<any>, text: string | interop.Pointer | interop.Reference<any>, x: number, y: number): void;

declare function canvas_native_context_fill_text_width(context: interop.Pointer | interop.Reference<any>, text: string | interop.Pointer | interop.Reference<any>, x: number, y: number, width: number): void;

declare function canvas_native_context_fill_with_path(context: interop.Pointer | interop.Reference<any>, path: interop.Pointer | interop.Reference<any>, rule: CanvasFillRule): void;

declare function canvas_native_context_flush(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_get_current_fill_style_type(context: interop.Pointer | interop.Reference<any>): PaintStyleType;

declare function canvas_native_context_get_current_stroke_style_type(context: interop.Pointer | interop.Reference<any>): PaintStyleType;

declare function canvas_native_context_get_fill_style(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_filter(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_font(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_global_alpha(context: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_context_get_global_composition(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_image_data(context: interop.Pointer | interop.Reference<any>, sx: number, sy: number, sw: number, sh: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_image_smoothing_enabled(context: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_context_get_image_smoothing_quality(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_letter_spacing(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_line_cap(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_line_dash(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_line_dash_offset(context: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_context_get_line_join(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_line_width(context: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_context_get_miter_limit(context: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_context_get_shadow_blur(context: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_context_get_shadow_color(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_shadow_color_buf(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_shadow_color_rgba(context: interop.Pointer | interop.Reference<any>, r: string | interop.Pointer | interop.Reference<any>, g: string | interop.Pointer | interop.Reference<any>, b: string | interop.Pointer | interop.Reference<any>, a: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_get_shadow_offset_x(context: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_context_get_shadow_offset_y(context: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_context_get_stroke_style(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_style_type(style: interop.Pointer | interop.Reference<any>): PaintStyleType;

declare function canvas_native_context_get_text_align(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_text_baseline(context: interop.Pointer | interop.Reference<any>): TextBaseLine;

declare function canvas_native_context_get_text_baseline_str(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_transform(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_get_word_spacing(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_is_point_in_path(context: interop.Pointer | interop.Reference<any>, x: number, y: number, rule: CanvasFillRule): boolean;

declare function canvas_native_context_is_point_in_path_str(context: interop.Pointer | interop.Reference<any>, x: number, y: number, rule: CanvasFillRule): boolean;

declare function canvas_native_context_is_point_in_path_with_path(context: interop.Pointer | interop.Reference<any>, path: interop.Pointer | interop.Reference<any>, x: number, y: number, rule: CanvasFillRule): boolean;

declare function canvas_native_context_is_point_in_path_with_path_str(context: interop.Pointer | interop.Reference<any>, path: interop.Pointer | interop.Reference<any>, x: number, y: number, rule: CanvasFillRule): boolean;

declare function canvas_native_context_is_point_in_stroke(context: interop.Pointer | interop.Reference<any>, x: number, y: number): boolean;

declare function canvas_native_context_is_point_in_stroke_with_path(context: interop.Pointer | interop.Reference<any>, path: interop.Pointer | interop.Reference<any>, x: number, y: number): boolean;

declare function canvas_native_context_line_to(context: interop.Pointer | interop.Reference<any>, x: number, y: number): void;

declare function canvas_native_context_measure_text(context: interop.Pointer | interop.Reference<any>, text: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_context_move_to(context: interop.Pointer | interop.Reference<any>, x: number, y: number): void;

declare function canvas_native_context_put_image_data(context: interop.Pointer | interop.Reference<any>, image_data: interop.Pointer | interop.Reference<any>, dx: number, dy: number, dirty_x: number, dirty_y: number, dirty_width: number, dirty_height: number): void;

declare function canvas_native_context_quadratic_curve_to(context: interop.Pointer | interop.Reference<any>, cpx: number, cpy: number, x: number, y: number): void;

declare function canvas_native_context_rect(context: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number): void;

declare function canvas_native_context_release(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_render(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_reset_transform(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_resize(context: interop.Pointer | interop.Reference<any>, width: number, height: number): void;

declare function canvas_native_context_restore(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_rotate(context: interop.Pointer | interop.Reference<any>, angle: number): void;

declare function canvas_native_context_round_rect(context: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number, radii: interop.Pointer | interop.Reference<number>, size: number): void;

declare function canvas_native_context_round_rect_tl_tr_br_bl(context: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number, top_left: number, top_right: number, bottom_right: number, bottom_left: number): void;

declare function canvas_native_context_save(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_scale(context: interop.Pointer | interop.Reference<any>, x: number, y: number): void;

declare function canvas_native_context_set_fill_style(context: interop.Pointer | interop.Reference<any>, style: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_filter(context: interop.Pointer | interop.Reference<any>, filter: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_font(context: interop.Pointer | interop.Reference<any>, font: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_global_alpha(context: interop.Pointer | interop.Reference<any>, alpha: number): void;

declare function canvas_native_context_set_global_composition(context: interop.Pointer | interop.Reference<any>, composition: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_image_smoothing_enabled(context: interop.Pointer | interop.Reference<any>, enabled: boolean): void;

declare function canvas_native_context_set_image_smoothing_quality(context: interop.Pointer | interop.Reference<any>, quality: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_letter_spacing(context: interop.Pointer | interop.Reference<any>, spacing: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_line_cap(context: interop.Pointer | interop.Reference<any>, cap: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_line_dash(context: interop.Pointer | interop.Reference<any>, dash: interop.Pointer | interop.Reference<number>, size: number): void;

declare function canvas_native_context_set_line_dash_offset(context: interop.Pointer | interop.Reference<any>, offset: number): void;

declare function canvas_native_context_set_line_join(context: interop.Pointer | interop.Reference<any>, join: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_line_width(context: interop.Pointer | interop.Reference<any>, width: number): void;

declare function canvas_native_context_set_miter_limit(context: interop.Pointer | interop.Reference<any>, limit: number): void;

declare function canvas_native_context_set_shadow_blur(context: interop.Pointer | interop.Reference<any>, blur: number): void;

declare function canvas_native_context_set_shadow_color(context: interop.Pointer | interop.Reference<any>, color: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_shadow_color_rgba(context: interop.Pointer | interop.Reference<any>, r: number, g: number, b: number, a: number): void;

declare function canvas_native_context_set_shadow_offset_x(context: interop.Pointer | interop.Reference<any>, x: number): void;

declare function canvas_native_context_set_shadow_offset_y(context: interop.Pointer | interop.Reference<any>, y: number): void;

declare function canvas_native_context_set_stroke_style(context: interop.Pointer | interop.Reference<any>, style: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_text_align(context: interop.Pointer | interop.Reference<any>, alignment: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_text_baseline(context: interop.Pointer | interop.Reference<any>, baseline: TextBaseLine): void;

declare function canvas_native_context_set_text_baseline_str(context: interop.Pointer | interop.Reference<any>, baseline: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_transform(context: interop.Pointer | interop.Reference<any>, a: number, b: number, c: number, d: number, e: number, f: number): void;

declare function canvas_native_context_set_transform_matrix(context: interop.Pointer | interop.Reference<any>, matrix: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_set_word_spacing(context: interop.Pointer | interop.Reference<any>, spacing: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_stroke(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_stroke_oval(context: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number): void;

declare function canvas_native_context_stroke_rect(context: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number): void;

declare function canvas_native_context_stroke_text(context: interop.Pointer | interop.Reference<any>, text: string | interop.Pointer | interop.Reference<any>, x: number, y: number): void;

declare function canvas_native_context_stroke_text_width(context: interop.Pointer | interop.Reference<any>, text: string | interop.Pointer | interop.Reference<any>, x: number, y: number, width: number): void;

declare function canvas_native_context_stroke_with_path(context: interop.Pointer | interop.Reference<any>, path: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_context_transform(context: interop.Pointer | interop.Reference<any>, a: number, b: number, c: number, d: number, e: number, f: number): void;

declare function canvas_native_context_translate(context: interop.Pointer | interop.Reference<any>, x: number, y: number): void;

declare function canvas_native_f32_buffer_get_bytes(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function canvas_native_f32_buffer_get_bytes_mut(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function canvas_native_f32_buffer_get_length(buffer: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_f32_buffer_release(buffer: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_font_add_family(alias: string | interop.Pointer | interop.Reference<any>, filenames: interop.Pointer | interop.Reference<interop.Pointer | interop.Reference<any>>, length: number): void;

declare function canvas_native_font_add_family_with_bytes(alias: string | interop.Pointer | interop.Reference<any>, bytes: string | interop.Pointer | interop.Reference<any>, length: number): void;

declare function canvas_native_font_clear(): void;

declare function canvas_native_gradient_add_color_stop(style: interop.Pointer | interop.Reference<any>, stop: number, color: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_helper_get_mime(data: string | interop.Pointer | interop.Reference<any>, size: number): interop.Pointer | interop.Reference<FileHelperMime>;

declare function canvas_native_helper_read_file(path: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_helper_read_file_get_data(file: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_helper_read_file_get_error(file: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_helper_read_file_get_extension(file: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_helper_read_file_get_mime(file: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_helper_read_file_has_error(file: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_helper_read_file_take_data(file: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_helper_release(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_helper_release_mime(mime: interop.Pointer | interop.Reference<FileHelperMime>): void;

declare function canvas_native_i32_buffer_get_bytes(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function canvas_native_i32_buffer_get_bytes_mut(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function canvas_native_i32_buffer_get_length(buffer: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_i32_buffer_release(buffer: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_image_asset_addr(asset: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_image_asset_close(asset: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_image_asset_create(): interop.Pointer | interop.Reference<any>;

declare function canvas_native_image_asset_get_addr(asset: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_image_asset_get_error(asset: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_image_asset_has_error(asset: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_image_asset_height(asset: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_image_asset_load_from_fd(asset: interop.Pointer | interop.Reference<any>, fd: number): boolean;

declare function canvas_native_image_asset_load_from_path(asset: interop.Pointer | interop.Reference<any>, path: string | interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_image_asset_load_from_raw(asset: interop.Pointer | interop.Reference<any>, width: number, height: number, array: string | interop.Pointer | interop.Reference<any>, size: number): boolean;

declare function canvas_native_image_asset_load_from_raw_encoded(asset: interop.Pointer | interop.Reference<any>, array: string | interop.Pointer | interop.Reference<any>, size: number): boolean;

declare function canvas_native_image_asset_load_from_url(asset: interop.Pointer | interop.Reference<any>, url: string | interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_image_asset_reference(asset: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_image_asset_release(asset: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_image_asset_width(asset: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_image_bitmap_create_from_asset(asset: interop.Pointer | interop.Reference<any>, flip_y: boolean, premultiply_alpha: ImageBitmapPremultiplyAlpha, color_space_conversion: ImageBitmapColorSpaceConversion, resize_quality: ImageBitmapResizeQuality, resize_width: number, resize_height: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_image_bitmap_create_from_asset_src_rect(asset: interop.Pointer | interop.Reference<any>, sx: number, sy: number, s_width: number, s_height: number, flip_y: boolean, premultiply_alpha: ImageBitmapPremultiplyAlpha, color_space_conversion: ImageBitmapColorSpaceConversion, resize_quality: ImageBitmapResizeQuality, resize_width: number, resize_height: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_image_bitmap_create_from_encoded_bytes(bytes: string | interop.Pointer | interop.Reference<any>, size: number, flip_y: boolean, premultiply_alpha: ImageBitmapPremultiplyAlpha, color_space_conversion: ImageBitmapColorSpaceConversion, resize_quality: ImageBitmapResizeQuality, resize_width: number, resize_height: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_image_bitmap_create_from_encoded_bytes_src_rect(bytes: string | interop.Pointer | interop.Reference<any>, size: number, sx: number, sy: number, s_width: number, s_height: number, flip_y: boolean, premultiply_alpha: ImageBitmapPremultiplyAlpha, color_space_conversion: ImageBitmapColorSpaceConversion, resize_quality: ImageBitmapResizeQuality, resize_width: number, resize_height: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(bytes: string | interop.Pointer | interop.Reference<any>, size: number, sx: number, sy: number, s_width: number, s_height: number, flip_y: boolean, premultiply_alpha: ImageBitmapPremultiplyAlpha, color_space_conversion: ImageBitmapColorSpaceConversion, resize_quality: ImageBitmapResizeQuality, resize_width: number, resize_height: number, output: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_image_bitmap_create_from_encoded_bytes_with_output(bytes: string | interop.Pointer | interop.Reference<any>, size: number, flip_y: boolean, premultiply_alpha: ImageBitmapPremultiplyAlpha, color_space_conversion: ImageBitmapColorSpaceConversion, resize_quality: ImageBitmapResizeQuality, resize_width: number, resize_height: number, output: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_image_data_create(width: number, height: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_image_data_get_data(image_data: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_image_data_get_height(image_data: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_image_data_get_length(image_data: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_image_data_get_width(image_data: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_image_data_release(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_image_filter_reference(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_image_filter_release(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_init_ios_webgpu(instance: number, view: interop.Pointer | interop.Reference<any>, width: number, height: number): number;

declare function canvas_native_ios_context_create_pattern_raw(context: number, width: number, height: number, bytes: string | interop.Pointer | interop.Reference<any>, size: number, repetition: string | interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_ios_context_custom_with_buffer_flush(context: number, bytes: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, alpha: boolean): void;

declare function canvas_native_ios_context_draw_image_dx_dy_dw_dh_with_bytes(context: number, bytes: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, dx: number, dy: number, d_width: number, d_height: number): boolean;

declare function canvas_native_ios_context_draw_image_dx_dy_with_bytes(context: number, bytes: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, dx: number, dy: number): boolean;

declare function canvas_native_ios_context_draw_image_with_bytes(context: number, bytes: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, sx: number, sy: number, s_width: number, s_height: number, dx: number, dy: number, d_width: number, d_height: number): boolean;

declare function canvas_native_ios_context_init_context_with_custom_surface(width: number, height: number, density: number, alpha: boolean, font_color: number, ppi: number, direction: number): number;

declare function canvas_native_ios_create_2d_context(view: interop.Pointer | interop.Reference<any>, width: number, height: number, alpha: boolean, density: number, font_color: number, ppi: number, direction: number): number;

declare function canvas_native_ios_create_2d_context_metal(view: interop.Pointer | interop.Reference<any>, alpha: boolean, density: number, samples: number, font_color: number, ppi: number, direction: number): number;

declare function canvas_native_ios_create_2d_context_metal_device_queue(view: interop.Pointer | interop.Reference<any>, device: interop.Pointer | interop.Reference<any>, queue: interop.Pointer | interop.Reference<any>, alpha: boolean, density: number, samples: number, font_color: number, ppi: number, direction: number): number;

declare function canvas_native_ios_create_webgl_context(view: interop.Pointer | interop.Reference<any>, alpha: boolean, antialias: boolean, depth: boolean, fail_if_major_performance_caveat: boolean, power_preference: number, premultiplied_alpha: boolean, preserve_drawing_buffer: boolean, stencil: boolean, desynchronized: boolean, xr_compatible: boolean, version: number): number;

declare function canvas_native_ios_flush_2d_context(context: number): void;

declare function canvas_native_ios_flush_2d_context_and_sync_cpu(context: number): void;

declare function canvas_native_ios_flush_webgl(context: number): boolean;

declare function canvas_native_ios_gl_make_current(context: number): void;

declare function canvas_native_ios_image_asset_load_from_bytes(asset: number, bytes: string | interop.Pointer | interop.Reference<any>, size: number): boolean;

declare function canvas_native_ios_present_drawable(context: number): void;

declare function canvas_native_ios_release_webgl(context: number): void;

declare function canvas_native_ios_resize_context_2d(context: number, width: number, height: number): void;

declare function canvas_native_ios_update_webgl_surface(view: number, _width: number, _height: number, context: number): void;

declare function canvas_native_ios_webgl_tex_image_2d(context: number, target: number, level: number, internalformat: number, format: number, type_: number, bytes: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, flip_y: boolean): void;

declare function canvas_native_ios_webgl_tex_sub_image_2d(context: number, target: number, level: number, xoffset: number, yoffset: number, format: number, type_: number, bytes: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, flip_y: boolean): void;

declare function canvas_native_matrix_clone(matrix: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_matrix_create(): interop.Pointer | interop.Reference<any>;

declare function canvas_native_matrix_get_a(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_b(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_c(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_d(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_e(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_f(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m11(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m12(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m13(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m14(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m21(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m22(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m23(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m24(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m31(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m32(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m33(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m34(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m41(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m42(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m43(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_get_m44(matrix: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_matrix_multiply_self(matrix: interop.Pointer | interop.Reference<any>, value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_matrix_premultiply_self(matrix: interop.Pointer | interop.Reference<any>, value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_matrix_release(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_matrix_rotate(angle: number, cx: number, cy: number, matrix: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_matrix_rotate_self(matrix: interop.Pointer | interop.Reference<any>, angle: number, cx: number, cy: number): void;

declare function canvas_native_matrix_scale_non_uniform(sx: number, sy: number, matrix: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_matrix_scale_non_uniform_self(matrix: interop.Pointer | interop.Reference<any>, sx: number, sy: number): void;

declare function canvas_native_matrix_set_a(matrix: interop.Pointer | interop.Reference<any>, a: number): void;

declare function canvas_native_matrix_set_b(matrix: interop.Pointer | interop.Reference<any>, b: number): void;

declare function canvas_native_matrix_set_c(matrix: interop.Pointer | interop.Reference<any>, c: number): void;

declare function canvas_native_matrix_set_d(matrix: interop.Pointer | interop.Reference<any>, d: number): void;

declare function canvas_native_matrix_set_e(matrix: interop.Pointer | interop.Reference<any>, e: number): void;

declare function canvas_native_matrix_set_f(matrix: interop.Pointer | interop.Reference<any>, f: number): void;

declare function canvas_native_matrix_set_m11(matrix: interop.Pointer | interop.Reference<any>, m11: number): void;

declare function canvas_native_matrix_set_m12(matrix: interop.Pointer | interop.Reference<any>, m12: number): void;

declare function canvas_native_matrix_set_m13(matrix: interop.Pointer | interop.Reference<any>, m13: number): void;

declare function canvas_native_matrix_set_m14(matrix: interop.Pointer | interop.Reference<any>, m14: number): void;

declare function canvas_native_matrix_set_m21(matrix: interop.Pointer | interop.Reference<any>, m21: number): void;

declare function canvas_native_matrix_set_m22(matrix: interop.Pointer | interop.Reference<any>, m22: number): void;

declare function canvas_native_matrix_set_m23(matrix: interop.Pointer | interop.Reference<any>, m23: number): void;

declare function canvas_native_matrix_set_m24(matrix: interop.Pointer | interop.Reference<any>, m24: number): void;

declare function canvas_native_matrix_set_m31(matrix: interop.Pointer | interop.Reference<any>, m31: number): void;

declare function canvas_native_matrix_set_m32(matrix: interop.Pointer | interop.Reference<any>, m32: number): void;

declare function canvas_native_matrix_set_m33(matrix: interop.Pointer | interop.Reference<any>, m33: number): void;

declare function canvas_native_matrix_set_m34(matrix: interop.Pointer | interop.Reference<any>, m34: number): void;

declare function canvas_native_matrix_set_m41(matrix: interop.Pointer | interop.Reference<any>, m41: number): void;

declare function canvas_native_matrix_set_m42(matrix: interop.Pointer | interop.Reference<any>, m42: number): void;

declare function canvas_native_matrix_set_m43(matrix: interop.Pointer | interop.Reference<any>, m43: number): void;

declare function canvas_native_matrix_set_m44(matrix: interop.Pointer | interop.Reference<any>, m44: number): void;

declare function canvas_native_matrix_skew_x(angle: number, matrix: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_matrix_skew_x_self(matrix: interop.Pointer | interop.Reference<any>, angle: number): void;

declare function canvas_native_matrix_skew_y(angle: number, matrix: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_matrix_skew_y_self(matrix: interop.Pointer | interop.Reference<any>, angle: number): void;

declare function canvas_native_matrix_translate(x: number, y: number, matrix: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_matrix_translate_self(matrix: interop.Pointer | interop.Reference<any>, x: number, y: number): void;

declare function canvas_native_matrix_update(matrix: interop.Pointer | interop.Reference<any>, slice: interop.Pointer | interop.Reference<number>, size: number): void;

declare function canvas_native_matrix_update_3d(matrix: interop.Pointer | interop.Reference<any>, slice: interop.Pointer | interop.Reference<number>, size: number): void;

declare function canvas_native_paint_style_from_bytes(context: interop.Pointer | interop.Reference<any>, repetition: number, width: number, height: number, bytes: string | interop.Pointer | interop.Reference<any>, size: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_paint_style_get_color_string(color: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_paint_style_get_current_fill_color_buf(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_paint_style_get_current_fill_color_r_g_b_a(context: interop.Pointer | interop.Reference<any>, r: string | interop.Pointer | interop.Reference<any>, g: string | interop.Pointer | interop.Reference<any>, b: string | interop.Pointer | interop.Reference<any>, a: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_paint_style_get_current_fill_color_string(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_paint_style_get_current_stroke_color_buf(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_paint_style_get_current_stroke_color_r_g_b_a(context: interop.Pointer | interop.Reference<any>, r: string | interop.Pointer | interop.Reference<any>, g: string | interop.Pointer | interop.Reference<any>, b: string | interop.Pointer | interop.Reference<any>, a: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_paint_style_get_current_stroke_color_string(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_paint_style_release(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_paint_style_set_fill_color_with_c_string(context: interop.Pointer | interop.Reference<any>, color: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_paint_style_set_fill_color_with_rgba(context: interop.Pointer | interop.Reference<any>, r: number, g: number, b: number, a: number): void;

declare function canvas_native_paint_style_set_stroke_color_with_c_string(context: interop.Pointer | interop.Reference<any>, color: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_paint_style_set_stroke_color_with_rgba(context: interop.Pointer | interop.Reference<any>, r: number, g: number, b: number, a: number): void;

declare function canvas_native_parse_css_color_rgba(value: string | interop.Pointer | interop.Reference<any>, r: string | interop.Pointer | interop.Reference<any>, g: string | interop.Pointer | interop.Reference<any>, b: string | interop.Pointer | interop.Reference<any>, a: string | interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_path_add_path(path: interop.Pointer | interop.Reference<any>, path_to_add: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_path_arc(path: interop.Pointer | interop.Reference<any>, x: number, y: number, radius: number, start_angle: number, end_angle: number, anti_clockwise: boolean): void;

declare function canvas_native_path_arc_to(path: interop.Pointer | interop.Reference<any>, x1: number, y1: number, x2: number, y2: number, radius: number): void;

declare function canvas_native_path_bezier_curve_to(path: interop.Pointer | interop.Reference<any>, cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void;

declare function canvas_native_path_close_path(path: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_path_create(): interop.Pointer | interop.Reference<any>;

declare function canvas_native_path_create_with_path(path: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_path_create_with_string(string: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_path_ellipse(path: interop.Pointer | interop.Reference<any>, x: number, y: number, radius_x: number, radius_y: number, rotation: number, start_angle: number, end_angle: number, anticlockwise: boolean): void;

declare function canvas_native_path_line_to(path: interop.Pointer | interop.Reference<any>, x: number, y: number): void;

declare function canvas_native_path_move_to(path: interop.Pointer | interop.Reference<any>, x: number, y: number): void;

declare function canvas_native_path_quadratic_curve_to(path: interop.Pointer | interop.Reference<any>, cpx: number, cpy: number, x: number, y: number): void;

declare function canvas_native_path_rect(path: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number): void;

declare function canvas_native_path_release(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_path_round_rect(path: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number, radii: interop.Pointer | interop.Reference<number>, size: number): void;

declare function canvas_native_path_round_rect_tl_tr_br_bl(path: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number, top_left: number, top_right: number, bottom_right: number, bottom_left: number): void;

declare function canvas_native_path_to_string(path: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_path_trim(path: interop.Pointer | interop.Reference<any>, start: number, end: number): void;

declare function canvas_native_pattern_from_ptr(ptr: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_pattern_set_transform(pattern: interop.Pointer | interop.Reference<any>, matrix: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_raf_create(callback: number, on_frame_callback: interop.FunctionReference<(p1: number, p2: number) => void>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_raf_get_started(raf: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_raf_release(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_raf_start(raf: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_raf_stop(raf: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_resize_ios_webgpu_uiview(context: number, view: interop.Pointer | interop.Reference<any>, width: number, height: number): void;

declare function canvas_native_string_buffer_get_length(buffer: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_string_buffer_get_value_at(buffer: interop.Pointer | interop.Reference<any>, index: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_string_buffer_release(buffer: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_string_destroy(value: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_text_decoder_create(decoding: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_text_decoder_decode(decoder: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_text_decoder_decode_as_bytes(decoder: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_text_decoder_decode_as_cow(decoder: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_text_decoder_decode_c_string(decoder: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_text_decoder_get_encoding(decoder: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_text_decoder_release(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_text_encoder_create(encoding: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_text_encoder_encode(encoder: interop.Pointer | interop.Reference<any>, text: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_text_encoder_get_encoding(encoder: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_text_encoder_release(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_text_metrics_get_actual_bounding_box_ascent(metrics: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_text_metrics_get_actual_bounding_box_descent(metrics: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_text_metrics_get_actual_bounding_box_left(metrics: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_text_metrics_get_actual_bounding_box_right(metrics: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_text_metrics_get_alphabetic_baseline(metrics: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_text_metrics_get_em_height_ascent(metrics: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_text_metrics_get_em_height_descent(metrics: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_text_metrics_get_font_bounding_box_ascent(metrics: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_text_metrics_get_font_bounding_box_descent(metrics: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_text_metrics_get_hanging_baseline(metrics: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_text_metrics_get_ideographic_baseline(metrics: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_text_metrics_get_width(metrics: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_text_metrics_release(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_to_data_url(context: interop.Pointer | interop.Reference<any>, format: string | interop.Pointer | interop.Reference<any>, quality: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_u16_buffer_get_bytes(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function canvas_native_u16_buffer_get_bytes_mut(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function canvas_native_u16_buffer_get_length(buffer: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_u16_buffer_release(buffer: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_u32_buffer_get_bytes(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function canvas_native_u32_buffer_get_bytes_mut(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function canvas_native_u32_buffer_get_length(buffer: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_u32_buffer_release(buffer: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_u8_buffer_get_bytes(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_u8_buffer_get_bytes_mut(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_u8_buffer_get_length(buffer: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_u8_buffer_release(buffer: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_begin_query(target: number, id: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_begin_transform_feedback(primitive_mode: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_bind_buffer_base(target: number, index: number, buffer: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_bind_buffer_range(target: number, index: number, buffer: number, offset: number, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_bind_sampler(unit: number, sampler: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_bind_transform_feedback(target: number, transform_feedback: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_bind_vertex_array(vertex_array: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_blit_framebuffer(src_x0: number, src_y0: number, src_x1: number, src_y1: number, dst_x0: number, dst_y0: number, dst_x1: number, dst_y1: number, mask: number, filter: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_clear_bufferfi(buffer: number, drawbuffer: number, depth: number, stencil: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_clear_bufferfv(buffer: number, drawbuffer: number, values: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_clear_bufferiv(buffer: number, drawbuffer: number, values: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_clear_bufferuiv(buffer: number, drawbuffer: number, values: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_client_wait_sync(sync: interop.Pointer | interop.Reference<any>, flags: number, timeout: number, state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl2_compressed_tex_sub_image3d(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, src: string | interop.Pointer | interop.Reference<any>, size: number, src_offset: number, src_length_override: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_compressed_tex_sub_image3d_none(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, image_size: number, offset: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_copy_buffer_sub_data(read_target: number, write_target: number, read_offset: number, write_offset: number, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_copy_tex_sub_image3d(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, x: number, y: number, width: number, height: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_create_query(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl2_create_sampler(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl2_create_transform_feedback(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl2_create_vertex_array(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl2_delete_query_with_query(id: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_delete_sampler_with_sampler(sampler: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_delete_sync_with_sync(sync: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_delete_transform_feedback(transform_feedback: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_delete_vertex_array_with_vertex_array(vertex_array: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_draw_arrays_instanced(mode: number, first: number, count: number, instance_count: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_draw_buffers(buffers: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_draw_elements_instanced(mode: number, count: number, type_: number, offset: number, instance_count: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_draw_range_elements(mode: number, start: number, end: number, count: number, type_: number, offset: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_end_query(target: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_end_transform_feedback(state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_fence_sync(condition: number, flags: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_framebuffer_texture_layer(target: number, attachment: number, texture: number, level: number, layer: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_get_active_uniform_block_name(program: number, uniform_block_index: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_get_active_uniform_block_parameter(program: number, uniform_block_index: number, pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_get_active_uniforms(program: number, uniform_indices: interop.Pointer | interop.Reference<number>, size: number, pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_get_buffer_sub_data(target: number, src_byte_offset: number, dst_data: string | interop.Pointer | interop.Reference<any>, size: number, dst_offset: number, length: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_get_frag_data_location(program: number, name: string | interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl2_get_indexed_parameter(target: number, index: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_get_internalformat_parameter(target: number, internalformat: number, pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_get_parameter(pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_get_query(target: number, pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_get_query_parameter(query: number, pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_get_sampler_parameter(sampler: number, pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_get_sync_parameter(sync: interop.Pointer | interop.Reference<any>, pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_get_transform_feedback_varying(program: number, index: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_get_uniform_block_index(program: number, uniform_block_name: string | interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl2_get_uniform_indices(program: number, uniform_names: interop.Pointer | interop.Reference<interop.Pointer | interop.Reference<any>>, size: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl2_indexed_parameter_get_buffer_value(param: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl2_indexed_parameter_get_is_buffer(param: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl2_indexed_parameter_get_value(param: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl2_invalidate_framebuffer(target: number, attachments: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_invalidate_sub_framebuffer(target: number, attachments: interop.Pointer | interop.Reference<number>, size: number, x: number, y: number, width: number, height: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_is_query(query: number, state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl2_is_sampler(sampler: number, state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl2_is_sync(sync: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl2_is_transform_feedback(transform_feedback: number, state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl2_is_vertex_array(vertex_array: number, state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl2_pause_transform_feedback(state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_read_buffer(src: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_renderbuffer_storage_multisample(target: number, samples: number, internal_format: number, width: number, height: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_resume_transform_feedback(state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_sampler_parameterf(sampler: number, pname: number, param: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_sampler_parameteri(sampler: number, pname: number, param: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_image2d_canvas2d(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type_: number, canvas: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_image2d_image_asset(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type_: number, image_asset: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_image2d_image_data(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type_: number, image_data: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_image2d_offset(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type_: number, offset: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_image2d_src_data_offset(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type_: number, src_data: string | interop.Pointer | interop.Reference<any>, src_data_size: number, offset: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_image2d_webgl(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type_: number, webgl: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_image3d(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type_: number, buf: string | interop.Pointer | interop.Reference<any>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_image3d_asset(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type_: number, asset: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_image3d_canvas2d(target: number, level: number, internalformat: number, _width: number, _height: number, depth: number, border: number, format: number, type_: number, canvas: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_image3d_none(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type_: number, offset: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_image3d_offset(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type_: number, buf: string | interop.Pointer | interop.Reference<any>, size: number, offset: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_storage2d(target: number, levels: number, internalformat: number, width: number, height: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_storage3d(target: number, levels: number, internalformat: number, width: number, height: number, depth: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_sub_image3d(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type_: number, buf: string | interop.Pointer | interop.Reference<any>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_sub_image3d_asset(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type_: number, asset: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_sub_image3d_canvas2d(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, _width: number, _height: number, depth: number, format: number, type_: number, canvas: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_sub_image3d_none(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type_: number, offset: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_tex_sub_image3d_offset(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type_: number, buf: string | interop.Pointer | interop.Reference<any>, size: number, offset: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_transform_feedback_varyings(program: number, varyings: interop.Pointer | interop.Reference<interop.Pointer | interop.Reference<any>>, size: number, buffer_mode: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform1ui(location: number, v0: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform1uiv(location: number, data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform2ui(location: number, v0: number, v1: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform2uiv(location: number, data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform3ui(location: number, v0: number, v1: number, v2: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform3uiv(location: number, data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform4ui(location: number, v0: number, v1: number, v2: number, v3: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform4uiv(location: number, data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform_block_binding(program: number, uniform_block_index: number, uniform_block_binding: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform_matrix2x3fv(location: number, transpose: boolean, data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform_matrix2x4fv(location: number, transpose: boolean, data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform_matrix3x2fv(location: number, transpose: boolean, data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform_matrix3x4fv(location: number, transpose: boolean, data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform_matrix4x2fv(location: number, transpose: boolean, data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_uniform_matrix4x3fv(location: number, transpose: boolean, data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_vertex_attrib_divisor(index: number, divisor: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_vertex_attrib_i4i(index: number, x: number, y: number, z: number, w: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_vertex_attrib_i4iv(index: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_vertex_attrib_i4ui(index: number, x: number, y: number, z: number, w: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl2_vertex_attrib_i4uiv(index: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_ANGLE_instanced_arrays_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_EXT_blend_minmax_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_EXT_color_buffer_half_float_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_EXT_disjoint_timer_query_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_EXT_sRGB_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_EXT_shader_texture_lod_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_EXT_texture_filter_anisotropic_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_OES_element_index_uint_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_OES_standard_derivatives_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_OES_texture_float_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_OES_texture_float_linear_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_OES_texture_half_float_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_OES_texture_half_float_linear_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_OES_vertex_array_object_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_WEBGL_color_buffer_float_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_WEBGL_compressed_texture_atc_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_WEBGL_compressed_texture_etc1_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_WEBGL_compressed_texture_etc_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_WEBGL_compressed_texture_pvrtc_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_WEBGL_compressed_texture_s3tc_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_WEBGL_compressed_texture_s3tc_srgb_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_WEBGL_depth_texture_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_WEBGL_draw_buffers_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_WEBGL_lose_context_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_WebGLResult_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_active_info_destroy(info: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_active_info_get_is_empty(info: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_active_info_get_name(info: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_active_info_get_size(info: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_active_info_get_type(info: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_active_texture(texture: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(mode: number, first: number, count: number, primcount: number, arrays: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(mode: number, count: number, type_: number, offset: number, primcount: number, arrays: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(index: number, divisor: number, arrays: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_attach_shader(program: number, shader: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_bind_attrib_location(program: number, index: number, name: string | interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_bind_buffer(target: number, buffer: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_bind_frame_buffer(target: number, framebuffer: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_bind_render_buffer(target: number, renderbuffer: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_bind_texture(target: number, texture: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_blend_color(red: number, green: number, blue: number, alpha: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_blend_equation(mode: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_blend_equation_separate(mode_rgb: number, mode_alpha: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_blend_func(sfactor: number, dfactor: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_blend_func_separate(src_rgb: number, dst_rgb: number, src_alpha: number, dst_alpha: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_data(target: number, src_data: string | interop.Pointer | interop.Reference<any>, size: number, usage: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_data_f32(target: number, src_data: interop.Pointer | interop.Reference<number>, size: number, usage: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_data_f64(target: number, src_data: interop.Pointer | interop.Reference<number>, size: number, usage: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_data_i16(target: number, src_data: interop.Pointer | interop.Reference<number>, size: number, usage: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_data_i32(target: number, src_data: interop.Pointer | interop.Reference<number>, size: number, usage: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_data_i8(target: number, src_data: interop.Pointer | interop.Reference<number>, size: number, usage: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_data_none(target: number, size: number, usage: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_data_u16(target: number, src_data: interop.Pointer | interop.Reference<number>, size: number, usage: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_data_u32(target: number, src_data: interop.Pointer | interop.Reference<number>, size: number, usage: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_sub_data(target: number, offset: number, src_data: string | interop.Pointer | interop.Reference<any>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_sub_data_f32(target: number, offset: number, src_data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_sub_data_f64(target: number, offset: number, src_data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_sub_data_i16(target: number, offset: number, src_data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_sub_data_i32(target: number, offset: number, src_data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_sub_data_i8(target: number, offset: number, src_data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_sub_data_none(target: number, offset: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_sub_data_u16(target: number, offset: number, src_data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_buffer_sub_data_u32(target: number, offset: number, src_data: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_check_frame_buffer_status(target: number, state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_clear(mask: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_clear_color(red: number, green: number, blue: number, alpha: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_clear_depth(depth: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_clear_stencil(stencil: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_color_mask(red: boolean, green: boolean, blue: boolean, alpha: boolean, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_commit(p1: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_compile_shader(shader: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_compressed_tex_image2d(target: number, level: number, internalformat: number, width: number, height: number, border: number, pixels: string | interop.Pointer | interop.Reference<any>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_compressed_tex_image2d_none(target: number, level: number, internalformat: number, width: number, height: number, border: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_compressed_tex_sub_image2d(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, pixels: string | interop.Pointer | interop.Reference<any>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_context_attribute_get_get_alpha(attr: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_context_attribute_get_get_antialias(attr: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_context_attribute_get_get_depth(attr: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_context_attribute_get_get_desynchronized(attr: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(attr: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_context_attribute_get_get_power_preference(attr: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(attr: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(attr: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_context_attribute_get_get_stencil(attr: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_context_attribute_get_get_xr_compatible(attr: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_context_extension_get_type(extension: interop.Pointer | interop.Reference<any>): WebGLExtensionType;

declare function canvas_native_webgl_context_extension_is_none(extension: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_context_extension_to_angle_instanced_arrays(extension: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_context_extension_to_draw_buffers(extension: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(extension: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_context_extension_to_lose_context(extension: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_context_extension_to_oes_vertex_array_object(extension: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_copy_tex_image2d(target: number, level: number, internalformat: number, x: number, y: number, width: number, height: number, border: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_copy_tex_sub_image2d(target: number, level: number, xoffset: number, yoffset: number, x: number, y: number, width: number, height: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_create(view: interop.Pointer | interop.Reference<any>, version: number, alpha: boolean, antialias: boolean, depth: boolean, fail_if_major_performance_caveat: boolean, power_preference: number, premultiplied_alpha: boolean, preserve_drawing_buffer: boolean, stencil: boolean, desynchronized: boolean, xr_compatible: boolean): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_create_buffer(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_create_framebuffer(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_create_no_window(width: number, height: number, version: number, alpha: boolean, antialias: boolean, depth: boolean, fail_if_major_performance_caveat: boolean, power_preference: number, premultiplied_alpha: boolean, preserve_drawing_buffer: boolean, stencil: boolean, desynchronized: boolean, xr_compatible: boolean, is_canvas: boolean): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_create_program(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_create_renderbuffer(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_create_shader(shader_type: number, state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_create_texture(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_cull_face(mode: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_delete_buffer(buffer: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_delete_framebuffer(frame_buffer: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_delete_program(program: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_delete_renderbuffer(render_buffer: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_delete_shader(shader: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_delete_texture(texture: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_depth_func(func: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_depth_mask(flag: boolean, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_depth_range(z_near: number, z_far: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_detach_shader(program: number, shader: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_disable(cap: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_disable_vertex_attrib_array(index: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_draw_arrays(mode: number, first: number, count: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_draw_buffers_draw_buffers_webgl(buffers: interop.Pointer | interop.Reference<number>, size: number, context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_draw_elements(mode: number, count: number, element_type: number, offset: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_enable(cap: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_enable_vertex_attrib_array(index: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(target: number, value: number, query: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(query: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(value: number, query: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(target: number, query: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(target: number, pname: number, query: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(target: number, pname: number, query: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(value: number, query: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(value: number, target: number, query: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_extension_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_finish(state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_flush(state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_framebuffer_attachment_parameter_destroy(parameter: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(param: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(param: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_framebuffer_attachment_parameter_get_value(param: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_framebuffer_renderbuffer(target: number, attachment: number, renderbuffertarget: number, renderbuffer: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_framebuffer_texture2d(target: number, attachment: number, textarget: number, texture: number, level: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_front_face(mode: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_generate_mipmap(target: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_get_active_attrib(program: number, index: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_active_uniform(program: number, index: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_attached_shaders(program: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_attrib_location(program: number, name: string | interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_get_buffer_parameter(target: number, pname: number, state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_get_context_attributes(state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_error(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_get_extension(name: string | interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_framebuffer_attachment_parameter(target: number, attachment: number, pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_is_context_lost(p1: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_get_parameter(pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_program_info_log(program: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_program_parameter(program: number, pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_renderbuffer_parameter(target: number, pname: number, state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_get_shader_info_log(shader: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_shader_parameter(shader: number, pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_shader_precision_format(shader_type: number, precision_type: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_shader_source(shader: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_supported_extensions(state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_supported_extensions_to_string(state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_tex_parameter(target: number, pname: number, state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_get_uniform(program: number, location: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_uniform_location(program: number, name: string | interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_get_vertex_attrib(index: number, pname: number, state: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_get_vertex_attrib_offset(index: number, pname: number, state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_hint(target: number, mode: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_is_buffer(buffer: number, state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_is_enabled(cap: number, state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_is_framebuffer(framebuffer: number, state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_is_program(program: number, state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_is_renderbuffer(renderbuffer: number, state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_is_shader(shader: number, state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_is_texture(texture: number, state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_line_width(width: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_link_program(program: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_lose_context_lose_context(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_lose_context_restore_context(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_make_current(state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_make_current_and_swap_buffers(state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(array_object: number, object: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(object: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(array_object: number, object: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(array_object: number, object: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_pixel_storei(pname: number, param: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_polygon_offset(factor: number, units: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_read_pixels_f32(x: number, y: number, width: number, height: number, format: number, pixel_type: number, pixels: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_read_pixels_u16(x: number, y: number, width: number, height: number, format: number, pixel_type: number, pixels: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_read_pixels_u8(x: number, y: number, width: number, height: number, format: number, pixel_type: number, pixels: string | interop.Pointer | interop.Reference<any>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_renderbuffer_storage(target: number, internal_format: number, width: number, height: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_resized(_state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_result_get_bool(result: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_result_get_bool_array(result: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_result_get_f32(result: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_result_get_f32_array(result: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_result_get_i32(result: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_result_get_i32_array(result: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_result_get_is_none(result: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_result_get_string(result: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_result_get_type(result: interop.Pointer | interop.Reference<any>): WebGLResultType;

declare function canvas_native_webgl_result_get_u32(result: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_result_get_u32_array(result: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_sample_coverage(value: number, invert: boolean, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_scissor(x: number, y: number, width: number, height: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_shader_precision_format_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_shader_precision_format_get_precision(shader: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_shader_precision_format_get_range_max(shader: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_shader_precision_format_get_range_min(shader: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_shader_source(shader: number, source: string | interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_state_destroy(state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_state_get_drawing_buffer_height(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_state_get_drawing_buffer_width(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_state_get_flip_y(state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_state_get_premultiplied_alpha(state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(state: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgl_stencil_func(func: number, reference: number, mask: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_stencil_func_separate(face: number, func: number, reference: number, mask: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_stencil_mask(mask: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_stencil_mask_separate(face: number, mask: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_stencil_op(fail: number, zfail: number, zpass: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_stencil_op_separate(face: number, fail: number, zfail: number, zpass: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_swap_buffers(state: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgl_tex_image2d(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, image_type: number, buf: string | interop.Pointer | interop.Reference<any>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_tex_image2d_canvas2d(target: number, level: number, internalformat: number, format: number, image_type: number, canvas: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_tex_image2d_image_asset(target: number, level: number, internalformat: number, format: number, image_type: number, image_asset: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_tex_image2d_image_none(target: number, level: number, internalformat: number, format: number, image_type: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_tex_image2d_none(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, image_type: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_tex_image2d_webgl(target: number, level: number, internalformat: number, format: number, image_type: number, webgl: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_tex_parameterf(target: number, pname: number, param: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_tex_parameteri(target: number, pname: number, param: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_tex_sub_image2d(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, image_type: number, buf: string | interop.Pointer | interop.Reference<any>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_tex_sub_image2d_asset(target: number, level: number, xoffset: number, yoffset: number, format: number, image_type: number, asset: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_tex_sub_image2d_canvas2d(target: number, level: number, xoffset: number, yoffset: number, format: number, image_type: number, canvas: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_tex_sub_image2d_webgl(target: number, level: number, xoffset: number, yoffset: number, format: number, image_type: number, webgl: interop.Pointer | interop.Reference<any>, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_to_data_url(state: interop.Pointer | interop.Reference<any>, format: string | interop.Pointer | interop.Reference<any>, quality: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgl_uniform1f(location: number, v0: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform1fv(location: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform1i(location: number, v0: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform1iv(location: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform2f(location: number, v0: number, v1: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform2fv(location: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform2i(location: number, v0: number, v1: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform2iv(location: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform3f(location: number, v0: number, v1: number, v2: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform3fv(location: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform3i(location: number, v0: number, v1: number, v2: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform3iv(location: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform4f(location: number, v0: number, v1: number, v2: number, v3: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform4fv(location: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform4i(location: number, v0: number, v1: number, v2: number, v3: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform4iv(location: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform_matrix2fv(location: number, transpose: boolean, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform_matrix3fv(location: number, transpose: boolean, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_uniform_matrix4fv(location: number, transpose: boolean, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_use_program(program: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_validate_program(program: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_vertex_attrib1f(index: number, v0: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_vertex_attrib1fv(index: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_vertex_attrib2f(index: number, v0: number, v1: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_vertex_attrib2fv(index: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_vertex_attrib3f(index: number, v0: number, v1: number, v2: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_vertex_attrib3fv(index: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_vertex_attrib4f(index: number, v0: number, v1: number, v2: number, v3: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_vertex_attrib4fv(index: number, value: interop.Pointer | interop.Reference<number>, size: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_vertex_attrib_pointer(index: number, size: number, d_type: number, normalized: boolean, stride: number, offset: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgl_viewport(x: number, y: number, width: number, height: number, state: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_adapter_get_features(adapter: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_adapter_get_limits(adapter: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<CanvasGPUSupportedLimits>;

declare function canvas_native_webgpu_adapter_info_architecture(_info: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_adapter_info_description(info: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_adapter_info_device(info: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_adapter_info_reference(info: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_adapter_info_release(info: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_adapter_info_vendor(info: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_adapter_is_fallback_adapter(adapter: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgpu_adapter_reference(adapter: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_adapter_release(adapter: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_adapter_request_adapter_info(adapter: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_adapter_request_device(
	adapter: interop.Pointer | interop.Reference<any>,
	label: string | interop.Pointer | interop.Reference<any>,
	required_features: interop.Pointer | interop.Reference<interop.Pointer | interop.Reference<any>>,
	required_features_length: number,
	required_limits: interop.Pointer | interop.Reference<CanvasGPUSupportedLimits>,
	callback: interop.FunctionReference<(p1: interop.Pointer | interop.Reference<any>, p2: interop.Pointer | interop.Reference<any>, p3: interop.Pointer | interop.Reference<any>) => void>,
	callback_data: interop.Pointer | interop.Reference<any>
): void;

declare function canvas_native_webgpu_bind_group_get_label(bind_group: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_bind_group_layout_get_label(bind_group_layout: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_bind_group_layout_reference(bind_group_layout: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_bind_group_layout_release(bind_group_layout: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_bind_group_reference(bind_group: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_bind_group_release(bind_group: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_buffer_destroy(buffer: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_buffer_get_label(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_buffer_get_mapped_range(buffer: interop.Pointer | interop.Reference<any>, offset: number, size: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_buffer_map_async(buffer: interop.Pointer | interop.Reference<any>, mode: GPUMapMode, offset: number, size: number, callback: interop.FunctionReference<(p1: CanvasGPUErrorType, p2: interop.Pointer | interop.Reference<any>, p3: interop.Pointer | interop.Reference<any>) => void>, callback_data: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_buffer_reference(buffer: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_buffer_release(buffer: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_buffer_size(buffer: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgpu_buffer_unmap(buffer: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_buffer_usage(buffer: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgpu_command_buffer_get_label(command_buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_command_buffer_reference(command_buffer: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_command_buffer_release(command_buffer: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_command_encoder_begin_compute_pass(command_encoder: interop.Pointer | interop.Reference<any>, query_set: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>, beginning_of_pass_write_index: number, end_of_pass_write_index: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_command_encoder_clear_buffer(command_encoder: interop.Pointer | interop.Reference<any>, buffer: interop.Pointer | interop.Reference<any>, offset: number, size: number): void;

declare function canvas_native_webgpu_command_encoder_copy_buffer_to_buffer(command_encoder: interop.Pointer | interop.Reference<any>, src: interop.Pointer | interop.Reference<any>, src_offset: number, dst: interop.Pointer | interop.Reference<any>, dst_offset: number, size: number): void;

declare function canvas_native_webgpu_command_encoder_copy_buffer_to_texture(command_encoder: interop.Pointer | interop.Reference<any>, src: interop.Pointer | interop.Reference<CanvasImageCopyBuffer>, dst: interop.Pointer | interop.Reference<CanvasImageCopyTexture>, copy_size: interop.Pointer | interop.Reference<CanvasExtent3d>): void;

declare function canvas_native_webgpu_command_encoder_copy_texture_to_buffer(command_encoder: interop.Pointer | interop.Reference<any>, src: interop.Pointer | interop.Reference<CanvasImageCopyTexture>, dst: interop.Pointer | interop.Reference<CanvasImageCopyBuffer>, copy_size: interop.Pointer | interop.Reference<CanvasExtent3d>): void;

declare function canvas_native_webgpu_command_encoder_copy_texture_to_texture(command_encoder: interop.Pointer | interop.Reference<any>, src: interop.Pointer | interop.Reference<CanvasImageCopyTexture>, dst: interop.Pointer | interop.Reference<CanvasImageCopyTexture>, copy_size: interop.Pointer | interop.Reference<CanvasExtent3d>): void;

declare function canvas_native_webgpu_command_encoder_finish(command_encoder: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_command_encoder_get_label(command_encoder: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_command_encoder_insert_debug_marker(command_encoder: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_command_encoder_pop_debug_group(command_encoder: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_command_encoder_push_debug_group(command_encoder: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_command_encoder_reference(command_encoder: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_command_encoder_release(command_encoder: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_command_encoder_resolve_query_set(command_encoder: interop.Pointer | interop.Reference<any>, query_set: interop.Pointer | interop.Reference<any>, first_query: number, query_count: number, dst: interop.Pointer | interop.Reference<any>, dst_offset: number): void;

declare function canvas_native_webgpu_command_encoder_write_timestamp(command_encoder: interop.Pointer | interop.Reference<any>, query_set: interop.Pointer | interop.Reference<any>, query_index: number): void;

declare function canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups(compute_pass: interop.Pointer | interop.Reference<any>, workgroup_count_x: number, workgroup_count_y: number, workgroup_count_z: number): void;

declare function canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups_indirect(compute_pass: interop.Pointer | interop.Reference<any>, indirect_buffer: interop.Pointer | interop.Reference<any>, indirect_offset: number): void;

declare function canvas_native_webgpu_compute_pass_encoder_end(compute_pass: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_compute_pass_encoder_get_label(compute_pass: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_compute_pass_encoder_insert_debug_marker(compute_pass: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_compute_pass_encoder_pop_debug_group(compute_pass: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_compute_pass_encoder_push_debug_group(compute_pass: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_compute_pass_encoder_reference(compute_pass: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_compute_pass_encoder_release(compute_pass: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_compute_pass_encoder_set_bind_group(compute_pass: interop.Pointer | interop.Reference<any>, index: number, bind_group: interop.Pointer | interop.Reference<any>, dynamic_offsets: interop.Pointer | interop.Reference<number>, dynamic_offsets_size: number, dynamic_offsets_start: number, dynamic_offsets_length: number): void;

declare function canvas_native_webgpu_compute_pass_encoder_set_pipeline(compute_pass: interop.Pointer | interop.Reference<any>, pipeline: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_compute_pipeline_get_bind_group_layout(pipeline: interop.Pointer | interop.Reference<any>, index: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_compute_pipeline_get_label(pipeline: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_compute_pipeline_reference(pipeline: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_compute_pipeline_release(pipeline: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_constants_create(): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_constants_destroy(constants: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_constants_insert(constants: interop.Pointer | interop.Reference<any>, key: string | interop.Pointer | interop.Reference<any>, value: number): void;

declare function canvas_native_webgpu_context_create(instance: interop.Pointer | interop.Reference<any>, view: interop.Pointer | interop.Reference<any>, width: number, height: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_context_create_uiview(instance: interop.Pointer | interop.Reference<any>, view: interop.Pointer | interop.Reference<any>, width: number, height: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_context_get_capabilities(context: interop.Pointer | interop.Reference<any>, adapter: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<CanvasSurfaceCapabilities>;

declare function canvas_native_webgpu_context_get_current_texture(context: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_context_present_surface(context: interop.Pointer | interop.Reference<any>, texture: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_context_reference(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_context_release(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_context_resize_uiview(context: interop.Pointer | interop.Reference<any>, view: interop.Pointer | interop.Reference<any>, width: number, height: number): void;

declare function canvas_native_webgpu_context_unconfigure(context: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_create_limits(): interop.Pointer | interop.Reference<CanvasGPUSupportedLimits>;

declare function canvas_native_webgpu_device_create_buffer(device: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>, size: number, usage: number, mapped_at_creation: boolean): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_device_create_command_encoder(device: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_device_create_pipeline_layout(device: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>, group_layouts: interop.Pointer | interop.Reference<interop.Pointer | interop.Reference<any>>, size: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_device_create_query_set(device: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>, type_: CanvasQueryType, count: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_device_create_shader_module(device: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>, source: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_device_destroy(device: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_device_get_features(device: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_device_get_label(device: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_device_get_limits(device: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<CanvasGPUSupportedLimits>;

declare function canvas_native_webgpu_device_get_queue(device: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_device_pop_error_scope(device: interop.Pointer | interop.Reference<any>, callback: interop.FunctionReference<(p1: CanvasGPUErrorType, p2: interop.Pointer | interop.Reference<any>, p3: interop.Pointer | interop.Reference<any>) => void>, userdata: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_device_push_error_scope(device: interop.Pointer | interop.Reference<any>, filter: CanvasGPUErrorFilter): void;

declare function canvas_native_webgpu_device_reference(device: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_device_release(device: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_device_set_lost_callback(device: interop.Pointer | interop.Reference<any>, callback: interop.FunctionReference<(p1: number, p2: interop.Pointer | interop.Reference<any>, p3: interop.Pointer | interop.Reference<any>) => void>, userdata: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_device_set_uncaptured_error_callback(device: interop.Pointer | interop.Reference<any>, callback: interop.FunctionReference<(p1: CanvasGPUErrorType, p2: interop.Pointer | interop.Reference<any>, p3: interop.Pointer | interop.Reference<any>) => void>, userdata: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_error_get_message(error: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_error_get_type(error: interop.Pointer | interop.Reference<any>): CanvasGPUErrorType;

declare function canvas_native_webgpu_get_pointer_addr(instance: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgpu_instance_create(): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_instance_release(instance: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_limits_release(limits: interop.Pointer | interop.Reference<CanvasGPUSupportedLimits>): void;

declare function canvas_native_webgpu_pipeline_layout_get_label(pipeline_layout: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_pipeline_layout_reference(pipeline_layout: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_pipeline_layout_release(pipeline_layout: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_query_set_destroy(query_set: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_query_set_get_count(query_set: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgpu_query_set_get_label(query_set: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_query_set_get_type(query_set: interop.Pointer | interop.Reference<any>): CanvasQueryType;

declare function canvas_native_webgpu_query_set_reference(query_set: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_query_set_release(query_set: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_queue_copy_context_to_texture(queue: interop.Pointer | interop.Reference<any>, source: interop.Pointer | interop.Reference<CanvasImageCopyCanvasRenderingContext2D>, destination: interop.Pointer | interop.Reference<CanvasImageCopyTexture>, size: interop.Pointer | interop.Reference<CanvasExtent3d>): void;

declare function canvas_native_webgpu_queue_copy_external_image_to_texture(queue: interop.Pointer | interop.Reference<any>, source: interop.Pointer | interop.Reference<CanvasImageCopyExternalImage>, destination: interop.Pointer | interop.Reference<CanvasImageCopyTexture>, size: interop.Pointer | interop.Reference<CanvasExtent3d>): void;

declare function canvas_native_webgpu_queue_copy_image_asset_to_texture(queue: interop.Pointer | interop.Reference<any>, source: interop.Pointer | interop.Reference<CanvasImageCopyImageAsset>, destination: interop.Pointer | interop.Reference<CanvasImageCopyTexture>, size: interop.Pointer | interop.Reference<CanvasExtent3d>): void;

declare function canvas_native_webgpu_queue_copy_webgl_to_texture(queue: interop.Pointer | interop.Reference<any>, source: interop.Pointer | interop.Reference<CanvasImageCopyWebGL>, destination: interop.Pointer | interop.Reference<CanvasImageCopyTexture>, size: interop.Pointer | interop.Reference<CanvasExtent3d>): void;

declare function canvas_native_webgpu_queue_get_label(queue: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_queue_on_submitted_work_done(queue: interop.Pointer | interop.Reference<any>, callback: interop.FunctionReference<(p1: interop.Pointer | interop.Reference<any>, p2: interop.Pointer | interop.Reference<any>) => void>, callback_data: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_queue_reference(queue: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_queue_release(queue: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_queue_submit(queue: interop.Pointer | interop.Reference<any>, command_buffers: interop.Pointer | interop.Reference<interop.Pointer | interop.Reference<any>>, command_buffers_size: number): void;

declare function canvas_native_webgpu_queue_write_buffer(queue: interop.Pointer | interop.Reference<any>, buffer: interop.Pointer | interop.Reference<any>, buffer_offset: number, data: string | interop.Pointer | interop.Reference<any>, data_size: number, data_offset: number, size: number): void;

declare function canvas_native_webgpu_queue_write_texture(queue: interop.Pointer | interop.Reference<any>, destination: interop.Pointer | interop.Reference<CanvasImageCopyTexture>, data_layout: interop.Pointer | interop.Reference<CanvasImageDataLayout>, size: interop.Pointer | interop.Reference<CanvasExtent3d>, buf: string | interop.Pointer | interop.Reference<any>, buf_size: number): void;

declare function canvas_native_webgpu_render_bundle_encoder_draw(render_bundle: interop.Pointer | interop.Reference<any>, vertex_count: number, instance_count: number, first_vertex: number, first_instance: number): void;

declare function canvas_native_webgpu_render_bundle_encoder_draw_indexed(render_bundle: interop.Pointer | interop.Reference<any>, index_count: number, instance_count: number, first_index: number, base_vertex: number, first_instance: number): void;

declare function canvas_native_webgpu_render_bundle_encoder_draw_indexed_indirect(render_bundle: interop.Pointer | interop.Reference<any>, indirect_buffer: interop.Pointer | interop.Reference<any>, indirect_offset: number): void;

declare function canvas_native_webgpu_render_bundle_encoder_draw_indirect(render_bundle: interop.Pointer | interop.Reference<any>, indirect_buffer: interop.Pointer | interop.Reference<any>, indirect_offset: number): void;

declare function canvas_native_webgpu_render_bundle_encoder_finish(render_bundle: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_render_bundle_encoder_get_label(render_bundle: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_render_bundle_encoder_insert_debug_marker(render_bundle: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_bundle_encoder_pop_debug_group(render_bundle: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_bundle_encoder_push_debug_group(render_bundle: interop.Pointer | interop.Reference<any>, label: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_bundle_encoder_reference(render_bundle: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_bundle_encoder_release(render_bundle: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_bundle_encoder_set_bind_group(render_bundle: interop.Pointer | interop.Reference<any>, index: number, bind_group: interop.Pointer | interop.Reference<any>, dynamic_offsets: interop.Pointer | interop.Reference<number>, dynamic_offsets_size: number, dynamic_offsets_start: number, dynamic_offsets_length: number): void;

declare function canvas_native_webgpu_render_bundle_encoder_set_index_buffer(render_bundle: interop.Pointer | interop.Reference<any>, buffer: interop.Pointer | interop.Reference<any>, index_format: CanvasIndexFormat, offset: number, size: number): void;

declare function canvas_native_webgpu_render_bundle_encoder_set_pipeline(render_bundle: interop.Pointer | interop.Reference<any>, pipeline: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_bundle_encoder_set_vertex_buffer(render_bundle: interop.Pointer | interop.Reference<any>, slot: number, buffer: interop.Pointer | interop.Reference<any>, offset: number, size: number): void;

declare function canvas_native_webgpu_render_bundle_get_label(bundle: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_render_bundle_reference(bundle: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_bundle_release(bundle: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_pass_encoder_begin_occlusion_query(render_pass: interop.Pointer | interop.Reference<any>, query_index: number): void;

declare function canvas_native_webgpu_render_pass_encoder_draw(render_pass: interop.Pointer | interop.Reference<any>, vertex_count: number, instance_count: number, first_vertex: number, first_instance: number): void;

declare function canvas_native_webgpu_render_pass_encoder_draw_indexed(render_pass: interop.Pointer | interop.Reference<any>, index_count: number, instance_count: number, first_index: number, base_vertex: number, first_instance: number): void;

declare function canvas_native_webgpu_render_pass_encoder_draw_indexed_indirect(render_pass: interop.Pointer | interop.Reference<any>, indirect_buffer: interop.Pointer | interop.Reference<any>, indirect_offset: number): void;

declare function canvas_native_webgpu_render_pass_encoder_draw_indirect(render_pass: interop.Pointer | interop.Reference<any>, indirect_buffer: interop.Pointer | interop.Reference<any>, indirect_offset: number): void;

declare function canvas_native_webgpu_render_pass_encoder_end(render_pass: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_pass_encoder_end_occlusion_query(render_pass: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_pass_encoder_execute_bundles(render_pass: interop.Pointer | interop.Reference<any>, bundles: interop.Pointer | interop.Reference<interop.Pointer | interop.Reference<any>>, bundles_size: number): void;

declare function canvas_native_webgpu_render_pass_encoder_get_label(render_pass: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_render_pass_encoder_insert_debug_marker(render_pass: interop.Pointer | interop.Reference<any>, marker_label: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_pass_encoder_multi_draw_indexed_indirect(render_pass: interop.Pointer | interop.Reference<any>, indirect_buffer: interop.Pointer | interop.Reference<any>, indirect_offset: number, count: number): void;

declare function canvas_native_webgpu_render_pass_encoder_multi_draw_indirect(render_pass: interop.Pointer | interop.Reference<any>, indirect_buffer: interop.Pointer | interop.Reference<any>, indirect_offset: number, count: number): void;

declare function canvas_native_webgpu_render_pass_encoder_pop_debug_group(render_pass: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_pass_encoder_push_debug_group(render_pass: interop.Pointer | interop.Reference<any>, group_label: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_pass_encoder_reference(render_pass: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_pass_encoder_release(render_pass: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_pass_encoder_set_bind_group(render_pass: interop.Pointer | interop.Reference<any>, index: number, bind_group: interop.Pointer | interop.Reference<any>, dynamic_offsets: interop.Pointer | interop.Reference<number>, dynamic_offsets_size: number, dynamic_offsets_start: number, dynamic_offsets_length: number): void;

declare function canvas_native_webgpu_render_pass_encoder_set_blend_constant(render_pass: interop.Pointer | interop.Reference<any>, color: interop.Pointer | interop.Reference<CanvasColor>): void;

declare function canvas_native_webgpu_render_pass_encoder_set_index_buffer(render_pass: interop.Pointer | interop.Reference<any>, buffer: interop.Pointer | interop.Reference<any>, index_format: CanvasIndexFormat, offset: number, size: number): void;

declare function canvas_native_webgpu_render_pass_encoder_set_pipeline(render_pass: interop.Pointer | interop.Reference<any>, pipeline: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_pass_encoder_set_scissor_rect(render_pass: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number): void;

declare function canvas_native_webgpu_render_pass_encoder_set_stencil_reference(render_pass: interop.Pointer | interop.Reference<any>, reference: number): void;

declare function canvas_native_webgpu_render_pass_encoder_set_vertex_buffer(render_pass: interop.Pointer | interop.Reference<any>, slot: number, buffer: interop.Pointer | interop.Reference<any>, offset: number, size: number): void;

declare function canvas_native_webgpu_render_pass_encoder_set_viewport(pass: interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, height: number, depth_min: number, depth_max: number): void;

declare function canvas_native_webgpu_render_pipeline_get_bind_group_layout(pipeline: interop.Pointer | interop.Reference<any>, index: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_render_pipeline_get_label(pipeline: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_render_pipeline_reference(pipeline: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_render_pipeline_release(pipeline: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_request_adapter(instance: interop.Pointer | interop.Reference<any>, options: interop.Pointer | interop.Reference<CanvasGPURequestAdapterOptions>, callback: interop.FunctionReference<(p1: interop.Pointer | interop.Reference<any>, p2: interop.Pointer | interop.Reference<any>) => void>, callback_data: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_sampler_get_label(sampler: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_sampler_reference(sampler: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_sampler_release(sampler: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_shader_module_get_label(shader_module: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_shader_module_reference(shader_module: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_shader_module_release(shader_module: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_struct_surface_capabilities_release(cap: interop.Pointer | interop.Reference<CanvasSurfaceCapabilities>): void;

declare function canvas_native_webgpu_texture_destroy(texture: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_texture_get_depth_or_array_layers(texture: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgpu_texture_get_dimension(texture: interop.Pointer | interop.Reference<any>): CanvasTextureDimension;

declare function canvas_native_webgpu_texture_get_height(texture: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgpu_texture_get_label(texture: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_texture_get_mip_level_count(texture: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgpu_texture_get_sample_count(texture: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgpu_texture_get_status(texture: interop.Pointer | interop.Reference<any>): SurfaceGetCurrentTextureStatus;

declare function canvas_native_webgpu_texture_get_suboptimal(texture: interop.Pointer | interop.Reference<any>): boolean;

declare function canvas_native_webgpu_texture_get_usage(texture: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgpu_texture_get_width(texture: interop.Pointer | interop.Reference<any>): number;

declare function canvas_native_webgpu_texture_reference(texture: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_texture_release(texture: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_texture_view_get_label(texture_view: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_texture_view_reference(texture_view: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_texture_view_release(texture_view: interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_webgpu_to_data_url(context: interop.Pointer | interop.Reference<any>, device: interop.Pointer | interop.Reference<any>, format: string | interop.Pointer | interop.Reference<any>, quality: number): interop.Pointer | interop.Reference<any>;

declare function canvas_native_webgpu_to_data_url_with_texture(context: interop.Pointer | interop.Reference<any>, device: interop.Pointer | interop.Reference<any>, texture: interop.Pointer | interop.Reference<any>, format: string | interop.Pointer | interop.Reference<any>, quality: number): interop.Pointer | interop.Reference<any>;
