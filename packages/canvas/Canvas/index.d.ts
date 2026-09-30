import { DOMMatrix } from '../Canvas2D';
import { CanvasBase } from './common';
import { CanvasRenderingContext2D } from '../Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from '../WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from '../WebGL2/WebGL2RenderingContext';
import { GPUCanvasContext } from '../WebGPU';
import { ImageBitmapRenderingContext } from '../ImageBitmapRenderingContext';
import type { OffscreenCanvas } from '../OffscreenCanvas';
import { LengthPercentage } from '@nativescript/core/css/parser';

export declare function createSVGMatrix(): DOMMatrix;

export class TouchEvent {
	readonly type: string;
	constructor(name, init?: { [key: string]: any });
	preventDefault();
	stopPropagation();
}

export class PointerEvent {
	readonly type: string;
	constructor(name, init?: { [key: string]: any });
	preventDefault();
	stopPropagation();
}

export declare class Canvas extends CanvasBase {
	readonly clientWidth: number;
	readonly clientHeight: number;
	native: any;

	set width(value: LengthPercentage | number | string | undefined);
	get width(): number;
	set height(value: LengthPercentage | number | string | undefined);
	get height(): number;
	lang: string;

	constructor();

	static useSurface: boolean;

	/** Default for `getContext('2d', { threaded })`: rasterize on a shared render thread (Android, iOS, Windows). */
	static threaded2D: boolean;

	/** Default for `getContext('webgl' | 'webgl2', { threaded })`: run the context on a shared WebGL thread (Android, iOS, Windows). */
	static threadedWebGL: boolean;

	surfaceOnTop: boolean;

	static forceGL: boolean;

	static createCustomView(): Canvas;

	createNativeView(): any;

	initNativeView(): void;

	disposeNativeView(): void;

	toDataURL(type?: string, encoderOptions?: number): any;

	/**
	 * Hands drawing over to an OffscreenCanvas that draws into this canvas's surface. Throws an
	 * `InvalidStateError` if the canvas already has a context or was transferred; afterwards its
	 * getContext() and width/height setters throw too.
	 */
	transferControlToOffscreen(): OffscreenCanvas;

	getContext(type: '2d', options?: any): CanvasRenderingContext2D | null;

	getContext(type: 'bitmaprenderer', options?: { alpha?: boolean }): ImageBitmapRenderingContext | null;

	getContext(type: 'webgl' | 'experimental-webgl', options?: any): WebGLRenderingContext | null;

	getContext(type: 'webgl2' | 'experimental-webgl2', options?: any): WebGL2RenderingContext | null;

	getContext(type: 'webgpu'): GPUCanvasContext | null;

	getContext(type: string, options?: any): CanvasRenderingContext2D | ImageBitmapRenderingContext | WebGLRenderingContext | WebGL2RenderingContext | GPUCanvasContext | null;

	getBoundingClientRect(): {
		x: number;
		y: number;
		width: number;
		height: number;
		top: number;
		right: number;
		bottom: number;
		left: number;
	};

	snapshot(): ImageSource | null;

	snapshot(flip?: boolean): ImageSource | null;

	toHTMLCanvas?(): any;
}
