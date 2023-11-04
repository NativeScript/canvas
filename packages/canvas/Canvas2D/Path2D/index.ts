import { DOMMatrix } from '../DOMMatrix';

import { Helpers } from '../../helpers';

let ctor;
export class Path2D {
	static {
		Helpers.initialize();
		ctor = global.CanvasModule.Path2D;
	}

	_native;
	constructor(instance?: any) {
		let nativeInstance;
		if (!instance) {
			nativeInstance = new ctor();
			this._native = nativeInstance;
			return this;
		}
		if (typeof instance === 'string') {
			nativeInstance = new ctor(instance);
		} else if (instance instanceof Path2D) {
			nativeInstance = new ctor(instance.native);
		} else {
			nativeInstance = new ctor();
		}
		this._native = nativeInstance;
	}

	get native() {
		return this._native;
	}


	addPath(path: Path2D, transform?: DOMMatrix): void {
		//const addPath = this._getMethod('addPath');
		if (transform) {
			this.native.addPath(path.native, transform.native);
		} else {
			this.native.addPath(path.native, null);
		}
	}

	arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise: boolean = false): void {
		//const arc = this._getMethod('arc');
		this.native.arc(x, y, radius, startAngle, endAngle, anticlockwise);
	}

	arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void {
		//const arcTo = this._getMethod('arcTo');
		this.native.arcTo(x1, y1, x2, y2, radius);
	}

	bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void {
		//const bezierCurveTo = this._getMethod('bezierCurveTo');
		this.native.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y);
	}

	closePath(): void {
		//const closePath = this._getMethod('closePath');
		this.native.closePath();
	}

	ellipse(x: number, y: number, radiusX: number, radiusY: number, rotation: number, startAngle: number, endAngle: number, anticlockwise: boolean = false): void {
		//const ellipse = this._getMethod('ellipse');
		this.native.ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise);
	}

	lineTo(x: number, y: number): void {
		//const lineTo = this._getMethod('lineTo');
		this.native.lineTo(x, y);
	}

	moveTo(x: number, y: number): void {
		//const moveTo = this._getMethod('moveTo');
		this.native.moveTo(x, y);
	}

	quadraticCurveTo(cpx: number, cpy: number, x: number, y: number): void {
		//const quadraticCurveTo = this._getMethod('quadraticCurveTo');
		this.native.quadraticCurveTo(cpx, cpy, x, y);
	}

	rect(x: number, y: number, width: number, height: number): void {
		//const rect = this._getMethod('rect');
		this.native.rect(x, y, width, height);
	}

	public roundRect(x: number, y: number, width: number, height: number, radii: number): void;
	public roundRect(x: number, y: number, width: number, height: number, radii: number[]): void;
	public roundRect(x: unknown, y: unknown, width: unknown, height: unknown, radii: unknown): void {
		//const roundRect = this._getMethod('roundRect');
		this.native.roundRect(x, y, width, height, radii);
	}

	__toSVG() {
		//const __toSVG = this._getMethod('__toSVG');
		return this.native.__toSVG();
	}
}
