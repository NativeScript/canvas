import { View, Screen } from '@nativescript/core';
import { ImageAsset } from '../../ImageAsset';
import { DrawableController, ElementGeometry, Mat4, mat4, Rect, rectContains, SnapshotRecord, COLD_INTERVAL } from './common';

export * from './common';

const SDK = android.os.Build.VERSION.SDK_INT;
// Below it drawables are polled.
const HAS_INVALIDATION_HOOK = SDK >= 26;

@NativeClass()
class NSCDrawableHost extends org.nativescript.widgets.StackLayout {
	owner: WeakRef<AndroidDrawableController>;

	constructor(context: android.content.Context) {
		super(context);
		return global.__native(this);
	}

	dispatchDraw(canvas: android.graphics.Canvas) {}

	// Reports only a size the canvas already has, so children never resize it.
	onMeasure(widthMeasureSpec: number, heightMeasureSpec: number) {
		const MeasureSpec = android.view.View.MeasureSpec;
		const parent = this.getParent() as android.view.ViewGroup;
		let siblingWidth = 0;
		let siblingHeight = 0;
		if (parent instanceof android.view.ViewGroup) {
			for (let i = 0; i < parent.getChildCount(); i++) {
				const child = parent.getChildAt(i);
				if (child !== this && child.getVisibility() !== android.view.View.GONE) {
					siblingWidth = Math.max(siblingWidth, child.getMeasuredWidth());
					siblingHeight = Math.max(siblingHeight, child.getMeasuredHeight());
				}
			}
		}
		const width = MeasureSpec.getMode(widthMeasureSpec) === MeasureSpec.EXACTLY ? MeasureSpec.getSize(widthMeasureSpec) : siblingWidth;
		const height = MeasureSpec.getMode(heightMeasureSpec) === MeasureSpec.EXACTLY ? MeasureSpec.getSize(heightMeasureSpec) : siblingHeight;
		super.onMeasure(MeasureSpec.makeMeasureSpec(width, MeasureSpec.EXACTLY), MeasureSpec.makeMeasureSpec(0, MeasureSpec.UNSPECIFIED));
		this.setMeasuredDimension(width, height);
	}

	onDescendantInvalidated(child: android.view.View, target: android.view.View) {
		super.onDescendantInvalidated(child, target);
		this.owner?.deref()?.invalidated(target);
	}

	dispatchTouchEvent(event: android.view.MotionEvent): boolean {
		return this.owner?.deref()?.dispatchTouch(event) ?? false;
	}
}

function toAndroidMatrix(m: Mat4): android.graphics.Matrix {
	const matrix = new android.graphics.Matrix();
	// Drops z: row-major 3x3 of x, y and w.
	matrix.setValues([m[0], m[4], m[12], m[1], m[5], m[13], m[3], m[7], m[15]]);
	return matrix;
}

interface TouchTarget {
	native: android.view.View;
	inverse: android.graphics.Matrix;
}

class AndroidDrawableController extends DrawableController {
	readonly host: NSCDrawableHost;
	private capturing = false;
	private touchTarget: TouchTarget | null = null;
	private byNative = new Map<any, View>();

	constructor(canvas: any) {
		super(canvas);
		const native = canvas.nativeViewProtected as android.widget.FrameLayout;
		this.host = new NSCDrawableHost(native.getContext());
		this.host.owner = new WeakRef(this);
		this.host.setOrientation(org.nativescript.widgets.Orientation.vertical);
		// Above the surfaces, for touch order.
		this.host.setTranslationZ(1e4);
		this.host.setClipChildren(false);
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

	drawables(): View[] {
		const list = super.drawables();
		this.byNative.clear();
		for (const view of list) {
			if (view.nativeViewProtected) {
				this.byNative.set(view.nativeViewProtected, view);
			}
		}
		return list;
	}

	invalidated(target: android.view.View) {
		if (this.capturing) {
			return;
		}
		let node: any = target;
		while (node && node !== this.host) {
			const view = this.byNative.get(node);
			if (view) {
				const rec = this.snapshots.get(view);
				if (rec) {
					rec.dirty = true;
				}
				return;
			}
			node = node.getParent?.();
		}
		// New since the last frame.
		for (const rec of this.snapshots.values()) {
			rec.dirty = true;
		}
	}

	private density() {
		return Screen.mainScreen.scale || 1;
	}

	private pixelTransform(geometry: ElementGeometry): Mat4 | null {
		const view = this.viewTransform(geometry);
		if (!view) {
			return null;
		}
		const d = this.density();
		return mat4.chain(mat4.scale(d, d), view, mat4.scale(1 / d, 1 / d));
	}

	private findTouchTarget(x: number, y: number): TouchTarget | null {
		const d = this.density();
		for (let i = this.hitOrder.length - 1; i >= 0; i--) {
			const view = this.hitOrder[i];
			const geometry = this.geometry.get(view);
			const native = view.nativeViewProtected as android.view.View;
			if (!geometry?.transform || !native || native.getVisibility() !== android.view.View.VISIBLE) {
				continue;
			}
			const forward = this.pixelTransform(geometry);
			const inverse = forward ? mat4.invert(forward) : null;
			if (!inverse) {
				continue;
			}
			const local = mat4.apply(inverse, x, y);
			if (local.x < 0 || local.y < 0 || local.x >= native.getWidth() || local.y >= native.getHeight()) {
				continue;
			}
			if (geometry.clip && !rectContains(geometry.clip, local.x / d, local.y / d)) {
				continue;
			}
			return { native, inverse: toAndroidMatrix(inverse) };
		}
		return null;
	}

	// The canvas still gets every event, as if it bubbled.
	dispatchTouch(event: android.view.MotionEvent): boolean {
		const MotionEvent = android.view.MotionEvent;
		const action = event.getActionMasked();
		if (action === MotionEvent.ACTION_DOWN) {
			this.touchTarget = this.findTouchTarget(event.getX(), event.getY());
		}
		const target = this.touchTarget;
		if (!target) {
			return false;
		}
		const copy = MotionEvent.obtain(event);
		copy.transform(target.inverse);
		let handled = false;
		try {
			handled = target.native.dispatchTouchEvent(copy);
		} finally {
			copy.recycle();
		}
		if (action === MotionEvent.ACTION_UP || action === MotionEvent.ACTION_CANCEL) {
			this.touchTarget = null;
		}
		if (action === MotionEvent.ACTION_DOWN && !handled) {
			this.touchTarget = null;
			return false;
		}
		const canvas = this.canvas?.nativeViewProtected as android.view.View;
		canvas?.onTouchEvent(event);
		return true;
	}

	// dispatchTouch reads the geometry; the views keep their layout position.
	protected applyGeometry(view: View, geometry: ElementGeometry | null) {}

	protected syncGeometry() {}

	protected needsCapture(view: View, rec: SnapshotRecord, frame: number): boolean {
		if (rec.dirty) {
			return true;
		}
		if (HAS_INVALIDATION_HOOK) {
			return false;
		}
		rec.phase ??= Math.floor(Math.random() * COLD_INTERVAL);
		return (frame + rec.phase) % COLD_INTERVAL === 0;
	}

	private draw(native: android.view.View, canvas: android.graphics.Canvas, scaleX: number, scaleY: number, source: Rect | null, nested: View[]) {
		const d = this.density();
		const hide = nested.map((v) => v.nativeViewProtected as android.view.View).filter((v) => v && v.getVisibility() === android.view.View.VISIBLE);
		const save = canvas.save();
		this.capturing = true;
		try {
			for (const v of hide) {
				v.setVisibility(android.view.View.INVISIBLE);
			}
			canvas.scale(scaleX / d, scaleY / d);
			canvas.translate(-native.getScrollX() - (source ? source.x * d : 0), -native.getScrollY() - (source ? source.y * d : 0));
			native.draw(canvas);
		} finally {
			for (const v of hide) {
				v.setVisibility(android.view.View.VISIBLE);
			}
			this.capturing = false;
			canvas.restoreToCount(save);
		}
	}

	protected capture(view: View, rec: SnapshotRecord, width: number, height: number, scaleX: number, scaleY: number, nested: View[]): boolean {
		const native = view.nativeViewProtected as android.view.View;
		if (!native || native.getWidth() <= 0 || native.getHeight() <= 0) {
			return false;
		}
		let bitmap = rec.bitmap as android.graphics.Bitmap;
		if (!bitmap || bitmap.getWidth() !== width || bitmap.getHeight() !== height) {
			bitmap?.recycle();
			bitmap = android.graphics.Bitmap.createBitmap(width, height, android.graphics.Bitmap.Config.ARGB_8888);
			rec.bitmap = bitmap;
			rec.canvas = new android.graphics.Canvas(bitmap);
		} else {
			bitmap.eraseColor(0);
		}
		this.draw(native, rec.canvas, scaleX, scaleY, null, nested);
		if (!rec.asset) {
			rec.asset = new ImageAsset();
		}
		rec.asset.loadFromNativeSync(bitmap);
		rec.width = width;
		rec.height = height;
		rec.dirty = false;
		return true;
	}

	render(view: View, width: number, height: number, source?: Rect): ImageAsset | null {
		const native = view.nativeViewProtected as android.view.View;
		width = Math.max(1, Math.round(width));
		height = Math.max(1, Math.round(height));
		if (!native || native.getWidth() <= 0 || native.getHeight() <= 0) {
			return null;
		}
		const css = this.cssSize(view);
		const rect = source ?? { x: 0, y: 0, width: css.width, height: css.height };
		if (rect.width <= 0 || rect.height <= 0) {
			return null;
		}
		const bitmap = android.graphics.Bitmap.createBitmap(width, height, android.graphics.Bitmap.Config.ARGB_8888);
		this.draw(native, new android.graphics.Canvas(bitmap), width / rect.width, height / rect.height, source ?? null, this.nestedDrawables(view));
		const asset = new ImageAsset();
		asset.loadFromNativeSync(bitmap);
		bitmap.recycle();
		return asset;
	}

	protected releaseRecord(rec: SnapshotRecord) {
		super.releaseRecord(rec);
		(rec.bitmap as android.graphics.Bitmap)?.recycle();
		rec.bitmap = rec.canvas = null;
	}

	dispose() {
		super.dispose();
		this.touchTarget = null;
		this.byNative.clear();
		const parent = this.host.getParent() as android.view.ViewGroup;
		parent?.removeView(this.host);
	}
}

export function createDrawableController(canvas: any): DrawableController | null {
	return new AndroidDrawableController(canvas);
}
