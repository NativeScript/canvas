import { SVGBase, srcProperty, syncProperty, initialSVG } from './common';
import { Http, knownFolders, path, Utils } from '@nativescript/core';
import { SVGItem } from './Elements/SVGItem';

export * from './Elements';

declare const TNSSVG;

export class Svg extends SVGBase {
	_svg;

	constructor() {
		super();
		this._svg = TNSSVG.alloc().initWithFrame(CGRectZero);
		this._svg.backgroundColor = UIColor.clearColor;
	}

	createNativeView() {
		return this._svg;
	}

	get native() {
		return this._svg;
	}


	[srcProperty.setNative](value: string) {
		if (typeof value === 'string') {
			if (value.indexOf('<svg') > -1) {
				this._svg.src = value;
			} else {
				if (value.startsWith('~')) {
					this._svg.srcPath = path.join(knownFolders.currentApp().path, value.replace('~', ''));
				} else if (value.startsWith('/')) {
					this._svg.srcPath = value;
				} else if (value.startsWith('http')) {
					Http.getFile(value)
						.then((res) => {
							this._svg.srcPath = res.path;
						})
						.catch((e) => {
							console.log(e);
						});
				}
			}
		}
	}

	[syncProperty.setNative](value: boolean) {
		this._svg.sync = value;
	}

	public onLayout(left: number, top: number, right: number, bottom: number): void {
		super.onLayout(left, top, right, bottom);
		this.__redraw();
	}

	public onMeasure(widthMeasureSpec: number, heightMeasureSpec: number) {
		const nativeView = this.nativeView;
		if (nativeView) {
			const width = Utils.layout.getMeasureSpecSize(widthMeasureSpec);
			const height = Utils.layout.getMeasureSpecSize(heightMeasureSpec);
			this.setMeasuredDimension(width, height);
		}
	}

	__redraw() {
		if (this._attachedToDom) {
			const domCopy: Document = this._dom.valueOf() as never;
			const serialized = this._serializer.serializeToString(domCopy);
			const width = domCopy.documentElement.getAttribute('width');
			const height = domCopy.documentElement.getAttribute('height');
			if (width === 'auto') {
				domCopy.documentElement.setAttribute('width', `${this.getMeasuredWidth()}px`);
			}
			if (height === 'auto') {
				domCopy.documentElement.setAttribute('height', `${this.getMeasuredHeight()}px`);
			}
			if (serialized !== initialSVG) {
				this.src = serialized;
			}
		}
	}

	onLoaded() {
		super.onLoaded();
		this._attachedToDom = true;
	}

	onUnloaded() {
		this._attachedToDom = false;
		super.onUnloaded();
	}

	addChild(view: SVGItem) {
		this._addView(view);
		view._attached = true;
		this.__children.push(view);
	}

	removeChild(view: SVGItem) {
		if (view._attached) {
			this._removeView(view);
			view._attached = false;
			this.__children = this.__children.filter((item) => item !== view);
		}
	}
}
