import { SVGGeometryElement } from './SVGGeometryElement';
import { Polygon } from '@nativescript/canvas-svg';

export class SVGPolygonElement extends SVGGeometryElement {
	constructor() {
		super('polygon');
		this.nativeElement = new Polygon();
	}
}
