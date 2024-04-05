package org.nativescript.canvas

import android.graphics.Bitmap
import android.opengl.GLUtils
import android.util.Log
import dalvik.annotation.optimization.FastNative

class NSCWebGLRenderingContext {
	companion object {
		init {
			NSCCanvas.loadLib()
		}

		@JvmStatic
		fun texImage2D(
			context: Long,
			target: Int,
			level: Int,
			internalformat: Int,
			format: Int,
			type: Int,
			image: Bitmap,
			flipY: Boolean
		) {
			nativeTexImage2D(context, target, level, internalformat, format, type, image, flipY)
		}


		@JvmStatic
		fun texSubImage2D(
			context: Long,
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			format: Int,
			type: Int,
			image: Bitmap,
			flipY: Boolean
		) {
			nativeTexSubImage2D(context, target, level, xoffset, yoffset, format, type, image, flipY)
		}


		@JvmStatic
		@FastNative
		private external fun nativeTexImage2D(
			context: Long,
			target: Int,
			level: Int,
			internalformat: Int,
			format: Int,
			type: Int,
			bitmap: Bitmap,
			flipY: Boolean,
		)


		@JvmStatic
		@FastNative
		private external fun nativeTexSubImage2D(
			context: Long,
			level: Int,
			target: Int,
			xoffset: Int,
			yoffset: Int,
			format: Int,
			type: Int,
			bitmap: Bitmap,
			flipY: Boolean,
		)
	}

}
