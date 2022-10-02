package org.nativescript.canvas

import java.nio.*
import java.nio.charset.Charset

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

	fun decode(buffer: ByteBuffer): String {
		val result = nativeDecodeBufferToBytes(nativeDecoder, buffer)
		return String(result, Charset.defaultCharset())
	}

	fun decode(bytes: ByteArray): String {
		val result = nativeDecodeToBytes(nativeDecoder, bytes)
		return String(result, Charset.defaultCharset())
	}

	fun decode(bytes: ShortArray): String {
		val buffer = ShortBuffer.wrap(bytes)
		val result = nativeDecodeBufferToBytes(nativeDecoder, buffer)
		return String(result, Charset.defaultCharset())
	}

	fun decode(bytes: IntArray): String {
		val buffer = IntBuffer.wrap(bytes)
		val result = nativeDecodeBufferToBytes(nativeDecoder, buffer)
		return String(result, Charset.defaultCharset())
	}

	fun decode(bytes: FloatArray): String {
		val buffer = FloatBuffer.wrap(bytes)
		val result = nativeDecodeBufferToBytes(nativeDecoder, buffer)
		return String(result, Charset.defaultCharset())
	}

	fun decode(bytes: DoubleArray): String {
		val buffer = DoubleBuffer.wrap(bytes)
		val result = nativeDecodeBufferToBytes(nativeDecoder, buffer)
		return String(result, Charset.defaultCharset())
	}

	fun decodeByteBuffer(buffer: ByteBuffer): String {
		val result = nativeDecodeBufferToBytes(nativeDecoder, buffer)
		return String(result, Charset.defaultCharset())
	}

	fun decodeShortBuffer(buffer: ShortBuffer): String {
		val result = nativeDecodeBufferToBytes(nativeDecoder, buffer)
		return String(result, Charset.defaultCharset())
	}

	fun decodeIntBuffer(buffer: IntBuffer): String {
		val result = nativeDecodeBufferToBytes(nativeDecoder, buffer)
		return String(result, Charset.defaultCharset())
	}

	fun decodeFloatBuffer(buffer: FloatBuffer): String {
		val result = nativeDecodeBufferToBytes(nativeDecoder, buffer)
		return String(result, Charset.defaultCharset())
	}

	fun decodeDoubleBuffer(buffer: DoubleBuffer): String {
		val result = nativeDecodeBufferToBytes(nativeDecoder, buffer)
		return String(result, Charset.defaultCharset())
	}


	fun decodeByte(bytes: ByteArray): String {
		return decode(bytes)
	}

	fun decodeShort(bytes: ShortArray): String {
		return decode(bytes)
	}

	fun decodeInt(bytes: IntArray): String {
		return decode(bytes)
	}

	fun decodeFloat(bytes: FloatArray): String {
		return decode(bytes)
	}

	fun decodeDouble(bytes: FloatArray): String {
		return decode(bytes)
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

		@JvmStatic
		private external fun nativeDecodeToBytes(decoder: Long, bytes: ByteArray): ByteArray

		@JvmStatic
		private external fun nativeDecodeBufferToBytes(decoder: Long, bytes: Buffer): ByteArray
	}
}
