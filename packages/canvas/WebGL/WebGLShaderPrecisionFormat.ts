export class WebGLShaderPrecisionFormat {
	readonly native: any;
	[Symbol.toStringTag] = 'WebGLShaderPrecisionFormat';

	constructor(native) {
		this.native = native;
	}

	get rangeMin() {
		return this.native.rangeMin;
	}

	get rangeMax(): number {
		return this.native.rangeMax;
	}

	get precision(): number {
		return this.native.precision;
	}

	public toString() {
		return '[object WebGLShaderPrecisionFormat]';
	}
}
