// None of these is, or loads, a view.
import { TextEncoder } from './TextEncoder';
import { TextDecoder } from './TextDecoder';
import { ImageBitmap } from './ImageBitmap';
import { ImageBitmapRenderingContext } from './ImageBitmapRenderingContext';
import { OffscreenCanvas, OffscreenCanvasRenderingContext2D } from './OffscreenCanvas';
import { ImageAsset } from './ImageAsset';
import { CanvasPattern, CanvasGradient, Path2D, ImageData, DOMMatrix } from './Canvas2D';
import { CanvasRenderingContext2D } from './Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from './WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from './WebGL2/WebGL2RenderingContext';
import { GPUBufferUsage, GPUMapMode, GPUShaderStage, GPUTextureUsage } from './WebGPU/Constants';

export function installCanvasGlobals() {
	const values = {
		OffscreenCanvas,
		OffscreenCanvasRenderingContext2D,
		CanvasRenderingContext2D,
		WebGLRenderingContext,
		WebGL2RenderingContext,
		CanvasPattern,
		CanvasGradient,
		TextEncoder,
		TextDecoder,
		Path2D,
		ImageData,
		DOMMatrix,
		ImageBitmap,
		ImageBitmapRenderingContext,
		ImageAsset,
		GPUBufferUsage,
		GPUTextureUsage,
		GPUMapMode,
		GPUShaderStage,
	};
	for (const name of Object.keys(values)) {
		Object.defineProperty(global, name, {
			value: values[name],
			configurable: true,
			writable: true,
		});
	}
}
