import { SVGGeometryElement } from './SVGGeometryElement';
import { Ellipse } from '@nativescript/canvas-svg';

export class SVGEllipseElement extends SVGGeometryElement {
	constructor() {
		super('ellipse');
		this.nativeElement = new Ellipse();
	}
}
