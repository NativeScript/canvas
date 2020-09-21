import {
	WebGLRenderingContextBase,
} from './common';

import {WebGLShader} from '../WebGLShader';
import {WebGLFramebuffer} from '../WebGLFramebuffer';
import {WebGLTexture} from '../WebGLTexture';
import {WebGLProgram} from '../WebGLProgram';
import {WebGLUniformLocation} from '../WebGLUniformLocation';
import {WebGLActiveInfo} from '../WebGLActiveInfo';
import {WebGLRenderbuffer} from '../WebGLRenderbuffer';
import {WebGLShaderPrecisionFormat} from '../WebGLShaderPrecisionFormat';
import {WebGLBuffer} from '../WebGLBuffer';

export declare class WebGLRenderingContext extends WebGLRenderingContextBase {
	readonly native: any;
	readonly drawingBufferHeight: number;
	readonly drawingBufferWidth: number;
	public static isDebug: boolean;
	public static filter: 'both' | 'error' | 'args';
	private context;

	constructor(context: any);

	activeTexture(texture: number): void;

	attachShader(program: WebGLProgram, shader: WebGLShader): void;

	bindAttribLocation(program: WebGLProgram, index: number, name: string): void;

	bindBuffer(target: number, buffer: WebGLBuffer): void;

	bindFramebuffer(target: number, framebuffer: WebGLFramebuffer): void;

	bindRenderbuffer(target: number, renderbuffer: WebGLRenderbuffer): void;

	bindTexture(target: number, texture: WebGLTexture): void;

	blendColor(red: number, green: number, blue: number, alpha: number): void;

	blendEquationSeparate(modeRGB: number, modeAlpha: number): void;

	blendEquation(mode: number): void;

	blendFuncSeparate(srcRGB?: number, dstRGB?: number, srcAlpha?: number, dstAlpha?: number): void;

	blendFunc(sfactor?: number, dfactor?: number): void;

	toNativeArray(value: any, type: string): any;

	bufferData(target: number, size: number, usage: number): void;
	bufferData(target: number, srcData: ArrayBuffer | ArrayBufferView, usage: number): void;

	bufferSubData(target: number, offset: number, srcData: ArrayBuffer | ArrayBufferView): void;

	checkFramebufferStatus(target: number): number;

	clearColor(red: number, green: number, blue: number, alpha: number): void;

	clearDepth(depth: number): void;

	clearStencil(stencil: number): void;

	clear(mask: number): void;

	colorMask(red: boolean, green: boolean, blue: boolean, alpha: boolean): void;

	commit(): void;

	compileShader(shader: WebGLShader): void;

	compressedTexImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, pixels: ArrayBufferView): void;

	compressedTexSubImage2D(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, pixels: ArrayBufferView): void;

	copyTexImage2D(target: number, level: number, internalformat: number, x: number, y: number, width: number, height: number, border: number): void;

	copyTexSubImage2D(target: number, level: number, xoffset: number, yoffset: number, x: number, y: number, width: number, height: number): void;

	createBuffer(): WebGLBuffer;

	createFramebuffer(): WebGLFramebuffer;

	createProgram(): WebGLProgram;

	createRenderbuffer(): WebGLRenderbuffer;

	createShader(type: number): WebGLShader;

	createTexture(): WebGLTexture;

	cullFace(mode: number): void;

	deleteBuffer(buffer: WebGLBuffer): void;

	deleteFramebuffer(frameBuffer: WebGLFramebuffer): void;

	deleteProgram(program: WebGLProgram): void;

	deleteRenderbuffer(renderBuffer: WebGLRenderbuffer): void;

	deleteShader(shader: WebGLRenderbuffer): void;

	deleteTexture(texture: WebGLTexture): void;

	depthFunc(func: number): void;

	depthMask(flag: boolean): void;

	depthRange(zNear: number, zFar: number): void;

	detachShader(program: WebGLProgram, shader: WebGLShader): void;

	disableVertexAttribArray(index: number): void;

	disable(cap: number): void;

	drawArrays(mode: number, first: number, count: number): void;

	drawElements(mode: number, count: number, type: number, offset: number): void;

	enableVertexAttribArray(index: number): void;

	enable(cap: number): void;

	finish(): void;

	flush(): void;

	framebufferRenderbuffer(target: number, attachment: number, renderbuffertarget: number, renderbuffer: WebGLRenderbuffer): void;

	framebufferTexture2D(target: number, attachment: number, textarget: number, texture: WebGLTexture, level: number): void;

	frontFace(mode: number): void;

	generateMipmap(target: number): void;

	getActiveAttrib(program: WebGLProgram, index: number): WebGLActiveInfo;

	getActiveUniform(program: WebGLProgram, index: number): WebGLActiveInfo;

	getAttachedShaders(program: WebGLProgram): WebGLShader[];

	getAttribLocation(program: WebGLProgram, name: string): number;

	getBufferParameter(target: number, pname: number): number;

	getContextAttributes(): any;

	getError(): number;

	getExtension(name: string): any;

	getFramebufferAttachmentParameter(target: number, attachment: number, pname: number): number | WebGLRenderbuffer | WebGLTexture;

	getJSArray(nativeArray: any): any[];

	getParameter(pname: number): number[] | number | WebGLBuffer | WebGLProgram | WebGLFramebuffer | WebGLRenderbuffer | WebGLTexture | Uint32Array | Int32Array | Float32Array | null;

	getProgramInfoLog(program: WebGLProgram): string;

	getProgramParameter(program: WebGLProgram, pname: number): number | boolean;

	getRenderbufferParameter(target: number, pname: number): number;

	getShaderInfoLog(shader: WebGLShader): string;

	getShaderParameter(shader: WebGLShader, pname: number): boolean | number;

	getShaderPrecisionFormat(shaderType: number, precisionType: number): WebGLShaderPrecisionFormat;

	getShaderSource(shader: WebGLShader): string;

	getSupportedExtensions(): string[];

	getTexParameter(target: number, pname: number): number;

	getUniformLocation(program: WebGLProgram, name: string): WebGLUniformLocation;

	getUniform(program: WebGLProgram, location: WebGLUniformLocation): any;

	getVertexAttribOffset(index: number, pname: number): number;

	getVertexAttrib(index: number, pname: number): number[] | boolean | number | Float32Array;

	hint(target: number, mode: number): void;

	isBuffer(buffer: WebGLBuffer): boolean;

	isContextLost(): boolean;

	isEnabled(cap: number): boolean;

	isFramebuffer(framebuffer: WebGLFramebuffer): boolean;

	isProgram(program: WebGLProgram): boolean;

	isRenderbuffer(renderbuffer: WebGLRenderbuffer): boolean;

	isShader(shader: WebGLShader): boolean;

	isTexture(texture: WebGLTexture): boolean;

	lineWidth(width: number): void;

	linkProgram(program: WebGLProgram): void;

	pixelStorei(pname: number, param: any): void;

	polygonOffset(factor: number, units: number): void;

	readPixels(x: number, y: number, width: number, height: number, format: number, type: number, pixels: ArrayBufferView): void;

	renderbufferStorage(target: number, internalFormat: number, width: number, height: number): void;

	sampleCoverage(value: number, invert: boolean): void;

	scissor(x: number, y: number, width: number, height: number): void;

	shaderSource(shader: WebGLShader, source: string): void;

	stencilFuncSeparate(face: number, func: number, ref: number, mask: number): void;

	stencilFunc(func: number, ref: number, mask: number): void;

	stencilMaskSeparate(face: number, mask: number): void;

	stencilMask(mask: number): void;

	stencilOpSeparate(face: number, fail: number, zfail: number, zpass: number): void;

	stencilOp(fail: number, zfail: number, zpass: number): void;

	texImage2D(target: number, level: number, internalformat: number, format: number, type: number, pixels?: any): void;
	texImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type: number, pixels: ArrayBufferView): void;

	texParameterf(target: number, pname: number, param: number): void;

	texParameteri(target: number, pname: number, param: number): void;

	texSubImage2D(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, type: number, pixels?: ArrayBufferView): void;
	texSubImage2D(target: number, level: number, xoffset: number, yoffset: number, format: number, type: number, pixels?: any): void;

	uniform1f(location: WebGLUniformLocation, v0: number): void;

	uniform1iv(location: WebGLUniformLocation, value: number[]): void;

	uniform1fv(location: WebGLUniformLocation, value: number[]): void;

	uniform1i(location: WebGLUniformLocation, v0: number): void;

	uniform2f(location: WebGLUniformLocation, v0: number, v1: number): void;

	uniform2iv(location: WebGLUniformLocation, value: number[]): void;

	uniform2fv(location: WebGLUniformLocation, value: number[]): void;

	uniform2i(location: WebGLUniformLocation, v0: number, v1: number): void;

	uniform3f(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void;

	uniform3iv(location: WebGLUniformLocation, value: number[]): void;

	uniform3fv(location: WebGLUniformLocation, value: number[]): void;

	uniform3i(location: WebGLUniformLocation, v0: number, v1: number, v2: number): void;

	uniform4f(location: WebGLUniformLocation, v0: number, v1: number, v2: number, v3: number): void;

	uniform4iv(location: WebGLUniformLocation, value: number[]): void;

	uniform4fv(location: WebGLUniformLocation, value: number[]): void;

	uniform4i(location: WebGLUniformLocation, v0: number, v1: number, v2: number, v3: number): void;

	uniformMatrix2fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void;

	uniformMatrix3fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void;

	uniformMatrix4fv(location: WebGLUniformLocation, transpose: boolean, value: number[]): void;

	useProgram(program: WebGLProgram): void;

	validateProgram(program: WebGLProgram): void;

	vertexAttrib1f(index: number, v0: number): void;

	vertexAttrib1fv(index: number, value: number[]): void;

	vertexAttrib2f(index: number, v0: number, v1: number): void;

	vertexAttrib2fv(index: number, value: number[]): void;

	vertexAttrib3f(index: number, v0: number, v1: number, v2: number): void;

	vertexAttrib3fv(index: number, value: number[]): void;

	vertexAttrib4f(index: number, v0: number, v1: number, v2: number, v3: number): void;

	vertexAttrib4fv(index: number, value: number[]): void;

	vertexAttribPointer(index: number, size: number, type: number, normalized: boolean, stride: number, offset: number): void;

	viewport(x: number, y: number, width: number, height: number): void;

}
