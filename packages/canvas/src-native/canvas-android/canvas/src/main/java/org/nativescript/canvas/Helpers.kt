package org.nativescript.canvas

import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Rect
import android.graphics.drawable.BitmapDrawable
import android.graphics.drawable.Drawable
import android.graphics.drawable.VectorDrawable
import android.os.Build
import androidx.core.graphics.createBitmap

object Helpers {
	val emptyFloat = FloatArray(0)

	fun getBitmap(src: Drawable): Bitmap {
		return (src as? BitmapDrawable)?.bitmap ?: run {
			val bitmap = createBitmap(src.intrinsicWidth, src.intrinsicHeight)
			val canvas = Canvas(bitmap)
			var previousBounds: Rect? = null
			(src as? VectorDrawable)?.let {
				previousBounds = src.bounds
				src.setBounds(0, 0, src.intrinsicWidth, src.intrinsicHeight)
			}

			src.draw(canvas)

			previousBounds?.let {
				src.bounds = it
			}

			bitmap
		}
	}
}
