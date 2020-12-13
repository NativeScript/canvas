import {CanvasRenderingContext} from "../../common";
import {WebGLShader} from '../WebGLShader';
import {WebGLFramebuffer} from '../WebGLFramebuffer';
import {WebGLTexture} from '../WebGLTexture';
import {WebGLProgram} from '../WebGLProgram';
import {WebGLUniformLocation} from '../WebGLUniformLocation';
import {WebGLActiveInfo} from '../WebGLActiveInfo';
import {WebGLRenderbuffer} from '../WebGLRenderbuffer';
import {WebGLShaderPrecisionFormat} from '../WebGLShaderPrecisionFormat';

export abstract class WebGLRenderingContextBase
	implements CanvasRenderingContext {
	abstract readonly drawingBufferHeight: number;
	abstract readonly drawingBufferWidth: number;
	public static isDebug = false;
	public static filter: 'both' | 'error' | 'args' = 'both';

	protected constructor(context) {
		this._native = context;
	}

	_native: any;

	get native() {
		return this._native;
	}

	_canvas: any;

	get canvas(): any {
		return this._canvas;
	}

	_type: string = 'none';

	get type() {
		return this._type;
	}

	// get EXTENSIONS(): number {
	//     return 7939;
	// }

	public get DEPTH_BUFFER_BIT(): number {
		return 0x00000100;
	}

	public get STENCIL_BUFFER_BIT(): number {
		return 0x00000400;
	}

	public get COLOR_BUFFER_BIT(): number {
		return 0x00004000;
	}

	public get POINTS(): number {
		return 0x0000;
	}

	public get LINES(): number {
		return 0x0001;
	}

	public get LINE_LOOP(): number {
		return 0x0002;
	}

	public get LINE_STRIP(): number {
		return 0x0003;
	}

	public get TRIANGLES(): number {
		return 0x0004;
	}

	public get TRIANGLE_STRIP(): number {
		return 0x0005;
	}

	public get TRIANGLE_FAN(): number {
		return 0x0006;
	}

	public get ZERO(): number {
		return 0;
	}

	public get ONE(): number {
		return 1;
	}


	public get SRC_COLOR(): number {
		return 0x0300;
	}

	public get ONE_MINUS_SRC_COLOR(): number {
		return 0x0301;
	}

	public get SRC_ALPHA(): number {
		return 0x0302;
	}

	public get ONE_MINUS_SRC_ALPHA(): number {
		return 0x0303;
	}

	public get DST_ALPHA(): number {
		return 0x0304;
	}

	public get ONE_MINUS_DST_ALPHA(): number {
		return 0x0305;
	}

	public get DST_COLOR(): number {
		return 0x0306;
	}

	public get ONE_MINUS_DST_COLOR(): number {
		return 0x0307;
	}

	public get SRC_ALPHA_SATURATE(): number {
		return 0x0308;
	}

	public get CONSTANT_COLOR(): number {
		return 0x8001;
	}

	public get ONE_MINUS_CONSTANT_COLOR(): number {
		return 0x8002;
	}

	public get CONSTANT_ALPHA(): number {
		return 0x8003;
	}

	public get ONE_MINUS_CONSTANT_ALPHA(): number {
		return 0x8004;
	}

	/* Blending equations */
	public get FUNC_ADD(): number {
		return 0x8006;
	}

	public get FUNC_SUBTRACT(): number {
		return 0x800A;
	}

	public get FUNC_REVERSE_SUBTRACT(): number {
		return 0x800B;
	}

	public get BLEND_EQUATION(): number {
		return 0x8009;
	}

	public get BLEND_EQUATION_RGB(): number {
		return 0x8009;
	}

	public get BLEND_EQUATION_ALPHA(): number {
		return 0x883D;
	}

	public get BLEND_DST_RGB(): number {
		return 0x80C8;
	}

	public get BLEND_SRC_RGB(): number {
		return 0x80C9;
	}

	public get BLEND_DST_ALPHA(): number {
		return 0x80CA;
	}

	public get BLEND_SRC_ALPHA(): number {
		return 0x80CB;
	}

	public get BLEND_COLOR(): number {
		return 0x8005;
	}

	public get ARRAY_BUFFER_BINDING(): number {
		return 0x8894;
	}

	public get ELEMENT_ARRAY_BUFFER_BINDING(): number {
		return 0x8895;
	}

	public get LINE_WIDTH(): number {
		return 0x0B21;
	}

	public get ALIASED_POINT_SIZE_RANGE(): number {
		return 0x846D;
	}

	public get ALIASED_LINE_WIDTH_RANGE(): number {
		return 0x846E;
	}

	public get CULL_FACE_MODE(): number {
		return 0x0B45;
	}

	public get FRONT_FACE(): number {
		return 0x0B46;
	}

	public get DEPTH_RANGE(): number {
		return 0x0B70;
	}

	public get DEPTH_WRITEMASK(): number {
		return 0x0B72;
	}

	public get DEPTH_CLEAR_VALUE(): number {
		return 0x0B73;
	}

	public get DEPTH_FUNC(): number {
		return 0x0B74;
	}

	public get STENCIL_CLEAR_VALUE(): number {
		return 0x0B91;
	}

	public get STENCIL_FUNC(): number {
		return 0x0B92;
	}

	public get STENCIL_FAIL(): number {
		return 0x0B94;
	}

	public get STENCIL_PASS_DEPTH_FAIL(): number {
		return 0x0B95;
	}

	public get STENCIL_PASS_DEPTH_PASS(): number {
		return 0x0B96;
	}

	public get STENCIL_REF(): number {
		return 0x0B97;
	}

	public get STENCIL_VALUE_MASK(): number {
		return 0x0B93;
	}

	public get STENCIL_WRITEMASK(): number {
		return 0x0B98;
	}

	public get STENCIL_BACK_FUNC(): number {
		return 0x8800;
	}

	public get STENCIL_BACK_FAIL(): number {
		return 0x8801;
	}

	public get STENCIL_BACK_PASS_DEPTH_FAIL(): number {
		return 0x8802;
	}

	public get STENCIL_BACK_PASS_DEPTH_PASS(): number {
		return 0x8803;
	}

	public get STENCIL_BACK_REF(): number {
		return 0x8CA3;
	}

	public get STENCIL_BACK_VALUE_MASK(): number {
		return 0x8CA4;
	}

	public get STENCIL_BACK_WRITEMASK(): number {
		return 0x8CA5;
	}

	// getCanvas(): Canvas;

	public get VIEWPORT(): number {
		return 0x0BA2;
	}

	public get SCISSOR_BOX(): number {
		return 0x0C10;
	}

	public get COLOR_CLEAR_VALUE(): number {
		return 0x0C22;
	}

	public get COLOR_WRITEMASK(): number {
		return 0x0C23;
	}

	public get UNPACK_ALIGNMENT(): number {
		return 0x0CF5;
	}

	public get PACK_ALIGNMENT(): number {
		return 0x0D05;
	}

	public get MAX_TEXTURE_SIZE(): number {
		return 0x0D33;
	}

	public get MAX_VIEWPORT_DIMS(): number {
		return 0x0D3A;
	}

	public get SUBPIXEL_BITS(): number {
		return 0x0D50;
	}

	public get RED_BITS(): number {
		return 0x0D52;
	}

	public get GREEN_BITS(): number {
		return 0x0D53;
	}

	public get BLUE_BITS(): number {
		return 0x0D54;
	}

	public get ALPHA_BITS(): number {
		return 0x0D55;
	}

	public get DEPTH_BITS(): number {
		return 0x0D56;
	}

	public get STENCIL_BITS(): number {
		return 0x0D57;
	}

	public get POLYGON_OFFSET_UNITS(): number {
		return 0x2A00;
	}

	public get POLYGON_OFFSET_FACTOR(): number {
		return 0x8038;
	}

	public get TEXTURE_BINDING_2D(): number {
		return 0x8069;
	}

	public get SAMPLE_BUFFERS(): number {
		return 0x80A8;
	}

	public get SAMPLES(): number {
		return 0x80A9;
	}

	public get SAMPLE_COVERAGE_VALUE(): number {
		return 0x80AA;
	}

	public get SAMPLE_COVERAGE_INVERT(): number {
		return 0x80AB;
	}

	public get COMPRESSED_TEXTURE_FORMATS(): number {
		return 0x86A3;
	}

	public get VENDOR(): number {
		return 0x1F00;
	}

	public get RENDERER(): number {
		return 0x1F01;
	}

	public get VERSION(): number {
		return 0x1F02;
	}

	public get IMPLEMENTATION_COLOR_READ_TYPE(): number {
		return 0x8B9A;
	}

	public get IMPLEMENTATION_COLOR_READ_FORMAT(): number {
		return 0x8B9B;
	}

	public get BROWSER_DEFAULT_WEBGL(): number {
		return 0x9244;
	}

	public get STATIC_DRAW(): number {
		return 0x88E4;
	}

	public get STREAM_DRAW(): number {
		return 0x88E0;
	}

	public get DYNAMIC_DRAW(): number {
		return 0x88E8;
	}

	public get ARRAY_BUFFER(): number {
		return 0x8892;
	}

	public get ELEMENT_ARRAY_BUFFER(): number {
		return 0x8893;
	}

	public get BUFFER_SIZE(): number {
		return 0x8764;
	}

	public get BUFFER_USAGE(): number {
		return 0x8765;
	}

	public get CURRENT_VERTEX_ATTRIB(): number {
		return 0x8626;
	}

	public get VERTEX_ATTRIB_ARRAY_ENABLED(): number {
		return 0x8622;
	}

	public get VERTEX_ATTRIB_ARRAY_SIZE(): number {
		return 0x8623;
	}

	public get VERTEX_ATTRIB_ARRAY_STRIDE(): number {
		return 0x8624;
	}

	public get VERTEX_ATTRIB_ARRAY_TYPE(): number {
		return 0x8625;
	}

	public get VERTEX_ATTRIB_ARRAY_NORMALIZED(): number {
		return 0x886A;
	}

	public get VERTEX_ATTRIB_ARRAY_POINTER(): number {
		return 0x8645;
	}

	public get VERTEX_ATTRIB_ARRAY_BUFFER_BINDING(): number {
		return 0x889F;
	}

	public get CULL_FACE(): number {
		return 0x0B44;
	}

	public get FRONT(): number {
		return 0x0404;
	}

	public get BACK(): number {
		return 0x0405;
	}

	public get FRONT_AND_BACK(): number {
		return 0x0408;
	}

	public get BLEND(): number {
		return 0x0BE2;
	}

	public get DEPTH_TEST(): number {
		return 0x0B71;
	}

	public get DITHER(): number {
		return 0x0BD0;
	}

	public get POLYGON_OFFSET_FILL(): number {
		return 0x8037;
	}

	public get SAMPLE_ALPHA_TO_COVERAGE(): number {
		return 0x809E;
	}

	public get SAMPLE_COVERAGE(): number {
		return 0x80A0;
	}

	public get SCISSOR_TEST(): number {
		return 0x0C11;
	}

	public get STENCIL_TEST(): number {
		return 0x0B90;
	}

	/* Errors */
	public get NO_ERROR(): number {
		return 0;
	}

	public get INVALID_ENUM(): number {
		return 0x0500;
	}

	public get INVALID_VALUE(): number {
		return 0x0501;
	}

	public get INVALID_OPERATION(): number {
		return 0x0502;
	}

	public get OUT_OF_MEMORY(): number {
		return 0x0505;
	}

	public get CONTEXT_LOST_WEBGL(): number {
		return 0x9242;
	}

	public get CW(): number {
		return 0x0900;
	}

	public get CCW(): number {
		return 0x0901;
	}

	public get DONT_CARE(): number {
		return 0x1100;
	}

	public get FASTEST(): number {
		return 0x1101;
	}

	public get NICEST(): number {
		return 0x1102;
	}

	public get GENERATE_MIPMAP_HINT(): number {
		return 0x8192;
	}

	public get BYTE(): number {
		return 0x1400;
	}

	public get UNSIGNED_BYTE(): number {
		return 0x1401;
	}

	public get SHORT(): number {
		return 0x1402;
	}

	public get UNSIGNED_SHORT(): number {
		return 0x1403;
	}

	public get INT(): number {
		return 0x1404;
	}

	public get UNSIGNED_INT(): number {
		return 0x1405;
	}

	public get FLOAT(): number {
		return 0x1406;
	}

	public get DEPTH_COMPONENT(): number {
		return 0x1902;
	}

	public get ALPHA(): number {
		return 0x1906;
	}

	public get RGB(): number {
		return 0x1907;
	}

	/* Clearing buffers */

	public get RGBA(): number {
		return 0x1908;
	}

	public get LUMINANCE(): number {
		return 0x1909;
	}

	public get LUMINANCE_ALPHA(): number {
		return 0x190A;
	}

	/* Clearing buffers */

	/* Rendering primitives */

	public get UNSIGNED_SHORT_4_4_4_4(): number {
		return 0x8033;
	}

	public get UNSIGNED_SHORT_5_5_5_1(): number {
		return 0x8034;
	}

	public get UNSIGNED_SHORT_5_6_5(): number {
		return 0x8363;
	}

	public get FRAGMENT_SHADER(): number {
		return 0x8B30;
	}

	public get VERTEX_SHADER(): number {
		return 0x8B31;
	}

	public get COMPILE_STATUS(): number {
		return 0x8B81;
	}

	public get DELETE_STATUS(): number {
		return 0x8B80;
	}

	/* Rendering primitives */

	/* Blending modes */

	public get LINK_STATUS(): number {
		return 0x8B82;
	}

	public get VALIDATE_STATUS(): number {
		return 0x8B83;
	}

	public get ATTACHED_SHADERS(): number {
		return 0x8B85;
	}

	public get ACTIVE_ATTRIBUTES(): number {
		return 0x8B89;
	}

	public get ACTIVE_UNIFORMS(): number {
		return 0x8B86;
	}

	public get MAX_VERTEX_ATTRIBS(): number {
		return 0x8869;
	}

	public get MAX_VERTEX_UNIFORM_VECTORS(): number {
		return 0x8DFB;
	}

	public get MAX_VARYING_VECTORS(): number {
		return 0x8DFC;
	}

	public get MAX_COMBINED_TEXTURE_IMAGE_UNITS(): number {
		return 0x8B4D;
	}

	public get MAX_VERTEX_TEXTURE_IMAGE_UNITS(): number {
		return 0x8B4C;
	}

	public get MAX_TEXTURE_IMAGE_UNITS(): number {
		return 0x8872;
	}


	public get MAX_FRAGMENT_UNIFORM_VECTORS(): number {
		return 0x8DFD;
	}

	public get SHADER_TYPE(): number {
		return 0x8B4F;
	}

	public get SHADING_LANGUAGE_VERSION(): number {
		return 0x8B8C;
	}

	public get CURRENT_PROGRAM(): number {
		return 0x8B8D;
	}

	/* Blending modes */

	public get NEVER(): number {
		return 0x0200;
	}

	public get LESS(): number {
		return 0x0201;
	}

	public get EQUAL(): number {
		return 0x0202;
	}

	/* Blending equations */

	/* Getting GL parameter information */

	public get LEQUAL(): number {
		return 0x0203;
	}

	public get GREATER(): number {
		return 0x0204;
	}

	public get NOTEQUAL(): number {
		return 0x0205;
	}

	public get GEQUAL(): number {
		return 0x0206;
	}

	public get ALWAYS(): number {
		return 0x0207;
	}

	public get KEEP(): number {
		return 0x1E00;
	}

	public get REPLACE(): number {
		return 0x1E01;
	}

	public get INCR(): number {
		return 0x1E02;
	}

	public get DECR(): number {
		return 0x1E03;
	}

	public get INVERT(): number {
		return 0x150A;
	}

	public get INCR_WRAP(): number {
		return 0x8507;
	}

	public get DECR_WRAP(): number {
		return 0x8508;
	}

	public get NEAREST(): number {
		return 0x2600;
	}

	public get LINEAR(): number {
		return 0x2601;
	}

	public get NEAREST_MIPMAP_NEAREST(): number {
		return 0x2700;
	}

	public get LINEAR_MIPMAP_NEAREST(): number {
		return 0x2701;
	}

	public get NEAREST_MIPMAP_LINEAR(): number {
		return 0x2702;
	}

	public get LINEAR_MIPMAP_LINEAR(): number {
		return 0x2703;
	}

	public get TEXTURE_MAG_FILTER(): number {
		return 0x2800;
	}

	public get TEXTURE_MIN_FILTER(): number {
		return 0x2801;
	}

	public get TEXTURE_WRAP_S(): number {
		return 0x2802;
	}

	public get TEXTURE_WRAP_T(): number {
		return 0x2803;
	}

	public get TEXTURE_2D(): number {
		return 0x0DE1;
	}

	public get TEXTURE(): number {
		return 0x1702;
	}

	public get TEXTURE_CUBE_MAP(): number {
		return 0x8513;
	}

	public get TEXTURE_BINDING_CUBE_MAP(): number {
		return 0x8514;
	}

	public get TEXTURE_CUBE_MAP_POSITIVE_X(): number {
		return 0x8515;
	}

	public get TEXTURE_CUBE_MAP_NEGATIVE_X(): number {
		return 0x8516;
	}

	public get TEXTURE_CUBE_MAP_POSITIVE_Y(): number {
		return 0x8517;
	}

	public get TEXTURE_CUBE_MAP_NEGATIVE_Y(): number {
		return 0x8518;
	}

	public get TEXTURE_CUBE_MAP_POSITIVE_Z(): number {
		return 0x8519;
	}

	public get TEXTURE_CUBE_MAP_NEGATIVE_Z(): number {
		return 0x851A;
	}

	public get MAX_CUBE_MAP_TEXTURE_SIZE(): number {
		return 0x851C;
	}

	public get TEXTURE0(): number {
		return 0x84C0;
	}

	public get TEXTURE1(): number {
		return 0x84C1;
	}

	public get TEXTURE2(): number {
		return 0x84C2;
	}

	public get TEXTURE3(): number {
		return 0x84C3;
	}

	public get TEXTURE4(): number {
		return 0x84C4;
	}

	public get TEXTURE5(): number {
		return 0x84C5;
	}

	public get TEXTURE6(): number {
		return 0x84C6;
	}

	public get TEXTURE7(): number {
		return 0x84C7;
	}

	public get TEXTURE8(): number {
		return 0x84C8;
	}

	public get TEXTURE9(): number {
		return 0x84C9;
	}

	public get TEXTURE10(): number {
		return 0x84CA;
	}

	public get TEXTURE11(): number {
		return 0x84CB;
	}

	public get TEXTURE12(): number {
		return 0x84CC;
	}

	public get TEXTURE13(): number {
		return 0x84CD;
	}

	public get TEXTURE14(): number {
		return 0x84CE;
	}

	public get TEXTURE15(): number {
		return 0x84CF;
	}

	public get TEXTURE16(): number {
		return 0x84D0;
	}

	public get TEXTURE17(): number {
		return 0x84D1;
	}

	public get TEXTURE18(): number {
		return 0x84D2;
	}

	public get TEXTURE19(): number {
		return 0x84D3;
	}

	public get TEXTURE20(): number {
		return 0x84D4;
	}

	public get TEXTURE21(): number {
		return 0x84D5;
	}

	public get TEXTURE22(): number {
		return 0x84D6;
	}

	public get TEXTURE23(): number {
		return 0x84D7;
	}

	public get TEXTURE24(): number {
		return 0x84D8;
	}

	public get TEXTURE25(): number {
		return 0x84D9;
	}

	public get TEXTURE26(): number {
		return 0x84DA;
	}

	public get TEXTURE27(): number {
		return 0x84DB;
	}

	public get TEXTURE28(): number {
		return 0x84DC;
	}

	public get TEXTURE29(): number {
		return 0x84DD;
	}

	/* Getting GL parameter information */

	/* Buffers */

	public get TEXTURE30(): number {
		return 0x84DE;
	}

	public get TEXTURE31(): number {
		return 0x84DF;
	}

	public get ACTIVE_TEXTURE(): number {
		return 0x84E0;
	}

	public get REPEAT(): number {
		return 0x2901;
	}

	public get CLAMP_TO_EDGE(): number {
		return 0x812F;
	}

	public get MIRRORED_REPEAT(): number {
		return 0x8370;
	}

	public get FLOAT_VEC2(): number {
		return 0x8B50;
	}

	/* Buffers */

	/* Vertex attributes */

	public get FLOAT_VEC3(): number {
		return 0x8B51;
	}

	public get FLOAT_VEC4(): number {
		return 0x8B52;
	}

	public get INT_VEC2(): number {
		return 0x8B53;
	}

	public get INT_VEC3(): number {
		return 0x8B54;
	}

	public get INT_VEC4(): number {
		return 0x8B55;
	}

	public get BOOL(): number {
		return 0x8B56;
	}

	public get BOOL_VEC2(): number {
		return 0x8B57;
	}

	public get BOOL_VEC3(): number {
		return 0x8B58;
	}

	/* Vertex attributes */

	/* Culling */

	public get BOOL_VEC4(): number {
		return 0x8B59;
	}

	public get FLOAT_MAT2(): number {
		return 0x8B5A;
	}

	public get FLOAT_MAT3(): number {
		return 0x8B5B;
	}

	public get FLOAT_MAT4(): number {
		return 0x8B5C;
	}

	/* Culling */

	/* Enabling and disabling */

	public get SAMPLER_2D(): number {
		return 0x8B5E;
	}

	public get SAMPLER_CUBE(): number {
		return 0x8B60;
	}

	public get LOW_FLOAT(): number {
		return 0x8DF0;
	}

	public get MEDIUM_FLOAT(): number {
		return 0x8DF1;
	}

	public get HIGH_FLOAT(): number {
		return 0x8DF2;
	}

	public get LOW_INT(): number {
		return 0x8DF3;
	}

	public get MEDIUM_INT(): number {
		return 0x8DF4;
	}

	public get HIGH_INT(): number {
		return 0x8DF5;
	}

	/* Enabling and disabling */

	public get FRAMEBUFFER(): number {
		return 0x8D40;
	}

	public get RENDERBUFFER(): number {
		return 0x8D41;
	}

	public get RGBA4(): number {
		return 0x8056;
	}

	public get RGB5_A1(): number {
		return 0x8057;
	}

	public get RGB565(): number {
		return 0x8D62;
	}

	public get DEPTH_COMPONENT16(): number {
		return 0x81A5;
	}

	public get STENCIL_INDEX8(): number {
		return 0x8D48;
	}

	/* Errors */

	/* Front face directions */

	public get DEPTH_STENCIL(): number {
		return 0x84F9;
	}

	public get RENDERBUFFER_WIDTH(): number {
		return 0x8D42;
	}

	/* Front face directions */

	/* Hints */

	public get RENDERBUFFER_HEIGHT(): number {
		return 0x8D43;
	}

	public get RENDERBUFFER_INTERNAL_FORMAT(): number {
		return 0x8D44;
	}

	public get RENDERBUFFER_RED_SIZE(): number {
		return 0x8D50;
	}

	public get RENDERBUFFER_GREEN_SIZE(): number {
		return 0x8D51;
	}

	/* Hints */

	/* Data types */

	public get RENDERBUFFER_BLUE_SIZE(): number {
		return 0x8D52;
	}

	public get RENDERBUFFER_ALPHA_SIZE(): number {
		return 0x8D53;
	}

	public get RENDERBUFFER_DEPTH_SIZE(): number {
		return 0x8D54;
	}

	public get RENDERBUFFER_STENCIL_SIZE(): number {
		return 0x8D55;
	}

	public get FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE(): number {
		return 0x8CD0;
	}

	public get FRAMEBUFFER_ATTACHMENT_OBJECT_NAME(): number {
		return 0x8CD1;
	}

	public get FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL(): number {
		return 0x8CD2;
	}

	/* Data types */

	/* Pixel formats */

	public get FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE(): number {
		return 0x8CD3;
	}

	public get COLOR_ATTACHMENT0(): number {
		return 0x8CE0;
	}

	public get DEPTH_ATTACHMENT(): number {
		return 0x8D00;
	}

	public get STENCIL_ATTACHMENT(): number {
		return 0x8D20;
	}

	public get DEPTH_STENCIL_ATTACHMENT(): number {
		return 0x821A;
	}

	public get NONE(): number {
		return 0;
	}

	/* Pixel formats */

	/* Pixel types */

	// public get UNSIGNED_BYTE(): number { return this.native.UNSIGNED_BYTE }

	public get FRAMEBUFFER_COMPLETE(): number {
		return 0x8CD5;
	}

	public get FRAMEBUFFER_INCOMPLETE_ATTACHMENT(): number {
		return 0x8CD6;
	}

	public get FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT(): number {
		return 0x8CD7;
	}

	/* Pixel types */

	/* Shaders */

	public get FRAMEBUFFER_INCOMPLETE_DIMENSIONS(): number {
		return 0x8CD9;
	}

	public get FRAMEBUFFER_UNSUPPORTED(): number {
		return 0x8CDD;
	}

	public get FRAMEBUFFER_BINDING(): number {
		return 0x8CA6;
	}

	public get RENDERBUFFER_BINDING(): number {
		return 0x8CA7;
	}

	public get MAX_RENDERBUFFER_SIZE(): number {
		return 0x84E8;
	}

	public get INVALID_FRAMEBUFFER_OPERATION(): number {
		return 0x0506;
	}

	public get UNPACK_FLIP_Y_WEBGL(): number {
		return 0x9240;
	}

	public get UNPACK_PREMULTIPLY_ALPHA_WEBGL(): number {
		return 0x9241;
	}

	public get UNPACK_COLORSPACE_CONVERSION_WEBGL(): number {
		return 0x9243;
	}


	toNativeArray(value: any[], type: string): any {
		return [];
	}

	abstract activeTexture(texture: number): void;

	abstract attachShader(program: WebGLProgram, shader: WebGLShader): void;

	abstract bindAttribLocation(
		program: WebGLProgram,
		index: number,
		name: string
	): void;

	abstract bindBuffer(target: number, buffer: WebGLBuffer): void;

	abstract bindFramebuffer(target: number, framebuffer: WebGLFramebuffer): void;

	abstract bindRenderbuffer(
		target: number,
		renderbuffer: WebGLRenderbuffer
	): void;

	abstract bindTexture(target: number, texture: WebGLTexture): void;

	abstract blendColor(
		red: number,
		green: number,
		blue: number,
		alpha: number
	): void;

	abstract blendEquationSeparate(modeRGB: number, modeAlpha: number): void;

	abstract blendEquation(mode: number): void;

	/* Shaders */

	/* Depth or stencil tests */

	abstract blendFuncSeparate(
		srcRGB: number,
		dstRGB: number,
		srcAlpha: number,
		dstAlpha: number
	): void;

	abstract blendFunc(sfactor: number, dfactor: number): void;

	abstract bufferData(target: number, size: number, usage: number): void;

	abstract bufferData(
		target: number,
		srcData: ArrayBuffer | ArrayBufferView,
		usage: number
	): void;

	abstract bufferSubData(
		target: number,
		offset: number,
		srcData: ArrayBuffer | ArrayBufferView
	): void;

	abstract checkFramebufferStatus(target: number): number;

	abstract clearColor(
		red: number,
		green: number,
		blue: number,
		alpha: number
	): void;

	abstract clearDepth(depth: number): void;

	/* Depth or stencil tests */

	/* Stencil actions */

	abstract clearStencil(stencil: number): void;

	abstract clear(mask: number): void;

	abstract colorMask(
		red: boolean,
		green: boolean,
		blue: boolean,
		alpha: boolean
	): void;

	abstract commit(): void;

	abstract compileShader(shader: WebGLShader): void;

	abstract compressedTexImage2D(
		target: number,
		level: number,
		internalformat: number,
		width: number,
		height: number,
		border: number,
		pixels: ArrayBufferView
	): void;

	abstract compressedTexSubImage2D(
		target: number,
		level: number,
		xoffset: number,
		yoffset: number,
		width: number,
		height: number,
		format: number,
		pixels: ArrayBufferView
	): void;

	/* Stencil actions */

	/* Textures */

	abstract copyTexImage2D(
		target: number,
		level: number,
		internalformat: number,
		x: number,
		y: number,
		width: number,
		height: number,
		border: number
	): void;

	abstract copyTexSubImage2D(
		target: number,
		level: number,
		xoffset: number,
		yoffset: number,
		x: number,
		y: number,
		width: number,
		height: number
	): void;

	abstract createBuffer(): WebGLBuffer;

	abstract createFramebuffer(): WebGLFramebuffer;

	abstract createProgram(): WebGLProgram;

	abstract createRenderbuffer(): WebGLRenderbuffer;

	abstract createShader(type: number): WebGLShader;

	abstract createTexture(): WebGLTexture;

	abstract cullFace(mode: number): void;

	abstract deleteBuffer(buffer: WebGLBuffer): void;

	abstract deleteFramebuffer(frameBuffer: WebGLFramebuffer): void;

	abstract deleteProgram(program: WebGLProgram): void;

	abstract deleteRenderbuffer(renderBuffer: WebGLRenderbuffer): void;

	abstract deleteShader(shader: WebGLRenderbuffer): void;

	abstract deleteTexture(texture: WebGLTexture): void;

	abstract depthFunc(func: number): void;

	abstract depthMask(flag: boolean): void;

	abstract depthRange(zNear: number, zFar: number): void;

	abstract detachShader(program: WebGLProgram, shader: WebGLShader): void;

	abstract disableVertexAttribArray(index: number): void;

	abstract disable(cap: number): void;

	abstract drawArrays(mode: number, first: number, count: number): void;

	abstract drawElements(
		mode: number,
		count: number,
		type: number,
		offset: number
	): void;

	abstract enableVertexAttribArray(index: number): void;

	abstract enable(cap: number): void;

	abstract finish(): void;

	abstract flush(): void;

	abstract framebufferRenderbuffer(
		target: number,
		attachment: number,
		renderbuffertarget: number,
		renderbuffer: WebGLRenderbuffer
	): void;

	abstract framebufferTexture2D(
		target: number,
		attachment: number,
		textarget: number,
		texture: WebGLTexture,
		level: number
	): void;

	abstract frontFace(mode: number): void;

	abstract generateMipmap(target: number): void;

	abstract getActiveAttrib(
		program: WebGLProgram,
		index: number
	): WebGLActiveInfo;

	abstract getActiveUniform(
		program: WebGLProgram,
		index: number
	): WebGLActiveInfo;

	abstract getAttachedShaders(program: WebGLProgram): WebGLShader[];

	abstract getAttribLocation(program: WebGLProgram, name: string): number;

	abstract getBufferParameter(target: number, pname: number): number;

	abstract getContextAttributes(): any;

	abstract getError(): number;

	abstract getExtension(name: string): any;

	abstract getFramebufferAttachmentParameter(
		target: number,
		attachment: number,
		pname: number
	): WebGLRenderbuffer | WebGLTexture | number;

	abstract getParameter(pname: number): any;

	abstract getProgramInfoLog(program: WebGLProgram): string;

	abstract getProgramParameter(program: WebGLProgram, pname: number): any;

	abstract getRenderbufferParameter(target: number, pname: number): number;

	abstract getShaderInfoLog(shader: WebGLShader): string;

	abstract getShaderParameter(shader: WebGLShader, pname: number): any;

	abstract getShaderPrecisionFormat(
		shaderType: number,
		precisionType: number
	): WebGLShaderPrecisionFormat | null;

	abstract getShaderSource(shader: WebGLShader): string;

	abstract getSupportedExtensions(): string[];

	abstract getTexParameter(target: number, pname: number): number;

	abstract getUniformLocation(
		program: WebGLProgram,
		name: string
	): WebGLUniformLocation;

	abstract getUniform(
		program: WebGLProgram,
		location: WebGLUniformLocation
	): number;

	abstract getVertexAttribOffset(index: number, pname: number): void;

	abstract getVertexAttrib(index: number, pname: number): any;

	abstract hint(target: number, mode: number): void;

	abstract isBuffer(buffer: WebGLBuffer): boolean;

	abstract isContextLost(): boolean;

	/* Textures */

	/* Uniform types */

	abstract isEnabled(cap: number): boolean;

	abstract isFramebuffer(framebuffer: WebGLFramebuffer): boolean;

	abstract isProgram(program: WebGLProgram): boolean;

	abstract isRenderbuffer(renderbuffer: WebGLRenderbuffer): boolean;

	abstract isShader(shader: WebGLShader): boolean;

	abstract isTexture(texture: WebGLTexture): boolean;

	abstract lineWidth(width: number): void;

	abstract linkProgram(program: WebGLProgram): void;

	abstract pixelStorei(pname: number, param: any): void;

	abstract polygonOffset(factor: number, units: number): void;

	abstract readPixels(
		x: number,
		y: number,
		width: number,
		height: number,
		format: number,
		type: number,
		pixels: ArrayBufferView
	): void;

	abstract renderbufferStorage(
		target: number,
		internalFormat: number,
		width: number,
		height: number
	): void;

	abstract sampleCoverage(value: number, invert: boolean): void;

	abstract scissor(x: number, y: number, width: number, height: number): void;

	abstract shaderSource(shader: WebGLShader, source: string): void;

	/* Uniform types */

	/* Shader precision-specified types */

	abstract stencilFuncSeparate(
		face: number,
		func: number,
		ref: number,
		mask: number
	): void;

	abstract stencilFunc(func: number, ref: number, mask: number): void;

	abstract stencilMaskSeparate(face: number, mask: number): void;

	abstract stencilMask(mask: number): void;

	abstract stencilOpSeparate(
		face: number,
		fail: number,
		zfail: number,
		zpass: number
	): void;

	abstract stencilOp(fail: number, zfail: number, zpass: number): void;

	/* Shader precision-specified types */

	/* Framebuffers and renderbuffers */

	abstract texImage2D(
		target: number,
		level: number,
		internalformat: number,
		format: number,
		type: number,
		pixels: any
	): void;

	abstract texImage2D(
		target: number,
		level: number,
		internalformat: number,
		width: number,
		height: number,
		border: number,
		format: number,
		type: number,
		pixels: ArrayBufferView
	): void;

	abstract texParameterf(target: number, pname: number, param: number): void;

	abstract texParameteri(target: number, pname: number, param: number): void;

	abstract texSubImage2D(
		target: number,
		level: number,
		xoffset: number,
		yoffset: number,
		width: number,
		height: number,
		format: number,
		type: number,
		pixels: ArrayBufferView
	): void;

	abstract texSubImage2D(
		target: number,
		level: number,
		xoffset: number,
		yoffset: number,
		format: number,
		type: number,
		pixels: any
	): void;

	abstract uniform1f(location: WebGLUniformLocation, v0: number): void;

	abstract uniform1iv(location: WebGLUniformLocation, value: number[]): void;

	abstract uniform1fv(location: WebGLUniformLocation, value: number[]): void;

	abstract uniform1i(location: WebGLUniformLocation, v0: number): void;

	abstract uniform2f(
		location: WebGLUniformLocation,
		v0: number,
		v1: number
	): void;

	abstract uniform2iv(location: WebGLUniformLocation, value: number[]): void;

	abstract uniform2fv(location: WebGLUniformLocation, value: number[]): void;

	abstract uniform2i(
		location: WebGLUniformLocation,
		v0: number,
		v1: number
	): void;

	abstract uniform3f(
		location: WebGLUniformLocation,
		v0: number,
		v1: number,
		v2: number
	): void;

	abstract uniform3iv(location: WebGLUniformLocation, value: number[]): void;

	abstract uniform3fv(location: WebGLUniformLocation, value: number[]): void;

	abstract uniform3i(
		location: WebGLUniformLocation,
		v0: number,
		v1: number,
		v2: number
	): void;

	abstract uniform4f(
		location: WebGLUniformLocation,
		v0: number,
		v1: number,
		v2: number,
		v3: number
	): void;

	abstract uniform4iv(location: WebGLUniformLocation, value: number[]): void;

	abstract uniform4fv(location: WebGLUniformLocation, value: number[]): void;

	abstract uniform4i(
		location: WebGLUniformLocation,
		v0: number,
		v1: number,
		v2: number,
		v3: number
	): void;

	abstract uniformMatrix2fv(
		location: WebGLUniformLocation,
		transpose: boolean,
		value: number[]
	): void;

	abstract uniformMatrix3fv(
		location: WebGLUniformLocation,
		transpose: boolean,
		value: number[]
	): void;

	abstract uniformMatrix4fv(
		location: WebGLUniformLocation,
		transpose: boolean,
		value: number[]
	): void;

	abstract useProgram(program: WebGLProgram): void;

	abstract validateProgram(program: WebGLProgram): void;

	abstract vertexAttrib1f(index: number, v0: number): void;

	abstract vertexAttrib1fv(index: number, value: number[]): void;

	abstract vertexAttrib2f(index: number, v0: number, v1: number): void;

	abstract vertexAttrib2fv(index: number, value: number[]): void;

	abstract vertexAttrib3f(
		index: number,
		v0: number,
		v1: number,
		v2: number
	): void;

	abstract vertexAttrib3fv(index: number, value: number[]): void;

	abstract vertexAttrib4f(
		index: number,
		v0: number,
		v1: number,
		v2: number,
		v3: number
	): void;

	// public get INVALID_FRAMEBUFFER_OPERATION(): number { return this.native.INVALID_FRAMEBUFFER_OPERATION }

	/* Framebuffers and renderbuffers */

	/* Pixel storage modes */

	abstract vertexAttrib4fv(index: number, value: number[]): void;

	abstract vertexAttribPointer(
		index: number,
		size: number,
		type: number,
		normalized: boolean,
		stride: number,
		offset: number
	): void;

	abstract viewport(x: number, y: number, width: number, height: number): void;

	/* Pixel storage modes */
}
