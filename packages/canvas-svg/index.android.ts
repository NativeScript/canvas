import { initialSVG, SVGBase, srcProperty, syncProperty } from './common';
import { Application, Http, knownFolders, path, Utils } from '@nativescript/core';
import { SVGItem } from './Elements/SVGItem';

export * from './Elements';

declare const org;

export class Svg extends SVGBase {
	_svg;

	constructor() {
		super();
		const context = Application.android.foregroundActivity || Application.android.startActivity || Utils.android.getApplicationContext();
		this._svg = new org.nativescript.canvas.svg.NSCSVG(context);
		this.on('layoutChanged', (args) => {
			this.__redraw();
		});
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
					Http.getFile(value)
						.then((res) => {
							this._svg.setSrcPath(res.path);
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

	__redraw() {
		if (this._attachedToDom) {
			const domCopy: Document = this._dom.valueOf() as never;
			const width = domCopy.documentElement.getAttribute('width');
			const height = domCopy.documentElement.getAttribute('height');
			const viewBox = domCopy.documentElement.getAttribute('viewBox');
			if (width === 'auto') {
				domCopy.documentElement.setAttribute('width', `${this.getMeasuredWidth()}`);
			}
			if (height === 'auto') {
				domCopy.documentElement.setAttribute('height', `${this.getMeasuredHeight()}`);
			}

			if (!viewBox) {
				const width = domCopy.documentElement.getAttribute('width');
				const height = domCopy.documentElement.getAttribute('height');
				domCopy.documentElement.setAttribute('viewBox', `0 0 ${width} ${height}`);
			}

			const serialized = this._serializer.serializeToString(domCopy);

			if (serialized !== initialSVG) {
				this.src = serialized;
			}
		}
	}

	onLoaded() {
		super.onLoaded();
		this._attachedToDom = true;
		console.log('onLoaded');
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
