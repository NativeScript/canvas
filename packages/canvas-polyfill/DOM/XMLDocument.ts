import { Document as Doc} from './Document';

import querySelector from 'query-selector';

import { DOMParser } from '@xmldom/xmldom';

export class XMLDocument extends Doc {
	private __instance: Document;
	static fromParser(instance) {
		if (instance) {
			const documentPrototype = Object.getPrototypeOf(instance);
			documentPrototype.querySelectorAll = function (selector: string) {
				return querySelector(selector, this);
			};

			const doc = new XMLDocument();
			doc.__instance = instance;
			return doc;
		}
		return null;
	}
}
