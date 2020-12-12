package com.github.triniwiz.canvas

enum class TNSFillRule(var rule: String, var value: Int) {
	NonZero("nonzero", 0), EvenOdd("evenodd", 1);

	override fun toString(): String {
		return rule
	}

	fun toNative(): Int {
		return value
	}

	companion object {
		fun fromNative(value: Int): TNSFillRule? {
			return when (value) {
				0 -> NonZero
				1 -> EvenOdd
				else -> null
			}
		}
	}
}
