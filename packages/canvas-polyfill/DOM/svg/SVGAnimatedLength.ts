const SVG_LENGTHTYPE_UNKNOWN = 0;
const SVG_LENGTHTYPE_NUMBER = 1;
const SVG_LENGTHTYPE_PERCENTAGE = 2;
const SVG_LENGTHTYPE_EMS = 3;
const SVG_LENGTHTYPE_EXS = 4;
const SVG_LENGTHTYPE_PX = 5;
const SVG_LENGTHTYPE_CM = 6;
const SVG_LENGTHTYPE_MM = 7;
const SVG_LENGTHTYPE_IN = 8;
const SVG_LENGTHTYPE_PT = 9;
const SVG_LENGTHTYPE_PC = 10;

// @ts-ignore
export class SVGLength {
	_unitType: number = SVG_LENGTHTYPE_UNKNOWN;

	_value: number = 0;
	_valueInSpecifiedUnits: number = 0;
	_valueAsString: number = 0;

	newValueSpecifiedUnits(unitType: number, valueInSpecifiedUnits: number) {}

	convertToSpecifiedUnits(unitType: number) {}

	static SVG_LENGTHTYPE_UNKNOWN = SVG_LENGTHTYPE_UNKNOWN;

	static SVG_LENGTHTYPE_NUMBER = SVG_LENGTHTYPE_NUMBER;

	static SVG_LENGTHTYPE_PERCENTAGE = SVG_LENGTHTYPE_PERCENTAGE;

	static SVG_LENGTHTYPE_EMS = SVG_LENGTHTYPE_EMS;

	static SVG_LENGTHTYPE_EXS = SVG_LENGTHTYPE_EXS;

	static SVG_LENGTHTYPE_PX = SVG_LENGTHTYPE_PX;

	static SVG_LENGTHTYPE_CM = SVG_LENGTHTYPE_CM;

	static SVG_LENGTHTYPE_MM = SVG_LENGTHTYPE_MM;

	static SVG_LENGTHTYPE_IN = SVG_LENGTHTYPE_IN;

	static SVG_LENGTHTYPE_PT = SVG_LENGTHTYPE_PT;

	static SVG_LENGTHTYPE_PC = SVG_LENGTHTYPE_PC;
}

// @ts-ignore
export class SVGAnimatedLength {
	_baseVal: SVGLength;
	_animVal: SVGLength;

	constructor() {
		this._baseVal = new SVGLength();
		this._animVal = new SVGLength();
	}

	get baseVal() {
		return this._baseVal;
	}

	get animVal() {
		return this._animVal;
	}
}

export const SVGAnimatedLengthEmpty = new SVGAnimatedLength();
Object.freeze(SVGAnimatedLengthEmpty);
