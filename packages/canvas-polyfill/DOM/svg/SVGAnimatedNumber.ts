import { getValueAndUnit } from './SVGUnits';

export class SVGAnimatedNumber {
	_baseVal: number;
	_animVal: number;
	_element;
	_key: string;
	constructor(element?, key?) {
		this._baseVal = 0;
		this._animVal = 0;
		this._element = element;
		this._key = key;
	}

	get baseVal() {
		const value = this._element?.getAttribute?.(this._key);
		if (typeof value === 'string') {
			const ret = getValueAndUnit(value);
			if (ret.unit === '%') {
				const retValue = Number(ret.value);
				if (!isNaN(retValue)) {
					return retValue / 100;
				}
			} else {
				const retValue = Number(ret.value);
				if (!isNaN(retValue)) {
					return retValue;
				}
			}
		}

		return this._baseVal;
	}

	get animVal() {
		// todo
		const value = this._element?.getAttribute?.(this._key);
		if (typeof value === 'string') {
			const ret = getValueAndUnit(value);
			if (ret.unit === '%') {
				const retValue = Number(ret.value);
				if (!isNaN(retValue)) {
					return retValue / 100;
				}
			} else {
				const retValue = Number(ret.value);
				if (!isNaN(retValue)) {
					return retValue;
				}
			}
		}
		return this._animVal;
	}
}
