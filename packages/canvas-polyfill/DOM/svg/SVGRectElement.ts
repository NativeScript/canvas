import { SVGElement } from './SVGElement';
import { Rect } from '@nativescript/canvas-svg';

export class SVGRectElement extends SVGElement {
	constructor() {
		super('rect');
		this.nativeElement = new Rect();
	}
}
