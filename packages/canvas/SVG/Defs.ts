import {AddChildFromBuilder} from "@nativescript/core";
import {SVGItem} from "./SVGItem";

export class Defs extends SVGItem implements AddChildFromBuilder {
	_views: any[];

	constructor() {
		super();
		this._views = [];
	}

	_addChildFromBuilder(name: string, value: any): void {
		value.parent = this;
		value._canvas = this._canvas;
		this._views.push(value);
		this._appendChild(value.id || value._domId, value);
	}
}
