import { LayoutBase, ViewBase, Utils, View, Screen } from '@nativescript/core';
import { Canvas } from '../Canvas';
import { Image } from './Image';
import { Paint } from './Paint';

enum State {
	None,
	Pending,
	Invalidating,
}

export class Dom extends LayoutBase {
	_canvas: Canvas;
	_children: Array<Paint | Image> = [];

	_raf: any;
	_state: State = State.None;
	_isReady: boolean = false;
	_onFrameCallback: ((frame: number) => void) | undefined = undefined;

	constructor() {
		super();
		this._canvas = new Canvas();
		this._canvas.on('ready', this._ready.bind(this));
		this._canvas.style.width = { unit: '%', value: 1 };
		this._canvas.style.height = 'auto';
	}

	createNativeView(): Object {
		if (__IOS__) {
			return UIView.new();
		}

		if (__ANDROID__) {
			return new android.widget.LinearLayout(this._context);
		}
		return super.createNativeView();
	}

	initNativeView(): void {
		super.initNativeView();
		this._addView(this._canvas);
	}

	onLoaded(): void {
		super.onLoaded();
		if (this._isReady) {
			this._bindRaf();
		}
	}

	onUnloaded(): void {
		if (this._raf) {
			cancelAnimationFrame(this._raf);
			this._raf = undefined;
			this._state = State.Pending;
		}
		super.onUnloaded();
	}

	public onLayout(left: number, top: number, right: number, bottom: number): void {
		super.onLayout(left, top, right, bottom);
		View.layoutChild(this, this._canvas, left, top, right, bottom);
		this._canvas.width = this.getMeasuredWidth();
		this._canvas.height = this.getMeasuredHeight();
		// Trigger a redraw now that dimensions are known. The scale transform is
		// applied at the top of every _draw() call so it is always up to date.
		this._dirty();
	}

	public onMeasure(widthMeasureSpec: number, heightMeasureSpec: number) {
		const nativeView = this.nativeView;
		if (nativeView) {
			const width = Utils.layout.getMeasureSpecSize(widthMeasureSpec);
			const height = Utils.layout.getMeasureSpecSize(heightMeasureSpec);
			View.measureChild(this, this._canvas, width, height);
			this.setMeasuredDimension(width, height);
		}
	}

	set onFrameCallback(value: ((frame: number) => void) | undefined) {
		this._onFrameCallback = value;
	}

	_ready() {
		this._isReady = true;
		this._dirty();
		this._draw(null);
	}

	_draw(ts: number | null) {
		const pending = (this._state & State.Pending) === State.Pending;
		if (pending) {
			const ctx = this._canvas.getContext('2d');
			if (ctx) {
				const scale = Screen.mainScreen.scale;
				const w = this._canvas.width as number;
				const h = this._canvas.height as number;
				this._state = State.Invalidating;
				ctx.setTransform(scale, 0, 0, scale, 0, 0);
				ctx.clearRect(0, 0, w / scale, h / scale);
				for (const child of this._children) {
					child.draw();
				}
				this._state = State.None;
			}
		}
		if (ts !== null) {
			this._onFrameCallback?.(ts);
		}
		this._raf = undefined;
		if ((this._state & State.Pending) === State.Pending || typeof this._onFrameCallback === 'function') {
			this._bindRaf();
		}
	}

	_bindRaf() {
		if (!this._isReady) {
			return;
		}
		if (this._raf) {
			return;
		}
		if ((this._state & State.Pending) === State.Pending || typeof this._onFrameCallback === 'function') {
			this._raf = requestAnimationFrame(this._draw.bind(this));
		}
	}

	_dirty() {
		this._state = this._state | State.Pending;
	}

	_addViewToNativeVisualTree(view: ViewBase, atIndex?: number): boolean {
		if (view === this._canvas) {
			if (__IOS__) {
				this.nativeView.addSubview(this._canvas.nativeView);
			}

			if (__ANDROID__) {
				this.nativeView.addView(this._canvas.nativeView);
			}
			return true;
		} else if (view instanceof Paint || view instanceof Image) {
			if (typeof (view as any)._addCanvas === 'function') {
				(view as any)._addCanvas(this._canvas);
			}
			if (typeof atIndex === 'number' && atIndex >= 0 && atIndex <= this._children.length) {
				this._children.splice(atIndex, 0, view);
			} else {
				this._children.push(view);
			}
		}
		return false;
	}

	_removeViewFromNativeVisualTree(view: ViewBase): void {
		if (view instanceof Paint || view instanceof Image) {
			const idx = this._children.indexOf(view);
			if (idx >= 0) {
				this._children.splice(idx, 1);
			}
			try {
				(view as any)._canvas = undefined;
			} catch (e) {}
			return;
		}
		super._removeViewFromNativeVisualTree(view);
	}
}
