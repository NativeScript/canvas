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
		if (this._key === 'href') {
			return this._element?.getAttribute(this._key) ?? this._element?.getAttribute('xlink:href') ?? this._baseVal;
		}
		return this._element?.getAttribute(this._key) ?? this._baseVal;
	}

	get animVal() {
		if (this._key === 'href') {
			return this._element?.getAttribute(this._key) ?? this._element?.getAttribute('xlink:href') ?? this._animVal;
		}
		return this._element?.getAttribute(this._key) ?? this._animVal;
	}
}
