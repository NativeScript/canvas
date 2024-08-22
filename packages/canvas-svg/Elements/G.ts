import { SVGItem } from './SVGItem';
import { DOMParser } from '@xmldom/xmldom';

export class G extends SVGItem {
	transform: string;
	x: any;
	y: any;
	__children = [];
	constructor() {
		super();
		this.__domElement = new DOMParser().parseFromString('<g></g>', 'image/svg+xml').documentElement;
	}

	__redraw() {
		let parent = this.parent;
		while (parent) {
			const next = parent.parent;
			if (!next) {
				(<any>parent)?.__redraw?.();
			}
			parent = next;
		}
	}

	addChild(view: SVGItem) {
		this._addView(view);
		view._attached = true;
		this.__children.push(view);
	}

	removeChild(view: SVGItem) {
		this._removeView(view);
		view._attached = false;
		this.__children = this.__children.filter((item) => item !== view);
	}
}
