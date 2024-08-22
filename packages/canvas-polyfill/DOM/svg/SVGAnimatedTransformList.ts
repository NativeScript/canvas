import { SVGTransformList } from './SVGTransform';
export class SVGAnimatedTransformList {
	_baseVal = new SVGTransformList();
	_animVal = new SVGTransformList();
	_element;
	_key: string;

	constructor(element?, key?) {
		this._element = element;
		this._key = key;
	}

	get baseVal() {
		return this._baseVal;
	}

	get animVal() {
		return this._animVal;
	}
}
