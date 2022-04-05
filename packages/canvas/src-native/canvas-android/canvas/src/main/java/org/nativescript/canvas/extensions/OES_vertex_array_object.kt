package org.nativescript.canvas.extensions

import android.opengl.GLES30
import android.os.Build
import androidx.annotation.RequiresApi
import org.nativescript.canvas.Constants
import org.nativescript.canvas.TNSCanvas
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

/**
 * Created by triniwiz on 5/1/20
 */
@RequiresApi(api = Build.VERSION_CODES.JELLY_BEAN_MR2)
class OES_vertex_array_object(var canvas: TNSCanvas) {
	var VERTEX_ARRAY_BINDING_OES = Constants.GL_VERTEX_ARRAY_BINDING_OES
	fun createVertexArrayOES(): Int {
		val array = IntArray(1)
		GLES30.glGenVertexArrays(1, array, 0)
		return array[0]
	}

	fun deleteVertexArrayOES(arrayObject: Int) {
		val array = intArrayOf(arrayObject)
		GLES30.glDeleteVertexArrays(1, array, 0)
	}

	fun isVertexArrayOES(arrayObject: Int): Boolean {
	return GLES30.glIsVertexArray(arrayObject)
	}

	fun bindVertexArrayOES(arrayObject: Int) {
		GLES30.glBindVertexArray(arrayObject)
	}
}
