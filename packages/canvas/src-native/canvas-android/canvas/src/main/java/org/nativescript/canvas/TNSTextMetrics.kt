package org.nativescript.canvas

/**
 * Created by triniwiz on 2019-07-10
 */

//@JvmInline
//value
class TNSTextMetrics internal constructor(private val metrics: Long) {
	val width: Float
		get() = nativeGetWidth(metrics)
	val actualBoundingBoxLeft: Float
		get() = nativeGetActualBoundingBoxLeft(metrics)
	val actualBoundingBoxRight: Float
		get() = nativeGetActualBoundingBoxRight(metrics)
	val actualBoundingBoxAscent: Float
		get() = nativeGetActualBoundingBoxAscent(metrics)
	val actualBoundingBoxDescent: Float
		get() = nativeGetActualBoundingBoxDescent(metrics)
	val fontBoundingBoxAscent: Float
		get() = nativeGetFontBoundingBoxAscent(metrics)
	val fontBoundingBoxDescent: Float
		get() = nativeGetFontBoundingBoxDescent(metrics)
	val emHeightAscent: Float
		get() = nativeGetEmHeightAscent(metrics)
	val emHeightDescent: Float
		get() = nativeGetEmHeightDescent(metrics)
	val hangingBaseline: Float
		get() = nativeGetHangingBaseline(metrics)
	val alphabeticBaseline: Float
		get() = nativeGetAlphabeticBaseline(metrics)
	val ideographicBaseline: Float
		get() = nativeGetIdeographicBaseline(metrics)

	@Synchronized
	@Throws(Throwable::class)
	protected fun finalize() {
		nativeDestroy(metrics)
	}

	companion object {
		@JvmStatic
		private external fun nativeGetWidth(metrics: Long): Float

		@JvmStatic
		private external fun nativeGetActualBoundingBoxLeft(metrics: Long): Float

		@JvmStatic
		private external fun nativeGetActualBoundingBoxRight(metrics: Long): Float

		@JvmStatic
		private external fun nativeGetActualBoundingBoxAscent(metrics: Long): Float

		@JvmStatic
		private external fun nativeGetActualBoundingBoxDescent(metrics: Long): Float

		@JvmStatic
		private external fun nativeGetFontBoundingBoxAscent(metrics: Long): Float

		@JvmStatic
		private external fun nativeGetFontBoundingBoxDescent(metrics: Long): Float

		@JvmStatic
		private external fun nativeGetEmHeightAscent(metrics: Long): Float

		@JvmStatic
		private external fun nativeGetEmHeightDescent(metrics: Long): Float

		@JvmStatic
		private external fun nativeGetHangingBaseline(metrics: Long): Float

		@JvmStatic
		private external fun nativeGetAlphabeticBaseline(metrics: Long): Float

		@JvmStatic
		private external fun nativeGetIdeographicBaseline(metrics: Long): Float

		@JvmStatic
		private external fun nativeDestroy(metrics: Long): Float
	}
}
