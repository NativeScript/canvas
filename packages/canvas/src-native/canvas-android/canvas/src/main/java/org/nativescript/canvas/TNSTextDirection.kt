package org.nativescript.canvas

enum class TNSTextDirection(private val direction: String, private val value: Int) {
	Ltr("ltr", 0), Rtl("rtl", 1);

	override fun toString(): String {
		return direction
	}

	internal fun fromNative(value: Int): TNSTextDirection? {
		if (value == 0) {
			return Ltr
		} else if (value == 1) {
			return Rtl
		}

		return null
	}

	fun toNative(): Int {
		return value
	}
}
