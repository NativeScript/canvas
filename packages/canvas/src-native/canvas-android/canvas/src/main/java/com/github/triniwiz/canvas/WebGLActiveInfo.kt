package com.github.triniwiz.canvas

/**
 * Created by triniwiz on 4/21/20
 */
class WebGLActiveInfo {
	var name = ""
	var size = 0
	var type = 0

	constructor()
	constructor(name: String, size: Int, type: Int) {
		this.name = name
		this.size = size
		this.type = type
	}
}
