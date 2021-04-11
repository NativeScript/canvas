package com.github.triniwiz.canvas

import java.nio.ByteBuffer

class TNSImageBitmap {
	internal var nativeImageAsset: Long

	internal constructor(asset: Long) {
		nativeImageAsset = asset
	}

	fun createFromBytes(bytes: ByteArray) {
		// TNSImageBitmap(asset)
	}

	fun createFromBytes(bytes: ByteArray, sx: Float, sy: Float,sWidth: Float,sHeight: Float) {
		// TNSImageBitmap(asset)
	}

	private var widthCache: Int = 0
	val width: Int
		get() {
			return widthCache
		}

	private var heightCache: Int = 0
	val height: Int
		get() {
			return heightCache
		}

	fun close() {
		TNSImageAsset.nativeDestroyImpl(nativeImageAsset)
		widthCache = 0
		heightCache = 0
		nativeImageAsset = 0
	}


	@Throws(Throwable::class)
	protected fun finalize() {
		if (nativeImageAsset > 0) {
			TNSImageAsset.nativeDestroyImpl(nativeImageAsset)
			nativeImageAsset = 0
		}
	}


	object Companion {
		@JvmStatic
		private external fun nativeCreateFromBuffer(
			imageBuffer: ByteBuffer,
			imageWidth: Float,
			imageHeight: Float,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
		)

		@JvmStatic
		private external fun nativeCreateFromBufferSrcRect(
			imageBuffer: ByteBuffer,
			imageWidth: Float,
			imageHeight: Float,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
		)

		@JvmStatic
		private external fun nativeCreateFromBytes(
			imageBuffer: ByteArray,
			imageWidth: Float,
			imageHeight: Float,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
		)

		@JvmStatic
		private external fun nativeCreateFromBytesSrcRect(
			imageBuffer: ByteArray,
			imageWidth: Float,
			imageHeight: Float,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
		)
	}
}
