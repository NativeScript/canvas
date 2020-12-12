package com.github.triniwiz.canvas

import java.nio.ByteBuffer
import java.nio.ByteOrder

/**
 * Created by triniwiz on 2019-08-04
 */
class TNSImageData(val width: Int, val height: Int, internal var nativeImageData: Long) {
	val data: ByteBuffer
		get() {
			return nativeData(nativeImageData)
		}

	@Throws(Throwable::class)
	protected fun finalize() {
		nativeDestroy(nativeImageData)
	}

	companion object {
		@JvmStatic
		private external fun nativeData(imageData: Long): ByteBuffer
		@JvmStatic
		private external fun nativeDestroy(imageData: Long)
	}
}
