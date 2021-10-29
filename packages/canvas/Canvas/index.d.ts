import {DOMMatrix} from '../Canvas2D';
import {CanvasBase} from './common';
import {CanvasRenderingContext2D} from '../Canvas2D/CanvasRenderingContext2D';
import {WebGLRenderingContext} from '../WebGL/WebGLRenderingContext';
import {WebGL2RenderingContext} from '../WebGL2/WebGL2RenderingContext';

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

	constructor();

	flush(): void;

	static createCustomView(): Canvas;

	createNativeView(): any;

	initNativeView(): void;

	disposeNativeView(): void;

	toDataURL(type?: string, encoderOptions?: number): any;

	getContext(type: string, options?: any): CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | null;

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
}
