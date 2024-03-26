import { SVGElement } from './SVGElement';
export class SVGMaskElement extends SVGElement {
	get children() {
		const children = (this._xmlDom?.documentElement ?? this._xmlDom)?.children;
		return children;
	}
}
