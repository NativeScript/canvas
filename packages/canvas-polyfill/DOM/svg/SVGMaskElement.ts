import { SVGElement } from './SVGElement';
export class SVGMaskElement extends SVGElement {
	get children() {
		const children = (this._xmlDom?.documentElement ?? this._xmlDom)?.children;
		console.log('SVGMaskElement', children, this._xmlDom, this._nativeElement, this.tagName, this.nodeName);
		return children;
	}
}
