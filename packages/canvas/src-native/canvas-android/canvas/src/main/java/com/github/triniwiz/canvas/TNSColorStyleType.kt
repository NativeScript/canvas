package com.github.triniwiz.canvas

/**
 * Created by triniwiz on 2019-07-13
 */
enum class TNSColorStyleType(private val type: String) {
	Color("color"), Gradient("gradient"), Pattern("pattern");

	override fun toString(): String {
		return type
	}
}
