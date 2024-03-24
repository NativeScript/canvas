import { Document as Doc } from './Document';

import querySelector from 'query-selector';

import { DOMParser } from '@xmldom/xmldom';

import { SVGSVGElement } from './svg/SVGSVGElement';

export class XMLDocument extends Doc {
	//@ts-ignore
	__instance: Document;
	static fromParser(instance) {
		if (instance) {
			const documentPrototype = Object.getPrototypeOf(instance);
			documentPrototype.querySelectorAll = function (selector: string) {
				return querySelector(selector, this);
			};

			const doc = new XMLDocument();
			doc.__instance = instance;
			const documentElement = instance?.documentElement;
			if (documentElement?.nodeName === 'svg' && documentElement?.namespaceURI === 'http://www.w3.org/2000/svg') {
				const svg = new SVGSVGElement('svg') as any;
				svg.__instance = instance.documentElement;
				(<any>doc)._documentElement = svg;
			}
			return doc;
		}
		return null;
	}
}
