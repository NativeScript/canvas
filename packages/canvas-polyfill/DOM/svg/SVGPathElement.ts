import { SVGElement } from './SVGElement';
import { Path } from '@nativescript/canvas-svg';

export class SVGPathElement extends SVGElement {
	constructor() {
		super('path');
		this.nativeElement = new Path();
	}

}
