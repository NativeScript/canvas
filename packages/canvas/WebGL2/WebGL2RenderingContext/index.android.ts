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

import {ImageAsset} from '../../ImageAsset';
import {ImageSource} from '@nativescript/core';
import {WebGL2RenderingContextBase} from "./common";
import {Canvas} from '../../Canvas';
import { ImageBitmap } from '../../ImageBitmap';

export class WebGL2RenderingContext extends WebGL2RenderingContextBase {
	constructor(context) {
		super(context);
	}

	// native: com.github.triniwiz.canvas.WebGL2RenderingContext;
	/* Transform feedback */

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

	bindBufferRange(
		target: number,
		index: number,
		buffer: WebGLBuffer,
		offset: number,
		size: number
	): void {
		this._glCheckError('bindBufferRange');
		const value = buffer ? buffer.native : 0;
		this.native.bindBufferRange(target, index, value, offset, size);
	}

	bindSampler(unit: number, sampler: WebGLSampler): void {
		this._glCheckError('bindSampler');
		const value = sampler ? sampler.native : 0;
		this.native.bindSampler(unit, value);
	}

	bindTransformFeedback(
		target: number,
		transformFeedback: WebGLTransformFeedback
	): void {
		this._glCheckError('bindTransformFeedback');
		const value = transformFeedback ? transformFeedback.native : 0;
		this.native.bindTransformFeedback(target, value);
	}

	bindVertexArray(vertexArray: WebGLVertexArrayObject): void {
		this._glCheckError('bindVertexArray');
		const value = vertexArray ? vertexArray.native : 0;
		this.native.bindVertexArray(value);
	}

	blitFramebuffer(
		srcX0: number,
		srcY0: number,
		srcX1: number,
		srcY1: number,
		dstX0: number,
		dstY0: number,
		dstX1: number,
		dstY1: number,
		mask: number,
		filter: number
	): void {
		this._glCheckError('blitFramebuffer');
		this.native.blitFramebuffer(
			srcX0,
			srcY0,
			srcX1,
			srcY1,
			dstX0,
			dstY0,
			dstX1,
			dstY1,
			mask,
			filter
		);
	}

	clearBufferfi(
		buffer: WebGLBuffer,
		drawbuffer: number,
		depth: number,
		stencil: number
	): void {
		this._glCheckError('clearBufferfi');
		const value = buffer ? buffer.native : 0;
		this.native.clearBufferfi(value, drawbuffer, depth, stencil);
	}

	clearBufferfv(
		buffer: WebGLBuffer,
		drawbuffer: number,
		values: number[] | Float32Array
	): void {
		this._glCheckError('clearBufferfv');
		const value = buffer ? buffer.native : 0;
		if (values instanceof Float32Array) {
			values = this.toNativeArray(values as any, 'float') as any;
		}
		this.native.clearBufferfv(value, drawbuffer, values as any);
	}

	clearBufferiv(
		buffer: WebGLBuffer,
		drawbuffer: number,
		values: number[] | Int32Array
	): void {
		this._glCheckError('clearBufferiv');
		const value = buffer ? buffer.native : 0;
		if (values instanceof Int32Array) {
			values = this.toNativeArray(values as any, 'int');
		}
		this.native.clearBufferiv(value, drawbuffer, values as any);
	}

	clearBufferuiv(
		buffer: WebGLBuffer,
		drawbuffer: number,
		values: number[] | Uint32Array
	): void {
		this._glCheckError('clearBufferuiv');
		const value = buffer ? buffer.native : 0;
		if (values instanceof Uint32Array) {
			values = this.toNativeArray(values as any, 'int');
		}
		this.native.clearBufferuiv(value, drawbuffer, values as any);
	}

	clientWaitSync(sync: WebGLSync, flags: number, timeout: number): number {
		this._glCheckError('clientWaitSync');
		const value = sync ? sync.native : 0;
		return this.native.clientWaitSync(value, flags, timeout);
	}

	compressedTexSubImage3D(
		target: number,
		level: number,
		xoffset: number,
		yoffset: number,
		zoffset: number,
		width: number,
		height: number,
		depth: number,
		format: number,
		imageSize: number,
		offset: any
	);

	compressedTexSubImage3D(
		target: number,
		level: number,
		xoffset: number,
		yoffset: number,
		zoffset: number,
		width: number,
		height: number,
		depth: number,
		format: number,
		srcData: any,
		srcOffset: number = 0,
		srcLengthOverride: number = 0
	): void {
		this._glCheckError('compressedTexSubImage3D');
		if (typeof srcOffset === 'number') {
			this.native.compressedTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				srcData,
				srcOffset
			);
		} else if (srcData && srcData.buffer) {
			if (srcData instanceof Uint8Array) {
				this.native.compressedTexSubImage3D(
					target,
					level,
					xoffset,
					yoffset,
					zoffset,
					width,
					height,
					depth,
					format,
					this.toNativeArray(srcData as any, 'byte'),
					srcOffset,
					srcLengthOverride
				);
			}
		}
	}

	/* Transform feedback */

	/* Framebuffers and renderbuffers */

	copyBufferSubData(
		readTarget: number,
		writeTarget: number,
		readOffset: number,
		writeOffset: number,
		size: number
	): void {
		this._glCheckError('copyBufferSubData');
		this.native.copyBufferSubData(
			readTarget,
			writeTarget,
			readOffset,
			writeOffset,
			size
		);
	}

	copyTexSubImage3D(
		target: number,
		level: number,
		xoffset: number,
		yoffset: number,
		zoffset: number,
		x: number,
		y: number,
		width: number,
		height: number
	): void {
		this._glCheckError('copyTexSubImage3D');
		this.native.copyTexSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			x,
			y,
			width,
			height
		);
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

	deleteQueryWithQuery(query: WebGLQuery): void {
		this._glCheckError('deleteQueryWithQuery');
		const value = query ? query.native : 0;
		this.native.deleteQuery(value);
	}

	deleteSamplerWithSampler(sampler: WebGLSampler): void {
		this._glCheckError('deleteSamplerWithSampler');
		const value = sampler ? sampler.native : 0;
		this.native.deleteSampler(value);
	}

	deleteSyncWithSync(sync: WebGLSync): void {
		this._glCheckError('deleteSyncWithSync');
		const value = sync ? sync.native : 0;
		this.native.deleteSync(value);
	}

	deleteTransformFeedback(transformFeedback: WebGLTransformFeedback): void {
		this._glCheckError('deleteTransformFeedback');
		const value = transformFeedback ? transformFeedback.native : 0;
		this.native.deleteTransformFeedback(value);
	}

	deleteVertexArray(vertexArray: WebGLVertexArrayObject): void {
		this._glCheckError('deleteVertexArray');
		const value = vertexArray ? vertexArray.native : 0;
		this.native.deleteVertexArray(value);
	}

	drawArraysInstanced(
		mode: number,
		first: number,
		count: number,
		instanceCount: number
	): void {
		this._glCheckError('drawArraysInstanced');
		this.native.drawArraysInstanced(mode, first, count, instanceCount);
	}

	drawBuffers(buffers: number[]): void {
		this._glCheckError('drawBuffers');
		this.native.drawBuffers(buffers);
	}

	drawElementsInstanced(
		mode: number,
		count: number,
		type: number,
		offset: number,
		instanceCount: number
	): void {
		this._glCheckError('drawElementsInstanced');
		this.native.drawElementsInstanced(mode, count, type, offset, instanceCount);
	}

	drawRangeElements(
		mode: number,
		start: number,
		end: number,
		count: number,
		type: number,
		offset: number
	): void {
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

	framebufferTextureLayer(
		target: number,
		attachment: number,
		texture: WebGLTexture,
		level: number,
		layer: number
	): void {
		this._glCheckError('framebufferTextureLayer');
		const value = texture ? texture.native : 0;
		this.native.framebufferTextureLayer(
			target,
			attachment,
			value,
			level,
			layer
		);
	}

	/* Framebuffers and renderbuffers */


	/* Uniforms */

	getActiveUniformBlockName(
		program: WebGLProgram,
		uniformBlockIndex: number
	): string {
		this._glCheckError('getActiveUniformBlockName');
		const value = program ? program.native : 0;
		return this.native.getActiveUniformBlockName(
			value,
			uniformBlockIndex
		);
	}

	getActiveUniformBlockParameter(
		program: WebGLProgram,
		uniformBlockIndex: number,
		pname: number
	): any {
		this._glCheckError('getActiveUniformBlockParameter');
		const value = program ? program.native : 0;
		const param = this.native.getActiveUniformBlockParameter(
			value,
			uniformBlockIndex,
			pname
		);
		if (pname === this.UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES) {
			return new Uint32Array(this.getJSArray(param));
		}
		return WebGL2RenderingContext.toPrimitive(param);
	}

	getActiveUniforms(
		program: WebGLProgram,
		uniformIndices: number[],
		pname: number
	): any {
		this._glCheckError('getActiveUniforms');
		const value = program ? program.native : 0;
		return this.getJSArray(
			this.native.getActiveUniforms(value, uniformIndices, pname)
		);
	}

	getBufferSubData(
		target: number,
		srcByteOffset: number,
		dstData: ArrayBuffer,
		dstOffset: number = 0,
		length: number = 0
	): void {
		this._glCheckError('getBufferSubData');
		this.native.getBufferSubData(
			target,
			srcByteOffset,
			new Uint8Array(dstData.slice(0)) as any,
			dstOffset,
			length
		);
	}

	getFragDataLocation(program: WebGLProgram, name: string): number {
		this._glCheckError('getFragDataLocation');
		const value = program ? program.native : 0;
		const result = this.native.getFragDataLocation(value, name);
		return result !== -1 ? result : null;
	}

	getIndexedParameter(target: number, index: number): any {
		this._glCheckError('getIndexedParameter');
		const param = this.native.getIndexedParameter(target, index);
		if (
			target === this.TRANSFORM_FEEDBACK_BUFFER_BINDING ||
			target === this.UNIFORM_BUFFER_BINDING
		) {
			return new WebGLBuffer(param);
		}
		return WebGL2RenderingContext.toPrimitive(param);
	}

	getInternalformatParameter(
		target: number,
		internalformat: number,
		pname: number
	): any {
		this._glCheckError('getInternalformatParameter');
		const param = this.native.getInternalformatParameter(
			target,
			internalformat,
			pname
		);
		if (pname === this.SAMPLES) {
			return new Int32Array(this.getJSArray(param));
		}
		return WebGL2RenderingContext.toPrimitive(param);
	}

	getQueryParameter(query: WebGLQuery, pname: number): any {
		this._glCheckError('getQueryParameter');
		const value = query ? query.native : 0;
		const result = this.native.getQueryParameter(value, pname);
		if (result === 0) {
			return null;
		}
		return WebGL2RenderingContext.toPrimitive(result);
	}


	//@ts-ignore
	getParameter(pname: number): number[] | number | WebGLBuffer | WebGLProgram | WebGLFramebuffer | WebGLRenderbuffer | WebGLTexture | Uint32Array | Int32Array | Float32Array | string | null {
		this._glCheckError('getParameter');
		this._checkArgs('activeTexture', arguments);
		const value = this.native.getParameter(pname);
		switch (pname) {
			case this.COPY_READ_BUFFER_BINDING:
			case this.COPY_WRITE_BUFFER_BINDING:
				if (value) {
					new WebGLBuffer(WebGL2RenderingContext.toPrimitive(value));
				}
				return null;
			case this.DRAW_FRAMEBUFFER_BINDING:
				if (value) {
					return new WebGLFramebuffer(WebGL2RenderingContext.toPrimitive(value));
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
		const value = sampler ? sampler.native : 0;
		return WebGL2RenderingContext.toPrimitive(this.native.getSamplerParameter(value, pname));
	}

	getSyncParameter(sync: WebGLSync, pname: number): any {
		this._glCheckError('getSyncParameter');
		const value = sync ? sync.native : 0;
		return WebGL2RenderingContext.toPrimitive(this.native.getSyncParameter(value, pname));
	}

	getTransformFeedbackVarying(program: WebGLProgram, index: number): any {
		this._glCheckError('getTransformFeedbackVarying');
		const value = program ? program.native : 0;
		const info = this.native.getTransformFeedbackVarying(value, index);
		if (info) {
			return new WebGLActiveInfo(info.name, info.size, info.size);
		}
		return null;
	}

	getUniformBlockIndex(
		program: WebGLProgram,
		uniformBlockName: string
	): number {
		this._glCheckError('getUniformBlockIndex');
		const value = program ? program.native : 0;
		return this.native.getUniformBlockIndex(value, uniformBlockName);
	}

	getUniformIndices(program: WebGLProgram, uniformNames: string[]): number[] {
		this._glCheckError('getUniformIndices');
		const value = program ? program.native : 0;
		return this.getJSArray(
			this.native.getUniformIndices(value, uniformNames)
		);
	}

	invalidateFramebuffer(target: number, attachments: number[]): void {
		this._glCheckError('invalidateFramebuffer');
		this.native.invalidateFramebuffer(target, attachments);
	}

	invalidateSubFramebuffer(
		target: number,
		attachments: number[],
		x: number,
		y: number,
		width: number,
		height: number
	): void {
		this._glCheckError('invalidateSubFramebuffer');
		this.native.invalidateSubFramebuffer(
			target,
			attachments,
			x,
			y,
			width,
			height
		);
	}

	isQuery(query: WebGLQuery): boolean {
		this._glCheckError('isQuery');
		const value = query ? query.native : 0;
		return this.native.isQuery(value);
	}

	isSampler(sampler: WebGLSampler): boolean {
		this._glCheckError('isSampler');
		const value = sampler ? sampler.native : 0;
		return this.native.isSampler(value);
	}

	isSync(sync: WebGLSync): boolean {
		this._glCheckError('isSync');
		const value = sync ? sync.native : 0;
		return this.native.isSync(value);
	}

	isTransformFeedback(transformFeedback: WebGLTransformFeedback): boolean {
		this._glCheckError('isTransformFeedback');
		const value = transformFeedback ? transformFeedback.native : 0;
		return this.native.isTransformFeedback(value);
	}

	isVertexArray(vertexArray: WebGLVertexArrayObject): boolean {
		this._glCheckError('isVertexArray');
		const value = vertexArray ? vertexArray.native : 0;
		return this.native.isVertexArray(value);
	}

	pauseTransformFeedback(): void {
		this._glCheckError('pauseTransformFeedback');
		this.native.pauseTransformFeedback();
	}

	readBuffer(src: number): void {
		this._glCheckError('readBuffer');
		this.native.readBuffer(src);
	}

	renderbufferStorageMultisample(
		target: number,
		samples: number,
		internalFormat: number,
		width: number,
		height: number
	): void {
		this._glCheckError('renderbufferStorageMultisample');
		this.native.renderbufferStorageMultisample(
			target,
			samples,
			internalFormat,
			width,
			height
		);
	}

	resumeTransformFeedback(): void {
		this._glCheckError('resumeTransformFeedback');
		this.native.resumeTransformFeedback();
	}

	samplerParameterf(sampler: WebGLSampler, pname: number, param: number): void {
		this._glCheckError('samplerParameterf');
		const value = sampler ? sampler.native : 0;
		this.native.samplerParameterf(value, pname, param);
	}


	/* Uniforms */

	/* Sync objects */

	samplerParameteri(sampler: WebGLSampler, pname: number, param: number): void {
		this._glCheckError('samplerParameteri');
		const value = sampler ? sampler.native : 0;
		this.native.samplerParameteri(value, pname, param);
	}

	texImage3D(
		target: number,
		level: number,
		internalformat: number,
		width: number,
		height: number,
		depth: number,
		border: number,
		format: number,
		type: number,
		offset: any
	);

	texImage3D(
		target: number,
		level: number,
		internalformat: number,
		width: number,
		height: number,
		depth: number,
		border: number,
		format: number,
		type: number,
		source: any
	): void {
		this._glCheckError('texImage3D');
		if (typeof source === 'number') {
			this.native.texImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				source
			);
		} else if (source && source.buffer) {
			if (source instanceof Uint8Array) {
				this.native.texImage3D(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					this.toNativeArray(source as any, 'byte')
				);
			}
		} else if (source instanceof android.graphics.Bitmap) {
			this.native.texImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				source
			);
		} else if (source instanceof ImageSource) {
			this.native.texImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				source.android
			);
		} else if (source instanceof ImageAsset) {
			this.native.texImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				source.native
			);
		} else if (source instanceof ImageBitmap) {
			this.native.texImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				source.native
			);
		} else if (
			source &&
			typeof source.tagName === 'string' &&
			(source.tagName === 'IMG' || source.tagName === 'IMAGE')
		) {
			if (source._imageSource instanceof ImageSource) {
				this.native.texImage3D(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					source._imageSource.android
				);
			} else if (source._image instanceof android.graphics.Bitmap) {
				this.native.texImage3D(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					source._image
				);
			} else if (source._asset instanceof ImageAsset) {
				this.native.texImage3D(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					source._asset.native
				);
			} else if (typeof source.src === 'string') {
				const result = ImageSource.fromFileSync(source.src);
				this.native.texImage3D(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					result ? result.android : null
				);
			}
		} else if (source &&
			typeof source.tagName === 'string' &&
			source.tagName === 'CANVAS' && source._canvas instanceof Canvas) {
			this.native.texImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				source._canvas.android
			);
		}
	}

	texStorage2D(
		target: number,
		levels: number,
		internalformat: number,
		width: number,
		height: number
	): void {
		this._glCheckError('texStorage2D');
		this.native.texStorage2D(target, levels, internalformat, width, height);
	}

	texStorage3D(
		target: number,
		levels: number,
		internalformat: number,
		width: number,
		height: number,
		depth: number
	): void {
		this._glCheckError('texStorage3D');
		this.native.texStorage3D(
			target,
			levels,
			internalformat,
			width,
			height,
			depth
		);
	}

	texSubImage3D(
		target: number,
		level: number,
		xoffset: number,
		yoffset: number,
		zoffset: number,
		width: number,
		height: number,
		depth: number,
		format: number,
		type: number,
		offset: any
	);

	texSubImage3D(
		target: number,
		level: number,
		xoffset: number,
		yoffset: number,
		zoffset: number,
		width: number,
		height: number,
		depth: number,
		format: number,
		type: number,
		srcData: any
	);

	texSubImage3D(
		target: number,
		level: number,
		xoffset: number,
		yoffset: number,
		zoffset: number,
		width: number,
		height: number,
		depth: number,
		format: number,
		type: number,
		srcData: any,
		srcOffset: number = 0
	): void {
		this._glCheckError('texSubImage3D');
		if (typeof srcData === 'number') {
			this.native.texSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				srcData
			);
		} else if (srcData && srcData.buffer) {
			if (srcData instanceof Uint8Array) {
				if (srcOffset) {
					this.native.texSubImage3D(
						target,
						level,
						xoffset,
						yoffset,
						zoffset,
						width,
						height,
						depth,
						format,
						type,
						this.toNativeArray(srcData as any, 'byte'),
						srcOffset
					);
				} else {
					this.native.texSubImage3D(
						target,
						level,
						xoffset,
						yoffset,
						zoffset,
						width,
						height,
						depth,
						format,
						type,
						this.toNativeArray(srcData as any, 'byte')
					);
				}
			}
		} else if (srcData instanceof android.graphics.Bitmap) {
			this.native.texSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				srcData
			);
		} else if (srcData instanceof ImageSource) {
			this.native.texSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				srcData.android
			);
		} else if (srcData instanceof ImageAsset) {
			this.native.texSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				srcData.native
			);
		} else if (srcData instanceof ImageBitmap) {
			this.native.texSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				srcData.native
			);
		} else if (
			srcData &&
			typeof srcData.tagName === 'string' &&
			(srcData.tagName === 'IMG' || srcData.tagName === 'IMAGE')
		) {
			if (srcData._imageSource instanceof ImageSource) {
				this.native.texSubImage3D(
					target,
					level,
					xoffset,
					yoffset,
					zoffset,
					width,
					height,
					depth,
					format,
					type,
					srcData._imageSource.android
				);
			} else if (srcData._image instanceof android.graphics.Bitmap) {
				this.native.texSubImage3D(
					target,
					level,
					xoffset,
					yoffset,
					zoffset,
					width,
					height,
					depth,
					format,
					type,
					srcData._image
				);
			} else if (srcData._asset instanceof ImageAsset) {
				this.native.texSubImage3D(
					target,
					level,
					xoffset,
					yoffset,
					zoffset,
					width,
					height,
					depth,
					format,
					type,
					srcData._asset.native
				);
			} else if (typeof srcData.src === 'string') {
				const result = ImageSource.fromFileSync(srcData.src);
				this.native.texSubImage3D(
					target,
					level,
					xoffset,
					yoffset,
					zoffset,
					width,
					height,
					depth,
					format,
					type,
					result ? result.android : null
				);
			}
		} else if (srcData &&
			typeof srcData.tagName === 'string' &&
			srcData.tagName === 'CANVAS' && srcData._canvas instanceof TNSCanvas) {
			this.native.texSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				srcData._canvas.android
			);
		}
	}

	transformFeedbackVaryings(
		program: WebGLProgram,
		varyings: string[],
		bufferMode: number
	): void {
		this._glCheckError('transformFeedbackVaryings');
		const value = program ? program.native : 0;
		this.native.transformFeedbackVaryings(value, varyings, bufferMode);
	}

	uniform1ui(location: WebGLUniformLocation, v0: number): void {
		this._glCheckError('uniform1ui');
		const value = location ? location.native : 0;
		this.native.uniform1ui(value, v0);
	}

	uniform1uiv(location: WebGLUniformLocation, data: Uint32Array): void {
		this._glCheckError('uniform1uiv');
		const value = location ? location.native : 0;
		this.native.uniform1uiv(value, Array.from(data as any));
	}

	uniform2ui(location: WebGLUniformLocation, v0: number, v1: number): void {
		this._glCheckError('uniform2ui');
		const value = location ? location.native : 0;
		this.native.uniform2ui(value, v0, v1);
	}

	uniform2uiv(location: WebGLUniformLocation, data: Uint32Array): void {
		this._glCheckError('uniform2uiv');
		const value = location ? location.native : 0;
		this.native.uniform2uiv(value, Array.from(data as any));
	}

	/* Sync objects */

	/* Miscellaneous constants */

	uniform3ui(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void {
		this._glCheckError('uniform3ui');
		const value = location ? location.native : 0;
		this.native.uniform3ui(value, v0, v1, v2);
	}

	uniform3uiv(location: WebGLUniformLocation, data: Uint32Array): void {
		this._glCheckError('uniform3uiv');
		const value = location ? location.native : 0;
		this.native.uniform3uiv(value, Array.from(data as any));
	}

	uniform4ui(
		location: WebGLUniformLocation,
		v0: number,
		v1: number,
		v2: number,
		v3: number
	): void {
		this._glCheckError('uniform4ui');
		const value = location ? location.native : 0;
		this.native.uniform4ui(value, v0, v1, v2, v3);
	}

	uniform4uiv(location: WebGLUniformLocation, data: Uint32Array): void {
		this._glCheckError('uniform4uiv');
		const value = location ? location.native : 0;
		this.native.uniform4uiv(value, Array.from(data as any));
	}

	uniformBlockBinding(
		program: WebGLProgram,
		uniformBlockIndex: number,
		uniformBlockBinding: number
	): void {
		this._glCheckError('uniformBlockBinding');
		const value = program ? program.native : 0;
		this.native.uniformBlockBinding(
			value,
			uniformBlockIndex,
			uniformBlockBinding
		);
	}

	uniformMatrix2x3fv(
		location: WebGLUniformLocation,
		transpose: boolean,
		data: Float32Array
	): void {
		if (data instanceof Float32Array) {
			data = Array.from(data as any) as any; //this.toNativeArray(data as any, 'float');
		}
		this._glCheckError('uniformMatrix2x3fv');
		const value = location ? location.native : 0;
		this.native.uniformMatrix2x3fv(value, transpose, data as any);
	}

	uniformMatrix2x4fv(
		location: WebGLUniformLocation,
		transpose: boolean,
		data: Float32Array
	): void {
		if (data instanceof Float32Array) {
			data = Array.from(data as any) as any; //this.toNativeArray(data as any, 'float');
		}
		this._glCheckError('uniformMatrix2x4fv');
		const value = location ? location.native : 0;
		this.native.uniformMatrix2x4fv(value, transpose, data as any);
	}

	uniformMatrix3x2fv(
		location: WebGLUniformLocation,
		transpose: boolean,
		data: Float32Array
	): void {
		if (data instanceof Float32Array) {
			data = Array.from(data as any) as any; //this.toNativeArray(data as any, 'float');
		}
		this._glCheckError('uniformMatrix3x2fv');
		const value = location ? location.native : 0;
		this.native.uniformMatrix3x2fv(value, transpose, data as any);
	}

	uniformMatrix3x4fv(
		location: WebGLUniformLocation,
		transpose: boolean,
		data: Float32Array
	): void {
		if (data instanceof Float32Array) {
			data = Array.from(data as any) as any; //this.toNativeArray(data as any, 'float');
		}
		this._glCheckError('uniformMatrix3x4fv');
		const value = location ? location.native : 0;
		this.native.uniformMatrix3x4fv(value, transpose, data as any);
	}

	uniformMatrix4x2fv(
		location: WebGLUniformLocation,
		transpose: boolean,
		data: Float32Array
	): void {
		if (data instanceof Float32Array) {
			data = Array.from(data as any) as any; //this.toNativeArray(data as any, 'float');
		}
		this._glCheckError('uniformMatrix4x2fv');
		const value = location ? location.native : 0;
		this.native.uniformMatrix4x2fv(value, transpose, data as any);
	}

	uniformMatrix4x3fv(
		location: WebGLUniformLocation,
		transpose: boolean,
		data: Float32Array
	): void {
		if (data instanceof Float32Array) {
			data = Array.from(data as any) as any; //this.toNativeArray(data as any, 'float');
		}
		this._glCheckError('uniformMatrix4x3fv');
		const value = location ? location.native : 0;
		this.native.uniformMatrix4x3fv(value, transpose, data as any);
	}

	vertexAttribDivisor(index: number, divisor: number): void {
		this._glCheckError('vertexAttribDivisor');
		this.native.vertexAttribDivisor(index, divisor);
	}

	vertexAttribI4i(
		index: number,
		v0: number,
		v1: number,
		v2: number,
		v3: number
	): void {
		this._glCheckError('vertexAttribI4i');
		this.native.vertexAttribI4i(index, v0, v1, v2, v3);
	}

	vertexAttribI4iv(index: number, value: number[] | Int32Array): void {
		if (value instanceof Int32Array) {
			value = Array.from(value) as any; //this.toNativeArray(value as any, 'int');
		}
		this._glCheckError('vertexAttribI4iv');
		this.native.vertexAttribI4uiv(index, value as any);
	}

	vertexAttribI4ui(
		index: number,
		v0: number,
		v1: number,
		v2: number,
		v3: number
	): void {
		this._glCheckError('vertexAttribI4ui');
		this.native.vertexAttribI4ui(index, v0, v1, v2, v3);
	}

	vertexAttribI4uiv(index: number, value: number[] | Uint32Array): void {
		if (value instanceof Uint32Array) {
			value = Array.from(value) as any; //this.toNativeArray(value as any, 'int');
		}
		this._glCheckError('vertexAttribI4uiv');
		this.native.vertexAttribI4uiv(index, value as any);
	}

	/* Miscellaneous constants */
}
