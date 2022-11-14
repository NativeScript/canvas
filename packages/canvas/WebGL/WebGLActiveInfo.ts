export class WebGLActiveInfo {
	_native: any;
	_name: string;
	get name(): string {
		if (this._native) {
			this._name = this._native.getName();
		}
		return this._name;
	}

	_size: number;
	get size(): number {
		if (this._native) {
			this._size = this._native.getSize();
		}
		return this._size;
	}

	_type: number;
	get type(): number {
		if (this._native) {
			this._type = this._native.getType();
		}
		return this._type;
	}

	[Symbol.toStringTag] = 'WebGLActiveInfo';

	constructor(name: string, size: number, type: number) {
		if (arguments.length === 1) {
			this._native = name;
		} else {
			this._name = name;
			this._size = size;
			this._type = type;
		}
	}

	public toString() {
		return '[object WebGLActiveInfo]';
	}
}
