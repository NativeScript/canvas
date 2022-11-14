import {Path2DBase} from './common';
import {DOMMatrix} from '../DOMMatrix';

declare let TNSPath2D;

export class Path2D extends Path2DBase {
	constructor(instance?: any) {
		let nativeInstance;
		if (typeof instance === 'string') {
			nativeInstance = TNSPath2D.alloc().initWithData(instance);
		} else if (instance instanceof Path2D) {
			nativeInstance = TNSPath2D.alloc().initWithPath(instance.native);
		} else if (instance instanceof TNSPath2D) {
			nativeInstance = TNSPath2D.alloc().initWithPath(instance);
		} else {
			nativeInstance = TNSPath2D.new();
		}
		super(nativeInstance);
	}

	addPath(path: Path2D, transform?: DOMMatrix): void {
		if (transform) {
			this.nativeInstance.addPathWithPathTransform(
				path.nativeInstance,
				transform.native
			);
		} else {
			this.nativeInstance.addPath(path.nativeInstance);
		}
	}

	arc(
		x: number,
		y: number,
		radius: number,
		startAngle: number,
		endAngle: number,
		anticlockwise: boolean = false
	): void {
		this.nativeInstance.arc(
			x,
			y,
			radius,
			startAngle,
			endAngle,
			anticlockwise
		);
	}

	arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void {
		this.nativeInstance.arcTo(x1, y1, x2, y2, radius);
	}

	bezierCurveTo(
		cp1x: number,
		cp1y: number,
		cp2x: number,
		cp2y: number,
		x: number,
		y: number
	): void {
		this.nativeInstance.arcTo(
			cp1x,
			cp1y,
			cp2x,
			cp2y,
			x,
			y
		);
	}

	closePath(): void {
		this.nativeInstance.closePath();
	}

	ellipse(
		x: number,
		y: number,
		radiusX: number,
		radiusY: number,
		rotation: number,
		startAngle: number,
		endAngle: number,
		anticlockwise: boolean = false
	): void {
		this.nativeInstance.ellipse(
			x,
			y,
			radiusX,
			radiusY,
			rotation,
			startAngle,
			endAngle,
			anticlockwise
		);
	}

	lineTo(x: number, y: number): void {
		this.nativeInstance.lineTo(x, y);
	}

	moveTo(x: number, y: number): void {
		this.nativeInstance.moveTo(x, y);
	}

	quadraticCurveTo(cpx: number, cpy: number, x: number, y: number): void {
		this.nativeInstance.quadraticCurveTo(cpx, cpy, x, y);
	}

	rect(x: number, y: number, width: number, height: number): void {
		this.nativeInstance.rect(x, y, width, height);
	}

	roundRect(x: number, y: number, width: number, height: number, radii: number): void;
	roundRect(x: number, y: number, width: number, height: number, radii: number[]): void;
	roundRect(x: number, y: number, width: number, height: number, radii: any): void {
		if (Array.isArray(radii)) {
			this.nativeInstance.roundRectWithXYWidthHeightRadii(x, y, width, height, radii);
		} else if (typeof radii === 'number') {
			this.nativeInstance.roundRectWithRadii(x, y, width, height, radii);
		}
	}
}
