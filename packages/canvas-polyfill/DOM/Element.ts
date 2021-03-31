import { Node } from './Node';

export class Element extends Node {
	private doc: any;
	private _classList: any;
	namespaceURI: any;
	nativeElement: any;
	private _width: number;
	private _height: number;
	constructor(tagName) {
		super(tagName.toUpperCase());

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

	setAttribute() {}

	removeAttribute() {}

	setAttributeNS() {}

	removeAttributeNS() {}

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
