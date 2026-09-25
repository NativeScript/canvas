import { SVGTextPositioningElement } from './SVGTextPositioningElement';
import { TSpan } from '@nativescript/canvas-svg';
export class SVGTSpanElement extends SVGTextPositioningElement {
	constructor() {
		super('tspan');
		this.nativeElement = new TSpan() as never;
	}
}
