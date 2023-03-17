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

import {ImageSource} from '@nativescript/core';
import {ImageAsset} from '../../ImageAsset';
import {WebGL2RenderingContextBase} from "./common";
import {Canvas} from '../../Canvas';
import { ImageBitmap } from '../../ImageBitmap';
import { Utils } from '../../utils';

export class WebGL2RenderingContext extends WebGL2RenderingContextBase {
	constructor(context) {
		super(context);
	}

	beginQuery(target: number, query: WebGLQuery): void {
		this._glCheckError('beginQuery');
		const value = query ? query.native : 0;
		this.native.beginQuery(target, value);
	}

	beginTransformFeedback(primitiveMode: number): void {
		this._glCheckError('beginTransformFeedback');
		this.native.beginTransformFeedback(primitiveMode);
	}

	bindBufferBase(target: number, index: number, buffer: WebGLBuffer): void {
		this._glCheckError('bindBufferBase');
		const value = buffer ? buffer.native : 0;
		this.native.bindBufferBase(target, index, value);
	}

	bindBufferRange(target: number, index: number, buffer: WebGLBuffer, offset: number, size: number): void {
		this._glCheckError('bindBufferRange');
		const value = buffer ? buffer.native : 0;
		this.native.bindBufferRange(target, index, value, offset, size);
	}

	bindSampler(unit: number, sampler: WebGLSampler): void {
		this._glCheckError('bindSampler');
		const value = sampler ? sampler.native : 0;
		this.native.bindSampler(unit, value);
	}

	bindTransformFeedback(target: number, transformFeedback: WebGLTransformFeedback): void {
		this._glCheckError('bindTransformFeedback');
		const value = transformFeedback ? transformFeedback.native : 0;
		this.native.bindTransformFeedback(target, value);
	}

	bindVertexArray(vertexArray: WebGLVertexArrayObject): void {
		this._glCheckError('bindVertexArray');
		const value = vertexArray ? vertexArray.native : 0;
		this.native.bindVertexArray(value);
	}

	blitFramebuffer(srcX0: number, srcY0: number, srcX1: number, srcY1: number, dstX0: number, dstY0: number, dstX1: number, dstY1: number, mask: number, filter: number): void {
		this._glCheckError('blitFramebuffer');
		this.native.blitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter);
	}

	clearBufferfi(buffer: WebGLBuffer, drawbuffer: number, depth: number, stencil: number): void {
		this._glCheckError('clearBufferfi');
		this.native.clearBufferfi(buffer.native, drawbuffer, depth, stencil);
	}

	clearBufferfv(buffer: WebGLBuffer, drawbuffer: number, values: number[] | Float32Array): void {
		this._glCheckError('clearBufferfv');
		if (!(values instanceof Float32Array)) {
			values = new Float32Array(values) as any;
		}
		this.native.clearBufferfvOffset(buffer.native, drawbuffer, values as any, (values as any).byteOffset);
	}

	clearBufferiv(buffer: WebGLBuffer, drawbuffer: number, values: number[] | Int32Array): void {
		this._glCheckError('clearBufferiv');
		if (!(values instanceof Int32Array)) {
			values = new Int32Array(values) as any;
		}
		this.native.clearBufferivOffset(buffer.native, drawbuffer, values as any, (values as any).byteOffset);
	}

	clearBufferuiv(buffer: WebGLBuffer, drawbuffer: number, values: number[] | Uint32Array): void {
		this._glCheckError('clearBufferuiv');
		if (!(values instanceof Uint32Array)) {
			values = new Uint32Array(values) as any;
		}
		this.native.clearBufferuivOffset(buffer.native, drawbuffer, values as any, (values as any).byteOffset);
	}

	clientWaitSync(sync: WebGLSync, flags: number, timeout: number): number {
		this._glCheckError('clientWaitSync');
		return this.native.clientWaitSync(sync.native, flags, timeout);
	}

	compressedTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, imageSize: number, offset: any);

	compressedTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, srcData: any, srcOffset: number = 0, srcLengthOverride: number = 0): void {
		this._glCheckError('compressedTexSubImage3D');
		if (typeof srcOffset === 'number') {
			this.native.compressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, srcData, srcOffset);
		} else if (srcData instanceof ArrayBuffer) {
			this.native.compressedTexSubImage3DU8(
				target, level, xoffset, yoffset, zoffset, width, height, depth, format, Array.from(srcData as any), srcOffset, srcLengthOverride
			);
		} else if (srcData && srcData.buffer) {
			if (srcData instanceof Int8Array) {
				this.native.compressedTexSubImage3DI8(
					target, level, xoffset, yoffset, zoffset, width, height, depth, format, Array.from(srcData as any), srcOffset, srcLengthOverride
				);
			} else if (srcData instanceof Uint8Array || srcData instanceof Uint8ClampedArray) {
				this.native.compressedTexSubImage3DU8(
					target, level, xoffset, yoffset, zoffset, width, height, depth, format, Array.from(srcData as any), srcOffset, srcLengthOverride
				);
			} else if (srcData instanceof Int16Array) {
				this.native.compressedTexSubImage3DI16(
					target, level, xoffset, yoffset, zoffset, width, height, depth, format, Array.from(srcData as any), srcOffset, srcLengthOverride
				);
			} else if (srcData instanceof Uint16Array) {
				this.native.compressedTexSubImage3DU16(
					target, level, xoffset, yoffset, zoffset, width, height, depth, format, Array.from(srcData as any), srcOffset, srcLengthOverride
				);
			} else if (srcData instanceof Int32Array) {
				this.native.compressedTexSubImage3DI32(
					target, level, xoffset, yoffset, zoffset, width, height, depth, format, Array.from(srcData as any), srcOffset, srcLengthOverride
				);
			} else if (srcData instanceof Uint32Array) {
				this.native.compressedTexSubImage3DU32(
					target, level, xoffset, yoffset, zoffset, width, height, depth, format, Array.from(srcData as any), srcOffset, srcLengthOverride
				);
			} else if (srcData instanceof Float32Array) {
				this.native.compressedTexSubImage3DF32(
					target, level, xoffset, yoffset, zoffset, width, height, depth, format, Array.from(srcData as any), srcOffset, srcLengthOverride
				);
			} else if (srcData instanceof Float64Array) {
				this.native.compressedTexSubImage3DF64(
					target, level, xoffset, yoffset, zoffset, width, height, depth, format, Array.from(srcData as any), srcOffset, srcLengthOverride
				);
			}
		}
	}

	/* Transform feedback */

	/* Framebuffers and renderbuffers */

	copyBufferSubData(readTarget: number, writeTarget: number, readOffset: number, writeOffset: number, size: number): void {
		this._glCheckError('copyBufferSubData');
		this.native.copyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size);
	}

	copyTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, x: number, y: number, width: number, height: number): void {
		this._glCheckError('copyTexSubImage3D');
		this.native.copyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height);
	}

	createQuery(): WebGLQuery {
		this._glCheckError('createQuery');
		return new WebGLQuery(this.native.createQuery());
	}

	createSampler(): WebGLSampler {
		this._glCheckError('createSampler');
		return new WebGLSampler(this.native.createSampler());
	}

	createTransformFeedback(): WebGLTransformFeedback {
		this._glCheckError('createTransformFeedback');
		return new WebGLTransformFeedback(this.native.createTransformFeedback());
	}

	createVertexArray(): WebGLVertexArrayObject {
		this._glCheckError('createVertexArray');
		return new WebGLVertexArrayObject(this.native.createVertexArray());
	}

	deleteQuery(query: WebGLQuery): void {
		this._glCheckError('deleteQueryWithQuery');
		this.native.deleteQuery(query.native);
	}

	deleteSampler(sampler: WebGLSampler): void {
		this._glCheckError('deleteSamplerWithSampler');
		this.native.deleteSampler(sampler.native);
	}

	deleteSync(sync: WebGLSync): void {
		this._glCheckError('deleteSyncWithSync');
		this.native.deleteSync(sync.native);
	}

	deleteTransformFeedback(transformFeedback: WebGLTransformFeedback): void {
		this._glCheckError('deleteTransformFeedback');
		this.native.deleteTransformFeedback(transformFeedback.native);
	}

	deleteVertexArray(vertexArray: WebGLVertexArrayObject): void {
		this._glCheckError('deleteVertexArray');
		this.native.deleteVertexArray(vertexArray.native);
	}

	drawArraysInstanced(mode: number, first: number, count: number, instanceCount: number): void {
		this._glCheckError('drawArraysInstanced');
		this.native.drawArraysInstanced(mode, first, count, instanceCount);
	}

	drawBuffers(buffers: number[]): void {
		this._glCheckError('drawBuffers');
		this.native.drawBuffers(buffers);
	}

	drawElementsInstanced(mode: number, count: number, type: number, offset: number, instanceCount: number): void {
		this._glCheckError('drawElementsInstanced');
		this.native.drawElementsInstanced(mode, count, type, offset, instanceCount);
	}

	drawRangeElements(mode: number, start: number, end: number, count: number, type: number, offset: number): void {
		this._glCheckError('drawRangeElements');
		this.native.drawRangeElements(mode, start, end, count, type, offset);
	}

	endQuery(target: number): void {
		this._glCheckError('endQuery');
		this.native.endQuery(target);
	}

	endTransformFeedback(): void {
		this._glCheckError('endTransformFeedback');
		this.native.endTransformFeedback();
	}

	fenceSync(condition: number, flags: number): void {
		this._glCheckError('fenceSync');
		this.native.fenceSync(condition, flags);
	}

	framebufferTextureLayer(target: number, attachment: number, texture: WebGLTexture, level: number, layer: number): void {
		this._glCheckError('framebufferTextureLayer');
		this.native.framebufferTextureLayer(target, attachment, texture.native, level, layer);
	}

	/* Framebuffers and renderbuffers */


	/* Uniforms */

	getActiveUniformBlockName(program: WebGLProgram, uniformBlockIndex: number): string {
		this._glCheckError('getActiveUniformBlockName');
		return this.native.getActiveUniformBlockName(program.native, uniformBlockIndex);
	}

	getActiveUniformBlockParameter(program: WebGLProgram, uniformBlockIndex: number, pname: number): any {
		this._glCheckError('getActiveUniformBlockParameter');
		const param = this.native.getActiveUniformBlockParameter(program.native, uniformBlockIndex, pname);
		if (pname === this.UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES) {
			return new Uint32Array(this.getJSArray(param));
		}
		return param;
	}

	getActiveUniforms(program: WebGLProgram, uniformIndices: number[], pname: number): any {
		this._glCheckError('getActiveUniforms');
		return this.getJSArray(this.native.getActiveUniforms(program.native, uniformIndices, pname));
	}

	getBufferSubData(target: number, srcByteOffset: number, dstData: ArrayBuffer, dstOffset: number = 0, length: number = 0): void {
		this._glCheckError('getBufferSubData');
		if (dstData instanceof ArrayBuffer) {
			this.native.getBufferSubDataSize(target, srcByteOffset,dstData as any , dstData.byteLength, dstOffset, length);
		}
	}

	getFragDataLocation(program: WebGLProgram, name: string): number {
		this._glCheckError('getFragDataLocation');
		const result = this.native.getFragDataLocation(program.native, name);
		return result !== -1 ? result : null;
	}

	getIndexedParameter(target: number, index: number): any {
		this._glCheckError('getIndexedParameter');
		const param = this.native.getIndexedParameter(target, index);
		if (target === this.TRANSFORM_FEEDBACK_BUFFER_BINDING || target === this.UNIFORM_BUFFER_BINDING) {
			return new WebGLBuffer(param);
		}
		return param;
	}

	getInternalformatParameter(target: number, internalformat: number, pname: number): any {
		this._glCheckError('getInternalformatParameter');
		const param = this.native.getInternalformatParameter(target, internalformat, pname);
		if (pname === this.SAMPLES) {
			return new Int32Array(this.getJSArray(param));
		}
		return param;
	}

	getQueryParameter(query: WebGLQuery, pname: number): any {
		this._glCheckError('getQueryParameter');
		const result = this.native.getQueryParameter(query.native, pname);
		if (result === 0) {
			return null;
		}
		return result;
	}

	//@ts-ignore
	getParameter(pname: number): number[] | number | WebGLBuffer | WebGLProgram | WebGLFramebuffer | WebGLRenderbuffer | WebGLTexture | Uint32Array | Int32Array | Float32Array | string | null {
		this._glCheckError('getParameter');
		this._checkArgs('getParameter', arguments);
		const value = this.native.getParameter(pname);
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
				return super.getParameter(pname);
		}
	}

	getQuery(target: number, pname: number): any {
		this._glCheckError('getQuery');
		const query = this.native.getQuery(target, pname);
		if (query) {
			return new WebGLQuery(query);
		}
		return null;
	}

	getSamplerParameter(sampler: WebGLSampler, pname: number): any {
		this._glCheckError('getSamplerParameter');
		return this.native.getSamplerParameter(sampler.native, pname);
	}

	getSyncParameter(sync: WebGLSync, pname: number): any {
		this._glCheckError('getSyncParameter');
		return this.native.getSyncParameter(sync.native, pname);
	}

	getTransformFeedbackVarying(program: WebGLProgram, index: number): any {
		this._glCheckError('getTransformFeedbackVarying');
		const info = this.native.getTransformFeedbackVarying(program.native, index);
		if (info) {
			return new WebGLActiveInfo(info);
		}
		return null;
	}

	getUniformBlockIndex(program: WebGLProgram, uniformBlockName: string): number {
		this._glCheckError('getUniformBlockIndex');
		return this.native.getUniformBlockIndex(program.native, uniformBlockName);
	}

	getUniformIndices(program: WebGLProgram, uniformNames: string[]): number[] {
		this._glCheckError('getUniformIndices');
		return this.getJSArray(this.native.getUniformIndices(program.native, uniformNames));
	}

	invalidateFramebuffer(target: number, attachments: number[]): void {
		this._glCheckError('invalidateFramebuffer');
		this.native.invalidateFramebuffer(target, attachments);
	}

	invalidateSubFramebuffer(target: number, attachments: number[], x: number, y: number, width: number, height: number): void {
		this._glCheckError('invalidateSubFramebuffer');
		this.native.invalidateSubFramebuffer(target, attachments, x, y, width, height);
	}

	isQuery(query: WebGLQuery): boolean {
		this._glCheckError('isQuery');
		return this.native.isQuery(query.native);
	}

	isSampler(sampler: WebGLSampler): boolean {
		this._glCheckError('isSampler');
		return this.native.isSampler(sampler.native);
	}

	isSync(sync: WebGLSync): boolean {
		this._glCheckError('isSync');
		return this.native.isSync(sync.native);
	}

	isTransformFeedback(transformFeedback: WebGLTransformFeedback): boolean {
		this._glCheckError('isTransformFeedback');
		return this.native.isTransformFeedback(transformFeedback.native);
	}

	isVertexArray(vertexArray: WebGLVertexArrayObject): boolean {
		this._glCheckError('isVertexArray');
		return this.native.isVertexArray(vertexArray.native);
	}

	pauseTransformFeedback(): void {
		this._glCheckError('pauseTransformFeedback');
		this.native.pauseTransformFeedback();
	}

	readBuffer(src: number): void {
		this._glCheckError('readBuffer');
		this.native.readBuffer(src);
	}

	renderbufferStorageMultisample(target: number, samples: number, internalFormat: number, width: number, height: number): void {
		this._glCheckError('renderbufferStorageMultisample');
		this.native.renderbufferStorageMultisample(target, samples, internalFormat, width, height);
	}

	resumeTransformFeedback(): void {
		this._glCheckError('resumeTransformFeedback');
		this.native.resumeTransformFeedback();
	}

	samplerParameterf(sampler: WebGLSampler, pname: number, param: number): void {
		this._glCheckError('samplerParameterf');
		this.native.samplerParameterf(sampler.native, pname, param);
	}


	/* Uniforms */

	/* Sync objects */

	samplerParameteri(sampler: WebGLSampler, pname: number, param: number): void {
		this._glCheckError('samplerParameteri');
		this.native.samplerParameteri(sampler.native, pname, param);
	}

	texImage3D(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, offset: any);
	texImage3D(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, source: any, srcOffset: number = 0): void {
		this._glCheckError('texImage3D');
		if (typeof source === 'number') {
			this.native.texImage3DOffset(target, level, internalformat, width, height, depth, border, format, type, source);
		} else if (source instanceof ArrayBuffer) {
			const data = NSData.dataWithData(source as any);
			if (data) {
				this.native.texImage3DData(target, level, internalformat, width, height, depth, border, format, type, data, srcOffset);
			} else {
				this.native.texImage3DU8(target, level, internalformat, width, height, depth, border, format, type, source, srcOffset);
			}
		}else if(source instanceof ArrayBuffer){
			(this.native as TNSWebGL2RenderingContext).texImage3DPixelsSizePixelOffsetSrcOffset(
				target, level, internalformat, width, height, depth, border, format, type,source as any, source.byteLength, 0 , srcOffset
			)
		} else if (source && source.buffer && Utils.isTypedArray(source)) {

			(this.native as TNSWebGL2RenderingContext).texImage3DPixelsSizePixelOffsetSrcOffset(
				target, level, internalformat, width, height, depth, border, format, type,source as any, source.byteLength, source.byteOffset , srcOffset
			)

			/*console.log('TypedArray');
			if (source instanceof Int8Array) {
				(this.native as TNSWebGL2RenderingContext).texImage3DPixelsSizePixelOffsetSrcOffset
				console.log('Int8Array');
				this.native.texImage3DI8(target, level, internalformat, width, height, depth, border, format, type, source, srcOffset);
			} else if (source instanceof Uint8Array || source instanceof Uint8ClampedArray) {
				console.log('Uint8Array' , 'Uint8ClampedArray');
				this.native.texImage3DU8(target, level, internalformat, width, height, depth, border, format, type, source, srcOffset);
			} else if (source instanceof Int16Array) {
				console.log('Int16Array');
				this.native.texImage3DI16(target, level, internalformat, width, height, depth, border, format, type, source, srcOffset);
			} else if (source instanceof Uint16Array) {
				console.log('Uint16Array');
				this.native.texImage3DU16(target, level, internalformat, width, height, depth, border, format, type, source, srcOffset);
			} else if (source instanceof Int32Array) {
				console.log('Int32Array');
				this.native.texImage3DI32(target, level, internalformat, width, height, depth, border, format, type, source, srcOffset);
			} else if (source instanceof Uint32Array) {
				console.log('Uint32Array');
				this.native.texImage3DU32(target, level, internalformat, width, height, depth, border, format, type, source, srcOffset);
			} else if (source instanceof Float32Array) {
				console.log('Float32Array', source);
				this.native.texImage3DF32(target, level, internalformat, width, height, depth, border, format, type, source, srcOffset);
			} else if (source instanceof Float64Array) {
				console.log('Float64Array');
				this.native.texImage3DF64(target, level, internalformat, width, height, depth, border, format, type, source, srcOffset);
			}

			*/
		} else if (source instanceof UIImage) {
			this.native.texImage3DSource(target, level, internalformat, width, height, depth, border, format, type, source);
		} else if (source instanceof ImageSource) {
			this.native.texImage3DSource(target, level, internalformat, width, height, depth, border, format, type, source.ios);
		} else if (source instanceof ImageAsset) {
			this.native.texImage3DAsset(target, level, internalformat, width, height, depth, border, format, type, source.native);
		} else if (source instanceof ImageBitmap) {
			this.native.texImage3DBitmap(target, level, internalformat, width, height, depth, border, format, type, source.native);
		}  else if (source &&
			typeof source.tagName === 'string' &&
			(source.tagName === 'IMG' || source.tagName === 'IMAGE')) {
			if (source._imageSource instanceof ImageSource) {
				this.native.texImage3DSource(target, level, internalformat, width, height, depth, border, format, type, source._imageSource.ios);
			} else if (source._image instanceof UIImage) {
				this.native.texImage3DSource(target, level, internalformat, width, height, depth, border, format, type, source._image);
			} else if (source._asset instanceof ImageAsset) {
				this.native.texImage3DAsset(target, level, internalformat, width, height, depth, border, format, type, source._asset.native);
			} else if (typeof source.src === 'string') {
				const result = ImageSource.fromFileSync(source.src);
				this.native.texImage3DSource(target, level, internalformat, width, height, depth, border, format, type, result ? result.ios : null);
			}
		} else if (source &&
			typeof source.tagName === 'string' &&
			source.tagName === 'CANVAS' && source._canvas instanceof Canvas) {
			this.native.texImage3DSource(target, level, internalformat, width, height, depth, border, format, type, source._canvas.ios);
		}
	}

	texStorage2D(target: number, levels: number, internalformat: number, width: number, height: number): void {
		this._glCheckError('texStorage2D');
		this.native.texStorage2D(target, levels, internalformat, width, height);
	}

	texStorage3D(target: number, levels: number, internalformat: number, width: number, height: number, depth: number): void {
		this._glCheckError('texStorage3D');
		this.native.texStorage3D(target, levels, internalformat, width, height, depth);
	}

	texSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, offset: any);

	texSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: any);

	texSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: any, srcOffset: number = 0): void {
		this._glCheckError('texSubImage3D');
		if (typeof srcData === 'number') {
			this.native.texSubImage3DOffset(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData);
		} else if (srcData instanceof ArrayBuffer) {
			const data = NSData.dataWithData(srcData as any);
			if (data) {
				this.native.texSubImage3DData(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, data, srcOffset);
			} else {
				this.native.texSubImage3DU8(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData, srcOffset);
			}
		} else if (srcData && srcData.buffer) {
			if (srcData instanceof Int8Array) {
				this.native.texSubImage3DI8(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData, srcOffset);
			} else if (srcData instanceof Uint8Array || srcData instanceof Uint8ClampedArray) {
				this.native.texSubImage3DU8(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData, srcOffset);
			} else if (srcData instanceof Int16Array) {
				this.native.texSubImage3DI16(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData, srcOffset);
			} else if (srcData instanceof Uint16Array) {
				this.native.texSubImage3DU16(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData, srcOffset);
			} else if (srcData instanceof Int32Array) {
				this.native.texSubImage3DI32(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData, srcOffset);
			} else if (srcData instanceof Uint32Array) {
				this.native.texSubImage3DU32(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData, srcOffset);
			} else if (srcData instanceof Float32Array) {
				this.native.texSubImage3DF32(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData, srcOffset);
			} else if (srcData instanceof Float64Array) {
				this.native.texSubImage3DF64(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData, srcOffset);
			}
		} else if (srcData instanceof UIImage) {
			this.native.texSubImage3DSrcData(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData);
		} else if (srcData instanceof ImageSource) {
			this.native.texSubImage3DSrcData(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData.ios);
		} else if (srcData instanceof ImageAsset) {
			this.native.texSubImage3DAsset(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData.native);
		} else if (srcData instanceof ImageBitmap) {
			this.native.texSubImage3DBitmap(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData.native);
		} else if (srcData &&
			typeof srcData.tagName === 'string' &&
			(srcData.tagName === 'IMG' || srcData.tagName === 'IMAGE')) {
			if (srcData._imageSource instanceof ImageSource) {
				this.native.texSubImage3DSrcData(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData._imageSource.ios);
			} else if (srcData._image instanceof UIImage) {
				this.native.texSubImage3DSrcData(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData._image);
			} else if (srcData._asset instanceof ImageAsset) {
				this.native.texSubImage3DAsset(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData._asset.native);
			} else if (typeof srcData.src === 'string') {
				const result = ImageSource.fromFileSync(srcData.src);
				this.native.texSubImage3DSrcData(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, result ? result.ios : null);
			}
		} else if (srcData &&
			typeof srcData.tagName === 'string' &&
			srcData.tagName === 'CANVAS' && srcData._canvas instanceof Canvas) {
			this.native.texSubImage3DAsset(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData._canvas.ios);
		}

	}

	transformFeedbackVaryings(program: WebGLProgram, varyings: string[], bufferMode: number): void {
		this._glCheckError('transformFeedbackVaryings');
		this.native.transformFeedbackVaryings(program.native, varyings, bufferMode);
	}

	uniform1ui(location: number, v0: number): void {
		this._glCheckError('uniform1ui');
		this.native.uniform1ui(location, v0);
	}

	uniform1uiv(location: number, data: Uint32Array): void {
		this._glCheckError('uniform1uiv');
		this.native.uniform1uivOffset(location, data as any, data.length, data.byteOffset);
	}

	uniform2ui(location: number, v0: number, v1: number): void {
		this._glCheckError('uniform2ui');
		this.native.uniform2ui(location, v0, v1);
	}

	uniform2uiv(location: number, data: Uint32Array): void {
		this._glCheckError('uniform2uiv');
		this.native.uniform2uivOffset(location, data as any, data.length, data.byteOffset);
	}

	/* Sync objects */

	/* Miscellaneous constants */

	uniform3ui(location: number, v0: number, v1: number, v2: number): void {
		this._glCheckError('uniform3ui');
		this.native.uniform3ui(location, v0, v1, v2);
	}

	uniform3uiv(location: number, data: Uint32Array): void {
		this._glCheckError('uniform3uiv');
		this.native.uniform3uivOffset(location, data as any, data.length, data.byteOffset);
	}

	uniform4ui(location: number, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('uniform4ui');
		this.native.uniform4ui(location, v0, v1, v2, v3);
	}

	uniform4uiv(location: number, data: Uint32Array): void {
		this._glCheckError('uniform4uiv');
		this.native.uniform4uivOffset(location, data as any, data.length, data.byteOffset);
	}

	uniformBlockBinding(program: WebGLProgram, uniformBlockIndex: number, uniformBlockBinding: number): void {
		this._glCheckError('uniformBlockBinding');
		this.native.uniformBlockBinding(program.native, uniformBlockIndex, uniformBlockBinding);
	}

	uniformMatrix2x3fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		this._glCheckError('uniformMatrix2x3fv');
		this.native.uniformMatrix2x3fvOffset(location.native, transpose, data as any, data.length, data.byteOffset);
	}

	uniformMatrix2x4fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		this._glCheckError('uniformMatrix2x4fv');
		this.native.uniformMatrix2x4fvOffset(location.native, transpose, data as any, data.length, data.byteOffset);
	}

	uniformMatrix3x2fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		this._glCheckError('uniformMatrix3x2fv');
		this.native.uniformMatrix3x2fvOffset(location.native, transpose, data as any, data.length, data.byteOffset);
	}

	uniformMatrix3x4fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		this._glCheckError('uniformMatrix3x4fv');
		this.native.uniformMatrix3x4fvOffset(location.native, transpose, data as any, data.length, data.byteOffset);
	}

	uniformMatrix4x2fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		this._glCheckError('uniformMatrix4x2fv');
		this.native.uniformMatrix4x2fv(location.native, transpose, data as any, data.length);
	}

	uniformMatrix4x3fv(location: WebGLUniformLocation, transpose: boolean, data: Float32Array): void {
		this._glCheckError('uniformMatrix4x3fv');
		this.native.uniformMatrix4x3fvOffset(location.native, transpose, data as any, data.length, data.byteOffset);
	}

	vertexAttribDivisor(index: number, divisor: number): void {
		this._glCheckError('vertexAttribDivisor');
		this.native.vertexAttribDivisor(index, divisor);
	}

	vertexAttribI4i(index: number, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('vertexAttribI4i');
		this.native.vertexAttribI4i(index, v0, v1, v2, v3);
	}

	vertexAttribI4iv(index: number, value: number[] | Int32Array): void {
		this._glCheckError('vertexAttribI4iv');
		if (!(value instanceof Int32Array)) {
			value = new Int32Array(value) as any;
		}
		this.native.vertexAttribI4ivOffset(index, value as any, (value as any).byteOffset);
	}

	vertexAttribI4ui(index: number, v0: number, v1: number, v2: number, v3: number): void {
		this._glCheckError('vertexAttribI4ui');
		this.native.vertexAttribI4ui(index, v0, v1, v2, v3);
	}

	vertexAttribI4uiv(index: number, value: number[] | Uint32Array): void {
		this._glCheckError('vertexAttribI4uiv');
		if (!(value instanceof Uint32Array)) {
			value = new Uint32Array(value) as any;
		}
		this.native.vertexAttribI4uivOffset(index, value as any, (value as any).byteOffset);
	}
	
	/* Miscellaneous constants */
}
