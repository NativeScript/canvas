import { View } from '@nativescript/core';
import { ImageAsset } from '../../ImageAsset';
import { DrawableController, ElementGeometry, Mat4, mat4, Rect, rectContains, SnapshotRecord, COLD_INTERVAL } from './common';

export * from './common';

// UIKit does not hit test views at alpha 0, hence the override.
@NativeClass()
class NSCDrawableHost extends UIView {
	_owner: WeakRef<IOSDrawableController>;

	static initWithOwner(owner: IOSDrawableController): NSCDrawableHost {
		const host = NSCDrawableHost.alloc().initWithFrame(CGRectZero) as NSCDrawableHost;
		host._owner = new WeakRef(owner);
		host.alpha = 0;
		host.clipsToBounds = false;
		host.autoresizingMask = UIViewAutoresizing.FlexibleWidth | UIViewAutoresizing.FlexibleHeight;
		return host;
	}

	hitTestWithEvent(point: CGPoint, event: _UIEvent): UIView {
		return this._owner?.deref()?.hitTest(this, point, event) ?? null;
	}

	pointInsideWithEvent(point: CGPoint, event: _UIEvent): boolean {
		return !!this.hitTestWithEvent(point, event);
	}
}

const BITMAP_INFO = CGImageAlphaInfo.kCGImageAlphaPremultipliedLast | CGBitmapInfo.kCGBitmapByteOrder32Big;

let colorSpace: any;
function deviceRGB() {
	if (!colorSpace) {
		colorSpace = CGColorSpaceCreateDeviceRGB();
	}
	return colorSpace;
}

interface Buffer {
	data: NSMutableData;
	context: any;
	bytes: Uint8Array;
}

function makeBuffer(width: number, height: number): Buffer {
	const data = NSMutableData.dataWithLength(width * height * 4);
	const context = CGBitmapContextCreate(data.mutableBytes, width, height, 8, width * 4, deviceRGB(), BITMAP_INFO);
	return { data, context, bytes: new Uint8Array(interop.bufferFromData(data)) };
}

function toCATransform(m: Mat4): CATransform3D {
	const t = new CATransform3D(CATransform3DIdentity);
	t.m11 = m[0];
	t.m12 = m[1];
	t.m13 = m[2];
	t.m14 = m[3];
	t.m21 = m[4];
	t.m22 = m[5];
	t.m23 = m[6];
	t.m24 = m[7];
	t.m31 = m[8];
	t.m32 = m[9];
	t.m33 = m[10];
	t.m34 = m[11];
	t.m41 = m[12];
	t.m42 = m[13];
	t.m43 = m[14];
	t.m44 = m[15];
	return t;
}

/** Assumes nothing in between is 3D. */
function chainToHost(superview: UIView, host: UIView): Mat4 {
	const o = superview.convertPointToView(CGPointMake(0, 0), host);
	const x = superview.convertPointToView(CGPointMake(1, 0), host);
	const y = superview.convertPointToView(CGPointMake(0, 1), host);
	return mat4.from({ a: x.x - o.x, b: x.y - o.y, c: y.x - o.x, d: y.y - o.y, e: o.x, f: o.y });
}

function renderLayer(nativeView: UIView, context: any, width: number, height: number, scaleX: number, scaleY: number, source: Rect | null, hide: UIView[]) {
	CGContextClearRect(context, CGRectMake(0, 0, width, height));
	CGContextSaveGState(context);
	CGContextTranslateCTM(context, 0, height);
	CGContextScaleCTM(context, scaleX, -scaleY);
	const origin = nativeView.layer.bounds.origin;
	CGContextTranslateCTM(context, -origin.x - (source?.x ?? 0), -origin.y - (source?.y ?? 0));
	let layer: CALayer = nativeView.layer;
	if (hide.length) {
		CATransaction.begin();
		CATransaction.setDisableActions(true);
		for (const view of hide) {
			view.layer.hidden = true;
		}
	} else {
		// Catches in-flight animations.
		layer = nativeView.layer.presentationLayer() ?? nativeView.layer;
	}
	try {
		layer.renderInContext(context);
	} finally {
		if (hide.length) {
			for (const view of hide) {
				view.layer.hidden = false;
			}
			CATransaction.commit();
		}
		CGContextRestoreGState(context);
	}
}

class IOSDrawableController extends DrawableController {
	readonly host: NSCDrawableHost;

	constructor(canvas: any) {
		super(canvas);
		this.host = NSCDrawableHost.initWithOwner(this);
		const native = canvas.nativeViewProtected as UIView;
		if (native) {
			native.addSubview(this.host);
			this.host.frame = native.bounds;
		}
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
		if (!canvas) {
			return;
		}
		if (this.host.superview !== canvas) {
			canvas.addSubview(this.host);
		} else if (canvas.subviews.lastObject !== this.host) {
			canvas.bringSubviewToFront(this.host);
		}
		this.host.frame = canvas.bounds;
	}

	hitTest(host: UIView, point: CGPoint, event: _UIEvent): UIView | null {
		this.markHot();
		for (let i = this.hitOrder.length - 1; i >= 0; i--) {
			const view = this.hitOrder[i];
			const geometry = this.geometry.get(view);
			const native = view.nativeViewProtected as UIView;
			if (!geometry?.transform || !native || native.hidden || !native.userInteractionEnabled) {
				continue;
			}
			const local = host.convertPointToView(point, native);
			const origin = native.bounds.origin;
			if (geometry.clip && !rectContains(geometry.clip, local.x - origin.x, local.y - origin.y)) {
				continue;
			}
			const hit = native.hitTestWithEvent(local, event);
			if (hit) {
				return hit;
			}
		}
		return null;
	}

	protected applyGeometry(view: View, geometry: ElementGeometry | null) {
		const native = view.nativeViewProtected as UIView;
		if (!native) {
			return;
		}
		const target = geometry ? this.viewTransform(geometry) : null;
		if (!target) {
			if ((view as any)._drawableTransform) {
				(view as any)._drawableTransform = null;
				native.layer.transform = CATransform3DIdentity;
				(view as any)._isTransformed = false;
				(view as any).updateNativeTransform?.();
			}
			return;
		}
		const superview = native.superview;
		if (!superview) {
			return;
		}
		const toHost = superview === this.host ? null : chainToHost(superview, this.host);
		const fromHost = toHost ? mat4.invert(toHost) : null;
		if (toHost && !fromHost) {
			return;
		}
		// p -> position + M * (p - anchor)
		const size = native.bounds.size;
		const anchor = native.layer.anchorPoint;
		const position = native.layer.position;
		const parts = [mat4.translate(-position.x, -position.y)];
		if (fromHost) {
			parts.push(fromHost);
		}
		parts.push(target, mat4.translate(anchor.x * size.width, anchor.y * size.height));
		const m = mat4.chain(...parts);
		if (mat4.equals(m, (view as any)._drawableTransform)) {
			return;
		}
		(view as any)._drawableTransform = m;
		// Core then resets the transform around its frame updates.
		(view as any)._isTransformed = true;
		CATransaction.begin();
		CATransaction.setDisableActions(true);
		native.layer.transform = toCATransform(m);
		CATransaction.commit();
	}

	protected needsCapture(view: View, rec: SnapshotRecord, frame: number): boolean {
		if (frame <= (rec.hotUntil ?? 0)) {
			return true;
		}
		// No redraw signal on iOS: idle drawables are polled.
		rec.phase ??= Math.floor(Math.random() * COLD_INTERVAL);
		return (frame + rec.phase) % COLD_INTERVAL === 0;
	}

	protected capture(view: View, rec: SnapshotRecord, width: number, height: number, scaleX: number, scaleY: number, nested: View[]): boolean {
		const native = view.nativeViewProtected as UIView;
		if (!native) {
			return false;
		}
		const resized = rec.width !== width || rec.height !== height || !rec.front;
		if (resized) {
			rec.front = makeBuffer(width, height);
			rec.back = makeBuffer(width, height);
		}
		const back: Buffer = rec.back;
		const hide = nested.map((v) => v.nativeViewProtected as UIView).filter((v) => !!v);
		renderLayer(native, back.context, width, height, scaleX, scaleY, null, hide);
		if (!resized && rec.asset && back.data.isEqualToData(rec.front.data)) {
			return false;
		}
		rec.back = rec.front;
		rec.front = back;
		if (!rec.asset) {
			rec.asset = new ImageAsset();
		}
		rec.asset.loadFromBytesSync(width, height, back.bytes, true);
		rec.width = width;
		rec.height = height;
		return true;
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
		const buffer = makeBuffer(width, height);
		const hide = this.nestedDrawables(view)
			.map((v) => v.nativeViewProtected as UIView)
			.filter((v) => !!v);
		renderLayer(native, buffer.context, width, height, width / rect.width, height / rect.height, source ?? null, hide);
		const asset = new ImageAsset();
		asset.loadFromBytesSync(width, height, buffer.bytes, true);
		return asset;
	}

	protected releaseRecord(rec: SnapshotRecord) {
		super.releaseRecord(rec);
		rec.front = rec.back = null;
	}

	dispose() {
		for (const view of Array.from(this.geometry.keys())) {
			this.applyGeometry(view, null);
		}
		super.dispose();
		this.host.removeFromSuperview();
	}
}

export function createDrawableController(canvas: any): DrawableController | null {
	return new IOSDrawableController(canvas);
}
