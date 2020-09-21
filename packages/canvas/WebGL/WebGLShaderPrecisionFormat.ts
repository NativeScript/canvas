export class WebGLShaderPrecisionFormat {
	readonly rangeMin: number;
	readonly rangeMax: number;
	readonly precision: number;
	[Symbol.toStringTag] = 'WebGLShaderPrecisionFormat';

	constructor(rangeMin: number, rangeMax: number, precision: number) {
		this.rangeMin = rangeMin;
		this.rangeMax = rangeMax;
		this.precision = precision;
	}

	public toString() {
		return '[object WebGLShaderPrecisionFormat]';
	}
}
