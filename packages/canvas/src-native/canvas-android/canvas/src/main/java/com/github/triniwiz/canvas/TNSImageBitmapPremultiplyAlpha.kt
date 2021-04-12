package com.github.triniwiz.canvas

enum class TNSImageBitmapPremultiplyAlpha(val alpha: String, val value: Int) {
	Default("default", 0),
	Premultiply("premultiply", 1),
	None("none", 2);

	override fun toString(): String {
		return alpha
	}

	fun toNative(): Int {
		return value
	}

	companion object {
		fun fromNative(value: Int): TNSImageBitmapPremultiplyAlpha? {
			return when (value) {
				0 -> Default
				1 -> Premultiply
				2 -> None
				else -> null
			}
		}
	}
}
