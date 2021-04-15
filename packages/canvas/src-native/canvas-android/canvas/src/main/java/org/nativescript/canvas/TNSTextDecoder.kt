package org.nativescript.canvas

import java.nio.*

/**
 * Created by triniwiz on 5/15/20
 */
class TNSTextDecoder {
	private var nativeDecoder: Long = 0

	constructor() {
		init("utf-8")
	}

	constructor(encoding: String) {
		init(encoding)
	}

	private fun init(encoding: String) {
		nativeDecoder = nativeInit(encoding)
	}

	@Throws(Throwable::class)
	protected fun finalize() {
		nativeDestroy(nativeDecoder)
	}

	fun decode(buffer: ByteArray): String {
		return nativeDecode(nativeDecoder, buffer)
	}

	fun decode(buffer: ByteBuffer): String {
		return nativeDecodeBuffer(nativeDecoder, buffer)
	}

	fun decode(bytes: ShortArray): String {
		val buffer = ShortBuffer.wrap(bytes)
		return nativeDecodeBuffer(nativeDecoder, buffer)
	}

	fun decode(bytes: IntArray): String {
		val buffer = IntBuffer.wrap(bytes)
		return nativeDecodeBuffer(nativeDecoder, buffer)
	}

	fun decode(bytes: FloatArray): String {
		val buffer = FloatBuffer.wrap(bytes)
		return nativeDecodeBuffer(nativeDecoder, buffer)
	}

	fun decode(bytes: DoubleArray): String {
		val buffer = DoubleBuffer.wrap(bytes)
		return nativeDecodeBuffer(nativeDecoder, buffer)
	}

	val encoding: String
		get() = nativeGetEncoding(nativeDecoder)

	companion object {
		@JvmStatic
		private external fun nativeInit(encoding: String): Long

		@JvmStatic
		private external fun nativeDestroy(decoder: Long)

		@JvmStatic
		private external fun nativeGetEncoding(decoder: Long): String

		@JvmStatic
		private external fun nativeDecode(decoder: Long, bytes: ByteArray): String

		@JvmStatic
		private external fun nativeDecodeBuffer(decoder: Long, bytes: Buffer): String
	}
}
