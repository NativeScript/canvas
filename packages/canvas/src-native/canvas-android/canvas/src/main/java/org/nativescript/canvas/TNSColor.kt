package org.nativescript.canvas

/**
 * Created by triniwiz on 5/30/20
 */
class TNSColor(var color: String) : TNSColorStyle() {
	override val styleType: TNSColorStyleType
		get() = TNSColorStyleType.Color

	constructor(style: Long) : this(nativeGetColorString(style))

	companion object {
		@JvmStatic
		private external fun nativeGetColorString(style: Long): String
	}
}
