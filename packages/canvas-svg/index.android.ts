import { initialSVG, SVGBase, srcProperty, syncProperty } from './common';
import { Application, Http, knownFolders, path, Utils } from '@nativescript/core';
import { SVGItem } from './Elements/SVGItem';

declare const org;

export class SVG extends SVGBase {
	_svg;

	constructor() {
		super();
		const context = Application.android.foregroundActivity || Application.android.startActivity || Utils.android.getApplicationContext();
		this._svg = new org.nativescript.canvas.NSCSVG(context);
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
				this._svg.setSrc(value);
			} else {
				if (value.startsWith('~')) {
					this._svg.setSrcPath(path.join(knownFolders.currentApp().path, value.replace('~', '')));
				} else if (value.startsWith('/')) {
					this._svg.setSrcPath(value);
				} else if (value.startsWith('http')) {
					Http.getString(value)
						.then((res) => {
							this._svg.setSrc(res);
						})
						.catch((e) => {
							console.log(e);
						});
				}
			}
		}
	}

	[syncProperty.setNative](value: boolean) {
		this._svg.setSync(value);
	}

	public onLayout(left: number, top: number, right: number, bottom: number): void {
		super.onLayout(left, top, right, bottom);
		this.__redraw();
	}

// this.__internalElement._dom.documentElement.setAttribute('id', value);
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
