import { WebGLRenderingContextBase } from '../../WebGL/WebGLRenderingContext';

export class WebGL2RenderingContextBase extends WebGLRenderingContextBase {
	constructor(context, contextOptions?) {
		super(context, contextOptions);
	}
	/* Getting GL parameter information */
	public get READ_BUFFER(): number {
		return 0x0c02;
	}

	public get UNPACK_ROW_LENGTH(): number {
		return 0x0cf2;
	}

	public get UNPACK_SKIP_ROWS(): number {
		return 0x0cf3;
	}

	public get UNPACK_SKIP_PIXELS(): number {
		return 0x0cf4;
	}

	public get PACK_ROW_LENGTH(): number {
		return 0x0d02;
	}

	public get PACK_SKIP_ROWS(): number {
		return 0x0d03;
	}

	public get PACK_SKIP_PIXELS(): number {
		return 0x0d04;
	}

	public get TEXTURE_BINDING_3D(): number {
		return 0x806a;
	}

	public get UNPACK_SKIP_IMAGES(): number {
		return 0x806d;
	}

	public get UNPACK_IMAGE_HEIGHT(): number {
		return 0x806e;
	}

	public get MAX_3D_TEXTURE_SIZE(): number {
		return 0x8073;
	}

	public get MAX_ELEMENTS_VERTICES(): number {
		return 0x80e8;
	}

	public get MAX_ELEMENTS_INDICES(): number {
		return 0x80e9;
	}

	public get MAX_TEXTURE_LOD_BIAS(): number {
		return 0x84fd;
	}

	public get MAX_FRAGMENT_UNIFORM_COMPONENTS(): number {
		return 0x8b49;
	}

	public get MAX_VERTEX_UNIFORM_COMPONENTS(): number {
		return 0x8b4a;
	}

	public get MAX_ARRAY_TEXTURE_LAYERS(): number {
		return 0x88ff;
	}

	public get MIN_PROGRAM_TEXEL_OFFSET(): number {
		return 0x8904;
	}

	public get MAX_PROGRAM_TEXEL_OFFSET(): number {
		return 0x8905;
	}

	public get MAX_VARYING_COMPONENTS(): number {
		return 0x8b4b;
	}

	public get FRAGMENT_SHADER_DERIVATIVE_HINT(): number {
		return 0x8b8b;
	}

	public get RASTERIZER_DISCARD(): number {
		return 0x8c89;
	}

	public get VERTEX_ARRAY_BINDING(): number {
		return 0x85b5;
	}

	public get MAX_VERTEX_OUTPUT_COMPONENTS(): number {
		return 0x9122;
	}

	public get MAX_FRAGMENT_INPUT_COMPONENTS(): number {
		return 0x9125;
	}

	public get MAX_SERVER_WAIT_TIMEOUT(): number {
		return 0x9111;
	}

	public get MAX_ELEMENT_INDEX(): number {
		return 0x8d6b;
	}

	public get RED(): number {
		return 0x1903;
	}

	public get RGB8(): number {
		return 0x8051;
	}

	public get RGBA8(): number {
		return 0x8058;
	}

	public get RGB10_A2(): number {
		return 0x8059;
	}

	public get TEXTURE_3D(): number {
		return 0x806f;
	}

	public get TEXTURE_WRAP_R(): number {
		return 0x8072;
	}

	public get TEXTURE_MIN_LOD(): number {
		return 0x813a;
	}

	public get TEXTURE_MAX_LOD(): number {
		return 0x813b;
	}

	public get TEXTURE_BASE_LEVEL(): number {
		return 0x813c;
	}

	public get TEXTURE_MAX_LEVEL(): number {
		return 0x813d;
	}

	public get TEXTURE_COMPARE_MODE(): number {
		return 0x884c;
	}

	public get TEXTURE_COMPARE_FUNC(): number {
		return 0x884d;
	}

	public get SRGB(): number {
		return 0x8c40;
	}

	public get SRGB8(): number {
		return 0x8c41;
	}

	public get SRGB8_ALPHA8(): number {
		return 0x8c43;
	}

	public get COMPARE_REF_TO_TEXTURE(): number {
		return 0x884e;
	}

	public get RGBA32F(): number {
		return 0x8814;
	}

	public get RGB32F(): number {
		return 0x8815;
	}

	public get RGBA16F(): number {
		return 0x881a;
	}

	public get RGB16F(): number {
		return 0x881b;
	}

	public get TEXTURE_2D_ARRAY(): number {
		return 0x8c1a;
	}

	public get TEXTURE_BINDING_2D_ARRAY(): number {
		return 0x8c1d;
	}

	public get R11F_G11F_B10F(): number {
		return 0x8c3a;
	}

	public get RGB9_E5(): number {
		return 0x8c3d;
	}

	public get RGBA32UI(): number {
		return 0x8d70;
	}

	public get RGB32UI(): number {
		return 0x8d71;
	}

	public get RGBA16UI(): number {
		return 0x8d76;
	}

	public get RGB16UI(): number {
		return 0x8d77;
	}

	public get RGBA8UI(): number {
		return 0x8d7c;
	}

	public get RGB8UI(): number {
		return 0x8d7d;
	}

	public get RGBA32I(): number {
		return 0x8d82;
	}

	public get RGB32I(): number {
		return 0x8d83;
	}

	public get RGBA16I(): number {
		return 0x8d88;
	}

	public get RGB16I(): number {
		return 0x8d89;
	}

	public get RGBA8I(): number {
		return 0x8d8e;
	}

	public get RGB8I(): number {
		return 0x8d8f;
	}

	public get RED_INTEGER(): number {
		return 0x8d94;
	}

	public get RGB_INTEGER(): number {
		return 0x8d98;
	}

	public get RGBA_INTEGER(): number {
		return 0x8d99;
	}

	public get R8(): number {
		return 0x8229;
	}

	public get RG8(): number {
		return 0x822b;
	}

	public get R16F(): number {
		return 0x822d;
	}

	public get R32F(): number {
		return 0x822e;
	}

	public get RG16F(): number {
		return 0x822f;
	}

	public get RG32F(): number {
		return 0x8230;
	}

	public get R8I(): number {
		return 0x8231;
	}

	public get R8UI(): number {
		return 0x8232;
	}

	public get R16I(): number {
		return 0x8233;
	}

	public get R16UI(): number {
		return 0x8234;
	}

	public get R32I(): number {
		return 0x8235;
	}

	public get R32UI(): number {
		return 0x8236;
	}

	public get RG8I(): number {
		return 0x8237;
	}

	public get RG8UI(): number {
		return 0x8238;
	}

	public get RG16I(): number {
		return 0x8239;
	}

	public get RG16UI(): number {
		return 0x823a;
	}

	public get RG32I(): number {
		return 0x823b;
	}

	public get RG32UI(): number {
		return 0x823c;
	}

	public get R8_SNORM(): number {
		return 0x8f94;
	}

	public get RG8_SNORM(): number {
		return 0x8f95;
	}

	public get RGB8_SNORM(): number {
		return 0x8f96;
	}

	public get RGBA8_SNORM(): number {
		return 0x8f97;
	}

	public get RGB10_A2UI(): number {
		return 0x906f;
	}

	public get TEXTURE_IMMUTABLE_FORMAT(): number {
		return 0x912f;
	}

	public get TEXTURE_IMMUTABLE_LEVELS(): number {
		return 0x82df;
	}

	public get UNSIGNED_INT_2_10_10_10_REV(): number {
		return 0x8368;
	}

	public get UNSIGNED_INT_10F_11F_11F_REV(): number {
		return 0x8c3b;
	}

	public get UNSIGNED_INT_5_9_9_9_REV(): number {
		return 0x8c3e;
	}

	public get FLOAT_32_UNSIGNED_INT_24_8_REV(): number {
		return 0x8dad;
	}

	public get UNSIGNED_INT_24_8(): number {
		return 0x84fa;
	}

	public get HALF_FLOAT(): number {
		return 0x140b;
	}

	public get RG(): number {
		return 0x8227;
	}

	public get RG_INTEGER(): number {
		return 0x8228;
	}

	public get INT_2_10_10_10_REV(): number {
		return 0x8d9f;
	}

	public get QUERY_RESULT_AVAILABLE(): number {
		return 0x8865;
	}

	public get QUERY_RESULT(): number {
		return 0x8866;
	}

	public get CURRENT_QUERY(): number {
		return 0x8867;
	}

	public get ANY_SAMPLES_PASSED(): number {
		return 0x8c2f;
	}

	public get ANY_SAMPLES_PASSED_CONSERVATIVE(): number {
		return 0x8d6a;
	}

	public get MAX_DRAW_BUFFERS(): number {
		return 0x8824;
	}

	public get DRAW_BUFFER0(): number {
		return 0x8825;
	}

	public get DRAW_BUFFER1(): number {
		return 0x8826;
	}

	public get DRAW_BUFFER2(): number {
		return 0x8827;
	}

	public get DRAW_BUFFER3(): number {
		return 0x8828;
	}

	public get DRAW_BUFFER4(): number {
		return 0x8829;
	}

	public get DRAW_BUFFER5(): number {
		return 0x882a;
	}

	public get DRAW_BUFFER6(): number {
		return 0x882b;
	}

	public get DRAW_BUFFER7(): number {
		return 0x882c;
	}

	public get DRAW_BUFFER8(): number {
		return 0x882d;
	}

	public get DRAW_BUFFER9(): number {
		return 0x882e;
	}

	public get DRAW_BUFFER10(): number {
		return 0x882f;
	}

	/* Getting GL parameter information */

	/* Textures */

	public get DRAW_BUFFER11(): number {
		return 0x8830;
	}

	public get DRAW_BUFFER12(): number {
		return 0x8831;
	}

	public get DRAW_BUFFER13(): number {
		return 0x8832;
	}

	public get DRAW_BUFFER14(): number {
		return 0x8833;
	}

	public get DRAW_BUFFER15(): number {
		return 0x8834;
	}

	public get MAX_COLOR_ATTACHMENTS(): number {
		return 0x8cdf;
	}

	public get COLOR_ATTACHMENT1(): number {
		return 0x8ce1;
	}

	public get COLOR_ATTACHMENT2(): number {
		return 0x8ce2;
	}

	public get COLOR_ATTACHMENT3(): number {
		return 0x8ce3;
	}

	public get COLOR_ATTACHMENT4(): number {
		return 0x8ce4;
	}

	public get COLOR_ATTACHMENT5(): number {
		return 0x8ce5;
	}

	public get COLOR_ATTACHMENT6(): number {
		return 0x8ce6;
	}

	public get COLOR_ATTACHMENT7(): number {
		return 0x8ce7;
	}

	public get COLOR_ATTACHMENT8(): number {
		return 0x8ce8;
	}

	public get COLOR_ATTACHMENT9(): number {
		return 0x8ce9;
	}

	public get COLOR_ATTACHMENT10(): number {
		return 0x8cea;
	}

	public get COLOR_ATTACHMENT11(): number {
		return 0x8ceb;
	}

	public get COLOR_ATTACHMENT12(): number {
		return 0x8cec;
	}

	public get COLOR_ATTACHMENT13(): number {
		return 0x8ced;
	}

	public get COLOR_ATTACHMENT14(): number {
		return 0x8cee;
	}

	public get COLOR_ATTACHMENT15(): number {
		return 0x8cef;
	}

	public get SAMPLER_3D(): number {
		return 0x8b5f;
	}

	public get SAMPLER_2D_SHADOW(): number {
		return 0x8b62;
	}

	public get SAMPLER_2D_ARRAY(): number {
		return 0x8dc1;
	}

	public get SAMPLER_2D_ARRAY_SHADOW(): number {
		return 0x8dc4;
	}

	public get SAMPLER_CUBE_SHADOW(): number {
		return 0x8dc5;
	}

	public get INT_SAMPLER_2D(): number {
		return 0x8dca;
	}

	public get INT_SAMPLER_3D(): number {
		return 0x8dcb;
	}

	public get INT_SAMPLER_CUBE(): number {
		return 0x8dcc;
	}

	public get INT_SAMPLER_2D_ARRAY(): number {
		return 0x8dcf;
	}

	public get UNSIGNED_INT_SAMPLER_2D(): number {
		return 0x8dd2;
	}

	public get UNSIGNED_INT_SAMPLER_3D(): number {
		return 0x8dd3;
	}

	public get UNSIGNED_INT_SAMPLER_CUBE(): number {
		return 0x8dd4;
	}

	public get UNSIGNED_INT_SAMPLER_2D_ARRAY(): number {
		return 0x8dd7;
	}

	public get MAX_SAMPLES(): number {
		return 0x8d57;
	}

	public get SAMPLER_BINDING(): number {
		return 0x8919;
	}

	public get PIXEL_PACK_BUFFER(): number {
		return 0x88eb;
	}

	public get PIXEL_UNPACK_BUFFER(): number {
		return 0x88ec;
	}

	public get PIXEL_PACK_BUFFER_BINDING(): number {
		return 0x88ed;
	}

	public get PIXEL_UNPACK_BUFFER_BINDING(): number {
		return 0x88ef;
	}

	public get COPY_READ_BUFFER(): number {
		return 0x8f36;
	}

	public get COPY_WRITE_BUFFER(): number {
		return 0x8f37;
	}

	public get COPY_READ_BUFFER_BINDING(): number {
		return 0x8f36;
	}

	public get COPY_WRITE_BUFFER_BINDING(): number {
		return 0x8f37;
	}

	public get FLOAT_MAT2x3(): number {
		return 0x8b65;
	}

	public get FLOAT_MAT2x4(): number {
		return 0x8b66;
	}

	public get FLOAT_MAT3x2(): number {
		return 0x8b67;
	}

	public get FLOAT_MAT3x4(): number {
		return 0x8b68;
	}

	public get FLOAT_MAT4x2(): number {
		return 0x8b69;
	}

	public get FLOAT_MAT4x3(): number {
		return 0x8b6a;
	}

	public get UNSIGNED_INT_VEC2(): number {
		return 0x8dc6;
	}

	public get UNSIGNED_INT_VEC3(): number {
		return 0x8dc7;
	}

	public get UNSIGNED_INT_VEC4(): number {
		return 0x8dc8;
	}

	public get UNSIGNED_NORMALIZED(): number {
		return 0x8c17;
	}

	public get SIGNED_NORMALIZED(): number {
		return 0x8f9c;
	}

	/* Vertex attributes */
	public get VERTEX_ATTRIB_ARRAY_INTEGER(): number {
		return 0x88fd;
	}

	public get VERTEX_ATTRIB_ARRAY_DIVISOR(): number {
		return 0x88fe;
	}

	public get TRANSFORM_FEEDBACK_BUFFER_MODE(): number {
		return 0x8c7f;
	}

	public get MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS(): number {
		return 0x8c80;
	}

	public get TRANSFORM_FEEDBACK_VARYINGS(): number {
		return 0x8c83;
	}

	public get TRANSFORM_FEEDBACK_BUFFER_START(): number {
		return 0x8c84;
	}

	public get TRANSFORM_FEEDBACK_BUFFER_SIZE(): number {
		return 0x8c85;
	}

	public get TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN(): number {
		return 0x8c88;
	}

	public get MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS(): number {
		return 0x8c8a;
	}

	/* Textures */

	/* Pixel types */

	public get MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS(): number {
		return 0x8c8b;
	}

	public get INTERLEAVED_ATTRIBS(): number {
		return 0x8c8c;
	}

	public get SEPARATE_ATTRIBS(): number {
		return 0x8c8d;
	}

	public get TRANSFORM_FEEDBACK_BUFFER(): number {
		return 0x8c8e;
	}

	public get TRANSFORM_FEEDBACK_BUFFER_BINDING(): number {
		return 0x8c8f;
	}

	public get TRANSFORM_FEEDBACK(): number {
		return 0x8e22;
	}

	public get TRANSFORM_FEEDBACK_PAUSED(): number {
		return 0x8e23;
	}

	public get TRANSFORM_FEEDBACK_ACTIVE(): number {
		return 0x8e24;
	}

	public get TRANSFORM_FEEDBACK_BINDING(): number {
		return 0x8e25;
	}

	/* Pixel types */

	/* Queries */

	public get FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING(): number {
		return 0x8210;
	}

	public get FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE(): number {
		return 0x8211;
	}

	public get FRAMEBUFFER_ATTACHMENT_RED_SIZE(): number {
		return 0x8212;
	}

	public get FRAMEBUFFER_ATTACHMENT_GREEN_SIZE(): number {
		return 0x8213;
	}

	public get FRAMEBUFFER_ATTACHMENT_BLUE_SIZE(): number {
		return 0x8214;
	}

	/* Queries */

	/* Draw buffers */

	public get FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE(): number {
		return 0x8215;
	}

	public get FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE(): number {
		return 0x8216;
	}

	public get FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE(): number {
		return 0x8217;
	}

	public get FRAMEBUFFER_DEFAULT(): number {
		return 0x8218;
	}

	public get DEPTH_STENCIL_ATTACHMENT(): number {
		return 0x821a;
	}

	public get DEPTH_STENCIL(): number {
		return 0x84f9;
	}

	public get DEPTH24_STENCIL8(): number {
		return 0x88f0;
	}

	public get DRAW_FRAMEBUFFER_BINDING(): number {
		return 0x8ca6;
	}

	public get READ_FRAMEBUFFER(): number {
		return 0x8ca8;
	}

	public get DRAW_FRAMEBUFFER(): number {
		return 0x8ca9;
	}

	public get READ_FRAMEBUFFER_BINDING(): number {
		return 0x8caa;
	}

	public get RENDERBUFFER_SAMPLES(): number {
		return 0x8cab;
	}

	public get FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER(): number {
		return 0x8cd4;
	}

	public get FRAMEBUFFER_INCOMPLETE_MULTISAMPLE(): number {
		return 0x8d56;
	}

	public get UNIFORM_BUFFER(): number {
		return 0x8a11;
	}

	public get UNIFORM_BUFFER_BINDING(): number {
		return 0x8a28;
	}

	public get UNIFORM_BUFFER_START(): number {
		return 0x8a29;
	}

	public get UNIFORM_BUFFER_SIZE(): number {
		return 0x8a2a;
	}

	public get MAX_VERTEX_UNIFORM_BLOCKS(): number {
		return 0x8a2b;
	}

	public get MAX_FRAGMENT_UNIFORM_BLOCKS(): number {
		return 0x8a2d;
	}

	public get MAX_COMBINED_UNIFORM_BLOCKS(): number {
		return 0x8a2e;
	}

	public get MAX_UNIFORM_BUFFER_BINDINGS(): number {
		return 0x8a2f;
	}

	public get MAX_UNIFORM_BLOCK_SIZE(): number {
		return 0x8a30;
	}

	public get MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS(): number {
		return 0x8a31;
	}

	public get MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS(): number {
		return 0x8a33;
	}

	public get UNIFORM_BUFFER_OFFSET_ALIGNMENT(): number {
		return 0x8a34;
	}

	public get ACTIVE_UNIFORM_BLOCKS(): number {
		return 0x8a36;
	}

	public get UNIFORM_TYPE(): number {
		return 0x8a37;
	}

	public get UNIFORM_SIZE(): number {
		return 0x8a38;
	}

	public get UNIFORM_BLOCK_INDEX(): number {
		return 0x8a3a;
	}

	public get UNIFORM_OFFSET(): number {
		return 0x8a3b;
	}

	public get UNIFORM_ARRAY_STRIDE(): number {
		return 0x8a3c;
	}

	public get UNIFORM_MATRIX_STRIDE(): number {
		return 0x8a3d;
	}

	/* Draw buffers */

	/* Samplers */

	public get UNIFORM_IS_ROW_MAJOR(): number {
		return 0x8a3e;
	}

	public get UNIFORM_BLOCK_BINDING(): number {
		return 0x8a3f;
	}

	public get UNIFORM_BLOCK_DATA_SIZE(): number {
		return 0x8a40;
	}

	public get UNIFORM_BLOCK_ACTIVE_UNIFORMS(): number {
		return 0x8a42;
	}

	public get UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES(): number {
		return 0x8a43;
	}

	public get UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER(): number {
		return 0x8a44;
	}

	public get UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER(): number {
		return 0x8a46;
	}

	public get OBJECT_TYPE(): number {
		return 0x9112;
	}

	public get SYNC_CONDITION(): number {
		return 0x9113;
	}

	public get SYNC_STATUS(): number {
		return 0x9114;
	}

	public get SYNC_FLAGS(): number {
		return 0x9115;
	}

	public get SYNC_FENCE(): number {
		return 0x9116;
	}

	public get SYNC_GPU_COMMANDS_COMPLETE(): number {
		return 0x9117;
	}

	public get UNSIGNALED(): number {
		return 0x9118;
	}

	public get SIGNALED(): number {
		return 0x9119;
	}

	/* Samplers */

	/* Buffers */

	public get ALREADY_SIGNALED(): number {
		return 0x911a;
	}

	public get TIMEOUT_EXPIRED(): number {
		return 0x911b;
	}

	public get CONDITION_SATISFIED(): number {
		return 0x911c;
	}

	public get WAIT_FAILED(): number {
		return 0x911d;
	}

	public get SYNC_FLUSH_COMMANDS_BIT(): number {
		return 0x00000001;
	}

	public get COLOR(): number {
		return 0x1800;
	}

	public get DEPTH(): number {
		return 0x1801;
	}

	public get STENCIL(): number {
		return 0x1802;
	}

	/* Buffers */

	/* Data types */

	public get MIN(): number {
		return 0x8007;
	}

	public get MAX(): number {
		return 0x8008;
	}

	public get DEPTH_COMPONENT24(): number {
		return 0x81a6;
	}

	public get STREAM_READ(): number {
		return 0x88e1;
	}

	public get STREAM_COPY(): number {
		return 0x88e2;
	}

	public get STATIC_READ(): number {
		return 0x88e5;
	}

	public get STATIC_COPY(): number {
		return 0x88e6;
	}

	public get DYNAMIC_READ(): number {
		return 0x88e9;
	}

	public get DYNAMIC_COPY(): number {
		return 0x88ea;
	}

	public get DEPTH_COMPONENT32F(): number {
		return 0x8cac;
	}

	public get DEPTH32F_STENCIL8(): number {
		return 0x8cad;
	}

	/* Data types */

	public get INVALID_INDEX(): number {
		return 0xffffffff;
	}

	public get TIMEOUT_IGNORED(): number {
		return -1;
	}

	/* Vertex attributes */

	/* Transform feedback */

	public get MAX_CLIENT_WAIT_TIMEOUT_WEBGL(): number {
		return 0x9247;
	}
}
