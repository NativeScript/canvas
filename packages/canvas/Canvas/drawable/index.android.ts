import { View, Screen } from '@nativescript/core';
import { ImageAsset } from '../../ImageAsset';
import { DrawableController, ElementGeometry, mat4, Rect, SnapshotRecord } from './common';

export * from './common';

declare const org: any;

/** Layout, invalidation, touch routing and capture live in `NSCDrawableHost` (Kotlin). */
class AndroidDrawableController extends DrawableController {
	readonly host: any;
	private list: View[] = [];
	private scaleX = 0;
	private scaleY = 0;

	constructor(canvas: any) {
		super(canvas);
		const native = canvas.nativeViewProtected as android.widget.FrameLayout;
		this.host = new org.nativescript.canvas.NSCDrawableHost(native.getContext());
		native.addView(this.host, new android.widget.FrameLayout.LayoutParams(android.view.ViewGroup.LayoutParams.MATCH_PARENT, android.view.ViewGroup.LayoutParams.MATCH_PARENT));
	}

	attachChild(child: View, index?: number): boolean {
		const native = child.nativeViewProtected as android.view.View;
		if (!native) {
			return false;
		}
		const count = this.host.getChildCount();
		this.host.addView(native, typeof index === 'number' && index < count ? index : -1);
		return true;
	}

	detachChild(child: View) {
		const native = child.nativeViewProtected as android.view.View;
		if (native && native.getParent() === this.host) {
			this.host.removeView(native);
		}
	}

	private density() {
		return Screen.mainScreen.scale || 1;
	}

	cssSize(view: View): { width: number; height: number } {
		const native = view.nativeViewProtected as android.view.View;
		if (!native) {
			return { width: 0, height: 0 };
		}
		const d = this.density();
		return { width: native.getWidth() / d, height: native.getHeight() / d };
	}

	protected refresh(list: View[]): View[] {
		const live = list.filter((view) => !!view.nativeViewProtected);
		if (live.length !== this.list.length || live.some((view, i) => view !== this.list[i])) {
			this.list = live;
			const array = Array.create(android.view.View, live.length);
			live.forEach((view, i) => (array[i] = view.nativeViewProtected));
			this.host.setDrawables(array);
		}
		const scale = this.gridScale();
		const d = this.density();
		const indices = this.host.update(scale.x / d, scale.y / d);
		if (!indices) {
			return [];
		}
		const changed: View[] = [];
		for (let i = 0; i < indices.length; i++) {
			const view = this.list[indices[i]];
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
		const snapshot = this.nativeSnapshot(view);
		if (!snapshot) {
			return rec.asset;
		}
		try {
			const bitmap = snapshot.toBitmap();
			if (bitmap) {
				rec.asset ??= new ImageAsset();
				rec.asset.loadFromNativeSync(bitmap);
				rec.assetGeneration = rec.generation;
			}
		} finally {
			snapshot.close();
		}
		return rec.asset;
	}

	render(view: View, width: number, height: number, source?: Rect): ImageAsset | null {
		const native = view.nativeViewProtected as android.view.View;
		width = Math.max(1, Math.round(width));
		height = Math.max(1, Math.round(height));
		if (!native) {
			return null;
		}
		const d = this.density();
		const css = this.cssSize(view);
		const rect = source ?? { x: 0, y: 0, width: css.width, height: css.height };
		if (rect.width <= 0 || rect.height <= 0) {
			return null;
		}
		const snapshot = this.host.capture(native, width, height, width / (rect.width * d), height / (rect.height * d), rect.x * d, rect.y * d);
		if (!snapshot) {
			return null;
		}
		try {
			const asset = new ImageAsset();
			asset.loadFromNativeSync(snapshot.toBitmap());
			return asset;
		} finally {
			snapshot.close();
		}
	}

	protected applyGeometry(view: View, geometry: ElementGeometry | null, moveToTop = false) {
		const native = view.nativeViewProtected;
		if (!native) {
			return;
		}
		const forward = geometry ? this.viewTransform(geometry) : null;
		const clip = geometry?.clip ?? null;
		const { send, raise } = this.geometryUpdate(view, native, forward, clip, moveToTop);
		if (!send) {
			return;
		}
		if (!forward) {
			this.host.clearGeometry(native);
			return;
		}
		const d = this.density();
		// Element pixels to host pixels.
		const m = mat4.chain(mat4.scale(d, d), forward, mat4.scale(1 / d, 1 / d));
		const left = clip ? Math.min(clip.x, clip.x + clip.width) * d : 0;
		const top = clip ? Math.min(clip.y, clip.y + clip.height) * d : 0;
		const right = clip ? left + Math.abs(clip.width) * d : 0;
		const bottom = clip ? top + Math.abs(clip.height) * d : 0;
		this.host.setGeometry(native, m[0], m[4], m[12], m[1], m[5], m[13], m[3], m[7], m[15], !!clip, left, top, right, bottom, !raise);
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
		(this.host.getParent() as android.view.ViewGroup)?.removeView(this.host);
	}
}

export function createDrawableController(canvas: any): DrawableController | null {
	return new AndroidDrawableController(canvas);
}
