import { View } from '@nativescript/core';
import { ImageAsset } from '../../ImageAsset';
import { DrawableController, ElementGeometry, Rect, SnapshotRecord } from './common';

export * from './common';

declare const NSWinRT: any;

const keys = new WeakMap<object, string>();

function keyOf(native: any): string {
	if (!native || typeof native !== 'object') {
		return '';
	}
	let key = keys.get(native);
	if (key === undefined) {
		key = String(NSWinRT.interop.pointerKey(native) ?? '');
		keys.set(native, key);
	}
	return key;
}

function key(view: View): string {
	return keyOf(view?.nativeViewProtected);
}

/**
 * Slots, geometry, change tracking and capture (RenderTargetBitmap, polled) live in
 * `CanvasModule.NSCDrawableHost` (crates/canvas-napi).
 */
class WindowsDrawableController extends DrawableController {
	readonly host: any;
	private list: View[] = [];
	private scaleX = 0;
	private scaleY = 0;

	constructor(canvas: any) {
		super(canvas);
		this.host = new global.CanvasModule.NSCDrawableHost(canvas._canvas);
	}

	/** XAML lays the panel out, not NativeScript. */
	protected canvasCssSize(canvas: any): { width: number; height: number } {
		return { width: canvas._viewWidth || 0, height: canvas._viewHeight || 0 };
	}

	cssSize(view: View): { width: number; height: number } {
		const k = key(view);
		if (!k) {
			return { width: 0, height: 0 };
		}
		const [width, height] = this.host.size(k);
		return { width, height };
	}

	/** Whether `element` (an event's OriginalSource) is inside the hosted children. */
	owns(element: any): boolean {
		const k = element ? String(NSWinRT.interop.pointerKey(element) ?? '') : '';
		return !!k && this.host.owns(k);
	}

	attachChild(child: View, index?: number): boolean {
		const k = key(child);
		return k ? this.host.attach(k, typeof index === 'number' ? index : -1) : false;
	}

	detachChild(child: View) {
		const k = key(child);
		if (k) {
			this.host.detach(k);
		}
	}

	markHot() {
		this.host.markHot();
	}

	protected refresh(list: View[]): View[] {
		const live = list.filter((view) => !!key(view));
		if (live.length !== this.list.length || live.some((view, i) => view !== this.list[i])) {
			this.list = live;
			this.host.setDrawables(live.map(key));
		}
		const scale = this.gridScale();
		// [index, cssWidth, cssHeight, ...]
		const updates: number[] | null = this.host.update(scale.x, scale.y, this.canvas?._viewWidth || 0);
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
		if (this.host.loadSnapshot(key(view), asset.native)) {
			rec.asset = asset;
			rec.assetGeneration = rec.generation;
		}
		return rec.asset;
	}

	render(view: View, width: number, height: number, source?: Rect): ImageAsset | null {
		const k = key(view);
		if (!k) {
			return null;
		}
		const asset = new ImageAsset();
		// An empty source rect is the whole snapshot.
		const ok = this.host.capture(k, Math.max(1, Math.round(width)), Math.max(1, Math.round(height)), source?.x ?? 0, source?.y ?? 0, source?.width ?? 0, source?.height ?? 0, asset.native);
		return ok ? asset : null;
	}

	protected applyGeometry(view: View, geometry: ElementGeometry | null, moveToTop = false) {
		const k = key(view);
		if (!k) {
			return;
		}
		const m = geometry ? this.viewTransform(geometry) : null;
		const clip = geometry?.clip ?? null;
		const { send, raise } = this.geometryUpdate(view, k, m, clip, moveToTop);
		if (!send) {
			return;
		}
		if (m) {
			this.host.setGeometry(k, m, !!clip, clip?.x ?? 0, clip?.y ?? 0, clip?.width ?? 0, clip?.height ?? 0, raise);
		} else {
			this.host.clearGeometry(k);
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
	if (!canvas?.nativeViewProtected || !canvas?._canvas || !global.CanvasModule?.NSCDrawableHost) {
		return null;
	}
	try {
		return new WindowsDrawableController(canvas);
	} catch (e) {
		// A canvas without a SwapChainPanel (one from a Worker).
		return null;
	}
}
