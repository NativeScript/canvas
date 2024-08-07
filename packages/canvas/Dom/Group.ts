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
	/*
		{matrix: number[]},
		 {perspective: number},
		  {rotate: string},
		  {rotateX: string},
		   {rotateY: string},
		    {rotateZ: string}, 
			{scale: number}, 
			{scaleX: number},
			 {scaleY: number},
			  {translateX: number}, 
			  {translateY: number},
			   {skewX: string},
			    {skewY: string}
		*/

	const matrix = value.matrix;

	if (Array.isArray(matrix)) {
		// todo
		//const add = new DOMMatrix(matrix);
		return;
	}

	const perspective = value.perspective;

	if (typeof perspective === 'number') {
		return;
	}

	const rotate = value.rotate;

	if (typeof rotate === 'string') {
		return;
	}

	const rotateX = value.rotateX;

	if (typeof rotateX === 'string') {
		return;
	}

	const rotateY = value.rotateY;

	if (typeof rotateY === 'string') {
		return;
	}

	const rotateZ = value.rotateZ;

	if (typeof rotateZ === 'string') {
		return;
	}

	const scale = value.scale;

	if (typeof scale === 'number') {
		return;
	}

	const scaleX = value.scaleX;

	if (typeof scaleX === 'number') {
		return;
	}

	const scaleY = value.scaleY;

	if (typeof scaleY === 'number') {
		return;
	}

	const translateX = value.translateX;

	if (typeof translateX === 'number') {
		return;
	}

	const translateY = value.translateY;

	if (typeof translateY === 'number') {
		return;
	}

	const skewX = value.skewX;

	if (typeof skewX === 'number') {
		_transform.c = skewX;
	}

	if (typeof skewX === 'string') {
		return;
	}

	const skewY = value.skewY;

	if (typeof skewY === 'number') {
		_transform.b = skewY;
	}

	if (typeof skewY === 'string') {
		return;
	}
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
				context.setTransform(this._matrix);
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
