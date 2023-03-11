package org.nativescript.canvas

import android.graphics.Bitmap
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
        fun loadImageFromBitmap(asset: Long, bitmap: Bitmap): Boolean {
            return nativeLoadFromBitmap(asset, bitmap)
        }

        @JvmStatic
        fun loadImageFromBitmapAsync(asset: Long, bitmap: Bitmap, callback: Callback) {
            executorService.execute {
                val done = nativeLoadFromBitmap(asset, bitmap)
                callback.onComplete(done)
            }
        }

        @JvmStatic
        private external fun nativeLoadFromBitmap(asset: Long, bitmap: Bitmap): Boolean

        private val executorService = Executors.newFixedThreadPool(10)
    }

}
