import { Property } from '@nativescript/core';
import { Paint } from '../Paint';
import { Path2D, DOMMatrix } from '../../Canvas2D';
export const pathProperty = new Property<Path, string | Path2D>({
	name: 'path',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export const startProperty = new Property<Path, number>({
	name: 'start',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export const endProperty = new Property<Path, number>({
	name: 'end',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export class Path extends Paint {
	_path: Path2D = new Path2D();
	path: string | Path2D;
	start: number;
	end: number;

	[pathProperty.setNative](value) {
		if (value) {
			this._path = new Path2D(value);
		}
	}

	[startProperty.setNative](value: number) {
		if (typeof value === 'number' && typeof this.end === 'number') {
			this._path.trim(value, this.end);
		}
	}

	[endProperty.setNative](value: number) {
		if (typeof value === 'number' && typeof this.start === 'number') {
			this._path.trim(this.start, value);
		}
	}
	draw() {
		if (this._path === undefined) {
			return;
		}
		const override_color = (this.parent as any)._overrideColor;
		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
		const style = this._getPaintStyle();
		const color = this._getColor();
		if (style === 'fill') {
			context.fillStyle = color;
			context.fill(this._path as any);
		} else if (style === 'stroke') {
			context.lineWidth = this._getStrokeWidth();
			context.lineJoin = this._getStrokeJoin();
			context.strokeStyle = color;
			context.stroke(this._path as any);
		}
	}

	addPath(path: Path2D, transform?: DOMMatrix): void {
		this._path.addPath(path, transform);
	}

	arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise: boolean = false): void {
		this._path.arc(x, y, radius, startAngle, endAngle, anticlockwise ?? false);
	}

	arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void {
		this._path.arcTo(x1, y1, x2, y2, radius);
	}

	bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void {
		this._path.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y);
	}

	closePath(): void {
		this._path.closePath();
	}

	ellipse(x: number, y: number, radiusX: number, radiusY: number, rotation: number, startAngle: number, endAngle: number, anticlockwise: boolean = false): void {
		this._path.ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise ?? false);
	}

	lineTo(x: number, y: number): void {
		this._path.lineTo(x, y);
	}

	moveTo(x: number, y: number): void {
		this._path.moveTo(x, y);
	}

	quadraticCurveTo(cpx: number, cpy: number, x: number, y: number): void {
		this._path.quadraticCurveTo(cpx, cpy, x, y);
	}

	rect(x: number, y: number, width: number, height: number): void {
		this._path.rect(x, y, width, height);
	}

	public roundRect(x: number, y: number, width: number, height: number, radii: number): void;
	public roundRect(x: number, y: number, width: number, height: number, radii: number[]): void;
	public roundRect(x: number, y: number, width: number, height: number, radii: number | number[]): void {
		this._path.roundRect(x, y, width, height, radii as any);
	}
}

pathProperty.register(Path);
