import { SVGElement } from './SVGElement';
import { G } from '@nativescript/canvas-svg';

export class SVGGElement extends SVGElement {
	constructor() {
		super('g');
		this.nativeElement = new G();
	}
}
