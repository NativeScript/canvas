package com.github.triniwiz.canvas

import java.nio.ByteBuffer

/**
 * Created by triniwiz on 2019-08-04
 */
class TNSImageData(val width: Int, val height: Int, internal var nativeImageData: Long) {
	internal var dataStore: ByteBuffer? = null

	init {
		if (nativeImageData == -1L) {
			dataStore = ByteBuffer.allocateDirect(width * height * 4)
		}
	}

	val data: ByteBuffer
		get() {
			if (nativeImageData == -1L) {
				return dataStore!!
			}
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
