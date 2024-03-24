import { SVGGeometryElement } from './SVGGeometryElement';
import { Polyline } from '@nativescript/canvas-svg';

export class SVGPolylineElement extends SVGGeometryElement {
	constructor() {
		super('polyline');
		this.nativeElement = new Polyline();
	}
}
