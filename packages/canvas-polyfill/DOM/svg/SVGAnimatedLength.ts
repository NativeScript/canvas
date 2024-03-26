import { getValueAndUnit } from './SVGUnits';

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

function enumToUnit(value: number): string {
	switch (value) {
		case SVG_LENGTHTYPE_NUMBER:
			return '';
		case SVG_LENGTHTYPE_PERCENTAGE:
			return '%';
		case SVG_LENGTHTYPE_EMS:
			return 'em';
		case SVG_LENGTHTYPE_EXS:
			return 'ex';
		case SVG_LENGTHTYPE_PX:
			return 'px';
		case SVG_LENGTHTYPE_CM:
			return 'cm';
		case SVG_LENGTHTYPE_MM:
			return 'mm';
		case SVG_LENGTHTYPE_IN:
			return 'in';
		case SVG_LENGTHTYPE_PT:
			return 'pt';
		case SVG_LENGTHTYPE_PC:
			return 'pc';
		default:
			return '';
	}
}

function unitToEnum(value: string): number {
	switch (value) {
		case '':
			return SVG_LENGTHTYPE_NUMBER;
		case '%':
			return SVG_LENGTHTYPE_PERCENTAGE;
		case 'em':
			return SVG_LENGTHTYPE_EMS;
		case 'ex':
			return SVG_LENGTHTYPE_EXS;
		case 'px':
			return SVG_LENGTHTYPE_PX;
		case 'cm':
			return SVG_LENGTHTYPE_CM;
		case 'mm':
			return SVG_LENGTHTYPE_MM;
		case 'in':
			return SVG_LENGTHTYPE_IN;
		case 'pt':
			return SVG_LENGTHTYPE_PT;
		case 'pc':
			return SVG_LENGTHTYPE_PC;
		default:
			return SVG_LENGTHTYPE_UNKNOWN;
	}
}

// @ts-ignore
export class SVGLength {
	_element;
	_unitType: number = SVG_LENGTHTYPE_UNKNOWN;

	_value: number = 0;
	_valueInSpecifiedUnits: number = 0;
	_valueAsString: string = '0';
	_key: string;

	constructor(element?, key?: string) {
		this._element = element;
		this._key = key;
	}

	newValueSpecifiedUnits(unitType: number, valueInSpecifiedUnits: number) {
		let unit = '';
		let valid = false;
		switch (unitType) {
			case SVG_LENGTHTYPE_NUMBER:
				valid = true;
				break;
			case SVG_LENGTHTYPE_PERCENTAGE:
				valid = true;
				unit = '%';
				break;
			case SVG_LENGTHTYPE_PERCENTAGE:
				valid = true;
				unit = '%';
				break;
			case SVG_LENGTHTYPE_EMS:
				valid = true;
				unit = 'em';
				break;
			case SVG_LENGTHTYPE_EXS:
				valid = true;
				unit = 'ex';
				break;
			case SVG_LENGTHTYPE_PX:
				valid = true;
				unit = 'px';
				break;
			case SVG_LENGTHTYPE_CM:
				valid = true;
				unit = 'cm';
				break;
			case SVG_LENGTHTYPE_MM:
				valid = true;
				unit = 'mm';
				break;
			case SVG_LENGTHTYPE_IN:
				valid = true;
				unit = 'in';
				break;
			case SVG_LENGTHTYPE_PT:
				valid = true;
				unit = 'pt';
				break;
			case SVG_LENGTHTYPE_PC:
				valid = true;
				unit = 'pc';
				break;
		}

		if (valid) {
			this._unitType = unitType;
			this._valueInSpecifiedUnits = valueInSpecifiedUnits;
			this._valueAsString = `${valueInSpecifiedUnits}${unit}`;
			//	this._element?.setAttribute?.(this._valueAsString);
		}
	}

	convertToSpecifiedUnits(unitType: number) {
		const value = this._element?.getAttribute?.(this._key);
		const converted = getValueAndUnit(value);
		switch (unitType) {
			case SVG_LENGTHTYPE_NUMBER:
				this._valueAsString = this._value + '';
				this._unitType = unitType;
				this._element?.setAttribute?.(this._key, this._valueAsString);
				break;
			case SVG_LENGTHTYPE_PERCENTAGE:
			case SVG_LENGTHTYPE_EMS:
			case SVG_LENGTHTYPE_EXS:
			case SVG_LENGTHTYPE_PX:
			case SVG_LENGTHTYPE_CM:
			case SVG_LENGTHTYPE_MM:
			case SVG_LENGTHTYPE_IN:
			case SVG_LENGTHTYPE_PT:
			case SVG_LENGTHTYPE_PC:
				if (converted.unit === '') {
					this._valueAsString = `${converted.value}${enumToUnit(unitType)}`;
					this._element.setAttribute?.(this._key, this._valueAsString);
				} else {
					this._valueAsString = `${this._value}${enumToUnit(unitType)}`;
				}

				this._unitType = unitType;
				break;
			case SVG_LENGTHTYPE_PX:
				if (converted.unit === '') {
					this._valueAsString = `${converted.value}${enumToUnit(unitType)}`;
					this._element.setAttribute?.(this._key, this._valueAsString);
				} else {
					this._valueAsString = `${this._value}${enumToUnit(unitType)}`;
				}

				this._unitType = unitType;
				break;
		}
	}

	set unitType(value: unknown) {}

	get unitType() {
		const value = this._element.getAttribute(this._key);
		if (value) {
			const ret = getValueAndUnit(value)?.unit;
			return unitToEnum(ret);
		}
		return this._unitType;
	}

	set value(value: unknown) {}

	get value() {
		const value = this._element.getAttribute(this._key);
		if (value) {
			const ret = Number(getValueAndUnit(value)?.value);
			if (!isNaN(ret)) {
				return ret;
			}
		}
		return this._value;
	}

	set valueInSpecifiedUnits(value: unknown) {}

	get valueInSpecifiedUnits() {
		const value = this._element.getAttribute(this._key);
		if (value) {
			const ret = Number(getValueAndUnit(value)?.value);
			if (!isNaN(ret)) {
				return ret;
			}
		}
		return this._valueInSpecifiedUnits;
	}

	set valueAsString(value: unknown) {}

	get valueAsString() {
		const value = this._element.getAttribute(this._key);
		if (value) {
			return value;
		}
		return this.valueAsString;
	}

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
	_element;
	_key: string;

	constructor(element?, key?: string) {
		this._baseVal = new SVGLength(element, key);
		this._animVal = new SVGLength(element, key);
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

export const SVGAnimatedLengthEmpty = new SVGAnimatedLength();
Object.freeze(SVGAnimatedLengthEmpty);
