package org.nativescript.canvas

enum class TNSPatternRepetition(val pattern: String, private val value: Int) {
	Repeat("repeat", 0), RepeatX("repeat-x", 1), RepeatY("repeat-y", 2), NoRepeat("no-repeat", 3);

	override fun toString(): String {
		return pattern
	}

	fun toNative(): Int {
		return value
	}
}
