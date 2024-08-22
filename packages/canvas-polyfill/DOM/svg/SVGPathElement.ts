import { SVGGeometryElement } from './SVGGeometryElement';
import { Path } from '@nativescript/canvas-svg';

export class SVGPathElement extends SVGGeometryElement {
	constructor() {
		super('path');
		this.nativeElement = new Path() as never;
	}
}
