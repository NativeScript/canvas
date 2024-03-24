import { SVGAnimatedLengthEmpty } from './SVGAnimatedLength';
import { SVGGeometryElement } from './SVGGeometryElement';
import { Line } from '@nativescript/canvas-svg';

export class SVGLineElement extends SVGGeometryElement {
	constructor() {
		super('line');
		this.nativeElement = new Line();
	}

	get x1() {
		const x1 = this.getAttribute('x1');
		if (!x1) {
			return SVGAnimatedLengthEmpty;
		}
		return null;
	}

	get y1() {
		const y1 = this.getAttribute('y1');
		if (!y1) {
			return SVGAnimatedLengthEmpty;
		}
		return null;
	}

	get x2() {
		const x2 = this.getAttribute('x2');
		if (!x2) {
			return SVGAnimatedLengthEmpty;
		}
		return null;
	}

	get y2() {
		const y2 = this.getAttribute('y2');
		if (!y2) {
			return SVGAnimatedLengthEmpty;
		}
		return null;
	}
}
