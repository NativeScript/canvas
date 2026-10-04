import { TouchEvent, PointerEvent, CustomEvent } from './Canvas/common';

import { installCanvasGlobals } from './globals';

export * from './Canvas2D';
export * from './ImageBitmap';
export * from './ImageBitmapRenderingContext';
export * from './OffscreenCanvas';
export * from './ImageAsset';
export * from './TextEncoder';
export * from './TextDecoder';
export * from './WebGL';
export * from './WebGL2';
export * from './WebGPU';

import { FontFace, FontFaceSet } from '@nativescript/font-manager';
import { NAPI_HOST } from './platform';
import { onFontLoaded } from './platform/index';

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

if (NAPI_HOST) {
	onFontLoaded((path, family) => {
		try {
			// As on the other platforms: a TrueType font registers under its own name.
			const useAlias = !/\.ttf$/i.test(path);
			global.CanvasModule.__addFontFamily(useAlias ? family : null, [path]);
		} catch {}
	});
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

installCanvasGlobals();

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

Object.defineProperty(global, 'CustomEvent', {
	value: CustomEvent,
	configurable: true,
	writable: true,
});

export { ImageBitmap } from './ImageBitmap';
export { ImageBitmapRenderingContext } from './ImageBitmapRenderingContext';
export { CanvasRenderingContext2D } from './Canvas2D/CanvasRenderingContext2D';
export { WebGLRenderingContext } from './WebGL/WebGLRenderingContext';
export { WebGL2RenderingContext } from './WebGL2/WebGL2RenderingContext';
export { Canvas, createSVGMatrix } from './Canvas';
export { TouchEvent, PointerEvent } from './Canvas/common';
export { FontFace, FontFaceSet, importFontsFromCSS, loadFontsFromCSS } from '@nativescript/font-manager';
export type { stretch, stretchName, stretchPercent } from '@nativescript/font-manager';
