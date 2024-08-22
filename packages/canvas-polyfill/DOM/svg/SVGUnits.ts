const SVG_ANGLETYPE_UNKNOWN = 0;
const SVG_ANGLETYPE_UNSPECIFIED = 1;
const SVG_ANGLETYPE_DEG = 2;
const SVG_ANGLETYPE_RAD = 3;
const SVG_ANGLETYPE_GRAD = 4;

function enumToUnit(value: number): string {
	switch (value) {
		case SVG_ANGLETYPE_DEG:
			return 'deg';
		case SVG_ANGLETYPE_RAD:
			return 'rad';
		case SVG_ANGLETYPE_GRAD:
			return 'grad';
		default:
			return '';
	}
}

const ex = /([ 0-9]*[0-9]+(?:\.|,)?[0-9]*)[ ]*(.*)/;
export function getValueAndUnit(value: string) {
	const ret = ex.exec(value);
	if (ret) {
		return { value: ret[1], unit: ret[2] };
	}
	return { value: '', unit: '' };
}

export class SVGAngle {
	_element;
	_key: string;
	_unitType: number = SVG_ANGLETYPE_UNKNOWN;
	_value: number = 0;
	_valueInSpecifiedUnits: number = 0;
	_valueAsString: string = '0';

	constructor(element?, key?: string) {
		this._element = element;
	}

	newValueSpecifiedUnits(unitType: number, valueInSpecifiedUnits: number) {
		let unit = '';
		let valid = false;
		switch (unitType) {
			case SVG_ANGLETYPE_UNSPECIFIED:
				valid = true;
				break;
			case SVG_ANGLETYPE_DEG:
				valid = true;
				unit = 'deg';
				this._value = valueInSpecifiedUnits;
				break;
			case SVG_ANGLETYPE_RAD:
				valid = true;
				unit = 'rad';
				this._value = valueInSpecifiedUnits * (180 / Math.PI);
				break;
			case SVG_ANGLETYPE_GRAD:
				valid = true;
				unit = 'grad';
				this._value = valueInSpecifiedUnits * (180 / 200);
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
		switch (unitType) {
			case SVG_ANGLETYPE_DEG:
			case SVG_ANGLETYPE_RAD:
			case SVG_ANGLETYPE_GRAD:
				this._valueAsString = `${this._value}${enumToUnit(unitType)}`;
				this._unitType = unitType;
				//this._element?.setAttribute?.(this._valueAsString);
				break;
		}
	}

	set unitType(value: unknown) {}

	get unitType() {
		return this._unitType;
	}

	set value(value: unknown) {}

	get value() {
		return this._value;
	}

	set valueInSpecifiedUnits(value: unknown) {}

	get valueInSpecifiedUnits() {
		return this._valueInSpecifiedUnits;
	}

	set valueAsString(value: unknown) {}

	get valueAsString() {
		return this.valueAsString;
	}

	static SVG_ANGLETYPE_UNKNOWN = SVG_ANGLETYPE_UNKNOWN;

	static SVG_ANGLETYPE_UNSPECIFIED = SVG_ANGLETYPE_UNSPECIFIED;

	static SVG_ANGLETYPE_DEG = SVG_ANGLETYPE_DEG;

	static SVG_ANGLETYPE_RAD = SVG_ANGLETYPE_RAD;

	static SVG_ANGLETYPE_GRAD = SVG_ANGLETYPE_GRAD;
}

export class SVGPoint {
	x: number = 0;
	y: number = 0;
}

export class SVGNumber {
	value: number = 0;
}

export class SVGRect {
	x: number = 0;
	y: number = 0;
	width: number = 0;
	height: number = 0;
}

export class SVGPointList extends Array {
	clear() {
		this.splice(0);
	}

	getItem(index: number) {
		return this[index];
	}

	insertItemBefore(newItem: SVGPoint, index: number) {
		this.splice(index - 1, 0, newItem);
	}

	replaceItem(newItem: SVGPoint, index: number) {
		this[index] = newItem;
	}

	removeItem(index: number) {
		this.splice(index, 1);
	}

	appendItem(newItem: SVGPoint) {
		this.push(newItem);
	}

	consolidate() {}

	get numberOfItems() {
		return this.length;
	}

	static fromString(points: string) {
		const items = points?.split?.(/[ ,]/g).map((p) => parseInt(p, 10)) ?? [];
		const count = items.length / 2;
		const ret = new SVGPointList(count);
		let index = 0;
		for (let i = 0; i < count; i++) {
			const point = new SVGPoint();
			point.x = items[index];
			point.y = items[index + 1];
			ret.push(point);
			index += 2;
		}
		return ret;
	}
}
