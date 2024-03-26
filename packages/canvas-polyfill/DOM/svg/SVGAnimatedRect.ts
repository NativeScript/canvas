import { SVGRect } from './SVGUnits';
function parseRect(value: string) {
	const ret = new SVGRect();
	if (!value) {
		return ret;
	}
	const items = value.split(' ');
	if (items.length) {
		const x = Number(items[0]);
		const y = Number(items[1]);
		const width = Number(items[2]);
		const height = Number(items[3]);
		if (!isNaN(x) && !isNaN(y) && !isNaN(width) && !isNaN(height)) {
			ret.x = x;
			ret.y = y;
			ret.width = width;
			ret.height = height;
		}
	}

	return ret;
}
export class SVGAnimatedRect {
	_baseVal: SVGRect = null;
	_animVal: SVGRect = null;
	_element;
	_key: string;

	constructor(element?, key?: string) {
		this._element = element;
		this._key = key;
	}

	get baseVal() {
		const attr = this._element?.getAttribute?.(this._key);
		return parseRect(attr);
	}

	get animVal() {
		// todo
		const attr = this._element?.getAttribute?.(this._key);
		return parseRect(attr);
	}
}
