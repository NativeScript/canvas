import {WebGLQuery} from '../WebGLQuery';
import {WebGLSampler} from '../WebGLSampler';
import {WebGLSync} from '../WebGLSync';
import {WebGLTransformFeedback} from '../WebGLTransformFeedback';
import {WebGLVertexArrayObject} from '../WebGLVertexArrayObject';
import {WebGLShader} from '../../WebGL/WebGLShader';
import {WebGLFramebuffer} from '../../WebGL/WebGLFramebuffer';
import {WebGLTexture} from '../../WebGL/WebGLTexture';
import {WebGLProgram} from '../../WebGL/WebGLProgram';
import {WebGLUniformLocation} from '../../WebGL/WebGLUniformLocation';
import {WebGLActiveInfo} from '../../WebGL/WebGLActiveInfo';
import {WebGLRenderbuffer} from '../../WebGL/WebGLRenderbuffer';
import {WebGLShaderPrecisionFormat} from '../../WebGL/WebGLShaderPrecisionFormat';
import {WebGLBuffer} from '../../WebGL/WebGLBuffer';
import {WebGL2RenderingContextBase} from "./common";

export declare class WebGL2RenderingContext extends WebGL2RenderingContextBase {
	readonly READ_BUFFER: number;
	readonly UNPACK_ROW_LENGTH: number;
	readonly UNPACK_SKIP_ROWS: number;
	readonly UNPACK_SKIP_PIXELS: number;
	readonly PACK_ROW_LENGTH: number;
	readonly PACK_SKIP_ROWS: number;
	readonly PACK_SKIP_PIXELS: number;
	readonly TEXTURE_BINDING_3D: number;
	readonly UNPACK_SKIP_IMAGES: number;
	readonly UNPACK_IMAGE_HEIGHT: number;
	readonly MAX_3D_TEXTURE_SIZE: number;
	readonly MAX_ELEMENTS_VERTICES: number;
	readonly MAX_ELEMENTS_INDICES: number;
	readonly MAX_TEXTURE_LOD_BIAS: number;
	readonly MAX_FRAGMENT_UNIFORM_COMPONENTS: number;
	readonly MAX_VERTEX_UNIFORM_COMPONENTS: number;
	readonly MAX_ARRAY_TEXTURE_LAYERS: number;
	readonly MIN_PROGRAM_TEXEL_OFFSET: number;
	readonly MAX_PROGRAM_TEXEL_OFFSET: number;
	readonly MAX_VARYING_COMPONENTS: number;
	readonly FRAGMENT_SHADER_DERIVATIVE_HINT: number;
	readonly RASTERIZER_DISCARD: number;
	readonly VERTEX_ARRAY_BINDING: number;
	readonly MAX_VERTEX_OUTPUT_COMPONENTS: number;
	readonly MAX_FRAGMENT_INPUT_COMPONENTS: number;
	readonly MAX_SERVER_WAIT_TIMEOUT: number;
	readonly MAX_ELEMENT_INDEX: number;
	readonly RED: number;
	readonly RGB8: number;
	readonly RGBA8: number;
	readonly RGB10_A2: number;
	readonly TEXTURE_3D: number;
	readonly TEXTURE_WRAP_R: number;
	readonly TEXTURE_MIN_LOD: number;
	readonly TEXTURE_MAX_LOD: number;
	readonly TEXTURE_BASE_LEVEL: number;
	readonly TEXTURE_MAX_LEVEL: number;
	readonly TEXTURE_COMPARE_MODE: number;
	readonly TEXTURE_COMPARE_FUNC: number;
	readonly SRGB: number;
	readonly SRGB8: number;
	readonly SRGB8_ALPHA8: number;
	readonly COMPARE_REF_TO_TEXTURE: number;
	readonly RGBA32F: number;
	readonly RGB32F: number;
	readonly RGBA16F: number;
	readonly RGB16F: number;
	readonly TEXTURE_2D_ARRAY: number;
	readonly TEXTURE_BINDING_2D_ARRAY: number;
	readonly R11F_G11F_B10F: number;
	readonly RGB9_E5: number;
	readonly RGBA32UI: number;
	readonly RGB32UI: number;
	readonly RGBA16UI: number;
	readonly RGB16UI: number;
	readonly RGBA8UI: number;
	readonly RGB8UI: number;
	readonly RGBA32I: number;
	readonly RGB32I: number;
	readonly RGBA16I: number;
	readonly RGB16I: number;
	readonly RGBA8I: number;
	readonly RGB8I: number;
	readonly RED_INTEGER: number;
	readonly RGB_INTEGER: number;
	readonly RGBA_INTEGER: number;
	readonly R8: number;
	readonly RG8: number;
	readonly R16F: number;
	readonly R32F: number;
	readonly RG16F: number;
	readonly RG32F: number;
	readonly R8I: number;
	readonly R8UI: number;
	readonly R16I: number;
	readonly R16UI: number;
	readonly R32I: number;
	readonly R32UI: number;
	readonly RG8I: number;
	readonly RG8UI: number;
	readonly RG16I: number;
	readonly RG16UI: number;
	readonly RG32I: number;
	readonly RG32UI: number;
	readonly R8_SNORM: number;
	readonly RG8_SNORM: number;
	readonly RGB8_SNORM: number;
	readonly RGBA8_SNORM: number;
	readonly RGB10_A2UI: number;
	readonly TEXTURE_IMMUTABLE_FORMAT: number;
	readonly TEXTURE_IMMUTABLE_LEVELS: number;
	readonly UNSIGNED_INT_2_10_10_10_REV: number;
	readonly UNSIGNED_INT_10F_11F_11F_REV: number;
	readonly UNSIGNED_INT_5_9_9_9_REV: number;
	readonly FLOAT_32_UNSIGNED_INT_24_8_REV: number;
	readonly UNSIGNED_INT_24_8: number;
	readonly HALF_FLOAT: number;
	readonly RG: number;
	readonly RG_INTEGER: number;
	readonly INT_2_10_10_10_REV: number;
	readonly QUERY_RESULT_AVAILABLE: number;
	readonly QUERY_RESULT: number;
	readonly CURRENT_QUERY: number;
	readonly ANY_SAMPLES_PASSED: number;
	readonly ANY_SAMPLES_PASSED_CONSERVATIVE: number;
	readonly MAX_DRAW_BUFFERS: number;
	readonly DRAW_BUFFER0: number;
	readonly DRAW_BUFFER1: number;
	readonly DRAW_BUFFER2: number;
	readonly DRAW_BUFFER3: number;
	readonly DRAW_BUFFER4: number;
	readonly DRAW_BUFFER5: number;
	readonly DRAW_BUFFER6: number;
	readonly DRAW_BUFFER7: number;
	readonly DRAW_BUFFER8: number;
	readonly DRAW_BUFFER9: number;
	readonly DRAW_BUFFER10: number;
	readonly DRAW_BUFFER11: number;
	readonly DRAW_BUFFER12: number;
	readonly DRAW_BUFFER13: number;
	readonly DRAW_BUFFER14: number;
	readonly DRAW_BUFFER15: number;
	readonly MAX_COLOR_ATTACHMENTS: number;
	readonly COLOR_ATTACHMENT1: number;
	readonly COLOR_ATTACHMENT2: number;
	readonly COLOR_ATTACHMENT3: number;
	readonly COLOR_ATTACHMENT4: number;
	readonly COLOR_ATTACHMENT5: number;
	readonly COLOR_ATTACHMENT6: number;
	readonly COLOR_ATTACHMENT7: number;
	readonly COLOR_ATTACHMENT8: number;
	readonly COLOR_ATTACHMENT9: number;
	readonly COLOR_ATTACHMENT10: number;
	readonly COLOR_ATTACHMENT11: number;
	readonly COLOR_ATTACHMENT12: number;
	readonly COLOR_ATTACHMENT13: number;
	readonly COLOR_ATTACHMENT14: number;
	readonly COLOR_ATTACHMENT15: number;
	readonly SAMPLER_3D: number;
	readonly SAMPLER_2D_SHADOW: number;
	readonly SAMPLER_2D_ARRAY: number;
	readonly SAMPLER_2D_ARRAY_SHADOW: number;
	readonly SAMPLER_CUBE_SHADOW: number;
	readonly INT_SAMPLER_2D: number;
	readonly INT_SAMPLER_3D: number;
	readonly INT_SAMPLER_CUBE: number;
	readonly INT_SAMPLER_2D_ARRAY: number;
	readonly UNSIGNED_INT_SAMPLER_2D: number;
	readonly UNSIGNED_INT_SAMPLER_3D: number;
	readonly UNSIGNED_INT_SAMPLER_CUBE: number;
	readonly UNSIGNED_INT_SAMPLER_2D_ARRAY: number;
	readonly MAX_SAMPLES: number;
	readonly SAMPLER_BINDING: number;
	readonly PIXEL_PACK_BUFFER: number;
	readonly PIXEL_UNPACK_BUFFER: number;
	readonly PIXEL_PACK_BUFFER_BINDING: number;
	readonly PIXEL_UNPACK_BUFFER_BINDING: number;
	readonly COPY_READ_BUFFER: number;
	readonly COPY_WRITE_BUFFER: number;
	readonly COPY_READ_BUFFER_BINDING: number;
	readonly COPY_WRITE_BUFFER_BINDING: number;
	readonly FLOAT_MAT2x3: number;
	readonly FLOAT_MAT2x4: number;
	readonly FLOAT_MAT3x2: number;
	readonly FLOAT_MAT3x4: number;
	readonly FLOAT_MAT4x2: number;
	readonly FLOAT_MAT4x3: number;
	readonly UNSIGNED_INT_VEC2: number;
	readonly UNSIGNED_INT_VEC3: number;
	readonly UNSIGNED_INT_VEC4: number;
	readonly UNSIGNED_NORMALIZED: number;
	readonly SIGNED_NORMALIZED: number;
	readonly VERTEX_ATTRIB_ARRAY_INTEGER: number;
	readonly VERTEX_ATTRIB_ARRAY_DIVISOR: number;
	readonly TRANSFORM_FEEDBACK_BUFFER_MODE: number;
	readonly MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: number;
	readonly TRANSFORM_FEEDBACK_VARYINGS: number;
	readonly TRANSFORM_FEEDBACK_BUFFER_START: number;
	readonly TRANSFORM_FEEDBACK_BUFFER_SIZE: number;
	readonly TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: number;
	readonly MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: number;
	readonly MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: number;
	readonly INTERLEAVED_ATTRIBS: number;
	readonly SEPARATE_ATTRIBS: number;
	readonly TRANSFORM_FEEDBACK_BUFFER: number;
	readonly TRANSFORM_FEEDBACK_BUFFER_BINDING: number;
	readonly TRANSFORM_FEEDBACK: number;
	readonly TRANSFORM_FEEDBACK_PAUSED: number;
	readonly TRANSFORM_FEEDBACK_ACTIVE: number;
	readonly TRANSFORM_FEEDBACK_BINDING: number;
	readonly FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: number;
	readonly FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: number;
	readonly FRAMEBUFFER_ATTACHMENT_RED_SIZE: number;
	readonly FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: number;
	readonly FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: number;
	readonly FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: number;
	readonly FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: number;
	readonly FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: number;
	readonly FRAMEBUFFER_DEFAULT: number;
	readonly DEPTH_STENCIL_ATTACHMENT: number;
	readonly DEPTH_STENCIL: number;
	readonly DEPTH24_STENCIL8: number;
	readonly DRAW_FRAMEBUFFER_BINDING: number;
	readonly READ_FRAMEBUFFER: number;
	readonly DRAW_FRAMEBUFFER: number;
	readonly READ_FRAMEBUFFER_BINDING: number;
	readonly RENDERBUFFER_SAMPLES: number;
	readonly FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: number;
	readonly FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: number;
	readonly UNIFORM_BUFFER: number;
	readonly UNIFORM_BUFFER_BINDING: number;
	readonly UNIFORM_BUFFER_START: number;
	readonly UNIFORM_BUFFER_SIZE: number;
	readonly MAX_VERTEX_UNIFORM_BLOCKS: number;
	readonly MAX_FRAGMENT_UNIFORM_BLOCKS: number;
	readonly MAX_COMBINED_UNIFORM_BLOCKS: number;
	readonly MAX_UNIFORM_BUFFER_BINDINGS: number;
	readonly MAX_UNIFORM_BLOCK_SIZE: number;
	readonly MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: number;
	readonly MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: number;
	readonly UNIFORM_BUFFER_OFFSET_ALIGNMENT: number;
	readonly ACTIVE_UNIFORM_BLOCKS: number;
	readonly UNIFORM_TYPE: number;
	readonly UNIFORM_SIZE: number;
	readonly UNIFORM_BLOCK_INDEX: number;
	readonly UNIFORM_OFFSET: number;
	readonly UNIFORM_ARRAY_STRIDE: number;
	readonly UNIFORM_MATRIX_STRIDE: number;
	readonly UNIFORM_IS_ROW_MAJOR: number;
	readonly UNIFORM_BLOCK_BINDING: number;
	readonly UNIFORM_BLOCK_DATA_SIZE: number;
	readonly UNIFORM_BLOCK_ACTIVE_UNIFORMS: number;
	readonly UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: number;
	readonly UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: number;
	readonly UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: number;
	readonly OBJECT_TYPE: number;
	readonly SYNC_CONDITION: number;
	readonly SYNC_STATUS: number;
	readonly SYNC_FLAGS: number;
	readonly SYNC_FENCE: number;
	readonly SYNC_GPU_COMMANDS_COMPLETE: number;
	readonly UNSIGNALED: number;
	readonly SIGNALED: number;
	readonly ALREADY_SIGNALED: number;
	readonly TIMEOUT_EXPIRED: number;
	readonly CONDITION_SATISFIED: number;
	readonly WAIT_FAILED: number;
	readonly SYNC_FLUSH_COMMANDS_BIT: number;
	readonly COLOR: number;
	readonly DEPTH: number;
	readonly STENCIL: number;
	readonly MIN: number;
	readonly MAX: number;
	readonly DEPTH_COMPONENT24: number;
	readonly STREAM_READ: number;
	readonly STREAM_COPY: number;
	readonly STATIC_READ: number;
	readonly STATIC_COPY: number;
	readonly DYNAMIC_READ: number;
	readonly DYNAMIC_COPY: number;
	readonly DEPTH_COMPONENT32F: number;
	readonly DEPTH32F_STENCIL8: number;
	readonly INVALID_INDEX: number;
	readonly TIMEOUT_IGNORED: number;
	readonly MAX_CLIENT_WAIT_TIMEOUT_WEBGL: number;

	constructor(context: any);

	beginQuery(target: number, query: WebGLQuery): void;

	beginTransformFeedback(primitiveMode: number): void;

	bindBufferBase(target: number, index: number, buffer: WebGLBuffer): void;

	bindBufferRange(target: number, index: number, buffer: WebGLBuffer, offset: number, size: number): void;

	bindSampler(unit: number, sampler: WebGLSampler): void;

	bindTransformFeedback(target: number, transformFeedback: WebGLTransformFeedback): void;

	bindVertexArray(vertexArray: WebGLVertexArrayObject): void;

	blitFramebuffer(srcX0: number, srcY0: number, srcX1: number, srcY1: number, dstX0: number, dstY0: number, dstX1: number, dstY1: number, mask: number, filter: number): void;

	clearBufferfi(buffer: WebGLBuffer, drawbuffer: number, depth: number, stencil: number): void;

	clearBufferfv(buffer: WebGLBuffer, drawbuffer: number, values: number[] | Float32Array): void;

	clearBufferiv(buffer: WebGLBuffer, drawbuffer: number, values: number[] | Int32Array): void;

	clearBufferuiv(buffer: WebGLBuffer, drawbuffer: number, values: number[] | Uint32Array): void;

	clientWaitSync(sync: WebGLSync, flags: number, timeout: number): number;

	compressedTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, imageSize: number, offset: any): any;

	copyBufferSubData(readTarget: number, writeTarget: number, readOffset: number, writeOffset: number, size: number): void;

	copyTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, x: number, y: number, width: number, height: number): void;

	createQuery(): WebGLQuery;

	createSampler(): WebGLSampler;

	createTransformFeedback(): WebGLTransformFeedback;

	createVertexArray(): WebGLVertexArrayObject;

	deleteQueryWithQuery(query: WebGLQuery): void;

	deleteSamplerWithSampler(sampler: WebGLSampler): void;

	deleteSyncWithSync(sync: WebGLSync): void;

	deleteTransformFeedback(transformFeedback: WebGLTransformFeedback): void;

	deleteVertexArrayWithVertexArray(vertexArray: WebGLVertexArrayObject): void;

	drawArraysInstanced(mode: number, first: number, count: number, instanceCount: number): void;

	drawBuffers(buffers: number[]): void;

	drawElementsInstanced(mode: number, count: number, type: number, offset: number, instanceCount: number): void;

	drawRangeElements(mode: number, start: number, end: number, count: number, type: number, offset: number): void;

	endQuery(target: number): void;

	endTransformFeedback(): void;

	fenceSync(condition: number, flags: number): void;

	framebufferTextureLayer(target: number, attachment: number, texture: WebGLTexture, level: number, layer: number): void;

	getActiveUniformBlockName(program: WebGLProgram, uniformBlockIndex: number): string;

	getActiveUniformBlockParameter(program: WebGLProgram, uniformBlockIndex: number, pname: number): any;

	getActiveUniforms(program: WebGLProgram, uniformIndices: number[], pname: number): any;

	getBufferSubData(target: number, srcByteOffset: number, dstData: ArrayBuffer, dstOffset?: number, length?: number): void;

	getFragDataLocation(program: WebGLProgram, name: string): number;

	getIndexedParameter(target: number, index: number): any;

	getInternalformatParameter(target: number, internalformat: number, pname: number): any;

	getQueryParameter(query: WebGLQuery, pname: number): any;

	getQuery(target: number, pname: number): any;

	getSamplerParameter(sampler: WebGLSampler, pname: number): any;

	getSyncParameter(sync: WebGLSync, pname: number): any;

	getTransformFeedbackVarying(program: WebGLProgram, index: number): any;

	getUniformBlockIndex(program: WebGLProgram, uniformBlockName: string): number;

	getUniformIndices(program: WebGLProgram, uniformNames: string[]): number[];

	invalidateFramebuffer(target: number, attachments: number[]): void;

	invalidateSubFramebuffer(target: number, attachments: number[], x: number, y: number, width: number, height: number): void;

	isQuery(query: WebGLQuery): boolean;

	isSampler(sampler: WebGLSampler): boolean;

	isSync(sync: WebGLSync): boolean;

	isTransformFeedback(transformFeedback: WebGLTransformFeedback): boolean;

	isVertexArray(vertexArray: WebGLVertexArrayObject): boolean;

	pauseTransformFeedback(): void;

	readBuffer(src: number): void;

	renderbufferStorageMultisample(target: number, samples: number, internalFormat: number, width: number, height: number): void;

	resumeTransformFeedback(): void;

	samplerParameterf(sampler: WebGLSampler, pname: number, param: number): void;

	samplerParameteri(sampler: WebGLSampler, pname: number, param: number): void;

	texImage3D(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, offset: any): any;

	texStorage2D(target: number, levels: number, internalformat: number, width: number, height: number): void;

	texStorage3D(target: number, levels: number, internalformat: number, width: number, height: number, depth: number): void;

	texSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, offset: any): any;

	texSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: any): any;

	transformFeedbackVaryings(program: WebGLProgram, varyings: string[], bufferMode: number): void;

	uniform1ui(location: number, v0: number): void;

	uniform1uiv(location: number, data: Uint32Array): void;

	uniform2ui(location: number, v0: number, v1: number): void;

	uniform2uiv(location: number, data: Uint32Array): void;

	uniform3ui(location: number, v0: number, v1: number, v2: number): void;

	uniform3uiv(location: number, data: Uint32Array): void;

	uniform4ui(location: number, v0: number, v1: number, v2: number, v3: number): void;

	uniform4uiv(location: number, data: Uint32Array): void;

	uniformBlockBinding(program: WebGLProgram, uniformBlockIndex: number, uniformBlockBinding: number): void;

	uniformMatrix2x3fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void;

	uniformMatrix2x4fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void;

	uniformMatrix3x2fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void;

	uniformMatrix3x4fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void;

	uniformMatrix4x2fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void;

	uniformMatrix4x3fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void;

	vertexAttribDivisor(index: number, divisor: number): void;

	vertexAttribI4i(index: number, v0: number, v1: number, v2: number, v3: number): void;

	vertexAttribI4iv(index: number, value: number[] | Int32Array): void;

	vertexAttribI4ui(index: number, v0: number, v1: number, v2: number, v3: number): void;

	vertexAttribI4uiv(index: number, value: number[] | Uint32Array): void;
}
