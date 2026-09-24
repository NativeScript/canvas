import { SVGItem } from './SVGItem';
import { createSvgElement } from '../NativeNode';

export class Pattern extends SVGItem {
	constructor() {
		super();
		this.__domElement = createSvgElement('pattern');
	}
}
