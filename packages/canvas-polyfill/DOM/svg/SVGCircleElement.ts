import { SVGGeometryElement } from './SVGGeometryElement';
import { Circle } from '@nativescript/canvas-svg';

export class SVGCircleElement extends SVGGeometryElement {
	//__internalElement: Circle;
	constructor() {
		super('circle');
		this.nativeElement = new Circle();
	}
}
