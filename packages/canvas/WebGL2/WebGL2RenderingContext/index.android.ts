import { WebGLQuery } from '../WebGLQuery';
import { WebGLSampler } from '../WebGLSampler';
import { WebGLSync } from '../WebGLSync';
import { WebGLTransformFeedback } from '../WebGLTransformFeedback';
import { WebGLVertexArrayObject } from '../WebGLVertexArrayObject';
import { WebGLShader } from '../../WebGL/WebGLShader';
import { WebGLFramebuffer } from '../../WebGL/WebGLFramebuffer';
import { WebGLTexture } from '../../WebGL/WebGLTexture';
import { WebGLProgram } from '../../WebGL/WebGLProgram';
import { WebGLUniformLocation } from '../../WebGL/WebGLUniformLocation';
import { WebGLActiveInfo } from '../../WebGL/WebGLActiveInfo';
import { WebGLRenderbuffer } from '../../WebGL/WebGLRenderbuffer';
import { WebGLShaderPrecisionFormat } from '../../WebGL/WebGLShaderPrecisionFormat';
import { WebGLBuffer } from '../../WebGL/WebGLBuffer';

import { ImageAsset } from '../../ImageAsset';
import { ImageSource, Screen } from '@nativescript/core';
import { WebGL2RenderingContextBase } from './common';
import { Canvas } from '../../Canvas';
import { ImageBitmap } from '../../ImageBitmap';

import { Helpers } from '../../helpers';

let ctor;

export class WebGL2RenderingContext extends WebGL2RenderingContextBase {
	static {
		Helpers.initialize();
		ctor = global.CanvasJSIModule.createWebGL2Context;
	}

	constructor(context, contextOptions) {
		const ctx = BigInt(context.getNativeContext().toString());

		let direction = 0;
		if (androidx.core.text.TextUtilsCompat.getLayoutDirectionFromLocale(java.util.Locale.getDefault()) === androidx.core.view.ViewCompat.LAYOUT_DIRECTION_RTL) {
			direction = 1;
		}
		const native = ctor(contextOptions, ctx, Screen.mainScreen.scale, -16777216, Screen.mainScreen.scale * 160, direction);

		super(native);
	}

	_methodCache = new Map();

	_getMethod(name: string) {
		const cached = this._methodCache.get(name);
		if (cached === undefined) {
			const ret = this.native[name];
			this._methodCache.set(name, ret);
			return ret;
		}

		return cached;
	}

	/* Transform feedback */

	beginQuery(target: number, query: WebGLQuery): void {
		this._glCheckError('beginQuery');
		const value = query.native;
		const beginQuery = this._getMethod('beginQuery');
		beginQuery(target, value);
	}

	beginTransformFeedback(primitiveMode: number): void {
		this._glCheckError('beginTransformFeedback');
		const beginTransformFeedback = this._getMethod('beginTransformFeedback');
		beginTransformFeedback(primitiveMode);
	}

	bindBufferBase(target: number, index: number, buffer: WebGLBuffer): void {
		this._glCheckError('bindBufferBase');
		const value = buffer ? buffer.native : null;
		const bindBufferBase = this._getMethod('bindBufferBase');
		bindBufferBase(target, index, value);
	}

	bindBufferRange(target: number, index: number, buffer: WebGLBuffer, offset: number, size: number): void {
		this._glCheckError('bindBufferRange');
		const value = buffer ? buffer.native : null;
		const bindBufferRange = this._getMethod('bindBufferRange');
		bindBufferRange(target, index, value, offset, size);
	}

	bindSampler(unit: number, sampler: WebGLSampler): void {
		this._glCheckError('bindSampler');
		const value = sampler ? sampler.native : null;
		const bindSampler = this._getMethod('bindSampler');
		bindSampler(unit, value);
	}

	bindTransformFeedback(target: number, transformFeedback: WebGLTransformFeedback): void {
		this._glCheckError('bindTransformFeedback');
		const value = transformFeedback ? transformFeedback.native : null;
		const bindTransformFeedback = this._getMethod('bindTransformFeedback');
		bindTransformFeedback(target, value);
	}

	bindVertexArray(vertexArray: WebGLVertexArrayObject): void {
		this._glCheckError('bindVertexArray');
		const value = vertexArray ? vertexArray.native : null;
		const bindVertexArray = this._getMethod('bindVertexArray');
		bindVertexArray(value);
	}

	blitFramebuffer(srcX0: number, srcY0: number, srcX1: number, srcY1: number, dstX0: number, dstY0: number, dstX1: number, dstY1: number, mask: number, filter: number): void {
		this._glCheckError('blitFramebuffer');
		const blitFramebuffer = this._getMethod('blitFramebuffer');
		blitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter);
	}

	clearBufferfi(buffer: WebGLBuffer, drawbuffer: number, depth: number, stencil: number): void {
		this._glCheckError('clearBufferfi');
		const value = buffer.native;
		const clearBufferfi = this._getMethod('clearBufferfi');
		clearBufferfi(value, drawbuffer, depth, stencil);
	}

	clearBufferfv(buffer: WebGLBuffer, drawbuffer: number, values: number[] | Float32Array): void {
		this._glCheckError('clearBufferfv');
		const clearBufferfv = this._getMethod('clearBufferfv');
		const value = buffer.native;
		clearBufferfv(value, drawbuffer, values as any);
	}

	clearBufferiv(buffer: WebGLBuffer, drawbuffer: number, values: number[] | Int32Array): void {
		this._glCheckError('clearBufferiv');
		const clearBufferiv = this._getMethod('clearBufferiv');
		const value = buffer ? buffer.native : 0;
		clearBufferiv(value, drawbuffer, values as any);
	}

	clearBufferuiv(buffer: WebGLBuffer, drawbuffer: number, values: number[] | Uint32Array): void {
		this._glCheckError('clearBufferuiv');
		const clearBufferuiv = this._getMethod('clearBufferuiv');
		const value = buffer ? buffer.native : 0;
		clearBufferuiv(value, drawbuffer, values as any);
	}

	clientWaitSync(sync: WebGLSync, flags: number, timeout: number): number {
		this._glCheckError('clientWaitSync');
		const clientWaitSync = this._getMethod('clientWaitSync');
		const value = sync.native;
		return clientWaitSync(value, flags, timeout);
	}

	compressedTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, imageSize: number, offset: any);

	compressedTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, srcData: any, srcOffset: number = 0, srcLengthOverride: number = 0): void {
		this._glCheckError('compressedTexSubImage3D');
		const compressedTexSubImage3D = this._getMethod('compressedTexSubImage3D');
		compressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, srcData, srcOffset);
	}

	/* Transform feedback */

	/* Framebuffers and renderbuffers */

	copyBufferSubData(readTarget: number, writeTarget: number, readOffset: number, writeOffset: number, size: number): void {
		this._glCheckError('copyBufferSubData');
		const copyBufferSubData = this._getMethod('copyBufferSubData');
		copyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size);
	}

	copyTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, x: number, y: number, width: number, height: number): void {
		this._glCheckError('copyTexSubImage3D');
		const copyTexSubImage3D = this._getMethod('copyTexSubImage3D');
		copyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height);
	}

	createQuery(): WebGLQuery {
		this._glCheckError('createQuery');
		const createQuery = this._getMethod('createQuery');
		return new WebGLQuery(createQuery());
	}

	createSampler(): WebGLSampler {
		this._glCheckError('createSampler');
		const createSampler = this._getMethod('createSampler');
		return new WebGLSampler(createSampler());
	}

	createTransformFeedback(): WebGLTransformFeedback {
		this._glCheckError('createTransformFeedback');
		const createTransformFeedback = this._getMethod('createTransformFeedback');
		return new WebGLTransformFeedback(createTransformFeedback());
	}

	createVertexArray(): WebGLVertexArrayObject {
		this._glCheckError('createVertexArray');
		const createVertexArray = this._getMethod('createVertexArray');
		return new WebGLVertexArrayObject(createVertexArray());
	}

	deleteQuery(query: WebGLQuery): void {
		this._glCheckError('deleteQuery');
		const deleteQuery = this._getMethod('deleteQuery');
		const value = query.native;
		deleteQuery(value);
	}

	deleteSampler(sampler: WebGLSampler): void {
		this._glCheckError('deleteSampler');
		const deleteSampler = this._getMethod('deleteSampler');
		const value = sampler.native;
		deleteSampler(value);
	}

	deleteSync(sync: WebGLSync): void {
		this._glCheckError('deleteSync');
		const deleteSync = this._getMethod('deleteSync');
		const value = sync.native;
		deleteSync(value);
	}

	deleteTransformFeedback(transformFeedback: WebGLTransformFeedback): void {
		this._glCheckError('deleteTransformFeedback');
		const deleteTransformFeedback = this._getMethod('deleteTransformFeedback');
		const value = transformFeedback.native;
		deleteTransformFeedback(value);
	}

	deleteVertexArray(vertexArray: WebGLVertexArrayObject): void {
		this._glCheckError('deleteVertexArray');
		const deleteVertexArray = this._getMethod('deleteVertexArray');
		const value = vertexArray.native;
		deleteVertexArray(value);
	}

	drawArraysInstanced(mode: number, first: number, count: number, instanceCount: number): void {
		this._glCheckError('drawArraysInstanced');
		const drawArraysInstanced = this._getMethod('drawArraysInstanced');
		drawArraysInstanced(mode, first, count, instanceCount);
	}

	drawBuffers(buffers: number[]): void {
		this._glCheckError('drawBuffers');
		const drawBuffers = this._getMethod('drawBuffers');
		drawBuffers(buffers);
	}

	drawElementsInstanced(mode: number, count: number, type: number, offset: number, instanceCount: number): void {
		this._glCheckError('drawElementsInstanced');
		const drawElementsInstanced = this._getMethod('drawElementsInstanced');
		drawElementsInstanced(mode, count, type, offset, instanceCount);
	}

	drawRangeElements(mode: number, start: number, end: number, count: number, type: number, offset: number): void {
		this._glCheckError('drawRangeElements');
		const drawRangeElements = this._getMethod('drawRangeElements');
		drawRangeElements(mode, start, end, count, type, offset);
	}

	endQuery(target: number): void {
		this._glCheckError('endQuery');
		const endQuery = this._getMethod('endQuery');
		endQuery(target);
	}

	endTransformFeedback(): void {
		this._glCheckError('endTransformFeedback');
		const endTransformFeedback = this._getMethod('endTransformFeedback');
		endTransformFeedback();
	}

	fenceSync(condition: number, flags: number): void {
		this._glCheckError('fenceSync');
		const fenceSync = this._getMethod('fenceSync');
		fenceSync(condition, flags);
	}

	framebufferTextureLayer(target: number, attachment: number, texture: WebGLTexture, level: number, layer: number): void {
		this._glCheckError('framebufferTextureLayer');
		const framebufferTextureLayer = this._getMethod('framebufferTextureLayer');
		const value = texture.native;
		framebufferTextureLayer(target, attachment, value, level, layer);
	}

	/* Framebuffers and renderbuffers */

	/* Uniforms */

	getActiveUniformBlockName(program: WebGLProgram, uniformBlockIndex: number): string {
		this._glCheckError('getActiveUniformBlockName');
		const getActiveUniformBlockName = this._getMethod('getActiveUniformBlockName');
		const value = program.native;
		return getActiveUniformBlockName(value, uniformBlockIndex);
	}

	getActiveUniformBlockParameter(program: WebGLProgram, uniformBlockIndex: number, pname: number): any {
		this._glCheckError('getActiveUniformBlockParameter');
		const getActiveUniformBlockParameter = this._getMethod('getActiveUniformBlockParameter');
		const value = program.native;
		return getActiveUniformBlockParameter(value, uniformBlockIndex, pname);
	}

	getActiveUniforms(program: WebGLProgram, uniformIndices: number[], pname: number): any {
		this._glCheckError('getActiveUniforms');
		const getActiveUniforms = this._getMethod('getActiveUniforms');
		const value = program.native;
		return getActiveUniforms(value, uniformIndices, pname);
	}

	getBufferSubData(target: number, srcByteOffset: number, dstData: ArrayBuffer, dstOffset: number = 0, length: number = 0): void {
		this._glCheckError('getBufferSubData');
		const getBufferSubData = this._getMethod('getBufferSubData');
		getBufferSubData(target, srcByteOffset, dstData, dstOffset, length);
	}

	getFragDataLocation(program: WebGLProgram, name: string): number {
		this._glCheckError('getFragDataLocation');
		const getFragDataLocation = this._getMethod('getFragDataLocation');
		const value = program.native;
		const result = getFragDataLocation(value, name);
		return result !== -1 ? result : null;
	}

	getIndexedParameter(target: number, index: number): any {
		this._glCheckError('getIndexedParameter');
		const getIndexedParameter = this._getMethod('getIndexedParameter');
		const param = getIndexedParameter(target, index);
		if (target === this.TRANSFORM_FEEDBACK_BUFFER_BINDING || target === this.UNIFORM_BUFFER_BINDING) {
			return new WebGLBuffer(param);
		}
		return param;
	}

	getInternalformatParameter(target: number, internalformat: number, pname: number): any {
		this._glCheckError('getInternalformatParameter');
		const getInternalformatParameter = this._getMethod('getInternalformatParameter');
		return getInternalformatParameter(target, internalformat, pname);
	}

	getQueryParameter(query: WebGLQuery, pname: number): any {
		this._glCheckError('getQueryParameter');
		const getQueryParameter = this._getMethod('getQueryParameter');
		const value = query.native;
		const result = getQueryParameter(value, pname);
		if (result === 0) {
			return null;
		}
		return result;
	}

	//@ts-ignore
	getParameter(pname: number): number[] | number | WebGLBuffer | WebGLProgram | WebGLFramebuffer | WebGLRenderbuffer | WebGLTexture | Uint32Array | Int32Array | Float32Array | string | null {
		this._glCheckError('getParameter');
		this._checkArgs('getParameter', arguments);
		const getParameter = this._getMethod('getParameter');
		const value = getParameter(pname);
		switch (pname) {
			case this.COPY_READ_BUFFER_BINDING:
			case this.COPY_WRITE_BUFFER_BINDING:
				if (value) {
					new WebGLBuffer(value);
				}
				return null;
			case this.DRAW_FRAMEBUFFER_BINDING:
				if (value) {
					return new WebGLFramebuffer(value);
				}
				return null;
			default:
				return (this as any)._handleGetParameter(pname, value);
		}
	}

	getQuery(target: number, pname: number): any {
		this._glCheckError('getQuery');
		const getQuery = this._getMethod('getQuery');
		const query = getQuery(target, pname);
		if (query) {
			return new WebGLQuery(query);
		}
		return null;
	}

	getSamplerParameter(sampler: WebGLSampler, pname: number): any {
		this._glCheckError('getSamplerParameter');
		const getSamplerParameter = this._getMethod('getSamplerParameter');
		const value = sampler.native;
		return getSamplerParameter(value, pname);
	}

	getSyncParameter(sync: WebGLSync, pname: number): any {
		this._glCheckError('getSyncParameter');
		const getSyncParameter = this._getMethod('getSyncParameter');
		const value = sync.native;
		return getSyncParameter(value, pname);
	}

	getTransformFeedbackVarying(program: WebGLProgram, index: number): any {
		this._glCheckError('getTransformFeedbackVarying');
		const value = program.native;
		const getTransformFeedbackVarying = this._getMethod('getTransformFeedbackVarying');
		const info = getTransformFeedbackVarying(value, index);
		if (info) {
			return new WebGLActiveInfo(info);
		}
		return null;
	}

	getUniformBlockIndex(program: WebGLProgram, uniformBlockName: string): number {
		this._glCheckError('getUniformBlockIndex');
		const getUniformBlockIndex = this._getMethod('getUniformBlockIndex');
		const value = program.native;
		return getUniformBlockIndex(value, uniformBlockName);
	}

	getUniformIndices(program: WebGLProgram, uniformNames: string[]): number[] {
		this._glCheckError('getUniformIndices');
		const getUniformIndices = this._getMethod('getUniformIndices');
		const value = program.native;
		return getUniformIndices(value, uniformNames);
	}

	invalidateFramebuffer(target: number, attachments: number[]): void {
		this._glCheckError('invalidateFramebuffer');
		const invalidateFramebuffer = this._getMethod('invalidateFramebuffer');
		invalidateFramebuffer(target, attachments);
	}

	invalidateSubFramebuffer(target: number, attachments: number[], x: number, y: number, width: number, height: number): void {
		this._glCheckError('invalidateSubFramebuffer');
		const invalidateSubFramebuffer = this._getMethod('invalidateSubFramebuffer');
		invalidateSubFramebuffer(target, attachments, x, y, width, height);
	}

	isQuery(query: WebGLQuery): boolean {
		this._glCheckError('isQuery');
		const isQuery = this._getMethod('isQuery');
		const value = query.native;
		return isQuery(value);
	}

	isSampler(sampler: WebGLSampler): boolean {
		this._glCheckError('isSampler');
		const isSampler = this._getMethod('isSampler');
		const value = sampler.native;
		return isSampler(value);
	}

	isSync(sync: WebGLSync): boolean {
		this._glCheckError('isSync');
		const isSync = this._getMethod('isSync');
		const value = sync.native;
		return isSync(value);
	}

	isTransformFeedback(transformFeedback: WebGLTransformFeedback): boolean {
		this._glCheckError('isTransformFeedback');
		const isTransformFeedback = this._getMethod('isTransformFeedback');
		const value = transformFeedback.native;
		return isTransformFeedback(value);
	}

	isVertexArray(vertexArray: WebGLVertexArrayObject): boolean {
		this._glCheckError('isVertexArray');
		const isVertexArray = this._getMethod('isVertexArray');
		const value = vertexArray.native;
		return isVertexArray(value);
	}

	pauseTransformFeedback(): void {
		this._glCheckError('pauseTransformFeedback');
		const pauseTransformFeedback = this._getMethod('pauseTransformFeedback');
		pauseTransformFeedback();
	}

	readBuffer(src: number): void {
		this._glCheckError('readBuffer');
		const readBuffer = this._getMethod('readBuffer');
		readBuffer(src);
	}

	renderbufferStorageMultisample(target: number, samples: number, internalFormat: number, width: number, height: number): void {
		this._glCheckError('renderbufferStorageMultisample');
		const renderbufferStorageMultisample = this._getMethod('renderbufferStorageMultisample');
		renderbufferStorageMultisample(target, samples, internalFormat, width, height);
	}

	resumeTransformFeedback(): void {
		this._glCheckError('resumeTransformFeedback');
		const resumeTransformFeedback = this._getMethod('resumeTransformFeedback');
		resumeTransformFeedback();
	}

	samplerParameterf(sampler: WebGLSampler, pname: number, param: number): void {
		this._glCheckError('samplerParameterf');
		const samplerParameterf = this._getMethod('samplerParameterf');
		const value = sampler.native;
		samplerParameterf(value, pname, param);
	}

	/* Uniforms */

	/* Sync objects */

	samplerParameteri(sampler: WebGLSampler, pname: number, param: number): void {
		this._glCheckError('samplerParameteri');
		const samplerParameteri = this._getMethod('samplerParameteri');
		const value = sampler.native;
		samplerParameteri(value, pname, param);
	}

	texImage3D(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, offset: any);

	texImage3D(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, source: any): void {
		this._glCheckError('texImage3D');
		const texImage3D = this._getMethod('texImage3D');
		if (typeof source === 'number') {
			texImage3D(target, level, internalformat, width, height, depth, border, format, type, source);
		} else if (source && source.buffer) {
			texImage3D(target, level, internalformat, width, height, depth, border, format, type, source);
		} else if (source instanceof android.graphics.Bitmap) {
			texImage3D(target, level, internalformat, width, height, depth, border, format, type, source);
		} else if (source instanceof ImageSource) {
			texImage3D(target, level, internalformat, width, height, depth, border, format, type, source.android);
		} else if (source instanceof ImageAsset) {
			texImage3D(target, level, internalformat, width, height, depth, border, format, type, source.native);
		} else if (source instanceof ImageBitmap) {
			texImage3D(target, level, internalformat, width, height, depth, border, format, type, source.native);
		} else if (source && typeof source.tagName === 'string' && (source.tagName === 'IMG' || source.tagName === 'IMAGE')) {
			if (source._imageSource instanceof ImageSource) {
				texImage3D(target, level, internalformat, width, height, depth, border, format, type, source._imageSource.android);
			} else if (source._image instanceof android.graphics.Bitmap) {
				texImage3D(target, level, internalformat, width, height, depth, border, format, type, source._image);
			} else if (source._asset instanceof ImageAsset) {
				texImage3D(target, level, internalformat, width, height, depth, border, format, type, source._asset.native);
			} else if (typeof source.src === 'string') {
				const result = ImageSource.fromFileSync(source.src);
				texImage3D(target, level, internalformat, width, height, depth, border, format, type, result ? result.android : null);
			}
		} else if (source && typeof source.tagName === 'string' && source.tagName === 'CANVAS' && source._canvas instanceof Canvas) {
			texImage3D(target, level, internalformat, width, height, depth, border, format, type, source._canvas.native);
		}
	}

	texStorage2D(target: number, levels: number, internalformat: number, width: number, height: number): void {
		this._glCheckError('texStorage2D');
		const texStorage2D = this._getMethod('texStorage2D');
		texStorage2D(target, levels, internalformat, width, height);
	}

	texStorage3D(target: number, levels: number, internalformat: number, width: number, height: number, depth: number): void {
		this._glCheckError('texStorage3D');
		const texStorage3D = this._getMethod('texStorage3D');
		texStorage3D(target, levels, internalformat, width, height, depth);
	}

	texSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, offset: any);

	texSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: any);

	texSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: any, srcOffset: number = 0): void {
		this._glCheckError('texSubImage3D');
		const texSubImage3D = this._getMethod('texSubImage3D');
		if (typeof srcData === 'number') {
			texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData);
		} else if (srcData && srcData.buffer) {
			texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData, srcOffset);
		} else if (srcData instanceof android.graphics.Bitmap) {
			texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData);
		} else if (srcData instanceof ImageSource) {
			texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData.android);
		} else if (srcData instanceof ImageAsset) {
			texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData.native);
		} else if (srcData instanceof ImageBitmap) {
			texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData.native);
		} else if (srcData && typeof srcData.tagName === 'string' && (srcData.tagName === 'IMG' || srcData.tagName === 'IMAGE')) {
			if (srcData._imageSource instanceof ImageSource) {
				texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData._imageSource.android);
			} else if (srcData._image instanceof android.graphics.Bitmap) {
				texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData._image);
			} else if (srcData._asset instanceof ImageAsset) {
				texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData._asset.native);
			} else if (typeof srcData.src === 'string') {
				const result = ImageSource.fromFileSync(srcData.src);
				texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, result ? result.android : null);
			}
		} else if (srcData && typeof srcData.tagName === 'string' && srcData.tagName === 'CANVAS' && srcData._canvas instanceof TNSCanvas) {
			texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData._canvas.native);
		}
	}

	transformFeedbackVaryings(program: WebGLProgram, varyings: string[], bufferMode: number): void {
		this._glCheckError('transformFeedbackVaryings');
		const transformFeedbackVaryings = this._getMethod('transformFeedbackVaryings');
		const value = program.native;
		transformFeedbackVaryings(value, varyings, bufferMode);
	}

	uniform1ui(location: WebGLUniformLocation, v0: number): void {
		this._glCheckError('uniform1ui');
		const value = location.native;
		const uniform1ui = this._getMethod('uniform1ui');
		uniform1ui(value, v0);
	}

	uniform1uiv(location: WebGLUniformLocation, data: Uint32Array): void {
		this._glCheckError('uniform1uiv');
		const uniform1uiv = this._getMethod('uniform1uiv');
		const value = location.native;
		uniform1uiv(value, data);
	}

	uniform2ui(location: WebGLUniformLocation, v0: number, v1: number): void {
		this._glCheckError('uniform2ui');
		const uniform2ui = this._getMethod('uniform2ui');
		const value = location.native;
		uniform2ui(value, v0, v1);
	}

	uniform2uiv(location: WebGLUniformLocation, data: Uint32Array): void {
		this._glCheckError('uniform2uiv');
		const uniform2uiv = this._getMethod('uniform2uiv');
		const value = location.native;
		uniform2uiv(value, data);
	}

	/* Sync objects */

	/* Miscellaneous constants */

	uniform3ui(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void {
		this._glCheckError('uniform3ui');
		const uniform3ui = this._getMethod('uniform3ui');
		const value = location.native;
		uniform3ui(value, v0, v1, v2);
	}

	uniform3uiv(location: WebGLUniformLocation, data: Uint32Array): void {
		this._glCheckError('uniform3uiv');
		const uniform3uiv = this._getMethod('uniform3uiv');
		const value = location.native;
		uniform3uiv(value, Array.from(data as any));
	}

	uniform4ui(location: WebGLUniformLocation, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('uniform4ui');
		const uniform4ui = this._getMethod('uniform4ui');
		const value = location.native;
		uniform4ui(value, v0, v1, v2, v3);
	}

	uniform4uiv(location: WebGLUniformLocation, data: Uint32Array): void {
		this._glCheckError('uniform4uiv');
		const uniform4uiv = this._getMethod('uniform4uiv');
		const value = location.native;
		uniform4uiv(value, Array.from(data as any));
	}

	uniformBlockBinding(program: WebGLProgram, uniformBlockIndex: number, uniformBlockBinding: number): void {
		this._glCheckError('uniformBlockBinding');
		const uniformBlockBindingFunc = this._getMethod('uniformBlockBinding');
		const value = program.native;
		uniformBlockBindingFunc(value, uniformBlockIndex, uniformBlockBinding);
	}

	uniformMatrix2x3fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		this._glCheckError('uniformMatrix2x3fv');
		const uniformMatrix2x3fv = this._getMethod('uniformMatrix2x3fv');
		const value = location.native;
		uniformMatrix2x3fv(value, transpose, data);
	}

	uniformMatrix2x4fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		this._glCheckError('uniformMatrix2x4fv');
		const uniformMatrix2x4fv = this._getMethod('uniformMatrix2x4fv');
		const value = location.native;
		uniformMatrix2x4fv(value, transpose, data);
	}

	uniformMatrix3x2fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		this._glCheckError('uniformMatrix3x2fv');
		const uniformMatrix3x2fv = this._getMethod('uniformMatrix3x2fv');
		const value = location.native;
		uniformMatrix3x2fv(value, transpose, data);
	}

	uniformMatrix3x4fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		this._glCheckError('uniformMatrix3x4fv');
		const uniformMatrix3x4fv = this._getMethod('uniformMatrix3x4fv');
		const value = location.native;
		uniformMatrix3x4fv(value, transpose, data);
	}

	uniformMatrix4x2fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		this._glCheckError('uniformMatrix4x2fv');
		const uniformMatrix4x2fv = this._getMethod('uniformMatrix4x2fv');
		const value = location.native;
		uniformMatrix4x2fv(value, transpose, data);
	}

	uniformMatrix4x3fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		this._glCheckError('uniformMatrix4x3fv');
		const uniformMatrix4x3fv = this._getMethod('uniformMatrix4x3fv');
		const value = location.native;
		uniformMatrix4x3fv(value, transpose, data as any);
	}

	vertexAttribDivisor(index: number, divisor: number): void {
		this._glCheckError('vertexAttribDivisor');
		const vertexAttribDivisor = this._getMethod('vertexAttribDivisor');
		vertexAttribDivisor(index, divisor);
	}

	vertexAttribI4i(index: number, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('vertexAttribI4i');
		const vertexAttribI4i = this._getMethod('vertexAttribI4i');
		vertexAttribI4i(index, v0, v1, v2, v3);
	}

	vertexAttribI4iv(index: number, value: number[] | Int32Array): void {
		this._glCheckError('vertexAttribI4iv');
		const vertexAttribI4iv = this._getMethod('vertexAttribI4iv');
		vertexAttribI4iv(index, value as any);
	}

	vertexAttribI4ui(index: number, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('vertexAttribI4ui');
		const vertexAttribI4ui = this._getMethod('vertexAttribI4ui');
		vertexAttribI4ui(index, v0, v1, v2, v3);
	}

	vertexAttribI4uiv(index: number, value: number[] | Uint32Array): void {
		this._glCheckError('vertexAttribI4uiv');
		const vertexAttribI4uiv = this._getMethod('vertexAttribI4uiv');
		vertexAttribI4uiv(index, value as any);
	}

	/* Miscellaneous constants */
}
