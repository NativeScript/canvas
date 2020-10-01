import {CanvasRenderingContext} from './common';

export {CanvasRenderingContext};
export {Canvas, createSVGMatrix} from './Canvas';
export * from './Canvas2D';
export * from './ImageAsset';
import {TextEncoder} from './TextEncoder';
import {TextDecoder} from './TextDecoder';

export * from './TextEncoder';
export * from './TextDecoder';
export * from './WebGL';
export * from './WebGL2';

export * from './SVG';

export {CanvasRenderingContext2D} from './Canvas2D/CanvasRenderingContext2D';
export {WebGLRenderingContext} from './WebGL/WebGLRenderingContext';
export {WebGL2RenderingContext} from './WebGL2/WebGL2RenderingContext';
import {CanvasPattern, CanvasGradient, Path2D} from './Canvas2D';

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
