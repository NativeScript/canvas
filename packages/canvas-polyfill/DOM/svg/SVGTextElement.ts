import { SVGTextPositioningElement } from './SVGTextPositioningElement';
import { Text } from '@nativescript/canvas-svg';
export class SVGTextElement extends SVGTextPositioningElement {
	constructor() {
		super('text');
		this.nativeElement = new Text() as never;
	}
}
