import {CanvasRenderingContext} from "../../common";
import {CanvasGradient} from '../CanvasGradient';
import {CanvasPattern} from '../CanvasPattern';
import {Path2D} from '../Path2D';
import {ImageData} from '../ImageData';
import {TextMetrics} from '../TextMetrics';

export abstract class CanvasRenderingContext2DBase
	implements CanvasRenderingContext {
	lineWidth: number;
	fillStyle: string | CanvasGradient | CanvasPattern;
	strokeStyle: string | CanvasGradient | CanvasPattern;
	lineCap: string;
	globalCompositeOperation: string;
	font: string;
	filter: string;
	globalAlpha: number;
	imageSmoothingEnabled: boolean;
	imageSmoothingQuality: string;
	lineDashOffset: number;
	lineJoin: string;
	miterLimit: number;
	shadowBlur: number;
	shadowColor: string;
	shadowOffsetX: number;
	shadowOffsetY: number;
	textAlign: string;
	direction: string;

	_canvas: any;
	get canvas(): any {
		return this._canvas;
	}

	_type: string = 'none';
	get type() {
		return this._type;
	}

	public abstract addHitRegion(region: any): void;

	public abstract arc(
		x: number,
		y: number,
		radius: number,
		startAngle: number,
		endAngle: number,
		anticlockwise: boolean
	): void;

	public abstract arcTo(
		x1: number,
		y1: number,
		x2: number,
		y2: number,
		radius: number
	): void;

	public abstract beginPath(): void;

	public abstract bezierCurveTo(
		cp1x: number,
		cp1y: number,
		cp2x: number,
		cp2y: number,
		x: number,
		y: number
	): void;

	public abstract clearHitRegions(): void;

	public abstract clearRect(
		x: number,
		y: number,
		width: number,
		height: number
	): void;

	public abstract clip(): void;

	public abstract clip(fillRule: string): void;

	public abstract clip(path: Path2D, fillRule: string): void;

	public abstract closePath(): void;

	public abstract createImageData(width: number, height: number): ImageData;

	public abstract createImageData(data: ImageData): ImageData;

	public abstract createLinearGradient(
		x0: number,
		y0: number,
		x1: number,
		y1: number
	);

	public abstract createPattern(
		image: any,
		repetition: string
	): CanvasPattern | null;

	public abstract createRadialGradient(
		x0: number,
		y0: number,
		r0: number,
		x1: number,
		y1: number,
		r1: number
	);

	public abstract drawFocusIfNeeded(element): void;
	public abstract drawFocusIfNeeded(path, element): void;

	public abstract drawImage(image: any, dx: number, dy: number): void;
	public abstract drawImage(
		image: any,
		dx: number,
		dy: number,
		dWidth: number,
		dHeight: number
	): void;
	public abstract drawImage(
		image: any,
		sx: number,
		sy: number,
		sWidth: number,
		sHeight: number,
		dx: number,
		dy: number,
		dWidth: number,
		dHeight: number
	): void;

	public abstract ellipse(
		x: number,
		y: number,
		radiusX: number,
		radiusY: number,
		rotation: number,
		startAngle: number,
		endAngle: number,
		anticlockwise: boolean
	): void;

	public abstract fill(): void;
	public abstract fill(fillRule?: string): void;
	public abstract fill(path: Path2D, fillRule: string): void;

	public abstract fillRect(
		x: number,
		y: number,
		width: number,
		height: number
	): void;

	public abstract fillText(
		text: string,
		x: number,
		y: number,
		maxWidth?: number
	): void;

	public abstract getImageData(
		sx: number,
		sy: number,
		sw: number,
		sh: number
	): ImageData;

	public abstract getLineDash(): number[];

	public abstract isPointInPath(
		x: number,
		y: number,
		fillRule: string
	): boolean;
	public abstract isPointInPath(
		path: Path2D,
		x: number,
		y: number,
		fillRule: string
	): boolean;

	public abstract isPointInStroke(x: number, y: number): boolean;

	public abstract isPointInStroke(
		path: Path2D,
		x: number,
		y: number
	): boolean;

	public abstract lineTo(x: number, y: number): void;

	public abstract measureText(text: string): TextMetrics;

	public abstract moveTo(x: number, y: number): void;

	public abstract putImageData(
		imageData: ImageData,
		dx: number,
		dy: number
	): void;

	public abstract putImageData(
		imageData: ImageData,
		dx: number,
		dy: number,
		dirtyX: number,
		dirtyY: number,
		dirtyWidth: number,
		dirtyHeight: number
	): void;

	public abstract quadraticCurveTo(
		cpx: number,
		cpy: number,
		x: number,
		y: number
	);

	public abstract rect(
		x: number,
		y: number,
		width: number,
		height: number
	): void;

	public abstract removeHitRegion(id: string): void;

	public abstract resetTransform(): void;

	public abstract restore(): void;

	public abstract rotate(angle: number): void;

	public abstract save(): void;

	public abstract scale(x: number, y: number): void;

	public abstract scrollPathIntoView(): void;

	public abstract scrollPathIntoView(path: Path2D): void;

	public abstract setLineDash(segments: number[]): void;

	public abstract setTransform(
		a: number,
		b: number,
		c: number,
		d: number,
		e: number,
		f: number
	): void;

	public abstract stroke(): void;

	public abstract stroke(path: Path2D): void;

	public abstract strokeRect(
		x: number,
		y: number,
		width: number,
		height: number
	): void;

	public abstract strokeText(
		text: string,
		x: number,
		y: number,
		maxWidth?: number
	): void;

	public abstract transform(
		a: number,
		b: number,
		c: number,
		d: number,
		e: number,
		f: number
	): void;

	public abstract translate(x: number, y: number): void;
}
