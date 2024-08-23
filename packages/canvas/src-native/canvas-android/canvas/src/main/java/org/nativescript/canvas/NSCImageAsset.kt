package org.nativescript.canvas

import android.content.res.Resources
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.graphics.Canvas
import android.os.Handler
import android.os.Looper
import androidx.annotation.Nullable
import dalvik.annotation.optimization.FastNative
import org.nativescript.canvas.NSCCanvas.Companion.nativeReleaseGL
import org.nativescript.canvas.NSCCanvas.Companion.nativeReleaseGLPointer
import java.nio.ByteBuffer
import java.util.concurrent.Executors

/**
 * Created by triniwiz on 5/4/20
 */
class NSCImageAsset(asset: Long) {
	var asset: Long = asset
		private set

	interface Callback {
		fun onComplete(done: Boolean)
	}

	@Synchronized
	@Throws(Throwable::class)
	protected fun finalize() {
		destroyImageAsset(asset)
		asset = 0
	}

	companion object {

		init {
			NSCCanvas.loadLib()
		}

		@JvmStatic
		fun createImageAsset(): Long {
			return nativeCreateImageAsset()
		}

		@JvmStatic
		fun destroyImageAsset(asset: Long) {
			return nativeDestroyImageAsset(asset)
		}

		@JvmStatic
		fun loadImageFromResource(resources: Resources, asset: Long, image: Int): Boolean {
			val bitmap = BitmapFactory.decodeResource(resources, image) ?: return false
			if (bitmap.config != Bitmap.Config.ARGB_8888) {
				val bm = Bitmap.createBitmap(bitmap.width, bitmap.height, Bitmap.Config.ARGB_8888)

				val canvas = Canvas(bm)
				canvas.drawBitmap(bitmap, 0F, 0F, null)
				return nativeLoadFromBitmap(asset, bitmap)
			}
			return nativeLoadFromBitmap(asset, bitmap)
		}


		@JvmStatic
		fun loadImageFromResourceAsync(
			resources: Resources,
			asset: Long,
			image: Int,
			callback: Callback
		) {
			val looper = Looper.myLooper()
			executorService.execute {
				val bitmap = BitmapFactory.decodeResource(resources, image)
				if (bitmap == null) {
					callback.onComplete(false)
					return@execute
				}
				val done = loadImageFromResource(resources, asset, image)
				if (looper != null) {
					val handle = Handler(looper)
					handle.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}

			}
		}

		@JvmStatic
		fun loadImageFromBitmap(asset: Long, bitmap: Bitmap): Boolean {
			if (bitmap.config != Bitmap.Config.ARGB_8888) {
				val bm = Bitmap.createBitmap(bitmap.width, bitmap.height, Bitmap.Config.ARGB_8888)

				val canvas = Canvas(bm)
				canvas.drawBitmap(bitmap, 0F, 0F, null)
				return nativeLoadFromBitmap(asset, bitmap)
			}
			return nativeLoadFromBitmap(asset, bitmap)
		}

		@JvmStatic
		fun loadFromPath(asset: Long, path: String): Boolean {
			return nativeLoadFromPath(asset, path)
		}

		@JvmStatic
		fun getError(asset: Long): String {
			return nativeGetError(asset) ?: ""
		}

		@JvmStatic
		fun loadWebP(asset: Long, path: String): Boolean {
			val bm = BitmapFactory.decodeFile(path)
			return nativeLoadFromBitmap(asset, bm)
		}

		@JvmStatic
		fun loadWebPAsync(asset: Long, path: String, callback: Callback) {
			val looper = Looper.myLooper()
			executorService.execute {
				val bm = BitmapFactory.decodeFile(path)
				val done = nativeLoadFromBitmap(asset, bm)

				if (looper != null) {
					val handle = Handler(looper)
					handle.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		@JvmStatic
		fun loadImageFromBitmapAsync(asset: Long, bitmap: Bitmap, callback: Callback) {
			val looper = Looper.myLooper()
			executorService.execute {
				val done = nativeLoadFromBitmap(asset, bitmap)
				if (looper != null) {
					val handle = Handler(looper)
					handle.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		@JvmStatic
		fun loadImageFromPathAsync(asset: Long, path: String, callback: Callback) {
			val looper = Looper.myLooper()
			executorService.execute {
				val done = nativeLoadFromPath(asset, path)
				if (looper != null) {
					val handle = Handler(looper)
					handle.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		@JvmStatic
		fun loadImageFromUrlAsync(asset: Long, url: String, callback: Callback) {
			val looper = Looper.myLooper()
			executorService.execute {
				val done = nativeLoadFromUrl(asset, url)
				if (looper != null) {
					val handle = Handler(looper)
					handle.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		@JvmStatic
		fun loadImageFromBufferAsync(asset: Long, buffer: ByteBuffer, callback: Callback) {
			val looper = Looper.myLooper()
			executorService.execute {
				val done: Boolean = if (buffer.isDirect) {
					nativeLoadFromBuffer(asset, buffer)
				} else {
					nativeLoadFromBytes(asset, buffer.array())
				}
				if (looper != null) {
					val handle = Handler(looper)
					handle.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		@JvmStatic
		fun loadImageFromBytesAsync(asset: Long, bytes: ByteArray, callback: Callback) {
			val looper = Looper.myLooper()
			executorService.execute {
				val done = nativeLoadFromBytes(asset, bytes)
				if (looper != null) {
					val handle = Handler(looper)
					handle.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		@JvmStatic
		fun loadImageFromBytesAsync(asset: Long, bitmap: Bitmap, callback: Callback) {
			val looper = Looper.myLooper()
			executorService.execute {
				val done = nativeLoadFromBitmap(asset, bitmap)
				if (looper != null) {
					val handle = Handler(looper)
					handle.post {
						callback.onComplete(done)
					}
				} else {
					callback.onComplete(done)
				}
			}
		}

		@JvmStatic
		fun getDimensions(asset: Long): IntArray {
			val ret = intArrayOf(0, 0)
			nativeGetDimensions(asset, ret)
			return ret
		}

		@JvmStatic
		private external fun nativeCreateImageAsset(): Long

		@JvmStatic
		external fun nativeDestroyImageAsset(asset: Long)

		@JvmStatic
		private external fun nativeLoadFromBitmap(asset: Long, bitmap: Bitmap): Boolean

		@JvmStatic
		private external fun nativeLoadFromPath(asset: Long, path: String): Boolean

		@JvmStatic
		private external fun nativeLoadFromUrl(asset: Long, path: String): Boolean

		@JvmStatic
		private external fun nativeLoadFromBuffer(asset: Long, buffer: ByteBuffer): Boolean

		@JvmStatic
		private external fun nativeLoadFromBytes(asset: Long, bytes: ByteArray): Boolean


		@JvmStatic
		private external fun nativeGetDimensions(asset: Long, dimensions: IntArray)

		@JvmStatic
		private external fun nativeGetError(asset: Long): String?

		private val executorService = Executors.newFixedThreadPool(4)
	}

}
