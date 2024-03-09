package org.nativescript.canvas

import android.graphics.Bitmap
import android.graphics.Canvas
import androidx.annotation.Nullable
import dalvik.annotation.optimization.FastNative
import java.util.concurrent.Executors

/**
 * Created by triniwiz on 5/4/20
 */
class NSCImageAsset {

	interface Callback {
		fun onComplete(done: Boolean)
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
		fun loadImageFromBitmap(asset: Long, bitmap: Bitmap): Boolean {
			if (bitmap.config == Bitmap.Config.RGB_565 || bitmap.config == Bitmap.Config.ARGB_4444) {
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
		fun loadImageFromBitmapAsync(asset: Long, bitmap: Bitmap, callback: Callback) {
			executorService.execute {
				val done = nativeLoadFromBitmap(asset, bitmap)
				callback.onComplete(done)
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
		private external fun nativeDestroyImageAsset(asset: Long)


		@JvmStatic
		private external fun nativeLoadFromBitmap(asset: Long, bitmap: Bitmap): Boolean

		@JvmStatic
		private external fun nativeLoadFromPath(asset: Long, path: String): Boolean

		@JvmStatic
		private external fun nativeGetDimensions(asset: Long, dimensions: IntArray)

		@JvmStatic
		private external fun nativeGetError(asset: Long): String?


		private val executorService = Executors.newFixedThreadPool(10)
	}

}
