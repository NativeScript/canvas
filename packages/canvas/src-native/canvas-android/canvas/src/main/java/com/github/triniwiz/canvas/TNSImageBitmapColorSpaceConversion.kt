package com.github.triniwiz.canvas

enum class TNSImageBitmapColorSpaceConversion(val space: String, val value: Int) {
	Default("default", 0),
	None("none", 1);

	override fun toString(): String {
		return space
	}

	fun toNative(): Int {
		return value
	}

	companion object {
		fun fromNative(value: Int): TNSImageBitmapColorSpaceConversion? {
			return when (value) {
				0 -> Default
				1 -> None
				else -> null
			}
		}
	}
}
