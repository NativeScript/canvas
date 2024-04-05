import { getPixelsPerInchForCurrentDevice } from '../../utils';
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

const INCHES_PER_CENTIMETRE = 0.393700787;
const INCHES_PER_MILLIMETER = 0.0393701;

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
	_unitType: number = SVG_LENGTHTYPE_NUMBER;

	_value: number = 0;
	_valueInSpecifiedUnits: number = 0;
	_key: string;

	constructor(element?, key?: string, defaultValue?: string) {
		this._element = element;
		this._key = key;
		if (defaultValue) {
			this.valueAsString = defaultValue;
		}
	}

	static emptyPercentage() {
		const length = new SVGLength();
		length._unitType = SVG_LENGTHTYPE_PERCENTAGE;
		return length;
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
			this._element?.setAttribute?.(this._key, this.valueAsString);
		}
	}

	convertToSpecifiedUnits(unitType: number) {
		/*
		VM762:1 Uncaught DOMException: Failed to execute 'convertToSpecifiedUnits' on 'SVGLength': Could not resolve relative length.
    at <anonymous>:1:3 when converting ftom % 
		*/
		if (unitType === this._unitType) {
			return;
		}
		const value = this._element?.getAttribute?.(this._key);
		const converted = getValueAndUnit(value);
		let valueAsInches: number;

		switch (this._unitType) {
			case SVG_LENGTHTYPE_NUMBER:
				this._unitType = unitType;
				this._element?.setAttribute?.(this._key, this.valueAsString);
				break;
			case SVG_LENGTHTYPE_PERCENTAGE:
				{
					if (unitType === SVG_LENGTHTYPE_NUMBER) {
						this._unitType = unitType;
						this._value = this._value / 100;
						this._valueInSpecifiedUnits = this._value;
					}
				}
				break;

			case SVG_LENGTHTYPE_EMS:
			case SVG_LENGTHTYPE_EXS:
				break;

			case SVG_LENGTHTYPE_CM:
			case SVG_LENGTHTYPE_MM:
			case SVG_LENGTHTYPE_IN:
			case SVG_LENGTHTYPE_PT:
			case SVG_LENGTHTYPE_PC:
				switch (this._unitType) {
					case SVG_LENGTHTYPE_CM:
						{
							valueAsInches = this._value * INCHES_PER_CENTIMETRE;
						}
						break;
					case SVG_LENGTHTYPE_MM:
						{
							valueAsInches = this._value * INCHES_PER_MILLIMETER;
						}
						break;
					case SVG_LENGTHTYPE_PT:
						{
							valueAsInches = this._value / 72.0;
						}
						break;
					case SVG_LENGTHTYPE_PC:
						{
							valueAsInches = (this._value * 12.0) / 72.0;
						}
						break;
					case SVG_LENGTHTYPE_IN:
						{
							valueAsInches = this._value;
						}
						break;
				}

				switch (unitType) {
					case SVG_LENGTHTYPE_CM:
						{
							this._value = valueAsInches / INCHES_PER_CENTIMETRE;
							this._unitType = unitType;
							this._element?.setAttribute?.(this._key, this.valueAsString);
						}
						break;
					case SVG_LENGTHTYPE_MM:
						{
							this._value = valueAsInches / INCHES_PER_MILLIMETER;
							this._unitType = unitType;
							this._element?.setAttribute?.(this._key, this.valueAsString);
						}
						break;
					case SVG_LENGTHTYPE_PT:
						{
							this._value = valueAsInches * 72.0;
							this._unitType = unitType;
							this._element?.setAttribute?.(this._key, this.valueAsString);
						}
						break;
					case SVG_LENGTHTYPE_PC:
						{
							this._value = (valueAsInches / 12.0) * 72.0;
							this._unitType = unitType;
							this._element?.setAttribute?.(this._key, this.valueAsString);
						}
						break;
					case SVG_LENGTHTYPE_PX:
						{
							this._value = valueAsInches * getPixelsPerInchForCurrentDevice();
							this._unitType = unitType;
							this._element?.setAttribute?.(this._key, this.valueAsString);
						}
						break;

					default:
						break;
				}

				break;
			case SVG_LENGTHTYPE_PX:
				switch (unitType) {
					case SVG_LENGTHTYPE_NUMBER:
						this._unitType = unitType;
						this._element?.setAttribute?.(this._key, this.valueAsString);
						break;
				}
				if (converted.unit === '') {
					//	this._valueAsString = `${converted.value}${enumToUnit(unitType)}`;
					//this._element.setAttribute?.(this._key, this._valueAsString);
				} else {
					//this._valueAsString = `${this._value}${enumToUnit(unitType)}`;
				}

				//this._unitType = unitType;
				break;
		}

		//	console.log('convertToSpecifiedUnits', this._element.nodeName, this.valueAsString, this._key);
	}

	set unitType(value: number) {}

	get unitType() {
		const value = this._element.getAttribute(this._key);
		if (value) {
			const ret = getValueAndUnit(value)?.unit;
			return unitToEnum(ret);
		}
		return this._unitType;
	}

	set value(value: number) {
		//this._value =
	}

	get value() {
		// throw if unit is relative w/o a parent
		const value = this._element.getAttribute(this._key);
		if (value) {
			const ret = Number(getValueAndUnit(value)?.value);
			if (!isNaN(ret)) {
				return ret;
			}
		}
		return this._value;
	}

	set valueInSpecifiedUnits(value: number) {
		if (typeof value === 'number') {
			this._valueInSpecifiedUnits = value;
		}
	}

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

	set valueAsString(value: string) {
		const valueAndUnit = getValueAndUnit(value);
		const unit = unitToEnum(valueAndUnit.unit);

		if (unit !== SVG_LENGTHTYPE_UNKNOWN) {
			this._unitType = unit;
			this._value = parseFloat(valueAndUnit.value);
			this._valueInSpecifiedUnits = this._value;
			this._element.setAttribute(this._key, value);
		} else {
			throw Error('An invalid or illegal string was specified');
		}
	}

	get valueAsString() {
		const value = this._element.getAttribute(this._key);
		if (value) {
			return value;
		}
		return `${this._valueInSpecifiedUnits}${enumToUnit(this._unitType)}`;
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
	_defaultValue: string;

	constructor(element?, key?: string, defaultValue: string = '0') {
		this._element = element;
		this._key = key;
		this._defaultValue = defaultValue ?? '0';
	}
	get baseVal() {
		if (this._baseVal === undefined) {
			this._baseVal = new SVGLength(this._element, this._key);
			const value = this._element?.getAttribute?.(this._key);
			this._baseVal.valueAsString = typeof value === 'string' && value.length ? value : this._defaultValue;
		}
		return this._baseVal;
	}

	get animVal() {
		if (this._animVal === undefined) {
			this._animVal = new SVGLength(this._element, this._key);
			const value = this._element?.getAttribute?.(this._key);
			this._baseVal.valueAsString = typeof value === 'string' && value.length ? value : this._defaultValue;
		}
		return this._animVal;
	}

	static emptyPercentage(element?, key?: string) {
		const length = new SVGAnimatedLength(element, key);
		length._baseVal._unitType = SVG_LENGTHTYPE_PERCENTAGE;
		length._animVal._unitType = SVG_LENGTHTYPE_PERCENTAGE;
		return length;
	}
}

export const SVGAnimatedLengthEmpty = new SVGAnimatedLength();
Object.freeze(SVGAnimatedLengthEmpty);
