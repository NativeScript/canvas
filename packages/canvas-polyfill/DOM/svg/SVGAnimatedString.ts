export class SVGAnimatedString {
	_baseVal: string;
	_animVal: string;
	_element;
	_key: string;

	constructor(element?, key?: string) {
		this._baseVal = '';
		this._animVal = '';
		this._element = element;
		this._key = key;
	}

	get baseVal() {
		return this._element?.getAttribute(this._key) ?? this._baseVal;
	}

	get animVal() {
		return this._element?.getAttribute(this._key) ?? this._animVal;
	}
}
