package com.github.triniwiz.canvas

enum class TNSLineJoin(private val lineJoin: String, private val value: Int) {
	Bevel("bevel", 0), Round("round", 1), Miter("miter", 2);

	override fun toString(): String {
		return lineJoin
	}

	fun toNative(): Int {
		return value
	}

	internal var isError = false

	companion object {
		fun fromNative(value: Int): TNSLineJoin {
			return when (value) {
				0 -> Bevel
				1 -> Round
				2 -> Miter
				else -> Bevel.apply { isError = true }
			}
		}
	}
}
