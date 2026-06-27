import { TouchEvent, PointerEvent, CustomEvent } from './Canvas/common';

import { TextEncoder } from './TextEncoder';
import { TextDecoder } from './TextDecoder';
import { ImageBitmap } from './ImageBitmap';
import { ImageAsset } from './ImageAsset';

import { CanvasPattern, CanvasGradient, Path2D, ImageData, DOMMatrix } from './Canvas2D';

import { CanvasRenderingContext2D } from './Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from './WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from './WebGL2/WebGL2RenderingContext';
import { GPUBufferUsage, GPUMapMode, GPUShaderStage, GPUTextureUsage } from './WebGPU/Constants';

export * from './Canvas2D';
export * from './ImageBitmap';
export * from './ImageAsset';
export * from './TextEncoder';
export * from './TextDecoder';
export * from './WebGL';
export * from './WebGL2';
export * from './WebGPU';

import { FontFace, FontFaceSet } from '@nativescript/font-manager';

const url_ex = /url\(([^)]+?)\.(woff2?|ttf|otf|eot)\)/;
declare const org, kotlin;
if (__ANDROID__) {
	try {
		org.nativescript.fontmanager.FontFaceSet.getInstance().addOnLoadingDoneListener(
			new kotlin.jvm.functions.Function1({
				invoke(font) {
					const path = font.getFontPath();
					if (path) {
						const name = font.getFontFamily();

						let extension: string | undefined;

						if (typeof path === 'string') {
							const matches = path.match(url_ex) ?? [];
							extension = matches[2];
						}

						const useAlias = extension !== 'ttf';
						global.CanvasModule.__addFontFamily(useAlias ? name : null, [path]);
					}
				},
			}),
		);
	} catch {}
}

if (__APPLE__) {
	NSCFontFaceSet.instance().addOnLoadingDoneListener((font) => {
		try {
			const path = font.fontPath;
			if (path) {
				const name = font.family;

				let extension: string | undefined;

				if (typeof path === 'string') {
					const matches = path.match(url_ex) ?? [];
					extension = matches[2];
				}

				const useAlias = extension !== 'ttf';
				global.CanvasModule.__addFontFamily(useAlias ? name : null, [path]);
			}
		} catch {}
	});
}

Object.defineProperty(global, 'fonts', {
	value: new FontFaceSet(),
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'FontFace', {
	value: FontFace,
	configurable: true,
	writable: true,
});

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

Object.defineProperty(global, 'ImageAsset', {
	value: ImageAsset,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'GPUBufferUsage', {
	value: GPUBufferUsage,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'GPUTextureUsage', {
	value: GPUTextureUsage,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'GPUMapMode', {
	value: GPUMapMode,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'GPUShaderStage', {
	value: GPUShaderStage,
	configurable: true,
	writable: true,
});

Object.defineProperty(global, 'CustomEvent', {
	value: CustomEvent,
	configurable: true,
	writable: true,
});

export { ImageBitmap } from './ImageBitmap';
export { CanvasRenderingContext2D } from './Canvas2D/CanvasRenderingContext2D';
export { WebGLRenderingContext } from './WebGL/WebGLRenderingContext';
export { WebGL2RenderingContext } from './WebGL2/WebGL2RenderingContext';
export { Canvas, createSVGMatrix } from './Canvas';
export { TouchEvent, PointerEvent } from './Canvas/common';
export { FontFace, FontFaceSet, importFontsFromCSS, loadFontsFromCSS } from '@nativescript/font-manager';
export type { stretch, stretchName, stretchPercent } from '@nativescript/font-manager';
