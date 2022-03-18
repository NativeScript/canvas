import { Document } from './Document';

export class XMLDocument extends Document {
	#instance;
	static fromParser(instance) {
		if (instance) {
			const doc = new XMLDocument();
			doc.#instance = instance;
			return doc;
		}
		return null;
	}

	getElementsByTagName(tagname) {
		return this.#instance.getElementsByTagName(tagname);
	}

	getElementById(id) {
		return this.#instance.getElementById(id);
	}
}
