import { SVGItem } from './SVGItem';
import { createSvgElement } from '../NativeNode';

export class ClipPath extends SVGItem {
	constructor() {
		super();
		this.__domElement = createSvgElement('clipPath');
	}
}
