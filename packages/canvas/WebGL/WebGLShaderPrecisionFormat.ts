export class WebGLShaderPrecisionFormat {
	readonly native: any;
	[Symbol.toStringTag] = 'WebGLShaderPrecisionFormat';

	constructor(native) {
		this.native = native;
	}

	get rangeMin() {
		return this.native.getRangeMin();
	}

	get rangeMax(): number {
		return this.native.getRangeMax();
	}

	get precision(): number {
		return this.native.getPrecision();
	}

	public toString() {
		return '[object WebGLShaderPrecisionFormat]';
	}
}
