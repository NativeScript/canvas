import { SVGElement } from './SVGElement';
import { SVGAnimatedTransformList } from './SVGAnimatedTransformList';
export class SVGGraphicsElement extends SVGElement {
	_transform = new SVGAnimatedTransformList(this, 'transform');

	get transform() {
		return this._transform;
	}
}
