import { EventTarget } from './EventTarget';
import setValue from 'set-value';


let id = 0;

export class Node extends EventTarget {
	className: any;
	readonly nodeName: string;

	private _id: string;

	constructor(nodeName: string) {
		super();

		this.className = {
			baseVal: ''
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

	appendChild(view) {
	}

	insertBefore(view) {
	}

	removeChild(view) {
	}

	setAttributeNS() {
	}
}
