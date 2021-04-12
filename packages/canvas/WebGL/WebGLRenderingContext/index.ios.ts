import { WebGLRenderingContextBase } from './common';

import { WebGLShader } from '../WebGLShader';
import { WebGLFramebuffer } from '../WebGLFramebuffer';
import { WebGLTexture } from '../WebGLTexture';
import { WebGLProgram } from '../WebGLProgram';
import { WebGLUniformLocation } from '../WebGLUniformLocation';
import { WebGLActiveInfo } from '../WebGLActiveInfo';
import { WebGLRenderbuffer } from '../WebGLRenderbuffer';
import { WebGLShaderPrecisionFormat } from '../WebGLShaderPrecisionFormat';
import { WebGLBuffer } from '../WebGLBuffer';
import { ImageSource } from '@nativescript/core';
import { ANGLE_instanced_arrays, EXT_blend_minmax, EXT_color_buffer_float, EXT_color_buffer_half_float, EXT_shader_texture_lod, EXT_sRGB, EXT_texture_filter_anisotropic, OES_element_index_uint, OES_fbo_render_mipmap, OES_standard_derivatives, OES_texture_float, OES_texture_float_linear, OES_texture_half_float, OES_texture_half_float_linear, OES_vertex_array_object, WEBGL_color_buffer_float, WEBGL_compressed_texture_etc, WEBGL_compressed_texture_etc1, WEBGL_compressed_texture_pvrtc, WEBGL_depth_texture, WEBGL_draw_buffers, WEBGL_lose_context } from '../WebGLExtensions';
import { ImageAsset } from '../../ImageAsset';
import { Canvas } from '../../Canvas';
import { Utils } from '../../utils';
import { ImageBitmap } from '../../ImageBitmap';

declare const TNS_EXT_blend_minmax,
	TNS_EXT_color_buffer_float,
	TNS_EXT_color_buffer_half_float,
	TNS_EXT_sRGB,
	TNS_EXT_shader_texture_lod,
	TNS_EXT_texture_filter_anisotropic,
	TNS_OES_element_index_uint,
	TNS_OES_fbo_render_mipmap,
	TNS_OES_standard_derivatives,
	TNS_OES_texture_float,
	TNS_OES_texture_float_linear,
	TNS_OES_texture_half_float,
	TNS_OES_texture_half_float_linear,
	TNS_OES_vertex_array_object,
	TNS_WEBGL_color_buffer_float,
	TNS_WEBGL_compressed_texture_etc,
	TNS_WEBGL_compressed_texture_etc1,
	TNS_WEBGL_compressed_texture_pvrtc,
	TNS_WEBGL_depth_texture,
	TNS_WEBGL_lose_context,
	TNS_ANGLE_instanced_arrays,
	TNS_WEBGL_draw_buffers;

export class WebGLRenderingContext extends WebGLRenderingContextBase {
	public static isDebug = false;
	public static filter: 'both' | 'error' | 'args' = 'both';
	private context//: TNSWebGLRenderingContext;

	constructor(context) {
		super(context);
		this.context = context;
	}

	get native() {
		return this.context;
	}

	get drawingBufferHeight() {
		return this.context.drawingBufferHeight;
	}

	get drawingBufferWidth() {
		return this.context.drawingBufferWidth;
	}

	activeTexture(texture: number): void {
		this._glCheckError('activeTexture');
		this._checkArgs('activeTexture', arguments);
		this.context.activeTexture(texture);
	}

	attachShader(program: WebGLProgram, shader: WebGLShader): void {
		const value = program ? program.native : 0;
		const value2 = shader ? shader.native : 0;
		this._glCheckError('attachShader');
		this._checkArgs('attachShader', arguments);
		this.context.attachShader(value, value2);
	}

	bindAttribLocation(program: WebGLProgram, index: number, name: string): void {
		this._glCheckError('bindAttribLocation');
		this._checkArgs('bindAttribLocation', arguments);
		const value = program ? program.native : 0;
		this.context.bindAttribLocation(value, index, name);
	}

	bindBuffer(target: number, buffer: WebGLBuffer): void {
		this._glCheckError('bindBuffer');
		this._checkArgs('bindBuffer', arguments);
		const value = buffer ? buffer.native : 0;
		this.context.bindBuffer(target, value);
	}

	bindFramebuffer(target: number, framebuffer: WebGLFramebuffer): void {
		this._glCheckError('bindFramebuffer');
		this._checkArgs('bindFramebuffer', arguments);
		const value = framebuffer ? framebuffer.native : 0;
		this.context.bindFramebuffer(target, value);
	}

	bindRenderbuffer(target: number, renderbuffer: WebGLRenderbuffer): void {
		this._glCheckError('bindRenderbuffer');
		this._checkArgs('bindRenderbuffer', arguments);
		const value = renderbuffer ? renderbuffer.native : 0;
		this.context.bindRenderbuffer(target, value);
	}

	bindTexture(target: number, texture: WebGLTexture): void {
		this._glCheckError('bindTexture');
		this._checkArgs('bindTexture', arguments);
		const value = texture ? texture.native : 0;
		this.context.bindTexture(target, value);
	}

	blendColor(red: number, green: number, blue: number, alpha: number): void {
		this._glCheckError('blendColor');
		this._checkArgs('blendColor', arguments);
		this.context.blendColor(red, green, blue, alpha);
	}

	blendEquationSeparate(modeRGB: number, modeAlpha: number): void {
		this._glCheckError('blendEquationSeparate');
		this._checkArgs('blendEquationSeparate', arguments);
		this.context.blendEquationSeparate(modeRGB, modeAlpha);
	}

	blendEquation(mode: number): void {
		this._glCheckError('blendEquation');
		this._checkArgs('blendEquation', arguments);
		this.context.blendEquation(mode);
	}

	blendFuncSeparate(srcRGB: number = this.ONE, dstRGB: number = this.ZERO, srcAlpha: number = this.ONE, dstAlpha: number = this.ZERO): void {
		this._glCheckError('blendFuncSeparate');
		this._checkArgs('blendFuncSeparate', arguments);
		this.context.blendFuncSeparate(srcRGB, dstRGB, srcAlpha, dstAlpha);
	}

	blendFunc(sfactor: number = this.ONE, dfactor: number = this.ZERO): void {
		this._glCheckError('blendFunc');
		this._checkArgs('blendFunc', arguments);
		this.context.blendFunc(sfactor, dfactor);
	}

	bufferData(target: number, size: number, usage: number): void;

	bufferData(target: number, srcData: ArrayBuffer | ArrayBufferView, usage: number): void;

	bufferData(target: any, srcData: any, usage: any) {
		this._glCheckError('bufferData');
		this._checkArgs('bufferData', arguments);
		if (typeof srcData === 'number') {
			this.context.bufferDataSize(target, srcData, usage);
		} else if (srcData instanceof ArrayBuffer) {
			// Array.from(srcData as any) bug
			const array = new Uint8Array(srcData);
			const data = Array.from(array);
			this.context.bufferDataU8(target, data, usage);
		} else if (srcData && srcData.buffer instanceof ArrayBuffer) {
			if (srcData instanceof Int8Array) {
				this.context.bufferDataI8(target, Array.from(srcData as any), usage);
			} else if (srcData instanceof Uint8Array || srcData instanceof Uint8ClampedArray) {
				this.context.bufferDataU8(target, Array.from(srcData as any), usage);
			} else if (srcData instanceof Int16Array) {
				this.context.bufferDataI16(target, Array.from(srcData as any), usage);
			} else if (srcData instanceof Uint16Array) {
				this.context.bufferDataU16(target, Array.from(srcData as any), usage);
			} else if (srcData instanceof Int32Array) {
				this.context.bufferDataI32(target, Array.from(srcData as any), usage);
			} else if (srcData instanceof Uint32Array) {
				this.context.bufferDataU32(target, Array.from(srcData as any), usage);
			} else if (srcData instanceof Float32Array) {
				this.context.bufferDataF32(target, Array.from(srcData as any), usage);
			} else if (srcData instanceof Float64Array) {
				this.context.bufferDataF64(target, Array.from(srcData as any), usage);
			}
		} else {
			this.context.bufferDataSrcData(target, srcData, usage);
		}
	}

	bufferSubData(target: number, offset: number, srcData: ArrayBuffer | ArrayBufferView): void {
		this._glCheckError('bufferSubData');
		this._checkArgs('bufferSubData', arguments);

		if (srcData instanceof Int8Array) {
			this.context.bufferSubDataI8(target, offset, Array.from(srcData as any));
		} else if (srcData instanceof ArrayBuffer) {
			const array = new Uint8Array(srcData);
			this.context.bufferSubDataU8(target, offset, Array.from(array));
		} else if (srcData instanceof Uint8Array || srcData instanceof Uint8ClampedArray) {
			this.context.bufferSubDataU8(target, offset, Array.from(srcData as any));
		} else if (srcData instanceof Int16Array) {
			this.context.bufferSubDataI16(target, offset, Array.from(srcData as any));
		} else if (srcData instanceof Uint16Array) {
			this.context.bufferSubDataU16(target, offset, Array.from(srcData as any));
		} else if (srcData instanceof Int32Array) {
			this.context.bufferSubDataI32(target, offset, Array.from(srcData as any));
		} else if (srcData instanceof Uint32Array) {
			this.context.bufferSubDataU32(target, offset, Array.from(srcData as any));
		} else if (srcData instanceof Float32Array) {
			this.context.bufferSubDataF32(target, offset, Array.from(srcData as any));
		} else if (srcData instanceof Float64Array) {
			this.context.bufferSubDataF64(target, offset, Array.from(srcData as any));
		}
	}

	checkFramebufferStatus(target: number): number {
		this._glCheckError('checkFramebufferStatus');
		this._checkArgs('checkFramebufferStatus', arguments);
		return this.context.checkFramebufferStatus(target);
	}

	clearColor(red: number, green: number, blue: number, alpha: number): void {
		this._glCheckError('clearColor');
		this._checkArgs('clearColor', arguments);
		this.context.clearColor(red, green, blue, alpha);
	}

	clearDepth(depth: number): void {
		this._glCheckError('clearDepth');
		this._checkArgs('clearDepth', arguments);
		this.context.clearDepth(depth);
	}

	clearStencil(stencil: number): void {
		this._glCheckError('clearStencil');
		this._checkArgs('clearStencil', arguments);
		this.context.clearStencil(stencil);
	}

	clear(mask: number): void {
		this._glCheckError('clear');
		this._checkArgs('clear', arguments);
		this.context.clear(mask);
		this._glCheckError('after clear');
	}

	colorMask(red: boolean, green: boolean, blue: boolean, alpha: boolean): void {
		this._glCheckError('colorMask');
		this._checkArgs('colorMask', arguments);
		this.context.colorMask(red, green, blue, alpha);
	}

	commit(): void {
		// NOOP
		this.context.commit();
	}

	compileShader(shader: WebGLShader): void {
		this._glCheckError('compileShader');
		this._checkArgs('compileShader', arguments);
		const value = shader ? shader.native : 0;
		this.context.compileShader(value);
	}

	compressedTexImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, pixels?: ArrayBufferView): void {
		this._glCheckError('compressedTexImage2D');
		this._checkArgs('compressedTexImage2D', arguments);
		if ((pixels && pixels.buffer instanceof ArrayBuffer) || pixels instanceof ArrayBuffer) {
			this.context.compressedTexImage2D(target, level, internalformat, width, height, border, NSData.dataWithData(pixels as any));
		} else {
			this.context.compressedTexImage2D(target, level, internalformat, width, height, border, pixels as any);
		}
	}

	compressedTexSubImage2D(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, pixels: ArrayBufferView): void {
		this._glCheckError('compressedTexSubImage2D');
		this._checkArgs('compressedTexSubImage2D', arguments);
		if ((pixels && pixels.buffer instanceof ArrayBuffer) || pixels instanceof ArrayBuffer) {
			this.context.compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, NSData.dataWithData(pixels as any));
		}
	}

	copyTexImage2D(target: number, level: number, internalformat: number, x: number, y: number, width: number, height: number, border: number): void {
		this._glCheckError('copyTexImage2D');
		this._checkArgs('copyTexImage2D', arguments);
		this.context.copyTexImage2D(target, level, internalformat, x, y, width, height, border);
	}

	copyTexSubImage2D(target: number, level: number, xoffset: number, yoffset: number, x: number, y: number, width: number, height: number): void {
		this._glCheckError('copyTexSubImage2D');
		this._checkArgs('copyTexSubImage2D', arguments);
		this.context.copyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height);
	}

	createBuffer(): WebGLBuffer {
		this._glCheckError('createBuffer');
		this._checkArgs('createBuffer', arguments);
		const id = this.context.createBuffer();
		return new WebGLBuffer(id);
	}

	createFramebuffer(): WebGLFramebuffer {
		this._glCheckError('createFramebuffer');
		this._checkArgs('createFramebuffer', arguments);
		return new WebGLFramebuffer(this.context.createFramebuffer());
	}

	createProgram(): WebGLProgram {
		this._glCheckError('createProgram');
		this._checkArgs('createProgram', arguments);
		return new WebGLProgram(this.context.createProgram());
	}

	createRenderbuffer(): WebGLRenderbuffer {
		this._glCheckError('createRenderbuffer');
		this._checkArgs('createRenderbuffer', arguments);
		return new WebGLRenderbuffer(this.context.createRenderbuffer());
	}

	createShader(type: number): WebGLShader {
		this._glCheckError('createShader');
		this._checkArgs('createShader', arguments);
		return new WebGLShader(this.context.createShader(type));
	}

	createTexture(): WebGLTexture {
		this._glCheckError('createTexture');
		this._checkArgs('createTexture', arguments);
		const id = this.context.createTexture();
		return new WebGLTexture(id);
	}

	cullFace(mode: number): void {
		this._glCheckError('cullFace');
		this._checkArgs('cullFace', arguments);
		this.context.cullFace(mode);
	}

	deleteBuffer(buffer: WebGLBuffer): void {
		this._glCheckError('deleteBuffer');
		this._checkArgs('deleteBuffer', arguments);
		const value = buffer ? buffer.native : 0;
		this.context.deleteBuffer(value);
	}

	deleteFramebuffer(frameBuffer: WebGLFramebuffer): void {
		this._glCheckError('deleteFramebuffer');
		this._checkArgs('deleteFramebuffer', arguments);
		const value = frameBuffer ? frameBuffer.native : 0;
		this.context.deleteFramebuffer(value);
	}

	deleteProgram(program: WebGLProgram): void {
		this._glCheckError('deleteProgram');
		this._checkArgs('deleteProgram', arguments);
		const value = program ? program.native : 0;
		this.context.deleteProgram(value);
	}

	deleteRenderbuffer(renderBuffer: WebGLRenderbuffer): void {
		this._glCheckError('deleteRenderbuffer');
		this._checkArgs('deleteRenderbuffer', arguments);
		const value = renderBuffer ? renderBuffer.native : 0;
		this.context.deleteRenderbuffer(value);
	}

	deleteShader(shader: WebGLRenderbuffer): void {
		this._glCheckError('deleteShader');
		this._checkArgs('deleteShader', arguments);
		const value = shader ? shader.native : 0;
		this.context.deleteShader(value);
	}

	deleteTexture(texture: WebGLTexture): void {
		this._glCheckError('deleteTexture');
		this._checkArgs('deleteTexture', arguments);
		const value = texture ? texture.native : 0;
		this.context.deleteTexture(value);
	}

	depthFunc(func: number): void {
		this._glCheckError('depthFunc');
		this._checkArgs('depthFunc', arguments);
		this.context.depthFunc(func);
	}

	depthMask(flag: boolean): void {
		this._glCheckError('depthMask');
		this._checkArgs('depthMask', arguments);
		this.context.depthMask(flag);
	}

	depthRange(zNear: number, zFar: number): void {
		this._glCheckError('depthRange');
		this._checkArgs('depthRange', arguments);
		this.context.depthRange(zNear, zFar);
	}

	detachShader(program: WebGLProgram, shader: WebGLShader): void {
		this._glCheckError('detachShader');
		this._checkArgs('detachShader', arguments);
		const value = program ? program.native : 0;
		const value2 = shader ? shader.native : 0;
		this.context.detachShader(value, value2);
	}

	disableVertexAttribArray(index: number): void {
		this._checkArgs('disableVertexAttribArray', arguments);
		this._glCheckError('disableVertexAttribArray');
		this.context.disableVertexAttribArray(index);
	}

	disable(cap: number): void {
		this._glCheckError('disable');
		this._checkArgs('disable', arguments);
		this.context.disable(cap);
	}

	drawArrays(mode: number, first: number, count: number): void {
		this._glCheckError('drawArrays');
		this._checkArgs('drawArrays', arguments);
		this.context.drawArrays(mode, first, count);
	}

	drawElements(mode: number, count: number, type: number, offset: number): void {
		this._glCheckError('drawElements');
		this._checkArgs('drawElements', arguments);
		this.context.drawElements(mode, count, type, offset);
	}

	enableVertexAttribArray(index: number): void {
		this._glCheckError('enableVertexAttribArray');
		this._checkArgs('enableVertexAttribArray', arguments);
		this.context.enableVertexAttribArray(index);
	}

	enable(cap: number): void {
		this._glCheckError('enable');
		this._checkArgs('enable', arguments);
		this.context.enable(cap);
	}

	finish(): void {
		this._glCheckError('finish');
		this._checkArgs('finish', arguments);
		this.context.finish();
	}

	flush(): void {
		this._glCheckError('flush');
		this._checkArgs('flush', arguments);
		this.context.flush();
	}

	framebufferRenderbuffer(target: number, attachment: number, renderbuffertarget: number, renderbuffer: WebGLRenderbuffer): void {
		this._glCheckError('framebufferRenderbuffer');
		this._checkArgs('framebufferRenderbuffer', arguments);
		const value = renderbuffer ? renderbuffer.native : 0;
		this.context.framebufferRenderbuffer(target, attachment, renderbuffertarget, value);
	}

	framebufferTexture2D(target: number, attachment: number, textarget: number, texture: WebGLTexture, level: number): void {
		this._glCheckError('framebufferTexture2D');
		this._checkArgs('framebufferTexture2D', arguments);
		const value = texture ? texture.native : 0;
		this.context.framebufferTexture2D(target, attachment, textarget, value, level);
	}

	frontFace(mode: number): void {
		this._glCheckError('frontFace');
		this._checkArgs('frontFace', arguments);
		this.context.frontFace(mode);
	}

	generateMipmap(target: number): void {
		this._glCheckError('generateMipmap');
		this._checkArgs('generateMipmap', arguments);
		this.context.generateMipmap(target);
	}

	getActiveAttrib(program: WebGLProgram, index: number): WebGLActiveInfo {
		this._glCheckError('getActiveAttrib');
		this._checkArgs('getActiveAttrib', arguments);
		const value = program ? program.native : 0;
		const attrib = this.context.getActiveAttrib(value, index);
		return new WebGLActiveInfo(attrib.name, attrib.size, attrib.type);
	}

	getActiveUniform(program: WebGLProgram, index: number): WebGLActiveInfo {
		this._glCheckError('getActiveUniform');
		this._checkArgs('getActiveUniform', arguments);
		const value = program ? program.native : 0;
		const uniform = this.context.getActiveUniform(value, index);
		return new WebGLActiveInfo(uniform.name, uniform.size, uniform.type);
	}

	getAttachedShaders(program: WebGLProgram): WebGLShader[] {
		this._glCheckError('getAttachedShaders');
		this._checkArgs('getAttachedShaders', arguments);
		const value = program ? program.native : 0;
		const shaders = this.context.getAttachedShaders(value);
		const length = shaders.count;
		const attachedShaders = [];
		for (let i = 0; i < length; i++) {
			attachedShaders.push(new WebGLShader(shaders.objectAtIndex(i)));
		}
		return attachedShaders;
	}

	getAttribLocation(program: WebGLProgram, name: string): number {
		this._glCheckError('getAttribLocation');
		this._checkArgs('getAttribLocation', arguments);
		const value = program ? program.native : 0;
		return this.context.getAttribLocation(value, name);
	}

	getBufferParameter(target: number, pname: number): number {
		this._glCheckError('getBufferParameter');
		this._checkArgs('getBufferParameter', arguments);
		return this.context.getBufferParameter(target, pname);
	}

	getContextAttributes(): any {
		this._glCheckError('getContextAttributes');
		this._checkArgs('getContextAttributes', arguments);
		const attributes = this.context.getContextAttributes();
		if (!attributes) {
			return null;
		}
		const keys = attributes.allKeys;
		const length = keys.count;
		const contextAttributes = {};
		for (let i = 0; i < length; i++) {
			const key = keys.objectAtIndex(i);
			contextAttributes[key] = attributes.objectForKey(key);
		}
		contextAttributes['stencil'] = true;
		return contextAttributes;
	}

	getError(): number {
		return this.context.getError();
	}

	getExtension(name: string) {
		this._checkArgs('getExtension', arguments);
		this._glCheckError('getExtension');
		const ext = this.context.getExtension(name);
		if (ext instanceof TNS_ANGLE_instanced_arrays) {
			return new ANGLE_instanced_arrays(ext);
		} else if (ext instanceof TNS_EXT_blend_minmax) {
			return new EXT_blend_minmax(ext);
		} else if (ext instanceof TNS_EXT_color_buffer_float) {
			return new EXT_color_buffer_float(ext);
		} else if (ext instanceof TNS_EXT_color_buffer_half_float) {
			return new EXT_color_buffer_half_float(ext);
		} else if (ext instanceof TNS_EXT_sRGB) {
			return new EXT_sRGB(ext);
		} else if (ext instanceof TNS_EXT_shader_texture_lod) {
			return new EXT_shader_texture_lod(ext);
		} else if (ext instanceof TNS_EXT_texture_filter_anisotropic) {
			return new EXT_texture_filter_anisotropic(ext);
		} else if (ext instanceof TNS_OES_element_index_uint) {
			return new OES_element_index_uint(ext);
		} else if (ext instanceof TNS_OES_fbo_render_mipmap) {
			return new OES_fbo_render_mipmap(ext);
		} else if (ext instanceof TNS_OES_standard_derivatives) {
			return new OES_standard_derivatives(ext);
		} else if (ext instanceof TNS_OES_texture_float) {
			return new OES_texture_float(ext);
		} else if (ext instanceof TNS_OES_texture_float_linear) {
			return new OES_texture_float_linear(ext);
		} else if (ext instanceof TNS_OES_texture_half_float) {
			return new OES_texture_half_float(ext);
		} else if (ext instanceof TNS_OES_texture_half_float_linear) {
			return new OES_texture_half_float_linear(ext);
		} else if (ext instanceof TNS_OES_vertex_array_object) {
			return new OES_vertex_array_object(ext);
		} else if (ext instanceof TNS_WEBGL_color_buffer_float) {
			return new WEBGL_color_buffer_float(ext);
		} else if (ext instanceof TNS_WEBGL_compressed_texture_etc) {
			return new WEBGL_compressed_texture_etc(ext);
		} else if (ext instanceof TNS_WEBGL_compressed_texture_etc1) {
			return new WEBGL_compressed_texture_etc1(ext);
		} else if (ext instanceof TNS_WEBGL_compressed_texture_pvrtc) {
			return new WEBGL_compressed_texture_pvrtc(ext);
		} else if (ext instanceof TNS_WEBGL_depth_texture) {
			return new WEBGL_depth_texture(ext);
		} else if (ext instanceof TNS_WEBGL_lose_context) {
			return new WEBGL_lose_context(ext);
		} else if (ext instanceof TNS_WEBGL_draw_buffers) {
			return new WEBGL_draw_buffers(ext);
		}
		return null;
	}

	getFramebufferAttachmentParameter(target: number, attachment: number, pname: number): number | WebGLRenderbuffer | WebGLTexture {
		this._glCheckError('getFramebufferAttachmentParameter');
		this._checkArgs('getFramebufferAttachmentParameter', arguments);
		const param = this.context.getFramebufferAttachmentParameter(target, attachment, pname);
		if (param.isRenderbuffer) {
			return new WebGLRenderbuffer(param.value);
		} else if (param.isTexture) {
			return new WebGLTexture(param.value);
		}
		return param.value;
	}

	getParameter(pname: number): number[] | number | WebGLBuffer | WebGLProgram | WebGLFramebuffer | WebGLRenderbuffer | WebGLTexture | Uint32Array | Int32Array | Float32Array | string | null {
		this._glCheckError('getParameter');
		this._checkArgs('getParameter', arguments);
		const value = this.context.getParameter(pname);
		switch (pname) {
			case this.COLOR_WRITEMASK:
			case this.COLOR_CLEAR_VALUE:
			case this.BLEND_COLOR:
			case this.ALIASED_LINE_WIDTH_RANGE:
			case this.ALIASED_POINT_SIZE_RANGE:
			case this.DEPTH_RANGE:
				return new Float32Array(Utils.toJSArray(value));
			case this.ARRAY_BUFFER_BINDING:
			case this.ELEMENT_ARRAY_BUFFER_BINDING:
				if(value){
					return new WebGLBuffer(value);
				}
				return null;
			case this.CURRENT_PROGRAM:
				if(value){
					return new WebGLProgram(value);
				}
				return null;
			case this.COMPRESSED_TEXTURE_FORMATS:
				return new Uint32Array(Utils.toJSArray(value));
			case this.RENDERBUFFER_BINDING:
				if(value){
					return new WebGLRenderbuffer(value);
				}
				return null;
			case this.FRAMEBUFFER_BINDING:
				if (value) {
					return new WebGLFramebuffer(value);
				}
				return null;
			case this.VIEWPORT:
			case this.SCISSOR_BOX:
			case this.MAX_VIEWPORT_DIMS:
				return new Int32Array(Utils.toJSArray(value));
			case this.TEXTURE_BINDING_CUBE_MAP:
			case this.TEXTURE_BINDING_2D:
				if (value) {
					return new WebGLTexture(value);
				}
				return null;
			case this.VERSION:
				let isMetal = false;
				if (this._canvas && this._canvas.useMetal) {
					isMetal = true;
				}
				if (this._type === 'webgl2') {
					return `WebGL 2.0 (${isMetal ? 'Metal' : 'OpenGL ES 3.0'} NativeScript)`;
				} else {
					return `WebGL 1.0 (${isMetal ? 'Metal' : 'OpenGL ES 3.0'} NativeScript)`;
				}
			default:
				return value;
		}
	}

	getProgramInfoLog(program: WebGLProgram): string {
		this._glCheckError('getProgramInfoLog');
		this._checkArgs('getProgramInfoLog', arguments);
		const value = program ? program.native : 0;
		return this.context.getProgramInfoLog(value);
	}

	getProgramParameter(program: WebGLProgram, pname: number): number | boolean {
		this._glCheckError('getProgramParameter');
		this._checkArgs('getProgramParameter', arguments);
		const value = program ? program.native : 0;
		return this.context.getProgramParameter(value, pname);
	}

	getRenderbufferParameter(target: number, pname: number): number {
		this._glCheckError('getRenderbufferParameter');
		this._checkArgs('getRenderbufferParameter', arguments);
		return this.context.getRenderbufferParameter(target, pname);
	}

	getShaderInfoLog(shader: WebGLShader): string {
		this._glCheckError('getShaderInfoLog');
		this._checkArgs('getShaderInfoLog', arguments);
		const value = shader ? shader.native : 0;
		return this.context.getShaderInfoLog(value);
	}

	getShaderParameter(shader: WebGLShader, pname: number): boolean | number {
		this._glCheckError('getShaderParameter');
		this._checkArgs('getShaderParameter', arguments);
		const value = shader ? shader.native : 0;
		return this.context.getShaderParameter(value, pname);
	}

	getShaderPrecisionFormat(shaderType: number, precisionType: number): WebGLShaderPrecisionFormat {
		this._glCheckError('getShaderPrecisionFormat');
		this._checkArgs('getShaderPrecisionFormat', arguments);
		const precision = this.context.getShaderPrecisionFormat(shaderType, precisionType);
		return new WebGLShaderPrecisionFormat(precision.rangeMin, precision.rangeMax, precision.precision);
	}

	getShaderSource(shader: WebGLShader): string {
		this._glCheckError('getShaderSource');
		this._checkArgs('getShaderSource', arguments);
		const value = shader ? shader.native : 0;
		return this.context.getShaderSource(value);
	}

	getSupportedExtensions(): string[] {
		this._glCheckError('getSupportedExtensions');
		this._checkArgs('getSupportedExtensions', arguments);
		const extensions = this.context.getSupportedExtensions();
		const array = [];
		const length = extensions.count;
		for (let i = 0; i < length; i++) {
			array.push(extensions.objectAtIndex(i));
		}
		return array;
	}

	getTexParameter(target: number, pname: number): number {
		this._glCheckError('getTexParameter');
		this._checkArgs('getTexParameter', arguments);
		return this.context.getTexParameter(target, pname);
	}

	getUniformLocation(program: WebGLProgram, name: string): WebGLUniformLocation {
		this._glCheckError('getUniformLocation');
		this._checkArgs('getUniformLocation', arguments);
		const value = program ? program.native : 0;
		const id = this.context.getUniformLocation(value, name);
		if (id === -1) {
			return null;
		}
		return new WebGLUniformLocation(id);
	}

	getUniform(program: WebGLProgram, location: WebGLUniformLocation): any {
		this._glCheckError('getUniform');
		this._checkArgs('getUniform', arguments);
		const value = program ? program.native : 0;
		const value2 = location ? location.native : 0;
		const type = interop.alloc(interop.sizeof(interop.types.uint32));
		glGetActiveUniform(value, value2, 0, null, null, type, null);
		const result = this.context.getUniform(value, value2);
		const ref = new interop.Reference(interop.types.uint32, type);
		switch (ref.value) {
			case this.FLOAT_VEC2:
			case this.FLOAT_VEC3:
			case this.FLOAT_VEC4:
			case this.FLOAT_MAT2:
			case this.FLOAT_MAT3:
			case this.FLOAT_MAT4:
				return new Float32Array(Utils.toJSArray(result));
			case this.INT_VEC2:
			case this.INT_VEC3:
			case this.INT_VEC4:
				return new Int32Array(Utils.toJSArray(result));
			default:
				break;
		}
		if (result instanceof NSArray || Array.isArray(result)) {
			return Utils.toJSArray(result);
		}

		return result;
	}

	getVertexAttribOffset(index: number, pname: number): number {
		this._glCheckError('getVertexAttribOffset');
		this._checkArgs('getVertexAttribOffset', arguments);
		return this.context.getVertexAttribOffset(index, pname);
	}

	getVertexAttrib(index: number, pname: number): number[] | boolean | number | Float32Array {
		this._glCheckError('getVertexAttrib');
		this._checkArgs('getVertexAttrib', arguments);
		const value = this.context.getVertexAttrib(index, pname);
		if (pname === this.CURRENT_VERTEX_ATTRIB) {
			return new Float32Array(Utils.toJSArray(value));
		}
		return value;
	}

	hint(target: number, mode: number): void {
		this._glCheckError('hint');
		this._checkArgs('hint', arguments);
		this.context.hint(target, mode);
	}

	isBuffer(buffer: WebGLBuffer): boolean {
		this._glCheckError('isBuffer');
		this._checkArgs('isBuffer', arguments);
		const value = buffer ? buffer.native : 0;
		return this.context.isBuffer(value);
	}

	isContextLost(): boolean {
		this._glCheckError('isContextLost');
		this._checkArgs('isContextLost', arguments);
		return this.context.isContextLost();
	}

	isEnabled(cap: number): boolean {
		this._glCheckError('isEnabled');
		this._checkArgs('isEnabled', arguments);
		return this.context.isEnabled(cap);
	}

	isFramebuffer(framebuffer: WebGLFramebuffer): boolean {
		this._glCheckError('isFramebuffer');
		this._checkArgs('isFramebuffer', arguments);
		const value = framebuffer ? framebuffer.native : 0;
		return this.context.isFramebuffer(value);
	}

	isProgram(program: WebGLProgram): boolean {
		this._glCheckError('isProgram');
		this._checkArgs('isProgram', arguments);
		const value = program ? program.native : 0;
		return this.context.isProgram(value);
	}

	isRenderbuffer(renderbuffer: WebGLRenderbuffer): boolean {
		this._glCheckError('isRenderbuffer');
		this._checkArgs('isRenderbuffer', arguments);
		const value = renderbuffer ? renderbuffer.native : 0;
		return this.context.isRenderbuffer(value);
	}

	isShader(shader: WebGLShader): boolean {
		this._glCheckError('isShader');
		this._checkArgs('isShader', arguments);
		const value = shader ? shader.native : 0;
		return this.context.isShader(value);
	}

	isTexture(texture: WebGLTexture): boolean {
		this._glCheckError('isTexture');
		this._checkArgs('isTexture', arguments);
		const value = texture ? texture.native : 0;
		return this.context.isTexture(value);
	}

	lineWidth(width: number): void {
		this._glCheckError('lineWidth');
		this._checkArgs('lineWidth', arguments);
		this.context.lineWidth(width);
	}

	linkProgram(program: WebGLProgram): void {
		this._glCheckError('linkProgram');
		this._checkArgs('linkProgram', arguments);
		const value = program ? program.native : 0;
		this.context.linkProgram(value);
	}

	pixelStorei(pname: number, param: any): void {
		this._glCheckError('pixelStorei');
		this._checkArgs('pixelStorei', arguments);
		// this.context.pixelStoreiWithPnameParam(pname, param);
		if (pname === this.UNPACK_FLIP_Y_WEBGL || pname === this.UNPACK_PREMULTIPLY_ALPHA_WEBGL) {
			this.context.pixelStorei(pname, !!param);
		} else if (pname === this.PACK_ALIGNMENT || pname === this.UNPACK_ALIGNMENT || pname === this.UNPACK_COLORSPACE_CONVERSION_WEBGL) {
			if (pname === this.UNPACK_COLORSPACE_CONVERSION_WEBGL) {
				param = 0x9244;
			} else if (pname === this.PACK_ALIGNMENT || pname === this.UNPACK_ALIGNMENT) {
				param = 4;
			}
			this.context.pixelStorei(pname, Number(param));
		} else {
			this.context.pixelStorei(pname, param);
		}
	}

	polygonOffset(factor: number, units: number): void {
		this._glCheckError('polygonOffset');
		this._checkArgs('polygonOffset', arguments);
		this.context.polygonOffset(factor, units);
	}

	readPixels(x: number, y: number, width: number, height: number, format: number, type: number, pixels: ArrayBufferView): void {
		this._glCheckError('readPixels');
		this._checkArgs('readPixels', arguments);
		if ((pixels && pixels.buffer instanceof ArrayBuffer) || pixels instanceof ArrayBuffer) {
			this.context.readPixels(x, y, width, height, format, type, NSData.dataWithData(pixels as any));
		}
	}

	renderbufferStorage(target: number, internalFormat: number, width: number, height: number): void {
		this._glCheckError('renderbufferStorage');
		this._checkArgs('renderbufferStorage', arguments);
		if (internalFormat === this.DEPTH_STENCIL) {
			// DEPTH24_STENCIL8 = 35056
			// DEPTH24_STENCIL8_OES = 0x88F0
			// 35056;
			internalFormat = 0x88f0;
		}
		this.context.renderbufferStorage(target, internalFormat, width, height);
	}

	sampleCoverage(value: number, invert: boolean): void {
		this._glCheckError('sampleCoverage');
		this._checkArgs('sampleCoverage', arguments);
		this.context.sampleCoverage(value, invert);
	}

	scissor(x: number, y: number, width: number, height: number): void {
		this._glCheckError('scissor');
		this._checkArgs('scissor', arguments);
		this.context.scissor(x, y, width, height);
	}

	shaderSource(shader: WebGLShader, source: string): void {
		this._glCheckError('shaderSource');
		this._checkArgs('shaderSource', arguments);
		const value = shader ? shader.native : 0;
		this.context.shaderSource(value, source);
	}

	stencilFuncSeparate(face: number, func: number, ref: number, mask: number): void {
		this._glCheckError('stencilFuncSeparate');
		this._checkArgs('stencilFuncSeparate', arguments);
		this.context.stencilFuncSeparate(face, func, ref, mask);
	}

	stencilFunc(func: number, ref: number, mask: number): void {
		this._glCheckError('stencilFunc');
		this._checkArgs('stencilFunc', arguments);
		this.context.stencilFunc(func, ref, mask);
	}

	stencilMaskSeparate(face: number, mask: number): void {
		this._glCheckError('stencilMaskSeparate');
		this._checkArgs('stencilMaskSeparate', arguments);
		this.context.stencilMaskSeparate(face, mask);
	}

	stencilMask(mask: number): void {
		this._glCheckError('stencilMask');
		this._checkArgs('stencilMask', arguments);
		this.context.stencilMask(mask);
	}

	stencilOpSeparate(face: number, fail: number, zfail: number, zpass: number): void {
		this._glCheckError('stencilOpSeparate');
		this._checkArgs('stencilOpSeparate', arguments);
		this.context.stencilOpSeparate(face, fail, zfail, zpass);
	}

	stencilOp(fail: number, zfail: number, zpass: number): void {
		this._glCheckError('stencilOp');
		this._checkArgs('stencilOp', arguments);
		this.context.stencilOp(fail, zfail, zpass);
	}

	texImage2D(target: number, level: number, internalformat: number, format: number, type: number, pixels?: any): void;
	texImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type: number, pixels?: ArrayBufferView): void;
	texImage2D(target: any, level: any, internalformat: any, width: any, height: any, border: any, format?: any, type?: any, pixels?: any) {
		this._glCheckError('texImage2D');
		this._checkArgs('texImage2D', arguments);
		/* TODO */
		// this.blendFunc(this.SRC_ALPHA, this.ONE_MINUS_SRC_ALPHA);
		// this.enable(this.BLEND);
		/* TODO */
		if (arguments.length === 9) {
			if ((pixels && pixels.buffer) || pixels instanceof ArrayBuffer) {
				if (pixels instanceof Uint8Array || pixels instanceof Uint8ClampedArray) {
					this.context.texImage2DU8(target, level, internalformat, width, height, border, format, type, Array.from(pixels as any));
				} else if (pixels instanceof Uint16Array) {
					this.context.texImage2DU16(target, level, internalformat, width, height, border, format, type, Array.from(pixels as any));
				} else if (pixels instanceof Uint32Array) {
					this.context.texImage2DU32(target, level, internalformat, width, height, border, format, type, Array.from(pixels as any));
				} else if (pixels instanceof Float32Array) {
					this.context.texImage2DF32(target, level, internalformat, width, height, border, format, type, Array.from(pixels as any));
				} else if (pixels instanceof ArrayBuffer) {
					const data = NSData.dataWithData(pixels as any);
					if (data) {
						this.context.texImage2DData(target, level, internalformat, width, height, border, format, type, data);
					} else {
						const array = new Uint8Array(pixels);
						this.context.texImage2DU8(target, level, internalformat, width, height, border, format, type, Array.from(array as any));
					}
				}
			} else {
				this.context.texImage2D(target, level, internalformat, width, height, border, format, type, pixels as any);
			}
		} else if (arguments.length === 6) {
			if (border && typeof border.tagName === 'string' && (border.tagName === 'VID' || border.tagName === 'VIDEO') && border._video && typeof border._video.getCurrentFrame === 'function') {
				border._video.getCurrentFrame(this.context,this, target, level, internalformat, width, height);
			} else if (border && typeof border.getCurrentFrame === 'function') {
				border.getCurrentFrame(this.context, this, target, level, internalformat, width, height);
			} else if (border instanceof ImageAsset) {
				this.context.texImage2DAsset(target, level, internalformat, width, height, border.native);
			} else if (border instanceof ImageBitmap) {
				this.context.texImage2DBitmap(target, level, internalformat, width, height, border.native);
			}  else if (border instanceof ImageSource) {
				this.context.texImage2DPixels(target, level, internalformat, width, height, border.ios);
			} else if (border instanceof UIImage) {
				this.context.texImage2DPixels(target, level, internalformat, width, height, border);
			} else if (border && typeof border.tagName === 'string' && (border.tagName === 'IMG' || border.tagName === 'IMAGE')) {
				if (border._asset instanceof ImageAsset) {
					this.context.texImage2DAsset(target, level, internalformat, width, height, border._asset.native);
				} else if (border._imageSource instanceof ImageSource) {
					this.context.texImage2DPixels(target, level, internalformat, width, height, border._imageSource.ios);
				} else if (border._image instanceof UIImage) {
					this.context.texImage2DPixels(target, level, internalformat, width, height, border._image);
				} else if (typeof border.src === 'string') {
					const asset = ImageSource.fromFileSync(border.src);
					this.context.texImage2DPixels(target, level, internalformat, width, height, asset.ios);
				}
			} else if (border && typeof border.tagName === 'string' && border.tagName === 'CANVAS' && border._canvas instanceof Canvas) {
				this.context.texImage2DCanvas(target, level, internalformat, width, height, border._canvas.ios);
			} else {
				this.context.texImage2D(target, level, internalformat, width, height, border as any);
			}
		}
		// this.blendFunc(this.SRC_ALPHA, this.ZERO);
		// this.disable(this.BLEND);
	}

	texParameterf(target: number, pname: number, param: number): void {
		this._glCheckError('texParameterf');
		this._checkArgs('texParameterf', arguments);
		this.context.texParameterf(target, pname, param);
	}

	texParameteri(target: number, pname: number, param: number): void {
		this._glCheckError('texParameteri');
		this._checkArgs('texParameteri', arguments);
		this.context.texParameteri(target, pname, param);
	}

	texSubImage2D(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, type: number, pixels: ArrayBufferView): void;
	texSubImage2D(target: number, level: number, xoffset: number, yoffset: number, format: number, type: number, pixels: any): void;
	texSubImage2D(target: any, level: any, xoffset: any, yoffset: any, width: any, height: any, format: any, type?: any, pixels?: any) {
		this._glCheckError('texSubImage2D');
		this._checkArgs('texSubImage2D', arguments);
		if (arguments.length === 9) {
			if ((pixels && pixels.buffer) || pixels instanceof ArrayBuffer) {
				if (pixels instanceof Uint8Array || pixels instanceof Uint8ClampedArray) {
					this.context.texSubImage2DU8(target, level, xoffset, yoffset, width, height, format, type, Array.from(pixels as any));
				} else if (pixels instanceof Uint16Array) {
					this.context.texSubImage2DU16(target, level, xoffset, yoffset, width, height, format, type, Array.from(pixels as any));
				} else if (pixels instanceof Float32Array) {
					this.context.texSubImage2DF32(target, level, xoffset, yoffset, width, height, format, type, Array.from(pixels as any));
				} else if (pixels instanceof ArrayBuffer) {
					const data = NSData.dataWithData(pixels as any);
					if (data) {
						this.context.texSubImage2DData(target, level, xoffset, yoffset, width, height, format, type, data);
					} else {
						const array = new Uint8Array(pixels);
						this.context.texSubImage2DU8(target, level, xoffset, yoffset, width, height, format, type, Array.from(array as any));
					}
				}
			} else {
				this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels as any);
			}
		} else if (arguments.length === 7) {
			if (format instanceof ImageAsset) {
				this.context.texSubImage2DAsset(target, level, xoffset, yoffset, width, height, format.native);
			}if (format instanceof ImageBitmap) {
				this.context.texSubImage2DBitmap(target, level, xoffset, yoffset, width, height, format.native);
			} else if (format instanceof UIImage) {
				this.context.texSubImage2DPixels(target, level, xoffset, yoffset, width, height, format);
			} else if (format instanceof ImageSource) {
				this.context.texSubImage2DPixels(target, level, xoffset, yoffset, width, height, format.ios);
			} else if (format && typeof format.tagName === 'string' && (format.tagName === 'IMG' || format.tagName === 'IMAGE')) {
				if (format._imageSource instanceof ImageSource) {
					this.context.texSubImage2DPixels(target, level, xoffset, yoffset, width, height, format._imageSource.ios);
				} else if (format._image instanceof UIImage) {
					this.context.texSubImage2DPixels(target, level, xoffset, yoffset, width, height, format._image);
				} else if (format._asset instanceof ImageAsset) {
					this.context.texSubImage2DAsset(target, level, xoffset, yoffset, width, height, format._asset.native);
				} else if (typeof format.src === 'string') {
					const result = ImageSource.fromFileSync(format.src);
					this.context.texSubImage2DPixels(target, level, xoffset, yoffset, width, height, result ? result.ios : null);
				}
			} else if (format && typeof format.tagName === 'string' && format.tagName === 'CANVAS' && format._canvas instanceof Canvas) {
				this.context.texSubImage2DCanvas(target, level, xoffset, yoffset, width, height, format._canvas.ios);
			} else {
				this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format as any);
			}
		}
	}

	uniform1f(location: WebGLUniformLocation, v0: number): void {
		this._glCheckError('uniform1f');
		this._checkArgs('uniform1f', arguments);
		this.context.uniform1f(location.native, v0);
	}

	uniform1iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform1iv');
		this._checkArgs('uniform1iv', arguments);
		if (!(value instanceof Int32Array)) {
			value = new Int32Array(value) as any;
		}
		this.context.uniform1iv(location.native, Array.from(value));
	}

	uniform1fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform1fv');
		this._checkArgs('uniform1fv', arguments);
		if (!(value instanceof Float32Array)) {
			value = new Float32Array(value) as any;
		}
		this.context.uniform1fv(location.native, Array.from(value));
	}

	uniform1i(location: WebGLUniformLocation, v0: number): void {
		this._glCheckError('uniform1i');
		this._checkArgs('uniform1i', arguments);
		this.context.uniform1i(location.native, v0);
	}

	uniform2f(location: WebGLUniformLocation, v0: number, v1: number): void {
		this._glCheckError('uniform2f');
		this._checkArgs('uniform2f', arguments);
		this.context.uniform2f(location.native, v0, v1);
	}

	uniform2iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform2iv');
		this._checkArgs('uniform2iv', arguments);
		if (!(value instanceof Int32Array)) {
			value = new Int32Array(value) as any;
		}
		this.context.uniform2iv(location.native, Array.from(value));
	}

	uniform2fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform2fv');
		this._checkArgs('uniform2fv', arguments);
		if (!(value instanceof Float32Array)) {
			value = new Float32Array(value) as any;
		}
		this.context.uniform2fv(location.native, Array.from(value));
	}

	uniform2i(location: WebGLUniformLocation, v0: number, v1: number): void {
		this._glCheckError('uniform2i');
		this._checkArgs('uniform2i', arguments);
		this.context.uniform2i(location.native, v0, v1);
	}

	uniform3f(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void {
		this._glCheckError('uniform3f');
		this._checkArgs('uniform3f', arguments);
		this.context.uniform3f(location.native, v0, v1, v2);
	}

	uniform3iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform3iv');
		this._checkArgs('uniform3iv', arguments);
		if (!(value instanceof Int32Array)) {
			value = new Int32Array(value) as any;
		}
		this.context.uniform3iv(location.native, Array.from(value));
	}

	uniform3fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform3fv');
		this._checkArgs('uniform3fv', arguments);
		if (!(value instanceof Float32Array)) {
			value = new Float32Array(value) as any;
		}
		this.context.uniform3fv(location.native, Array.from(value));
	}

	uniform3i(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void {
		this._glCheckError('uniform3i');
		this._checkArgs('uniform3i', arguments);
		this.context.uniform3i(location.native, v0, v1, v2);
	}

	uniform4f(location: WebGLUniformLocation, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('uniform4f');
		this._checkArgs('uniform4f', arguments);
		this.context.uniform4f(location.native, v0, v1, v2, v3);
	}

	uniform4iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform4iv');
		this._checkArgs('uniform4iv', arguments);
		if (!(value instanceof Int32Array)) {
			value = new Int32Array(value) as any;
		}
		this.context.uniform4iv(location.native, Array.from(value));
	}

	uniform4fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform4fv');
		this._checkArgs('uniform4fv', arguments);
		if (!(value instanceof Float32Array)) {
			value = new Float32Array(value) as any;
		}
		this.context.uniform4fv(location.native, Array.from(value));
	}

	uniform4i(location: WebGLUniformLocation, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('uniform4i');
		this._checkArgs('uniform4i', arguments);
		this.context.uniform4i(location.native, v0, v1, v2, v3);
	}

	uniformMatrix2fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void {
		this._glCheckError('uniformMatrix2fv');
		this._checkArgs('uniformMatrix2fv', arguments);
		if (!(value instanceof Float32Array)) {
			value = new Float32Array(value) as any;
		}
		this.context.uniformMatrix2fv(location.native, transpose, Array.from(value));
	}

	uniformMatrix3fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void {
		this._glCheckError('uniformMatrix3fv');
		this._checkArgs('uniformMatrix3fv', arguments);
		if (!(value instanceof Float32Array)) {
			value = new Float32Array(value) as any;
		}
		this.context.uniformMatrix3fv(location.native, transpose, Array.from(value));
	}

	uniformMatrix4fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void {
		this._glCheckError('uniformMatrix4fv');
		this._checkArgs('uniformMatrix4fv', arguments);
		if (!(value instanceof Float32Array)) {
			value = new Float32Array(value) as any;
		}
		this.context.uniformMatrix4fv(location.native, transpose, Array.from(value));
	}

	useProgram(program: WebGLProgram): void {
		this._glCheckError('useProgram');
		this._checkArgs('useProgram', arguments);
		const value = program ? program.native : 0;
		this.context.useProgram(value);
	}

	validateProgram(program: WebGLProgram): void {
		this._glCheckError('validateProgram');
		this._checkArgs('validateProgram', arguments);
		const value = program ? program.native : 0;
		this.context.validateProgram(value);
	}

	vertexAttrib1f(index: number, v0: number): void {
		this._glCheckError('vertexAttrib1f');
		this._checkArgs('vertexAttrib1f', arguments);
		this.context.vertexAttrib1f(index, v0);
	}

	vertexAttrib1fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib1fv');
		this._checkArgs('vertexAttrib1fv', arguments);
		if (!(value instanceof Float32Array)) {
			value = new Float32Array(value) as any;
		}
		this.context.vertexAttrib1fv(index, Array.from(value));
	}

	vertexAttrib2f(index: number, v0: number, v1: number): void {
		this._glCheckError('vertexAttrib2f');
		this._checkArgs('vertexAttrib2f', arguments);
		this.context.vertexAttrib2f(index, v0, v1);
	}

	vertexAttrib2fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib2fv');
		this._checkArgs('vertexAttrib2fv', arguments);
		this.context.vertexAttrib2fv(index, Array.from(value as any));
	}

	vertexAttrib3f(index: number, v0: number, v1: number, v2: number): void {
		this._glCheckError('vertexAttrib3f');
		this._checkArgs('vertexAttrib3f', arguments);
		this.context.vertexAttrib3f(index, v0, v1, v2);
	}

	vertexAttrib3fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib3fv');
		this._checkArgs('vertexAttrib3fv', arguments);
		if (!(value instanceof Float32Array)) {
			value = new Float32Array(value) as any;
		}
		this.context.vertexAttrib3fv(index, Array.from(value));
	}

	vertexAttrib4f(index: number, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('vertexAttrib4f');
		this._checkArgs('vertexAttrib4f', arguments);
		this.context.vertexAttrib4f(index, v0, v1, v2, v3);
	}

	vertexAttrib4fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib4fv');
		this._checkArgs('vertexAttrib4fv', arguments);
		if (!(value instanceof Float32Array)) {
			value = new Float32Array(value) as any;
		}
		this.context.vertexAttrib4fv(index, Array.from(value));
	}

	vertexAttribPointer(index: number, size: number, type: number, normalized: boolean, stride: number, offset: number): void {
		this._glCheckError('vertexAttribPointer');
		this._checkArgs('vertexAttribPointer', arguments);
		this.context.vertexAttribPointer(index, size, type, normalized, stride, offset);
	}

	viewport(x: number, y: number, width: number, height: number): void {
		this._glCheckError('viewport');
		this._checkArgs('viewport', arguments);
		this.context.viewport(x, y, width, height);
	}

	protected getJSArray(value): any[] {
		const count = value.count;
		const array = [];
		for (let i = 0; i < count; i++) {
			array.push(value.objectAtIndex(i));
		}
		return array;
	}

	protected _glCheckError(message: string) {
		if (!WebGLRenderingContext.isDebug) {
			return;
		}
		if (WebGLRenderingContext.filter === 'both' || WebGLRenderingContext.filter === 'error') {
			console.log(message, this.getError());
		}
	}

	protected _checkArgs(message, args) {
		if (!WebGLRenderingContext.isDebug) {
			return;
		}
		if (WebGLRenderingContext.filter === 'both' || WebGLRenderingContext.filter === 'args') {
			console.log('/**** ', message, ' ****/');
			console.dir(args);
			console.log('/**** ', message, ' ****/');
		}
	}
}
