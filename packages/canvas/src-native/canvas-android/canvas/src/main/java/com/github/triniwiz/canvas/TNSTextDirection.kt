package com.github.triniwiz.canvas

enum class TNSTextDirection(private val direction: String, private val value: Int) {
	Ltr("ltr", 0), Rtl("rtl", 1);

	override fun toString(): String {
		return direction
	}

	fun toNative(): Int {
		return value
	}
}
