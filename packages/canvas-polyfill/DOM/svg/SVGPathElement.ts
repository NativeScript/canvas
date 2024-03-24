import { SVGGraphicsElement } from './SVGGraphicsElement';
import { Path } from '@nativescript/canvas-svg';

export class SVGPathElement extends SVGGraphicsElement {
	constructor() {
		super('path');
		this.nativeElement = new Path();
	}

}
