import { SVGItem } from './SVGItem';
import { createSvgElement } from '../NativeNode';

export class Use extends SVGItem {
	constructor() {
		super();
		this.__domElement = createSvgElement('use');
	}
}
