import { SVGItem } from './SVGItem';
import { createSvgElement } from '../NativeNode';

export class Symbol extends SVGItem {
	constructor() {
		super();
		this.__domElement = createSvgElement('symbol');
	}
}
