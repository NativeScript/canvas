package org.nativescript.canvas

import android.os.Handler
import android.os.Looper
import java.nio.ByteBuffer
import java.util.concurrent.Executors


class NSCImageBitmap(asset: Long) {
	var asset: Long = asset
		private set

	interface Callback {
		fun onComplete(done: Boolean)
	}

	class Options {
		var premultiplyAlpha = ImageBitmapPremultiplyAlpha.Default
		var colorSpaceConversion = ImageBitmapColorSpaceConversion.Default
		var resizeQuality = ImageBitmapResizeQuality.Low

		// only handling flipY or not
		var imageOrientation = ImageBitmapImageOrientation.FromImage
		var resizeWidth: Float = 0F
		var resizeHeight: Float = 0F
	}

	@Synchronized
	@Throws(Throwable::class)
	protected fun finalize() {
		NSCImageAsset.nativeDestroyImageAsset(asset)
		asset = 0
	}


	companion object {
		init {
			NSCCanvas.loadLib()
		}

		@JvmStatic
		fun createFrom(asset: Long, data: ByteBuffer, callback: Callback) {
			val loop = Looper.myLooper()
			executorService.execute {
				val done = if (data.isDirect) {
					nativeLoadFromBuffer(asset, data)
				} else {
					nativeLoadFromBytes(asset, data.array())
				}
				if (loop != null) {
					val handler = Handler(loop)
					handler.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		@JvmStatic
		fun createFrom(asset: Long, data: ByteArray, callback: Callback) {
			val loop = Looper.myLooper()
			executorService.execute {
				val done = nativeLoadFromBytes(asset, data)
				if (loop != null) {
					val handler = Handler(loop)
					handler.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		@JvmStatic
		fun createFromOptions(asset: Long, data: ByteBuffer, options: Options, callback: Callback) {
			val loop = Looper.myLooper()
			executorService.execute {
				val done = if (data.isDirect) {
					nativeLoadFromBufferOptions(
						asset,
						data,
						options.imageOrientation.value == 1,
						options.premultiplyAlpha.value,
						options.colorSpaceConversion.value,
						options.resizeQuality.value,
						options.resizeWidth,
						options.resizeHeight
					)
				} else {
					nativeLoadFromBytesOptions(
						asset, data.array(), options.imageOrientation.value == 1,
						options.premultiplyAlpha.value,
						options.colorSpaceConversion.value,
						options.resizeQuality.value,
						options.resizeWidth,
						options.resizeHeight
					)
				}
				if (loop != null) {
					val handler = Handler(loop)
					handler.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		@JvmStatic
		fun createFromOptions(asset: Long, data: ByteArray, options: Options, callback: Callback) {
			val loop = Looper.myLooper()
			executorService.execute {
				val done = nativeLoadFromBytesOptions(
					asset, data, options.imageOrientation.value == 1,
					options.premultiplyAlpha.value,
					options.colorSpaceConversion.value,
					options.resizeQuality.value,
					options.resizeWidth,
					options.resizeHeight
				)
				if (loop != null) {
					val handler = Handler(loop)
					handler.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		@JvmStatic
		fun createFromRectOptions(
			asset: Long,
			data: ByteBuffer,
			x: Float,
			y: Float,
			width: Float,
			height: Float,
			options: Options,
			callback: Callback
		) {
			val loop = Looper.myLooper()
			executorService.execute {
				val done = if (data.isDirect) {
					nativeLoadFromBufferRectOptions(
						asset,
						data,
						x, y, width, height,
						options.imageOrientation.value == 1,
						options.premultiplyAlpha.value,
						options.colorSpaceConversion.value,
						options.resizeQuality.value,
						options.resizeWidth,
						options.resizeHeight
					)
				} else {
					nativeLoadFromBytesRectOptions(
						asset, data.array(),
						x, y, width, height,
						options.imageOrientation.value == 1,
						options.premultiplyAlpha.value,
						options.colorSpaceConversion.value,
						options.resizeQuality.value,
						options.resizeWidth,
						options.resizeHeight
					)
				}
				if (loop != null) {
					val handler = Handler(loop)
					handler.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		@JvmStatic
		fun createFromRectOptions(
			asset: Long,
			data: ByteArray,
			x: Float,
			y: Float,
			width: Float,
			height: Float,
			options: Options,
			callback: Callback
		) {
			val loop = Looper.myLooper()
			executorService.execute {
				val done = nativeLoadFromBytesRectOptions(
					asset, data,
					x, y, width, height,
					options.imageOrientation.value == 1,
					options.premultiplyAlpha.value,
					options.colorSpaceConversion.value,
					options.resizeQuality.value,
					options.resizeWidth,
					options.resizeHeight
				)
				if (loop != null) {
					val handler = Handler(loop)
					handler.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		private val executorService = Executors.newFixedThreadPool(4)

		@JvmStatic
		private external fun nativeLoadFromBuffer(asset: Long, buffer: ByteBuffer): Boolean

		@JvmStatic
		private external fun nativeLoadFromBytes(asset: Long, bytes: ByteArray): Boolean

		@JvmStatic
		private external fun nativeLoadFromBufferOptions(
			asset: Long, buffer: ByteBuffer,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float
		): Boolean

		@JvmStatic
		private external fun nativeLoadFromBytesOptions(
			asset: Long, bytes: ByteArray,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float
		): Boolean


		@JvmStatic
		private external fun nativeLoadFromBufferRectOptions(
			asset: Long, buffer: ByteBuffer,
			x: Float,
			y: Float,
			width: Float,
			height: Float,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float
		): Boolean

		@JvmStatic
		private external fun nativeLoadFromBytesRectOptions(
			asset: Long, bytes: ByteArray,
			x: Float,
			y: Float,
			width: Float,
			height: Float,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float
		): Boolean

	}

}
