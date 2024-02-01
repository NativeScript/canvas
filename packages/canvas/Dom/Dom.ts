import { EventData, LayoutBase, ViewBase, Utils, View } from '@nativescript/core';
import { Canvas } from '../Canvas';
import { Image } from './Image';
import { Paint } from './Paint';
export class Dom extends LayoutBase {
	_canvas: Canvas;
	_children = [];

	constructor() {
		super();
		this._canvas = new Canvas();
		this._canvas.on('ready', this._ready.bind(this));
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

	public onLayout(left: number, top: number, right: number, bottom: number): void {
		super.onLayout(left, top, right, bottom);
		View.layoutChild(this, this._canvas, left, top, right, bottom);
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

	_ready(args: EventData) {
		for (const child of this._children) {
			child.draw();
		}
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
