package com.github.triniwiz.canvas

enum class TNSImageSmoothingQuality(private val quality: String, private val value: Int) {
	Low("low", 0), Medium("medium", 1), High("high", 2);

	override fun toString(): String {
		return quality
	}

	internal var isError = false

	fun toNative(): Int {
		return value
	}

	companion object {
		fun fromNative(value: Int): TNSImageSmoothingQuality {
			return when (value) {
				0 -> Low
				1 -> Medium
				2 -> High
				else -> Low.apply { isError = true }
			}
		}
	}
}
