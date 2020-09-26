import {SVGItem} from "./SVGItem";
import {AddChildFromBuilder} from "@nativescript/core/ui/core/view";


export class Symbol extends SVGItem implements AddChildFromBuilder {
	_views: any[];

	constructor() {
		super();
		this._views = [];
	}


	_addChildFromBuilder(name: string, value: any): void {
		value._canvas = this._canvas;
		value.parent = this;
		this._views.push(value);
		this._appendChild(value.id || value._domId, value);
	}
}
