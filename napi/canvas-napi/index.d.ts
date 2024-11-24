/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export type JSWebGLRenderingContext = WebGLRenderingContext;
export declare class WebGLRenderingContext {
	get drawingBufferWidth(): number;
	get drawingBufferHeight(): number;
	static withView(view: number, version: number, alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number, premultipliedAlpha: boolean, preserveDrawingBuffer: boolean, stencil: boolean, desynchronized: boolean, xrCompatible: boolean): JsWebGlRenderingContext;
	static offscreen(width: number, height: number, version: number, alpha: boolean, antialias: boolean, depth: boolean, failIfMajorPerformanceCaveat: boolean, powerPreference: number, premultipliedAlpha: boolean, preserveDrawingBuffer: boolean, stencil: boolean, desynchronized: boolean, xrCompatible: boolean, isCanvas: boolean): JsWebGlRenderingContext;
	clearColor(red: number, green: number, blue: number, alpha: number): void;
	clear(mask: number): void;
	flush(): void;
	toDataURL(format: string, encoderOptions?: number | undefined | null): string;
	get COLOR_BUFFER_BIT(): number;
}
export type JSPath2D = Path2D;
export declare class Path2D {
	constructor(data?: Path2D | string | undefined | null);
	addPath(path: Path2D): void;
	arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise?: boolean | undefined | null): void;
	arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void;
	bezierCurveTo(cp1X: number, cp1Y: number, cp2X: number, cp2Y: number, x: number, y: number): void;
	closePath(): void;
	ellipse(x: number, y: number, radiusX: number, radiusY: number, rotation: number, startAngle: number, endAngle: number, anticlockwise?: boolean | undefined | null): void;
	lineTo(x: number, y: number): void;
	moveTo(x: number, y: number): void;
	quadraticCurveTo(cpx: number, cpy: number, x: number, y: number): void;
	rect(x: number, y: number, width: number, height: number): void;
	roundRect(x: number, y: number, width: number, height: number, radii: number | Array<number>): void;
	trim(start: number, end: number): void;
	toSvg(): string;
}
export type JSCanvasRenderingContext2D = CanvasRenderingContext2D;
export declare class CanvasRenderingContext2D {
	static withView(view: number, width: number, height: number, density: number, alpha: boolean, fontColor: number, ppi: number, direction: number): JsCanvasRenderingContext2D;
	static withCpu(width: number, height: number, density: number, alpha: boolean, fontColor: number, ppi: number, direction: number): JsCanvasRenderingContext2D;
	arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise?: boolean | undefined | null): void;
	beginPath(): void;
	render(): void;
	fill(path?: Path2D | undefined | null, rule?: string | undefined | null): void;
	fillText(text: string, x: number, y: number, maxWidth?: number | undefined | null): void;
	get shadowColor(): string;
	set shadowColor(color: string);
	get shadowOffsetX(): number;
	set shadowOffsetX(x: number);
	get shadowOffsetY(): number;
	set shadowOffsetY(y: number);
	get lineWidth(): number;
	set lineWidth(width: number);
	get globalAlpha(): number;
	set globalAlpha(alpha: number);
	get direction(): string;
	set direction(direction: string);
	get imageSmoothingEnabled(): boolean;
	set imageSmoothingEnabled(enabled: boolean);
	get fillStyle(): unknown;
	set fillStyle(style: string | CanvasPattern | CanvasGradient);
	get strokeStyle(): unknown;
	set strokeStyle(style: string | CanvasPattern | CanvasGradient);
	get font(): string;
	set font(value: string);
	fillRect(x: number, y: number, width: number, height: number): void;
	toDataURL(format: string, encoderOptions?: number | undefined | null): string;
	drawImage(image: JSImageAsset, sx?: number | undefined | null, sy?: number | undefined | null, sWidth?: number | undefined | null, sHeight?: number | undefined | null, dx?: number | undefined | null, dy?: number | undefined | null, dWidth?: number | undefined | null, dHeight?: number | undefined | null): void;
	rect(x: number, y: number, width: number, height: number): void;
	rotate(angle: number): void;
	roundRect(x: number, y: number, width: number, height: number, radii: number | Array<number>): void;
	setLineDash(segments: Array<number>): void;
	stroke(path?: Path2D | undefined | null): void;
	strokeRect(x: number, y: number, width: number, height: number): void;
	strokeText(text: string, x: number, y: number, maxWidth?: number | undefined | null): void;
	strokeOval(x: number, y: number, width: number, height: number): void;
	setTransform(a: number | JSDOMMatrix, b?: number | undefined | null, c?: number | undefined | null, d?: number | undefined | null, e?: number | undefined | null, f?: number | undefined | null): void;
	transform(a: number, b: number, c: number, d: number, e: number, f: number): void;
	translate(x: number, y: number): void;
}
export type JSCanvasPattern = CanvasPattern;
export declare class CanvasPattern {}
export type JSCCanvasGradient = CanvasGradient;
export declare class CanvasGradient {}
export type JSImageAsset = ImageAsset;
export declare class ImageAsset {
	constructor();
	get width(): number;
	get height(): number;
	get error(): string;
	fromUrlSync(url: string): boolean;
	fromUrl(url: string): Promise<boolean>;
	loadUrlCallback(url: string, callback: (...args: any[]) => any): void;
}
export type JSDOMMatrix = DOMMatrix;
export declare class DOMMatrix {
	constructor(data?: Array<number> | undefined | null);
}
