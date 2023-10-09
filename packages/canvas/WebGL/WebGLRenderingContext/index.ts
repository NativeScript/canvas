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

import { ImageSource, profile, Screen } from '@nativescript/core';
import {
	ANGLE_instanced_arrays,
	EXT_blend_minmax,
	EXT_color_buffer_half_float,
	EXT_disjoint_timer_query,
	EXT_shader_texture_lod,
	EXT_sRGB,
	EXT_texture_filter_anisotropic,
	OES_element_index_uint,
	OES_standard_derivatives,
	OES_texture_float,
	OES_texture_float_linear,
	OES_texture_half_float,
	OES_texture_half_float_linear,
	OES_vertex_array_object,
	WEBGL_color_buffer_float,
	WEBGL_compressed_texture_atc,
	WEBGL_compressed_texture_etc,
	WEBGL_compressed_texture_etc1,
	WEBGL_compressed_texture_pvrtc,
	WEBGL_compressed_texture_s3tc,
	WEBGL_depth_texture,
	WEBGL_draw_buffers,
	WEBGL_lose_context,
} from '../WebGLExtensions';
import { ImageAsset } from '../../ImageAsset';
import { Canvas } from '../../Canvas';
import { ImageBitmap } from '../../ImageBitmap';

import { Helpers } from '../../helpers';

let ctor;

export class WebGLRenderingContext extends WebGLRenderingContextBase {
	public static isDebug = false;
	public static filter: 'both' | 'error' | 'args' = 'both';
	_context;

	static {
		Helpers.initialize();
		ctor = global.CanvasModule.createWebGLContext;
	}

	constructor(context, contextOptions?) {
		super(context);
		if (contextOptions) {
			let nativeContext = 0;
			if (global.isAndroid) {
				nativeContext = context.getNativeContext().toString();
			}

			if (global.isIOS) {
				nativeContext = context.nativeContext.toString();
			}

			const ctx = BigInt(nativeContext);

			let direction = 0;

			if (global.isAndroid) {
				if (androidx.core.text.TextUtilsCompat.getLayoutDirectionFromLocale(java.util.Locale.getDefault()) === androidx.core.view.ViewCompat.LAYOUT_DIRECTION_RTL) {
					direction = 1;
				}
			} else {
				// todo
				//direction = 1;
			}

			this._context = ctor(contextOptions, ctx, Screen.mainScreen.scale, -16777216, Screen.mainScreen.scale * 160, direction);
		} else {
			this._context = context;
		}
	}

	get native() {
		return this._context;
	}

	get drawingBufferHeight() {
		if (global.isAndroid) {
			return this._canvas?.drawingBufferHeight ?? 0;
		}
		return this.native.drawingBufferHeight;
	}

	get drawingBufferWidth() {
		if (global.isAndroid) {
			return this._canvas?.drawingBufferWidth ?? 0;
		}
		return this.native.drawingBufferWidth;
	}

	__toDataURL() {
		return this.native.__toDataURL(arguments[0], arguments[1]);
	}

	@profile
	activeTexture(texture: number): void {
		this._glCheckError('activeTexture');
		this._checkArgs('activeTexture', arguments);
		this.native.activeTexture(texture);
	}

	@profile
	attachShader(program: WebGLProgram, shader: WebGLShader): void {
		this._glCheckError('attachShader');
		this._checkArgs('attachShader', arguments);
		const value = program.native;
		const value2 = shader.native;
		this.native.attachShader(value, value2);
	}

	@profile
	bindAttribLocation(program: WebGLProgram, index: number, name: string): void {
		this._glCheckError('bindAttribLocation');
		this._checkArgs('bindAttribLocation', arguments);
		const value = program.native;
		this.native.bindAttribLocation(value, index, name);
	}

	@profile
	bindBuffer(target: number, buffer: WebGLBuffer): void {
		this._glCheckError('bindBuffer');
		this._checkArgs('bindBuffer', arguments);
		const value = buffer ? buffer.native : null;
		this.native.bindBuffer(target, value);
	}

	@profile
	bindFramebuffer(target: number, framebuffer: WebGLFramebuffer): void {
		this._glCheckError('bindFramebuffer');
		this._checkArgs('bindFramebuffer', arguments);
		const value = framebuffer ? framebuffer.native : null;
		this.native.bindFramebuffer(target, value);
	}

	@profile
	bindRenderbuffer(target: number, renderbuffer: WebGLRenderbuffer): void {
		this._glCheckError('bindRenderbuffer');
		this._checkArgs('bindRenderbuffer', arguments);
		const value = renderbuffer ? renderbuffer.native : null;
		this.native.bindRenderbuffer(target, value);
	}

	@profile
	bindTexture(target: number, texture: WebGLTexture): void {
		this._glCheckError('bindTexture');
		this._checkArgs('bindTexture', arguments);
		const value = texture ? texture.native : null;
		this.native.bindTexture(target, value);
	}

	@profile
	blendColor(red: number, green: number, blue: number, alpha: number): void {
		this._glCheckError('blendColor');
		this._checkArgs('blendColor', arguments);
		this.native.blendColor(red, green, blue, alpha);
	}

	@profile
	blendEquationSeparate(modeRGB: number, modeAlpha: number): void {
		this._glCheckError('blendEquationSeparate');
		this._checkArgs('blendEquationSeparate', arguments);
		this.native.blendEquationSeparate(modeRGB, modeAlpha);
	}

	@profile
	blendEquation(mode: number): void {
		this._glCheckError('blendEquation');
		this._checkArgs('blendEquation', arguments);
		this.native.blendEquation(mode);
	}

	@profile
	blendFuncSeparate(srcRGB: number = this.ONE, dstRGB: number = this.ZERO, srcAlpha: number = this.ONE, dstAlpha: number = this.ZERO): void {
		this._glCheckError('blendFuncSeparate');
		this._checkArgs('blendFuncSeparate', arguments);
		this.native.blendFuncSeparate(srcRGB, dstRGB, srcAlpha, dstAlpha);
	}

	@profile
	blendFunc(sfactor: number = this.ONE, dfactor: number = this.ZERO): void {
		this._glCheckError('blendFunc');
		this._checkArgs('blendFunc', arguments);
		this.native.blendFunc(sfactor, dfactor);
	}

	bufferData(target: number, size: number, usage: number): void;

	bufferData(target: number, srcData: ArrayBuffer | ArrayBufferView, usage: number): void;

	@profile
	bufferData(target: any, srcData: any, usage: any) {
		this._glCheckError('bufferData');
		this._checkArgs('bufferData', arguments);
		this.native.bufferData(target, srcData, usage);
	}

	@profile
	bufferSubData(target: number, offset: number, srcData: ArrayBuffer | ArrayBufferView): void {
		this._glCheckError('bufferSubData');
		this._checkArgs('bufferSubData', arguments);
		this.native.bufferSubData(target, offset, srcData);
	}

	@profile
	checkFramebufferStatus(target: number): number {
		this._glCheckError('checkFramebufferStatus');
		this._checkArgs('checkFramebufferStatus', arguments);
		return this.native.checkFramebufferStatus(target);
	}

	@profile
	clearColor(red: number, green: number, blue: number, alpha: number): void {
		this._glCheckError('clearColor');
		this._checkArgs('clearColor', arguments);
		this.native.clearColor(red, green, blue, alpha);
	}

	@profile
	clearDepth(depth: number): void {
		this._glCheckError('clearDepth');
		this._checkArgs('clearDepth', arguments);
		this.native.clearDepth(depth);
	}

	@profile
	clearStencil(stencil: number): void {
		this._glCheckError('clearStencil');
		this._checkArgs('clearStencil', arguments);
		this.native.clearStencil(stencil);
	}

	@profile
	clear(mask: number): void {
		this._glCheckError('clear');
		this._checkArgs('clear', arguments);
		this.native.clear(mask);
	}

	@profile
	colorMask(red: boolean, green: boolean, blue: boolean, alpha: boolean): void {
		this._glCheckError('colorMask');
		this._checkArgs('colorMask', arguments);
		this.native.colorMask(red, green, blue, alpha);
	}

	@profile
	commit(): void {
		// NOOP
		this.native.commit();
	}

	@profile
	compileShader(shader: WebGLShader): void {
		this._glCheckError('compileShader');
		this._checkArgs('compileShader', arguments);
		const value = shader.native;
		this.native.compileShader(value);
	}

	@profile
	compressedTexImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, pixels: ArrayBuffer | ArrayBufferView): void {
		this._glCheckError('compressedTexImage2D');
		this._checkArgs('compressedTexImage2D', arguments);
		this.native.compressedTexImage2D(target, level, internalformat, width, height, border, pixels);
	}

	@profile
	compressedTexSubImage2D(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, pixels: ArrayBufferView): void {
		this._glCheckError('compressedTexSubImage2D');
		this._checkArgs('compressedTexSubImage2D', arguments);
		this.native.compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels);
	}

	@profile
	copyTexImage2D(target: number, level: number, internalformat: number, x: number, y: number, width: number, height: number, border: number): void {
		this._glCheckError('copyTexImage2D');
		this._checkArgs('copyTexImage2D', arguments);
		this.native.copyTexImage2D(target, level, internalformat, x, y, width, height, border);
	}

	@profile
	copyTexSubImage2D(target: number, level: number, xoffset: number, yoffset: number, x: number, y: number, width: number, height: number): void {
		this._glCheckError('copyTexSubImage2D');
		this._checkArgs('copyTexSubImage2D', arguments);
		this.native.copyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height);
	}

	@profile
	createBuffer(): WebGLBuffer {
		this._glCheckError('createBuffer');
		this._checkArgs('createBuffer', arguments);
		const id = this.native.createBuffer();
		return new WebGLBuffer(id);
	}

	@profile
	createFramebuffer(): WebGLFramebuffer {
		this._glCheckError('createFramebuffer');
		this._checkArgs('createFramebuffer', arguments);
		return new WebGLFramebuffer(this.native.createFramebuffer());
	}

	@profile
	createProgram(): WebGLProgram {
		this._glCheckError('createProgram');
		this._checkArgs('createProgram', arguments);
		return new WebGLProgram(this.native.createProgram());
	}

	@profile
	createRenderbuffer(): WebGLRenderbuffer {
		this._glCheckError('createRenderbuffer');
		this._checkArgs('createRenderbuffer', arguments);
		return new WebGLRenderbuffer(this.native.createRenderbuffer());
	}

	@profile
	createShader(type: number): WebGLShader {
		this._glCheckError('createShader');
		this._checkArgs('createShader', arguments);
		return new WebGLShader(this.native.createShader(type));
	}

	@profile
	createTexture(): WebGLTexture {
		this._glCheckError('createTexture');
		this._checkArgs('createTexture', arguments);
		return new WebGLTexture(this.native.createTexture());
	}

	@profile
	cullFace(mode: number): void {
		this._glCheckError('cullFace');
		this._checkArgs('cullFace', arguments);
		this.native.cullFace(mode);
	}

	@profile
	deleteBuffer(buffer: WebGLBuffer): void {
		this._glCheckError('deleteBuffer');
		this._checkArgs('deleteBuffer', arguments);
		const value = buffer.native;
		this.native.deleteBuffer(value);
	}

	@profile
	deleteFramebuffer(frameBuffer: WebGLFramebuffer): void {
		this._glCheckError('deleteFramebuffer');
		this._checkArgs('deleteFramebuffer', arguments);
		const value = frameBuffer.native;
		this.native.deleteFramebuffer(value);
	}

	@profile
	deleteProgram(program: WebGLProgram): void {
		this._glCheckError('deleteProgram');
		this._checkArgs('deleteProgram', arguments);
		const value = program.native;
		this.native.deleteProgram(value);
	}

	@profile
	deleteRenderbuffer(renderBuffer: WebGLRenderbuffer): void {
		this._glCheckError('deleteRenderbuffer');
		this._checkArgs('deleteRenderbuffer', arguments);
		const value = renderBuffer.native;
		this.native.deleteRenderbuffer(value);
	}

	@profile
	deleteShader(shader: WebGLRenderbuffer): void {
		this._glCheckError('deleteShader');
		this._checkArgs('deleteShader', arguments);
		this.native.deleteShader(shader.native);
	}

	@profile
	deleteTexture(texture: WebGLTexture): void {
		this._glCheckError('deleteTexture');
		this._checkArgs('deleteTexture', arguments);
		const value = texture.native;
		this.native.deleteTexture(value);
	}

	@profile
	depthFunc(func: number): void {
		this._glCheckError('depthFunc');
		this._checkArgs('depthFunc', arguments);
		this.native.depthFunc(func);
	}

	@profile
	depthMask(flag: boolean): void {
		this._glCheckError('depthMask');
		this._checkArgs('depthMask', arguments);
		this.native.depthMask(flag);
	}

	@profile
	depthRange(zNear: number, zFar: number): void {
		this._glCheckError('depthRange');
		this._checkArgs('depthRange', arguments);
		this.native.depthRange(zNear, zFar);
	}

	@profile
	detachShader(program: WebGLProgram, shader: WebGLShader): void {
		this._glCheckError('detachShader');
		this._checkArgs('detachShader', arguments);
		const value = program.native;
		const value2 = shader.native;
		this.native.detachShader(value, value2);
	}

	@profile
	disableVertexAttribArray(index: number): void {
		this._glCheckError('disableVertexAttribArray');
		this._checkArgs('disableVertexAttribArray', arguments);
		this.native.disableVertexAttribArray(index);
	}

	@profile
	disable(cap: number): void {
		this._glCheckError('disable');
		this._checkArgs('disable', arguments);
		this.native.disable(cap);
	}

	@profile
	drawArrays(mode: number, first: number, count: number): void {
		this._glCheckError('drawArrays');
		this._checkArgs('drawArrays', arguments);
		this.native.drawArrays(mode, first, count);
	}

	@profile
	drawElements(mode: number, count: number, type: number, offset: number): void {
		this._glCheckError('drawElements');
		this._checkArgs('drawElements', arguments);
		this.native.drawElements(mode, count, type, offset);
	}

	@profile
	enableVertexAttribArray(index: number): void {
		this._glCheckError('enableVertexAttribArray');
		this._checkArgs('enableVertexAttribArray', arguments);
		this.native.enableVertexAttribArray(index);
	}

	@profile
	enable(cap: number): void {
		this._glCheckError('enable');
		this._checkArgs('enable', arguments);
		this.native.enable(cap);
	}

	@profile
	finish(): void {
		this._glCheckError('finish');
		this.native.finish();
	}

	@profile
	flush(): void {
		this._glCheckError('flush');
		this.native.flush();
	}

	@profile
	framebufferRenderbuffer(target: number, attachment: number, renderbuffertarget: number, renderbuffer: WebGLRenderbuffer): void {
		this._glCheckError('framebufferRenderbuffer');
		this._checkArgs('framebufferRenderbuffer', arguments);
		const value = renderbuffer.native;
		this.native.framebufferRenderbuffer(target, attachment, renderbuffertarget, value);
	}

	@profile
	framebufferTexture2D(target: number, attachment: number, textarget: number, texture: WebGLTexture, level: number): void {
		this._glCheckError('framebufferTexture2D');
		this._checkArgs('framebufferTexture2D', arguments);
		const value = texture.native;
		this.native.framebufferTexture2D(target, attachment, textarget, value, level);
	}

	@profile
	frontFace(mode: number): void {
		this._checkArgs('frontFace', arguments);
		this._glCheckError('frontFace');
		this.native.frontFace(mode);
	}

	@profile
	generateMipmap(target: number): void {
		this._checkArgs('generateMipmap', arguments);
		this._glCheckError('generateMipmap');
		this.native.generateMipmap(target);
	}

	@profile
	getActiveAttrib(program: WebGLProgram, index: number): WebGLActiveInfo {
		this._glCheckError('getActiveAttrib');
		this._checkArgs('getActiveAttrib', arguments);
		const value = program.native;
		const attrib = this.native.getActiveAttrib(value, index);
		return new WebGLActiveInfo(attrib);
	}

	@profile
	getActiveUniform(program: WebGLProgram, index: number): WebGLActiveInfo {
		this._glCheckError('getActiveUniform');
		this._checkArgs('getActiveUniform', arguments);
		const value = program.native;
		const uniform = this.native.getActiveUniform(value, index);
		return new WebGLActiveInfo(uniform);
	}

	@profile
	getAttachedShaders(program: WebGLProgram): WebGLShader[] {
		this._glCheckError('getAttachedShaders');
		this._checkArgs('getAttachedShaders', arguments);
		const value = program.native;
		return this.native.getAttachedShaders(value).map((shader) => new WebGLShader(shader));
	}

	@profile
	getAttribLocation(program: WebGLProgram, name: string): number {
		this._glCheckError('getAttribLocation');
		this._checkArgs('getAttribLocation', arguments);
		const value = program.native;
		return this.native.getAttribLocation(value, name);
	}

	@profile
	getBufferParameter(target: number, pname: number): number {
		this._glCheckError('getBufferParameter');
		this._checkArgs('getBufferParameter', arguments);
		return this.native.getBufferParameter(target, pname);
	}

	@profile
	getContextAttributes(): any {
		return this.native.getContextAttributes();
	}

	@profile
	getError(): number {
		return this.native.getError();
	}

	@profile
	getExtension(name: string) {
		this._glCheckError('getExtension');
		this._checkArgs('getExtension', arguments);
		if (name === 'EXT_disjoint_timer_query_webgl2') {
			return null;
		}

		const ext = this.native.getExtension(name);
		if (ext) {
			const ext_name = ext.ext_name;
			switch (ext_name) {
				case 'ANGLE_instanced_arrays':
					return new ANGLE_instanced_arrays(ext);
				case 'EXT_blend_minmax':
					return new EXT_blend_minmax(ext);
				case 'EXT_color_buffer_half_float':
					return new EXT_color_buffer_half_float(ext);
				case 'EXT_disjoint_timer_query':
					return new EXT_disjoint_timer_query(ext);
				case 'EXT_sRGB':
					return new EXT_sRGB(ext);
				case 'EXT_shader_texture_lod':
					return new EXT_shader_texture_lod(ext);
				case 'EXT_texture_filter_anisotropic':
					return new EXT_texture_filter_anisotropic(ext);
				case 'OES_element_index_uint':
					return new OES_element_index_uint(ext);
				case 'OES_standard_derivatives':
					return new OES_standard_derivatives(ext);
				case 'OES_texture_float':
					return new OES_texture_float(ext);
				case 'OES_texture_float_linear':
					return new OES_texture_float_linear(ext);
				case 'OES_texture_half_float':
					return new OES_texture_half_float(ext);
				case 'OES_texture_half_float_linear':
					return new OES_texture_half_float_linear(ext);
				case 'OES_vertex_array_object':
					return new OES_vertex_array_object(ext);
				case 'WEBGL_color_buffer_float':
					return new WEBGL_color_buffer_float(ext);
				case 'WEBGL_compressed_texture_atc':
					return new WEBGL_compressed_texture_atc(ext);
				case 'WEBGL_compressed_texture_etc':
					return new WEBGL_compressed_texture_etc(ext);
				case 'WEBGL_compressed_texture_etc1':
					return new WEBGL_compressed_texture_etc1(ext);
				case 'WEBGL_compressed_texture_pvrtc':
					return new WEBGL_compressed_texture_pvrtc(ext);
				case 'WEBGL_compressed_texture_s3tc':
					return new WEBGL_compressed_texture_s3tc(ext);
				case 'WEBGL_lose_context':
					return new WEBGL_lose_context(ext);
				case 'WEBGL_depth_texture':
					return new WEBGL_depth_texture(ext);
				case 'WEBGL_draw_buffers':
					return new WEBGL_draw_buffers(ext);
			}
		}
		return null;
	}

	@profile
	getFramebufferAttachmentParameter(target: number, attachment: number, pname: number): number | WebGLRenderbuffer | WebGLTexture {
		this._glCheckError('getFramebufferAttachmentParameter');
		this._checkArgs('getFramebufferAttachmentParameter', arguments);

		const param = this.native.getFramebufferAttachmentParameter(target, attachment, pname);
		if (typeof param === 'object') {
			if (param.isRenderbuffer) {
				return new WebGLRenderbuffer(param);
			} else if (param.isTexture()) {
				return new WebGLTexture(param);
			}
		}
		return param;
	}

	_handleGetParameter(pname, value) {
		switch (pname) {
			case this.COLOR_WRITEMASK:
			case this.COLOR_CLEAR_VALUE:
			case this.BLEND_COLOR:
			case this.ALIASED_LINE_WIDTH_RANGE:
			case this.ALIASED_POINT_SIZE_RANGE:
			case this.DEPTH_RANGE:
				return value;
			case this.ARRAY_BUFFER_BINDING:
			case this.ELEMENT_ARRAY_BUFFER_BINDING:
				if (value) {
					new WebGLBuffer(value);
				}
				return null;
			case this.CURRENT_PROGRAM:
				if (value) {
					return new WebGLProgram(value);
				}
				return null;
			case this.COMPRESSED_TEXTURE_FORMATS:
				return value;
			case this.RENDERBUFFER_BINDING:
				if (value) {
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
				return value;
			case this.TEXTURE_BINDING_CUBE_MAP:
			case this.TEXTURE_BINDING_2D:
				if (value) {
					return new WebGLTexture(value);
				}
				return null;
			case this.VERSION:
				if (this._type === 'webgl2') {
					return 'WebGL 2.0 (OpenGL ES 3.0 NativeScript)';
				} else {
					return 'WebGL 1.0 (OpenGL ES 2.0 NativeScript)';
				}
			default:
				return value;
		}
	}

	@profile
	getParameter(pname: number): number[] | number | WebGLBuffer | WebGLProgram | WebGLFramebuffer | WebGLRenderbuffer | WebGLTexture | Uint32Array | Int32Array | Float32Array | string | null {
		this._glCheckError('getParameter');
		this._checkArgs('getParameter', arguments);
		const value = this.native.getParameter(pname);
		return this._handleGetParameter(pname, value);
	}

	@profile
	getProgramInfoLog(program: WebGLProgram): string {
		this._glCheckError('getProgramInfoLog');
		this._checkArgs('getProgramInfoLog', arguments);
		const value = program.native;
		return this.native.getProgramInfoLog(value);
	}

	@profile
	getProgramParameter(program: WebGLProgram, pname: number): number | boolean {
		this._glCheckError('getProgramParameter');
		this._checkArgs('getProgramParameter', arguments);
		const value = program.native;
		const result = this.native.getProgramParameter(value, pname);
		return result;
	}

	@profile
	getRenderbufferParameter(target: number, pname: number): number {
		this._glCheckError('getRenderbufferParameter');
		this._checkArgs('getRenderbufferParameter', arguments);
		return this.native.getRenderbufferParameter(target, pname);
	}

	@profile
	getShaderInfoLog(shader: WebGLShader): string {
		this._glCheckError('getShaderInfoLog');
		this._checkArgs('getShaderInfoLog', arguments);
		const value = shader.native;
		return this.native.getShaderInfoLog(value);
	}

	@profile
	getShaderParameter(shader: WebGLShader, pname: number): boolean | number {
		this._glCheckError('getShaderParameter');
		this._checkArgs('getShaderParameter', arguments);
		const value = shader.native;
		const result = this.native.getShaderParameter(value, pname);
		return result;
	}

	@profile
	getShaderPrecisionFormat(shaderType: number, precisionType: number): WebGLShaderPrecisionFormat {
		this._glCheckError('getShaderPrecisionFormat');
		this._checkArgs('getShaderPrecisionFormat', arguments);
		const precision = this.native.getShaderPrecisionFormat(shaderType, precisionType);
		return new WebGLShaderPrecisionFormat(precision);
	}

	@profile
	getShaderSource(shader: WebGLShader): string {
		this._glCheckError('getShaderSource');
		this._checkArgs('getShaderSource', arguments);
		const value = shader ? shader.native : 0;
		return this.native.getShaderSource(value);
	}

	@profile
	getSupportedExtensions(): string[] {
		this._glCheckError('getSupportedExtensions');
		return this.native.getSupportedExtensions();
	}

	@profile
	getTexParameter(target: number, pname: number): number {
		this._glCheckError('getTexParameter');
		this._checkArgs('getTexParameter', arguments);
		return this.native.getTexParameter(target, pname);
	}

	@profile
	getUniformLocation(program: WebGLProgram, name: string): WebGLUniformLocation {
		this._glCheckError('getUniformLocation');
		this._checkArgs('getUniformLocation', arguments);
		const value = program.native;
		const id = this.native.getUniformLocation(value, name);
		if (id === -1) {
			return null;
		}
		return new WebGLUniformLocation(id);
	}

	@profile
	getUniform(program: WebGLProgram, location: WebGLUniformLocation): any {
		this._glCheckError('getUniform');
		this._checkArgs('getUniform', arguments);
		const value = program.native;
		const value2 = location.native;
		const uniform = this.native.getUniform(value, value2);
		if (uniform && uniform.length) {
			return uniform;
		}
		return uniform;
	}

	@profile
	getVertexAttribOffset(index: number, pname: number): number {
		this._glCheckError('getVertexAttribOffset');
		this._checkArgs('getVertexAttribOffset', arguments);
		return this.native.getVertexAttribOffset(index, pname);
	}

	@profile
	getVertexAttrib(index: number, pname: number): number[] | boolean | number | Float32Array {
		this._glCheckError('getVertexAttrib');
		this._checkArgs('getVertexAttrib', arguments);
		const value = this.native.getVertexAttrib(index, pname);
		if (pname === this.CURRENT_VERTEX_ATTRIB) {
			return value;
		}
		return value;
	}

	@profile
	hint(target: number, mode: number): void {
		this._glCheckError('hint');
		this._checkArgs('hint', arguments);
		this.native.hint(target, mode);
	}

	@profile
	isBuffer(buffer: WebGLBuffer): boolean {
		this._glCheckError('isBuffer');
		this._checkArgs('isBuffer', arguments);
		const value = buffer.native;
		return this.native.isBuffer(value);
	}

	@profile
	isContextLost(): boolean {
		this._glCheckError('isContextLost');
		return this.native.isContextLost();
	}

	@profile
	isEnabled(cap: number): boolean {
		this._glCheckError('isEnabled');
		this._checkArgs('isEnabled', arguments);
		return this.native.isEnabled(cap);
	}

	@profile
	isFramebuffer(framebuffer: WebGLFramebuffer): boolean {
		this._glCheckError('isFramebuffer');
		this._checkArgs('isFramebuffer', arguments);
		const value = framebuffer ? framebuffer.native : 0;
		return this.native.isFramebuffer(value);
	}

	@profile
	isProgram(program: WebGLProgram): boolean {
		this._glCheckError('isProgram');
		this._checkArgs('isProgram', arguments);
		const value = program.native;
		return this.native.isProgram(value);
	}

	@profile
	isRenderbuffer(renderbuffer: WebGLRenderbuffer): boolean {
		this._glCheckError('isRenderbuffer');
		this._checkArgs('isRenderbuffer', arguments);
		const value = renderbuffer.native;
		return this.native.isRenderbuffer(value);
	}

	@profile
	isShader(shader: WebGLShader): boolean {
		this._glCheckError('isShader');
		this._checkArgs('isShader', arguments);
		const value = shader.native;
		return this.native.isShader(value);
	}

	@profile
	isTexture(texture: WebGLTexture): boolean {
		this._glCheckError('isTexture');
		this._checkArgs('isTexture', arguments);
		const value = texture.native;
		return this.native.isTexture(value);
	}

	@profile
	lineWidth(width: number): void {
		this._glCheckError('lineWidth');
		this._checkArgs('lineWidth', arguments);
		this.native.lineWidth(width);
	}

	@profile
	linkProgram(program: WebGLProgram): void {
		this._glCheckError('linkProgram');
		this._checkArgs('linkProgram', arguments);
		const value = program ? program.native : 0;
		this.native.linkProgram(value);
	}

	_flipY = false;
	@profile
	pixelStorei(pname: number, param: any): void {
		this._glCheckError('pixelStorei');
		this._checkArgs('pixelStorei', arguments);
		if (pname === this.UNPACK_FLIP_Y_WEBGL || pname === this.UNPACK_PREMULTIPLY_ALPHA_WEBGL) {
			if (pname === this.UNPACK_FLIP_Y_WEBGL) {
				this._flipY = param;
			}
			this.native.pixelStorei(pname, param);
		} else if (pname === this.PACK_ALIGNMENT || pname === this.UNPACK_ALIGNMENT || pname === this.UNPACK_COLORSPACE_CONVERSION_WEBGL) {
			if (pname === this.UNPACK_COLORSPACE_CONVERSION_WEBGL) {
				param = 0x9244;
			} else if (pname === this.PACK_ALIGNMENT || pname === this.UNPACK_ALIGNMENT) {
				//param = 1;
			}
			this.native.pixelStorei(pname, param);
		} else {
			this.native.pixelStorei(pname, param);
		}
	}

	@profile
	polygonOffset(factor: number, units: number): void {
		this._glCheckError('polygonOffset');
		this._checkArgs('polygonOffset', arguments);
		this.native.polygonOffset(factor, units);
	}

	@profile
	readPixels(x: number, y: number, width: number, height: number, format: number, type: number, pixels: ArrayBuffer | ArrayBufferView): void {
		this._glCheckError('readPixels');
		this._checkArgs('readPixels', arguments);
		this.native.readPixels(x, y, width, height, format, type, pixels);
	}

	@profile
	renderbufferStorage(target: number, internalFormat: number, width: number, height: number): void {
		this._glCheckError('renderbufferStorage');
		this._checkArgs('renderbufferStorage', arguments);
		if (internalFormat === this.DEPTH_STENCIL) {
			// DEPTH24_STENCIL8 = 35056
			// DEPTH24_STENCIL8_OES = 0x88F0
			internalFormat = 0x88f0;
		}
		this.native.renderbufferStorage(target, internalFormat, width, height);
	}

	@profile
	sampleCoverage(value: number, invert: boolean): void {
		this._glCheckError('sampleCoverage');
		this._checkArgs('sampleCoverage', arguments);
		this.native.sampleCoverage(value, invert);
	}

	@profile
	scissor(x: number, y: number, width: number, height: number): void {
		this._glCheckError('scissor');
		this._checkArgs('scissor', arguments);
		this.native.scissor(x, y, width, height);
	}

	@profile
	shaderSource(shader: WebGLShader, source: string): void {
		this._glCheckError('shaderSource');
		this._checkArgs('shaderSource', arguments);
		const value = shader ? shader.native : 0;
		this.native.shaderSource(value, source);
	}

	@profile
	stencilFuncSeparate(face: number, func: number, ref: number, mask: number): void {
		this._glCheckError('stencilFuncSeparate');
		this._checkArgs('stencilFuncSeparate', arguments);
		this.native.stencilFuncSeparate(face, func, ref, mask);
	}

	@profile
	stencilFunc(func: number, ref: number, mask: number): void {
		this._glCheckError('stencilFunc');
		this._checkArgs('stencilFunc', arguments);
		this.native.stencilFunc(func, ref, mask);
	}

	@profile
	stencilMaskSeparate(face: number, mask: number): void {
		this._glCheckError('stencilMaskSeparate');
		this._checkArgs('stencilMaskSeparate', arguments);
		this.native.stencilMaskSeparate(face, mask);
	}

	@profile
	stencilMask(mask: number): void {
		this._glCheckError('stencilMask');
		this._checkArgs('stencilMask', arguments);
		this.native.stencilMask(mask);
	}

	@profile
	stencilOpSeparate(face: number, fail: number, zfail: number, zpass: number): void {
		this._glCheckError('stencilOpSeparate');
		this._checkArgs('stencilOpSeparate', arguments);
		this.native.stencilOpSeparate(face, fail, zfail, zpass);
	}

	@profile
	stencilOp(fail: number, zfail: number, zpass: number): void {
		this._glCheckError('stencilOp');
		this._checkArgs('stencilOp', arguments);
		this.native.stencilOp(fail, zfail, zpass);
	}

	texImage2D(target: number, level: number, internalformat: number, format: number, type: number, pixels: any): void;

	texImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type: number, pixels: ArrayBufferView): void;

	@profile
	texImage2D(target: any, level: any, internalformat: any, width: any, height: any, border: any, format?: any, type?: any, pixels?: any) {
		this._glCheckError('texImage2D');
		this._checkArgs('texImage2D', arguments);
		if (arguments.length === 9) {
			this.native.texImage2D(target, level, internalformat, width, height, border, format, type ?? internalformat, pixels);
		} else if (arguments.length === 6) {
			if (border && typeof border.tagName === 'string' && (border.tagName === 'VID' || border.tagName === 'VIDEO') && border._video && typeof border._video.getCurrentFrame === 'function') {
				border._video.getCurrentFrame(this.native, this, target, level, internalformat, width, height);
			} else if (border && typeof border.getCurrentFrame === 'function') {
				border.getCurrentFrame(this.native, this, target, level, internalformat, width, height);
			} else if (border instanceof ImageAsset) {
				this.native.texImage2D(target, level, internalformat, width, height, border.native);
			} else if (border instanceof ImageBitmap) {
				this.native.texImage2D(target, level, internalformat, width, height, border.native);
			} else if (global.isAndroid && border instanceof android.graphics.Bitmap) {
				// todo ios
				this.native.texImage2D(target, level, internalformat, width, height, border);
			} else if (border instanceof ImageSource) {
				this.native.texImage2D(target, level, internalformat, width, height, border.android);
			} else if (border && typeof border.tagName === 'string' && (border.tagName === 'IMG' || border.tagName === 'IMAGE')) {
				if (border._asset instanceof ImageAsset) {
					this.native.texImage2D(target, level, internalformat, width, height, border._asset.native);
				} else if (border._imageSource instanceof ImageSource) {
					this.native.texImage2D(target, level, internalformat, width, height, border._imageSource.android);
				} else if (global.isAndroid && border._image instanceof android.graphics.Bitmap) {
					//todo ios
					this.native.texImage2D(target, level, internalformat, width, height, border._image);
				} else if (typeof border.src === 'string') {
					this.native.texImage2D(target, level, internalformat, width, height, ImageSource.fromFileSync(border.src).android);
				}
			} else if (border && typeof border.tagName === 'string' && border.tagName === 'CANVAS' && border._canvas instanceof Canvas) {
				this.native.texImage2D(target, level, internalformat, width, height, border._canvas.native);
			}
		}
	}

	@profile
	texParameterf(target: number, pname: number, param: number): void {
		this._glCheckError('texParameterf');
		this._checkArgs('texParameterf', arguments);
		this.native.texParameterf(target, pname, param);
	}

	@profile
	texParameteri(target: number, pname: number, param: number): void {
		this._glCheckError('texParameteri');
		this._checkArgs('texParameteri', arguments);
		this.native.texParameteri(target, pname, param);
	}

	texSubImage2D(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, type: number, pixels: ArrayBufferView): void;

	texSubImage2D(target: number, level: number, xoffset: number, yoffset: number, format: number, type: number, pixels: any): void;

	@profile
	texSubImage2D(target: any, level: any, xoffset: any, yoffset: any, width: any, height: any, format: any, type?: any, pixels?: any) {
		this._glCheckError('texSubImage2D');
		this._checkArgs('texSubImage2D', arguments);
		if (arguments.length === 9) {
			this.native.texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels);
		} else if (arguments.length === 7) {
			if (global.isAndroid && format instanceof android.graphics.Bitmap) {
				// todo
				this.native.texSubImage2D(target, level, xoffset, yoffset, width, height, format);
			} else if (format instanceof ImageSource) {
				this.native.texSubImage2D(target, level, xoffset, yoffset, width, height, format.android);
			} else if (format instanceof ImageAsset) {
				this.native.texSubImage2D(target, level, xoffset, yoffset, width, height, format.native);
			} else if (format instanceof ImageBitmap) {
				this.native.texSubImage2D(target, level, xoffset, yoffset, width, height, format.native);
			} else if (format && typeof format.tagName === 'string' && (format.tagName === 'IMG' || format.tagName === 'IMAGE')) {
				if (format._imageSource instanceof ImageSource) {
					this.native.texSubImage2D(target, level, xoffset, yoffset, width, height, format._imageSource.android);
				} else if (global.isAndroid && format._image instanceof android.graphics.Bitmap) {
					// todo
					this.native.texSubImage2D(target, level, xoffset, yoffset, width, height, format._image);
				} else if (format._asset instanceof ImageAsset) {
					this.native.texSubImage2D(target, level, xoffset, yoffset, width, height, format._asset.native);
				} else if (typeof format.src === 'string') {
					const result = ImageSource.fromFileSync(format.src);
					this.native.texSubImage2D(target, level, xoffset, yoffset, width, height, result ? result.android : null);
				}
			} else if (format && typeof format.tagName === 'string' && format.tagName === 'CANVAS' && format._canvas instanceof Canvas) {
				this.native.texSubImage2D(target, level, xoffset, yoffset, width, height, format._canvas.native);
			}
		}
	}

	@profile
	uniform1f(location: WebGLUniformLocation, v0: number): void {
		this._glCheckError('uniform1f');
		this._checkArgs('uniform1f', arguments);
		const loc = location.native;
		this.native.uniform1f(loc, v0);
	}

	@profile
	uniform1iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform1iv');
		this._checkArgs('uniform1iv', arguments);
		const loc = location.native;
		this.native.uniform1iv(loc, value);
	}

	@profile
	uniform1fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform1fv');
		this._checkArgs('uniform1fv', arguments);
		const loc = location.native;
		this.native.uniform1fv(loc, value);
	}

	@profile
	uniform1i(location: WebGLUniformLocation, v0: number): void {
		this._glCheckError('uniform1i');
		this._checkArgs('uniform1i', arguments);
		const loc = location.native;
		this.native.uniform1i(loc, Number(v0));
	}

	@profile
	uniform2f(location: WebGLUniformLocation, v0: number, v1: number): void {
		this._glCheckError('uniform2f');
		this._checkArgs('uniform2f', arguments);
		const loc = location.native;
		this.native.uniform2f(loc, v0, v1);
	}

	@profile
	uniform2iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform2iv');
		this._checkArgs('uniform2iv', arguments);

		const loc = location.native;
		this.native.uniform2iv(loc, value);
	}

	@profile
	uniform2fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform2fv');
		this._checkArgs('uniform2fv', arguments);

		const loc = location.native;
		this.native.uniform2fv(loc, value);
	}

	@profile
	uniform2i(location: WebGLUniformLocation, v0: number, v1: number): void {
		this._glCheckError('uniform2i');
		this._checkArgs('uniform2i', arguments);
		const loc = location.native;
		this.native.uniform2i(loc, v0, v1);
	}

	@profile
	uniform3f(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void {
		this._glCheckError('uniform3f');
		this._checkArgs('uniform3f', arguments);
		const loc = location.native;
		this.native.uniform3f(loc, v0, v1, v2);
	}

	@profile
	uniform3iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform3iv');
		this._checkArgs('uniform3iv', arguments);

		const loc = location.native;
		this.native.uniform3iv(loc, value);
	}

	@profile
	uniform3fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform3fv');
		this._checkArgs('uniform3fv', arguments);

		const loc = location.native;
		this.native.uniform3fv(loc, value);
	}

	@profile
	uniform3i(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void {
		this._glCheckError('uniform3i');
		this._checkArgs('uniform3i', arguments);
		const loc = location.native;
		this.native.uniform3i(loc, v0, v1, v2);
	}

	@profile
	uniform4f(location: WebGLUniformLocation, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('uniform4f');
		this._checkArgs('uniform4f', arguments);
		const loc = location.native;
		this.native.uniform4f(loc, v0, v1, v2, v3);
	}

	@profile
	uniform4iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform4iv');
		this._checkArgs('uniform4iv', arguments);

		const loc = location.native;
		this.native.uniform4iv(loc, value);
	}

	@profile
	uniform4fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform4fv');
		this._checkArgs('uniform4fv', arguments);

		const loc = location.native;
		this.native.uniform4fv(loc, value);
	}

	@profile
	uniform4i(location: WebGLUniformLocation, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('uniform4i');
		this._checkArgs('uniform4i', arguments);
		const loc = location.native;
		this.native.uniform4i(loc, v0, v1, v2, v3);
	}

	@profile
	uniformMatrix2fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void {
		this._glCheckError('uniformMatrix2fv');
		this._checkArgs('uniformMatrix2fv', arguments);

		const loc = location.native;
		this.native.uniformMatrix2fv(loc, transpose, value);
	}

	@profile
	uniformMatrix3fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void {
		this._glCheckError('uniformMatrix3fv');
		this._checkArgs('uniformMatrix3fv', arguments);

		const loc = location.native;
		this.native.uniformMatrix3fv(loc, transpose, value);
	}

	@profile
	uniformMatrix4fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void {
		this._glCheckError('uniformMatrix4fv');
		this._checkArgs('uniformMatrix4fv', arguments);

		const loc = location.native;
		this.native.uniformMatrix4fv(loc, transpose, value);
	}

	@profile
	useProgram(program: WebGLProgram): void {
		this._glCheckError('useProgram');
		this._checkArgs('useProgram', arguments);
		const value = program ? program.native : null;
		this.native.useProgram(value);
	}

	@profile
	validateProgram(program: WebGLProgram): void {
		this._glCheckError('validateProgram');
		this._checkArgs('validateProgram', arguments);
		const value = program.native;
		this.native.validateProgram(value);
	}

	@profile
	vertexAttrib1f(index: number, v0: number): void {
		this._glCheckError('vertexAttrib1f');
		this._checkArgs('vertexAttrib1f', arguments);
		this.native.vertexAttrib1f(index, v0);
	}

	@profile
	vertexAttrib1fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib1fv');
		this._checkArgs('vertexAttrib1fv', arguments);

		this.native.vertexAttrib1fv(index, value);
	}

	@profile
	vertexAttrib2f(index: number, v0: number, v1: number): void {
		this._glCheckError('vertexAttrib2f');
		this._checkArgs('vertexAttrib2f', arguments);
		this.native.vertexAttrib2f(index, v0, v1);
	}

	@profile
	vertexAttrib2fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib2fv');
		this._checkArgs('vertexAttrib2fv', arguments);

		this.native.vertexAttrib2fv(index, value);
	}

	@profile
	vertexAttrib3f(index: number, v0: number, v1: number, v2: number): void {
		this._glCheckError('vertexAttrib3f');
		this._checkArgs('vertexAttrib3f', arguments);
		this.native.vertexAttrib3f(index, v0, v1, v2);
	}

	@profile
	vertexAttrib3fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib3fv');
		this._checkArgs('vertexAttrib3fv', arguments);

		this.native.vertexAttrib3fv(index, value);
	}

	@profile
	vertexAttrib4f(index: number, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('vertexAttrib4f');
		this._checkArgs('vertexAttrib4f', arguments);
		this.native.vertexAttrib4f(index, v0, v1, v2, v3);
	}

	@profile
	vertexAttrib4fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib4fv');
		this._checkArgs('vertexAttrib4fv', arguments);
		this.native.vertexAttrib4fv(index, value);
	}

	@profile
	vertexAttribPointer(index: number, size: number, type: number, normalized: boolean, stride: number, offset: number): void {
		this._glCheckError('vertexAttribPointer');
		this._checkArgs('vertexAttribPointer', arguments);
		this.native.vertexAttribPointer(index, size, type, !!normalized, stride, offset);
	}

	@profile
	viewport(x: number, y: number, width: number, height: number): void {
		this._glCheckError('viewport');
		this._checkArgs('viewport', arguments);
		this.native.viewport(x, y, width, height);
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
