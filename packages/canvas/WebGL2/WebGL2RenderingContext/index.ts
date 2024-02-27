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

export class WebGL2RenderingContext extends WebGL2RenderingContextBase {
	static {
		Helpers.initialize();
	}

	_context;

	constructor(context, contextOptions) {
		super(null);
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
			direction = (<any>org).nativescript.canvas.NSCCanvas.getDirection();
		} else {
			//direction = 1;
		}

		const native = global.CanvasModule.createWebGL2Context(contextOptions, ctx, Screen.mainScreen.scale, -16777216, Screen.mainScreen.scale * 160, direction);
		this._context = native;
	}

	/* Transform feedback */

	beginQuery(target: number, query: WebGLQuery): void {
		const value = query.native;
		this.native.beginQuery(target, value);
	}

	beginTransformFeedback(primitiveMode: number): void {
		this.native.beginTransformFeedback(primitiveMode);
	}

	bindBufferBase(target: number, index: number, buffer: WebGLBuffer): void {
		const value = buffer ? buffer.native : null;
		this.native.bindBufferBase(target, index, value);
	}

	bindBufferRange(target: number, index: number, buffer: WebGLBuffer, offset: number, size: number): void {
		const value = buffer ? buffer.native : null;
		this.native.bindBufferRange(target, index, value, offset, size);
	}

	bindSampler(unit: number, sampler: WebGLSampler): void {
		const value = sampler ? sampler.native : null;
		this.native.bindSampler(unit, value);
	}

	bindTransformFeedback(target: number, transformFeedback: WebGLTransformFeedback): void {
		const value = transformFeedback ? transformFeedback.native : null;
		this.native.bindTransformFeedback(target, value);
	}

	bindVertexArray(vertexArray: WebGLVertexArrayObject): void {
		const value = vertexArray ? vertexArray.native : null;
		this.native.bindVertexArray(value);
	}

	blitFramebuffer(srcX0: number, srcY0: number, srcX1: number, srcY1: number, dstX0: number, dstY0: number, dstX1: number, dstY1: number, mask: number, filter: number): void {
		this.native.blitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter);
	}

	clearBufferfi(buffer: WebGLBuffer, drawbuffer: number, depth: number, stencil: number): void {
		const value = buffer.native;
		this.native.clearBufferfi(value, drawbuffer, depth, stencil);
	}

	clearBufferfv(buffer: WebGLBuffer, drawbuffer: number, values: number[] | Float32Array): void {
		const value = buffer.native;
		this.native.clearBufferfv(value, drawbuffer, values);
	}

	clearBufferiv(buffer: WebGLBuffer, drawbuffer: number, values: number[] | Int32Array): void {
		const value = buffer ? buffer.native : 0;
		this.native.clearBufferiv(value, drawbuffer, values);
	}

	clearBufferuiv(buffer: WebGLBuffer, drawbuffer: number, values: number[] | Uint32Array): void {
		const value = buffer ? buffer.native : 0;
		this.native.clearBufferuiv(value, drawbuffer, values);
	}

	clientWaitSync(sync: WebGLSync, flags: number, timeout: number): number {
		const value = sync.native;
		return this.native.clientWaitSync(value, flags, timeout);
	}

	compressedTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, imageSize: number, offset: any);

	compressedTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, srcData: any, srcOffset: number = 0, srcLengthOverride: number = 0): void {
		this.native.compressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, srcData, srcOffset);
	}

	/* Transform feedback */

	/* Framebuffers and renderbuffers */

	copyBufferSubData(readTarget: number, writeTarget: number, readOffset: number, writeOffset: number, size: number): void {
		this.native.copyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size);
	}

	copyTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, x: number, y: number, width: number, height: number): void {
		this.native.copyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height);
	}

	createQuery(): WebGLQuery {
		return new WebGLQuery(this.native.createQuery());
	}

	createSampler(): WebGLSampler {
		return new WebGLSampler(this.native.createSampler());
	}

	createTransformFeedback(): WebGLTransformFeedback {
		return new WebGLTransformFeedback(this.native.createTransformFeedback());
	}

	createVertexArray(): WebGLVertexArrayObject {
		return new WebGLVertexArrayObject(this.native.createVertexArray());
	}

	deleteQuery(query: WebGLQuery): void {
		const value = query.native;
		this.native.deleteQuery(value);
	}

	deleteSampler(sampler: WebGLSampler): void {
		const value = sampler.native;
		this.native.deleteSampler(value);
	}

	deleteSync(sync: WebGLSync): void {
		const value = sync.native;
		this.native.deleteSync(value);
	}

	deleteTransformFeedback(transformFeedback: WebGLTransformFeedback): void {
		const value = transformFeedback.native;
		this.native.deleteTransformFeedback(value);
	}

	deleteVertexArray(vertexArray: WebGLVertexArrayObject): void {
		const value = vertexArray.native;
		this.native.deleteVertexArray(value);
	}

	drawArraysInstanced(mode: number, first: number, count: number, instanceCount: number): void {
		this.native.drawArraysInstanced(mode, first, count, instanceCount);
	}

	drawBuffers(buffers: number[]): void {
		this.native.drawBuffers(buffers);
	}

	drawElementsInstanced(mode: number, count: number, type: number, offset: number, instanceCount: number): void {
		this.native.drawElementsInstanced(mode, count, type, offset, instanceCount);
	}

	drawRangeElements(mode: number, start: number, end: number, count: number, type: number, offset: number): void {
		this.native.drawRangeElements(mode, start, end, count, type, offset);
	}

	endQuery(target: number): void {
		this.native.endQuery(target);
	}

	endTransformFeedback(): void {
		this.native.endTransformFeedback();
	}

	fenceSync(condition: number, flags: number): WebGLSync {
		return new WebGLSync(this.native.fenceSync(condition, flags));
	}

	framebufferTextureLayer(target: number, attachment: number, texture: WebGLTexture, level: number, layer: number): void {
		const value = texture.native;
		this.native.framebufferTextureLayer(target, attachment, value, level, layer);
	}

	/* Framebuffers and renderbuffers */

	/* Uniforms */

	getActiveUniformBlockName(program: WebGLProgram, uniformBlockIndex: number): string {
		const value = program.native;
		return this.native.getActiveUniformBlockName(value, uniformBlockIndex);
	}

	getActiveUniformBlockParameter(program: WebGLProgram, uniformBlockIndex: number, pname: number): any {
		const value = program.native;
		return this.native.getActiveUniformBlockParameter(value, uniformBlockIndex, pname);
	}

	getActiveUniforms(program: WebGLProgram, uniformIndices: number[], pname: number): any {
		const value = program.native;
		return this.native.getActiveUniforms(value, uniformIndices, pname);
	}

	getBufferSubData(target: number, srcByteOffset: number, dstData: ArrayBuffer, dstOffset: number = 0, length: number = 0): void {
		this.native.getBufferSubData(target, srcByteOffset, dstData, dstOffset, length);
	}

	getFragDataLocation(program: WebGLProgram, name: string): number {
		const value = program.native;
		const result = this.native.getFragDataLocation(value, name);
		return result !== -1 ? result : null;
	}

	getIndexedParameter(target: number, index: number): any {
		const param = this.native.getIndexedParameter(target, index);
		if (target === this.TRANSFORM_FEEDBACK_BUFFER_BINDING || target === this.UNIFORM_BUFFER_BINDING) {
			return new WebGLBuffer(param);
		}
		return param;
	}

	getInternalformatParameter(target: number, internalformat: number, pname: number): any {
		return this.native.getInternalformatParameter(target, internalformat, pname);
	}

	getQueryParameter(query: WebGLQuery, pname: number): any {
		const value = query.native;
		const result = this.native.getQueryParameter(value, pname);
		if (result === 0) {
			return null;
		}
		return result;
	}

	//@ts-ignore
	getParameter(pname: number): number[] | number | WebGLBuffer | WebGLProgram | WebGLFramebuffer | WebGLRenderbuffer | WebGLTexture | Uint32Array | Int32Array | Float32Array | string | null {
		const value = this.native.getParameter(pname);
		switch (pname) {
			case this.COPY_READ_BUFFER_BINDING:
			case this.COPY_WRITE_BUFFER_BINDING:
				if (value) {
					return new WebGLBuffer(value);
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
		const query = this.native.getQuery(target, pname);
		if (query) {
			return new WebGLQuery(query);
		}
		return null;
	}

	getSamplerParameter(sampler: WebGLSampler, pname: number): any {
		const value = sampler.native;
		return this.native.getSamplerParameter(value, pname);
	}

	getSyncParameter(sync: WebGLSync, pname: number): any {
		const value = sync.native;
		return this.native.getSyncParameter(value, pname);
	}

	getTransformFeedbackVarying(program: WebGLProgram, index: number): any {
		const value = program.native;
		const info = this.native.getTransformFeedbackVarying(value, index);
		if (info) {
			return new WebGLActiveInfo(info);
		}
		return null;
	}

	getUniformBlockIndex(program: WebGLProgram, uniformBlockName: string): number {
		const value = program.native;
		return this.native.getUniformBlockIndex(value, uniformBlockName);
	}

	getUniformIndices(program: WebGLProgram, uniformNames: string[]): number[] {
		const value = program.native;
		return this.native.getUniformIndices(value, uniformNames);
	}

	invalidateFramebuffer(target: number, attachments: number[]): void {
		this.native.invalidateFramebuffer(target, attachments);
	}

	invalidateSubFramebuffer(target: number, attachments: number[], x: number, y: number, width: number, height: number): void {
		this.native.invalidateSubFramebuffer(target, attachments, x, y, width, height);
	}

	isQuery(query: WebGLQuery): boolean {
		const value = query.native;
		return this.native.isQuery(value);
	}

	isSampler(sampler: WebGLSampler): boolean {
		const value = sampler.native;
		return this.native.isSampler(value);
	}

	isSync(sync: WebGLSync): boolean {
		const value = sync.native;
		return this.native.isSync(value);
	}

	isTransformFeedback(transformFeedback: WebGLTransformFeedback): boolean {
		const value = transformFeedback.native;
		return this.native.isTransformFeedback(value);
	}

	isVertexArray(vertexArray: WebGLVertexArrayObject): boolean {
		const value = vertexArray.native;
		return this.native.isVertexArray(value);
	}

	pauseTransformFeedback(): void {
		this.native.pauseTransformFeedback();
	}

	readBuffer(src: number): void {
		this.native.readBuffer(src);
	}

	renderbufferStorageMultisample(target: number, samples: number, internalFormat: number, width: number, height: number): void {
		this.native.renderbufferStorageMultisample(target, samples, internalFormat, width, height);
	}

	resumeTransformFeedback(): void {
		this.native.resumeTransformFeedback();
	}

	samplerParameterf(sampler: WebGLSampler, pname: number, param: number): void {
		const value = sampler.native;
		this.native.samplerParameterf(value, pname, param);
	}

	/* Uniforms */

	/* Sync objects */

	samplerParameteri(sampler: WebGLSampler, pname: number, param: number): void {
		const value = sampler.native;
		this.native.samplerParameteri(value, pname, param);
	}

	texImage3D(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, offset: any);

	texImage3D(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, source: any): void {
		if (typeof source === 'number') {
			this.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, source);
		} else if (source && source.buffer) {
			this.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, source);
		} else if (source instanceof android.graphics.Bitmap) {
			this.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, source);
		} else if (source instanceof ImageSource) {
			this.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, source.android);
		} else if (source instanceof ImageAsset) {
			this.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, source.native);
		} else if (source instanceof Canvas) {
			this.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, source.native);
		} else if (source instanceof ImageBitmap) {
			this.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, source.native);
		} else if (source && typeof source.tagName === 'string' && (source.tagName === 'IMG' || source.tagName === 'IMAGE')) {
			if (source._imageSource instanceof ImageSource) {
				this.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, source._imageSource.android);
			} else if (source._image instanceof android.graphics.Bitmap) {
				this.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, source._image);
			} else if (source._asset instanceof ImageAsset) {
				this.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, source._asset.native);
			} else if (typeof source.src === 'string') {
				const result = ImageSource.fromFileSync(source.src);
				this.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, result ? result.android : null);
			}
		} else if (source && typeof source.tagName === 'string' && source.tagName === 'CANVAS' && source._canvas instanceof Canvas) {
			this.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, source._canvas.native);
		}
	}

	texStorage2D(target: number, levels: number, internalformat: number, width: number, height: number): void {
		this.native.texStorage2D(target, levels, internalformat, width, height);
	}

	texStorage3D(target: number, levels: number, internalformat: number, width: number, height: number, depth: number): void {
		this.native.texStorage3D(target, levels, internalformat, width, height, depth);
	}

	texSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, offset: any);

	texSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: any);

	texSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: any, srcOffset: number = 0): void {
		if (typeof srcData === 'number') {
			this.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData);
		} else if (srcData && srcData.buffer) {
			this.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData, srcOffset);
		} else if (srcData instanceof android.graphics.Bitmap) {
			this.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData);
		} else if (srcData instanceof ImageSource) {
			this.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData.android);
		} else if (srcData instanceof ImageAsset) {
			this.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData.native);
		} else if (srcData instanceof Canvas) {
			this.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData.native);
		} else if (srcData instanceof ImageBitmap) {
			this.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData.native);
		} else if (srcData && typeof srcData.tagName === 'string' && (srcData.tagName === 'IMG' || srcData.tagName === 'IMAGE')) {
			if (srcData._imageSource instanceof ImageSource) {
				this.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData._imageSource.android);
			} else if (srcData._image instanceof android.graphics.Bitmap) {
				this.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData._image);
			} else if (srcData._asset instanceof ImageAsset) {
				this.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData._asset.native);
			} else if (typeof srcData.src === 'string') {
				const result = ImageSource.fromFileSync(srcData.src);
				this.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, result ? result.android : null);
			}
		} else if (srcData && typeof srcData.tagName === 'string' && srcData.tagName === 'CANVAS' && srcData._canvas instanceof Canvas) {
			this.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData._canvas.native);
		}
	}

	transformFeedbackVaryings(program: WebGLProgram, varyings: string[], bufferMode: number): void {
		const value = program.native;
		this.native.transformFeedbackVaryings(value, varyings, bufferMode);
	}

	uniform1ui(location: WebGLUniformLocation, v0: number): void {
		const value = location.native;
		this.native.uniform1ui(value, v0);
	}

	uniform1uiv(location: WebGLUniformLocation, data: Uint32Array): void {
		const value = location.native;
		this.native.uniform1uiv(value, data);
	}

	uniform2ui(location: WebGLUniformLocation, v0: number, v1: number): void {
		const value = location.native;
		this.native.uniform2ui(value, v0, v1);
	}

	uniform2uiv(location: WebGLUniformLocation, data: Uint32Array): void {
		const value = location.native;
		this.native.uniform2uiv(value, data);
	}

	/* Sync objects */

	/* Miscellaneous constants */

	uniform3ui(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void {
		const value = location.native;
		this.native.uniform3ui(value, v0, v1, v2);
	}

	uniform3uiv(location: WebGLUniformLocation, data: Uint32Array): void {
		const value = location.native;
		this.native.uniform3uiv(value, data);
	}

	uniform4ui(location: WebGLUniformLocation, v0: number, v1: number, v2: number, v3: number): void {
		const value = location.native;
		this.native.uniform4ui(value, v0, v1, v2, v3);
	}

	uniform4uiv(location: WebGLUniformLocation, data: Uint32Array): void {
		const value = location.native;
		this.native.uniform4uiv(value, data);
	}

	uniformBlockBinding(program: WebGLProgram, uniformBlockIndex: number, uniformBlockBinding: number): void {
		const value = program.native;
		this.native.uniformBlockBinding(value, uniformBlockIndex, uniformBlockBinding);
	}

	uniformMatrix2x3fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		const value = location.native;
		this.native.uniformMatrix2x3fv(value, transpose, data);
	}

	uniformMatrix2x4fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		const value = location.native;
		this.native.uniformMatrix2x4fv(value, transpose, data);
	}

	uniformMatrix3x2fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		const value = location.native;
		this.native.uniformMatrix3x2fv(value, transpose, data);
	}

	uniformMatrix3x4fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		const value = location.native;
		this.native.uniformMatrix3x4fv(value, transpose, data);
	}

	uniformMatrix4x2fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		const value = location.native;
		this.native.uniformMatrix4x2fv(value, transpose, data);
	}

	uniformMatrix4x3fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		const value = location.native;
		this.native.uniformMatrix4x3fv(value, transpose, data);
	}

	vertexAttribDivisor(index: number, divisor: number): void {
		this.native.vertexAttribDivisor(index, divisor);
	}

	vertexAttribI4i(index: number, v0: number, v1: number, v2: number, v3: number): void {
		this.native.vertexAttribI4i(index, v0, v1, v2, v3);
	}

	vertexAttribI4iv(index: number, value: number[] | Int32Array): void {
		this.native.vertexAttribI4iv(index, value);
	}

	vertexAttribI4ui(index: number, v0: number, v1: number, v2: number, v3: number): void {
		this.native.vertexAttribI4ui(index, v0, v1, v2, v3);
	}

	vertexAttribI4uiv(index: number, value: number[] | Uint32Array): void {
		this.native.vertexAttribI4uiv(index, value);
	}

	/* Miscellaneous constants */
}
