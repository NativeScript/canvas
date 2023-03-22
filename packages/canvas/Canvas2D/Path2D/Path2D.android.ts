import { Path2DBase } from './common';
import { DOMMatrix } from '../DOMMatrix';

import { Helpers } from '../../helpers';

let ctor;
export class Path2D extends Path2DBase {
	static {
		Helpers.initialize();
		ctor = global.CanvasJSIModule.Path2D;
	}

	constructor(instance?: any) {
		super(null);
		let nativeInstance;
		if (typeof instance === 'string') {
			nativeInstance = ctor(instance);
		} else if (instance instanceof Path2D) {
			nativeInstance = ctor(instance.native);
		} else {
			nativeInstance = ctor();
		}
		this.nativeInstance = nativeInstance;
	}

	_methodCache = new Map();

	_getMethod(name: string) {
		const cached = this._methodCache.get(name);
		if (cached === undefined) {
			const ret = this.nativeInstance[name];
			this._methodCache.set(name, ret);
			return ret;
		}
		return cached;
	}

	addPath(path: Path2D, transform?: DOMMatrix): void {
		const addPath = this._getMethod('addPath');
		if (transform) {
			addPath(path.nativeInstance, transform.native);
		} else {
			addPath(path.nativeInstance, null);
		}
	}

	arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise: boolean = false): void {
		const arc = this._getMethod('arc');
		arc(x, y, radius, startAngle, endAngle, anticlockwise);
	}

	arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void {
		const arcTo = this._getMethod('arcTo');
		arcTo(x1, y1, x2, y2, radius);
	}

	bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void {
		const bezierCurveTo = this._getMethod('bezierCurveTo');
		bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y);
	}

	closePath(): void {
		const closePath = this._getMethod('closePath');
		closePath();
	}

	ellipse(x: number, y: number, radiusX: number, radiusY: number, rotation: number, startAngle: number, endAngle: number, anticlockwise: boolean = false): void {
		const ellipse = this._getMethod('ellipse');
		ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise);
	}

	lineTo(x: number, y: number): void {
		const lineTo = this._getMethod('lineTo');
		lineTo(x, y);
	}

	moveTo(x: number, y: number): void {
		const moveTo = this._getMethod('moveTo');
		moveTo(x, y);
	}

	quadraticCurveTo(cpx: number, cpy: number, x: number, y: number): void {
		const quadraticCurveTo = this._getMethod('quadraticCurveTo');
		quadraticCurveTo(cpx, cpy, x, y);
	}

	rect(x: number, y: number, width: number, height: number): void {
		const rect = this._getMethod('rect');
		rect(x, y, width, height);
	}

	public roundRect(x: number, y: number, width: number, height: number, radii: number): void;
	public roundRect(x: number, y: number, width: number, height: number, radii: number[]): void;
	public roundRect(x: unknown, y: unknown, width: unknown, height: unknown, radii: unknown): void {
		const roundRect = this._getMethod('roundRect');
		roundRect(x, y, width, height, radii);
	}

	__toSVG() {
		const __toSVG = this._getMethod('__toSVG');
		return __toSVG();
	}
}
