import { DOMMatrix } from '../DOMMatrix';

import { Helpers } from '../../helpers';

export class Path2D {
	static {
		Helpers.initialize();
	}

	private _native;
	constructor(instance?: any) {
		let nativeInstance;
		if (!instance) {
			nativeInstance = new global.CanvasModule.Path2D();
			this._native = nativeInstance;
			return this;
		}
		if (typeof instance === 'string') {
			nativeInstance = new global.CanvasModule.Path2D(instance);
		} else if (instance instanceof Path2D) {
			nativeInstance = new global.CanvasModule.Path2D(instance.native);
		}
		this._native = nativeInstance;
	}

	get native() {
		return this._native;
	}

	addPath(path: Path2D, transform?: DOMMatrix): void {
		if (transform) {
			this.native.addPath(path.native, transform.native);
		} else {
			this.native.addPath(path.native, null);
		}
	}

	arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise: boolean = false): void {
		this.native.arc(x, y, radius, startAngle, endAngle, anticlockwise ?? false);
	}

	arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void {
		this.native.arcTo(x1, y1, x2, y2, radius);
	}

	bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void {
		this.native.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y);
	}

	closePath(): void {
		this.native.closePath();
	}

	ellipse(x: number, y: number, radiusX: number, radiusY: number, rotation: number, startAngle: number, endAngle: number, anticlockwise: boolean = false): void {
		this.native.ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise ?? false);
	}

	lineTo(x: number, y: number): void {
		this.native.lineTo(x, y);
	}

	moveTo(x: number, y: number): void {
		this.native.moveTo(x, y);
	}

	quadraticCurveTo(cpx: number, cpy: number, x: number, y: number): void {
		this.native.quadraticCurveTo(cpx, cpy, x, y);
	}

	rect(x: number, y: number, width: number, height: number): void {
		this.native.rect(x, y, width, height);
	}

	public roundRect(x: number, y: number, width: number, height: number, radii: number): void;
	public roundRect(x: number, y: number, width: number, height: number, radii: number[]): void;
	public roundRect(x: unknown, y: unknown, width: unknown, height: unknown, radii: unknown): void {
		this.native.roundRect(x, y, width, height, radii);
	}

	trim(start: number, end: number): void {
		this.native.trim(start, end);
	}

	__toSVG() {
		return this.native.__toSVG();
	}
}
