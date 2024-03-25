export class SVGAnimatedString {
	_baseVal: string;
	_animVal: string;
	_element;

	constructor(element?) {
		this._baseVal = '';
		this._animVal = '';
		this._element = element;
	}

	get baseVal() {
		return this._baseVal;
	}

	get animVal() {
		return this._animVal;
	}
}
