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
		val lock = canvas.webGLRenderingContext?.lock ?: canvas.webGL2RenderingContext?.lock
		canvas.queueEvent {
			GLES30.glGenVertexArrays(1, array, 0)
			lock?.countDown()
		}
		try {
			lock?.await(2, TimeUnit.SECONDS)
		} catch (ignored: InterruptedException) {
		}
		return array[0]
	}

	fun deleteVertexArrayOES(arrayObject: Int) {
		val array = intArrayOf(arrayObject)
		val lock = canvas.webGLRenderingContext?.lock ?: canvas.webGL2RenderingContext?.lock
		canvas.queueEvent {
			GLES30.glDeleteVertexArrays(1, array, 0)
			lock?.countDown()
		}
		try {
			lock?.await(2, TimeUnit.SECONDS)
		} catch (ignored: InterruptedException) {
		}
	}

	fun isVertexArrayOES(arrayObject: Int): Boolean {
		val value = BooleanArray(1)
		val lock = canvas.webGLRenderingContext?.lock ?: canvas.webGL2RenderingContext?.lock
		canvas.queueEvent {
			value[0] = GLES30.glIsVertexArray(arrayObject)
			lock?.countDown()
		}
		try {
			lock?.await(2, TimeUnit.SECONDS)
		} catch (ignored: InterruptedException) {
		}
		return value[0]
	}

	fun bindVertexArrayOES(arrayObject: Int) {
		val lock = canvas.webGLRenderingContext?.lock ?: canvas.webGL2RenderingContext?.lock
		canvas.queueEvent {
			GLES30.glBindVertexArray(arrayObject)
			lock?.countDown()
		}
		try {
			lock?.await(2, TimeUnit.SECONDS)
		} catch (ignored: InterruptedException) {
		}
	}
}
