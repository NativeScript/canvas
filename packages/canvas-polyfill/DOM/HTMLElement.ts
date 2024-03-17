import { Element } from './Element';

export class HTMLElement extends Element {
	constructor(tagName: string = '') {
		super(tagName ?? '');
	}
}
