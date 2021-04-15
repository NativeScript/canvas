package org.nativescript.canvas

/**
 * Created by triniwiz on 2019-07-13
 */
enum class TNSTextAlignment(private val textAlign: String, private val value: Int) {
	Start("start", 0), Left("left", 1), Center("center", 2), Right("right", 3), End("end", 4);

	override fun toString(): String {
		return textAlign
	}

	internal var isError = false

	fun toNative(): Int {
		return value
	}

	companion object {
		fun fromNative(value: Int): TNSTextAlignment {
			return when (value) {
				0 -> Start
				1 -> Left
				2 -> Center
				3 -> Right
				4 -> End
				else -> Start.apply {
					isError = true
				}
			}
		}
	}
}
