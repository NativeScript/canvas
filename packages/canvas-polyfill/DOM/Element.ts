import { Node } from './Node';

export class Element extends Node {
	private doc: any;
	private _classList: any;
	namespaceURI: any;
	nativeElement: any;
	private _width: number;
	private _height: number;
	_instance;
	constructor(tagName) {
		if (typeof tagName === 'object') {
			super(tagName.tagName);
			this._instance = tagName;
		} else {
			super(tagName.toUpperCase());
		}

		this.doc = {
			body: {
				innerHTML: '',
			},
		};
		this._classList = new Set();
	}

	get classList() {
		return this._classList;
	}

	get tagName() {
		return this.nodeName;
	}

	getAttribute(key) {
		return this._instance?.getAttribute?.(key) ?? null;
	}

	setAttribute(key, value) {
		this._instance?.setAttribute?.(key, value);
	}

	removeAttribute(key, value) {
		console.log(this.className, 'removeAttribute', key, value);
	}

	setAttributeNS() {}

	removeAttributeNS() {}

	get attributes(){
		return this._instance?.attributes ?? [];
	}

	get clientWidth() {
		return this.innerWidth;
	}

	get clientHeight() {
		return this.innerHeight;
	}

	get offsetWidth() {
		return this.innerWidth;
	}

	get offsetHeight() {
		return this.innerHeight;
	}

	get innerWidth() {
		return this.width;
	}

	get innerHeight() {
		return this.height;
	}

	get width() {
		return this._width;
	}

	set width(value: number) {
		this._width = value;
	}

	get height() {
		return this._height;
	}

	set height(value: number) {
		this._height = value;
	}

	get ontouchstart() {
		return {};
	}
}
