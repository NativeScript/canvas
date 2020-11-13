import {View, Style, CssProperty, AddChildFromBuilder, Frame, Property, path, knownFolders} from '@nativescript/core';
import {Canvas} from '../Canvas';

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
	defaultValue: undefined
})

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
	cssName: 'stroke-linecap'
})

export const strokeLinejoinProperty = new CssProperty<Style, any>({
	name: 'strokeLinejoin',
	cssName: 'stroke-linejoin'
})

export const strokeMiterlimitProperty = new CssProperty<Style,any>({
	name: 'strokeMiterlimit',
	cssName: 'stroke-miterlimit'
})
export const srcProperty = new Property<SVG, string>({
	name: 'src'
});



import {Canvg, presets} from 'canvg';
import {File} from '@nativescript/core';
import {DOMParser} from 'xmldom';

export class SVG extends View implements AddChildFromBuilder {
	_canvas: any;
	_views: any[];
	_children: Map<string, View>;
	src: string;
	_domParser: DOMParser;
	private _isReady: boolean;

	constructor() {
		super();
		this._canvas = new (Canvas as any)(false);
		this._views = [];
		this._children = new Map<string, View>();
		this._domParser = new DOMParser();
	}

	private async _handleSVG(svg) {
		if (typeof svg === "string") {
			if (svg.startsWith('~')) {
				try {
					const file = path.join(knownFolders.currentApp().path, svg.replace('~', ''));
					const r = File.fromPath(file);
					const text = await r.readText();
					const ctx = this._canvas.getContext('2d');
					console.log(text);
					const s = await Canvg.fromString(ctx, text, {
						DOMParser
					});
					console.log('done?')
					s.start();
				} catch (e) {
					console.log('failed to loade svg: ', e);
				}
			}
		}
	}

	[srcProperty.setNative](value: string) {
		if (this._isReady) {
			console.log('readdddd');
			this._handleSVG(value);
		}
	}

	createNativeView(): Object {
		this._canvas.on('ready', (args) => {
			console.log('ready');
			if (this.src) {
				setTimeout(() => {
					this._handleSVG(this.src);
				})
			} else {
				this._views.forEach((view) => {
					if (typeof view.handleValues === 'function') {
						view.handleValues();
					}
				});
			}
			this._isReady = true;
		});
		//console.log(this.parent);
		this._addView(this._canvas);
		if (this._canvas.ios) {
			return this._canvas.ios;
		}
		return this._canvas.android;
	}

	_addChildFromBuilder(name: string, value: any): void {
		value._canvas = this._canvas;
		value.parent = this;
		this._views.push(value);
		this._children.set(value.id || value._domId, value);
	}

	_getViewById(name: string) {
		if (typeof name === 'string') {
			if (name.indexOf('url') > -1) {
				name = name.replace('url', '').replace('(', '').replace(')', '');
			}

			if (name.indexOf('#')) {
				name = name.replace('#', '');
			}

			if (name.startsWith("'")) {
				name = name.replace("'", '').replace("'", '');
			}
			return (this as any)._children.get(name);
		}
		return null;
	}
}


srcProperty.register(SVG);
stopColorProperty.register(Style);
strokeWidthProperty.register(Style);
strokeProperty.register(Style);
fillProperty.register(Style);
fillOpacityProperty.register(Style);
fillRuleProperty.register(Style);
strokeLinecapProperty.register(Style);
strokeLinejoinProperty.register(Style);
strokeMiterlimitProperty.register(Style);
