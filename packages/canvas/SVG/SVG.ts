import { Http, View, Style, CssProperty, AddChildFromBuilder, Frame, Property, path, knownFolders, CSSType, Application, Utils } from '@nativescript/core';

export const strokeProperty = new CssProperty<Style, any>({
	name: 'stroke',
	cssName: 'stroke',
	defaultValue: undefined,
});

export const strokeWidthProperty = new CssProperty<Style, number>({
	name: 'strokeWidth',
	cssName: 'stroke-width',
	defaultValue: 1,
});

export const fillProperty = new CssProperty<Style, any>({
	name: 'fill',
	cssName: 'fill',
	defaultValue: undefined,
});

export const fillRuleProperty = new CssProperty<Style, any>({
	name: 'fillRule',
	cssName: 'fill-rule',
	defaultValue: undefined,
});

export const fillOpacityProperty = new CssProperty<Style, any>({
	name: 'fillOpacity',
	cssName: 'fill-opacity',
	defaultValue: undefined,
});

export const stopColorProperty = new CssProperty<Style, any>({
	name: 'stopColor',
	cssName: 'stop-color',
	defaultValue: undefined,
});

export const strokeLinecapProperty = new CssProperty<Style, any>({
	name: 'strokeLinecap',
	cssName: 'stroke-linecap',
});

export const strokeLinejoinProperty = new CssProperty<Style, any>({
	name: 'strokeLinejoin',
	cssName: 'stroke-linejoin',
});

export const strokeMiterlimitProperty = new CssProperty<Style, any>({
	name: 'strokeMiterlimit',
	cssName: 'stroke-miterlimit',
});
export const srcProperty = new Property<Svg, string>({
	name: 'src',
});

declare const TNSSVG, org;
import { SVGItem } from './SVGItem';
import { DOMParser, XMLSerializer } from 'xmldom';
const initialSVG = '<svg width="auto" height="auto" xmlns="http://www.w3.org/2000/svg"></svg>';
@CSSType('Svg')
export class Svg extends View {
	public static readyEvent = 'ready';
	_dom;
	_serializer;
	_svg: any;
	src: string;
	__children = [];
	__attachedToDom = false;
	constructor() {
		super();
		if (global.isAndroid) {
			const activity = Application.android.foregroundActivity || Application.android.startActivity;
			this._svg = new org.nativescript.canvas.TNSSVG(activity);
		} else if (global.isIOS) {
			this._svg = TNSSVG.alloc().initWithFrame(CGRectZero);
			this._svg.backgroundColor = UIColor.clearColor;
		}
		this._dom = new DOMParser().parseFromString(initialSVG);
		this._serializer = new XMLSerializer();
	}

	[srcProperty.setNative](value: string) {
		if (typeof value === 'string') {
			if (value.indexOf('<svg') > -1) {
				if (global.isAndroid) {
					this._svg.setSrc(value);
				} else if (global.isIOS) {
					this._svg.src = value;
				}
			} else {
				if (value.startsWith('~')) {
					if (global.isAndroid) {
						this._svg.setSrcPath(path.join(knownFolders.currentApp().path, value.replace('~', '')));
					} else if (global.isIOS) {
						this._svg.srcPath = path.join(knownFolders.currentApp().path, value.replace('~', ''));
					}
				} else if (value.startsWith('/')) {
					if (global.isAndroid) {
						this._svg.setSrcPath(value);
					} else if (global.isIOS) {
						this._svg.srcPath = value;
					}
				} else if (value.startsWith('http')) {
					Http.getString(value)
						.then((res) => {
							if (global.isAndroid) {
								this._svg.setSrc(res);
							} else if (global.isIOS) {
								this._svg.src = res;
							}
						})
						.catch((e) => {
							console.log(e);
						});
				}
			}
		}
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
		if (this.__attachedToDom) {
			const domCopy = this._dom.valueOf();
			const width = domCopy.documentElement.getAttribute('width');
			const height = domCopy.documentElement.getAttribute('height');
			if (width === 'auto') {
				domCopy.documentElement.setAttribute('width', `${this.getMeasuredWidth()}px`);
			}
			if (height === 'auto') {
				domCopy.documentElement.setAttribute('height', `${this.getMeasuredHeight()}px`);
			}
			const serialized = this._serializer.serializeToString(domCopy);
			if (serialized !== initialSVG) {
				this.src = serialized;
			}
		}
	}

	createNativeView(): Object {
		this.__attachedToDom = true;
		return this._svg;
	}

	initNativeView() {
		super.initNativeView();
	}

	addChild(view: SVGItem) {
		this._addView(view);
		view.__attached = true;
		this.__children.push(view);
	}

	removeChild(view: SVGItem) {
		if (view.__attached) {
			this._removeView(view);
			view.__attached = false;
			this.__children = this.__children.filter((item) => item !== view);
		}
	}
}

srcProperty.register(Svg);
stopColorProperty.register(Style);
strokeWidthProperty.register(Style);
strokeProperty.register(Style);
fillProperty.register(Style);
fillOpacityProperty.register(Style);
fillRuleProperty.register(Style);
strokeLinecapProperty.register(Style);
strokeLinejoinProperty.register(Style);
strokeMiterlimitProperty.register(Style);
