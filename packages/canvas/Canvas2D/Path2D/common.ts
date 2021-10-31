import {DOMMatrixBase} from "../DOMMatrix";

export abstract class Path2DBase {
	protected nativeInstance: any;

	protected constructor(instance: any) {
		this.nativeInstance = instance;
	}

	get native() {
		return this.nativeInstance;
	}

	static [Symbol.hasInstance](obj) {
		if (obj.native && obj.constructor.name === 'Path2D') return true;
	}

	public abstract addPath(
		path: Path2DBase,
		transform?: DOMMatrixBase
	): void;

	public abstract closePath(): void;

	public abstract moveTo(x: number, y: number): void;

	public abstract lineTo(x: number, y: number): void;

	public abstract bezierCurveTo(
		cp1x: number,
		cp1y: number,
		cp2x: number,
		cp2y: number,
		x: number,
		y: number
	): void;

	public abstract quadraticCurveTo(
		cpx: number,
		cpy: number,
		x: number,
		y: number
	): void;

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

	public abstract rect(
		x: number,
		y: number,
		width: number,
		height: number
	): void;
}
