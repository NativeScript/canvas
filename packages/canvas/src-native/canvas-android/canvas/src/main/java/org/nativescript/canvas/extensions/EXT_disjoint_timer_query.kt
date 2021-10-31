package org.nativescript.canvas.extensions

import android.opengl.GLES20
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
class EXT_disjoint_timer_query(var canvas: TNSCanvas) {
	fun createQueryEXT(): Int {
		val lock = CountDownLatch(1)
		val query = IntArray(1)
		canvas.queueEvent(Runnable {
			GLES30.glGenQueries(1, query, 0)
			lock.countDown()
		})
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (ignored: InterruptedException) {
		}
		return query[0]
	}

	fun deleteQueryEXT(query: Int) {
		val lock = CountDownLatch(1)
		val id = intArrayOf(query)
		canvas.queueEvent(Runnable {
			GLES30.glDeleteQueries(1, id, 0)
			lock.countDown()
		})
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (ignored: InterruptedException) {
		}
	}

	fun isQueryEXT(query: Int): Boolean {
		val lock = CountDownLatch(1)
		val value = BooleanArray(1)
		canvas.queueEvent(Runnable {
			value[0] = GLES30.glIsQuery(query)
			lock.countDown()
		})
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (ignored: InterruptedException) {
		}
		return value[0]
	}

	fun beginQueryEXT(target: Int, query: Int) {
		val lock = CountDownLatch(1)
		canvas.queueEvent(Runnable {
			GLES30.glBeginQuery(target, query)
			lock.countDown()
		})
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (ignored: InterruptedException) {
		}
	}

	fun endQueryEXT(target: Int) {
		val lock = CountDownLatch(1)
		canvas.queueEvent(Runnable {
			GLES30.glEndQuery(target)
			lock.countDown()
		})
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (ignored: InterruptedException) {
		}
	}

	fun queryCounterEXT(query: Int, target: Int) {
		// NOOP
	}

	fun getQueryEXT(target: Int, pname: Int): Int {
		val lock = CountDownLatch(1)
		val query = IntArray(1)
		canvas.queueEvent(Runnable {
			GLES30.glGetQueryiv(target, pname, query, 0)
			lock.countDown()
		})
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (ignored: InterruptedException) {
		}
		return query[0]
	}

	fun getQueryObjectEXT(query: Int, pname: Int): Any {
		val lock = CountDownLatch(1)
		val value = IntArray(1)
		canvas.queueEvent(Runnable {
			GLES30.glGetQueryObjectuiv(query, pname, value, 0)
			lock.countDown()
		})
		try {
			lock.await(2, TimeUnit.SECONDS)
		} catch (ignored: InterruptedException) {
		}
		return if (pname == QUERY_RESULT_AVAILABLE_EXT) {
			value[0] == GLES20.GL_TRUE
		} else value[0]
	}

	companion object {
		var QUERY_COUNTER_BITS_EXT = Constants.GL_QUERY_COUNTER_BITS_EXT
		var CURRENT_QUERY_EXT = Constants.GL_CURRENT_QUERY_EXT
		var QUERY_RESULT_EXT = Constants.GL_QUERY_RESULT_EXT
		var QUERY_RESULT_AVAILABLE_EXT = Constants.GL_QUERY_RESULT_AVAILABLE_EXT
		var TIME_ELAPSED_EXT = Constants.GL_TIME_ELAPSED_EXT
		var TIMESTAMP_EXT = Constants.GL_TIMESTAMP_EXT
		var GPU_DISJOINT_EXT = Constants.GL_GPU_DISJOINT_EXT
	}
}
