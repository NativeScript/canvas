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
		ctor = global.CanvasJSIModule.createWebGLContext;
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
		return this.native.drawingBufferHeight;
	}

	get drawingBufferWidth() {
		return this.native.drawingBufferWidth;
	}

	__toDataURL() {
		const __toDataURL = this._getMethod('__toDataURL');
		return __toDataURL(arguments[0], arguments[1]);
	}

	@profile
	activeTexture(texture: number): void {
		this._glCheckError('activeTexture');
		this._checkArgs('activeTexture', arguments);
		const activeTexture = this._getMethod('activeTexture');
		activeTexture(texture);
	}

	@profile
	attachShader(program: WebGLProgram, shader: WebGLShader): void {
		this._glCheckError('attachShader');
		this._checkArgs('attachShader', arguments);
		const value = program.native;
		const value2 = shader.native;
		const attachShader = this._getMethod('attachShader');
		attachShader(value, value2);
	}

	@profile
	bindAttribLocation(program: WebGLProgram, index: number, name: string): void {
		this._glCheckError('bindAttribLocation');
		this._checkArgs('bindAttribLocation', arguments);
		const bindAttribLocation = this._getMethod('bindAttribLocation');
		const value = program.native;
		bindAttribLocation(value, index, name);
	}

	@profile
	bindBuffer(target: number, buffer: WebGLBuffer): void {
		this._glCheckError('bindBuffer');
		this._checkArgs('bindBuffer', arguments);
		const value = buffer ? buffer.native : null;
		const bindBuffer = this._getMethod('bindBuffer');
		bindBuffer(target, value);
	}

	@profile
	bindFramebuffer(target: number, framebuffer: WebGLFramebuffer): void {
		this._glCheckError('bindFramebuffer');
		this._checkArgs('bindFramebuffer', arguments);
		const value = framebuffer ? framebuffer.native : null;
		const bindFramebuffer = this._getMethod('bindFramebuffer');
		bindFramebuffer(target, value);
	}

	@profile
	bindRenderbuffer(target: number, renderbuffer: WebGLRenderbuffer): void {
		this._glCheckError('bindRenderbuffer');
		this._checkArgs('bindRenderbuffer', arguments);
		const value = renderbuffer ? renderbuffer.native : null;
		const bindRenderbuffer = this._getMethod('bindRenderbuffer');
		bindRenderbuffer(target, value);
	}

	@profile
	bindTexture(target: number, texture: WebGLTexture): void {
		this._glCheckError('bindTexture');
		this._checkArgs('bindTexture', arguments);
		const value = texture ? texture.native : null;
		const bindTexture = this._getMethod('bindTexture');
		bindTexture(target, value);
	}

	@profile
	blendColor(red: number, green: number, blue: number, alpha: number): void {
		this._glCheckError('blendColor');
		this._checkArgs('blendColor', arguments);
		const blendColor = this._getMethod('blendColor');
		blendColor(red, green, blue, alpha);
	}

	@profile
	blendEquationSeparate(modeRGB: number, modeAlpha: number): void {
		this._glCheckError('blendEquationSeparate');
		this._checkArgs('blendEquationSeparate', arguments);
		const blendEquationSeparate = this._getMethod('blendEquationSeparate');
		blendEquationSeparate(modeRGB, modeAlpha);
	}

	@profile
	blendEquation(mode: number): void {
		this._glCheckError('blendEquation');
		this._checkArgs('blendEquation', arguments);
		const blendEquation = this._getMethod('blendEquation');
		blendEquation(mode);
	}

	@profile
	blendFuncSeparate(srcRGB: number = this.ONE, dstRGB: number = this.ZERO, srcAlpha: number = this.ONE, dstAlpha: number = this.ZERO): void {
		this._glCheckError('blendFuncSeparate');
		this._checkArgs('blendFuncSeparate', arguments);
		const blendFuncSeparate = this._getMethod('blendFuncSeparate');
		blendFuncSeparate(srcRGB, dstRGB, srcAlpha, dstAlpha);
	}

	@profile
	blendFunc(sfactor: number = this.ONE, dfactor: number = this.ZERO): void {
		this._glCheckError('blendFunc');
		this._checkArgs('blendFunc', arguments);
		const blendFunc = this._getMethod('blendFunc');
		blendFunc(sfactor, dfactor);
	}

	bufferData(target: number, size: number, usage: number): void;

	bufferData(target: number, srcData: ArrayBuffer | ArrayBufferView, usage: number): void;

	@profile
	bufferData(target: any, srcData: any, usage: any) {
		this._glCheckError('bufferData');
		this._checkArgs('bufferData', arguments);
		const bufferData = this._getMethod('bufferData');
		bufferData(target, srcData, usage);
	}

	@profile
	bufferSubData(target: number, offset: number, srcData: ArrayBuffer | ArrayBufferView): void {
		this._glCheckError('bufferSubData');
		this._checkArgs('bufferSubData', arguments);
		const bufferSubData = this._getMethod('bufferSubData');
		bufferSubData(target, offset, srcData);
	}

	@profile
	checkFramebufferStatus(target: number): number {
		this._glCheckError('checkFramebufferStatus');
		this._checkArgs('checkFramebufferStatus', arguments);
		const checkFramebufferStatus = this._getMethod('checkFramebufferStatus');
		return checkFramebufferStatus(target);
	}

	@profile
	clearColor(red: number, green: number, blue: number, alpha: number): void {
		this._glCheckError('clearColor');
		this._checkArgs('clearColor', arguments);
		const clearColor = this._getMethod('clearColor');
		clearColor(red, green, blue, alpha);
	}

	@profile
	clearDepth(depth: number): void {
		this._glCheckError('clearDepth');
		this._checkArgs('clearDepth', arguments);
		const clearDepth = this._getMethod('clearDepth');
		clearDepth(depth);
	}

	@profile
	clearStencil(stencil: number): void {
		this._glCheckError('clearStencil');
		this._checkArgs('clearStencil', arguments);
		const clearStencil = this._getMethod('clearStencil');
		clearStencil(stencil);
	}

	@profile
	clear(mask: number): void {
		this._glCheckError('clear');
		this._checkArgs('clear', arguments);
		const clear = this._getMethod('clear');
		clear(mask);
	}

	@profile
	colorMask(red: boolean, green: boolean, blue: boolean, alpha: boolean): void {
		this._glCheckError('colorMask');
		this._checkArgs('colorMask', arguments);
		const colorMask = this._getMethod('colorMask');
		colorMask(red, green, blue, alpha);
	}

	@profile
	commit(): void {
		// NOOP
		const commit = this._getMethod('commit');
		commit();
	}

	@profile
	compileShader(shader: WebGLShader): void {
		this._glCheckError('compileShader');
		this._checkArgs('compileShader', arguments);
		const compileShader = this._getMethod('compileShader');
		const value = shader.native;
		compileShader(value);
	}

	@profile
	compressedTexImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, pixels: ArrayBuffer | ArrayBufferView): void {
		this._glCheckError('compressedTexImage2D');
		this._checkArgs('compressedTexImage2D', arguments);
		const compressedTexImage2D = this._getMethod('compressedTexImage2D');
		compressedTexImage2D(target, level, internalformat, width, height, border, pixels);
	}

	@profile
	compressedTexSubImage2D(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, pixels: ArrayBufferView): void {
		this._glCheckError('compressedTexSubImage2D');
		this._checkArgs('compressedTexSubImage2D', arguments);
		const compressedTexSubImage2D = this._getMethod('compressedTexSubImage2D');
		compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels);
	}

	@profile
	copyTexImage2D(target: number, level: number, internalformat: number, x: number, y: number, width: number, height: number, border: number): void {
		this._glCheckError('copyTexImage2D');
		this._checkArgs('copyTexImage2D', arguments);
		const copyTexImage2D = this._getMethod('copyTexImage2D');
		copyTexImage2D(target, level, internalformat, x, y, width, height, border);
	}

	@profile
	copyTexSubImage2D(target: number, level: number, xoffset: number, yoffset: number, x: number, y: number, width: number, height: number): void {
		this._glCheckError('copyTexSubImage2D');
		this._checkArgs('copyTexSubImage2D', arguments);
		const copyTexSubImage2D = this._getMethod('copyTexSubImage2D');
		copyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height);
	}

	@profile
	createBuffer(): WebGLBuffer {
		this._glCheckError('createBuffer');
		this._checkArgs('createBuffer', arguments);
		const createBuffer = this._getMethod('createBuffer');
		const id = createBuffer();
		return new WebGLBuffer(id);
	}

	@profile
	createFramebuffer(): WebGLFramebuffer {
		this._glCheckError('createFramebuffer');
		this._checkArgs('createFramebuffer', arguments);
		const createFramebuffer = this._getMethod('createFramebuffer');
		return new WebGLFramebuffer(createFramebuffer());
	}

	@profile
	createProgram(): WebGLProgram {
		this._glCheckError('createProgram');
		this._checkArgs('createProgram', arguments);
		const createProgram = this._getMethod('createProgram');
		return new WebGLProgram(createProgram());
	}

	@profile
	createRenderbuffer(): WebGLRenderbuffer {
		this._glCheckError('createRenderbuffer');
		this._checkArgs('createRenderbuffer', arguments);
		const createRenderbuffer = this._getMethod('createRenderbuffer');
		return new WebGLRenderbuffer(createRenderbuffer());
	}

	@profile
	createShader(type: number): WebGLShader {
		this._glCheckError('createShader');
		this._checkArgs('createShader', arguments);
		const createShader = this._getMethod('createShader');
		return new WebGLShader(createShader(type));
	}

	@profile
	createTexture(): WebGLTexture {
		this._glCheckError('createTexture');
		this._checkArgs('createTexture', arguments);
		const createTexture = this._getMethod('createTexture');
		return new WebGLTexture(createTexture());
	}

	@profile
	cullFace(mode: number): void {
		this._glCheckError('cullFace');
		this._checkArgs('cullFace', arguments);
		const cullFace = this._getMethod('cullFace');
		cullFace(mode);
	}

	@profile
	deleteBuffer(buffer: WebGLBuffer): void {
		this._glCheckError('deleteBuffer');
		this._checkArgs('deleteBuffer', arguments);
		const value = buffer.native;
		const deleteBuffer = this._getMethod('deleteBuffer');
		deleteBuffer(value);
	}

	@profile
	deleteFramebuffer(frameBuffer: WebGLFramebuffer): void {
		this._glCheckError('deleteFramebuffer');
		this._checkArgs('deleteFramebuffer', arguments);
		const value = frameBuffer.native;
		const deleteFramebuffer = this._getMethod('deleteFramebuffer');
		deleteFramebuffer(value);
	}

	@profile
	deleteProgram(program: WebGLProgram): void {
		this._glCheckError('deleteProgram');
		this._checkArgs('deleteProgram', arguments);
		const value = program.native;
		const deleteProgram = this._getMethod('deleteProgram');
		deleteProgram(value);
	}

	@profile
	deleteRenderbuffer(renderBuffer: WebGLRenderbuffer): void {
		this._glCheckError('deleteRenderbuffer');
		this._checkArgs('deleteRenderbuffer', arguments);
		const value = renderBuffer.native;
		const deleteRenderbuffer = this._getMethod('deleteRenderbuffer');
		deleteRenderbuffer(value);
	}

	@profile
	deleteShader(shader: WebGLRenderbuffer): void {
		this._glCheckError('deleteShader');
		this._checkArgs('deleteShader', arguments);
		const deleteShader = this._getMethod('deleteShader');
		deleteShader(shader.native);
	}

	@profile
	deleteTexture(texture: WebGLTexture): void {
		this._glCheckError('deleteTexture');
		this._checkArgs('deleteTexture', arguments);
		const value = texture.native;
		const deleteTexture = this._getMethod('deleteTexture');
		deleteTexture(value);
	}

	@profile
	depthFunc(func: number): void {
		this._glCheckError('depthFunc');
		this._checkArgs('depthFunc', arguments);
		const depthFunc = this._getMethod('depthFunc');
		depthFunc(func);
	}

	@profile
	depthMask(flag: boolean): void {
		this._glCheckError('depthMask');
		this._checkArgs('depthMask', arguments);
		const depthMask = this._getMethod('depthMask');
		depthMask(flag);
	}

	@profile
	depthRange(zNear: number, zFar: number): void {
		this._glCheckError('depthRange');
		this._checkArgs('depthRange', arguments);
		const depthRange = this._getMethod('depthRange');
		depthRange(zNear, zFar);
	}

	@profile
	detachShader(program: WebGLProgram, shader: WebGLShader): void {
		this._glCheckError('detachShader');
		this._checkArgs('detachShader', arguments);
		const value = program.native;
		const value2 = shader.native;
		const detachShader = this._getMethod('detachShader');
		detachShader(value, value2);
	}

	@profile
	disableVertexAttribArray(index: number): void {
		this._glCheckError('disableVertexAttribArray');
		this._checkArgs('disableVertexAttribArray', arguments);
		const disableVertexAttribArray = this._getMethod('disableVertexAttribArray');
		disableVertexAttribArray(index);
	}

	@profile
	disable(cap: number): void {
		this._glCheckError('disable');
		this._checkArgs('disable', arguments);
		const disable = this._getMethod('disable');
		disable(cap);
	}

	@profile
	drawArrays(mode: number, first: number, count: number): void {
		this._glCheckError('drawArrays');
		this._checkArgs('drawArrays', arguments);
		const drawArrays = this._getMethod('drawArrays');
		drawArrays(mode, first, count);
	}

	@profile
	drawElements(mode: number, count: number, type: number, offset: number): void {
		this._glCheckError('drawElements');
		this._checkArgs('drawElements', arguments);
		const drawElements = this._getMethod('drawElements');
		drawElements(mode, count, type, offset);
	}

	@profile
	enableVertexAttribArray(index: number): void {
		this._glCheckError('enableVertexAttribArray');
		this._checkArgs('enableVertexAttribArray', arguments);
		const enableVertexAttribArray = this._getMethod('enableVertexAttribArray');
		enableVertexAttribArray(index);
	}

	@profile
	enable(cap: number): void {
		this._glCheckError('enable');
		this._checkArgs('enable', arguments);
		const enable = this._getMethod('enable');
		enable(cap);
	}

	@profile
	finish(): void {
		this._glCheckError('finish');
		const finish = this._getMethod('finish');
		finish();
	}

	@profile
	flush(): void {
		this._glCheckError('flush');
		const flush = this._getMethod('flush');
		flush();
	}

	@profile
	framebufferRenderbuffer(target: number, attachment: number, renderbuffertarget: number, renderbuffer: WebGLRenderbuffer): void {
		this._glCheckError('framebufferRenderbuffer');
		this._checkArgs('framebufferRenderbuffer', arguments);
		const value = renderbuffer.native;
		const framebufferRenderbuffer = this._getMethod('framebufferRenderbuffer');
		framebufferRenderbuffer(target, attachment, renderbuffertarget, value);
	}

	@profile
	framebufferTexture2D(target: number, attachment: number, textarget: number, texture: WebGLTexture, level: number): void {
		this._glCheckError('framebufferTexture2D');
		this._checkArgs('framebufferTexture2D', arguments);
		const value = texture.native;
		const framebufferTexture2D = this._getMethod('framebufferTexture2D');
		framebufferTexture2D(target, attachment, textarget, value, level);
	}

	@profile
	frontFace(mode: number): void {
		this._checkArgs('frontFace', arguments);
		this._glCheckError('frontFace');
		const frontFace = this._getMethod('frontFace');
		frontFace(mode);
	}

	@profile
	generateMipmap(target: number): void {
		this._checkArgs('generateMipmap', arguments);
		this._glCheckError('generateMipmap');
		const generateMipmap = this._getMethod('generateMipmap');
		generateMipmap(target);
	}

	@profile
	getActiveAttrib(program: WebGLProgram, index: number): WebGLActiveInfo {
		this._glCheckError('getActiveAttrib');
		this._checkArgs('getActiveAttrib', arguments);
		const value = program.native;
		const getActiveAttrib = this._getMethod('getActiveAttrib');
		const attrib = getActiveAttrib(value, index);
		return new WebGLActiveInfo(attrib);
	}

	@profile
	getActiveUniform(program: WebGLProgram, index: number): WebGLActiveInfo {
		this._glCheckError('getActiveUniform');
		this._checkArgs('getActiveUniform', arguments);
		const value = program.native;
		const getActiveUniform = this._getMethod('getActiveUniform');
		const uniform = getActiveUniform(value, index);
		return new WebGLActiveInfo(uniform);
	}

	@profile
	getAttachedShaders(program: WebGLProgram): WebGLShader[] {
		this._glCheckError('getAttachedShaders');
		this._checkArgs('getAttachedShaders', arguments);
		const getAttachedShaders = this._getMethod('getAttachedShaders');
		const value = program.native;
		return getAttachedShaders(value).map((shader) => new WebGLShader(shader));
	}

	@profile
	getAttribLocation(program: WebGLProgram, name: string): number {
		this._glCheckError('getAttribLocation');
		this._checkArgs('getAttribLocation', arguments);
		const getAttribLocation = this._getMethod('getAttribLocation');
		const value = program.native;
		return getAttribLocation(value, name);
	}

	@profile
	getBufferParameter(target: number, pname: number): number {
		this._glCheckError('getBufferParameter');
		this._checkArgs('getBufferParameter', arguments);
		const getBufferParameter = this._getMethod('getBufferParameter');
		return getBufferParameter(target, pname);
	}

	@profile
	getContextAttributes(): any {
		const getContextAttributes = this._getMethod('getContextAttributes');
		return getContextAttributes();
	}

	@profile
	getError(): number {
		const getError = this._getMethod('getError');
		return getError();
	}

	@profile
	getExtension(name: string) {
		this._glCheckError('getExtension');
		this._checkArgs('getExtension', arguments);
		if (name === 'EXT_disjoint_timer_query_webgl2') {
			return null;
		}
		const getExtension = this._getMethod('getExtension');

		const ext = getExtension(name);
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

		const getFramebufferAttachmentParameter = this._getMethod('getFramebufferAttachmentParameter');

		const param = getFramebufferAttachmentParameter(target, attachment, pname);
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
		const getParameter = this._getMethod('getParameter');
		const value = getParameter(pname);
		return this._handleGetParameter(pname, value);
	}

	@profile
	getProgramInfoLog(program: WebGLProgram): string {
		this._glCheckError('getProgramInfoLog');
		this._checkArgs('getProgramInfoLog', arguments);
		const getProgramInfoLog = this._getMethod('getProgramInfoLog');
		const value = program.native;
		return getProgramInfoLog(value);
	}

	@profile
	getProgramParameter(program: WebGLProgram, pname: number): number | boolean {
		this._glCheckError('getProgramParameter');
		this._checkArgs('getProgramParameter', arguments);
		const getProgramParameter = this._getMethod('getProgramParameter');
		const value = program.native;
		const result = getProgramParameter(value, pname);
		return result;
	}

	@profile
	getRenderbufferParameter(target: number, pname: number): number {
		this._glCheckError('getRenderbufferParameter');
		this._checkArgs('getRenderbufferParameter', arguments);
		const getRenderbufferParameter = this._getMethod('getRenderbufferParameter');
		return getRenderbufferParameter(target, pname);
	}

	@profile
	getShaderInfoLog(shader: WebGLShader): string {
		this._glCheckError('getShaderInfoLog');
		this._checkArgs('getShaderInfoLog', arguments);
		const getShaderInfoLog = this._getMethod('getShaderInfoLog');
		const value = shader.native;
		return getShaderInfoLog(value);
	}

	@profile
	getShaderParameter(shader: WebGLShader, pname: number): boolean | number {
		this._glCheckError('getShaderParameter');
		this._checkArgs('getShaderParameter', arguments);
		const getShaderParameter = this._getMethod('getShaderParameter');
		const value = shader.native;
		const result = getShaderParameter(value, pname);
		return result;
	}

	@profile
	getShaderPrecisionFormat(shaderType: number, precisionType: number): WebGLShaderPrecisionFormat {
		this._glCheckError('getShaderPrecisionFormat');
		this._checkArgs('getShaderPrecisionFormat', arguments);
		const getShaderPrecisionFormat = this._getMethod('getShaderPrecisionFormat');
		const precision = getShaderPrecisionFormat(shaderType, precisionType);
		return new WebGLShaderPrecisionFormat(precision);
	}

	@profile
	getShaderSource(shader: WebGLShader): string {
		this._glCheckError('getShaderSource');
		this._checkArgs('getShaderSource', arguments);
		const getShaderSource = this._getMethod('getShaderSource');
		const value = shader ? shader.native : 0;
		return getShaderSource(value);
	}

	@profile
	getSupportedExtensions(): string[] {
		this._glCheckError('getSupportedExtensions');
		const getSupportedExtensions = this._getMethod('getSupportedExtensions');
		return getSupportedExtensions();
	}

	@profile
	getTexParameter(target: number, pname: number): number {
		this._glCheckError('getTexParameter');
		this._checkArgs('getTexParameter', arguments);
		const getTexParameter = this._getMethod('getTexParameter');
		return getTexParameter(target, pname);
	}

	@profile
	getUniformLocation(program: WebGLProgram, name: string): WebGLUniformLocation {
		this._glCheckError('getUniformLocation');
		this._checkArgs('getUniformLocation', arguments);
		const getUniformLocation = this._getMethod('getUniformLocation');
		const value = program.native;
		const id = getUniformLocation(value, name);
		if (id === -1) {
			return null;
		}
		return new WebGLUniformLocation(id);
	}

	@profile
	getUniform(program: WebGLProgram, location: WebGLUniformLocation): any {
		this._glCheckError('getUniform');
		this._checkArgs('getUniform', arguments);
		const getUniform = this._getMethod('getUniform');
		const value = program.native;
		const value2 = location.native;
		const uniform = getUniform(value, value2);
		if (uniform && uniform.length) {
			return uniform;
		}
		return uniform;
	}

	@profile
	getVertexAttribOffset(index: number, pname: number): number {
		this._glCheckError('getVertexAttribOffset');
		this._checkArgs('getVertexAttribOffset', arguments);
		const getVertexAttribOffset = this._getMethod('getVertexAttribOffset');
		return getVertexAttribOffset(index, pname);
	}

	@profile
	getVertexAttrib(index: number, pname: number): number[] | boolean | number | Float32Array {
		this._glCheckError('getVertexAttrib');
		this._checkArgs('getVertexAttrib', arguments);
		const getVertexAttrib = this._getMethod('getVertexAttrib');
		const value = getVertexAttrib(index, pname);
		if (pname === this.CURRENT_VERTEX_ATTRIB) {
			return value;
		}
		return value;
	}

	@profile
	hint(target: number, mode: number): void {
		this._glCheckError('hint');
		this._checkArgs('hint', arguments);
		const hint = this._getMethod('hint');
		hint(target, mode);
	}

	@profile
	isBuffer(buffer: WebGLBuffer): boolean {
		this._glCheckError('isBuffer');
		this._checkArgs('isBuffer', arguments);
		const isBuffer = this._getMethod('isBuffer');
		const value = buffer.native;
		return isBuffer(value);
	}

	@profile
	isContextLost(): boolean {
		this._glCheckError('isContextLost');
		const isContextLost = this._getMethod('isContextLost');
		return isContextLost();
	}

	@profile
	isEnabled(cap: number): boolean {
		this._glCheckError('isEnabled');
		this._checkArgs('isEnabled', arguments);
		const isEnabled = this._getMethod('isEnabled');
		return isEnabled(cap);
	}

	@profile
	isFramebuffer(framebuffer: WebGLFramebuffer): boolean {
		this._glCheckError('isFramebuffer');
		this._checkArgs('isFramebuffer', arguments);
		const isFramebuffer = this._getMethod('isFramebuffer');
		const value = framebuffer ? framebuffer.native : 0;
		return isFramebuffer(value);
	}

	@profile
	isProgram(program: WebGLProgram): boolean {
		this._glCheckError('isProgram');
		this._checkArgs('isProgram', arguments);
		const isProgram = this._getMethod('isProgram');
		const value = program.native;
		return isProgram(value);
	}

	@profile
	isRenderbuffer(renderbuffer: WebGLRenderbuffer): boolean {
		this._glCheckError('isRenderbuffer');
		this._checkArgs('isRenderbuffer', arguments);
		const isRenderbuffer = this._getMethod('isRenderbuffer');
		const value = renderbuffer.native;
		return isRenderbuffer(value);
	}

	@profile
	isShader(shader: WebGLShader): boolean {
		this._glCheckError('isShader');
		this._checkArgs('isShader', arguments);
		const isShader = this._getMethod('isShader');
		const value = shader.native;
		return isShader(value);
	}

	@profile
	isTexture(texture: WebGLTexture): boolean {
		this._glCheckError('isTexture');
		this._checkArgs('isTexture', arguments);
		const isTexture = this._getMethod('isTexture');
		const value = texture.native;
		return isTexture(value);
	}

	@profile
	lineWidth(width: number): void {
		this._glCheckError('lineWidth');
		this._checkArgs('lineWidth', arguments);
		const lineWidth = this._getMethod('lineWidth');
		lineWidth(width);
	}

	@profile
	linkProgram(program: WebGLProgram): void {
		this._glCheckError('linkProgram');
		this._checkArgs('linkProgram', arguments);
		const linkProgram = this._getMethod('linkProgram');
		const value = program ? program.native : 0;
		linkProgram(value);
	}

	_flipY = false;
	@profile
	pixelStorei(pname: number, param: any): void {
		this._glCheckError('pixelStorei');
		this._checkArgs('pixelStorei', arguments);
		const pixelStorei = this._getMethod('pixelStorei');
		if (pname === this.UNPACK_FLIP_Y_WEBGL || pname === this.UNPACK_PREMULTIPLY_ALPHA_WEBGL) {
			if (pname === this.UNPACK_FLIP_Y_WEBGL) {
				this._flipY = param;
			}
			pixelStorei(pname, param);
		} else if (pname === this.PACK_ALIGNMENT || pname === this.UNPACK_ALIGNMENT || pname === this.UNPACK_COLORSPACE_CONVERSION_WEBGL) {
			if (pname === this.UNPACK_COLORSPACE_CONVERSION_WEBGL) {
				param = 0x9244;
			} else if (pname === this.PACK_ALIGNMENT || pname === this.UNPACK_ALIGNMENT) {
				param = 4;
			}
			pixelStorei(pname, param);
		} else {
			pixelStorei(pname, param);
		}
	}

	@profile
	polygonOffset(factor: number, units: number): void {
		this._glCheckError('polygonOffset');
		this._checkArgs('polygonOffset', arguments);
		const polygonOffset = this._getMethod('polygonOffset');
		polygonOffset(factor, units);
	}

	@profile
	readPixels(x: number, y: number, width: number, height: number, format: number, type: number, pixels: ArrayBuffer | ArrayBufferView): void {
		this._glCheckError('readPixels');
		this._checkArgs('readPixels', arguments);
		const readPixels = this._getMethod('readPixels');
		readPixels(x, y, width, height, format, type, pixels);
	}

	@profile
	renderbufferStorage(target: number, internalFormat: number, width: number, height: number): void {
		this._glCheckError('renderbufferStorage');
		this._checkArgs('renderbufferStorage', arguments);
		const renderbufferStorage = this._getMethod('renderbufferStorage');
		if (internalFormat === this.DEPTH_STENCIL) {
			// DEPTH24_STENCIL8 = 35056
			// DEPTH24_STENCIL8_OES = 0x88F0
			internalFormat = 0x88f0;
		}
		renderbufferStorage(target, internalFormat, width, height);
	}

	@profile
	sampleCoverage(value: number, invert: boolean): void {
		this._glCheckError('sampleCoverage');
		this._checkArgs('sampleCoverage', arguments);
		const sampleCoverage = this._getMethod('sampleCoverage');
		sampleCoverage(value, invert);
	}

	@profile
	scissor(x: number, y: number, width: number, height: number): void {
		this._glCheckError('scissor');
		this._checkArgs('scissor', arguments);
		const scissor = this._getMethod('scissor');
		scissor(x, y, width, height);
	}

	@profile
	shaderSource(shader: WebGLShader, source: string): void {
		this._glCheckError('shaderSource');
		this._checkArgs('shaderSource', arguments);
		const shaderSource = this._getMethod('shaderSource');
		const value = shader ? shader.native : 0;
		shaderSource(value, source);
	}

	@profile
	stencilFuncSeparate(face: number, func: number, ref: number, mask: number): void {
		this._glCheckError('stencilFuncSeparate');
		this._checkArgs('stencilFuncSeparate', arguments);
		const stencilFuncSeparate = this._getMethod('stencilFuncSeparate');
		stencilFuncSeparate(face, func, ref, mask);
	}

	@profile
	stencilFunc(func: number, ref: number, mask: number): void {
		this._glCheckError('stencilFunc');
		this._checkArgs('stencilFunc', arguments);
		const stencilFunc = this._getMethod('stencilFunc');
		stencilFunc(func, ref, mask);
	}

	@profile
	stencilMaskSeparate(face: number, mask: number): void {
		this._glCheckError('stencilMaskSeparate');
		this._checkArgs('stencilMaskSeparate', arguments);
		const stencilMaskSeparate = this._getMethod('stencilMaskSeparate');
		stencilMaskSeparate(face, mask);
	}

	@profile
	stencilMask(mask: number): void {
		this._glCheckError('stencilMask');
		this._checkArgs('stencilMask', arguments);
		const stencilMask = this._getMethod('stencilMask');
		stencilMask(mask);
	}

	@profile
	stencilOpSeparate(face: number, fail: number, zfail: number, zpass: number): void {
		this._glCheckError('stencilOpSeparate');
		this._checkArgs('stencilOpSeparate', arguments);
		const stencilOpSeparate = this._getMethod('stencilOpSeparate');
		stencilOpSeparate(face, fail, zfail, zpass);
	}

	@profile
	stencilOp(fail: number, zfail: number, zpass: number): void {
		this._glCheckError('stencilOp');
		this._checkArgs('stencilOp', arguments);
		const stencilOp = this._getMethod('stencilOp');
		stencilOp(fail, zfail, zpass);
	}

	texImage2D(target: number, level: number, internalformat: number, format: number, type: number, pixels: any): void;

	texImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type: number, pixels: ArrayBufferView): void;

	@profile
	texImage2D(target: any, level: any, internalformat: any, width: any, height: any, border: any, format?: any, type?: any, pixels?: any) {
		this._glCheckError('texImage2D');
		this._checkArgs('texImage2D', arguments);
		const texImage2D = this._getMethod('texImage2D');
		if (arguments.length === 9) {
			texImage2D(target, level, internalformat, width, height, border, format, type ?? internalformat, pixels);
		} else if (arguments.length === 6) {
			if (border && typeof border.tagName === 'string' && (border.tagName === 'VID' || border.tagName === 'VIDEO') && border._video && typeof border._video.getCurrentFrame === 'function') {
				border._video.getCurrentFrame(this.native, this, target, level, internalformat, width, height);
			} else if (border && typeof border.getCurrentFrame === 'function') {
				border.getCurrentFrame(this.native, this, target, level, internalformat, width, height);
			} else if (border instanceof ImageAsset) {
				texImage2D(target, level, internalformat, width, height, border.native);
			} else if (border instanceof ImageBitmap) {
				texImage2D(target, level, internalformat, width, height, border.native);
			} else if (global.isAndroid && border instanceof android.graphics.Bitmap) {
				// todo ios
				texImage2D(target, level, internalformat, width, height, border);
			} else if (border instanceof ImageSource) {
				texImage2D(target, level, internalformat, width, height, border.android);
			} else if (border && typeof border.tagName === 'string' && (border.tagName === 'IMG' || border.tagName === 'IMAGE')) {
				if (border._asset instanceof ImageAsset) {
					texImage2D(target, level, internalformat, width, height, border._asset.native);
				} else if (border._imageSource instanceof ImageSource) {
					texImage2D(target, level, internalformat, width, height, border._imageSource.android);
				} else if (global.isAndroid && border._image instanceof android.graphics.Bitmap) {
					//todo ios
					texImage2D(target, level, internalformat, width, height, border._image);
				} else if (typeof border.src === 'string') {
					texImage2D(target, level, internalformat, width, height, ImageSource.fromFileSync(border.src).android);
				}
			} else if (border && typeof border.tagName === 'string' && border.tagName === 'CANVAS' && border._canvas instanceof Canvas) {
				texImage2D(target, level, internalformat, width, height, border._canvas.native);
			}
		}
	}

	@profile
	texParameterf(target: number, pname: number, param: number): void {
		this._glCheckError('texParameterf');
		this._checkArgs('texParameterf', arguments);
		const texParameterf = this._getMethod('texParameterf');
		texParameterf(target, pname, param);
	}

	@profile
	texParameteri(target: number, pname: number, param: number): void {
		this._glCheckError('texParameteri');
		this._checkArgs('texParameteri', arguments);
		const texParameteri = this._getMethod('texParameteri');
		texParameteri(target, pname, param);
	}

	texSubImage2D(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, type: number, pixels: ArrayBufferView): void;

	texSubImage2D(target: number, level: number, xoffset: number, yoffset: number, format: number, type: number, pixels: any): void;

	@profile
	texSubImage2D(target: any, level: any, xoffset: any, yoffset: any, width: any, height: any, format: any, type?: any, pixels?: any) {
		this._glCheckError('texSubImage2D');
		this._checkArgs('texSubImage2D', arguments);
		const texSubImage2D = this._getMethod('texSubImage2D');
		if (arguments.length === 9) {
			texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels);
		} else if (arguments.length === 7) {
			if (global.isAndroid && format instanceof android.graphics.Bitmap) {
				// todo
				texSubImage2D(target, level, xoffset, yoffset, width, height, format);
			} else if (format instanceof ImageSource) {
				texSubImage2D(target, level, xoffset, yoffset, width, height, format.android);
			} else if (format instanceof ImageAsset) {
				texSubImage2D(target, level, xoffset, yoffset, width, height, format.native);
			} else if (format instanceof ImageBitmap) {
				texSubImage2D(target, level, xoffset, yoffset, width, height, format.native);
			} else if (format && typeof format.tagName === 'string' && (format.tagName === 'IMG' || format.tagName === 'IMAGE')) {
				if (format._imageSource instanceof ImageSource) {
					texSubImage2D(target, level, xoffset, yoffset, width, height, format._imageSource.android);
				} else if (global.isAndroid && format._image instanceof android.graphics.Bitmap) {
					// todo
					texSubImage2D(target, level, xoffset, yoffset, width, height, format._image);
				} else if (format._asset instanceof ImageAsset) {
					texSubImage2D(target, level, xoffset, yoffset, width, height, format._asset.native);
				} else if (typeof format.src === 'string') {
					const result = ImageSource.fromFileSync(format.src);
					texSubImage2D(target, level, xoffset, yoffset, width, height, result ? result.android : null);
				} else if (format && typeof format.tagName === 'string' && format.tagName === 'CANVAS' && format._canvas instanceof Canvas) {
					texSubImage2D(target, level, xoffset, yoffset, width, height, format._canvas.native);
				}
			}
		}
	}

	@profile
	uniform1f(location: WebGLUniformLocation, v0: number): void {
		this._glCheckError('uniform1f');
		this._checkArgs('uniform1f', arguments);
		const uniform1f = this._getMethod('uniform1f');
		const loc = location.native;
		uniform1f(loc, v0);
	}

	@profile
	uniform1iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform1iv');
		this._checkArgs('uniform1iv', arguments);
		const loc = location.native;
		const uniform1iv = this._getMethod('uniform1iv');
		uniform1iv(loc, value);
	}

	@profile
	uniform1fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform1fv');
		this._checkArgs('uniform1fv', arguments);
		const loc = location.native;
		const uniform1fv = this._getMethod('uniform1fv');
		uniform1fv(loc, value);
	}

	@profile
	uniform1i(location: WebGLUniformLocation, v0: number): void {
		this._glCheckError('uniform1i');
		this._checkArgs('uniform1i', arguments);
		const uniform1i = this._getMethod('uniform1i');
		const loc = location.native;
		uniform1i(loc, Number(v0));
	}

	@profile
	uniform2f(location: WebGLUniformLocation, v0: number, v1: number): void {
		this._glCheckError('uniform2f');
		this._checkArgs('uniform2f', arguments);
		const uniform2f = this._getMethod('uniform2f');
		const loc = location.native;
		uniform2f(loc, v0, v1);
	}

	@profile
	uniform2iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform2iv');
		this._checkArgs('uniform2iv', arguments);

		const loc = location.native;
		const uniform2iv = this._getMethod('uniform2iv');
		uniform2iv(loc, value);
	}

	@profile
	uniform2fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform2fv');
		this._checkArgs('uniform2fv', arguments);

		const loc = location.native;
		const uniform2fv = this._getMethod('uniform2fv');
		uniform2fv(loc, value);
	}

	@profile
	uniform2i(location: WebGLUniformLocation, v0: number, v1: number): void {
		this._glCheckError('uniform2i');
		this._checkArgs('uniform2i', arguments);
		const uniform2i = this._getMethod('uniform2i');
		const loc = location.native;
		uniform2i(loc, v0, v1);
	}

	@profile
	uniform3f(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void {
		this._glCheckError('uniform3f');
		this._checkArgs('uniform3f', arguments);
		const uniform3f = this._getMethod('uniform3f');
		const loc = location.native;
		uniform3f(loc, v0, v1, v2);
	}

	@profile
	uniform3iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform3iv');
		this._checkArgs('uniform3iv', arguments);

		const loc = location.native;
		const uniform3iv = this._getMethod('uniform3iv');
		uniform3iv(loc, value);
	}

	@profile
	uniform3fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform3fv');
		this._checkArgs('uniform3fv', arguments);

		const loc = location.native;
		const uniform3fv = this._getMethod('uniform3fv');
		uniform3fv(loc, value);
	}

	@profile
	uniform3i(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void {
		this._glCheckError('uniform3i');
		this._checkArgs('uniform3i', arguments);
		const uniform3i = this._getMethod('uniform3i');
		const loc = location.native;
		uniform3i(loc, v0, v1, v2);
	}

	@profile
	uniform4f(location: WebGLUniformLocation, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('uniform4f');
		this._checkArgs('uniform4f', arguments);
		const uniform4f = this._getMethod('uniform4f');
		const loc = location.native;
		uniform4f(loc, v0, v1, v2, v3);
	}

	@profile
	uniform4iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform4iv');
		this._checkArgs('uniform4iv', arguments);

		const loc = location.native;
		const uniform4iv = this._getMethod('uniform4iv');
		uniform4iv(loc, value);
	}

	@profile
	uniform4fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform4fv');
		this._checkArgs('uniform4fv', arguments);

		const loc = location.native;
		const uniform4fv = this._getMethod('uniform4fv');
		uniform4fv(loc, value);
	}

	@profile
	uniform4i(location: WebGLUniformLocation, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('uniform4i');
		this._checkArgs('uniform4i', arguments);
		const uniform4i = this._getMethod('uniform4i');
		const loc = location.native;
		uniform4i(loc, v0, v1, v2, v3);
	}

	@profile
	uniformMatrix2fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void {
		this._glCheckError('uniformMatrix2fv');
		this._checkArgs('uniformMatrix2fv', arguments);

		const loc = location.native;
		const uniformMatrix2fv = this._getMethod('uniformMatrix2fv');
		uniformMatrix2fv(loc, transpose, value);
	}

	@profile
	uniformMatrix3fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void {
		this._glCheckError('uniformMatrix3fv');
		this._checkArgs('uniformMatrix3fv', arguments);

		const loc = location.native;
		const uniformMatrix3fv = this._getMethod('uniformMatrix3fv');
		uniformMatrix3fv(loc, transpose, value);
	}

	@profile
	uniformMatrix4fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void {
		this._glCheckError('uniformMatrix4fv');
		this._checkArgs('uniformMatrix4fv', arguments);

		const loc = location.native;
		const uniformMatrix4fv = this._getMethod('uniformMatrix4fv');
		uniformMatrix4fv(loc, transpose, value);
	}

	@profile
	useProgram(program: WebGLProgram): void {
		this._glCheckError('useProgram');
		this._checkArgs('useProgram', arguments);
		const useProgram = this._getMethod('useProgram');
		const value = program ? program.native : null;
		useProgram(value);
	}

	@profile
	validateProgram(program: WebGLProgram): void {
		this._glCheckError('validateProgram');
		this._checkArgs('validateProgram', arguments);
		const validateProgram = this._getMethod('validateProgram');
		const value = program.native;
		validateProgram(value);
	}

	@profile
	vertexAttrib1f(index: number, v0: number): void {
		this._glCheckError('vertexAttrib1f');
		this._checkArgs('vertexAttrib1f', arguments);
		const vertexAttrib1f = this._getMethod('vertexAttrib1f');
		vertexAttrib1f(index, v0);
	}

	@profile
	vertexAttrib1fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib1fv');
		this._checkArgs('vertexAttrib1fv', arguments);

		const vertexAttrib1fv = this._getMethod('vertexAttrib1fv');
		vertexAttrib1fv(index, value);
	}

	@profile
	vertexAttrib2f(index: number, v0: number, v1: number): void {
		this._glCheckError('vertexAttrib2f');
		this._checkArgs('vertexAttrib2f', arguments);
		const vertexAttrib2f = this._getMethod('vertexAttrib2f');
		vertexAttrib2f(index, v0, v1);
	}

	@profile
	vertexAttrib2fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib2fv');
		this._checkArgs('vertexAttrib2fv', arguments);

		const vertexAttrib2fv = this._getMethod('vertexAttrib2fv');
		vertexAttrib2fv(index, value);
	}

	@profile
	vertexAttrib3f(index: number, v0: number, v1: number, v2: number): void {
		this._glCheckError('vertexAttrib3f');
		this._checkArgs('vertexAttrib3f', arguments);
		const vertexAttrib3f = this._getMethod('vertexAttrib3f');
		vertexAttrib3f(index, v0, v1, v2);
	}

	@profile
	vertexAttrib3fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib3fv');
		this._checkArgs('vertexAttrib3fv', arguments);

		const vertexAttrib3fv = this._getMethod('vertexAttrib3fv');
		vertexAttrib3fv(index, value);
	}

	@profile
	vertexAttrib4f(index: number, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('vertexAttrib4f');
		this._checkArgs('vertexAttrib4f', arguments);
		const vertexAttrib4f = this._getMethod('vertexAttrib4f');
		vertexAttrib4f(index, v0, v1, v2, v3);
	}

	@profile
	vertexAttrib4fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib4fv');
		this._checkArgs('vertexAttrib4fv', arguments);

		const vertexAttrib4fv = this._getMethod('vertexAttrib4fv');
		vertexAttrib4fv(index, value);
	}

	@profile
	vertexAttribPointer(index: number, size: number, type: number, normalized: boolean, stride: number, offset: number): void {
		this._glCheckError('vertexAttribPointer');
		this._checkArgs('vertexAttribPointer', arguments);
		const vertexAttribPointer = this._getMethod('vertexAttribPointer');
		vertexAttribPointer(index, size, type, normalized, stride, offset);
	}

	@profile
	viewport(x: number, y: number, width: number, height: number): void {
		this._glCheckError('viewport');
		this._checkArgs('viewport', arguments);
		const viewport = this._getMethod('viewport');
		viewport(x, y, width, height);
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
