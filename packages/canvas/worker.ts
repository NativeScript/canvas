// The canvas API without the `Canvas` view, for Workers.
import '@nativescript/core/globals';
import { Helpers } from './helpers';
import { installCanvasGlobals } from './globals';
import { GPU } from './WebGPU/GPU';

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

declare const org;

if (__ANDROID__) {
	// JNI_OnLoad otherwise runs only when the view's class loads.
	org.nativescript.canvas.NSCCanvas.loadLib();
}
Helpers.initialize();
installCanvasGlobals();

const scope: any = global;
scope.navigator ??= {};
if (!('gpu' in scope.navigator)) {
	const gpu = new GPU();
	Object.defineProperty(scope.navigator, 'gpu', { get: () => gpu, configurable: true });
}
