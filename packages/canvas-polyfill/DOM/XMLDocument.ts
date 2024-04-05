import { Document as Doc } from './Document';

import querySelector from 'query-selector';

import { SVGSVGElement } from './svg/SVGSVGElement';

export class XMLDocument extends Doc {
	constructor() {
		super();
	}

	get documentElement() {
		if (this._documentElement) {
			return this._documentElement as never;
		}
		return this.__instance?.documentElement as never;
	}

	//@ts-ignore
	private __instance: Document;
	static fromParser(instance) {
		if (instance) {
			const documentPrototype = Object.getPrototypeOf(instance);
			documentPrototype.querySelectorAll = function (selector: string) {
				return querySelector(selector, this);
			};

			const doc = new XMLDocument() as any;

			doc.body = null;
			doc._documentElement = null;
			doc.head = null;

			doc.__instance = instance;
			const documentElement = instance?.documentElement;

			if (documentElement?.nodeName === 'svg' && documentElement?.namespaceURI === 'http://www.w3.org/2000/svg') {
				const svg = new SVGSVGElement();
				svg.nativeElement.__domElement = instance.documentElement;
				(<any>doc)._documentElement = svg;
			}
			return doc;
		}
		return null;
	}
}
