export class WebGLActiveInfo {
	readonly name: string;
	readonly size: number;
	readonly type: number;
	[Symbol.toStringTag] = 'WebGLActiveInfo';

	constructor(name: string, size: number, type: number) {
		this.name = name;
		this.size = size;
		this.type = type;
	}

	public toString() {
		return '[object WebGLActiveInfo]';
	}
}
