package com.github.triniwiz.canvas

enum class TNSLineCap(private val lineCap: String, private val value: Int) {
	Butt("butt", 0), Round("round", 1), Square("square", 2);

	override fun toString(): String {
		return lineCap
	}

	fun toNative(): Int {
		return value
	}

	internal var isError: Boolean = false

	companion object {
		fun fromNative(value: Int): TNSLineCap {
			return when (value) {
				0 -> Butt
				1 -> Round
				2 -> Square
				else -> {
					Butt.apply {
						isError = true
					}
				}
			}
		}
	}
}
