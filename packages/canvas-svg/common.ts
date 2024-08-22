import { booleanConverter, CssProperty, CSSType, Property, Style, View } from '@nativescript/core';
import { DOMParser, XMLSerializer } from '@xmldom/xmldom';
export * from './Elements';
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
export const srcProperty = new Property<SVGBase, string>({
	name: 'src',
});

export const syncProperty = new Property<SVGBase, boolean>({
	name: 'sync',
	defaultValue: false,
	valueConverter: booleanConverter,
});

export const initialSVG = '<svg xmlns="http://www.w3.org/2000/svg"></svg>';

@CSSType('Svg')
export class SVGBase extends View {
	public static readyEvent = 'ready';
	__domElement: Element;
	_serializer: XMLSerializer;
	__children = [];
	_attachedToDom = false;
	src: string;
	sync: boolean;

	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString(initialSVG, 'image/svg+xml').documentElement;
		this._serializer = new XMLSerializer();
	}
}

syncProperty.register(SVGBase);
srcProperty.register(SVGBase);
stopColorProperty.register(Style);
strokeWidthProperty.register(Style);
strokeProperty.register(Style);
fillProperty.register(Style);
fillOpacityProperty.register(Style);
fillRuleProperty.register(Style);
strokeLinecapProperty.register(Style);
strokeLinejoinProperty.register(Style);
strokeMiterlimitProperty.register(Style);
