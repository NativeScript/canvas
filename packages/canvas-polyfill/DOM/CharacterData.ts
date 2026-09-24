import { Node } from './Node';

export class CharacterData extends Node {
	private _data: string;
	/** The native text node, once this has been appended into an SVG. */
	__domNode;
	constructor(data: string) {
		super('');
		this._data = data;
	}

	get data() {
		return this._data;
	}

	set data(value: string) {
		this._data = value == null ? '' : String(value);
		if (this.__domNode) {
			this.__domNode.textContent = this._data;
		}
	}

	get textContent() {
		return this.data;
	}

	set textContent(value: string) {
		this.data = value;
	}

	get nodeValue() {
		return this.data;
	}

	set nodeValue(value: string) {
		this.data = value;
	}

	get length() {
		return this._data.length;
	}
}
