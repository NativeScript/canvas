import {WebGLRenderingContext} from "../../WebGL/WebGLRenderingContext";

export class WebGL2RenderingContextBase extends WebGLRenderingContext {
	/* Getting GL parameter information */
	public get READ_BUFFER(): number {
		return 0x0C02;
	}

	public get UNPACK_ROW_LENGTH(): number {
		return 0x0CF2;
	}

	public get UNPACK_SKIP_ROWS(): number {
		return 0x0CF3;
	}

	public get UNPACK_SKIP_PIXELS(): number {
		return 0x0CF4;
	}

	public get PACK_ROW_LENGTH(): number {
		return 0x0D02;
	}

	public get PACK_SKIP_ROWS(): number {
		return 0x0D03;
	}

	public get PACK_SKIP_PIXELS(): number {
		return 0x0D04;
	}

	public get TEXTURE_BINDING_3D(): number {
		return 0x806A;
	}

	public get UNPACK_SKIP_IMAGES(): number {
		return 0x806D;
	}

	public get UNPACK_IMAGE_HEIGHT(): number {
		return 0x806E;
	}

	public get MAX_3D_TEXTURE_SIZE(): number {
		return 0x8073;
	}

	public get MAX_ELEMENTS_VERTICES(): number {
		return 0x80E8;
	}

	public get MAX_ELEMENTS_INDICES(): number {
		return 0x80E9;
	}

	public get MAX_TEXTURE_LOD_BIAS(): number {
		return 0x84FD;
	}

	public get MAX_FRAGMENT_UNIFORM_COMPONENTS(): number {
		return 0x8B49;
	}

	public get MAX_VERTEX_UNIFORM_COMPONENTS(): number {
		return 0x8B4A;
	}

	public get MAX_ARRAY_TEXTURE_LAYERS(): number {
		return 0x88FF;
	}

	public get MIN_PROGRAM_TEXEL_OFFSET(): number {
		return 0x8904;
	}

	public get MAX_PROGRAM_TEXEL_OFFSET(): number {
		return 0x8905;
	}

	public get MAX_VARYING_COMPONENTS(): number {
		return 0x8B4B;
	}

	public get FRAGMENT_SHADER_DERIVATIVE_HINT(): number {
		return 0x8B8B;
	}

	public get RASTERIZER_DISCARD(): number {
		return 0x8C89;
	}

	public get VERTEX_ARRAY_BINDING(): number {
		return 0x85B5;
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
		return 0x8D6B;
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
		return 0x806F;
	}

	public get TEXTURE_WRAP_R(): number {
		return 0x8072;
	}

	public get TEXTURE_MIN_LOD(): number {
		return 0x813A;
	}

	public get TEXTURE_MAX_LOD(): number {
		return 0x813B;
	}

	public get TEXTURE_BASE_LEVEL(): number {
		return 0x813C;
	}

	public get TEXTURE_MAX_LEVEL(): number {
		return 0x813D;
	}

	public get TEXTURE_COMPARE_MODE(): number {
		return 0x884C;
	}

	public get TEXTURE_COMPARE_FUNC(): number {
		return 0x884D;
	}

	public get SRGB(): number {
		return 0x8C40;
	}

	public get SRGB8(): number {
		return 0x8C41;
	}

	public get SRGB8_ALPHA8(): number {
		return 0x8C43;
	}

	public get COMPARE_REF_TO_TEXTURE(): number {
		return 0x884E;
	}

	public get RGBA32F(): number {
		return 0x8814;
	}

	public get RGB32F(): number {
		return 0x8815;
	}

	public get RGBA16F(): number {
		return 0x881A;
	}

	public get RGB16F(): number {
		return 0x881B;
	}

	public get TEXTURE_2D_ARRAY(): number {
		return 0x8C1A;
	}

	public get TEXTURE_BINDING_2D_ARRAY(): number {
		return 0x8C1D;
	}

	public get R11F_G11F_B10F(): number {
		return 0x8C3A;
	}

	public get RGB9_E5(): number {
		return 0x8C3D;
	}

	public get RGBA32UI(): number {
		return 0x8D70;
	}

	public get RGB32UI(): number {
		return 0x8D71;
	}

	public get RGBA16UI(): number {
		return 0x8D76;
	}

	public get RGB16UI(): number {
		return 0x8D77;
	}

	public get RGBA8UI(): number {
		return 0x8D7C;
	}

	public get RGB8UI(): number {
		return 0x8D7D;
	}

	public get RGBA32I(): number {
		return 0x8D82;
	}

	public get RGB32I(): number {
		return 0x8D83;
	}

	public get RGBA16I(): number {
		return 0x8D88;
	}

	public get RGB16I(): number {
		return 0x8D89;
	}

	public get RGBA8I(): number {
		return 0x8D8E;
	}

	public get RGB8I(): number {
		return 0x8D8F;
	}

	public get RED_INTEGER(): number {
		return 0x8D94;
	}

	public get RGB_INTEGER(): number {
		return 0x8D98;
	}

	public get RGBA_INTEGER(): number {
		return 0x8D99;
	}

	public get R8(): number {
		return 0x8229;
	}

	public get RG8(): number {
		return 0x822B;
	}

	public get R16F(): number {
		return 0x822D;
	}

	public get R32F(): number {
		return 0x822E;
	}

	public get RG16F(): number {
		return 0x822F;
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
		return 0x823A;
	}

	public get RG32I(): number {
		return 0x823B;
	}

	public get RG32UI(): number {
		return 0x823C;
	}

	public get R8_SNORM(): number {
		return 0x8F94;
	}

	public get RG8_SNORM(): number {
		return 0x8F95;
	}

	public get RGB8_SNORM(): number {
		return 0x8F96;
	}

	public get RGBA8_SNORM(): number {
		return 0x8F97;
	}

	public get RGB10_A2UI(): number {
		return 0x906F;
	}

	public get TEXTURE_IMMUTABLE_FORMAT(): number {
		return 0x912F;
	}

	public get TEXTURE_IMMUTABLE_LEVELS(): number {
		return 0x82DF;
	}

	public get UNSIGNED_INT_2_10_10_10_REV(): number {
		return 0x8368;
	}

	public get UNSIGNED_INT_10F_11F_11F_REV(): number {
		return 0x8C3B;
	}

	public get UNSIGNED_INT_5_9_9_9_REV(): number {
		return 0x8C3E;
	}

	public get FLOAT_32_UNSIGNED_INT_24_8_REV(): number {
		return 0x8DAD;
	}

	public get UNSIGNED_INT_24_8(): number {
		return 0x84FA;
	}

	public get HALF_FLOAT(): number {
		return 0x140B;
	}

	public get RG(): number {
		return 0x8227;
	}

	public get RG_INTEGER(): number {
		return 0x8228;
	}

	public get INT_2_10_10_10_REV(): number {
		return 0x8D9F;
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
		return 0x8C2F;
	}

	public get ANY_SAMPLES_PASSED_CONSERVATIVE(): number {
		return 0x8D6A;
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
		return 0x882A;
	}

	public get DRAW_BUFFER6(): number {
		return 0x882B;
	}

	public get DRAW_BUFFER7(): number {
		return 0x882C;
	}

	public get DRAW_BUFFER8(): number {
		return 0x882D;
	}

	public get DRAW_BUFFER9(): number {
		return 0x882E;
	}

	public get DRAW_BUFFER10(): number {
		return 0x882F;
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
		return 0x8CDF;
	}

	public get COLOR_ATTACHMENT1(): number {
		return 0x8CE1;
	}

	public get COLOR_ATTACHMENT2(): number {
		return 0x8CE2;
	}

	public get COLOR_ATTACHMENT3(): number {
		return 0x8CE3;
	}

	public get COLOR_ATTACHMENT4(): number {
		return 0x8CE4;
	}

	public get COLOR_ATTACHMENT5(): number {
		return 0x8CE5;
	}

	public get COLOR_ATTACHMENT6(): number {
		return 0x8CE6;
	}

	public get COLOR_ATTACHMENT7(): number {
		return 0x8CE7;
	}

	public get COLOR_ATTACHMENT8(): number {
		return 0x8CE8;
	}

	public get COLOR_ATTACHMENT9(): number {
		return 0x8CE9;
	}

	public get COLOR_ATTACHMENT10(): number {
		return 0x8CEA;
	}

	public get COLOR_ATTACHMENT11(): number {
		return 0x8CEB;
	}

	public get COLOR_ATTACHMENT12(): number {
		return 0x8CEC;
	}

	public get COLOR_ATTACHMENT13(): number {
		return 0x8CED;
	}

	public get COLOR_ATTACHMENT14(): number {
		return 0x8CEE;
	}

	public get COLOR_ATTACHMENT15(): number {
		return 0x8CEF;
	}

	public get SAMPLER_3D(): number {
		return 0x8B5F;
	}

	public get SAMPLER_2D_SHADOW(): number {
		return 0x8B62;
	}

	public get SAMPLER_2D_ARRAY(): number {
		return 0x8DC1;
	}

	public get SAMPLER_2D_ARRAY_SHADOW(): number {
		return 0x8DC4;
	}

	public get SAMPLER_CUBE_SHADOW(): number {
		return 0x8DC5;
	}

	public get INT_SAMPLER_2D(): number {
		return 0x8DCA;
	}

	public get INT_SAMPLER_3D(): number {
		return 0x8DCB;
	}

	public get INT_SAMPLER_CUBE(): number {
		return 0x8DCC;
	}

	public get INT_SAMPLER_2D_ARRAY(): number {
		return 0x8DCF;
	}

	public get UNSIGNED_INT_SAMPLER_2D(): number {
		return 0x8DD2;
	}

	public get UNSIGNED_INT_SAMPLER_3D(): number {
		return 0x8DD3;
	}

	public get UNSIGNED_INT_SAMPLER_CUBE(): number {
		return 0x8DD4;
	}

	public get UNSIGNED_INT_SAMPLER_2D_ARRAY(): number {
		return 0x8DD7;
	}

	public get MAX_SAMPLES(): number {
		return 0x8D57;
	}

	public get SAMPLER_BINDING(): number {
		return 0x8919;
	}

	public get PIXEL_PACK_BUFFER(): number {
		return 0x88EB;
	}

	public get PIXEL_UNPACK_BUFFER(): number {
		return 0x88EC;
	}

	public get PIXEL_PACK_BUFFER_BINDING(): number {
		return 0x88ED;
	}

	public get PIXEL_UNPACK_BUFFER_BINDING(): number {
		return 0x88EF;
	}

	public get COPY_READ_BUFFER(): number {
		return 0x8F36;
	}

	public get COPY_WRITE_BUFFER(): number {
		return 0x8F37;
	}

	public get COPY_READ_BUFFER_BINDING(): number {
		return 0x8F36;
	}

	public get COPY_WRITE_BUFFER_BINDING(): number {
		return 0x8F37;
	}

	public get FLOAT_MAT2x3(): number {
		return 0x8B65;
	}

	public get FLOAT_MAT2x4(): number {
		return 0x8B66;
	}

	public get FLOAT_MAT3x2(): number {
		return 0x8B67;
	}

	public get FLOAT_MAT3x4(): number {
		return 0x8B68;
	}

	public get FLOAT_MAT4x2(): number {
		return 0x8B69;
	}

	public get FLOAT_MAT4x3(): number {
		return 0x8B6A;
	}

	public get UNSIGNED_INT_VEC2(): number {
		return 0x8DC6;
	}

	public get UNSIGNED_INT_VEC3(): number {
		return 0x8DC7;
	}

	public get UNSIGNED_INT_VEC4(): number {
		return 0x8DC8;
	}

	public get UNSIGNED_NORMALIZED(): number {
		return 0x8C17;
	}

	public get SIGNED_NORMALIZED(): number {
		return 0x8F9C;
	}

	/* Vertex attributes */
	public get VERTEX_ATTRIB_ARRAY_INTEGER(): number {
		return 0x88FD;
	}

	public get VERTEX_ATTRIB_ARRAY_DIVISOR(): number {
		return 0x88FE;
	}

	public get TRANSFORM_FEEDBACK_BUFFER_MODE(): number {
		return 0x8C7F;
	}

	public get MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS(): number {
		return 0x8C80;
	}

	public get TRANSFORM_FEEDBACK_VARYINGS(): number {
		return 0x8C83;
	}

	public get TRANSFORM_FEEDBACK_BUFFER_START(): number {
		return 0x8C84;
	}

	public get TRANSFORM_FEEDBACK_BUFFER_SIZE(): number {
		return 0x8C85;
	}

	public get TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN(): number {
		return 0x8C88;
	}

	public get MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS(): number {
		return 0x8C8A;
	}

	/* Textures */


	/* Pixel types */

	public get MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS(): number {
		return 0x8C8B;
	}

	public get INTERLEAVED_ATTRIBS(): number {
		return 0x8C8C;
	}

	public get SEPARATE_ATTRIBS(): number {
		return 0x8C8D;
	}

	public get TRANSFORM_FEEDBACK_BUFFER(): number {
		return 0x8C8E;
	}

	public get TRANSFORM_FEEDBACK_BUFFER_BINDING(): number {
		return 0x8C8F;
	}

	public get TRANSFORM_FEEDBACK(): number {
		return 0x8E22;
	}

	public get TRANSFORM_FEEDBACK_PAUSED(): number {
		return 0x8E23;
	}

	public get TRANSFORM_FEEDBACK_ACTIVE(): number {
		return 0x8E24;
	}

	public get TRANSFORM_FEEDBACK_BINDING(): number {
		return 0x8E25;
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
		return 0x821A;
	}

	public get DEPTH_STENCIL(): number {
		return 0x84F9;
	}

	public get DEPTH24_STENCIL8(): number {
		return 0x88F0;
	}

	public get DRAW_FRAMEBUFFER_BINDING(): number {
		return 0x8CA6;
	}

	public get READ_FRAMEBUFFER(): number {
		return 0x8CA8;
	}

	public get DRAW_FRAMEBUFFER(): number {
		return 0x8CA9;
	}

	public get READ_FRAMEBUFFER_BINDING(): number {
		return 0x8CAA;
	}

	public get RENDERBUFFER_SAMPLES(): number {
		return 0x8CAB;
	}

	public get FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER(): number {
		return 0x8CD4;
	}

	public get FRAMEBUFFER_INCOMPLETE_MULTISAMPLE(): number {
		return 0x8D56;
	}

	public get UNIFORM_BUFFER(): number {
		return 0x8A11;
	}

	public get UNIFORM_BUFFER_BINDING(): number {
		return 0x8A28;
	}

	public get UNIFORM_BUFFER_START(): number {
		return 0x8A29;
	}

	public get UNIFORM_BUFFER_SIZE(): number {
		return 0x8A2A;
	}

	public get MAX_VERTEX_UNIFORM_BLOCKS(): number {
		return 0x8A2B;
	}

	public get MAX_FRAGMENT_UNIFORM_BLOCKS(): number {
		return 0x8A2D;
	}

	public get MAX_COMBINED_UNIFORM_BLOCKS(): number {
		return 0x8A2E;
	}

	public get MAX_UNIFORM_BUFFER_BINDINGS(): number {
		return 0x8A2F;
	}

	public get MAX_UNIFORM_BLOCK_SIZE(): number {
		return 0x8A30;
	}

	public get MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS(): number {
		return 0x8A31;
	}

	public get MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS(): number {
		return 0x8A33;
	}

	public get UNIFORM_BUFFER_OFFSET_ALIGNMENT(): number {
		return 0x8A34;
	}

	public get ACTIVE_UNIFORM_BLOCKS(): number {
		return 0x8A36;
	}

	public get UNIFORM_TYPE(): number {
		return 0x8A37;
	}

	public get UNIFORM_SIZE(): number {
		return 0x8A38;
	}

	public get UNIFORM_BLOCK_INDEX(): number {
		return 0x8A3A;
	}

	public get UNIFORM_OFFSET(): number {
		return 0x8A3B;
	}

	public get UNIFORM_ARRAY_STRIDE(): number {
		return 0x8A3C;
	}

	public get UNIFORM_MATRIX_STRIDE(): number {
		return 0x8A3D;
	}

	/* Draw buffers */

	/* Samplers */

	public get UNIFORM_IS_ROW_MAJOR(): number {
		return 0x8A3E;
	}

	public get UNIFORM_BLOCK_BINDING(): number {
		return 0x8A3F;
	}

	public get UNIFORM_BLOCK_DATA_SIZE(): number {
		return 0x8A40;
	}

	public get UNIFORM_BLOCK_ACTIVE_UNIFORMS(): number {
		return 0x8A42;
	}

	public get UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES(): number {
		return 0x8A43;
	}

	public get UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER(): number {
		return 0x8A44;
	}

	public get UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER(): number {
		return 0x8A46;
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
		return 0x911A;
	}

	public get TIMEOUT_EXPIRED(): number {
		return 0x911B;
	}

	public get CONDITION_SATISFIED(): number {
		return 0x911C;
	}

	public get WAIT_FAILED(): number {
		return 0x911D;
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
		return 0x81A6;
	}

	public get STREAM_READ(): number {
		return 0x88E1;
	}

	public get STREAM_COPY(): number {
		return 0x88E2;
	}

	public get STATIC_READ(): number {
		return 0x88E5;
	}

	public get STATIC_COPY(): number {
		return 0x88E6;
	}

	public get DYNAMIC_READ(): number {
		return 0x88E9;
	}

	public get DYNAMIC_COPY(): number {
		return 0x88EA;
	}

	public get DEPTH_COMPONENT32F(): number {
		return 0x8CAC;
	}

	public get DEPTH32F_STENCIL8(): number {
		return 0x8CAD;
	}


	/* Data types */

	public get INVALID_INDEX(): number {
		return 0xFFFFFFFF;
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
