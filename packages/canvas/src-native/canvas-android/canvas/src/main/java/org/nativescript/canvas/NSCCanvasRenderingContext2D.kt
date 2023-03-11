package org.nativescript.canvas

import android.graphics.Bitmap

class NSCCanvasRenderingContext2D {
    companion object {

        init {
            NSCCanvas.loadLib()
        }

        @JvmStatic
        fun createPattern(context: Long, bitmap: Bitmap, repetition: String): Long {
            return nativeCreatePattern(context, bitmap, repetition)
        }

        @JvmStatic
        private external fun nativeCreatePattern(
            context: Long,
            bitmap: Bitmap,
            repetition: String
        ): Long
    }
}