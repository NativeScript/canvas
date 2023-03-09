export class WebGLActiveInfo {
	[Symbol.toStringTag] = 'WebGLActiveInfo';

	native;
	constructor(native) {
		this.native = native;
	}

	public toString() {
		return '[object WebGLActiveInfo]';
	}

	get name() {
		return this.native.name;
	}

	get size() {
		return this.native.size;
	}

	get type() {
		return this.native.type;
	}
}
