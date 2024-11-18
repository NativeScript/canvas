import { EventTarget } from './EventTarget';

const DOCUMENT_POSITION_DISCONNECTED = 1;
const DOCUMENT_POSITION_CONTAINS = 8;

export class Node extends EventTarget {
	static ELEMENT_NODE = 1;

	static ATTRIBUTE_NODE = 2;

	static TEXT_NODE = 3;

	static CDATA_SECTION_NODE = 4;

	static PROCESSING_INSTRUCTION_NODE = 7;

	static COMMENT_NODE = 8;

	static DOCUMENT_NODE = 9;

	static DOCUMENT_TYPE_NODE = 10;

	static DOCUMENT_FRAGMENT_NODE = 11;

	className: any;
	nodeType: number;

	readonly nodeName: string;

	_ownerDocument: Document;

	get isConnected() {
		return false;
	}

	get ownerDocument() {
		return this._ownerDocument;
	}

	get childNodes() {
		return (<any>this).nativeElement?.__domElement?.childNodes ?? [];
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
