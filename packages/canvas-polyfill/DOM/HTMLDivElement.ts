import { HTMLElement } from './HTMLElement';
import { StackLayout } from '@nativescript/core';

export class HTMLDivElement extends HTMLElement {
	constructor() {
		super('div');
		const div = arguments[0];

		// using StackLayout for now
		if (!(div instanceof StackLayout)) {
			this.nativeElement = new StackLayout();
		} else {
			this.nativeElement = arguments[0];
		}
	}
}
