import {View, Style, Frame, isIOS} from '@nativescript/core';
import {Canvas} from '../Canvas';
import {CssProperty} from '@nativescript/core/ui/core/properties';
import {AddChildFromBuilder} from '@nativescript/core/ui/core/view';

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

export class SVG extends View implements AddChildFromBuilder {
	_canvas: any;
	_views: any[];
	_children: Map<string, View>;
	_isReady: boolean = false;

	constructor() {
		super();
		this._canvas = new (Canvas as any)(true);
		this._views = [];
		this._children = new Map<string, View>();
	}

	initNativeView() {
		super.initNativeView();
		this._canvas.on('ready', args => {
			console.log('ready');
			this._isReady = true;
			this._views.forEach((view) => {
				if (typeof view.handleValues === 'function') {
					view.handleValues();
				}
			});
		});

		console.log(this._canvas.parent);
		if (!this._canvas.parent) {
			Frame.topmost()._addView(this._canvas);
		}
	}

	createNativeView() {
		return isIOS ? this._canvas.ios : this._canvas.android;
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

stopColorProperty.register(Style);
strokeWidthProperty.register(Style);
strokeProperty.register(Style);
fillProperty.register(Style);
fillOpacityProperty.register(Style);
