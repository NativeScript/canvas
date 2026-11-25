import { View } from '@nativescript/core';
import { ImageAsset } from '../../ImageAsset';
import { DrawableController, ElementGeometry, Rect, SnapshotRecord } from './common';

export * from './common';

const addresses = new WeakMap<object, number>();

function address(view: View): number {
	const native: any = view?.nativeViewProtected;
	if (!native || typeof native !== 'object') {
		return 0;
	}
	let value = addresses.get(native);
	if (value === undefined) {
		value = typeof native.toNumber === 'function' ? native.toNumber() : 0;
		addresses.set(native, value);
	}
	return value;
}

/**
 * Parenting, allocation (which places each child where it was drawn and clips it there, for GTK's
 * picking), change tracking and capture live in `CanvasModule.NSCDrawableHost` (crates/canvas-napi).
 */
class LinuxDrawableController extends DrawableController {
	readonly host: any;
	private list: View[] = [];
	private scaleX = 0;
	private scaleY = 0;

	constructor(canvas: any) {
		super(canvas);
		this.host = new global.CanvasModule.NSCDrawableHost(canvas._canvas);
	}

	attachChild(child: View, index?: number): boolean {
		const widget = address(child);
		return widget ? this.host.attach(widget, typeof index === 'number' ? index : -1) : false;
	}

	detachChild(child: View) {
		const widget = address(child);
		if (widget) {
			this.host.detach(widget);
		}
	}

	/** Layout units on Linux are DIPs. */
	layoutHost() {
		const sizes: number[] = [];
		this.canvas?.eachChildView((child: View) => {
			const widget = address(child);
			if (widget) {
				sizes.push(widget, child.getMeasuredWidth(), child.getMeasuredHeight());
			}
			return true;
		});
		if (sizes.length) {
			this.host.placeAll(sizes);
		}
	}

	cssSize(view: View): { width: number; height: number } {
		const widget = address(view);
		if (!widget) {
			return { width: 0, height: 0 };
		}
		const [width, height] = this.host.size(widget);
		return { width, height };
	}

	protected refresh(list: View[]): View[] {
		const live = list.filter((view) => !!address(view));
		if (live.length !== this.list.length || live.some((view, i) => view !== this.list[i])) {
			this.list = live;
			this.host.setDrawables(live.map(address));
		}
		const scale = this.gridScale();
		// [index, cssWidth, cssHeight, ...]
		const updates: number[] | null = this.host.update(scale.x, scale.y);
		if (!updates) {
			return [];
		}
		const changed: View[] = [];
		for (let i = 0; i + 2 < updates.length; i += 3) {
			const view = this.list[updates[i]];
			if (!view) {
				continue;
			}
			this.recordSnapshot(view, scale, { width: updates[i + 1], height: updates[i + 2] });
			changed.push(view);
		}
		return changed;
	}

	assetOf(view: View, rec: SnapshotRecord): ImageAsset | null {
		if (rec.asset && rec.assetGeneration === rec.generation) {
			return rec.asset;
		}
		const asset = rec.asset ?? new ImageAsset();
		if (this.host.loadSnapshot(address(view), asset.native)) {
			rec.asset = asset;
			rec.assetGeneration = rec.generation;
		}
		return rec.asset;
	}

	render(view: View, width: number, height: number, source?: Rect): ImageAsset | null {
		const widget = address(view);
		width = Math.max(1, Math.round(width));
		height = Math.max(1, Math.round(height));
		if (!widget) {
			return null;
		}
		const asset = new ImageAsset();
		// An empty source rect is the whole border box.
		return this.host.capture(widget, width, height, source?.x ?? 0, source?.y ?? 0, source?.width ?? 0, source?.height ?? 0, asset.native) ? asset : null;
	}

	protected applyGeometry(view: View, geometry: ElementGeometry | null, moveToTop = false) {
		const widget = address(view);
		if (!widget) {
			return;
		}
		const m = geometry ? this.viewTransform(geometry) : null;
		const clip = geometry?.clip ?? null;
		const { send, raise } = this.geometryUpdate(view, widget, m, clip, moveToTop);
		if (!send) {
			return;
		}
		if (m) {
			this.host.setGeometry(widget, m, !!clip, clip?.x ?? 0, clip?.y ?? 0, clip?.width ?? 0, clip?.height ?? 0, raise);
		} else {
			this.host.clearGeometry(widget);
		}
	}

	/** Geometry is in grid units: re-sent only when the grid's scale changes. */
	protected syncGeometry() {
		const scale = this.gridScale();
		if (scale.x === this.scaleX && scale.y === this.scaleY) {
			return;
		}
		this.scaleX = scale.x;
		this.scaleY = scale.y;
		for (const [view, geometry] of this.geometry) {
			this.applyGeometry(view, geometry, false);
		}
	}

	protected releaseRecord(rec: SnapshotRecord) {
		super.releaseRecord(rec);
		rec.native = null;
	}

	dispose() {
		super.dispose();
		this.host.dispose();
	}
}

export function createDrawableController(canvas: any): DrawableController | null {
	return global.CanvasModule?.NSCDrawableHost ? new LinuxDrawableController(canvas) : null;
}
