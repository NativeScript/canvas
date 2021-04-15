package org.nativescript.canvas.extensions

import android.opengl.GLES30
import android.os.Build
import androidx.annotation.RequiresApi

/**
 * Created by triniwiz on 5/8/20
 */
@RequiresApi(api = Build.VERSION_CODES.JELLY_BEAN_MR2)
class ANGLE_instanced_arrays {
	val VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE = GLES30.GL_VERTEX_ATTRIB_ARRAY_DIVISOR
	fun drawArraysInstancedANGLE(mode: Int, first: Int, count: Int, primcount: Int) {
		GLES30.glDrawArraysInstanced(mode, first, count, primcount)
	}

	fun drawElementsInstancedANGLE(mode: Int, count: Int, type: Int, offset: Int, primcount: Int) {
		GLES30.glDrawElementsInstanced(mode, count, type, offset, primcount)
	}

	fun vertexAttribDivisorANGLE(index: Int, divisor: Int) {
		GLES30.glVertexAttribDivisor(index, divisor)
	}
}
