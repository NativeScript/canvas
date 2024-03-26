import { SVGTextPositioningElement } from './SVGTextPositioningElement';
import { Text } from '@nativescript/canvas-svg';
export class SVGTextElement extends SVGTextPositioningElement {
	constructor() {
		super('text');
		this._nativeElement = new Text();
	}
}
