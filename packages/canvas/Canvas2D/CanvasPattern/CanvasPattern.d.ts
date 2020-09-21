import {CanvasPatternBase} from './common';
import {DOMMatrix} from '../DOMMatrix/DOMMatrix';

export declare class CanvasPattern extends CanvasPatternBase {
	constructor(instance: any);

	setTransform(matrix: DOMMatrix): void;
}
