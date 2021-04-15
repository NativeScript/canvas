package org.nativescript.canvas

/**
 * Created by triniwiz on 2019-07-13
 */
enum class TNSTextBaseline(var baseLine: String) {
	Top("top"), Hanging("hanging"), Middle("middle"), Alphabetic("alphabetic"), Ideographic("ideographic"), Bottom(
		"bottom"
	);

	override fun toString(): String {
		return baseLine
	}
}
