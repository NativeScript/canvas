import { SVGElement } from './SVGElement';
import { SVGAnimatedNumber } from './SVGAnimatedNumber';
import { Stop } from '@nativescript/canvas-svg';

export class SVGStopElement extends SVGElement {
	_offset = new SVGAnimatedNumber(this, 'offset');

	constructor() {
		super('stop');
		this.nativeElement = new Stop() as never;
	}

	get offset() {
		return this._offset;
	}
}
