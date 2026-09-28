import { SVGItem } from './SVGItem';
import { createSvgElement } from '../NativeNode';

export class Stop extends SVGItem {
	offset: any;

	constructor() {
		super();
		this.__domElement = createSvgElement('stop');
	}
}
