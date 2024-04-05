import { SVGElement } from './SVGElement';
export class SVGMaskElement extends SVGElement {
	get children() {
		const children = this.__domElement.children;
		return children;
	}
}
