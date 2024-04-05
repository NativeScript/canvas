const SVG_PRESERVEASPECTRATIO_UNKNOWN = 0;
const SVG_PRESERVEASPECTRATIO_NONE = 1;
const SVG_PRESERVEASPECTRATIO_XMINYMIN = 2;
const SVG_PRESERVEASPECTRATIO_XMIDYMIN = 3;
const SVG_PRESERVEASPECTRATIO_XMAXYMIN = 4;
const SVG_PRESERVEASPECTRATIO_XMINYMID = 5;
const SVG_PRESERVEASPECTRATIO_XMIDYMID = 6;
const SVG_PRESERVEASPECTRATIO_XMAXYMID = 7;
const SVG_PRESERVEASPECTRATIO_XMINYMAX = 8;
const SVG_PRESERVEASPECTRATIO_XMIDYMAX = 9;
const SVG_PRESERVEASPECTRATIO_XMAXYMAX = 10;

const SVG_MEETORSLICE_UNKNOWN = 0;
const SVG_MEETORSLICE_MEET = 1;
const SVG_MEETORSLICE_SLICE = 2;

export class SVGPreserveAspectRatio {
	_element;
	_key: string;
	_align = SVG_PRESERVEASPECTRATIO_XMIDYMID;
	_meetOrSlice = SVG_MEETORSLICE_UNKNOWN;
	constructor(element?, key?: string) {
		this._element = element;
		this._key = key;
	}
	get align() {
		return this._align;
	}

	set align(value) {
		if (value >= 1 && value <= 10) {
			this._align = value;
			this._element?.setAttribute?.(this._key, `${value} ${this._meetOrSlice}`);
		}
	}
	get meetOrSlice() {
		return this._meetOrSlice;
	}

	set meetOrSlice(value) {
		if (value >= 1 && value <= 2) {
			this._meetOrSlice = value;
			this._element?.setAttribute?.(this._key, `${this._align} ${value}`);
		}
	}

	static SVG_MEETORSLICE_UNKNOWN = SVG_MEETORSLICE_UNKNOWN;
	static SVG_MEETORSLICE_MEET = SVG_MEETORSLICE_MEET;
	static SVG_MEETORSLICE_SLICE = SVG_MEETORSLICE_SLICE;
	static SVG_PRESERVEASPECTRATIO_UNKNOWN = SVG_PRESERVEASPECTRATIO_UNKNOWN;
	static SVG_PRESERVEASPECTRATIO_NONE = SVG_PRESERVEASPECTRATIO_NONE;
	static SVG_PRESERVEASPECTRATIO_XMINYMIN = SVG_PRESERVEASPECTRATIO_XMINYMIN;
	static SVG_PRESERVEASPECTRATIO_XMIDYMIN = SVG_PRESERVEASPECTRATIO_XMIDYMIN;
	static SVG_PRESERVEASPECTRATIO_XMAXYMIN = SVG_PRESERVEASPECTRATIO_XMAXYMIN;
	static SVG_PRESERVEASPECTRATIO_XMINYMID = SVG_PRESERVEASPECTRATIO_XMINYMID;
	static SVG_PRESERVEASPECTRATIO_XMIDYMID = SVG_PRESERVEASPECTRATIO_XMIDYMID;
	static SVG_PRESERVEASPECTRATIO_XMAXYMID = SVG_PRESERVEASPECTRATIO_XMAXYMID;
	static SVG_PRESERVEASPECTRATIO_XMINYMAX = SVG_PRESERVEASPECTRATIO_XMINYMAX;
	static SVG_PRESERVEASPECTRATIO_XMIDYMAX = SVG_PRESERVEASPECTRATIO_XMIDYMAX;
	static SVG_PRESERVEASPECTRATIO_XMAXYMAX = SVG_PRESERVEASPECTRATIO_XMAXYMAX;
}
export class SVGAnimatedPreserveAspectRatio {
	_element;
	_key: string;
	_baseVal = new SVGPreserveAspectRatio();
	_animVal = new SVGPreserveAspectRatio();

	constructor(element?, key?: string, defaultValue?: string) {
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
