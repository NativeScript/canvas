import { DOMMatrix } from '../Canvas2D';
import { CanvasBase } from './common';
import { CanvasRenderingContext2D } from '../Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from '../WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from '../WebGL2/WebGL2RenderingContext';
import { GPUCanvasContext } from '../WebGPU';
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
	private _2dContext;
	private canvas;
	native: any;
	parentElement: any;

	set width(value: LengthPercentage | number | string);
	get width(): number;
	set height(value: LengthPercentage | number | string);
	get height(): number;

	constructor();

	flush(): void;

	static useSurface: boolean;

	static forceGL: boolean;

	static createCustomView(): Canvas;

	createNativeView(): any;

	initNativeView(): void;

	disposeNativeView(): void;

	toDataURL(type?: string, encoderOptions?: number): any;

	getContext(type: '2d', options?: any): CanvasRenderingContext2D | null;

	getContext(type: 'bitmaprenderer', options?: any): any;

	getContext(type: 'webgl' | 'experimental-webgl', options?: any): WebGLRenderingContext | null;

	getContext(type: 'webgl2' | 'experimental-webgl2', options?: any): WebGL2RenderingContext | null;

	getContext(type: 'webgpu'): GPUCanvasContext | null;

	getContext(type: string, options?: any): CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | GPUCanvasContext | null;

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
}
