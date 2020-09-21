import {WebGLVertexArrayObject, WebGLQuery} from '../../WebGL2';

export declare class EXT_blend_minmax {
	MAX_EXT: number;
	MIN_EXT: number;

	constructor(nativeInstance);
}

export class ANGLE_instanced_arrays {

	VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: number;

	constructor(nativeInstance);

	public drawArraysInstancedANGLE(mode: number, first: number, count: number, primcount: number);

	public drawElementsInstancedANGLE(mode: number, count: number, type: number, offset: number, primcount: number);

	public vertexAttribDivisorANGLE(index: number, divisor: number);
}

export declare class EXT_color_buffer_float {
	R11F_G11F_B10F: number;

	R16F: number;

	R32F: number;

	RG16F: number;

	RG32F: number;

	RGB16F: number;

	RGBA32F: number;

	constructor(nativeInstance);
}

export declare class EXT_color_buffer_half_float {
	FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: number;

	RGB16F_EXT: number;

	RGBA16F_EXT: number;

	UNSIGNED_NORMALIZED_EXT: number;

	constructor(nativeInstance);
}

export declare class EXT_sRGB {
	FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: number;

	SRGB8_ALPHA8_EXT: number;

	SRGB_ALPHA_EXT: number;

	SRGB_EXT: number;

	constructor(nativeInstance);
}

export declare class EXT_shader_texture_lod {
	constructor(nativeInstance);
}

export declare class EXT_texture_filter_anisotropic {
	MAX_TEXTURE_MAX_ANISOTROPY_EXT: number;

	TEXTURE_MAX_ANISOTROPY_EXT: number;

	constructor(nativeInstance);
}

export declare class OES_element_index_uint {
	UNSIGNED_INT: number;

	constructor(nativeInstance);
}

export declare class OES_fbo_render_mipmap {
	constructor(nativeInstance);
}

export declare class OES_standard_derivatives {
	constructor(nativeInstance);
}

export declare class OES_texture_float {
	constructor(nativeInstance);
}

export declare class OES_texture_float_linear {
	constructor(nativeInstance);
}

export declare class OES_texture_half_float {
	HALF_FLOAT_OES: number;

	constructor(nativeInstance);
}

export declare class OES_texture_half_float_linear {
	constructor(nativeInstance);
}

export declare class OES_vertex_array_object {
	VERTEX_ARRAY_BINDING_OES: number;

	constructor(nativeInstance);

	bindVertexArrayOES(arrayObject: WebGLVertexArrayObject);

	createVertexArrayOES(): WebGLVertexArrayObject;

	deleteVertexArrayOESWithArrayObject(arrayObject: WebGLVertexArrayObject): void;

	isVertexArrayOESWithArrayObject(arrayObject: WebGLVertexArrayObject): boolean;
}

export declare class WEBGL_color_buffer_float {
	FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: number;

	RGB32F_EXT: number;

	RGBA32F_EXT: number;

	UNSIGNED_NORMALIZED_EXT: number;

	constructor(nativeInstance);
}

export declare class WEBGL_compressed_texture_etc {
	COMPRESSED_R11_EAC: number;

	COMPRESSED_RG11_EAC: number;

	COMPRESSED_RGB8_ETC2: number;

	COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: number;

	COMPRESSED_RGBA8_ETC2_EAC: number;

	COMPRESSED_SIGNED_R11_EAC: number;

	COMPRESSED_SIGNED_RG11_EAC: number;

	COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: number;

	COMPRESSED_SRGB8_ETC2: number;

	COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: number;

	constructor(nativeInstance);
}

export declare class WEBGL_compressed_texture_etc1 {
	COMPRESSED_RGB_ETC1_WEBGL: number;

	constructor(nativeInstance);
}

export declare class WEBGL_compressed_texture_pvrtc {
	COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: number;

	COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: number;

	COMPRESSED_RGB_PVRTC_2BPPV1_IMG: number;

	COMPRESSED_RGB_PVRTC_4BPPV1_IMG: number;

	constructor(nativeInstance);
}

export declare class WEBGL_depth_texture {
	UNSIGNED_INT_24_8_WEBGL: number;

	constructor(nativeInstance);
}

export declare class WEBGL_lose_context {


	constructor(nativeInstance);

	public loseContext();

	public restoreContext();
}


export declare class EXT_disjoint_timer_query {

	QUERY_COUNTER_BITS_EXT;
	CURRENT_QUERY_EXT;
	QUERY_RESULT_EXT;
	QUERY_RESULT_AVAILABLE_EXT;
	TIME_ELAPSED_EXT;
	TIMESTAMP_EXT;
	GPU_DISJOINT_EXT;

	constructor(nativeInstance);

	public createQueryEXT(): WebGLQuery;

	public deleteQueryEXT(query: WebGLQuery): void

	public isQueryEXT(query: WebGLQuery): boolean;

	public beginQueryEXT(target: number, query: WebGLQuery): void

	public endQueryEXT(target: number): void;

	public queryCounterEXT(query: WebGLQuery, target: number): void

	public getQueryEXT(target: number, pname: number): WebGLQuery | number;

	public getQueryObjectEXT(query: WebGLQuery, pname: number): boolean | number;
}


export declare class WEBGL_compressed_texture_atc {

	COMPRESSED_RGB_ATC_WEBGL;
	COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL;
	COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL;

	constructor(nativeInstance);
}


export declare class WEBGL_compressed_texture_s3tc {
	COMPRESSED_RGB_S3TC_DXT1_EXT;
	COMPRESSED_RGBA_S3TC_DXT1_EXT;
	COMPRESSED_RGBA_S3TC_DXT3_EXT;
	COMPRESSED_RGBA_S3TC_DXT5_EXT;


	constructor(nativeInstance);
}

export declare class WEBGL_compressed_texture_s3tc_srgb {
	COMPRESSED_SRGB_S3TC_DXT1_EXT;
	COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT;
	COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT;
	COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT;


	constructor(nativeInstance);
}

export declare class WEBGL_draw_buffers {
	COLOR_ATTACHMENT0_WEBGL;
	COLOR_ATTACHMENT1_WEBGL;
	COLOR_ATTACHMENT2_WEBGL;
	COLOR_ATTACHMENT3_WEBGL;
	COLOR_ATTACHMENT4_WEBGL;
	COLOR_ATTACHMENT5_WEBGL;
	COLOR_ATTACHMENT6_WEBGL;
	COLOR_ATTACHMENT7_WEBGL;
	COLOR_ATTACHMENT8_WEBGL;
	COLOR_ATTACHMENT9_WEBGL;
	COLOR_ATTACHMENT10_WEBGL;
	COLOR_ATTACHMENT11_WEBGL;
	COLOR_ATTACHMENT12_WEBGL;
	COLOR_ATTACHMENT13_WEBGL;
	COLOR_ATTACHMENT14_WEBGL;
	COLOR_ATTACHMENT15_WEBGL;
	DRAW_BUFFER0_WEBGL;
	DRAW_BUFFER1_WEBGL;
	DRAW_BUFFER2_WEBGL;
	DRAW_BUFFER3_WEBGL;
	DRAW_BUFFER4_WEBGL;
	DRAW_BUFFER5_WEBGL;
	DRAW_BUFFER6_WEBGL;
	DRAW_BUFFER7_WEBGL;
	DRAW_BUFFER8_WEBGL;
	DRAW_BUFFER9_WEBGL;
	DRAW_BUFFER10_WEBGL;
	DRAW_BUFFER11_WEBGL;
	DRAW_BUFFER12_WEBGL;
	DRAW_BUFFER13_WEBGL;
	DRAW_BUFFER14_WEBGL;
	DRAW_BUFFER15_WEBGL;
	MAX_COLOR_ATTACHMENTS_WEBGL;
	MAX_DRAW_BUFFERS_WEBGL;


	constructor(nativeInstance);
}
