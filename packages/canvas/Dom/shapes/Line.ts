import { Property } from '@nativescript/core';
import { Paint } from '../Paint';
import { Path2D } from '../../Canvas2D';

export const p1Property = new Property<Line, { x: number; y: number }>({
	name: 'p1',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

export const p2Property = new Property<Line, { x: number; y: number }>({
	name: 'p2',
	valueChanged(target, oldValue, newValue) {
		target.invalidate();
	},
});

function parsePoint(v: any): { x: number; y: number } | null {
	if (v === undefined || v === null) return null;
	if (typeof v === 'object' && 'x' in v && 'y' in v) {
		return { x: Number(v.x) || 0, y: Number(v.y) || 0 };
	}
	if (typeof v === 'string') {
		const parts = v
			.split(/[, ]+/)
			.map((s) => s.trim())
			.filter(Boolean);
		if (parts.length >= 2) {
			const x = parseFloat(parts[0]);
			const y = parseFloat(parts[1]);
			return { x: isNaN(x) ? 0 : x, y: isNaN(y) ? 0 : y };
		}
		if (parts.length === 1) {
			const x = parseFloat(parts[0]);
			return { x: isNaN(x) ? 0 : x, y: 0 };
		}
		return null;
	}
	if (typeof v === 'number') {
		return { x: v, y: 0 };
	}
	return null;
}

export class Line extends Paint {
	p1?: { x: number; y: number };
	p2?: { x: number; y: number };

	// Lines cannot be filled (a moveTo+lineTo path has zero fill area).
	// Default to 'stroke'; only respect an explicit 'fill' if the user set it.
	_getPaintStyle(): 'fill' | 'stroke' {
		if (this.paintStyle === 'fill') return 'fill';
		if (this._inGroup) {
			return (this.parent as any)?._getPaintStyle?.() ?? 'stroke';
		}
		return 'stroke';
	}

	draw() {
		const p1 = parsePoint(this.p1);
		const p2 = parsePoint(this.p2);
		if (!p1 || !p2) return;

		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
		const line = new Path2D();
		line.moveTo(p1.x, p1.y);
		line.lineTo(p2.x, p2.y);
		const color = this._getColor();
		const style = this._getPaintStyle();

		if (style === 'fill') {
			context.fillStyle = color;
			context.fill(line);
		} else {
			context.strokeStyle = color;
			context.lineWidth = this._getStrokeWidth();
			context.lineJoin = this._getStrokeJoin();
			context.stroke(line);
		}
	}
}

p1Property.register(Line);
p2Property.register(Line);
