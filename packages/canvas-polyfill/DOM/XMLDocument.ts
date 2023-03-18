import { Document } from './Document';

export class XMLDocument extends Document {
	_instance;
	static fromParser(instance) {
		if (instance) {
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
}
