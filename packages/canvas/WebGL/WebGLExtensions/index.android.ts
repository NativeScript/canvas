import {WebGLVertexArrayObject, WebGLQuery} from '../../WebGL2';
import {WebGLRenderingContext} from '../../WebGL/WebGLRenderingContext';

export class EXT_blend_minmax {
	MAX_EXT: number;
	MIN_EXT: number;

	constructor(nativeInstance) {
		this.MAX_EXT = nativeInstance.MAX_EXT;
		this.MIN_EXT = nativeInstance.MIN_EXT;
	}
}

export class ANGLE_instanced_arrays {
	VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: number;
	private nativeInstance;

	constructor(nativeInstance) {
		this.nativeInstance = nativeInstance;
		this.VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE = nativeInstance.VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE;
	}

	public drawArraysInstancedANGLE(mode: number, first: number, count: number, primcount: number) {
		this.nativeInstance.drawArraysInstancedANGLE(mode, first, count, primcount);
	}

	public drawElementsInstancedANGLE(mode: number, count: number, type: number, offset: number, primcount: number) {
		this.nativeInstance.drawElementsInstancedANGLE(mode, count, type, offset, primcount);
	}

	public vertexAttribDivisorANGLE(index: number, divisor: number) {
		this.nativeInstance.vertexAttribDivisorANGLE(index, divisor);
	}
}

export class EXT_color_buffer_float {
	R11F_G11F_B10F: number;

	R16F: number;

	R32F: number;

	RG16F: number;

	RG32F: number;

	RGB16F: number;

	RGBA32F: number;

	constructor(nativeInstance) {
		this.R11F_G11F_B10F = nativeInstance.R11F_G11F_B10F;
		this.R16F = nativeInstance.R16F;
		this.R32F = nativeInstance.R32F;
		this.RG16F = nativeInstance.RG16F;
		this.RG32F = nativeInstance.RG32F;
		this.RGB16F = nativeInstance.RGB16F;
		this.RGBA32F = nativeInstance.RGBA32F;
	}
}

export class EXT_color_buffer_half_float {
	FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: number;

	RGB16F_EXT: number;

	RGBA16F_EXT: number;

	UNSIGNED_NORMALIZED_EXT: number;

	constructor(nativeInstance) {
		this.FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT =
			nativeInstance.FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT;
		this.RGB16F_EXT = nativeInstance.RGB16F_EXT;
		this.RGBA16F_EXT = nativeInstance.RGBA16F_EXT;
		this.UNSIGNED_NORMALIZED_EXT = nativeInstance.UNSIGNED_NORMALIZED_EXT;
	}
}

export class EXT_sRGB {
	FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: number;

	SRGB8_ALPHA8_EXT: number;

	SRGB_ALPHA_EXT: number;

	SRGB_EXT: number;

	constructor(nativeInstance) {
		this.FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT =
			nativeInstance.FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT;
		this.SRGB8_ALPHA8_EXT = nativeInstance.SRGB8_ALPHA8_EXT;
		this.SRGB_ALPHA_EXT = nativeInstance.SRGB_ALPHA_EXT;
		this.SRGB_EXT = nativeInstance.SRGB_EXT;
	}
}

export class EXT_shader_texture_lod {
	constructor(nativeInstance) {
	}
}

export class EXT_texture_filter_anisotropic {
	MAX_TEXTURE_MAX_ANISOTROPY_EXT: number;

	TEXTURE_MAX_ANISOTROPY_EXT: number;

	constructor(nativeInstance) {
		this.MAX_TEXTURE_MAX_ANISOTROPY_EXT =
			nativeInstance.MAX_TEXTURE_MAX_ANISOTROPY_EXT;
		this.TEXTURE_MAX_ANISOTROPY_EXT = nativeInstance.TEXTURE_MAX_ANISOTROPY_EXT;
	}
}

export class OES_element_index_uint {
	UNSIGNED_INT: number;

	constructor(nativeInstance) {
		this.UNSIGNED_INT = nativeInstance.UNSIGNED_INT;
	}
}

export class OES_fbo_render_mipmap {
	constructor(nativeInstance) {
	}
}

export class OES_standard_derivatives {
	constructor(nativeInstance) {
	}
}

export class OES_texture_float {
	constructor(nativeInstance) {
	}
}

export class OES_texture_float_linear {
	constructor(nativeInstance) {
	}
}

export class OES_texture_half_float {
	HALF_FLOAT_OES: number;

	constructor(nativeInstance) {
		this.HALF_FLOAT_OES = nativeInstance.HALF_FLOAT_OES;
	}
}

export class OES_texture_half_float_linear {
	constructor(nativeInstance) {
	}
}

export class OES_vertex_array_object {
	VERTEX_ARRAY_BINDING_OES;
	private nativeInstance;

	constructor(nativeInstance) {
		this.nativeInstance = nativeInstance;
		this.VERTEX_ARRAY_BINDING_OES = nativeInstance.VERTEX_ARRAY_BINDING_OES;
	}

	createVertexArrayOES(): WebGLVertexArrayObject {
		return new WebGLVertexArrayObject(this.nativeInstance.createVertexArrayOES());
	}

	deleteVertexArrayOES(arrayObject: WebGLVertexArrayObject) {
		const value = arrayObject ? arrayObject.native : 0;
		this.nativeInstance.deleteVertexArrayOES(value);
	}

	isVertexArrayOES(arrayObject: WebGLVertexArrayObject) {
		const value = arrayObject ? arrayObject.native : 0;
		return this.nativeInstance.isVertexArrayOES(value);
	}

	bindVertexArrayOES(arrayObject: WebGLVertexArrayObject) {
		const value = arrayObject ? arrayObject.native : 0;
		this.nativeInstance.deleteVertexArrayOES(value);
	}

}

export class WEBGL_color_buffer_float {
	FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: number;

	RGB32F_EXT: number;

	RGBA32F_EXT: number;

	UNSIGNED_NORMALIZED_EXT: number;

	constructor(nativeInstance) {
		this.FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT =
			nativeInstance.FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT;
		this.RGB32F_EXT = nativeInstance.RGB32F_EXT;
		this.RGBA32F_EXT = nativeInstance.RGBA32F_EXT;
		this.UNSIGNED_NORMALIZED_EXT = nativeInstance.UNSIGNED_NORMALIZED_EXT;
	}
}

export class WEBGL_compressed_texture_etc {
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

	constructor(nativeInstance) {
		this.COMPRESSED_R11_EAC = nativeInstance.COMPRESSED_R11_EAC;
		this.COMPRESSED_RG11_EAC = nativeInstance.COMPRESSED_RG11_EAC;
		this.COMPRESSED_RGB8_ETC2 = nativeInstance.COMPRESSED_RGB8_ETC2;
		this.COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2 =
			nativeInstance.COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2;
		this.COMPRESSED_RGBA8_ETC2_EAC = nativeInstance.COMPRESSED_RGBA8_ETC2_EAC;
		this.COMPRESSED_SIGNED_R11_EAC = nativeInstance.COMPRESSED_SIGNED_R11_EAC;
		this.COMPRESSED_SIGNED_RG11_EAC = nativeInstance.COMPRESSED_SIGNED_RG11_EAC;
		this.COMPRESSED_SRGB8_ALPHA8_ETC2_EAC =
			nativeInstance.COMPRESSED_SRGB8_ALPHA8_ETC2_EAC;
		this.COMPRESSED_SRGB8_ETC2 = nativeInstance.COMPRESSED_SRGB8_ETC2;
		this.COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2 =
			nativeInstance.COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2;
	}
}

export class WEBGL_compressed_texture_etc1 {
	COMPRESSED_RGB_ETC1_WEBGL: number;

	constructor(nativeInstance) {
		this.COMPRESSED_RGB_ETC1_WEBGL = nativeInstance.COMPRESSED_RGB_ETC1_WEBGL;
	}
}

export class WEBGL_compressed_texture_pvrtc {
	COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: number;

	COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: number;

	COMPRESSED_RGB_PVRTC_2BPPV1_IMG: number;

	COMPRESSED_RGB_PVRTC_4BPPV1_IMG: number;

	constructor(nativeInstance) {
		this.COMPRESSED_RGBA_PVRTC_2BPPV1_IMG =
			nativeInstance.COMPRESSED_RGBA_PVRTC_2BPPV1_IMG;
		this.COMPRESSED_RGBA_PVRTC_4BPPV1_IMG =
			nativeInstance.COMPRESSED_RGBA_PVRTC_4BPPV1_IMG;
		this.COMPRESSED_RGB_PVRTC_2BPPV1_IMG =
			nativeInstance.COMPRESSED_RGB_PVRTC_2BPPV1_IMG;
		this.COMPRESSED_RGB_PVRTC_4BPPV1_IMG =
			nativeInstance.COMPRESSED_RGB_PVRTC_4BPPV1_IMG;
	}
}

export class WEBGL_depth_texture {
	UNSIGNED_INT_24_8_WEBGL: number;

	constructor(nativeInstance) {
		this.UNSIGNED_INT_24_8_WEBGL = nativeInstance.UNSIGNED_INT_24_8_WEBGL;
	}
}

export class WEBGL_lose_context {
	private nativeInstance: any;

	constructor(nativeInstance) {
		this.nativeInstance = nativeInstance;
	}

	public loseContext() {
		this.nativeInstance.loseContext();
	}

	public restoreContext() {
		this.nativeInstance.restoreContext();
	}
}

export class EXT_disjoint_timer_query {
	QUERY_COUNTER_BITS_EXT;
	CURRENT_QUERY_EXT;
	QUERY_RESULT_EXT;
	QUERY_RESULT_AVAILABLE_EXT;
	TIME_ELAPSED_EXT;
	TIMESTAMP_EXT;
	GPU_DISJOINT_EXT;
	private nativeInstance: any;

	constructor(nativeInstance) {
		this.nativeInstance = nativeInstance;
		this.QUERY_COUNTER_BITS_EXT = nativeInstance.QUERY_COUNTER_BITS_EXT;
		this.CURRENT_QUERY_EXT = nativeInstance.CURRENT_QUERY_EXT;
		this.QUERY_RESULT_EXT = nativeInstance.QUERY_RESULT_EXT;
		this.QUERY_RESULT_AVAILABLE_EXT = nativeInstance.QUERY_RESULT_AVAILABLE_EXT;
		this.TIME_ELAPSED_EXT = nativeInstance.TIME_ELAPSED_EXT;
		this.TIMESTAMP_EXT = nativeInstance.TIMESTAMP_EXT;
		this.GPU_DISJOINT_EXT = nativeInstance.GPU_DISJOINT_EXT;
	}

	public createQueryEXT(): WebGLQuery {
		return new WebGLQuery(this.nativeInstance.createQueryEXT());
	}

	public deleteQueryEXT(query: WebGLQuery) {
		const value = query ? query.native : 0;
		this.nativeInstance.deleteQueryEXT(value);
	}

	public isQueryEXT(query: WebGLQuery): boolean {
		const value = query ? query.native : 0;
		return this.nativeInstance.isQueryEXT(value);
	}

	public beginQueryEXT(target: number, query: WebGLQuery) {
		const value = query ? query.native : 0;
		this.nativeInstance.beginQueryEXT(target, value);
	}

	public endQueryEXT(target: number) {
		this.nativeInstance.endQueryEXT(target);
	}

	public queryCounterEXT(query: WebGLQuery, target: number) {
		const value = query ? query.native : 0;
		// NOOP
	}

	public getQueryEXT(target: number, pname: number) {
		const value = this.nativeInstance.getQueryEXT(target, pname);
		if (!!value) {
			return null;
		}
		switch (pname) {
			case this.CURRENT_QUERY_EXT:
				return new WebGLQuery(value);
			default:
				return value;
		}
	}

	public getQueryObjectEXT(query: WebGLQuery, pname: number) {
		const id = query ? query.native : 0;
		const value = this.nativeInstance.getQueryObjectEXT(id, pname);
		if (value instanceof java.lang.Object) {
			return (WebGLRenderingContext as any).toPrimitive(value);
		}
		return value;
	}
}


export class WEBGL_compressed_texture_atc {
	COMPRESSED_RGB_ATC_WEBGL;
	COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL;
	COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL;
	private nativeInstance: any;

	constructor(nativeInstance) {
		this.COMPRESSED_RGB_ATC_WEBGL = nativeInstance.COMPRESSED_RGB_ATC_WEBGL;
		this.COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL = nativeInstance.COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL;
		this.COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL = nativeInstance.COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL;
	}
}


export class WEBGL_compressed_texture_s3tc {
	COMPRESSED_RGB_S3TC_DXT1_EXT;
	COMPRESSED_RGBA_S3TC_DXT1_EXT;
	COMPRESSED_RGBA_S3TC_DXT3_EXT;
	COMPRESSED_RGBA_S3TC_DXT5_EXT;
	private nativeInstance: any;

	constructor(nativeInstance) {
		this.COMPRESSED_RGB_S3TC_DXT1_EXT = nativeInstance.COMPRESSED_RGB_S3TC_DXT1_EXT;
		this.COMPRESSED_RGBA_S3TC_DXT1_EXT = nativeInstance.COMPRESSED_RGBA_S3TC_DXT1_EXT;
		this.COMPRESSED_RGBA_S3TC_DXT3_EXT = nativeInstance.COMPRESSED_RGBA_S3TC_DXT3_EXT;
		this.COMPRESSED_RGBA_S3TC_DXT5_EXT = nativeInstance.COMPRESSED_RGBA_S3TC_DXT5_EXT;
	}
}

export class WEBGL_compressed_texture_s3tc_srgb {
	COMPRESSED_SRGB_S3TC_DXT1_EXT;
	COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT;
	COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT;
	COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT;
	private nativeInstance: any;

	constructor(nativeInstance) {
		this.COMPRESSED_SRGB_S3TC_DXT1_EXT = nativeInstance.COMPRESSED_SRGB_S3TC_DXT1_EXT;
		this.COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT = nativeInstance.COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT;
		this.COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT = nativeInstance.COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT;
		this.COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT = nativeInstance.COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT;
	}
}

export class WEBGL_draw_buffers {
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

	private nativeInstance: any;

	constructor(nativeInstance) {
		this.nativeInstance = nativeInstance;
		this.COLOR_ATTACHMENT0_WEBGL = nativeInstance.COLOR_ATTACHMENT0_EXT;
		this.COLOR_ATTACHMENT1_WEBGL = nativeInstance.COLOR_ATTACHMENT1_EXT;
		this.COLOR_ATTACHMENT2_WEBGL = nativeInstance.COLOR_ATTACHMENT2_EXT;
		this.COLOR_ATTACHMENT3_WEBGL = nativeInstance.COLOR_ATTACHMENT3_EXT;
		this.COLOR_ATTACHMENT4_WEBGL = nativeInstance.COLOR_ATTACHMENT4_EXT;
		this.COLOR_ATTACHMENT5_WEBGL = nativeInstance.COLOR_ATTACHMENT5_EXT;
		this.COLOR_ATTACHMENT6_WEBGL = nativeInstance.COLOR_ATTACHMENT6_EXT;
		this.COLOR_ATTACHMENT7_WEBGL = nativeInstance.COLOR_ATTACHMENT7_EXT;
		this.COLOR_ATTACHMENT8_WEBGL = nativeInstance.COLOR_ATTACHMENT8_EXT;
		this.COLOR_ATTACHMENT9_WEBGL = nativeInstance.COLOR_ATTACHMENT9_EXT;
		this.COLOR_ATTACHMENT10_WEBGL = nativeInstance.COLOR_ATTACHMENT10_EXT;
		this.COLOR_ATTACHMENT11_WEBGL = nativeInstance.COLOR_ATTACHMENT11_EXT;
		this.COLOR_ATTACHMENT12_WEBGL = nativeInstance.COLOR_ATTACHMENT12_EXT;
		this.COLOR_ATTACHMENT13_WEBGL = nativeInstance.COLOR_ATTACHMENT13_EXT;
		this.COLOR_ATTACHMENT14_WEBGL = nativeInstance.COLOR_ATTACHMENT14_EXT;
		this.COLOR_ATTACHMENT15_WEBGL = nativeInstance.COLOR_ATTACHMENT15_EXT;
		this.DRAW_BUFFER0_WEBGL = nativeInstance.DRAW_BUFFER0_EXT;
		this.DRAW_BUFFER1_WEBGL = nativeInstance.DRAW_BUFFER1_EXT;
		this.DRAW_BUFFER2_WEBGL = nativeInstance.DRAW_BUFFER2_EXT;
		this.DRAW_BUFFER3_WEBGL = nativeInstance.DRAW_BUFFER3_EXT;
		this.DRAW_BUFFER4_WEBGL = nativeInstance.DRAW_BUFFER4_EXT;
		this.DRAW_BUFFER5_WEBGL = nativeInstance.DRAW_BUFFER5_EXT;
		this.DRAW_BUFFER6_WEBGL = nativeInstance.DRAW_BUFFER6_EXT;
		this.DRAW_BUFFER7_WEBGL = nativeInstance.DRAW_BUFFER7_EXT;
		this.DRAW_BUFFER8_WEBGL = nativeInstance.DRAW_BUFFER8_EXT;
		this.DRAW_BUFFER9_WEBGL = nativeInstance.DRAW_BUFFER9_EXT;
		this.DRAW_BUFFER10_WEBGL = nativeInstance.DRAW_BUFFER10_EXT;
		this.DRAW_BUFFER11_WEBGL = nativeInstance.DRAW_BUFFER11_EXT;
		this.DRAW_BUFFER12_WEBGL = nativeInstance.DRAW_BUFFER12_EXT;
		this.DRAW_BUFFER13_WEBGL = nativeInstance.DRAW_BUFFER13_EXT;
		this.DRAW_BUFFER14_WEBGL = nativeInstance.DRAW_BUFFER14_EXT;
		this.DRAW_BUFFER15_WEBGL = nativeInstance.DRAW_BUFFER15_EXT;
		this.MAX_COLOR_ATTACHMENTS_WEBGL = nativeInstance.MAX_COLOR_ATTACHMENTS_EXT;
		this.MAX_DRAW_BUFFERS_WEBGL = nativeInstance.MAX_DRAW_BUFFERS_EXT;
	}

	public drawBuffersWEBGL(buffers: number[]) {
		this.nativeInstance.drawBuffersWEBGL(buffers);
	}

}
