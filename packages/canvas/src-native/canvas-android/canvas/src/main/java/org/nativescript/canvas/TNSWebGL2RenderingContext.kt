package org.nativescript.canvas

import android.graphics.Bitmap
import android.opengl.GLES30
import android.os.Build
import android.os.Build.VERSION_CODES
import androidx.annotation.RequiresApi
import java.nio.*
import java.nio.charset.Charset
import java.nio.charset.StandardCharsets
import java.util.concurrent.CountDownLatch

/**
 * Created by triniwiz on 5/1/20
 */
@RequiresApi(VERSION_CODES.JELLY_BEAN_MR2)
class TNSWebGL2RenderingContext : TNSWebGLRenderingContext {
	constructor(canvas: TNSCanvas) : super(canvas)
	constructor(canvas: TNSCanvas, attrs: Map<String?, Any?>?) : super(canvas, attrs)

	fun beginQuery(target: Int, query: Int) {
		ensureContextIsCurrent()
		GLES30.glBeginQuery(target, query)
	}

	fun beginTransformFeedback(primitiveMode: Int) {
		ensureContextIsCurrent()
		GLES30.glBeginTransformFeedback(primitiveMode)

	}

	fun bindBufferBase(target: Int, index: Int, buffer: Int) {
		ensureContextIsCurrent()
		GLES30.glBindBufferBase(target, index, buffer)

	}

	fun bindBufferRange(target: Int, index: Int, buffer: Int, offset: Int, size: Int) {
		ensureContextIsCurrent()
		GLES30.glBindBufferRange(target, index, buffer, offset, size)
	}

	fun bindSampler(unit: Int, sampler: Int) {
		ensureContextIsCurrent()
		GLES30.glBindSampler(unit, sampler)

	}

	fun bindTransformFeedback(target: Int, transformFeedback: Int) {
		ensureContextIsCurrent()
		GLES30.glBindTransformFeedback(target, transformFeedback)

	}

	fun bindVertexArray(vertexArray: Int) {
		ensureContextIsCurrent()
		GLES30.glBindVertexArray(vertexArray)

	}

	fun blitFramebuffer(
		srcX0: Int, srcY0: Int, srcX1: Int, srcY1: Int,
		dstX0: Int, dstY0: Int, dstX1: Int, dstY1: Int,
		mask: Int, filter: Int
	) {
		ensureContextIsCurrent()
		GLES30.glBlitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)

	}

	fun clearBufferfv(buffer: Int, drawbuffer: Int, values: FloatArray?) {
		ensureContextIsCurrent()
		GLES30.glClearBufferfv(buffer, drawbuffer, FloatBuffer.wrap(values))

	}

	fun clearBufferiv(buffer: Int, drawbuffer: Int, values: IntArray?) {
		ensureContextIsCurrent()
		GLES30.glClearBufferiv(buffer, drawbuffer, IntBuffer.wrap(values))

	}

	fun clearBufferuiv(buffer: Int, drawbuffer: Int, values: IntArray?) {
		ensureContextIsCurrent()
		GLES30.glClearBufferuiv(buffer, drawbuffer, IntBuffer.wrap(values))

	}

	fun clearBufferfi(buffer: Int, drawbuffer: Int, depth: Float, stencil: Int) {
		ensureContextIsCurrent()
		GLES30.glClearBufferfi(buffer, drawbuffer, depth, stencil)

	}

	fun clientWaitSync(sync: Long, flags: Int, timeout: Long): Int {
		return GLES30.glClientWaitSync(sync, flags, timeout)
	}

	fun compressedTexSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		imageSize: Int,
		offset: Int
	) {
		ensureContextIsCurrent()
		GLES30.glCompressedTexSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			imageSize,
			offset
		)

	}

	fun compressedTexSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		srcData: ByteArray,
		srcOffset: Int,
		srcLengthOverride: Int
	) {
		ensureContextIsCurrent()
		var size = srcData.size
		val buffer = ByteBuffer.wrap(srcData)
		val offset = srcOffset
		val overrideLength = srcLengthOverride
		if (srcLengthOverride == 0) {
			size -= offset
		} else if (overrideLength > size - offset) {

		}
		buffer.position(offset)
		GLES30.glCompressedTexSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			size,
			buffer
		)

	}

	fun compressedTexSubImage3DByteBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		srcData: ByteBuffer,
		srcOffset: Int,
		srcLengthOverride: Int
	) {
		compressedTexSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			srcData,
			srcOffset,
			srcLengthOverride
		)
	}


	fun compressedTexSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		srcData: ByteBuffer,
		srcOffset: Int,
		srcLengthOverride: Int
	) {
		ensureContextIsCurrent()
		var size = srcData.capacity()
		if (srcLengthOverride == 0) {
			size -= srcOffset
		} else if (srcLengthOverride > size - srcOffset) {

		}
		srcData.position(srcOffset)
		GLES30.glCompressedTexSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			size,
			srcData
		)

	}

	fun copyBufferSubData(
		readTarget: Int,
		writeTarget: Int,
		readOffset: Int,
		writeOffset: Int,
		size: Int
	) {
		ensureContextIsCurrent()
		GLES30.glCopyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size)

	}

	fun copyTexSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		x: Int,
		y: Int,
		width: Int,
		height: Int
	) {
		ensureContextIsCurrent()
		clearIfComposited()
		GLES30.glCopyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height)
	}

	fun createQuery(): Int {
		val lock = CountDownLatch(1)
		val query = IntBuffer.allocate(1)
		ensureContextIsCurrent()
		GLES30.glGenQueries(1, query)
		return query[0]
	}

	fun createSampler(): Int {
		val lock = CountDownLatch(1)
		val sampler = IntBuffer.allocate(1)
		ensureContextIsCurrent()
		GLES30.glGenSamplers(1, sampler)

		return sampler[0]
	}

	fun createVertexArray(): Int {
		val lock = CountDownLatch(1)
		val array = IntBuffer.allocate(1)
		ensureContextIsCurrent()
		GLES30.glGenVertexArrays(1, array)

		return array[0]
	}

	fun createTransformFeedback(): Int {
		val lock = CountDownLatch(1)
		val id = IntBuffer.allocate(1)
		ensureContextIsCurrent()
		GLES30.glGenTransformFeedbacks(1, id)

		return id[0]
	}

	fun deleteQuery(query: Int) {
		ensureContextIsCurrent()
		val id = intArrayOf(query)
		GLES30.glDeleteQueries(1, id, 0)

	}

	fun deleteSampler(sampler: Int) {
		ensureContextIsCurrent()
		val id = intArrayOf(sampler)
		GLES30.glDeleteQueries(1, id, 0)

	}

	fun deleteSync(sync: Int) {
		ensureContextIsCurrent()
		GLES30.glDeleteSync(sync.toLong())

	}

	fun deleteTransformFeedback(transformFeedback: Int) {
		ensureContextIsCurrent()
		val feedback = intArrayOf(transformFeedback)
		GLES30.glDeleteTransformFeedbacks(1, feedback, 0)

	}

	fun deleteVertexArray(vertexArray: Int) {
		ensureContextIsCurrent()
		val array = intArrayOf(vertexArray)
		GLES30.glDeleteVertexArrays(1, array, 0)

	}

	fun drawArraysInstanced(mode: Int, first: Int, count: Int, instanceCount: Int) {
		ensureContextIsCurrent()
		clearIfComposited()
		GLES30.glDrawArraysInstanced(mode, first, count, instanceCount)
		updateCanvas()

	}

	fun drawElementsInstanced(mode: Int, count: Int, type: Int, offset: Int, instanceCount: Int) {
		ensureContextIsCurrent()
		clearIfComposited()
		GLES30.glDrawElementsInstanced(mode, count, type, offset, instanceCount)
		updateCanvas()

	}

	fun drawRangeElements(mode: Int, start: Int, end: Int, count: Int, type: Int, offset: Int) {
		ensureContextIsCurrent()
		clearIfComposited()
		GLES30.glDrawRangeElements(mode, start, end, count, type, offset)
		updateCanvas()

	}

	fun drawBuffers(buffers: IntArray) {
		ensureContextIsCurrent()
		GLES30.glDrawBuffers(buffers.size, IntBuffer.wrap(buffers))

	}


	fun drawBuffersBuffer(buffers: IntBuffer) {
		drawBuffers(buffers)
	}

	private fun drawBuffers(buffers: IntBuffer) {
		ensureContextIsCurrent()
		GLES30.glDrawBuffers(buffers.capacity(), buffers)

	}

	fun endQuery(target: Int) {
		ensureContextIsCurrent()
		GLES30.glEndQuery(target)

	}

	fun endTransformFeedback() {
		ensureContextIsCurrent()
		GLES30.glEndTransformFeedback()

	}

	fun fenceSync(condition: Int, flags: Int) {
		ensureContextIsCurrent()
		GLES30.glFenceSync(condition, flags)

	}

	fun framebufferTextureLayer(target: Int, attachment: Int, texture: Int, level: Int, layer: Int) {
		ensureContextIsCurrent()
		GLES30.glFramebufferTextureLayer(target, attachment, texture, level, layer)

	}

	fun getActiveUniformBlockName(program: Int, uniformBlockIndex: Int): String? {
		val value: String
		ensureContextIsCurrent()
		val maxNameLength = IntBuffer.allocate(1)
		GLES30.glGetProgramiv(program, GLES30.GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH, maxNameLength)
		val name = ByteArray(maxNameLength[0])
		val length = IntBuffer.allocate(1)
		GLES30.glGetActiveUniformBlockName(program, uniformBlockIndex, length, ByteBuffer.wrap(name))
		if (Build.VERSION.SDK_INT >= VERSION_CODES.KITKAT) {
			value = String(name, StandardCharsets.UTF_8)
		} else {
			value = String(name, Charset.forName("UTF-8"))
		}

		return value
	}

	fun getActiveUniformBlockParameter(program: Int, uniformBlockIndex: Int, pname: Int): Any? {
		val value: Any?
		ensureContextIsCurrent()
		when (pname) {
			GLES30.GL_UNIFORM_BLOCK_BINDING, GLES30.GL_UNIFORM_BLOCK_DATA_SIZE, GLES30.GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS -> {
				val intValue = IntBuffer.allocate(1)
				GLES30.glGetActiveUniformBlockiv(program, uniformBlockIndex, pname, intValue)
				value = intValue[0]
			}
			GLES30.GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES -> {
				val uniformCount = IntBuffer.allocate(1)
				GLES30.glGetActiveUniformBlockiv(
					program,
					uniformBlockIndex,
					GLES30.GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS,
					uniformCount
				)
				val indices = IntArray(uniformCount[0])
				GLES30.glGetActiveUniformBlockiv(
					program,
					uniformBlockIndex,
					pname,
					IntBuffer.wrap(indices)
				)
				value = indices
			}
			GLES30.GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER, GLES30.GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER -> {
				val boolValue = IntBuffer.allocate(1)
				GLES30.glGetActiveUniformBlockiv(program, uniformBlockIndex, pname, boolValue)
				value = boolValue[0] == GLES30.GL_TRUE
			}
			else -> value = null
		}

		return value
	}

	internal enum class ReturnType {
		EnumType, UnsignedIntType, IntType, BoolType
	}

	fun getActiveUniforms(program: Int, uniformIndices: IntArray, pname: Int): Any? {
		val value: Any?
		ensureContextIsCurrent()
		var returnType: ReturnType?
		when (pname) {
			GLES30.GL_UNIFORM_TYPE -> returnType = ReturnType.EnumType
			GLES30.GL_UNIFORM_SIZE -> returnType = ReturnType.UnsignedIntType
			GLES30.GL_UNIFORM_BLOCK_INDEX, GLES30.GL_UNIFORM_OFFSET, GLES30.GL_UNIFORM_ARRAY_STRIDE, GLES30.GL_UNIFORM_MATRIX_STRIDE -> returnType =
				ReturnType.IntType
			GLES30.GL_UNIFORM_IS_ROW_MAJOR -> returnType = ReturnType.BoolType
			else -> {
				value = null
				return value
			}
		}
		val activeUniforms = IntBuffer.allocate(1)
		GLES30.glGetProgramiv(
			program, GLES30.GL_ACTIVE_UNIFORMS,
			activeUniforms
		)
		val size = uniformIndices.size
		for (i in 0 until size) {
			if (i >= activeUniforms[0]) {
				value = null
				return value
			}
		}
		val result = IntArray(size)
		GLES30.glGetActiveUniformsiv(
			program, uniformIndices.size,
			IntBuffer.wrap(uniformIndices), pname, IntBuffer.wrap(result)
		)
		value = when (returnType) {
			ReturnType.IntType, ReturnType.EnumType, ReturnType.UnsignedIntType -> result
			ReturnType.BoolType -> fromGLint(result)
		}

		return value
	}

	fun fromGLint(value: IntArray): BooleanArray {
		val array = BooleanArray(value.size)
		for (i in value.indices) {
			array[i] = value[i] == GLES30.GL_TRUE
		}
		return array
	}

	fun getBufferSubData(
		target: Int,
		srcByteOffset: Int,
		dstData: ByteArray,
		dstOffset: Int,
		length: Int
	) {
		ensureContextIsCurrent()
		if (length == 0) {
		}
		val size: Int = dstData.size * SIZE_OF_BYTE
		val typeSize: Int = SIZE_OF_BYTE
		var byteLength = 0
		if (length > 0) {
			// type size is at most 8, so no overflow.
			byteLength = length * typeSize
		}
		var byteOffset = 0
		if (dstOffset > 0) {
			// type size is at most 8, so no overflow.
			byteOffset = dstOffset * typeSize
		}
		var total = byteOffset
		total += byteLength
		if (total > size) {
			return
		}
		if (byteLength == 0) {
			byteLength = size - byteOffset
		}
		GLES30.glBufferSubData(target, byteOffset, byteLength, ByteBuffer.wrap(dstData))

	}

	fun getFragDataLocation(program: Int, name: String?): Int {
		ensureContextIsCurrent()

		return GLES30.glGetFragDataLocation(program, name)
	}

	fun getIndexedParameter(target: Int, index: Int): Any? {
		var value: IndexedParameter? = null
		ensureContextIsCurrent()
		val binding = IndexedParameter()
		when (target) {
			GLES30.GL_UNIFORM_BUFFER_BINDING, GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_BINDING -> {
				val newTarget = IntBuffer.allocate(1)
				GLES30.glGetIntegerv(target, newTarget)
				// NO BINDING RETURN
				if (newTarget[0] == 0) {
					return value
				}
				val buffer = IntBuffer.allocate(1)
				GLES30.glGetIntegeri_v(newTarget[0], index, buffer)
				binding.bufferValue = buffer[0]
				binding.isBuffer = true
				value = binding
			}
			GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_SIZE, GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_START, GLES30.GL_UNIFORM_BUFFER_SIZE, GLES30.GL_UNIFORM_BUFFER_START -> {
				val ptr = LongBuffer.allocate(1)
				GLES30.glGetInteger64i_v(target, index, ptr)
				binding.isBuffer = false
				binding.value = ptr[0]
				value = binding
			}
			else -> {
				value = null
			}
		}
		return value
	}

	fun getInternalformatParameter(target: Int, internalformat: Int, pname: Int): Any? {
		var value: Any? = null
		ensureContextIsCurrent()
		when (internalformat) {
			GLES30.GL_RGB, GLES30.GL_RGBA, GLES30.GL_R8UI, GLES30.GL_R8I, GLES30.GL_R16UI, GLES30.GL_R16I, GLES30.GL_R32UI, GLES30.GL_R32I, GLES30.GL_RG8UI, GLES30.GL_RG8I, GLES30.GL_RG16UI, GLES30.GL_RG16I, GLES30.GL_RG32UI, GLES30.GL_RG32I, GLES30.GL_RGBA8UI, GLES30.GL_RGBA8I, GLES30.GL_RGB10_A2UI, GLES30.GL_RGBA16UI, GLES30.GL_RGBA16I, GLES30.GL_RGBA32UI, GLES30.GL_RGBA32I -> {
				value = IntArray(0)
				return value
			}
			GLES30.GL_R8, GLES30.GL_RG8, GLES30.GL_RGB565, GLES30.GL_RGBA8, GLES30.GL_SRGB8_ALPHA8, GLES30.GL_RGB5_A1, GLES30.GL_RGBA4, GLES30.GL_RGB10_A2, GLES30.GL_DEPTH_COMPONENT16, GLES30.GL_DEPTH_COMPONENT24, GLES30.GL_DEPTH_COMPONENT32F, GLES30.GL_DEPTH24_STENCIL8, GLES30.GL_DEPTH32F_STENCIL8, GLES30.GL_STENCIL_INDEX8 -> {
			}
			GLES30.GL_R16F, GLES30.GL_RG16F, GLES30.GL_R32F, GLES30.GL_RG32F, GLES30.GL_RGBA32F, GLES30.GL_R11F_G11F_B10F -> {
			}
			else -> {
				value = null
				return value
			}
		}
		if (pname == GLES30.GL_SAMPLES) {
			val length = IntBuffer.allocate(1)
			GLES30.glGetInternalformativ(
				target, internalformat,
				GLES30.GL_NUM_SAMPLE_COUNTS, 1, length
			)
			if (length[0] <= 0) {
				value = IntArray(0)
				return value
			}
			val values = IntArray(length[0])
			GLES30.glGetInternalformativ(
				target,
				internalformat,
				pname,
				length[0],
				IntBuffer.wrap(values)
			)
			value = values
		} else {
			value = null
		}

		return value
	}


	override fun getParameter(pname: Int): Any? {
		return when (pname) {
			COPY_READ_BUFFER_BINDING, COPY_WRITE_BUFFER_BINDING -> {
				var parameter: Any? = null
				ensureContextIsCurrent()
				when (pname) {
					COPY_READ_BUFFER_BINDING, COPY_WRITE_BUFFER_BINDING, DRAW_FRAMEBUFFER_BINDING -> {
						val param = IntArray(1)
						GLES30.glGetIntegerv(pname, param, 0)
						parameter =
							if ((pname == COPY_READ_BUFFER_BINDING || pname == COPY_WRITE_BUFFER_BINDING || pname == DRAW_FRAMEBUFFER_BINDING) && param[0] == 0) {
								null
							} else {
								param[0]
							}
					}
				}
				parameter
			}
			else -> {
				super.getParameter(pname)
			}
		}
	}

	fun getQuery(target: Int, pname: Int): Any? {
		ensureContextIsCurrent()
		return if (pname == GLES30.GL_CURRENT_QUERY) {
			val params = IntBuffer.allocate(1)
			GLES30.glGetQueryiv(target, pname, params)
			params[0]
		} else {
			null
		}
	}

	fun getQueryParameter(query: Int, pname: Int): Any? {
		val value: Any?
		ensureContextIsCurrent()
		val params = IntBuffer.allocate(1)
		GLES30.glGetQueryObjectuiv(query, pname, params)
		when (pname) {
			GLES30.GL_QUERY_RESULT -> value = params[0] == GLES30.GL_TRUE
			GLES30.GL_QUERY_RESULT_AVAILABLE -> value = params
			else -> value = null
		}

		return value
	}

	fun getSamplerParameter(sampler: Int, pname: Int): Any? {
		val value: Any?
		ensureContextIsCurrent()
		when (pname) {
			TEXTURE_MAX_LOD, TEXTURE_MIN_LOD -> {
				val floatValue = FloatBuffer.allocate(1)
				GLES30.glGetSamplerParameterfv(sampler, pname, floatValue)
				value = floatValue[0]
			}
			TEXTURE_COMPARE_FUNC, TEXTURE_COMPARE_MODE, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_R, TEXTURE_WRAP_S, TEXTURE_WRAP_T -> {
				val intValue = IntBuffer.allocate(1)
				GLES30.glGetSamplerParameteriv(sampler, pname, intValue)
				value = intValue[0]
			}
			else -> value = null
		}

		return value
	}

	fun getSyncParameter(sync: Int, pname: Int): Any? {
		val value: Any?
		ensureContextIsCurrent()
		when (pname) {
			GLES30.GL_OBJECT_TYPE, GLES30.GL_SYNC_STATUS, GLES30.GL_SYNC_CONDITION, GLES30.GL_SYNC_FLAGS -> {
				val values = IntBuffer.allocate(1)
				val length = IntBuffer.allocate(1)
				GLES30.glGetSynciv(sync.toLong(), pname, 1, length, values)
				value = values[0]
			}
			else -> value = null
		}

		return value
	}

	fun getTransformFeedbackVarying(program: Int, index: Int): Any? {
		val info: WebGLActiveInfo?
		ensureContextIsCurrent()
		val maxIndex = IntBuffer.allocate(1)
		GLES30.glGetProgramiv(program, GLES30.GL_TRANSFORM_FEEDBACK_VARYINGS, maxIndex)
		if (index >= maxIndex[0]) {
			info = null
			return info
		}
		val maxNameLength = IntArray(1)
		GLES30.glGetProgramiv(
			program, GLES30.GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
			maxNameLength, 0
		)
		if (maxNameLength[0] <= 0) {
			info = null
			return info
		}
		var name = ByteArray(maxNameLength[0])
		val length = IntArray(1)
		val size = IntArray(1)
		val type = IntArray(1)
		GLES30.glGetTransformFeedbackVarying(
			program,
			index,
			maxNameLength[0],
			length,
			0,
			size,
			0,
			type,
			0,
			name,
			0
		)
		if (length[0] == 0 || size[0] == 0 || type[0] == 0) {
			info = null
			return info
		}
		name = name.copyOfRange(0, length[0])
		val nameValue = String(name)
		info = WebGLActiveInfo(nameValue, size[0], type[0])

		return info
	}

	fun getUniformBlockIndex(program: Int, uniformBlockName: String?): Int {
		ensureContextIsCurrent()
		return GLES30.glGetUniformBlockIndex(program, uniformBlockName)
	}

	fun getUniformIndices(program: Int, uniformNames: Array<String?>): IntArray {
		ensureContextIsCurrent()
		val indices = IntArray(uniformNames.size)
		GLES30.glGetUniformIndices(program, uniformNames, IntBuffer.wrap(indices))
		return indices
	}

	fun invalidateFramebuffer(target: Int, attachments: IntArray) {
		ensureContextIsCurrent()
		GLES30.glInvalidateFramebuffer(target, attachments.size, IntBuffer.wrap(attachments))

	}

	fun invalidateSubFramebuffer(
		target: Int,
		attachments: IntArray,
		x: Int,
		y: Int,
		width: Int,
		height: Int
	) {
		ensureContextIsCurrent()
		GLES30.glInvalidateSubFramebuffer(
			target,
			attachments.size,
			IntBuffer.wrap(attachments),
			x,
			y,
			width,
			height
		)

	}

	fun isQuery(query: Int): Boolean {
		ensureContextIsCurrent()
		return GLES30.glIsQuery(query)
	}

	fun isSampler(sampler: Int): Boolean {
		ensureContextIsCurrent()
		return GLES30.glIsSampler(sampler)
	}

	fun isSync(sync: Int): Boolean {
		ensureContextIsCurrent()
		return GLES30.glIsSync(sync.toLong())
	}

	fun isTransformFeedback(transformFeedback: Int): Boolean {
		ensureContextIsCurrent()
		return GLES30.glIsTransformFeedback(transformFeedback)
	}

	fun isVertexArray(vertexArray: Int): Boolean {
		ensureContextIsCurrent()
		return GLES30.glIsVertexArray(vertexArray)
	}

	fun pauseTransformFeedback() {
		ensureContextIsCurrent()
		GLES30.glPauseTransformFeedback()

	}

	fun readBuffer(src: Int) {
		ensureContextIsCurrent()
		GLES30.glReadBuffer(src)

	}

	fun renderbufferStorageMultisample(
		target: Int,
		samples: Int,
		internalFormat: Int,
		width: Int,
		height: Int
	) {
		ensureContextIsCurrent()
		GLES30.glRenderbufferStorageMultisample(target, samples, internalFormat, width, height)

	}

	fun resumeTransformFeedback() {
		ensureContextIsCurrent()
		GLES30.glResumeTransformFeedback()

	}

	fun samplerParameteri(sampler: Int, pname: Int, param: Int) {
		ensureContextIsCurrent()
		GLES30.glSamplerParameteri(sampler, pname, param)

	}

	fun samplerParameterf(sampler: Int, pname: Int, param: Float) {
		ensureContextIsCurrent()
		GLES30.glSamplerParameterf(sampler, pname, param)

	}

	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		offset: Int
	) {
		ensureContextIsCurrent()
		GLES30.glTexImage3D(
			target,
			level,
			internalformat,
			width,
			height,
			depth,
			border,
			format,
			type,
			offset
		)

	}


	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: ByteBuffer?
	) {
		ensureContextIsCurrent()
		source?.let {
			if (it.isDirect) {
				nativeTexImage3DBuffer(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					it,
					flipYWebGL
				)
			} else {
				nativeTexImage3DByteArray(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					it.array(),
					flipYWebGL
				)
			}
		} ?: run {
			GLES30.glTexImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: ShortBuffer?
	) {
		ensureContextIsCurrent()
		source?.let {
			if (it.isDirect) {
				nativeTexImage3DBuffer(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					it,
					flipYWebGL
				)
			} else {
				nativeTexImage3DShortArray(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					it.array(),
					flipYWebGL
				)
			}
		} ?: run {
			GLES30.glTexImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: IntBuffer?
	) {
		ensureContextIsCurrent()
		source?.let {
			if (it.isDirect) {
				nativeTexImage3DBuffer(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					it,
					flipYWebGL
				)
			} else {
				nativeTexImage3DIntArray(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					it.array(),
					flipYWebGL
				)
			}
		} ?: run {
			GLES30.glTexImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: LongBuffer?
	) {
		ensureContextIsCurrent()
		source?.let {
			if (it.isDirect) {
				nativeTexImage3DBuffer(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					it,
					flipYWebGL
				)
			} else {
				nativeTexImage3DLongArray(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					it.array(),
					flipYWebGL
				)
			}
		} ?: run {
			GLES30.glTexImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: FloatBuffer?
	) {
		ensureContextIsCurrent()
		source?.let {
			if (it.isDirect) {
				nativeTexImage3DBuffer(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					it,
					flipYWebGL
				)
			} else {
				nativeTexImage3DFloatArray(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					it.array(),
					flipYWebGL
				)
			}
		} ?: run {
			GLES30.glTexImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: DoubleBuffer?
	) {
		ensureContextIsCurrent()
		source?.let {
			if (it.isDirect) {
				nativeTexImage3DBuffer(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					it,
					flipYWebGL
				)
			} else {
				nativeTexImage3DDoubleArray(
					target,
					level,
					internalformat,
					width,
					height,
					depth,
					border,
					format,
					type,
					it.array(),
					flipYWebGL
				)
			}
		} ?: run {
			GLES30.glTexImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage3DByteBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: ByteBuffer?
	) {
		texImage3D(target, level, internalformat, width, height, depth, border, format, type, source)
	}


	fun texImage3DShortBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: ShortBuffer?
	) {
		texImage3D(target, level, internalformat, width, height, depth, border, format, type, source)
	}

	fun texImage3DIntBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: IntBuffer?
	) {
		texImage3D(target, level, internalformat, width, height, depth, border, format, type, source)
	}

	fun texImage3DLongBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: LongBuffer?
	) {
		texImage3D(target, level, internalformat, width, height, depth, border, format, type, source)
	}

	fun texImage3DFloatBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: FloatBuffer?
	) {
		texImage3D(target, level, internalformat, width, height, depth, border, format, type, source)
	}

	fun texImage3DDoubleBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: DoubleBuffer?
	) {
		texImage3D(target, level, internalformat, width, height, depth, border, format, type, source)
	}

	fun texImage3DByte(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: ByteArray?
	) {
		texImage3D(target, level, internalformat, width, height, depth, border, format, type, source)
	}


	fun texImage3DShort(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: ShortArray?
	) {
		texImage3D(target, level, internalformat, width, height, depth, border, format, type, source)
	}

	fun texImage3DInt(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: IntArray?
	) {
		texImage3D(target, level, internalformat, width, height, depth, border, format, type, source)
	}

	fun texImage3DLong(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: LongArray?
	) {
		texImage3D(target, level, internalformat, width, height, depth, border, format, type, source)
	}

	fun texImage3DFloat(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: FloatArray?
	) {
		texImage3D(target, level, internalformat, width, height, depth, border, format, type, source)
	}

	fun texImage3DDouble(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: DoubleArray?
	) {
		texImage3D(target, level, internalformat, width, height, depth, border, format, type, source)
	}

	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: ByteArray?
	) {
		ensureContextIsCurrent()
		source?.let {
			nativeTexImage3DByteArray(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: ShortArray?
	) {
		ensureContextIsCurrent()
		source?.let {
			nativeTexImage3DShortArray(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: IntArray?
	) {
		ensureContextIsCurrent()
		source?.let {
			nativeTexImage3DIntArray(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: LongArray?
	) {
		ensureContextIsCurrent()
		source?.let {
			nativeTexImage3DLongArray(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: FloatArray?
	) {
		ensureContextIsCurrent()
		source?.let {
			nativeTexImage3DFloatArray(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				null
			)
		}

	}

	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: DoubleArray?
	) {
		ensureContextIsCurrent()
		source?.let {
			nativeTexImage3DDoubleArray(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexImage3D(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				null
			)
		}

	}

	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: Bitmap
	) {
		ensureContextIsCurrent()
		nativeTexImage3DBitmap(
			target,
			level,
			internalformat,
			width,
			height,
			depth,
			border,
			format,
			type,
			source,
			flipYWebGL
		)

	}

	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		source: TNSCanvas
	) {
		val lock = CountDownLatch(1)


		var ss: ByteBuffer? = null
		val bitmap: Bitmap?
		if (!source.renderer.softwareRenderer) {
			bitmap = source.renderer.surface?.bitmap
			if (bitmap == null) {
				ss = source.snapshot()
			}
		} else {
			bitmap = source.renderer.cpuView?.view
			source.flush()
		}

		ensureContextIsCurrent()
		if (bitmap != null) {
			nativeTexImage3DBitmap(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				bitmap,
				flipYWebGL
			)
		}

		if (ss != null) {
			nativeTexImage3DBuffer(
				target,
				level,
				internalformat,
				width,
				height,
				depth,
				border,
				format,
				type,
				ss,
				flipYWebGL
			)
		}

	}

	fun texImage3D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int,
		border: Int,
		format: Int,
		type: Int,
		asset: TNSImageAsset
	) {
		ensureContextIsCurrent()
		nativeTexImage3DAsset(
			target,
			level,
			internalformat,
			width,
			height,
			depth,
			border,
			format,
			type,
			asset.nativeImageAsset,
			flipYWebGL
		)

	}

	fun texStorage2D(target: Int, levels: Int, internalformat: Int, width: Int, height: Int) {
		ensureContextIsCurrent()
		GLES30.glTexStorage2D(target, levels, internalformat, width, height)

	}

	fun texStorage3D(
		target: Int,
		levels: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		depth: Int
	) {
		ensureContextIsCurrent()
		GLES30.glTexStorage3D(target, levels, internalformat, width, height, depth)

	}

	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		offset: Int
	) {
		ensureContextIsCurrent()
		GLES30.glTexSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			offset
		)

	}

	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: Bitmap
	) {
		ensureContextIsCurrent()
		nativeTexSubImage3DBitmap(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			flipYWebGL
		)

	}

	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: TNSCanvas
	) {
		val lock = CountDownLatch(1)

		var ss: ByteBuffer? = null
		val bitmap: Bitmap?
		if (!srcData.renderer.softwareRenderer) {
			bitmap = srcData.renderer.surface?.bitmap
			if (bitmap == null) {
				ss = srcData.snapshot()
			}
		} else {
			bitmap = srcData.renderer.cpuView?.view
			srcData.flush()
		}

		ensureContextIsCurrent()

		if (bitmap != null) {
			nativeTexSubImage3DBitmap(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				bitmap,
				flipYWebGL
			)
		}

		if (ss != null) {
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				ss,
				flipYWebGL
			)
		}


	}


	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		asset: TNSImageAsset
	) {
		ensureContextIsCurrent()
		nativeTexSubImage3DAsset(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			asset.nativeImageAsset,
			flipYWebGL
		)

	}


	fun texSubImage3DByte(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: ByteArray?,
		srcOffset: Int = 0
	) {
		texSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			srcOffset
		)
	}


	fun texSubImage3DShort(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: ShortArray?,
		srcOffset: Int = 0
	) {
		texSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			srcOffset
		)
	}

	fun texSubImage3DInt(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: IntArray?,
		srcOffset: Int = 0
	) {
		texSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			srcOffset
		)
	}


	fun texSubImage3DLong(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: LongArray?,
		srcOffset: Int = 0
	) {
		texSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			srcOffset
		)
	}


	fun texSubImage3DFloat(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: FloatArray?,
		srcOffset: Int = 0
	) {
		texSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			srcOffset
		)
	}


	fun texSubImage3DDouble(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: DoubleArray?,
		srcOffset: Int = 0
	) {
		texSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			srcOffset
		)
	}


	fun texSubImage3DByteBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: ByteBuffer?,
		srcOffset: Int = 0
	) {
		texSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			srcOffset
		)
	}


	fun texSubImage3DShortBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: ShortBuffer?,
		srcOffset: Int = 0
	) {
		texSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			srcOffset
		)
	}

	fun texSubImage3DIntBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: IntBuffer?,
		srcOffset: Int = 0
	) {
		texSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			srcOffset
		)
	}


	fun texSubImage3DLongBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: LongBuffer?,
		srcOffset: Int = 0
	) {
		texSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			srcOffset
		)
	}


	fun texSubImage3DFloatBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: FloatBuffer?,
		srcOffset: Int = 0
	) {
		texSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			srcOffset
		)
	}


	fun texSubImage3DDoubleBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: DoubleBuffer?,
		srcOffset: Int = 0
	) {
		texSubImage3D(
			target,
			level,
			xoffset,
			yoffset,
			zoffset,
			width,
			height,
			depth,
			format,
			type,
			srcData,
			srcOffset
		)
	}


	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: ByteBuffer?,
		srcOffset: Int = 0
	) {
		ensureContextIsCurrent()
		srcData?.let {
			it.position(srcOffset)
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				null
			)
		}

	}


	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: ShortBuffer?,
		srcOffset: Int = 0
	) {
		ensureContextIsCurrent()
		srcData?.let {
			it.position(srcOffset)
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				null
			)
		}

	}


	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: IntBuffer?,
		srcOffset: Int = 0
	) {
		ensureContextIsCurrent()
		srcData?.let {
			it.position(srcOffset)
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				null
			)
		}

	}


	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: LongBuffer?,
		srcOffset: Int = 0
	) {
		ensureContextIsCurrent()
		srcData?.let {
			it.position(srcOffset)
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				null
			)
		}

	}


	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: FloatBuffer?,
		srcOffset: Int = 0
	) {
		ensureContextIsCurrent()
		srcData?.let {
			it.position(srcOffset)
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				null
			)
		}

	}


	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: DoubleBuffer?,
		srcOffset: Int = 0
	) {
		ensureContextIsCurrent()
		srcData?.let {
			it.position(srcOffset)
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				null
			)
		}

	}

	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: ByteArray?,
		srcOffset: Int = 0
	) {
		ensureContextIsCurrent()
		srcData?.let {
			val size = it.size
			val buffer = ByteBuffer.allocateDirect(size).order(ByteOrder.nativeOrder())
			buffer.put(it)
			buffer.rewind()
			buffer.position(srcOffset)
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				buffer,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				null
			)
		}

	}


	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: ShortArray?,
		srcOffset: Int = 0
	) {
		ensureContextIsCurrent()
		srcData?.let {
			val size = it.size * SIZE_OF_SHORT
			val buffer = ByteBuffer.allocateDirect(size).order(ByteOrder.nativeOrder())
			buffer.asShortBuffer().put(it)
			buffer.rewind()
			buffer.position(srcOffset)
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				buffer,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				null
			)
		}

	}

	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: IntArray?,
		srcOffset: Int = 0
	) {
		ensureContextIsCurrent()
		srcData?.let {
			val size = it.size * SIZE_OF_INT
			val buffer = ByteBuffer.allocateDirect(size).order(ByteOrder.nativeOrder())
			buffer.asIntBuffer().put(it)
			buffer.rewind()
			buffer.position(srcOffset)
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				buffer,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				null
			)
		}

	}


	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: LongArray?,
		srcOffset: Int = 0
	) {
		ensureContextIsCurrent()
		srcData?.let {
			val size = it.size * SIZE_OF_LONG
			val buffer = ByteBuffer.allocateDirect(size).order(ByteOrder.nativeOrder())
			buffer.asLongBuffer().put(it)
			buffer.rewind()
			buffer.position(srcOffset)
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				buffer,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				null
			)
		}

	}


	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: FloatArray?,
		srcOffset: Int = 0
	) {
		ensureContextIsCurrent()
		srcData?.let {
			val size = it.size * SIZE_OF_FLOAT
			val buffer = ByteBuffer.allocateDirect(size).order(ByteOrder.nativeOrder())
			buffer.asFloatBuffer().put(it)
			buffer.rewind()
			buffer.position(srcOffset)
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				buffer,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				null
			)
		}

	}

	fun texSubImage3D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		zoffset: Int,
		width: Int,
		height: Int,
		depth: Int,
		format: Int,
		type: Int,
		srcData: DoubleArray?,
		srcOffset: Int = 0
	) {
		ensureContextIsCurrent()
		srcData?.let {
			val size = it.size * SIZE_OF_DOUBLE
			val buffer = ByteBuffer.allocateDirect(size).order(ByteOrder.nativeOrder())
			buffer.asDoubleBuffer().put(it)
			buffer.rewind()
			buffer.position(srcOffset)
			nativeTexSubImage3DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				buffer,
				flipYWebGL
			)
		} ?: run {
			GLES30.glTexSubImage3D(
				target,
				level,
				xoffset,
				yoffset,
				zoffset,
				width,
				height,
				depth,
				format,
				type,
				null
			)
		}

	}

	fun transformFeedbackVaryings(program: Int, varyings: Array<String?>?, bufferMode: Int) {
		ensureContextIsCurrent()
		GLES30.glTransformFeedbackVaryings(program, varyings, bufferMode)

	}

	fun uniform1ui(location: Int, v0: Int) {
		ensureContextIsCurrent()
		GLES30.glUniform1ui(location, v0)

	}

	fun uniform2ui(location: Int, v0: Int, v1: Int) {
		ensureContextIsCurrent()
		GLES30.glUniform2ui(location, v0, v1)

	}

	fun uniform3ui(location: Int, v0: Int, v1: Int, v2: Int) {
		ensureContextIsCurrent()
		GLES30.glUniform3ui(location, v0, v1, v2)

	}

	fun uniform4ui(location: Int, v0: Int, v1: Int, v2: Int, v3: Int) {
		ensureContextIsCurrent()
		GLES30.glUniform4ui(location, v0, v1, v2, v3)

	}

	fun uniform1uiv(location: Int, data: IntArray) {
		ensureContextIsCurrent()
		val count = data.size / 1
		GLES30.glUniform1uiv(location, count, IntBuffer.wrap(data))

	}

	fun uniform2uiv(location: Int, data: IntArray) {
		ensureContextIsCurrent()
		val count = data.size / 2
		GLES30.glUniform2uiv(location, count, IntBuffer.wrap(data))

	}

	fun uniform3uiv(location: Int, data: IntArray) {
		ensureContextIsCurrent()
		val count = data.size / 3
		GLES30.glUniform3uiv(location, count, IntBuffer.wrap(data))

	}

	fun uniform4uiv(location: Int, data: IntArray) {
		ensureContextIsCurrent()
		val count = data.size / 4
		GLES30.glUniform4uiv(location, count, IntBuffer.wrap(data))

	}


	fun uniform1uivBuffer(location: Int, data: IntBuffer) {
		ensureContextIsCurrent()
		val count = data.capacity() / 1
		GLES30.glUniform1uiv(location, count, data)

	}

	fun uniform2uivBuffer(location: Int, data: IntBuffer) {
		ensureContextIsCurrent()
		val count = data.capacity() / 2
		GLES30.glUniform2uiv(location, count, data)

	}

	fun uniform3uivBuffer(location: Int, data: IntBuffer) {
		ensureContextIsCurrent()
		val count = data.capacity() / 3
		GLES30.glUniform3uiv(location, count, data)

	}

	fun uniform4uivBuffer(location: Int, data: IntBuffer) {
		ensureContextIsCurrent()
		val count = data.capacity() / 4
		GLES30.glUniform4uiv(location, count, data)

	}

	fun uniformBlockBinding(program: Int, uniformBlockIndex: Int, uniformBlockBinding: Int) {
		ensureContextIsCurrent()
		GLES30.glUniformBlockBinding(program, uniformBlockIndex, uniformBlockBinding)

	}

	fun uniformMatrix3x2fv(location: Int, transpose: Boolean, data: FloatArray) {
		ensureContextIsCurrent()
		val count = data.size / 6
		GLES30.glUniformMatrix3x2fv(location, count, transpose, FloatBuffer.wrap(data))

	}

	fun uniformMatrix4x2fv(location: Int, transpose: Boolean, data: FloatArray) {
		ensureContextIsCurrent()
		val count = data.size / 8
		GLES30.glUniformMatrix4x2fv(location, count, transpose, FloatBuffer.wrap(data))

	}

	fun uniformMatrix2x3fv(location: Int, transpose: Boolean, data: FloatArray) {
		ensureContextIsCurrent()
		val count = data.size / 6
		GLES30.glUniformMatrix2x3fv(location, count, transpose, FloatBuffer.wrap(data))

	}

	fun uniformMatrix4x3fv(location: Int, transpose: Boolean, data: FloatArray) {
		ensureContextIsCurrent()
		val count = data.size / 12
		GLES30.glUniformMatrix4x3fv(location, count, transpose, FloatBuffer.wrap(data))

	}

	fun uniformMatrix2x4fv(location: Int, transpose: Boolean, data: FloatArray) {
		ensureContextIsCurrent()
		val count = data.size / 8
		GLES30.glUniformMatrix2x4fv(location, count, transpose, FloatBuffer.wrap(data))

	}

	fun uniformMatrix3x4fv(location: Int, transpose: Boolean, data: FloatArray) {
		ensureContextIsCurrent()
		val count = data.size / 12
		GLES30.glUniformMatrix3x4fv(location, count, transpose, FloatBuffer.wrap(data))

	}


	fun uniformMatrix3x2fvBuffer(location: Int, transpose: Boolean, data: FloatBuffer) {
		ensureContextIsCurrent()
		val count = data.capacity() / 6
		GLES30.glUniformMatrix3x2fv(location, count, transpose, data)

	}

	fun uniformMatrix4x2fvBuffer(location: Int, transpose: Boolean, data: FloatBuffer) {
		ensureContextIsCurrent()
		val count = data.capacity() / 8
		GLES30.glUniformMatrix4x2fv(location, count, transpose, data)

	}

	fun uniformMatrix2x3fvBuffer(location: Int, transpose: Boolean, data: FloatBuffer) {
		ensureContextIsCurrent()
		val count = data.capacity() / 6
		GLES30.glUniformMatrix2x3fv(location, count, transpose, data)

	}

	fun uniformMatrix4x3fvBuffer(location: Int, transpose: Boolean, data: FloatBuffer) {
		ensureContextIsCurrent()
		val count = data.capacity() / 12
		GLES30.glUniformMatrix4x3fv(location, count, transpose, data)

	}

	fun uniformMatrix2x4fvBuffer(location: Int, transpose: Boolean, data: FloatBuffer) {
		ensureContextIsCurrent()
		val count = data.capacity() / 8
		GLES30.glUniformMatrix2x4fv(location, count, transpose, data)

	}

	fun uniformMatrix3x4fvBuffer(location: Int, transpose: Boolean, data: FloatBuffer) {
		ensureContextIsCurrent()
		val count = data.capacity() / 12
		GLES30.glUniformMatrix3x4fv(location, count, transpose, data)

	}


	fun vertexAttribDivisor(index: Int, divisor: Int) {
		ensureContextIsCurrent()
		GLES30.glVertexAttribDivisor(index, divisor)

	}

	fun vertexAttribI4i(index: Int, v0: Int, v1: Int, v2: Int, v3: Int) {
		ensureContextIsCurrent()
		GLES30.glVertexAttribI4i(index, v0, v1, v2, v3)

	}

	fun vertexAttribI4ui(index: Int, v0: Int, v1: Int, v2: Int, v3: Int) {
		ensureContextIsCurrent()
		GLES30.glVertexAttribI4ui(index, v0, v1, v2, v3)

	}

	fun vertexAttribI4iv(index: Int, value: IntArray?) {
		ensureContextIsCurrent()
		GLES30.glVertexAttribI4iv(index, IntBuffer.wrap(value))

	}

	fun vertexAttribI4uiv(index: Int, value: IntArray?) {
		ensureContextIsCurrent()
		GLES30.glVertexAttribI4uiv(index, IntBuffer.wrap(value))

	}


	fun vertexAttribI4ivBuffer(index: Int, value: IntBuffer?) {
		ensureContextIsCurrent()
		GLES30.glVertexAttribI4iv(index, value)
	}

	fun vertexAttribI4uivBuffer(index: Int, value: IntBuffer?) {
		ensureContextIsCurrent()
		GLES30.glVertexAttribI4uiv(index, value)
	}

	/* Getting GL parameter information */
	val READ_BUFFER = GLES30.GL_READ_BUFFER
	val UNPACK_ROW_LENGTH = GLES30.GL_UNPACK_ROW_LENGTH
	val UNPACK_SKIP_ROWS = GLES30.GL_UNPACK_SKIP_ROWS
	val UNPACK_SKIP_PIXELS = GLES30.GL_UNPACK_SKIP_PIXELS
	val PACK_ROW_LENGTH = GLES30.GL_PACK_ROW_LENGTH
	val PACK_SKIP_ROWS = GLES30.GL_PACK_SKIP_ROWS
	val PACK_SKIP_PIXELS = GLES30.GL_PACK_SKIP_PIXELS
	val TEXTURE_BINDING_3D = GLES30.GL_TEXTURE_BINDING_3D
	val UNPACK_SKIP_IMAGES = GLES30.GL_UNPACK_SKIP_IMAGES
	val UNPACK_IMAGE_HEIGHT = GLES30.GL_UNPACK_IMAGE_HEIGHT
	val MAX_3D_TEXTURE_SIZE = GLES30.GL_MAX_3D_TEXTURE_SIZE
	val MAX_ELEMENTS_VERTICES = GLES30.GL_MAX_ELEMENTS_VERTICES
	val MAX_ELEMENTS_INDICES = GLES30.GL_MAX_ELEMENTS_INDICES
	val MAX_TEXTURE_LOD_BIAS = GLES30.GL_MAX_TEXTURE_LOD_BIAS
	val MAX_FRAGMENT_UNIFORM_COMPONENTS = GLES30.GL_MAX_FRAGMENT_UNIFORM_COMPONENTS
	val MAX_VERTEX_UNIFORM_COMPONENTS = GLES30.GL_MAX_VERTEX_UNIFORM_COMPONENTS
	val MAX_ARRAY_TEXTURE_LAYERS = GLES30.GL_MAX_ARRAY_TEXTURE_LAYERS
	val MIN_PROGRAM_TEXEL_OFFSET = GLES30.GL_MIN_PROGRAM_TEXEL_OFFSET
	val MAX_PROGRAM_TEXEL_OFFSET = GLES30.GL_MAX_PROGRAM_TEXEL_OFFSET
	val MAX_VARYING_COMPONENTS = GLES30.GL_MAX_VARYING_COMPONENTS
	val FRAGMENT_SHADER_DERIVATIVE_HINT = GLES30.GL_FRAGMENT_SHADER_DERIVATIVE_HINT
	val RASTERIZER_DISCARD = GLES30.GL_RASTERIZER_DISCARD
	val VERTEX_ARRAY_BINDING = GLES30.GL_VERTEX_ARRAY_BINDING
	val MAX_VERTEX_OUTPUT_COMPONENTS = GLES30.GL_MAX_VERTEX_OUTPUT_COMPONENTS
	val MAX_FRAGMENT_INPUT_COMPONENTS = GLES30.GL_MAX_FRAGMENT_INPUT_COMPONENTS
	val MAX_SERVER_WAIT_TIMEOUT = GLES30.GL_MAX_SERVER_WAIT_TIMEOUT
	val MAX_ELEMENT_INDEX = GLES30.GL_MAX_ELEMENT_INDEX

	/* Getting GL parameter information */ /* Textures */
	val RED = GLES30.GL_RED
	val RGB8 = GLES30.GL_RGB8
	val RGBA8 = GLES30.GL_RGBA8
	val RGB10_A2 = GLES30.GL_RGB10_A2
	val TEXTURE_3D = GLES30.GL_TEXTURE_3D
	val TEXTURE_WRAP_R = GLES30.GL_TEXTURE_WRAP_R
	val TEXTURE_MIN_LOD = GLES30.GL_TEXTURE_MIN_LOD
	val TEXTURE_MAX_LOD = GLES30.GL_TEXTURE_MAX_LOD
	val TEXTURE_BASE_LEVEL = GLES30.GL_TEXTURE_BASE_LEVEL
	val TEXTURE_MAX_LEVEL = GLES30.GL_TEXTURE_MAX_LEVEL
	val TEXTURE_COMPARE_MODE = GLES30.GL_TEXTURE_COMPARE_MODE
	val TEXTURE_COMPARE_FUNC = GLES30.GL_TEXTURE_COMPARE_FUNC
	val SRGB = GLES30.GL_SRGB
	val SRGB8 = GLES30.GL_SRGB8
	val SRGB8_ALPHA8 = GLES30.GL_SRGB8_ALPHA8
	val COMPARE_REF_TO_TEXTURE = GLES30.GL_COMPARE_REF_TO_TEXTURE
	val RGBA32F = GLES30.GL_RGBA32F
	val RGB32F = GLES30.GL_RGB32F
	val RGBA16F = GLES30.GL_RGBA16F
	val RGB16F = GLES30.GL_RGB16F
	val TEXTURE_2D_ARRAY = GLES30.GL_TEXTURE_2D_ARRAY
	val TEXTURE_BINDING_2D_ARRAY = GLES30.GL_TEXTURE_BINDING_2D_ARRAY
	val R11F_G11F_B10F = GLES30.GL_R11F_G11F_B10F
	val RGB9_E5 = GLES30.GL_RGB9_E5
	val RGBA32UI = GLES30.GL_RGBA32UI
	val RGB32UI = GLES30.GL_RGB32UI
	val RGBA16UI = GLES30.GL_RGBA16UI
	val RGB16UI = GLES30.GL_RGB16UI
	val RGBA8UI = GLES30.GL_RGBA8UI
	val RGB8UI = GLES30.GL_RGB8UI
	val RGBA32I = GLES30.GL_RGBA32I
	val RGB32I = GLES30.GL_RGB32I
	val RGBA16I = GLES30.GL_RGBA16I
	val RGB16I = GLES30.GL_RGB16I
	val RGBA8I = GLES30.GL_RGBA8I
	val RGB8I = GLES30.GL_RGB8I
	val RED_INTEGER = GLES30.GL_RED_INTEGER
	val RGB_INTEGER = GLES30.GL_RGB_INTEGER
	val RGBA_INTEGER = GLES30.GL_RGBA_INTEGER
	val R8 = GLES30.GL_R8
	val RG8 = GLES30.GL_RG8
	val R16F = GLES30.GL_R16F
	val R32F = GLES30.GL_R32F
	val RG16F = GLES30.GL_RG16F
	val RG32F = GLES30.GL_RG32F
	val R8I = GLES30.GL_R8I
	val R8UI = GLES30.GL_R8UI
	val R16I = GLES30.GL_R16I
	val R16UI = GLES30.GL_R16UI
	val R32I = GLES30.GL_R32I
	val R32UI = GLES30.GL_R32UI
	val RG8I = GLES30.GL_RG8I
	val RG8UI = GLES30.GL_RG8UI
	val RG16I = GLES30.GL_RG16I
	val RG16UI = GLES30.GL_RG16UI
	val RG32I = GLES30.GL_RG32I
	val RG32UI = GLES30.GL_RG32UI
	val R8_SNORM = GLES30.GL_R8_SNORM
	val RG8_SNORM = GLES30.GL_RG8_SNORM
	val RGB8_SNORM = GLES30.GL_RGB8_SNORM
	val RGBA8_SNORM = GLES30.GL_RGBA8_SNORM
	val RGB10_A2UI = GLES30.GL_RGB10_A2UI
	val TEXTURE_IMMUTABLE_FORMAT = GLES30.GL_TEXTURE_IMMUTABLE_FORMAT
	val TEXTURE_IMMUTABLE_LEVELS = GLES30.GL_TEXTURE_IMMUTABLE_LEVELS

	/* Textures */ /* Pixel types */
	val UNSIGNED_INT_2_10_10_10_REV = GLES30.GL_UNSIGNED_INT_2_10_10_10_REV
	val UNSIGNED_INT_10F_11F_11F_REV = GLES30.GL_UNSIGNED_INT_10F_11F_11F_REV
	val UNSIGNED_INT_5_9_9_9_REV = GLES30.GL_UNSIGNED_INT_5_9_9_9_REV
	val FLOAT_32_UNSIGNED_INT_24_8_REV = GLES30.GL_FLOAT_32_UNSIGNED_INT_24_8_REV
	val UNSIGNED_INT_24_8 = GLES30.GL_UNSIGNED_INT_24_8
	val HALF_FLOAT = GLES30.GL_HALF_FLOAT
	val RG = GLES30.GL_RG
	val RG_INTEGER = GLES30.GL_RG_INTEGER
	val INT_2_10_10_10_REV = GLES30.GL_INT_2_10_10_10_REV

	/* Pixel types */ /* Queries */
	val QUERY_RESULT_AVAILABLE = GLES30.GL_QUERY_RESULT_AVAILABLE
	val QUERY_RESULT = GLES30.GL_QUERY_RESULT
	val CURRENT_QUERY = GLES30.GL_CURRENT_QUERY
	val ANY_SAMPLES_PASSED = GLES30.GL_ANY_SAMPLES_PASSED
	val ANY_SAMPLES_PASSED_CONSERVATIVE = GLES30.GL_ANY_SAMPLES_PASSED_CONSERVATIVE

	/* Queries */ /* Draw buffers */
	val MAX_DRAW_BUFFERS = GLES30.GL_MAX_DRAW_BUFFERS
	val DRAW_BUFFER0 = GLES30.GL_DRAW_BUFFER0
	val DRAW_BUFFER1 = GLES30.GL_DRAW_BUFFER1
	val DRAW_BUFFER2 = GLES30.GL_DRAW_BUFFER2
	val DRAW_BUFFER3 = GLES30.GL_DRAW_BUFFER3
	val DRAW_BUFFER4 = GLES30.GL_DRAW_BUFFER4
	val DRAW_BUFFER5 = GLES30.GL_DRAW_BUFFER5
	val DRAW_BUFFER6 = GLES30.GL_DRAW_BUFFER6
	val DRAW_BUFFER7 = GLES30.GL_DRAW_BUFFER7
	val DRAW_BUFFER8 = GLES30.GL_DRAW_BUFFER8
	val DRAW_BUFFER9 = GLES30.GL_DRAW_BUFFER9
	val DRAW_BUFFER10 = GLES30.GL_DRAW_BUFFER10
	val DRAW_BUFFER11 = GLES30.GL_DRAW_BUFFER11
	val DRAW_BUFFER12 = GLES30.GL_DRAW_BUFFER12
	val DRAW_BUFFER13 = GLES30.GL_DRAW_BUFFER13
	val DRAW_BUFFER14 = GLES30.GL_DRAW_BUFFER14
	val DRAW_BUFFER15 = GLES30.GL_DRAW_BUFFER15
	val MAX_COLOR_ATTACHMENTS = GLES30.GL_MAX_COLOR_ATTACHMENTS
	val COLOR_ATTACHMENT1 = GLES30.GL_COLOR_ATTACHMENT1
	val COLOR_ATTACHMENT2 = GLES30.GL_COLOR_ATTACHMENT2
	val COLOR_ATTACHMENT3 = GLES30.GL_COLOR_ATTACHMENT3
	val COLOR_ATTACHMENT4 = GLES30.GL_COLOR_ATTACHMENT4
	val COLOR_ATTACHMENT5 = GLES30.GL_COLOR_ATTACHMENT5
	val COLOR_ATTACHMENT6 = GLES30.GL_COLOR_ATTACHMENT6
	val COLOR_ATTACHMENT7 = GLES30.GL_COLOR_ATTACHMENT7
	val COLOR_ATTACHMENT8 = GLES30.GL_COLOR_ATTACHMENT8
	val COLOR_ATTACHMENT9 = GLES30.GL_COLOR_ATTACHMENT9
	val COLOR_ATTACHMENT10 = GLES30.GL_COLOR_ATTACHMENT10
	val COLOR_ATTACHMENT11 = GLES30.GL_COLOR_ATTACHMENT11
	val COLOR_ATTACHMENT12 = GLES30.GL_COLOR_ATTACHMENT12
	val COLOR_ATTACHMENT13 = GLES30.GL_COLOR_ATTACHMENT13
	val COLOR_ATTACHMENT14 = GLES30.GL_COLOR_ATTACHMENT14
	val COLOR_ATTACHMENT15 = GLES30.GL_COLOR_ATTACHMENT15

	/* Draw buffers */ /* Samplers */
	val SAMPLER_3D = GLES30.GL_SAMPLER_3D
	val SAMPLER_2D_SHADOW = GLES30.GL_SAMPLER_2D_SHADOW
	val SAMPLER_2D_ARRAY = GLES30.GL_SAMPLER_2D_ARRAY
	val SAMPLER_2D_ARRAY_SHADOW = GLES30.GL_SAMPLER_2D_ARRAY_SHADOW
	val SAMPLER_CUBE_SHADOW = GLES30.GL_SAMPLER_CUBE_SHADOW
	val INT_SAMPLER_2D = GLES30.GL_INT_SAMPLER_2D
	val INT_SAMPLER_3D = GLES30.GL_INT_SAMPLER_3D
	val INT_SAMPLER_CUBE = GLES30.GL_INT_SAMPLER_CUBE
	val INT_SAMPLER_2D_ARRAY = GLES30.GL_INT_SAMPLER_2D_ARRAY
	val UNSIGNED_INT_SAMPLER_2D = GLES30.GL_UNSIGNED_INT_SAMPLER_2D
	val UNSIGNED_INT_SAMPLER_3D = GLES30.GL_UNSIGNED_INT_SAMPLER_3D
	val UNSIGNED_INT_SAMPLER_CUBE = GLES30.GL_UNSIGNED_INT_SAMPLER_CUBE
	val UNSIGNED_INT_SAMPLER_2D_ARRAY = GLES30.GL_UNSIGNED_INT_SAMPLER_2D_ARRAY
	val MAX_SAMPLES = GLES30.GL_MAX_SAMPLES
	val SAMPLER_BINDING = GLES30.GL_SAMPLER_BINDING

	/* Samplers */ /* Buffers */
	val PIXEL_PACK_BUFFER = GLES30.GL_PIXEL_PACK_BUFFER
	val PIXEL_UNPACK_BUFFER = GLES30.GL_PIXEL_UNPACK_BUFFER
	val PIXEL_PACK_BUFFER_BINDING = GLES30.GL_PIXEL_PACK_BUFFER_BINDING
	val PIXEL_UNPACK_BUFFER_BINDING = GLES30.GL_PIXEL_UNPACK_BUFFER_BINDING
	val COPY_READ_BUFFER = GLES30.GL_COPY_READ_BUFFER
	val COPY_WRITE_BUFFER = GLES30.GL_COPY_WRITE_BUFFER
	val COPY_READ_BUFFER_BINDING = GLES30.GL_COPY_READ_BUFFER_BINDING
	val COPY_WRITE_BUFFER_BINDING = GLES30.GL_COPY_WRITE_BUFFER_BINDING

	/* Buffers */ /* Data types */
	val FLOAT_MAT2x3 = GLES30.GL_FLOAT_MAT2x3
	val FLOAT_MAT2x4 = GLES30.GL_FLOAT_MAT2x4
	val FLOAT_MAT3x2 = GLES30.GL_FLOAT_MAT3x2
	val FLOAT_MAT3x4 = GLES30.GL_FLOAT_MAT3x4
	val FLOAT_MAT4x2 = GLES30.GL_FLOAT_MAT4x2
	val FLOAT_MAT4x3 = GLES30.GL_FLOAT_MAT4x3
	val UNSIGNED_INT_VEC2 = GLES30.GL_UNSIGNED_INT_VEC2
	val UNSIGNED_INT_VEC3 = GLES30.GL_UNSIGNED_INT_VEC3
	val UNSIGNED_INT_VEC4 = GLES30.GL_UNSIGNED_INT_VEC4
	val UNSIGNED_NORMALIZED = GLES30.GL_UNSIGNED_NORMALIZED
	val SIGNED_NORMALIZED = GLES30.GL_SIGNED_NORMALIZED

	/* Data types */ /* Vertex attributes */
	val VERTEX_ATTRIB_ARRAY_INTEGER = GLES30.GL_VERTEX_ATTRIB_ARRAY_INTEGER
	val VERTEX_ATTRIB_ARRAY_DIVISOR = GLES30.GL_VERTEX_ATTRIB_ARRAY_DIVISOR

	/* Vertex attributes */ /* Transform feedback */
	val TRANSFORM_FEEDBACK_BUFFER_MODE = GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_MODE
	val MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS =
		GLES30.GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS
	val TRANSFORM_FEEDBACK_VARYINGS = GLES30.GL_TRANSFORM_FEEDBACK_VARYINGS
	val TRANSFORM_FEEDBACK_BUFFER_START = GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_START
	val TRANSFORM_FEEDBACK_BUFFER_SIZE = GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_SIZE
	val TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN = GLES30.GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN
	val MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS =
		GLES30.GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS
	val MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS = GLES30.GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS
	val INTERLEAVED_ATTRIBS = GLES30.GL_INTERLEAVED_ATTRIBS
	val SEPARATE_ATTRIBS = GLES30.GL_SEPARATE_ATTRIBS
	val TRANSFORM_FEEDBACK_BUFFER = GLES30.GL_TRANSFORM_FEEDBACK_BUFFER
	val TRANSFORM_FEEDBACK_BUFFER_BINDING = GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_BINDING
	val TRANSFORM_FEEDBACK = GLES30.GL_TRANSFORM_FEEDBACK
	val TRANSFORM_FEEDBACK_PAUSED = GLES30.GL_TRANSFORM_FEEDBACK_PAUSED
	val TRANSFORM_FEEDBACK_ACTIVE = GLES30.GL_TRANSFORM_FEEDBACK_ACTIVE
	val TRANSFORM_FEEDBACK_BINDING = GLES30.GL_TRANSFORM_FEEDBACK_BINDING

	/* Transform feedback */ /* Framebuffers and renderbuffers */
	val FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING = GLES30.GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING
	val FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE
	val FRAMEBUFFER_ATTACHMENT_RED_SIZE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE
	val FRAMEBUFFER_ATTACHMENT_GREEN_SIZE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE
	val FRAMEBUFFER_ATTACHMENT_BLUE_SIZE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE
	val FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE
	val FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE
	val FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE
	val FRAMEBUFFER_DEFAULT = GLES30.GL_FRAMEBUFFER_DEFAULT
	val DEPTH24_STENCIL8 = GLES30.GL_DEPTH24_STENCIL8
	val DRAW_FRAMEBUFFER_BINDING = GLES30.GL_DRAW_FRAMEBUFFER_BINDING
	val READ_FRAMEBUFFER = GLES30.GL_READ_FRAMEBUFFER
	val DRAW_FRAMEBUFFER = GLES30.GL_DRAW_FRAMEBUFFER
	val READ_FRAMEBUFFER_BINDING = GLES30.GL_READ_FRAMEBUFFER_BINDING
	val RENDERBUFFER_SAMPLES = GLES30.GL_RENDERBUFFER_SAMPLES
	val FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER = GLES30.GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER
	val FRAMEBUFFER_INCOMPLETE_MULTISAMPLE = GLES30.GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE

	/* Framebuffers and renderbuffers */ /* Uniforms */
	val UNIFORM_BUFFER = GLES30.GL_UNIFORM_BUFFER
	val UNIFORM_BUFFER_BINDING = GLES30.GL_UNIFORM_BUFFER_BINDING
	val UNIFORM_BUFFER_START = GLES30.GL_UNIFORM_BUFFER_START
	val UNIFORM_BUFFER_SIZE = GLES30.GL_UNIFORM_BUFFER_SIZE
	val MAX_VERTEX_UNIFORM_BLOCKS = GLES30.GL_MAX_VERTEX_UNIFORM_BLOCKS
	val MAX_FRAGMENT_UNIFORM_BLOCKS = GLES30.GL_MAX_FRAGMENT_UNIFORM_BLOCKS
	val MAX_COMBINED_UNIFORM_BLOCKS = GLES30.GL_MAX_COMBINED_UNIFORM_BLOCKS
	val MAX_UNIFORM_BUFFER_BINDINGS = GLES30.GL_MAX_UNIFORM_BUFFER_BINDINGS
	val MAX_UNIFORM_BLOCK_SIZE = GLES30.GL_MAX_UNIFORM_BLOCK_SIZE
	val MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS = GLES30.GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS
	val MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS = GLES30.GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS
	val UNIFORM_BUFFER_OFFSET_ALIGNMENT = GLES30.GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT
	val ACTIVE_UNIFORM_BLOCKS = GLES30.GL_ACTIVE_UNIFORM_BLOCKS
	val UNIFORM_TYPE = GLES30.GL_UNIFORM_TYPE
	val UNIFORM_SIZE = GLES30.GL_UNIFORM_SIZE
	val UNIFORM_BLOCK_INDEX = GLES30.GL_UNIFORM_BLOCK_INDEX
	val UNIFORM_OFFSET = GLES30.GL_UNIFORM_OFFSET
	val UNIFORM_ARRAY_STRIDE = GLES30.GL_UNIFORM_ARRAY_STRIDE
	val UNIFORM_MATRIX_STRIDE = GLES30.GL_UNIFORM_MATRIX_STRIDE
	val UNIFORM_IS_ROW_MAJOR = GLES30.GL_UNIFORM_IS_ROW_MAJOR
	val UNIFORM_BLOCK_BINDING = GLES30.GL_UNIFORM_BLOCK_BINDING
	val UNIFORM_BLOCK_DATA_SIZE = GLES30.GL_UNIFORM_BLOCK_DATA_SIZE
	val UNIFORM_BLOCK_ACTIVE_UNIFORMS = GLES30.GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS
	val UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES = GLES30.GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES
	val UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER =
		GLES30.GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER
	val UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER =
		GLES30.GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER

	/* Uniforms */ /* Sync objects */
	val OBJECT_TYPE = GLES30.GL_OBJECT_TYPE
	val SYNC_CONDITION = GLES30.GL_SYNC_CONDITION
	val SYNC_STATUS = GLES30.GL_SYNC_STATUS
	val SYNC_FLAGS = GLES30.GL_SYNC_FLAGS
	val SYNC_FENCE = GLES30.GL_SYNC_FENCE
	val SYNC_GPU_COMMANDS_COMPLETE = GLES30.GL_SYNC_GPU_COMMANDS_COMPLETE
	val UNSIGNALED = GLES30.GL_UNSIGNALED
	val SIGNALED = GLES30.GL_SIGNALED
	val ALREADY_SIGNALED = GLES30.GL_ALREADY_SIGNALED
	val TIMEOUT_EXPIRED = GLES30.GL_TIMEOUT_EXPIRED
	val CONDITION_SATISFIED = GLES30.GL_CONDITION_SATISFIED
	val WAIT_FAILED = GLES30.GL_WAIT_FAILED
	val SYNC_FLUSH_COMMANDS_BIT = GLES30.GL_SYNC_FLUSH_COMMANDS_BIT

	/* Sync objects */ /* Miscellaneous constants */
	val COLOR = GLES30.GL_COLOR
	val DEPTH = GLES30.GL_DEPTH
	val STENCIL = GLES30.GL_STENCIL
	val MIN = GLES30.GL_MIN
	val MAX = GLES30.GL_MAX
	val DEPTH_COMPONENT24 = GLES30.GL_DEPTH_COMPONENT24
	val STREAM_READ = GLES30.GL_STREAM_READ
	val STREAM_COPY = GLES30.GL_STREAM_COPY
	val STATIC_READ = GLES30.GL_STATIC_READ
	val STATIC_COPY = GLES30.GL_STATIC_COPY
	val DYNAMIC_READ = GLES30.GL_DYNAMIC_READ
	val DYNAMIC_COPY = GLES30.GL_DYNAMIC_COPY
	val DEPTH_COMPONENT32F = GLES30.GL_DEPTH_COMPONENT32F
	val DEPTH32F_STENCIL8 = GLES30.GL_DEPTH32F_STENCIL8
	val INVALID_INDEX = GLES30.GL_INVALID_INDEX
	val TIMEOUT_IGNORED = GLES30.GL_TIMEOUT_IGNORED
	val MAX_CLIENT_WAIT_TIMEOUT_WEBGL = 0x9247 /* Miscellaneous constants */

	companion object {
		@JvmStatic
		private external fun nativeTexImage3DAsset(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			depth: Int,
			border: Int,
			format: Int,
			image_type: Int,
			asset: Long,
			flipY: Boolean
		)

		@JvmStatic
		private external fun nativeTexSubImage3DAsset(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			zoffset: Int,
			width: Int,
			height: Int,
			depth: Int,
			format: Int,
			type: Int,
			asset: Long,
			flipY: Boolean
		)

		@JvmStatic
		private external fun nativeTexImage3DBuffer(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			depth: Int,
			border: Int,
			format: Int,
			type: Int,
			buffer: Buffer,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexImage3DByteArray(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			depth: Int,
			border: Int,
			format: Int,
			type: Int,
			byteArray: ByteArray,
			flipYWebGL: Boolean
		)


		@JvmStatic
		private external fun nativeTexImage3DShortArray(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			depth: Int,
			border: Int,
			format: Int,
			type: Int,
			shortArray: ShortArray,
			flipYWebGL: Boolean
		)


		@JvmStatic
		private external fun nativeTexImage3DIntArray(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			depth: Int,
			border: Int,
			format: Int,
			type: Int,
			intArray: IntArray,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexImage3DFloatArray(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			depth: Int,
			border: Int,
			format: Int,
			type: Int,
			floatArray: FloatArray,
			flipYWebGL: Boolean
		)


		@JvmStatic
		private external fun nativeTexImage3DDoubleArray(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			depth: Int,
			border: Int,
			format: Int,
			type: Int,
			doubleArray: DoubleArray,
			flipYWebGL: Boolean
		)


		@JvmStatic
		private external fun nativeTexImage3DLongArray(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			depth: Int,
			border: Int,
			format: Int,
			type: Int,
			longArray: LongArray,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexImage3DBitmap(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			depth: Int,
			border: Int,
			format: Int,
			type: Int,
			source: Bitmap,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexSubImage3DBitmap(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			zoffset: Int,
			width: Int,
			height: Int,
			depth: Int,
			format: Int,
			type: Int,
			srcData: Bitmap,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexSubImage3DBuffer(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			zoffset: Int,
			width: Int,
			height: Int,
			depth: Int,
			format: Int,
			type: Int,
			buffer: Buffer,
			flipYWebGL: Boolean
		)


		@JvmStatic
		private external fun nativeTexSubImage3DByteArray(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			zoffset: Int,
			width: Int,
			height: Int,
			depth: Int,
			format: Int,
			type: Int,
			byteArray: ByteArray,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexSubImage3DShortArray(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			zoffset: Int,
			width: Int,
			height: Int,
			depth: Int,
			format: Int,
			type: Int,
			shortArray: ShortArray,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexSubImage3DIntArray(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			zoffset: Int,
			width: Int,
			height: Int,
			depth: Int,
			format: Int,
			type: Int,
			intArray: IntArray,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexSubImage3DLongArray(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			zoffset: Int,
			width: Int,
			height: Int,
			depth: Int,
			format: Int,
			type: Int,
			longArray: LongArray,
			flipYWebGL: Boolean
		)


		@JvmStatic
		private external fun nativeTexSubImage3DFloatArray(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			zoffset: Int,
			width: Int,
			height: Int,
			depth: Int,
			format: Int,
			type: Int,
			floatArray: FloatArray,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexSubImage3DDoubleArray(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			zoffset: Int,
			width: Int,
			height: Int,
			depth: Int,
			format: Int,
			type: Int,
			doubleArray: DoubleArray,
			flipYWebGL: Boolean
		)

	}
}
