import { SVGGraphicsElement } from './SVGGraphicsElement';
import { G } from '@nativescript/canvas-svg';

export class SVGGElement extends SVGGraphicsElement {
	constructor() {
		super('g');
		this.nativeElement = new G() as never;
	}
}
