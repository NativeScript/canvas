import { EventData, LayoutBase, ViewBase } from '@nativescript/core';
import { Canvas } from '../Canvas';
import { Image } from './Image';
import { Paint } from './Paint';
export class Dom extends LayoutBase {
	_canvas;
	_children = [];

	constructor() {
		super();
		this._canvas = new Canvas();
		this._canvas.on('ready', this._ready.bind(this));
	}

	createNativeView(): Object {
		return new android.widget.LinearLayout(this._context);
	}

	initNativeView(): void {
		super.initNativeView();
		this._addView(this._canvas);
	}

	_ready(args: EventData) {
		console.time('_ready');
		for (const child of this._children) {
			child.draw();
		}

		// this._children.forEach((child) => {
		// 	child.draw();
		// });
		console.timeEnd('_ready');
	}
	_addViewToNativeVisualTree(view: ViewBase, atIndex?: number): boolean {
		if (view === this._canvas) {
			this.nativeView.addView(this._canvas.nativeView);
			return true;
		} else if (view instanceof Paint || view instanceof Image) {
			view._addCanvas(this._canvas);
			this._children.push(view);
		}
		return false;
	}
}
