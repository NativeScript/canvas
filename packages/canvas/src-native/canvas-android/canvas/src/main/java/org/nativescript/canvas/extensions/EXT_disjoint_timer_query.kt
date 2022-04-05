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
		val query = IntArray(1)
		GLES30.glGenQueries(1, query, 0)
		return query[0]
	}

	fun deleteQueryEXT(query: Int) {
		val id = intArrayOf(query)
		GLES30.glDeleteQueries(1, id, 0)
	}

	fun isQueryEXT(query: Int): Boolean {
		return GLES30.glIsQuery(query)
	}

	fun beginQueryEXT(target: Int, query: Int) {
		GLES30.glBeginQuery(target, query)
	}

	fun endQueryEXT(target: Int) {
		GLES30.glEndQuery(target)
	}

	fun queryCounterEXT(query: Int, target: Int) {
		// NOOP
	}

	fun getQueryEXT(target: Int, pname: Int): Int {
		val query = IntArray(1)
		GLES30.glGetQueryiv(target, pname, query, 0)
		return query[0]
	}

	fun getQueryObjectEXT(query: Int, pname: Int): Any {
		val value = IntArray(1)
		GLES30.glGetQueryObjectuiv(query, pname, value, 0)
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
