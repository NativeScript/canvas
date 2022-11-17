package org.nativescript.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.drawable.BitmapDrawable
import java.io.ByteArrayOutputStream
import java.net.URL
import java.nio.ByteBuffer
import java.util.concurrent.CountDownLatch
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit

/**
 * Created by triniwiz on 5/4/20
 */
class TNSImageAsset {
	internal var nativeImageAsset: Long

	interface Callback {
		fun onSuccess(value: Any?)
		fun onError(error: String?)
	}

	val width: Int
		get() = if (nativeImageAsset == 0L) {
			0
		} else nativeGetWidth(nativeImageAsset)
	val height: Int
		get() = if (nativeImageAsset == 0L) {
			0
		} else nativeGetHeight(nativeImageAsset)
	val bytes: ByteArray
		get() = nativeGetBytes(nativeImageAsset)

	private var hasResourceError = false
	private var resourceError: String? = null
	val error: String?
		get() {
			if (nativeImageAsset == 0L) {
				return null
			}
			if (hasResourceError) {
				return resourceError
			}
			return if (!nativeHasError(nativeImageAsset)) {
				null
			} else nativeGetError(nativeImageAsset)
		}

	fun scale(x: Int, y: Int) {
		if (nativeImageAsset == 0L) {
			return
		}
		nativeScale(nativeImageAsset, x, y)
	}

	fun flipX() {
		if (nativeImageAsset == 0L) {
			return
		}
		nativeFlipX(nativeImageAsset)
	}

	fun flipY() {
		if (nativeImageAsset == 0L) {
			return
		}
		nativeFlipY(nativeImageAsset)
	}

	fun save(path: String, format: TNSImageAssetFormat): Boolean {
		return if (nativeImageAsset == 0L) {
			false
		} else nativeSave(
			nativeImageAsset,
			path,
			format.format
		)
	}

	fun saveAsync(path: String, format: TNSImageAssetFormat, callback: Callback) {
		executorService.submit {
			if (save(path, format)) {
				callback.onSuccess(true)
			} else {
				callback.onError(error)
			}
		}
	}

	fun loadImageFromResource(id: Int, context: Context): Boolean {
		try {
			val drawable =
				if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.LOLLIPOP) {
					context.resources.getDrawable(id, null)
				} else {
					context.resources.getDrawable(id)
				}

			return loadImageFromImage(
				(drawable as? BitmapDrawable)?.bitmap ?: run {
					val bitmap = Bitmap.createBitmap(
						drawable.intrinsicWidth,
						drawable.intrinsicHeight,
						Bitmap.Config.ARGB_8888
					)
					val canvas = Canvas(bitmap)
					drawable.draw(canvas)
					bitmap
				}
			)
		} catch (e: Exception) {
			hasResourceError = true
			resourceError = e.toString()
		}

		return !hasResourceError
	}

	fun loadImageFromPath(path: String): Boolean {
		return if (nativeImageAsset == 0L) {
			false
		} else nativeLoadAssetPath(nativeImageAsset, path)
	}

	// Blocking because why not 😂
	fun loadImageFromUrl(url: String): Boolean {
		resourceError = null
		val lock = CountDownLatch(1)
		var result = false
		executorService.execute {
			try {
				val urlSrc =
					URL(url)
				val bs = ByteArrayOutputStream2()
				urlSrc.openStream().use { input ->
					bs.use { output ->
						input.copyTo(output)
					}
				}
				result = loadImageFromBytes(bs.buf())
			} catch (e: Exception) {
				resourceError = e.message
				result = false
			}
			lock.countDown()
		}
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (e: java.lang.Exception) {

		}
		return result
	}

	fun loadImageFromUrlAsync(url: String, callback: Callback) {
		resourceError = null
		executorService.execute {
			try {
				val urlSrc =
					URL(url)
				val bs = ByteArrayOutputStream2()
				urlSrc.openStream().use { input ->
					bs.use { output ->
						input.copyTo(output)
					}
				}
				val loaded = loadImageFromBytes(bs.buf())
				if (loaded) {
					callback.onSuccess(true)
				} else {
					callback.onError(error)
				}
			} catch (e: Exception) {
				resourceError = e.message
				callback.onError(error)
			}
		}
	}

	fun loadImageFromPathAsync(path: String, callback: Callback) {
		executorService.submit {
			if (nativeLoadAssetPath(nativeImageAsset, path)) {
				callback.onSuccess(true)
			} else {
				callback.onError(error)
			}
		}
	}

	fun loadImageFromBytes(buffer: ByteArray): Boolean {
		return if (nativeImageAsset == 0L) {
			false
		} else nativeLoadAssetBytes(nativeImageAsset, buffer)
	}

	fun loadImageFromBufferAsync(buffer: ByteBuffer, callback: Callback) {
		executorService.submit {
			if (nativeLoadAssetBuffer(nativeImageAsset, buffer)) {
				callback.onSuccess(true)
			} else {
				callback.onError(error)
			}
		}
	}

	fun loadImageFromBuffer(buffer: ByteBuffer): Boolean {
		return if (nativeImageAsset == 0L) {
			false
		} else nativeLoadAssetBuffer(nativeImageAsset, buffer)
	}

	fun loadImageFromBytesAsync(buffer: ByteArray, callback: Callback) {
		executorService.submit {
			if (nativeLoadAssetBytes(nativeImageAsset, buffer)) {
				callback.onSuccess(true)
			} else {
				callback.onError(error)
			}
		}
	}

	fun loadImageFromImage(bitmap: Bitmap): Boolean {
		if (nativeImageAsset == 0L) {
			return false
		}
		val os = ByteArrayOutputStream()
		bitmap.compress(Bitmap.CompressFormat.PNG, 100, os)
		return loadImageFromBytes(os.toByteArray())
	}

	fun loadImageFromImageAsync(bitmap: Bitmap, callback: Callback) {
		executorService.submit {
			val os = ByteArrayOutputStream()
			bitmap.compress(Bitmap.CompressFormat.PNG, 100, os)
			if (loadImageFromBytes(os.toByteArray())) {
				callback.onSuccess(true)
			} else {
				callback.onError(error)
			}
		}
	}

	@Throws(Throwable::class)
	protected fun finalize() {
		nativeDestroy(nativeImageAsset)
		nativeImageAsset = 0
	}

	internal class ByteArrayOutputStream2 : ByteArrayOutputStream {
		constructor() : super() {}
		constructor(size: Int) : super(size) {}

		/**
		 * Returns the internal buffer of this ByteArrayOutputStream, without copying.
		 */
		@Synchronized
		fun buf(): ByteArray {
			return buf
		}
	}

	companion object {
		@JvmStatic
		private external fun nativeInit(): Long

		@JvmStatic
		internal fun nativeGetWidthImpl(asset: Long): Int {
			return nativeGetWidth(asset)
		}

		@JvmStatic
		private external fun nativeGetWidth(asset: Long): Int


		@JvmStatic
		internal fun nativeGetHeightImpl(asset: Long): Int {
			return nativeGetHeight(asset)
		}

		@JvmStatic
		private external fun nativeGetHeight(asset: Long): Int

		@JvmStatic
		private external fun nativeFlipX(asset: Long): Boolean

		@JvmStatic
		private external fun nativeFlipY(asset: Long): Boolean

		@JvmStatic
		private external fun nativeGetBytes(asset: Long): ByteArray

		@JvmStatic
		private external fun nativeScale(asset: Long, x: Int, y: Int): Boolean

		@JvmStatic
		private external fun nativeGetError(asset: Long): String

		@JvmStatic
		private external fun nativeHasError(asset: Long): Boolean

		@JvmStatic
		private external fun nativeSave(asset: Long, path: String, format: Int): Boolean

		@JvmStatic
		private external fun nativeLoadAssetPath(asset: Long, path: String): Boolean

		@JvmStatic
		private external fun nativeLoadAssetBytes(asset: Long, buffer: ByteArray): Boolean

		@JvmStatic
		private external fun nativeLoadAssetBuffer(asset: Long, buffer: ByteBuffer): Boolean

		@JvmStatic
		private external fun nativeDestroy(asset: Long)

		@JvmStatic
		internal fun nativeDestroyImpl(asset: Long) {
			nativeDestroy(asset)
		}

		private val executorService = Executors.newCachedThreadPool()
	}

	init {
		nativeImageAsset = nativeInit()
	}
}
