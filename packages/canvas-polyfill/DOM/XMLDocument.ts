import { Document } from './Document';

import querySelector from 'query-selector';

export class XMLDocument extends Document {
	_instance;
	static fromParser(instance) {
		if (instance) {
			const documentPrototype = Object.getPrototypeOf(instance);

			documentPrototype.querySelectorAll = function querySelectorAll(selector) {
				return querySelector(selector, this);
			};

			const doc = new XMLDocument();
			doc._instance = instance;
			return doc;
		}
		return null;
	}

	getElementsByTagName(tagname) {
		return this._instance.getElementsByTagName(tagname);
	}

	getElementById(id) {
		return this._instance.getElementById(id);
	}

	//@ts-ignore
	get documentElement() {
		return this._instance.documentElement;
	}

	//@ts-ignore
	set documentElement(value) {}


	getAttribute(key) {
		return this._instance?.getAttribute?.(key) ?? null;
	}

	setAttribute(key, value) {
		this._instance?.setAttribute?.(key, value);
		console.log(this.className, 'setAttribute', key, value);
	}
}
