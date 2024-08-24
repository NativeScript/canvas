import { EventTarget } from './EventTarget';

const DOCUMENT_POSITION_DISCONNECTED = 1;
const DOCUMENT_POSITION_CONTAINS = 8;

export class Node extends EventTarget {
	className: any;
	readonly nodeName: string;

	_ownerDocument: Document;

	get ownerDocument() {
		return this._ownerDocument;
	}

	constructor(nodeName: string) {
		super();

		this.className = {
			baseVal: '',
		};
		this.nodeName = nodeName;
	}

	append(view) {}

	appendChild(view) {
		if (view instanceof Document) {
			return;
		}
		if (view instanceof Element) {
		}
	}

	insertBefore(view) {}

	removeChild(view) {}

	compareDocumentPosition(otherNode) {
		// const element = (<any>this)._xmlDom?.documentElement ?? (<any>this)._xmlDom;
		if (this.nodeName === '#document') {
			return DOCUMENT_POSITION_CONTAINS;
		}

		return DOCUMENT_POSITION_DISCONNECTED;
	}

	contains(otherNode: Node) {
		if (this.nodeName === 'BODY') {
			return !!(otherNode as any)?.parent || !!(otherNode as any)?.nativeElement?.parent;
		}
		return false;
	}
}
