import { View, Screen } from '@nativescript/core';
import { ImageAsset } from '../../ImageAsset';
import { DOMMatrix } from '../../Canvas2D/DOMMatrix';

/** DOMMatrix order (m11, m12, ... m44), column vectors. */
export type Mat4 = Float64Array;

export const mat4 = {
	identity(): Mat4 {
		const m = new Float64Array(16);
		m[0] = m[5] = m[10] = m[15] = 1;
		return m;
	},

	translate(x: number, y: number): Mat4 {
		const m = mat4.identity();
		m[12] = x;
		m[13] = y;
		return m;
	},

	scale(x: number, y: number): Mat4 {
		const m = mat4.identity();
		m[0] = x;
		m[5] = y;
		return m;
	},

	/** a * b: b applies first. */
	multiply(a: Mat4, b: Mat4): Mat4 {
		const out = new Float64Array(16);
		for (let col = 0; col < 4; col++) {
			for (let row = 0; row < 4; row++) {
				let sum = 0;
				for (let k = 0; k < 4; k++) {
					sum += a[k * 4 + row] * b[col * 4 + k];
				}
				out[col * 4 + row] = sum;
			}
		}
		return out;
	},

	chain(...ms: Mat4[]): Mat4 {
		let out = ms[0];
		for (let i = 1; i < ms.length; i++) {
			out = mat4.multiply(out, ms[i]);
		}
		return out;
	},

	invert(a: Mat4): Mat4 | null {
		const [a00, a01, a02, a03, a10, a11, a12, a13, a20, a21, a22, a23, a30, a31, a32, a33] = a as any;
		const b00 = a00 * a11 - a01 * a10;
		const b01 = a00 * a12 - a02 * a10;
		const b02 = a00 * a13 - a03 * a10;
		const b03 = a01 * a12 - a02 * a11;
		const b04 = a01 * a13 - a03 * a11;
		const b05 = a02 * a13 - a03 * a12;
		const b06 = a20 * a31 - a21 * a30;
		const b07 = a20 * a32 - a22 * a30;
		const b08 = a20 * a33 - a23 * a30;
		const b09 = a21 * a32 - a22 * a31;
		const b10 = a21 * a33 - a23 * a31;
		const b11 = a22 * a33 - a23 * a32;
		let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
		if (!det || !Number.isFinite(det)) {
			return null;
		}
		det = 1 / det;
		const out = new Float64Array(16);
		out[0] = (a11 * b11 - a12 * b10 + a13 * b09) * det;
		out[1] = (a02 * b10 - a01 * b11 - a03 * b09) * det;
		out[2] = (a31 * b05 - a32 * b04 + a33 * b03) * det;
		out[3] = (a22 * b04 - a21 * b05 - a23 * b03) * det;
		out[4] = (a12 * b08 - a10 * b11 - a13 * b07) * det;
		out[5] = (a00 * b11 - a02 * b08 + a03 * b07) * det;
		out[6] = (a32 * b02 - a30 * b05 - a33 * b01) * det;
		out[7] = (a20 * b05 - a22 * b02 + a23 * b01) * det;
		out[8] = (a10 * b10 - a11 * b08 + a13 * b06) * det;
		out[9] = (a01 * b08 - a00 * b10 - a03 * b06) * det;
		out[10] = (a30 * b04 - a31 * b02 + a33 * b00) * det;
		out[11] = (a21 * b02 - a20 * b04 - a23 * b00) * det;
		out[12] = (a11 * b07 - a10 * b09 - a12 * b06) * det;
		out[13] = (a00 * b09 - a01 * b07 + a02 * b06) * det;
		out[14] = (a31 * b01 - a30 * b03 - a32 * b00) * det;
		out[15] = (a20 * b03 - a21 * b01 + a22 * b00) * det;
		return out;
	},

	apply(m: Mat4, x: number, y: number): { x: number; y: number } {
		const w = m[3] * x + m[7] * y + m[15];
		const px = m[0] * x + m[4] * y + m[12];
		const py = m[1] * x + m[5] * y + m[13];
		return w && w !== 1 ? { x: px / w, y: py / w } : { x: px, y: py };
	},

	equals(a: Mat4 | null, b: Mat4 | null, epsilon = 1e-6): boolean {
		if (a === b) {
			return true;
		}
		if (!a || !b) {
			return false;
		}
		for (let i = 0; i < 16; i++) {
			if (Math.abs(a[i] - b[i]) > epsilon) {
				return false;
			}
		}
		return true;
	},

	is2D(m: Mat4): boolean {
		return m[2] === 0 && m[3] === 0 && m[6] === 0 && m[7] === 0 && m[8] === 0 && m[9] === 0 && m[10] === 1 && m[11] === 0 && m[14] === 0 && m[15] === 1;
	},

	from(init: any): Mat4 {
		const m = mat4.identity();
		if (!init) {
			return m;
		}
		if (init instanceof Float64Array && init.length === 16) {
			return new Float64Array(init);
		}
		const has3D = init.m11 !== undefined || init.m13 !== undefined || init.m33 !== undefined;
		if (has3D) {
			const keys = ['m11', 'm12', 'm13', 'm14', 'm21', 'm22', 'm23', 'm24', 'm31', 'm32', 'm33', 'm34', 'm41', 'm42', 'm43', 'm44'];
			for (let i = 0; i < 16; i++) {
				const value = init[keys[i]];
				if (typeof value === 'number') {
					m[i] = value;
				}
			}
			return m;
		}
		m[0] = init.a ?? 1;
		m[1] = init.b ?? 0;
		m[4] = init.c ?? 0;
		m[5] = init.d ?? 1;
		m[12] = init.e ?? 0;
		m[13] = init.f ?? 0;
		return m;
	},

	toDOMMatrix(m: Mat4 | null): DOMMatrix {
		if (!m) {
			return new DOMMatrix();
		}
		if (mat4.is2D(m)) {
			return new DOMMatrix([m[0], m[1], m[4], m[5], m[12], m[13]]);
		}
		return new DOMMatrix(Array.from(m));
	},
};

export interface Rect {
	x: number;
	y: number;
	width: number;
	height: number;
}

export function rectFrom(init: any): Rect | null {
	if (!init) {
		return null;
	}
	return {
		x: Number(init.x ?? 0),
		y: Number(init.y ?? 0),
		width: Number(init.width ?? 0),
		height: Number(init.height ?? 0),
	};
}

export function sameRect(a: Rect | null, b: Rect | null): boolean {
	return a === b || (!!a && !!b && a.x === b.x && a.y === b.y && a.width === b.width && a.height === b.height);
}

export function rectContains(rect: Rect, x: number, y: number) {
	const left = Math.min(rect.x, rect.x + rect.width);
	const top = Math.min(rect.y, rect.y + rect.height);
	return x >= left && y >= top && x < left + Math.abs(rect.width) && y < top + Math.abs(rect.height);
}

export function isDrawable(view: any): boolean {
	if (!view) {
		return false;
	}
	const value = view.drawable;
	return value !== undefined && value !== null && value !== false && value !== 'false';
}

export function domError(name: string, message: string) {
	const error: any = new Error(message);
	error.name = name;
	return error;
}

const internal = Symbol('[[ElementImage]]');

export class ElementImage {
	/** @internal */ _asset: ImageAsset | null = null;
	/** @internal */ _width = 0;
	/** @internal */ _height = 0;
	/** @internal Grid units. */
	_defaultWidth = 0;
	/** @internal */ _defaultHeight = 0;
	/** @internal DIP. */
	_cssWidth = 0;
	/** @internal */ _cssHeight = 0;
	/** @internal */ _element: WeakRef<View> | null = null;
	/** @internal */ _canvas: WeakRef<any> | null = null;

	constructor(key?: symbol) {
		if (key !== internal) {
			throw new TypeError('Illegal constructor');
		}
	}

	/** @internal */
	static _create(): ElementImage {
		return new ElementImage(internal);
	}

	get [Symbol.toStringTag]() {
		return 'ElementImage';
	}

	/** @internal */
	get _closed() {
		return this._asset === null;
	}

	close() {
		this._asset = null;
	}
}

export interface ElementSource {
	/** Built on first use where the snapshot lives on the GPU. */
	readonly asset: ImageAsset;
	/** A retained native snapshot (close it), or null. */
	nativeSnapshot?: () => any;
	width: number;
	height: number;
	/** Grid units. */
	defaultWidth: number;
	defaultHeight: number;
	/** DIP. */
	cssWidth: number;
	cssHeight: number;
	view: View | null;
	canvas: any;
}

export interface ElementGeometry {
	/** Border box (DIP) to canvas grid. */
	transform: Mat4 | null;
	/** Border box coordinates. */
	clip: Rect | null;
}

export interface SnapshotRecord {
	asset: ImageAsset | null;
	width: number;
	height: number;
	cssWidth: number;
	cssHeight: number;
	defaultWidth: number;
	defaultHeight: number;
	[key: string]: any;
}

export abstract class DrawableController {
	protected readonly canvasRef: WeakRef<any>;
	protected snapshots = new Map<View, SnapshotRecord>();
	protected geometry = new Map<View, ElementGeometry>();
	/** Bottom to top. */
	protected hitOrder: View[] = [];
	protected paintRequested = false;
	private generation = 0;
	/** What the native host last got for each view. */
	private sent = new Map<View, { native: any; m: Mat4; clip: Rect | null }>();
	/** The view last raised above the others for hit testing. */
	private top: View | null = null;
	protected disposed = false;

	constructor(canvas: any) {
		this.canvasRef = new WeakRef(canvas);
	}

	get canvas(): any {
		return this.canvasRef.deref();
	}

	abstract attachChild(child: View, index?: number): boolean;

	abstract detachChild(child: View): void;

	/** `source` is in DIP. */
	abstract render(view: View, width: number, height: number, source?: Rect): ImageAsset | null;

	protected abstract applyGeometry(view: View, geometry: ElementGeometry | null, moveToTop?: boolean): void;

	protected releaseRecord(rec: SnapshotRecord) {
		rec.asset = null;
	}

	dispose() {
		this.disposed = true;
		DrawableController.deactivate(this);
		for (const rec of this.snapshots.values()) {
			this.releaseRecord(rec);
		}
		this.snapshots.clear();
		this.geometry.clear();
		this.sent.clear();
		this.top = null;
		this.hitOrder = [];
	}

	protected canvasCssSize(canvas: any): { width: number; height: number } {
		const density = Screen.mainScreen.scale || 1;
		return { width: canvas.getMeasuredWidth() / density, height: canvas.getMeasuredHeight() / density };
	}

	/** Pixels per DIP of the canvas grid. */
	gridScale(): { x: number; y: number } {
		const canvas = this.canvas;
		const fallback = Screen.mainScreen.scale || 1;
		if (!canvas) {
			return { x: fallback, y: fallback };
		}
		const { width: cssWidth, height: cssHeight } = this.canvasCssSize(canvas);
		const width = canvas.width;
		const height = canvas.height;
		return {
			x: cssWidth > 0 && width > 0 ? width / cssWidth : fallback,
			y: cssHeight > 0 && height > 0 ? height / cssHeight : fallback,
		};
	}

	drawables(): View[] {
		const out: View[] = [];
		const visit = (view: View) => {
			if (isDrawable(view)) {
				out.push(view);
			}
			view.eachChildView((child) => {
				visit(child);
				return true;
			});
		};
		this.canvas?.eachChildView((child: View) => {
			visit(child);
			return true;
		});
		return out;
	}

	contains(view: View): boolean {
		const canvas = this.canvas;
		let node: any = view?.parent;
		while (node) {
			if (node === canvas) {
				return true;
			}
			node = node.parent;
		}
		return false;
	}

	cssSize(view: View): { width: number; height: number } {
		const size = view.getActualSize?.();
		return { width: size?.width ?? 0, height: size?.height ?? 0 };
	}

	defaultSize(view: View): { width: number; height: number } {
		const { width, height } = this.cssSize(view);
		const scale = this.gridScale();
		return { width: width * scale.x, height: height * scale.y };
	}

	snapshot(view: View): SnapshotRecord | null {
		const rec = this.snapshots.get(view);
		return rec?.asset || rec?.native ? rec : null;
	}

	/** The snapshot as an ImageAsset, for consumers with no GPU path. */
	assetOf(view: View, rec: SnapshotRecord): ImageAsset | null {
		return rec.asset;
	}

	/** A retained native snapshot the GPU paths can draw (close it when done), or null. */
	nativeSnapshot(view: View): any {
		return null;
	}

	markHot() {}

	requestPaint() {
		this.paintRequested = true;
		DrawableController.activate(this);
	}

	getGeometry(view: View): ElementGeometry | null {
		return this.geometry.get(view) ?? null;
	}

	updateGeometry(view: View, options: any = {}) {
		let geometry = this.geometry.get(view);
		const known = !!geometry;
		if (!geometry) {
			geometry = { transform: null, clip: null };
			this.geometry.set(view, geometry);
		}
		if (options && 'canvasTransform' in options) {
			geometry.transform = options.canvasTransform == null ? null : mat4.from(options.canvasTransform);
		}
		if (options && 'canvasClip' in options) {
			geometry.clip = rectFrom(options.canvasClip);
		}
		const index = this.hitOrder.indexOf(view);
		const moveToTop = !(options?.preserveHitTestOrder && known && index > -1);
		if (moveToTop) {
			if (index > -1) {
				this.hitOrder.splice(index, 1);
			}
			this.hitOrder.push(view);
		}
		this.applyGeometry(view, geometry, moveToTop);
	}

	clearGeometry(view: View) {
		if (!this.geometry.delete(view)) {
			return;
		}
		const index = this.hitOrder.indexOf(view);
		if (index > -1) {
			this.hitOrder.splice(index, 1);
		}
		this.applyGeometry(view, null);
	}

	/** Border box (DIP) to canvas view (DIP). */
	protected viewTransform(geometry: ElementGeometry): Mat4 | null {
		if (!geometry.transform) {
			return null;
		}
		const scale = this.gridScale();
		return mat4.multiply(mat4.scale(1 / scale.x, 1 / scale.y), geometry.transform);
	}

	/**
	 * drawElementImage sets geometry on every draw, and most of it is unchanged: whether the host
	 * needs `m` (null: cleared) and `clip` for `view`, and whether it still needs raising.
	 */
	protected geometryUpdate(view: View, native: any, m: Mat4 | null, clip: Rect | null, moveToTop: boolean): { send: boolean; raise: boolean } {
		const last = this.sent.get(view);
		if (!m) {
			this.sent.delete(view);
			return { send: !!last, raise: false };
		}
		const raise = moveToTop && this.top !== view;
		if (!raise && last && last.native === native && mat4.equals(last.m, m) && sameRect(last.clip, clip)) {
			return { send: false, raise: false };
		}
		this.sent.set(view, { native, m, clip: clip ? { ...clip } : null });
		if (raise) {
			this.top = view;
		}
		return { send: true, raise };
	}

	protected syncGeometry() {
		for (const [view, geometry] of this.geometry) {
			this.applyGeometry(view, geometry);
		}
	}

	frame(frameNumber: number) {
		const canvas = this.canvas;
		if (!canvas || this.disposed) {
			DrawableController.deactivate(this);
			return;
		}
		const changed: View[] = [];
		// The grid's scale is only known once the canvas has a size.
		const size = canvas.content === 'drawable' ? this.canvasCssSize(canvas) : null;
		if (size && size.width > 0 && size.height > 0) {
			const list = this.drawables();
			const live = new Set(list);
			for (const [view, rec] of this.snapshots) {
				if (!live.has(view)) {
					this.releaseRecord(rec);
					this.snapshots.delete(view);
				}
			}
			for (const view of Array.from(this.geometry.keys())) {
				if (!live.has(view) && !this.contains(view)) {
					this.clearGeometry(view);
				}
			}
			changed.push(...this.refresh(list));
			this.syncGeometry();
		}
		if (changed.length || this.paintRequested) {
			this.paintRequested = false;
			canvas._dispatchPaint?.(changed);
		}
		if (canvas.content !== 'drawable' || !canvas.isLoaded) {
			DrawableController.deactivate(this);
		}
	}

	/** Brings snapshots up to date; returns the views that got a new one. */
	protected abstract refresh(list: View[]): View[];

	/** Notes that `view` has a new native snapshot, taken at `scale`; `css` is its size (DIP). */
	protected recordSnapshot(view: View, scale: { x: number; y: number }, css = this.cssSize(view)) {
		let rec = this.snapshots.get(view);
		if (!rec) {
			rec = { asset: null, width: 0, height: 0, cssWidth: 0, cssHeight: 0, defaultWidth: 0, defaultHeight: 0 };
			this.snapshots.set(view, rec);
		}
		rec.native = true;
		rec.width = Math.max(1, Math.ceil(css.width * scale.x - 1e-3));
		rec.height = Math.max(1, Math.ceil(css.height * scale.y - 1e-3));
		rec.cssWidth = css.width;
		rec.cssHeight = css.height;
		rec.defaultWidth = css.width * scale.x;
		rec.defaultHeight = css.height * scale.y;
		rec.generation = ++this.generation;
	}

	private static active = new Set<DrawableController>();
	private static scheduled = false;
	static frameNumber = 0;

	static activate(controller: DrawableController) {
		if (controller.disposed) {
			return;
		}
		DrawableController.active.add(controller);
		DrawableController.schedule();
	}

	static deactivate(controller: DrawableController) {
		DrawableController.active.delete(controller);
	}

	private static schedule() {
		if (DrawableController.scheduled || DrawableController.active.size === 0) {
			return;
		}
		const raf = (global as any).requestAnimationFrame;
		if (typeof raf !== 'function') {
			return;
		}
		DrawableController.scheduled = true;
		raf(DrawableController.tick);
	}

	// Re-requested last, so it runs after the page's callbacks.
	private static tick = () => {
		DrawableController.scheduled = false;
		const frame = ++DrawableController.frameNumber;
		const list = Array.from(DrawableController.active).reverse();
		for (const controller of list) {
			try {
				controller.frame(frame);
			} catch (e) {
				console.error(e);
			}
		}
		DrawableController.schedule();
	};
}

export function elementImageSource(image: ElementImage, method: string): ElementSource {
	if (image._closed) {
		throw domError('InvalidStateError', `Failed to execute '${method}': The ElementImage is closed.`);
	}
	return {
		asset: image._asset,
		width: image._width,
		height: image._height,
		defaultWidth: image._defaultWidth,
		defaultHeight: image._defaultHeight,
		cssWidth: image._cssWidth,
		cssHeight: image._cssHeight,
		view: image._element?.deref() ?? null,
		canvas: image._canvas?.deref() ?? null,
	};
}

export function canvasOf(view: any): any {
	let node = view?.parent;
	while (node) {
		if (typeof node._elementSource === 'function') {
			return node;
		}
		node = node.parent;
	}
	return null;
}

/** `owner` is null for WebGPU. */
export function resolveElementSource(owner: any, element: any, method: string): ElementSource {
	if (element instanceof ElementImage) {
		return elementImageSource(element, method);
	}
	const canvas = owner && typeof owner._elementSource === 'function' ? owner : owner ? null : canvasOf(element);
	if (!canvas) {
		throw new TypeError(`Failed to execute '${method}': parameter 1 is not of type 'ElementImage'.`);
	}
	return canvas._elementSource(element, method);
}

/** `crop` is in snapshot pixels. */
export function elementAsset(source: ElementSource, method: string, crop?: Rect | null, width?: number, height?: number): { asset: ImageAsset; width: number; height: number } {
	const rect = crop ?? { x: 0, y: 0, width: source.width, height: source.height };
	const outWidth = Math.round(width ?? rect.width);
	const outHeight = Math.round(height ?? rect.height);
	if (rect.x === 0 && rect.y === 0 && rect.width === source.width && rect.height === source.height && outWidth === source.width && outHeight === source.height) {
		return { asset: source.asset, width: outWidth, height: outHeight };
	}
	const controller: DrawableController | null = source.canvas?._drawables ?? null;
	if (!controller || !source.view || outWidth <= 0 || outHeight <= 0) {
		throw domError('InvalidStateError', `Failed to execute '${method}': The element is no longer available to crop or scale.`);
	}
	const rx = source.width / (source.cssWidth || 1);
	const ry = source.height / (source.cssHeight || 1);
	const asset = controller.render(source.view, outWidth, outHeight, { x: rect.x / rx, y: rect.y / ry, width: rect.width / rx, height: rect.height / ry });
	if (!asset) {
		throw domError('InvalidStateError', `Failed to execute '${method}': The element could not be rendered.`);
	}
	return { asset, width: outWidth, height: outHeight };
}
