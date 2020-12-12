package com.github.triniwiz.canvas

import java.nio.ByteBuffer

/**
 * Created by triniwiz on 5/15/20
 */
class TNSTextEncoder {
	private var nativeEncoder: Long = 0

	constructor() {
		init("utf-8")
	}

	constructor(encoding: String) {
		init(encoding)
	}

	private fun init(encoding: String) {
		nativeEncoder = nativeInit(encoding)
	}

	@Throws(Throwable::class)
	protected fun finalize() {
		nativeDestroy(nativeEncoder)
	}

	val encoding: String
		get() = nativeGetEncoding(nativeEncoder)

	fun encode(text: String): ByteBuffer {
		val buf = nativeEncode(nativeEncoder, text)
		return ByteBuffer.wrap(buf)
	}

	companion object {
		@JvmStatic
		private external fun nativeInit(encoding: String): Long

		@JvmStatic
		private external fun nativeDestroy(encoder: Long)

		@JvmStatic
		private external fun nativeGetEncoding(encoder: Long): String

		@JvmStatic
		private external fun nativeEncode(encoder: Long, text: String): ByteArray
	}
}
