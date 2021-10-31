import { TouchEvent, PointerEvent } from './Canvas';

export { Canvas, createSVGMatrix, TouchEvent, PointerEvent } from './Canvas';
import { TextEncoder } from './TextEncoder';
import { TextDecoder } from './TextDecoder';
import { ImageBitmap } from './ImageBitmap';

import { CanvasPattern, CanvasGradient, Path2D, ImageData, DOMMatrix } from './Canvas2D';

import { CanvasRenderingContext2D } from './Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from './WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from './WebGL2/WebGL2RenderingContext';

export * from './Canvas2D';
export * from './ImageBitmap';
export * from './ImageAsset';
export * from './TextEncoder';
export * from './TextDecoder';
export * from './WebGL';
export * from './WebGL2';
export * from './SVG';


Object.defineProperty(global, 'CanvasRenderingContext2D', {
	value: CanvasRenderingContext2D,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'WebGLRenderingContext', {
	value: WebGLRenderingContext,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'WebGL2RenderingContext', {
	value: WebGL2RenderingContext,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'CanvasPattern', {
	value: CanvasPattern,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'CanvasGradient', {
	value: CanvasGradient,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'TextEncoder', {
	value: TextEncoder,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'TextDecoder', {
	value: TextDecoder,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'Path2D', {
	value: Path2D,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'ImageData', {
	value: ImageData,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'DOMMatrix', {
	value: DOMMatrix,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'ImageBitmap', {
	value: ImageBitmap,
	configurable: true,
	writable: true,
});


Object.defineProperty(global, 'TouchEvent', {
	value: TouchEvent,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'PointerEvent', {
	value: PointerEvent,
	configurable: true,
	writable: true,
});

export { ImageBitmap } from './ImageBitmap'
export { CanvasRenderingContext2D } from './Canvas2D/CanvasRenderingContext2D';
export { WebGLRenderingContext } from './WebGL/WebGLRenderingContext';
export { WebGL2RenderingContext } from './WebGL2/WebGL2RenderingContext';