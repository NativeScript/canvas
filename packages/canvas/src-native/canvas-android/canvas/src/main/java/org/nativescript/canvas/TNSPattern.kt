package org.nativescript.canvas

/**
 * Created by triniwiz on 5/30/20
 */
class TNSPattern internal constructor(var style: Long) : TNSColorStyle() {


	fun setTransform(matrix: TNSDOMMatrix) {
		nativeSetTransform(style, matrix.matrix)
	}

	@Throws(Throwable::class)
	protected fun finalize() {
		nativeDestroy(style)
	}

	override val styleType: TNSColorStyleType
		get() = TNSColorStyleType.Pattern

	companion object {
		@JvmStatic
		private external fun nativeSetTransform(pattern: Long, matrix: Long): Long
	}
}
