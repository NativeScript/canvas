import { EventData, LayoutBase, ViewBase, Utils, View, Screen } from '@nativescript/core';
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
	_children = [];

	_raf: any;
	_state: State = State.None;
	_isReady: boolean = false;
	_onFrameCallback?: (frame: number) => void = null;

	constructor() {
		super();
		this._canvas = new Canvas();
		this._canvas.on('ready', this._ready.bind(this));
		this._canvas.style.width = { unit: '%', value: 1 };
		this._canvas.style.height = 'auto';
	}

	createNativeView(): Object {
		if (global.isIOS) {
			return UIView.new();
		}

		if (global.isAndroid) {
			return new android.widget.LinearLayout(this._context);
		}
		return null;
	}

	initNativeView(): void {
		super.initNativeView();
		this._addView(this._canvas);
	}

	onLoaded(): void {
		super.onLoaded();
		if (this._ready) {
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
		// auto scale to screen size
		const ctx = this._canvas.getContext('2d');
		ctx.scale(Screen.mainScreen.scale, Screen.mainScreen.scale);
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

	set onFrameCallback(value: (frame: number) => void | null) {
		this._onFrameCallback = value;
	}

	_ready(args: EventData) {
		this._isReady = true;
		this._dirty();
		this._draw(null);
	}

	_draw(ts) {
		const state = this._state & State.Pending;
		if (state === State.Pending) {
			const ctx = this._canvas.getContext('2d');
			this._state = State.Invalidating;
			ctx.clearRect(0, 0, this._canvas.width as number, this._canvas.height as number);
			for (const child of this._children) {
				child.draw();
			}
			this._state = State.None;
		}
		this._onFrameCallback?.(ts);
		this._bindRaf();
	}

	_bindRaf() {
		if (!this._isReady) {
			return;
		}
		this._raf = requestAnimationFrame(this._draw.bind(this));
	}

	_dirty() {
		this._state = this._state | State.Pending;
	}

	_addViewToNativeVisualTree(view: ViewBase, atIndex?: number): boolean {
		if (view === this._canvas) {
			if (global.isIOS) {
				this.nativeView.addSubview(this._canvas.nativeView);
			}

			if (global.isAndroid) {
				this.nativeView.addView(this._canvas.nativeView);
			}
			return true;
		} else if (view instanceof Paint || view instanceof Image) {
			view._addCanvas(this._canvas);
			this._children.push(view);
		}
		return false;
	}
}
