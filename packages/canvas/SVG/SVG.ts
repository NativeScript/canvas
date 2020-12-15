import { View, Style, CssProperty, AddChildFromBuilder, Frame, Property, path, knownFolders, CSSType,Screen } from '@nativescript/core';
import { Canvas } from '../Canvas';

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

export const strokeMiterlimitProperty = new CssProperty<Style, any>({
	name: 'strokeMiterlimit',
	cssName: 'stroke-miterlimit'
})
export const srcProperty = new Property<Svg, string>({
	name: 'src'
});



import { File } from '@nativescript/core';
import { DOMParser } from 'xmldom';
import { layout } from '@nativescript/core/utils';

@CSSType('Svg')
export class Svg extends View implements AddChildFromBuilder {
	public static readyEvent = 'ready';
	_canvas: any;
	_views: any[];
	_children: Map<string, View>;
	#viewPostion: Map<number, string>;
	src: string;
	_domParser: DOMParser;
	private _isReady: boolean;

	constructor() {
		super();
		this._canvas = new (Canvas as any)(true);
		this.#viewPostion = new Map();
		this._views = [];
		this._children = new Map<string, View>();
		this._domParser = new DOMParser();
	}



	[srcProperty.setNative](value: string) {
		if (this._isReady) {

		}
	}

	_refresh() {
		if (this.#inBatch) { return; }
		this._views.forEach((view) => {
			if (typeof view.handleValues === 'function') {
				view.handleValues(this._canvas);
			}
		})
	}
	_forceRedraw() {
		const ctx = this._canvas.getContext('2d');
		ctx.clearRect(0, 0, ctx.width, ctx.height);
		this._views.forEach((view) => {
			if (typeof view.handleValues === 'function') {
				view.handleValues(this._canvas);
			}
		});
	}
	#inBatch = false;
	batch(action: () => void) {
		this.#inBatch = true;
		action();
		this.#inBatch = false;
		this._refresh();
	}
	createNativeView(): Object {
		this._canvas.on('ready', (args) => {
			if (global.isAndroid) {
				if (this.src) {

				} else {
					this._refresh()
				}
				this._isReady = true;
				this.notify({ eventName: 'ready', object: this })
			} else {
				setTimeout(() => {
					if (this.src) {

					} else {
						this._refresh()
					}
					this._isReady = true;
					this.notify({ eventName: 'ready', object: this })
				})
			}
		});


		this._addView(this._canvas);
		if (this._canvas.ios) {
			return this._canvas.ios;
		}
		return this._canvas.android;
	}

	// This method won't be called in Android because we use the native android layout.
	public onMeasure(widthMeasureSpec: number, heightMeasureSpec: number): void {
		const result = View.measureChild(this, this._canvas, widthMeasureSpec, heightMeasureSpec);

		const width = layout.getMeasureSpecSize(widthMeasureSpec);
		const widthMode = layout.getMeasureSpecMode(widthMeasureSpec);

		const height = layout.getMeasureSpecSize(heightMeasureSpec);
		const heightMode = layout.getMeasureSpecMode(heightMeasureSpec);

		const measureWidth = Math.max(result.measuredWidth, this.effectiveMinWidth);
		const measureHeight = Math.max(result.measuredHeight, this.effectiveMinHeight);

		const widthAndState = View.resolveSizeAndState(measureWidth, width, widthMode, 0);
		const heightAndState = View.resolveSizeAndState(measureHeight, height, heightMode, 0);

		this.setMeasuredDimension(widthAndState, heightAndState);
	}

	public onLayout(left: number, top: number, right: number, bottom: number): void {
		View.layoutChild(this, this._canvas, 0, 0, right - left, bottom - top);
	}

	eachChildView(callback: (child: View) => boolean) {
		this._children.forEach((view, key) => {
			callback(view);
		});
	}


	public _addChildFromBuilder(name: string, value: any) {
		if (value instanceof View) {
			this.addChild(value);
		}
	}

	getChildrenCount(): number {
		return this._children.size;
	}

	// overrides the base property.
	get _childrenCount(): number {
		return this._children.size;
	}

	getChildAt(index: number): View {
		return this._views[index];
	}

	getChildIndex(child: View): number {
		return this._views.indexOf(child);
	}

	public getChildById(id: string) {
		return this._getViewById(id);
	}

	public _registerLayoutChild(child: View) {
		//Overridden
	}

	public _unregisterLayoutChild(child: View) {
		//Overridden
	}

	public addChild(child: View): void {
		this._views.push(child);
		this._children.set(`${child.id || child._domId}`, child);
		this._addView(child);
		this._registerLayoutChild(child);
		this._refresh();
	}

	public insertChild(child: View, atIndex: number): void {
		this._views.splice(atIndex, 0, child);
		this._children.set(`${child.id || child._domId}`, child);
		this._addView(child, atIndex);
		this._registerLayoutChild(child);
		this._refresh();
	}

	public removeChild(child: View): void {
		this._removeView(child);
		// TODO: consider caching the index on the child.
		const index = this._views.indexOf(child);
		this._views.splice(index, 1);
		this._children.delete(`${child.id || child._domId}`)
		this._unregisterLayoutChild(child);
		if (!this.#isRemoving) {
			this._refresh();
		}
	}

	#isRemoving: boolean = false;
	public removeChildren(): void {
		this.#isRemoving = true;
		while (this.getChildrenCount() !== 0) {
			this.removeChild(this._views[this.getChildrenCount() - 1]);
		}
		if (this.#isRemoving) {
			this._refresh();
		}
		this.#isRemoving = false;
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
