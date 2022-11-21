package org.nativescript.canvas

import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Rect
import android.graphics.drawable.BitmapDrawable
import android.graphics.drawable.Drawable
import android.graphics.drawable.VectorDrawable
import android.os.Build

object Helpers {
	val emptyFloat = FloatArray(0)

	fun getBitmap(src: Drawable): Bitmap {
		return (src as? BitmapDrawable)?.bitmap ?: run {
			val bitmap = Bitmap.createBitmap(
				src.intrinsicWidth,
				src.intrinsicHeight,
				Bitmap.Config.ARGB_8888
			)
			val canvas = Canvas(bitmap)
			var previousBounds: Rect? = null
			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
				(src as? VectorDrawable)?.let {
					previousBounds = src.bounds
					src.setBounds(0, 0, src.intrinsicWidth, src.intrinsicHeight)
				}
			}

			src.draw(canvas)

			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
				previousBounds?.let {
					src.bounds = it
				}
			}

			bitmap
		}
	}
}
