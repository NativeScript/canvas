import { Node } from './Node';

export class CharacterData extends Node {
	private _data: string;
	constructor(data: string) {
		super('');
		this._data = data;
	}

	get data() {
		return this._data;
	}

	get length() {
		return this._data.length;
	}
}
