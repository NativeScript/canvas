import { SVGAnimatedLengthEmpty } from './SVGAnimatedLength';
import { SVGGeometryElement } from './SVGGeometryElement';
import { Rect } from '@nativescript/canvas-svg';

export class SVGRectElement extends SVGGeometryElement {
	constructor() {
		super('rect');
		this.nativeElement = new Rect();
	}

	get x() {
		const x = this.getAttribute('x');
		if (!x) {
			return SVGAnimatedLengthEmpty;
		}
		return null;
	}

	get y() {
		const y = this.getAttribute('y');
		if (!y) {
			return SVGAnimatedLengthEmpty;
		}
		return null;
	}

	// @ts-ignore
	get width() {
		const width = this.getAttribute('width');
		if (!width) {
			return SVGAnimatedLengthEmpty;
		}
		return null;
	}

	// @ts-ignore
	get height() {
		const height = this.getAttribute('height');
		if (!height) {
			return SVGAnimatedLengthEmpty;
		}
		return null;
	}

	get rx() {
		const rx = this.getAttribute('rx');
		if (!rx) {
			return SVGAnimatedLengthEmpty;
		}
		return null;
	}

	get ry() {
		const ry = this.getAttribute('ry');
		if (!ry) {
			return SVGAnimatedLengthEmpty;
		}
		return null;
	}
}
