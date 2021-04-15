package org.nativescript.canvas

/**
 * Created by triniwiz on 5/30/20
 */
class TNSCanvasGradient internal constructor(var style: Long) : TNSColorStyle() {
	override val styleType = TNSColorStyleType.Gradient

	fun addColorStop(offset: Float, color: String) {
		nativeAddColorStop(style, offset, color)
	}

	@Synchronized
	@Throws(Throwable::class)
	protected fun finalize() {
		nativeDestroy(style)
	}

	companion object {
		@JvmStatic
		private external fun nativeAddColorStop(style: Long, stop: Float, color: String)
	}
}
