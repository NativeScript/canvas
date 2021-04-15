package org.nativescript.canvas

import android.graphics.Bitmap
import android.os.Handler
import android.os.Looper
import java.nio.ByteBuffer
import java.util.concurrent.Executors

class TNSImageBitmap internal constructor(asset: Long) {

	var nativeImageAsset: Long = asset

	val width: Int
		get() = if (nativeImageAsset == 0L) {
			0
		} else TNSImageAsset.nativeGetWidthImpl(nativeImageAsset)
	val height: Int
		get() = if (nativeImageAsset == 0L) {
			0
		} else TNSImageAsset.nativeGetHeightImpl(nativeImageAsset)

	fun close() {
		TNSImageAsset.nativeDestroyImpl(nativeImageAsset)
		nativeImageAsset = 0
	}


	@Throws(Throwable::class)
	protected fun finalize() {
		if (nativeImageAsset > 0) {
			TNSImageAsset.nativeDestroyImpl(nativeImageAsset)
			nativeImageAsset = 0
		}
	}

	interface Callback {
		fun onSuccess(result: TNSImageBitmap)
		fun onError(message: String)
	}

	class Options {
		var flipY = false
		var premultiplyAlpha = TNSImageBitmapPremultiplyAlpha.Default
		var colorSpaceConversion = TNSImageBitmapColorSpaceConversion.Default
		var resizeQuality = TNSImageBitmapResizeQuality.Low
		var resizeWidth = 0f
		var resizeHeight = 0f
	}

	companion object {
		val FAILED_TO_LOAD: String = "Failed to load image"

		private val handler = Handler(Looper.getMainLooper())

		@JvmStatic
		private val executor = Executors.newCachedThreadPool()


		@JvmStatic
		fun createFromBuffer(
			buffer: ByteBuffer,
			imageWidth: Float,
			imageHeight: Float,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val asset: Long
				if(buffer.isDirect){
					asset = nativeCreateFromBufferSrcRect(
						buffer,
						imageWidth,
						imageHeight,
						sx,
						sy,
						sWidth,
						sHeight,
						options.flipY,
						options.premultiplyAlpha.toNative(),
						options.colorSpaceConversion.toNative(),
						options.resizeQuality.toNative(),
						options.resizeWidth,
						options.resizeHeight
					)
				}else {
					asset = nativeCreateFromBytesSrcRect(
						buffer.array(),
						imageWidth,
						imageHeight,
						sx,
						sy,
						sWidth,
						sHeight,
						options.flipY,
						options.premultiplyAlpha.toNative(),
						options.colorSpaceConversion.toNative(),
						options.resizeQuality.toNative(),
						options.resizeWidth,
						options.resizeHeight
					)
				}

				handler.post {
					if (asset > 0) {
						callback.onSuccess(TNSImageBitmap(asset))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}


		@JvmStatic
		fun createFromBuffer(
			buffer: ByteBuffer,
			imageWidth: Float,
			imageHeight: Float,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val asset: Long
				if (buffer.isDirect){
					asset = nativeCreateFromBuffer(
						buffer,
						imageWidth,
						imageHeight,
						options.flipY,
						options.premultiplyAlpha.toNative(),
						options.colorSpaceConversion.toNative(),
						options.resizeQuality.toNative(),
						options.resizeWidth,
						options.resizeHeight
					)
				}else {
					asset = nativeCreateFromBytes(
						buffer.array(),
						imageWidth,
						imageHeight,
						options.flipY,
						options.premultiplyAlpha.toNative(),
						options.colorSpaceConversion.toNative(),
						options.resizeQuality.toNative(),
						options.resizeWidth,
						options.resizeHeight
					)
				}
				handler.post {
					if (asset > 0) {
						callback.onSuccess(TNSImageBitmap(asset))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}


		@JvmStatic
		fun createFromBufferEncoded(
			buffer: ByteBuffer,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val asset: Long
				if (buffer.isDirect){
					asset = nativeCreateFromBufferEncodedSrcRect(
						buffer,
						sx,
						sy,
						sWidth,
						sHeight,
						options.flipY,
						options.premultiplyAlpha.toNative(),
						options.colorSpaceConversion.toNative(),
						options.resizeQuality.toNative(),
						options.resizeWidth,
						options.resizeHeight
					)
				}else {
					asset = nativeCreateFromBytesEncodedSrcRect(
						buffer.array(),
						sx,
						sy,
						sWidth,
						sHeight,
						options.flipY,
						options.premultiplyAlpha.toNative(),
						options.colorSpaceConversion.toNative(),
						options.resizeQuality.toNative(),
						options.resizeWidth,
						options.resizeHeight
					)
				}
				handler.post {
					if (asset > 0) {
						callback.onSuccess(TNSImageBitmap(asset))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}


		@JvmStatic
		fun createFromBufferEncoded(
			buffer: ByteBuffer,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val asset: Long
				if(buffer.isDirect){
					asset = nativeCreateFromBufferEncoded(
						buffer,
						options.flipY,
						options.premultiplyAlpha.toNative(),
						options.colorSpaceConversion.toNative(),
						options.resizeQuality.toNative(),
						options.resizeWidth,
						options.resizeHeight
					)
				}else {
					 asset = nativeCreateFromBytesEncoded(
						buffer.array(),
						options.flipY,
						options.premultiplyAlpha.toNative(),
						options.colorSpaceConversion.toNative(),
						options.resizeQuality.toNative(),
						options.resizeWidth,
						options.resizeHeight
					)
				}
				handler.post {
					if (asset > 0) {
						callback.onSuccess(TNSImageBitmap(asset))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}


		@JvmStatic
		fun createFromImageData(
			imageData: TNSImageData,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromAsset(
					imageData.nativeImageData,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)

				handler.post {
					if (result > 0) {
						callback.onSuccess(TNSImageBitmap(result))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}

		@JvmStatic
		fun createFromImageData(
			imageData: TNSImageData,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromAssetSrcRect(
					imageData.nativeImageData,
					sx, sy, sWidth, sHeight,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)

				handler.post {
					if (result > 0) {
						callback.onSuccess(TNSImageBitmap(result))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}


		@JvmStatic
		fun createFromCanvas(
			canvas: TNSCanvas,
			options: Options,
			callback: Callback
		) {
			val bytes = canvas.snapshot()
			createFromBytes(
				bytes, canvas.width.toFloat(), canvas.height.toFloat(), options, callback
			)
		}

		@JvmStatic
		fun createFromCanvas(
			canvas: TNSCanvas,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			options: Options,
			callback: Callback
		) {
			val bytes = canvas.snapshot()
			createFromBytes(
				bytes, sx,
				sy,
				sWidth,
				sHeight, canvas.width.toFloat(), canvas.height.toFloat(), options, callback
			)
		}


		@JvmStatic
		fun createFromBytes(
			bytes: ByteArray,
			imageWidth: Float,
			imageHeight: Float,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val asset = nativeCreateFromBytes(
					bytes,
					imageWidth,
					imageHeight,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)
				handler.post {
					if (asset > 0) {
						callback.onSuccess(TNSImageBitmap(asset))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}

		@JvmStatic
		fun createFromBytes(
			bytes: ByteArray,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val asset = nativeCreateFromBytesEncodedSrcRect(
					bytes,
					sx,
					sy,
					sWidth,
					sHeight,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)
				handler.post {
					if (asset > 0) {
						callback.onSuccess(TNSImageBitmap(asset))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}


		@JvmStatic
		fun createFromBytesEncoded(
			bytes: ByteArray,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val asset = nativeCreateFromBytesEncodedSrcRect(
					bytes,
					sx,
					sy,
					sWidth,
					sHeight,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)
				handler.post {
					if (asset > 0) {
						callback.onSuccess(TNSImageBitmap(asset))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}


		@JvmStatic
		fun createFromBytesEncoded(
			bytes: ByteArray,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val asset = nativeCreateFromBytesEncoded(
					bytes,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)
				handler.post {
					if (asset > 0) {
						callback.onSuccess(TNSImageBitmap(asset))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}

		@JvmStatic
		fun createFromBytes(
			bytes: ByteArray,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			imageWidth: Float,
			imageHeight: Float,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val asset = nativeCreateFromBytesSrcRect(
					bytes,
					imageWidth,
					imageHeight,
					sx,
					sy,
					sWidth,
					sHeight,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)
				handler.post {
					if (asset > 0) {
						callback.onSuccess(TNSImageBitmap(asset))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}


		@JvmStatic
		fun createFromImageBitmap(
			bitmap: TNSImageBitmap,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromAsset(
					bitmap.nativeImageAsset,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)

				handler.post {
					if (result > 0) {
						callback.onSuccess(TNSImageBitmap(result))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}

		@JvmStatic
		fun createFromImageBitmap(
			bitmap: TNSImageBitmap,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromAssetSrcRect(
					bitmap.nativeImageAsset,
					sx, sy, sWidth, sHeight,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)

				handler.post {
					if (result > 0) {
						callback.onSuccess(TNSImageBitmap(result))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}


		@JvmStatic
		fun createFromImageAsset(
			asset: TNSImageAsset,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromAsset(
					asset.nativeImageAsset,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)

				handler.post {
					if (result > 0) {
						callback.onSuccess(TNSImageBitmap(result))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}

		@JvmStatic
		fun createFromImageAsset(
			asset: TNSImageAsset,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromAssetSrcRect(
					asset.nativeImageAsset,
					sx, sy, sWidth, sHeight,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)

				handler.post {
					if (result > 0) {
						callback.onSuccess(TNSImageBitmap(result))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}


		@JvmStatic
		fun createFromBitmap(
			bitmap: Bitmap,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromBitmap(
					bitmap,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)

				handler.post {
					if (result > 0) {
						callback.onSuccess(TNSImageBitmap(result))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}

		@JvmStatic
		fun createFromBitmap(
			bitmap: Bitmap,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			options: Options,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromBitmapSrcRect(
					bitmap,
					sx, sy, sWidth, sHeight,
					options.flipY,
					options.premultiplyAlpha.toNative(),
					options.colorSpaceConversion.toNative(),
					options.resizeQuality.toNative(),
					options.resizeWidth,
					options.resizeHeight
				)

				handler.post {
					if (result > 0) {
						callback.onSuccess(TNSImageBitmap(result))
					} else {
						callback.onError(FAILED_TO_LOAD)
					}
				}
			}
		}


		@JvmStatic
		private external fun nativeCreateFromBitmap(
			bitmap: Bitmap,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float
		): Long


		@JvmStatic
		private external fun nativeCreateFromBitmapSrcRect(
			bitmap: Bitmap,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float
		): Long

		@JvmStatic
		private external fun nativeCreateFromAsset(
			asset: Long,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
		): Long

		@JvmStatic
		private external fun nativeCreateFromAssetSrcRect(
			asset: Long,
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
		): Long


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
		): Long

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
		): Long


		@JvmStatic
		private external fun nativeCreateFromBufferEncoded(
			imageBuffer: ByteBuffer,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
		): Long

		@JvmStatic
		private external fun nativeCreateFromBufferEncodedSrcRect(
			imageBuffer: ByteBuffer,
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
		): Long

		@JvmStatic
		private external fun nativeCreateFromBytesEncoded(
			imageBuffer: ByteArray,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
		): Long

		@JvmStatic
		private external fun nativeCreateFromBytesEncodedSrcRect(
			imageBuffer: ByteArray,
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
		): Long

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
		): Long

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
		): Long
	}
}
