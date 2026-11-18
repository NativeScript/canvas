import { View } from '@nativescript/core';
import { ImageAsset } from '../../ImageAsset';
import { DrawableController, ElementGeometry, Rect, SnapshotRecord } from './common';

export * from './common';

declare const NSCDrawableHost: any;

/** Hit testing, layer transforms, change detection and capture live in `NSCDrawableHost` (Swift). */
class IOSDrawableController extends DrawableController {
	readonly host: any;
	private list: View[] = [];

	constructor(canvas: any) {
		super(canvas);
		const native = canvas.nativeViewProtected as UIView;
		this.host = NSCDrawableHost.alloc().initWithFrame(native.bounds);
		native.addSubview(this.host);
	}

	// Core's size reads the frame, which follows the hit-testing transform.
	cssSize(view: View): { width: number; height: number } {
		const native = view.nativeViewProtected as UIView;
		if (!native) {
			return { width: 0, height: 0 };
		}
		const size = native.bounds.size;
		return { width: size.width, height: size.height };
	}

	attachChild(child: View, index?: number): boolean {
		const native = child.nativeViewProtected as UIView;
		if (!native) {
			return false;
		}
		// Core would inset them by the window's safe area.
		if (!(child as any).iosIgnoreSafeArea) {
			(child as any).iosIgnoreSafeArea = true;
		}
		if (typeof index !== 'number' || index >= this.host.subviews.count) {
			this.host.addSubview(native);
		} else {
			this.host.insertSubviewAtIndex(native, index);
		}
		return true;
	}

	detachChild(child: View) {
		(child.nativeViewProtected as UIView)?.removeFromSuperview();
	}

	layoutHost() {
		const canvas = this.canvas?.nativeViewProtected as UIView;
		if (canvas) {
			this.host.frame = canvas.bounds;
		}
	}

	markHot() {
		this.host.markHot();
	}

	protected refresh(list: View[]): View[] {
		const live = list.filter((view) => !!view.nativeViewProtected);
		if (live.length !== this.list.length || live.some((view, i) => view !== this.list[i])) {
			this.list = live;
			this.host.setDrawables(live.map((view) => view.nativeViewProtected));
		}
		const scale = this.gridScale();
		const indices = this.host.update(scale.x, scale.y);
		if (!indices) {
			return [];
		}
		const changed: View[] = [];
		for (let i = 0; i < indices.count; i++) {
			const view = this.list[indices.objectAtIndex(i)];
			if (!view) {
				continue;
			}
			this.recordSnapshot(view, scale);
			changed.push(view);
		}
		return changed;
	}

	nativeSnapshot(view: View): any {
		const native = view.nativeViewProtected;
		return native ? this.host.snapshot(native) : null;
	}

	assetOf(view: View, rec: SnapshotRecord): ImageAsset | null {
		if (rec.asset && rec.assetGeneration === rec.generation) {
			return rec.asset;
		}
		const image = this.nativeSnapshot(view)?.image();
		if (image) {
			rec.asset ??= new ImageAsset();
			rec.asset.loadFromNativeSync(image);
			rec.assetGeneration = rec.generation;
		}
		return rec.asset;
	}

	render(view: View, width: number, height: number, source?: Rect): ImageAsset | null {
		const native = view.nativeViewProtected as UIView;
		width = Math.max(1, Math.round(width));
		height = Math.max(1, Math.round(height));
		if (!native) {
			return null;
		}
		const css = this.cssSize(view);
		const rect = source ?? { x: 0, y: 0, width: css.width, height: css.height };
		if (rect.width <= 0 || rect.height <= 0) {
			return null;
		}
		const image = this.host.capture(native, width, height, width / rect.width, height / rect.height, rect.x, rect.y)?.image();
		if (!image) {
			return null;
		}
		const asset = new ImageAsset();
		asset.loadFromNativeSync(image);
		return asset;
	}

	protected applyGeometry(view: View, geometry: ElementGeometry | null, moveToTop = false) {
		const native = view.nativeViewProtected;
		if (!native) {
			return;
		}
		const m = geometry ? this.viewTransform(geometry) : null;
		const clip = geometry?.clip ?? null;
		const { send, raise } = this.geometryUpdate(view, native, m, clip, moveToTop);
		if (!send) {
			return;
		}
		if (!m) {
			this.host.clearGeometry(native);
			(view as any)._isTransformed = false;
			(view as any).updateNativeTransform?.();
			return;
		}
		// Core then resets the transform around its frame updates.
		(view as any)._isTransformed = true;
		this.host.setGeometry(native, Array.from(m), !!clip, clip?.x ?? 0, clip?.y ?? 0, clip?.width ?? 0, clip?.height ?? 0, !raise);
	}

	/** The host re-syncs transforms with layout every frame. */
	protected syncGeometry() {}

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
	return new IOSDrawableController(canvas);
}
