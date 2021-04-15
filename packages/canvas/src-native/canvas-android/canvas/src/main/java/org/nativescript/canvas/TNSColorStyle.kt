package org.nativescript.canvas

/**
 * Created by triniwiz on 2019-07-13
 */
abstract class TNSColorStyle {
	abstract val styleType: TNSColorStyleType?

	companion object {
		@JvmStatic
		external fun nativeDestroy(style: Long)
	}
}
