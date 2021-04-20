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

import { ImageSource, Screen } from '@nativescript/core';
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

export class WebGLRenderingContext extends WebGLRenderingContextBase {
	public static isDebug = false;
	public static filter: 'both' | 'error' | 'args' = 'both';
	private context: org.nativescript.canvas.TNSWebGLRenderingContext;

	constructor(context) {
		super(context);
		this.context = context;
	}

	get native() {
		return this.context;
	}

	get drawingBufferHeight() {
		return this.context.getDrawingBufferHeight();
	}

	get drawingBufferWidth() {
		return this.context.getDrawingBufferWidth();
	}

	static toPrimitive(value): any {
		if (value instanceof java.lang.Integer) {
			return value.intValue();
		} else if (value instanceof java.lang.Long) {
			return value.longValue();
		} else if (value instanceof java.lang.Short) {
			return value.shortValue();
		} else if (value instanceof java.lang.Byte) {
			return value.byteValue();
		} else if (value instanceof java.lang.Boolean) {
			return value.booleanValue();
		} else if (value instanceof java.lang.String) {
			return value.toString();
		} else if (value instanceof java.lang.Float) {
			return value.floatValue();
		} else if (value instanceof java.lang.Double) {
			return value.doubleValue();
		}
		return value;
	}

	getJSArray(value): any[] {
		const count = value.length;
		const array: number[] = [];
		for (let i = 0; i < count; i++) {
			array.push(value[i]);
		}
		return array as any;
	}

	activeTexture(texture: number): void {
		this._glCheckError('activeTexture');
		this._checkArgs('activeTexture', arguments);
		this.context.activeTexture(texture);
	}

	attachShader(program: WebGLProgram, shader: WebGLShader): void {
		this._glCheckError('attachShader');
		this._checkArgs('attachShader', arguments);
		const value = program ? program.native : 0;
		const value2 = shader ? shader.native : 0;
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
		let value = buffer ? buffer.native : 0;
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

	_lastTexture: {
		target: number;
		texture: number;
	} = { target: 0, texture: 0 };
	bindTexture(target: number, texture: WebGLTexture): void {
		this._glCheckError('bindTexture');
		this._checkArgs('bindTexture', arguments);
		const value = texture ? texture.native : 0;
		if (value > 0) {
			this._lastTexture = {
				target,
				texture: value,
			};
		}

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

	toNativeArray(value: any, type: string): any {
		// if (value && value.length && typeof type === 'string') {
		//     const size = value.length;
		//     const nativeArray = Array.create(type, size);
		//     for (let i = 0; i < size; i++) {
		//         nativeArray[i] = value[i];
		//     }
		//     return nativeArray;
		// }

		let length = 0;
		if (value instanceof Int8Array || value instanceof Uint8Array || value instanceof Uint8ClampedArray || value instanceof Int16Array || value instanceof Uint16Array || value instanceof Int32Array || value instanceof Uint32Array || value instanceof Float32Array || value instanceof Float64Array) {
			length = value.length;
		} else if (Array.isArray(value)) {
			length = value.length;
		}

		if (typeof type === 'string') {
			const array = Array.from(value as any) as any;
			let nativeArray;
			switch (type) {
				case 'byte':
					nativeArray = java.nio.ByteBuffer.wrap(array).array();
					break;
				case 'short':
					nativeArray = java.nio.ShortBuffer.wrap(array).array();
					break;
				case 'float':
					nativeArray = java.nio.FloatBuffer.wrap(array).array();
					break;
				case 'int':
					nativeArray = java.nio.IntBuffer.wrap(array).array();
					break;
				case 'double':
					nativeArray = java.nio.DoubleBuffer.wrap(array).array();
					break;
				case 'long':
					nativeArray = java.nio.LongBuffer.wrap(array).array();
					break;
				default:
					nativeArray = array;
					break;
			}
			return nativeArray as any;
		}
		return [] as any;
	}

	bufferData(target: number, size: number, usage: number): void;

	bufferData(target: number, srcData: ArrayBuffer | ArrayBufferView, usage: number): void;

	bufferData(target: any, srcData: any, usage: any) {
		this._glCheckError('bufferData');
		this._checkArgs('bufferData', arguments);
		if (typeof srcData === 'number') {
			this.context.bufferData(target, srcData, usage);
		} else if (srcData instanceof ArrayBuffer) {
			// @ts-ignore
			if (srcData.nativeObject) {
				// @ts-ignore
				this.context.bufferData(target, srcData.nativeObject, usage);
			} else {
				this.context.bufferDataByte(target, Array.from(new Uint8Array(srcData as any)), usage);
			}
			//this.context.bufferData(target, this.toNativeArray(new Uint8Array(srcData as any) as any, 'byte'), usage);
		} else if (srcData && srcData.buffer instanceof ArrayBuffer) {
			if (srcData instanceof Uint8Array || srcData instanceof Uint8ClampedArray) {
				//this.context.bufferData(target, this.toNativeArray(srcData as any, 'byte'), usage);
				this.context.bufferDataByte(target, Array.from(srcData), usage);
			} else if (srcData instanceof Uint16Array || srcData instanceof Int16Array) {
				//this.context.bufferData(target, this.toNativeArray(srcData as any, 'short'), usage);
				this.context.bufferDataShort(target, Array.from(srcData), usage);
			} else if (srcData instanceof Uint32Array || srcData instanceof Int32Array) {
				//this.context.bufferData(target, this.toNativeArray(srcData as any, 'int'), usage);
				this.context.bufferDataInt(target, Array.from(srcData), usage);
			} else if (srcData instanceof Float32Array) {
				//this.context.bufferData(target, this.toNativeArray(srcData as any, 'float'), usage);
				this.context.bufferDataFloat(target, Array.from(srcData), usage);
			}
		} else if (arguments.length === 3 && !srcData) {
			this.context.bufferData(target, 0, usage);
		}
	}

	bufferSubData(target: number, offset: number, srcData: ArrayBuffer | ArrayBufferView): void {
		this._glCheckError('bufferSubData');
		this._checkArgs('bufferSubData', arguments);
		if (srcData instanceof ArrayBuffer) {
			//this.context.bufferSubData(target, offset, this.toNativeArray(new Uint8Array(srcData as any) as any, 'byte'));
			// @ts-ignore
			if (srcData.nativeObject) {
				// @ts-ignore
				this.context.bufferSubData(target, offset, srcData.nativeObject);
			} else {
				this.context.bufferSubDataByte(target, offset, Array.from(new Uint8Array(srcData as any)));
			}
		} else if (srcData && srcData.buffer instanceof ArrayBuffer) {
			if (srcData instanceof Uint8Array || srcData instanceof Uint8ClampedArray) {
				//this.context.bufferSubData(target, offset, this.toNativeArray(srcData as any, 'byte'));
				this.context.bufferSubDataByte(target, offset, Array.from(srcData));
			} else if (srcData instanceof Uint16Array || srcData instanceof Int16Array) {
				// this.context.bufferSubData(target, offset, this.toNativeArray(srcData as any, 'short'));
				this.context.bufferSubDataShort(target, offset, Array.from(srcData));
			} else if (srcData instanceof Uint32Array || srcData instanceof Int32Array) {
				// this.context.bufferSubData(target, offset, this.toNativeArray(srcData as any, 'int'));
				this.context.bufferSubDataInt(target, offset, Array.from(srcData));
			} else if (srcData instanceof Float32Array) {
				// this.context.bufferSubData(target, offset, this.toNativeArray(srcData as any, 'float'));
				this.context.bufferSubDataFloat(target, offset, Array.from(srcData));
			}
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

	compressedTexImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, pixels: ArrayBufferView): void {
		this._glCheckError('compressedTexImage2D');
		this._checkArgs('compressedTexImage2D', arguments);

		if (pixels && pixels.buffer instanceof ArrayBuffer) {
			if (pixels instanceof Uint8Array) {
				// this.context.compressedTexImage2D(target, level, internalformat, width, height, border, this.toNativeArray(pixels as any, 'byte'));
				this.context.compressedTexImage2DByte(target, level, internalformat, width, height, border, Array.from(pixels));
			} else if (pixels instanceof Uint16Array || pixels instanceof Int16Array) {
				// this.context.compressedTexImage2D(target, level, internalformat, width, height, border, this.toNativeArray(pixels as any, 'short'));
				this.context.compressedTexImage2DShort(target, level, internalformat, width, height, border, Array.from(pixels));
			} else if (pixels instanceof Uint32Array || pixels instanceof Int32Array) {
				// this.context.compressedTexImage2D(target, level, internalformat, width, height, border, this.toNativeArray(pixels as any, 'int'));
				this.context.compressedTexImage2DInt(target, level, internalformat, width, height, border, Array.from(pixels));
			} else if (pixels instanceof Float32Array) {
				// this.context.compressedTexImage2D(target, level, internalformat, width, height, border, this.toNativeArray(pixels as any, 'float'));
				this.context.compressedTexImage2DFloat(target, level, internalformat, width, height, border, Array.from(pixels));
			}
		} else if (pixels instanceof ArrayBuffer) {
			// this.context.compressedTexImage2D(target, level, internalformat, width, height, border, this.toNativeArray(new Uint8Array(pixels as any) as any, 'byte'));
			// @ts-ignore
			if (pixels.nativeObject) {
				// @ts-ignore
				this.context.compressedTexImage2D(target, level, internalformat, width, height, border, pixels.nativeObject);
			} else {
				this.context.compressedTexImage2DByte(target, level, internalformat, width, height, border, Array.from(new Uint8Array(pixels)));
			}
		}
	}

	compressedTexSubImage2D(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, pixels: ArrayBufferView): void {
		this._glCheckError('compressedTexSubImage2D');
		this._checkArgs('compressedTexSubImage2D', arguments);
		if (pixels && pixels.buffer instanceof ArrayBuffer) {
			if (pixels instanceof Uint8Array) {
				// this.context.compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, this.toNativeArray(pixels as any, 'byte'));
				this.context.compressedTexSubImage2DByte(target, level, xoffset, yoffset, width, height, format, Array.from(pixels));
			} else if (pixels instanceof Uint16Array || pixels instanceof Int16Array) {
				// this.context.compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, this.toNativeArray(pixels as any, 'short'));
				this.context.compressedTexSubImage2DShort(target, level, xoffset, yoffset, width, height, format, Array.from(pixels));
			} else if (pixels instanceof Uint32Array || pixels instanceof Int32Array) {
				// this.context.compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, this.toNativeArray(pixels as any, 'int'));
				this.context.compressedTexSubImage2DInt(target, level, xoffset, yoffset, width, height, format, Array.from(pixels));
			} else if (pixels instanceof Float32Array) {
				// this.context.compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, this.toNativeArray(pixels as any, 'float'));
				this.context.compressedTexSubImage2DFloat(target, level, xoffset, yoffset, width, height, format, Array.from(pixels));
			}
		} else if (pixels instanceof ArrayBuffer) {
			// this.context.compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, this.toNativeArray(new Uint8Array(pixels as any) as any, 'byte'));
			// @ts-ignore
			if (pixels.nativeObject) {
				// @ts-ignore
				this.context.compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels.nativeObject);
			} else {
				this.context.compressedTexSubImage2DByte(target, level, xoffset, yoffset, width, height, format, Array.from(new Uint8Array(pixels as any)));
			}
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
		return new WebGLTexture(this.context.createTexture());
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
		this._glCheckError('disableVertexAttribArray');
		this._checkArgs('disableVertexAttribArray', arguments);
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
		this.context.finish();
	}

	flush(): void {
		this._glCheckError('flush');
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
		this._checkArgs('activeTexture', arguments);
		const value = texture ? texture.native : 0;
		this.context.framebufferTexture2D(target, attachment, textarget, value, level);
	}

	frontFace(mode: number): void {
		this._checkArgs('activeTexture', arguments);
		this._glCheckError('activeTexture');
		this.context.frontFace(mode);
	}

	generateMipmap(target: number): void {
		this._checkArgs('activeTexture', arguments);
		this._glCheckError('activeTexture');
		this.context.generateMipmap(target);
	}

	getActiveAttrib(program: WebGLProgram, index: number): WebGLActiveInfo {
		this._glCheckError('getActiveAttrib');
		this._checkArgs('getActiveAttrib', arguments);
		const value = program ? program.native : 0;
		const attrib = this.context.getActiveAttrib(value, index);
		return new WebGLActiveInfo(attrib.getName(), attrib.getSize(), attrib.getType());
	}

	getActiveUniform(program: WebGLProgram, index: number): WebGLActiveInfo {
		this._glCheckError('getActiveUniform');
		this._checkArgs('getActiveUniform', arguments);
		const value = program ? program.native : 0;
		const uniform = this.context.getActiveUniform(value, index);
		return new WebGLActiveInfo(uniform.getName(), uniform.getSize(), uniform.getType());
	}

	getAttachedShaders(program: WebGLProgram): WebGLShader[] {
		this._glCheckError('getAttachedShaders');
		this._checkArgs('getAttachedShaders', arguments);
		const value = program ? program.native : 0;
		return this.getJSArray(this.context.getAttachedShaders(value)).map((shader) => new WebGLShader(shader));
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
		const attributes = this.context.getContextAttributes();
		if (!attributes) {
			return null;
		}
		const keys = attributes.keySet().toArray();
		const length = keys.length;
		const contextAttributes = {};
		for (let i = 0; i < length; i++) {
			const key = keys[i];
			contextAttributes[key] = attributes.get(key);
		}
		return contextAttributes;
	}

	getError(): number {
		return this.context.getError();
	}

	getExtension(name: string) {
		this._glCheckError('getExtension');
		this._checkArgs('getExtension', arguments);
		if (name === 'EXT_disjoint_timer_query_webgl2') {
			return null;
		}
		const ext = this.context.getExtension(name);
		if (ext) {
			const clazz = ext && ext.getClass();
			const classObjName = clazz && clazz.getName();
			switch (classObjName) {
				case 'org.nativescript.canvas.extensions.ANGLE_instanced_arrays':
					return new ANGLE_instanced_arrays(ext);
				case 'org.nativescript.canvas.extensions.EXT_blend_minmax':
					return new EXT_blend_minmax(ext);
				case 'org.nativescript.canvas.extensions.EXT_color_buffer_half_float':
					return new EXT_color_buffer_half_float(ext);
				case 'org.nativescript.canvas.extensions.EXT_disjoint_timer_query':
					return new EXT_disjoint_timer_query(ext);
				case 'org.nativescript.canvas.extensions.EXT_sRGB':
					return new EXT_sRGB(ext);
				case 'org.nativescript.canvas.extensions.EXT_shader_texture_lod':
					return new EXT_shader_texture_lod(ext);
				case 'org.nativescript.canvas.extensions.EXT_texture_filter_anisotropic':
					return new EXT_texture_filter_anisotropic(ext);
				case 'org.nativescript.canvas.extensions.OES_element_index_uint':
					return new OES_element_index_uint(ext);
				case 'org.nativescript.canvas.extensions.OES_standard_derivatives':
					return new OES_standard_derivatives(ext);
				case 'org.nativescript.canvas.extensions.OES_texture_float':
					return new OES_texture_float(ext);
				case 'org.nativescript.canvas.extensions.OES_texture_float_linear':
					return new OES_texture_float_linear(ext);
				case 'org.nativescript.canvas.extensions.OES_texture_half_float':
					return new OES_texture_half_float(ext);
				case 'org.nativescript.canvas.extensions.OES_texture_half_float_linear':
					return new OES_texture_half_float_linear(ext);
				case 'org.nativescript.canvas.extensions.OES_vertex_array_object':
					return new OES_vertex_array_object(ext);
				case 'org.nativescript.canvas.extensions.WEBGL_color_buffer_float':
					return new WEBGL_color_buffer_float(ext);
				case 'org.nativescript.canvas.extensions.WEBGL_compressed_texture_atc':
					return new WEBGL_compressed_texture_atc(ext);
				case 'org.nativescript.canvas.extensions.WEBGL_compressed_texture_etc':
					return new WEBGL_compressed_texture_etc(ext);
				case 'org.nativescript.canvas.extensions.WEBGL_compressed_texture_etc1':
					return new WEBGL_compressed_texture_etc1(ext);
				case 'org.nativescript.canvas.extensions.WEBGL_compressed_texture_pvrtc':
					return new WEBGL_compressed_texture_pvrtc(ext);
				case 'org.nativescript.canvas.extensions.WEBGL_compressed_texture_s3tc':
					return new WEBGL_compressed_texture_s3tc(ext);
				case 'org.nativescript.canvas.extensions.WEBGL_lose_context':
					return new WEBGL_lose_context(ext);
				case 'org.nativescript.canvas.extensions.WEBGL_depth_texture':
					return new WEBGL_depth_texture(ext);
				case 'org.nativescript.canvas.extensions.WEBGL_draw_buffers':
					return new WEBGL_draw_buffers(ext);
			}
		}
		return null;
	}

	getFramebufferAttachmentParameter(target: number, attachment: number, pname: number): number | WebGLRenderbuffer | WebGLTexture {
		this._glCheckError('getFramebufferAttachmentParameter');
		this._checkArgs('getFramebufferAttachmentParameter', arguments);
		const param = this.context.getFramebufferAttachmentParameter(target, attachment, pname);
		if (param.isRenderbuffer()) {
			return new WebGLRenderbuffer(param.getValue());
		} else if (param.isTexture()) {
			return new WebGLTexture(param.getValue());
		}
		return WebGLRenderingContext.toPrimitive(param.getValue());
	}

	getParameter(pname: number): number[] | number | WebGLBuffer | WebGLProgram | WebGLFramebuffer | WebGLRenderbuffer | WebGLTexture | Uint32Array | Int32Array | Float32Array | string | null {
		this._glCheckError('getParameter');
		this._checkArgs('activeTexture', arguments);
		const value = this.context.getParameter(pname);
		switch (pname) {
			case this.COLOR_WRITEMASK:
			case this.COLOR_CLEAR_VALUE:
			case this.BLEND_COLOR:
			case this.ALIASED_LINE_WIDTH_RANGE:
			case this.ALIASED_POINT_SIZE_RANGE:
			case this.DEPTH_RANGE:
				return new Float32Array(this.getJSArray(value));
			case this.ARRAY_BUFFER_BINDING:
			case this.ELEMENT_ARRAY_BUFFER_BINDING:
				if (value) {
					new WebGLBuffer(WebGLRenderingContext.toPrimitive(value));
				}
				return null;
			case this.CURRENT_PROGRAM:
				if (value) {
					return new WebGLProgram(WebGLRenderingContext.toPrimitive(value));
				}
				return null;
			case this.COMPRESSED_TEXTURE_FORMATS:
				return new Uint32Array(this.getJSArray(value));
			case this.RENDERBUFFER_BINDING:
				if (value) {
					return new WebGLRenderbuffer(WebGLRenderingContext.toPrimitive(value));
				}
				return null;
			case this.FRAMEBUFFER_BINDING:
				if (value) {
					return new WebGLFramebuffer(WebGLRenderingContext.toPrimitive(value));
				}
				return null;
			case this.VIEWPORT:
			case this.SCISSOR_BOX:
			case this.MAX_VIEWPORT_DIMS:
				return new Int32Array(this.getJSArray(value));
			case this.TEXTURE_BINDING_CUBE_MAP:
			case this.TEXTURE_BINDING_2D:
				if (value) {
					return new WebGLTexture(WebGLRenderingContext.toPrimitive(value));
				}
				return null;
			case this.VERSION:
				if (this._type === 'webgl2') {
					return 'WebGL 2.0 (OpenGL ES 3.0 NativeScript)';
				} else {
					return 'WebGL 1.0 (OpenGL ES 2.0 NativeScript)';
				}
			default:
				return WebGLRenderingContext.toPrimitive(value);
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
		const result = this.context.getProgramParameter(value, pname);
		return WebGLRenderingContext.toPrimitive(result);
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
		const result = this.context.getShaderParameter(value, pname);
		return WebGLRenderingContext.toPrimitive(result);
	}

	getShaderPrecisionFormat(shaderType: number, precisionType: number): WebGLShaderPrecisionFormat {
		this._glCheckError('getShaderPrecisionFormat');
		this._checkArgs('getShaderPrecisionFormat', arguments);
		const precision = this.context.getShaderPrecisionFormat(shaderType, precisionType);
		return new WebGLShaderPrecisionFormat(precision.getRangeMin(), precision.getRangeMax(), precision.getPrecision());
	}

	getShaderSource(shader: WebGLShader): string {
		this._glCheckError('getShaderSource');
		this._checkArgs('getShaderSource', arguments);
		const value = shader ? shader.native : 0;
		return this.context.getShaderSource(value);
	}

	getSupportedExtensions(): string[] {
		this._glCheckError('getSupportedExtensions');
		return this.getJSArray(this.context.getSupportedExtensions());
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
		const uniform = this.context.getUniform(value, value2);
		if (uniform && uniform.length) {
			return this.getJSArray(uniform);
		}
		return WebGLRenderingContext.toPrimitive(uniform);
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
			return new Float32Array(this.getJSArray(value));
		}
		return WebGLRenderingContext.toPrimitive(value);
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
		if (pname === this.UNPACK_FLIP_Y_WEBGL || pname === this.UNPACK_PREMULTIPLY_ALPHA_WEBGL) {
			this.context.pixelStorei(pname, java.lang.Boolean.valueOf(!!param));
		} else if (pname === this.PACK_ALIGNMENT || pname === this.UNPACK_ALIGNMENT || pname === this.UNPACK_COLORSPACE_CONVERSION_WEBGL) {
			if (pname === this.UNPACK_COLORSPACE_CONVERSION_WEBGL) {
				param = 0x9244;
			} else if (pname === this.PACK_ALIGNMENT || pname === this.UNPACK_ALIGNMENT) {
				param = 4;
			}
			this.context.pixelStorei(pname, java.lang.Integer.valueOf(param));
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
		if (pixels && pixels.buffer instanceof ArrayBuffer) {
			if (pixels instanceof Uint8Array) {
				// this.context.readPixels(x, y, width, height, format, type, this.toNativeArray(pixels as any, 'byte'));
				this.context.readPixelsByte(x, y, width, height, format, type, Array.from(pixels));
			} else if (pixels instanceof Uint16Array || pixels instanceof Int16Array) {
				// this.context.readPixels(x, y, width, height, format, type, this.toNativeArray(pixels as any, 'short'));
				this.context.readPixelsShort(x, y, width, height, format, type, Array.from(pixels));
			} else if (pixels instanceof Uint32Array || pixels instanceof Int32Array) {
				// this.context.readPixels(x, y, width, height, format, type, this.toNativeArray(pixels as any, 'int'));
				this.context.readPixelsInt(x, y, width, height, format, type, Array.from(pixels));
			} else if (pixels instanceof Float32Array) {
				// this.context.readPixels(x, y, width, height, format, type, this.toNativeArray(pixels as any, 'float'));
				this.context.readPixelsFloat(x, y, width, height, format, type, Array.from(pixels));
			}
		} else if (pixels instanceof ArrayBuffer) {
			// this.context.readPixels(x, y, width, height, format, type, this.toNativeArray(new Uint8Array(pixels as any) as any, 'byte'));
			// @ts-ignore
			if (pixels.nativeObject) {
				// @ts-ignore
				this.context.readPixels(x, y, width, height, format, type, pixels.nativeObject);
			} else {
				this.context.readPixelsByte(x, y, width, height, format, type, Array.from(new Uint8Array(pixels as any)));
			}
		}
	}

	renderbufferStorage(target: number, internalFormat: number, width: number, height: number): void {
		this._glCheckError('renderbufferStorage');
		this._checkArgs('renderbufferStorage', arguments);
		if (internalFormat === this.DEPTH_STENCIL) {
			// DEPTH24_STENCIL8 = 35056
			// DEPTH24_STENCIL8_OES = 0x88F0
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

	texImage2D(target: number, level: number, internalformat: number, format: number, type: number, pixels: any): void;

	texImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type: number, pixels: ArrayBufferView): void;

	texImage2D(target: any, level: any, internalformat: any, width: any, height: any, border: any, format?: any, type?: any, pixels?: any) {
		this._glCheckError('texImage2D');
		this._checkArgs('texImage2D', arguments);
		/* TODO */
		// this.blendFunc(this.SRC_ALPHA, this.ONE_MINUS_SRC_ALPHA);
		// this.enable(this.BLEND);
		/* TODO */
		if (arguments.length === 9) {
			if (pixels && pixels.buffer instanceof ArrayBuffer) {
				if (pixels instanceof Uint8Array) {
					// this.context.texImage2D(target, level, internalformat, width, height, border, format, type, this.toNativeArray(pixels as any, 'byte'));
					this.context.texImage2DByte(target, level, internalformat, width, height, border, format, type, Array.from(pixels));
				} else if (pixels instanceof Uint16Array || pixels instanceof Int16Array) {
					// this.context.texImage2D(target, level, internalformat, width, height, border, format, type, this.toNativeArray(pixels as any, 'short'));
					this.context.texSubImage2DShort(target, level, internalformat, width, height, border, format, type, Array.from(pixels));
				} else if (pixels instanceof Uint32Array || pixels instanceof Int32Array) {
					// this.context.texImage2D(target, level, internalformat, width, height, border, format, type, this.toNativeArray(pixels as any, 'int'));
					this.context.texImage2DInt(target, level, internalformat, width, height, border, format, type, Array.from(pixels));
				} else if (pixels instanceof Float32Array) {
					// this.context.texImage2D(target, level, internalformat, width, height, border, format, type, this.toNativeArray(pixels as any, 'float'));
					this.context.texImage2DFloat(target, level, internalformat, width, height, border, format, type, Array.from(pixels));
				}
			} else if (pixels instanceof ArrayBuffer) {
				// @ts-ignore // ArrayBuffer backed by nio buffer
				if (pixels.nativeObject) {
					// @ts-ignore
					this.context.texImage2D(target, level, internalformat, width, height, border, format, type, pixels.nativeObject);
				} else {
					// this.context.texImage2D(target, level, internalformat, width, height, border, format, type, this.toNativeArray(new Uint8Array(pixels as any) as any, 'byte'));
					this.context.texImage2DByte(target, level, internalformat, width, height, border, format, type, Array.from(new Uint8Array(pixels)));
				}
			} else {
				this.context.texImage2D(target, level, internalformat, width, height, border, format, type, pixels as any);
			}
		} else if (arguments.length === 6) {
			if (border && typeof border.tagName === 'string' && (border.tagName === 'VID' || border.tagName === 'VIDEO') && border._video && typeof border._video.getCurrentFrame === 'function') {
				border._video.getCurrentFrame(this.context, this, target, level, internalformat, width, height);
			} else if (border && typeof border.getCurrentFrame === 'function') {
				border.getCurrentFrame(this.context, this, target, level, internalformat, width, height);
			} else if (border instanceof ImageAsset) {
				this.context.texImage2D(target, level, internalformat, width, height, border.native);
			} else if (border instanceof ImageBitmap) {
				this.context.texImage2D(target, level, internalformat, width, height, border.native);
			} else if (border instanceof android.graphics.Bitmap) {
				this.context.texImage2D(target, level, internalformat, width, height, border);
			} else if (border instanceof ImageSource) {
				this.context.texImage2D(target, level, internalformat, width, height, border.android);
			} else if (border && typeof border.tagName === 'string' && (border.tagName === 'IMG' || border.tagName === 'IMAGE')) {
				if (border._asset instanceof ImageAsset) {
					this.context.texImage2D(target, level, internalformat, width, height, border._asset.native);
				} else if (border._imageSource instanceof ImageSource) {
					this.context.texImage2D(target, level, internalformat, width, height, border._imageSource.android);
				} else if (border._image instanceof android.graphics.Bitmap) {
					this.context.texImage2D(target, level, internalformat, width, height, border._image);
				} else if (typeof border.src === 'string') {
					this.context.texImage2D(target, level, internalformat, width, height, ImageSource.fromFileSync(border.src).android);
				}
			} else if (border && typeof border.tagName === 'string' && border.tagName === 'CANVAS' && border._canvas instanceof Canvas) {
				this.context.texImage2D(target, level, internalformat, width, height, border._canvas.android);
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
			if (pixels && pixels.buffer) {
				if (pixels instanceof Uint8Array) {
					// this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, this.toNativeArray(pixels as any, 'byte'));
					this.context.texSubImage2DByte(target, level, xoffset, yoffset, width, height, format, type, Array.from(pixels));
				} else if (pixels instanceof Uint16Array || pixels instanceof Int16Array) {
					// this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, this.toNativeArray(pixels as any, 'short'));
					this.context.texSubImage2DShort(target, level, xoffset, yoffset, width, height, format, type, Array.from(pixels));
				} else if (pixels instanceof Uint32Array || pixels instanceof Int32Array) {
					// this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, this.toNativeArray(pixels as any, 'int'));
					this.context.texSubImage2DInt(target, level, xoffset, yoffset, width, height, format, type, Array.from(pixels));
				} else if (pixels instanceof Float32Array) {
					// this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, this.toNativeArray(pixels as any, 'float'));
					this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, Array.from(pixels));
				}
			} else if (pixels instanceof ArrayBuffer) {
				// this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, this.toNativeArray(new Uint8Array(pixels as any) as any, 'byte'));
				// @ts-ignore
				if (pixels.nativeObject) {
					// @ts-ignore
					this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels.nativeObject);
				} else {
					this.context.texSubImage2DByte(target, level, xoffset, yoffset, width, height, format, type, Array.from(new Uint8Array(pixels)));
				}
			}
		} else if (arguments.length === 7) {
			if (format instanceof android.graphics.Bitmap) {
				this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format);
			} else if (format instanceof ImageSource) {
				this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format.android);
			} else if (format instanceof ImageAsset) {
				this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format.native);
			} else if (format && typeof format.tagName === 'string' && (format.tagName === 'IMG' || format.tagName === 'IMAGE')) {
				if (format._imageSource instanceof ImageSource) {
					this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format._imageSource.android);
				} else if (format._image instanceof android.graphics.Bitmap) {
					this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format._image);
				} else if (format._asset instanceof ImageAsset) {
					this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format._asset.native);
				} else if (typeof format.src === 'string') {
					const result = ImageSource.fromFileSync(format.src);
					this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, result ? result.android : null);
				} else if (format && typeof format.tagName === 'string' && format.tagName === 'CANVAS' && format._canvas instanceof Canvas) {
					this.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format._canvas.android);
				}
			}
		}
	}

	uniform1f(location: WebGLUniformLocation, v0: number): void {
		this._glCheckError('uniform1f');
		this._checkArgs('uniform1f', arguments);
		const loc = location ? location.native : 0;
		this.context.uniform1f(loc, v0);
	}

	uniform1iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform1iv');
		this._checkArgs('uniform1iv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		const loc = location ? location.native : 0;
		this.context.uniform1iv(loc, value);
	}

	uniform1fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform1fv');
		this._checkArgs('uniform1fv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		const loc = location ? location.native : 0;
		this.context.uniform1fv(loc, value);
	}

	uniform1i(location: WebGLUniformLocation, v0: number): void {
		this._glCheckError('uniform1i');
		this._checkArgs('uniform1i', arguments);
		const loc = location ? location.native : 0;
		this.context.uniform1i(loc, Number(v0));
	}

	uniform2f(location: WebGLUniformLocation, v0: number, v1: number): void {
		this._glCheckError('uniform2f');
		this._checkArgs('uniform2f', arguments);
		const loc = location ? location.native : 0;
		this.context.uniform2f(loc, v0, v1);
	}

	uniform2iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform2iv');
		this._checkArgs('uniform2iv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		const loc = location ? location.native : 0;
		this.context.uniform2iv(loc, value);
	}

	uniform2fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform2fv');
		this._checkArgs('uniform2fv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		const loc = location ? location.native : 0;
		this.context.uniform2fv(loc, value);
	}

	uniform2i(location: WebGLUniformLocation, v0: number, v1: number): void {
		this._glCheckError('uniform2i');
		this._checkArgs('uniform2i', arguments);
		const loc = location ? location.native : 0;
		this.context.uniform2i(loc, v0, v1);
	}

	uniform3f(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void {
		this._glCheckError('uniform3f');
		this._checkArgs('uniform3f', arguments);
		const loc = location ? location.native : 0;
		this.context.uniform3f(loc, v0, v1, v2);
	}

	uniform3iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform3iv');
		this._checkArgs('uniform3iv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		const loc = location ? location.native : 0;
		this.context.uniform3iv(loc, value);
	}

	uniform3fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform3fv');
		this._checkArgs('uniform3fv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		const loc = location ? location.native : 0;
		this.context.uniform3fv(loc, value);
	}

	uniform3i(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void {
		this._glCheckError('uniform3i');
		this._checkArgs('uniform3i', arguments);
		const loc = location ? location.native : 0;
		this.context.uniform3i(loc, v0, v1, v2);
	}

	uniform4f(location: WebGLUniformLocation, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('uniform4f');
		this._checkArgs('uniform4f', arguments);
		const loc = location ? location.native : 0;
		this.context.uniform4f(loc, v0, v1, v2, v3);
	}

	uniform4iv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform4iv');
		this._checkArgs('uniform4iv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		const loc = location ? location.native : 0;
		this.context.uniform4iv(loc, value);
	}

	uniform4fv(location: WebGLUniformLocation, value: number[]): void {
		this._glCheckError('uniform4fv');
		this._checkArgs('uniform4fv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		const loc = location ? location.native : 0;
		this.context.uniform4fv(loc, value);
	}

	uniform4i(location: WebGLUniformLocation, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('uniform4i');
		this._checkArgs('uniform4i', arguments);
		const loc = location ? location.native : 0;
		this.context.uniform4i(loc, v0, v1, v2, v3);
	}

	uniformMatrix2fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void {
		this._glCheckError('uniformMatrix2fv');
		this._checkArgs('uniformMatrix2fv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		const loc = location ? location.native : 0;
		this.context.uniformMatrix2fv(loc, transpose, value);
	}

	uniformMatrix3fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void {
		this._glCheckError('uniformMatrix3fv');
		this._checkArgs('uniformMatrix3fv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		const loc = location ? location.native : 0;
		this.context.uniformMatrix3fv(loc, transpose, value);
	}

	uniformMatrix4fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void {
		this._glCheckError('uniformMatrix4fv');
		this._checkArgs('uniformMatrix4fv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		const loc = location ? location.native : 0;
		this.context.uniformMatrix4fv(loc, transpose, value);
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
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		this.context.vertexAttrib1fv(index, value);
	}

	vertexAttrib2f(index: number, v0: number, v1: number): void {
		this._glCheckError('vertexAttrib2f');
		this._checkArgs('vertexAttrib2f', arguments);
		this.context.vertexAttrib2f(index, v0, v1);
	}

	vertexAttrib2fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib2fv');
		this._checkArgs('vertexAttrib2fv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		this.context.vertexAttrib2fv(index, value);
	}

	vertexAttrib3f(index: number, v0: number, v1: number, v2: number): void {
		this._glCheckError('vertexAttrib3f');
		this._checkArgs('vertexAttrib3f', arguments);
		this.context.vertexAttrib3f(index, v0, v1, v2);
	}

	vertexAttrib3fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib3fv');
		this._checkArgs('vertexAttrib3fv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		this.context.vertexAttrib3fv(index, value);
	}

	vertexAttrib4f(index: number, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('vertexAttrib4f');
		this._checkArgs('vertexAttrib4f', arguments);
		this.context.vertexAttrib4f(index, v0, v1, v2, v3);
	}

	vertexAttrib4fv(index: number, value: number[]): void {
		this._glCheckError('vertexAttrib4fv');
		this._checkArgs('vertexAttrib4fv', arguments);
		if (!Array.isArray(value)) {
			value = Array.from(value);
		}
		this.context.vertexAttrib4fv(index, value);
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
