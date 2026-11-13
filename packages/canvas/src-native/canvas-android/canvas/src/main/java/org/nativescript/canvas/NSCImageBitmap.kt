package org.nativescript.canvas

import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.os.Handler
import android.os.Looper
import java.io.ByteArrayOutputStream
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
				val skia = if (data.isDirect) {
					nativeLoadFromBuffer(asset, data)
				} else {
					nativeLoadFromBytes(asset, data.array())
				}
				val done = skia || systemPng(data)?.let { png -> nativeLoadFromBytes(asset, png) } == true
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
				val skia = nativeLoadFromBytes(asset, data)
				val done = skia || systemPng(data)?.let { png -> nativeLoadFromBytes(asset, png) } == true
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
				val skia = if (data.isDirect) {
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
				val done = skia || systemPng(data)?.let { png -> nativeLoadFromBytesOptions(asset, png, options.imageOrientation.value == 1,
						options.premultiplyAlpha.value,
						options.colorSpaceConversion.value,
						options.resizeQuality.value,
						options.resizeWidth,
						options.resizeHeight
					) } == true
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
				val skia = nativeLoadFromBytesOptions(
					asset, data, options.imageOrientation.value == 1,
					options.premultiplyAlpha.value,
					options.colorSpaceConversion.value,
					options.resizeQuality.value,
					options.resizeWidth,
					options.resizeHeight
				)
				val done = skia || systemPng(data)?.let { png -> nativeLoadFromBytesOptions(asset, png, options.imageOrientation.value == 1,
					options.premultiplyAlpha.value,
					options.colorSpaceConversion.value,
					options.resizeQuality.value,
					options.resizeWidth,
					options.resizeHeight
				) } == true
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
				val skia = if (data.isDirect) {
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
				val done = skia || systemPng(data)?.let { png -> nativeLoadFromBytesRectOptions(asset, png,
						x, y, width, height,
						options.imageOrientation.value == 1,
						options.premultiplyAlpha.value,
						options.colorSpaceConversion.value,
						options.resizeQuality.value,
						options.resizeWidth,
						options.resizeHeight
					) } == true
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
				val skia = nativeLoadFromBytesRectOptions(
					asset, data,
					x, y, width, height,
					options.imageOrientation.value == 1,
					options.premultiplyAlpha.value,
					options.colorSpaceConversion.value,
					options.resizeQuality.value,
					options.resizeWidth,
					options.resizeHeight
				)
				val done = skia || systemPng(data)?.let { png -> nativeLoadFromBytesRectOptions(asset, png,
					x, y, width, height,
					options.imageOrientation.value == 1,
					options.premultiplyAlpha.value,
					options.colorSpaceConversion.value,
					options.resizeQuality.value,
					options.resizeWidth,
					options.resizeHeight
				) } == true
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

		/** Bytes Skia can't decode (WebP, HEIF), decoded by Android and re-encoded as PNG; null if Android can't either */
		private fun systemPng(data: ByteArray): ByteArray? {
			val bitmap = BitmapFactory.decodeByteArray(data, 0, data.size) ?: return null
			return try {
				val out = ByteArrayOutputStream()
				if (bitmap.compress(Bitmap.CompressFormat.PNG, 100, out)) out.toByteArray() else null
			} finally {
				bitmap.recycle()
			}
		}

		private fun systemPng(data: ByteBuffer): ByteArray? {
			val view = data.duplicate()
			view.rewind()
			val bytes = ByteArray(view.remaining())
			view.get(bytes)
			return systemPng(bytes)
		}

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
