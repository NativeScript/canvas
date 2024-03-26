import { EventTarget } from './EventTarget';
import setValue from 'set-value';

const DOCUMENT_POSITION_DISCONNECTED = 1;
const DOCUMENT_POSITION_CONTAINS = 8;

export class Node extends EventTarget {
	className: any;
	readonly nodeName: string;

	private _id: string;

	constructor(nodeName: string) {
		super();

		this.className = {
			baseVal: '',
		};
		this.nodeName = nodeName;
	}

	set id(value) {
		this._id = value;
		const nativeElement = this['nativeElement'];
		if (nativeElement) {
			setValue(nativeElement, 'id', value);
		}
	}

	get id() {
		return this._id;
	}

	get ownerDocument() {
		return window.document;
	}

	appendChild(view) {}

	insertBefore(view) {}

	removeChild(view) {}

	setAttributeNS() {}

	compareDocumentPosition(otherNode) {
		const element = (<any>this)._xmlDom?.documentElement ?? (<any>this)._xmlDom;
		if (this.nodeName === '#document') {
			return DOCUMENT_POSITION_CONTAINS;
		}

		return DOCUMENT_POSITION_DISCONNECTED;
	}
}
