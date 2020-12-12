package com.github.triniwiz.canvas

import java.io.File

/**
 * Created by triniwiz on 5/17/20
 */
object TNSFileReader {
	private external fun nativeRead(file: String): ByteArray
	fun read(file: File): ByteArray {
		return read(file.absolutePath)
	}

	fun read(file: String): ByteArray {
		return nativeRead(file)
	}
}
