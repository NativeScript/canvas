import { LayoutBase, Property, ShorthandProperty, Style, View, ViewBase } from '@nativescript/core';
import { originXProperty, originYProperty } from '@nativescript/core/ui/core/view';
import { Image } from './Image';
import { Paint } from './Paint';
import { DOMMatrix } from '../Canvas2D/DOMMatrix';
import { Canvas } from '../Canvas';

export const matrixProperty = new Property<Group, DOMMatrix>({
	name: 'matrix',
});

export const originProperty = new ShorthandProperty<Style, { x: number; y: number }>({
	name: 'origin',
	cssName: 'origin',
	getter: function () {
		const view = this as any;
		return { x: view.originX, y: view.originY };
	},
	converter(value: any): any {
		if (value === null || value === undefined) {
			return [];
		}
		const type = typeof value;
		if (type === 'string') {
			try {
				const val = JSON.parse(JSON.stringify(value as any));

				return [
					[originXProperty, val.x ?? 0],
					[originYProperty, val.y ?? 0],
				];
			} catch (error) {}
		}

		if (type === 'object') {
			return [
				[originXProperty, value.x],
				[originYProperty, value.y],
			];
		}

		return [];
	},
});

export const transformProperty = new Property<Group, any[]>({
	name: 'transform',
});

/*
ScaleX = 0,
    SkewX = 4,
    TransX = 12,
    SkewY = 1,
    ScaleY = 5,
    TransY = 13,
    Persp0 = 3,
    Persp1 = 7,
    Persp2 = 15,
*/

/*
M11 = 0,
    M12 = 1,
    M13 = 2,
    M14 = 3,
    M21 = 4,
    M22 = 5,
    M23 = 6,
    M24 = 7,
    M31 = 8,
    M32 = 9,
    M33 = 10,
    M34 = 11,
    M41 = 12,
    M42 = 13,
    M43 = 14,
    M44 = 15,
*/

function parseTransformation(value, _transform: DOMMatrix) {
	function parseNumber(v: any): number | null {
		if (v === null || v === undefined) return null;
		if (typeof v === 'number') return v;
		if (typeof v === 'string') {
			const n = parseFloat(v);
			return isNaN(n) ? null : n;
		}
		return null;
	}

	function parseAngle(v: any): number | null {
		if (v === null || v === undefined) return null;
		if (typeof v === 'number') return v; // assume radians
		if (typeof v === 'string') {
			const m = v.match(/^\s*([-+]?\d*\.?\d+)(deg|rad)?\s*$/i);
			if (!m) return null;
			const val = parseFloat(m[1]);
			const unit = (m[2] || 'deg').toLowerCase();
			if (unit === 'rad') return val;
			return (val * Math.PI) / 180; // degrees -> radians
		}
		return null;
	}

	// read current matrix
	let a1 = _transform.a ?? 1;
	let b1 = _transform.b ?? 0;
	let c1 = _transform.c ?? 0;
	let d1 = _transform.d ?? 1;
	let e1 = _transform.e ?? 0;
	let f1 = _transform.f ?? 0;

	function setMatrix(m: { a: number; b: number; c: number; d: number; e: number; f: number }) {
		_transform.a = m.a;
		_transform.b = m.b;
		_transform.c = m.c;
		_transform.d = m.d;
		_transform.e = m.e;
		_transform.f = m.f;
		a1 = m.a;
		b1 = m.b;
		c1 = m.c;
		d1 = m.d;
		e1 = m.e;
		f1 = m.f;
	}

	function multiply(M: any, N: any) {
		// M and N are {a,b,c,d,e,f}
		return {
			a: M.a * N.a + M.c * N.b,
			b: M.b * N.a + M.d * N.b,
			c: M.a * N.c + M.c * N.d,
			d: M.b * N.c + M.d * N.d,
			e: M.a * N.e + M.c * N.f + M.e,
			f: M.b * N.e + M.d * N.f + M.f,
		};
	}

	// matrix array (6 elements [a,b,c,d,e,f] or 16 elements, treat 6 primarily)
	if (Array.isArray(value.matrix)) {
		const m = value.matrix;
		if (m.length === 6) {
			const T = { a: m[0], b: m[1], c: m[2], d: m[3], e: m[4], f: m[5] };
			const res = multiply({ a: a1, b: b1, c: c1, d: d1, e: e1, f: f1 }, T);
			setMatrix(res);
			return;
		} else if (m.length === 16) {
			// assume DOMMatrix 4x4 column-major: map to 2D
			const T = { a: m[0], b: m[1], c: m[4], d: m[5], e: m[12], f: m[13] };
			const res = multiply({ a: a1, b: b1, c: c1, d: d1, e: e1, f: f1 }, T as any);
			setMatrix(res);
			return;
		}
	}

	// translate
	const tx = parseNumber(value.translateX ?? value.tx ?? (value.translate && value.translate.x));
	const ty = parseNumber(value.translateY ?? value.ty ?? (value.translate && value.translate.y));
	if (tx !== null || ty !== null) {
		const T = { a: 1, b: 0, c: 0, d: 1, e: tx ?? 0, f: ty ?? 0 };
		const res = multiply({ a: a1, b: b1, c: c1, d: d1, e: e1, f: f1 }, T);
		setMatrix(res);
	}

	// scale
	const scale = parseNumber(value.scale ?? null);
	const sx = parseNumber(value.scaleX ?? value.sx) ?? scale ?? null;
	const sy = parseNumber(value.scaleY ?? value.sy) ?? scale ?? null;
	if (sx !== null || sy !== null) {
		const T = { a: sx ?? 1, b: 0, c: 0, d: sy ?? 1, e: 0, f: 0 };
		const res = multiply({ a: a1, b: b1, c: c1, d: d1, e: e1, f: f1 }, T);
		setMatrix(res);
	}

	// rotate (support rotate / rotateZ)
	const rot = value.rotate ?? value.rotateZ ?? null;
	const angle = parseAngle(rot);
	if (angle !== null) {
		const cos = Math.cos(angle);
		const sin = Math.sin(angle);
		const T = { a: cos, b: sin, c: -sin, d: cos, e: 0, f: 0 };
		const res = multiply({ a: a1, b: b1, c: c1, d: d1, e: e1, f: f1 }, T);
		setMatrix(res);
	}

	// skew
	if (value.skewX !== undefined) {
		const ang = parseAngle(value.skewX);
		if (ang !== null) {
			const t = Math.tan(ang);
			const T = { a: 1, b: 0, c: t, d: 1, e: 0, f: 0 };
			const res = multiply({ a: a1, b: b1, c: c1, d: d1, e: e1, f: f1 }, T);
			setMatrix(res);
		}
	}
	if (value.skewY !== undefined) {
		const ang = parseAngle(value.skewY);
		if (ang !== null) {
			const t = Math.tan(ang);
			const T = { a: 1, b: t, c: 0, d: 1, e: 0, f: 0 };
			const res = multiply({ a: a1, b: b1, c: c1, d: d1, e: e1, f: f1 }, T);
			setMatrix(res);
		}
	}

	// ignore perspective and 3D transforms (rotateX/rotateY) for now
}

export class Group extends Paint {
	_children: Set<Paint> = new Set();

	_matrix: DOMMatrix;
	originX: number = 0;
	originY: number = 0;
	//origin: { x: number; y: number };
	transform: any[];
	constructor() {
		super();
	}
	[transformProperty.setNative](value) {
		if (Array.isArray(value)) {
			const matrix = new DOMMatrix();
			value.forEach((item) => {
				parseTransformation(item, matrix);
			});
			this._matrix = matrix;
			this.invalidate();
		}
	}

	set origin(value) {
		// @ts-ignore
		this.style.origin = value;
	}

	get origin() {
		// @ts-ignore
		return this.style.origin;
	}

	_addCanvas(canvas: Canvas) {
		this._canvas = canvas;
		for (const child of this._children) {
			child._addCanvas(canvas);
		}
	}

	addChild(view: View): void {
		if (view instanceof Paint || view instanceof Image) {
			view._addCanvas(this._canvas);
			(view as any)._inGroup = true;
			this._children.add(view as any);
		}
	}

	draw() {
		if (this._children.size > 0) {
			const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;
			context.save();
			const origin = this.origin;
			context.translate(origin.x ?? 0, origin.y ?? 0);
			if (this._matrix) {
				// Multiply current transform by the group's matrix to preserve translation
				context.transform(this._matrix.a, this._matrix.b, this._matrix.c, this._matrix.d, this._matrix.e, this._matrix.f);
			}
			context.lineWidth = this.strokeWidth;
			this._children.forEach((child) => {
				child.draw();
			});

			context.restore();
		}
	}

	_addViewToNativeVisualTree(view: ViewBase, _atIndex?: number): boolean {
		if (view instanceof Paint || view instanceof Image) {
			view._addCanvas(this._canvas);
			(view as any)._inGroup = true;
			this._children.add(view as any);
		}
		return false;
	}
	_removeViewFromNativeVisualTree(view: ViewBase): void {
		(view as any)._inGroup = false;
		this._children.delete(view as any);
		super._removeViewFromNativeVisualTree(view);
	}
}

matrixProperty.register(Group);
originProperty.register(Style);
transformProperty.register(Group);
