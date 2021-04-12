package com.github.triniwiz.canvas

enum class TNSImageBitmapResizeQuality(val quality: String, val value: Int){
	Low("low", 0),
	Medium("medium", 1),
	High("high", 2),
	Pixelated("pixelated", 3);

	override fun toString(): String {
		return quality
	}

	fun toNative(): Int {
		return value
	}

	companion object {
		fun fromNative(value: Int): TNSImageBitmapResizeQuality? {
			return when (value) {
				0 -> Low
				1 -> Medium
				2 -> High
				3 -> Pixelated
				else -> null
			}
		}
	}
}
