import { SVGElement } from './SVGElement';
import { SVGAnimatedNumber } from './SVGAnimatedNumber';

export class SVGStopElement extends SVGElement {
	_offset = new SVGAnimatedNumber(this, 'offset');

	constructor() {
		super('stop');
	}

	get offset() {
		return this._offset;
	}
}
