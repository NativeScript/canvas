import { SVGElement } from './SVGElement';
import { Circle } from '@nativescript/canvas-svg';

export class SVGCircleElement extends SVGElement {
	//__internalElement: Circle;
	constructor() {
		super('circle');
		this.nativeElement = new Circle();
	}
}
