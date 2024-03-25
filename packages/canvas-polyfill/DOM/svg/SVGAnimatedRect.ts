import { SVGRect } from './SVGUnits';
function parseRect(value: string) {
	if (!value) {
		return null;
	}
	const items = value.split(' ');
	if (items.length) {
		const x = Number(items[0]);
		const y = Number(items[1]);
		const width = Number(items[2]);
		const height = Number(items[3]);
		if (!isNaN(x) && !isNaN(y) && !isNaN(width) && !isNaN(height)) {
			const ret = new SVGRect();
			ret.x = x;
			ret.y = y;
			ret.width = width;
			ret.height = height;
			return ret;
		}
		return null;
	}
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
		// if (!attr && this._key === 'viewBox') {
		// 	const ret = new SVGRect();
		// 	const width = Number(this._element?.getAttribute?.('width'));
		// 	const height = Number(this._element?.getAttribute?.('height'));
		// 	ret.width = width;
		// 	ret.height = height;
		// 	console.log('size', ret);
		// 	return ret;
		// }
		return parseRect(attr);
	}

	get animVal() {
		// todo
		const attr = this._element?.getAttribute?.(this._key);
		if (attr) {
			return parseRect(attr);
		}
		return this._animVal;
	}
}
