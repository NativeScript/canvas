package org.nativescript.canvas

/**
 * Created by triniwiz on 4/22/20
 */
class WebGLShaderPrecisionFormat {
	var rangeMin = 0
	var rangeMax = 0
	var precision = 0

	constructor()
	constructor(rangeMin: Int, rangeMax: Int, precision: Int) {
		this.rangeMin = rangeMin
		this.rangeMax = rangeMax
		this.precision = precision
	}
}
