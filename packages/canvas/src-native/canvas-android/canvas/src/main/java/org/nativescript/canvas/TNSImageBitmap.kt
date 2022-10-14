package org.nativescript.canvas

import android.graphics.Bitmap
import android.os.Handler
import android.os.Looper
import java.nio.ByteBuffer
import java.util.concurrent.Executors

class TNSImageBitmap {

	interface Callback {
		fun onSuccess(asset: Long)
		fun onError(message: String)
	}


	companion object {
		init {
			TNSCanvas.loadLib()
		}

		val FAILED_TO_LOAD: String = "Failed to load image"

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
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val asset: Long
				if (buffer.isDirect) {
					asset = nativeCreateFromBufferSrcRect(
						buffer,
						imageWidth,
						imageHeight,
						sx,
						sy,
						sWidth,
						sHeight,
						flipY,
						premultiplyAlpha,
						colorSpaceConversion,
						resizeQuality,
						resizeWidth,
						resizeHeight
					)
				} else {
					asset = nativeCreateFromBytesSrcRect(
						buffer.array(),
						imageWidth,
						imageHeight,
						sx,
						sy,
						sWidth,
						sHeight,
						flipY,
						premultiplyAlpha,
						colorSpaceConversion,
						resizeQuality,
						resizeWidth,
						resizeHeight
					)
				}

				if (asset != 0L) {
					callback.onSuccess(asset)
				} else {
					callback.onError(FAILED_TO_LOAD)
				}
			}
		}


		@JvmStatic
		fun createFromBuffer(
			buffer: ByteBuffer,
			imageWidth: Float,
			imageHeight: Float,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val asset: Long
				if (buffer.isDirect) {
					asset = nativeCreateFromBuffer(
						buffer,
						imageWidth,
						imageHeight,
						flipY,
						premultiplyAlpha,
						colorSpaceConversion,
						resizeQuality,
						resizeWidth,
						resizeHeight
					)
				} else {
					asset = nativeCreateFromBytes(
						buffer.array(),
						imageWidth,
						imageHeight,
						flipY,
						premultiplyAlpha,
						colorSpaceConversion,
						resizeQuality,
						resizeWidth,
						resizeHeight
					)
				}

				if (asset != 0L) {
					callback.onSuccess(asset)
				} else {
					callback.onError(FAILED_TO_LOAD)
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
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val asset: Long
				if (buffer.isDirect) {
					asset = nativeCreateFromBufferEncodedSrcRect(
						buffer,
						sx,
						sy,
						sWidth,
						sHeight,
						flipY,
						premultiplyAlpha,
						colorSpaceConversion,
						resizeQuality,
						resizeWidth,
						resizeHeight
					)
				} else {
					asset = nativeCreateFromBytesEncodedSrcRect(
						buffer.array(),
						sx,
						sy,
						sWidth,
						sHeight,
						flipY,
						premultiplyAlpha,
						colorSpaceConversion,
						resizeQuality,
						resizeWidth,
						resizeHeight
					)
				}
				if (asset != 0L) {
					callback.onSuccess(asset)
				} else {
					callback.onError(FAILED_TO_LOAD)
				}
			}
		}


		@JvmStatic
		fun createFromBufferEncoded(
			buffer: ByteBuffer,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val asset: Long
				if (buffer.isDirect) {
					asset = nativeCreateFromBufferEncoded(
						buffer,
						flipY,
						premultiplyAlpha,
						colorSpaceConversion,
						resizeQuality,
						resizeWidth,
						resizeHeight
					)
				} else {
					asset = nativeCreateFromBytesEncoded(
						buffer.array(),
						flipY,
						premultiplyAlpha,
						colorSpaceConversion,
						resizeQuality,
						resizeWidth,
						resizeHeight
					)
				}
				if (asset != 0L) {
					callback.onSuccess(asset)
				} else {
					callback.onError(FAILED_TO_LOAD)
				}
			}
		}


		@JvmStatic
		fun createFromImageData(
			imageData: Long,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromAsset(
					imageData,
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)

				if (result != 0L) {
					callback.onSuccess(result)
				} else {
					callback.onError(FAILED_TO_LOAD)
				}
			}
		}

		@JvmStatic
		fun createFromImageData(
			imageData: Long,
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
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromAssetSrcRect(
					imageData,
					sx, sy, sWidth, sHeight,
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)

				if (result != 0L) {
					callback.onSuccess(result)
				} else {
					callback.onError(FAILED_TO_LOAD)
				}
			}
		}


		@JvmStatic
		fun createFromCanvas(
			canvas: TNSCanvas,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			val bytes = canvas.snapshot()
			createFromBuffer(
				bytes,
				canvas.width.toFloat(),
				canvas.height.toFloat(),
				flipY,
				premultiplyAlpha,
				colorSpaceConversion,
				resizeQuality,
				resizeWidth,
				resizeHeight,
				callback
			)
		}

		@JvmStatic
		fun createFromCanvas(
			canvas: TNSCanvas,
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
			callback: Callback
		) {
			val bytes = canvas.snapshot()
			createFromBuffer(
				bytes, sx,
				sy,
				sWidth,
				sHeight, canvas.width.toFloat(), canvas.height.toFloat(),
				flipY,
				premultiplyAlpha,
				colorSpaceConversion,
				resizeQuality,
				resizeWidth,
				resizeHeight,
				callback
			)
		}


		@JvmStatic
		fun createFromBytes(
			bytes: ByteArray,
			imageWidth: Float,
			imageHeight: Float,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val asset = nativeCreateFromBytes(
					bytes,
					imageWidth,
					imageHeight,
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)
				if (asset != 0L) {
					callback.onSuccess(asset)
				} else {
					callback.onError(FAILED_TO_LOAD)
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
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val asset = nativeCreateFromBytesEncodedSrcRect(
					bytes,
					sx,
					sy,
					sWidth,
					sHeight,
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)
				if (asset != 0L) {
					callback.onSuccess(asset)
				} else {
					callback.onError(FAILED_TO_LOAD)
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
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val asset = nativeCreateFromBytesEncodedSrcRect(
					bytes,
					sx,
					sy,
					sWidth,
					sHeight,
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)
				if (asset != 0L) {
					callback.onSuccess(asset)
				} else {
					callback.onError(FAILED_TO_LOAD)
				}
			}
		}


		@JvmStatic
		fun createFromBytesEncoded(
			bytes: ByteArray,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val asset = nativeCreateFromBytesEncoded(
					bytes,
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)
				if (asset != 0L) {
					callback.onSuccess(asset)
				} else {
					callback.onError(FAILED_TO_LOAD)
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
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
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
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)
				if (asset != 0L) {
					callback.onSuccess(asset)
				} else {
					callback.onError(FAILED_TO_LOAD)
				}
			}
		}


		@JvmStatic
		fun createFromImageBitmap(
			bitmap: Long,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromAsset(
					bitmap,
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)

				if (result != 0L) {
					callback.onSuccess(result)
				} else {
					callback.onError(FAILED_TO_LOAD)
				}
			}
		}

		@JvmStatic
		fun createFromImageBitmap(
			bitmap: Long,
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
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromAssetSrcRect(
					bitmap,
					sx, sy, sWidth, sHeight,
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)

				if (result != 0L) {
					callback.onSuccess(result)
				} else {
					callback.onError(FAILED_TO_LOAD)
				}
			}
		}


		@JvmStatic
		fun createFromImageAsset(
			asset: Long,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromAsset(
					asset,
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)

				if (result != 0L) {
					callback.onSuccess(result)
				} else {
					callback.onError(FAILED_TO_LOAD)
				}
			}
		}

		@JvmStatic
		fun createFromImageAsset(
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
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromAssetSrcRect(
					asset,
					sx, sy, sWidth, sHeight,
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)

				if (result != 0L) {
					callback.onSuccess(result)
				} else {
					callback.onError(FAILED_TO_LOAD)
				}
			}
		}


		@JvmStatic
		fun createFromBitmap(
			bitmap: Bitmap,
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromBitmap(
					bitmap,
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)

				if (result != 0L) {
					callback.onSuccess(result)
				} else {
					callback.onError(FAILED_TO_LOAD)
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
			flipY: Boolean,
			premultiplyAlpha: Int,
			colorSpaceConversion: Int,
			resizeQuality: Int,
			resizeWidth: Float,
			resizeHeight: Float,
			callback: Callback
		) {
			executor.execute {
				val result = nativeCreateFromBitmapSrcRect(
					bitmap,
					sx, sy, sWidth, sHeight,
					flipY,
					premultiplyAlpha,
					colorSpaceConversion,
					resizeQuality,
					resizeWidth,
					resizeHeight
				)

				if (result != 0L) {
					callback.onSuccess(result)
				} else {
					callback.onError(FAILED_TO_LOAD)
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
