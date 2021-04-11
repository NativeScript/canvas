import { Http, View, Style, CssProperty, AddChildFromBuilder, Frame, Property, path, knownFolders, CSSType, Screen, Application } from '@nativescript/core';

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

import { File } from '@nativescript/core';
import { layout } from '@nativescript/core/utils';
@CSSType('Svg')
export class Svg extends View {
	public static readyEvent = 'ready';
	_svg: any;
	src: string;
	constructor() {
		super();
		if (global.isAndroid) {
			const activity = Application.android.foregroundActivity || Application.android.startActivity;
			this._svg = new (com as any).github.triniwiz.canvas.TNSSVG(activity);
		} else if (global.isIOS) {
			//	this._svg = TNSSVG.new();
			//	this._svg.backgroundColor = UIColor.clearColor;
		}
	}

	[srcProperty.setNative](value: string) {
		if (typeof value === 'string') {
			if (value.indexOf('<svg') > -1) {
				if (global.isAndroid) {
					this._svg.setSrc(value);
				} else if (global.isIOS) {
					setTimeout(() => {
						this._svg.src = value;
					}, 1000);
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
								setTimeout(() => {
									console.log(!!res);
									this._svg.src = res;
								}, 1000);
							}
						})
						.catch((e) => {
							console.log(e);
						});
				}
			}
		}
	}

	#inBatch = false;
	batch(action: () => void) {}

	createNativeView(): Object {
		return this._svg;
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
