package org.nativescript.canvas

import android.graphics.Bitmap
import android.opengl.GLES20
import android.opengl.GLUtils
import android.os.Build
import android.os.Build.VERSION_CODES
import android.util.Log
import org.nativescript.canvas.extensions.*
import java.io.UnsupportedEncodingException
import java.nio.*
import java.nio.charset.Charset
import java.nio.charset.StandardCharsets
import java.util.*
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit


/**
 * Created by triniwiz on 4/21/20
 */
open class TNSWebGLRenderingContext : TNSCanvasRenderingContext {
	var canvas: TNSCanvas

	constructor(canvas: TNSCanvas) {
		this.canvas = canvas
	}

	constructor(canvas: TNSCanvas, attrs: Map<String?, Any?>?) {
		this.canvas = canvas
	}

	internal fun ensureContextIsCurrent() {
		canvas.renderer.makeCurrent(canvas.renderer.mEGLSurface)
	}

	val drawingBufferWidth: Int
		get() = canvas.drawingBufferWidth
	val drawingBufferHeight: Int
		get() = canvas.drawingBufferHeight

	internal fun runOnGLThread(runnable: Runnable?) {

	}

	fun updateCanvas() {
		// synchronized (canvasView.lock) {
		canvas.invalidateState = GLRenderer.InvalidateState.PENDING
		//}
	}

	internal val GL_UNSIGNED_BYTE = 0x1401
	internal val GL_FLOAT = 0x1406
	internal val GL_HALF_FLOAT = 0x140B
	internal val GL_UNSIGNED_SHORT_5_6_5 = 0x8363
	internal val GL_UNSIGNED_SHORT_4_4_4_4 = 0x8033
	internal val GL_UNSIGNED_SHORT_5_5_5_1 = 0x8034
	internal val GL_LUMINANCE = 0x1909
	internal val GL_ALPHA = 0x1906
	internal val GL_LUMINANCE_ALPHA = 0x190A
	internal val GL_RGB = 0x1907
	internal val GL_RGBA = 0x1908


	fun activeTexture(texture: Int) {
		ensureContextIsCurrent()
		GLES20.glActiveTexture(texture)
	}

	fun attachShader(program: Int, shader: Int) {
		ensureContextIsCurrent()
		GLES20.glAttachShader(program, shader)
	}

	fun bindAttribLocation(program: Int, index: Int, name: String?) {
		ensureContextIsCurrent()
		GLES20.glBindAttribLocation(program, index, name)
	}

	fun bindBuffer(target: Int, buffer: Int) {
		ensureContextIsCurrent()
		GLES20.glBindBuffer(target, buffer)
	}

	fun bindBuffer(target: Int, buffer: Any?) {
		ensureContextIsCurrent()
		GLES20.glBindBuffer(target, 0)
	}

	fun bindFramebuffer(target: Int, framebuffer: Int) {
		ensureContextIsCurrent()
		GLES20.glBindFramebuffer(target, framebuffer)
	}

	fun bindRenderbuffer(target: Int, renderbuffer: Int) {
		ensureContextIsCurrent()
		GLES20.glBindRenderbuffer(target, renderbuffer)
	}

	fun bindTexture(target: Int, texture: Int) {
		ensureContextIsCurrent()
		GLES20.glBindTexture(target, texture)
	}

	fun blendColor(red: Float, green: Float, blue: Float, alpha: Float) {
		ensureContextIsCurrent()
		GLES20.glBlendColor(red, green, blue, alpha)
	}

	fun blendEquation(mode: Int) {
		ensureContextIsCurrent()
		GLES20.glBlendEquation(mode)
	}

	fun blendEquationSeparate(modeRGB: Int, modeAlpha: Int) {
		ensureContextIsCurrent()
		GLES20.glBlendEquationSeparate(modeRGB, modeAlpha)
	}

	fun blendFunc(sfactor: Int, dfactor: Int) {
		ensureContextIsCurrent()
		GLES20.glBlendFunc(sfactor, dfactor)
	}

	fun blendFuncSeparate(srcRGB: Int, dstRGB: Int, srcAlpha: Int, dstAlpha: Int) {
		ensureContextIsCurrent()
		GLES20.glBlendFuncSeparate(srcRGB, dstRGB, srcAlpha, dstAlpha)
	}

	fun bufferData(target: Int, size: Int, usage: Int) {
		ensureContextIsCurrent()
		GLES20.glBufferData(target, size, null, usage)
	}

	fun bufferData(target: Int, srcData: Any?, usage: Int) {
		ensureContextIsCurrent()
		GLES20.glBufferData(target, 0, null, usage)
	}

	fun bufferDataByte(target: Int, srcData: ByteArray, usage: Int) {
		bufferData(target, srcData, usage);
	}

	fun bufferDataShort(target: Int, srcData: ShortArray, usage: Int) {
		bufferData(target, srcData, usage)
	}

	fun bufferDataFloat(target: Int, srcData: FloatArray, usage: Int) {
		bufferData(target, srcData, usage)
	}

	fun bufferDataInt(target: Int, srcData: IntArray, usage: Int) {
		bufferData(target, srcData, usage)
	}

	fun bufferData(target: Int, srcData: ByteArray, usage: Int) {
		ensureContextIsCurrent()
		val buffer = ByteBuffer.wrap(srcData)
		GLES20.glBufferData(target, srcData.size, buffer, usage)
	}

	fun bufferData(target: Int, srcData: ShortArray, usage: Int) {
		ensureContextIsCurrent()
		val size = srcData.size * SIZE_OF_SHORT
		val buffer = ShortBuffer.wrap(srcData)
		GLES20.glBufferData(target, size, buffer, usage)
	}

	fun bufferData(target: Int, srcData: FloatArray, usage: Int) {
		ensureContextIsCurrent()
		val size = srcData.size * SIZE_OF_FLOAT
		val buffer = FloatBuffer.wrap(srcData)
		GLES20.glBufferData(target, size, buffer, usage)
	}

	fun bufferData(target: Int, srcData: IntArray, usage: Int) {
		ensureContextIsCurrent()
		val size = srcData.size * SIZE_OF_INT
		val buffer = IntBuffer.wrap(srcData)
		GLES20.glBufferData(target, size, buffer, usage)
	}


	fun bufferDataByteBuffer(target: Int, srcData: ByteBuffer, usage: Int) {
		bufferData(target, srcData, usage);
	}

	fun bufferDataShortBuffer(target: Int, srcData: ShortBuffer, usage: Int) {
		bufferData(target, srcData, usage)
	}

	fun bufferDataFloatBuffer(target: Int, srcData: FloatBuffer, usage: Int) {
		bufferData(target, srcData, usage)
	}

	fun bufferDataIntBuffer(target: Int, srcData: IntBuffer, usage: Int) {
		bufferData(target, srcData, usage)
	}


	fun bufferData(target: Int, srcData: ByteBuffer, usage: Int) {
		ensureContextIsCurrent()
		GLES20.glBufferData(target, srcData.capacity(), srcData, usage)
	}

	fun bufferData(target: Int, srcData: ShortBuffer, usage: Int) {
		ensureContextIsCurrent()
		GLES20.glBufferData(target, srcData.capacity() * SIZE_OF_SHORT, srcData, usage)

	}

	fun bufferData(target: Int, srcData: IntBuffer, usage: Int) {
		ensureContextIsCurrent()
		GLES20.glBufferData(target, srcData.capacity() * SIZE_OF_INT, srcData, usage)

	}

	fun bufferData(target: Int, srcData: FloatBuffer, usage: Int) {
		ensureContextIsCurrent()
		GLES20.glBufferData(target, srcData.capacity() * SIZE_OF_FLOAT, srcData, usage)

	}

	fun bufferSubDataByte(target: Int, offset: Int, srcData: ByteArray) {
		bufferSubData(target, offset, srcData)
	}

	fun bufferSubDataShort(target: Int, offset: Int, srcData: ShortArray) {
		bufferSubData(target, offset, srcData)
	}

	fun bufferSubDataInt(target: Int, offset: Int, srcData: IntArray) {
		bufferSubData(target, offset, srcData)
	}

	fun bufferSubDataFloat(target: Int, offset: Int, srcData: FloatArray) {
		bufferSubData(target, offset, srcData)
	}

	fun bufferSubData(target: Int, offset: Int, srcData: ByteArray) {
		ensureContextIsCurrent()
		val size = srcData.size
		val buffer = ByteBuffer.wrap(srcData)
		GLES20.glBufferSubData(target, offset, size, buffer)

	}

	fun bufferSubData(target: Int, offset: Int, srcData: ShortArray) {
		ensureContextIsCurrent()
		val size = srcData.size * SIZE_OF_SHORT
		val buffer = ShortBuffer.wrap(srcData)
		val os = SIZE_OF_SHORT * offset
		GLES20.glBufferSubData(target, os, size, buffer)

	}

	fun bufferSubData(target: Int, offset: Int, srcData: IntArray) {
		ensureContextIsCurrent()
		val size = srcData.size * SIZE_OF_INT
		val buffer = IntBuffer.wrap(srcData)
		val os = SIZE_OF_INT * offset
		GLES20.glBufferSubData(target, os, size, buffer)

	}

	fun bufferSubData(target: Int, offset: Int, srcData: FloatArray) {
		ensureContextIsCurrent()
		val size = srcData.size * SIZE_OF_FLOAT
		val buffer = FloatBuffer.wrap(srcData)
		val os = SIZE_OF_FLOAT * offset
		GLES20.glBufferSubData(target, os, size, buffer)

	}


	fun bufferSubDataByteBuffer(target: Int, offset: Int, srcData: ByteBuffer) {
		bufferSubData(target, offset, srcData)
	}

	fun bufferSubDataShortBuffer(target: Int, offset: Int, srcData: ShortBuffer) {
		bufferSubData(target, offset, srcData)
	}

	fun bufferSubDataIntBuffer(target: Int, offset: Int, srcData: IntBuffer) {
		bufferSubData(target, offset, srcData)
	}

	fun bufferSubDataFloatBuffer(target: Int, offset: Int, srcData: FloatBuffer) {
		bufferSubData(target, offset, srcData)
	}


	fun bufferSubData(target: Int, offset: Int, srcData: ByteBuffer) {
		ensureContextIsCurrent()
		GLES20.glBufferSubData(target, offset, srcData.capacity(), srcData)

	}

	fun bufferSubData(target: Int, offset: Int, srcData: ShortBuffer) {
		ensureContextIsCurrent()
		GLES20.glBufferSubData(target, offset, srcData.capacity() * SIZE_OF_SHORT, srcData)

	}

	fun bufferSubData(target: Int, offset: Int, srcData: IntBuffer) {
		ensureContextIsCurrent()
		GLES20.glBufferSubData(target, offset, srcData.capacity() * SIZE_OF_INT, srcData)

	}

	fun bufferSubData(target: Int, offset: Int, srcData: FloatBuffer) {
		ensureContextIsCurrent()
		GLES20.glBufferSubData(target, offset, srcData.capacity() * SIZE_OF_FLOAT, srcData)

	}

	fun checkFramebufferStatus(target: Int): Int {
		ensureContextIsCurrent()
		return GLES20.glCheckFramebufferStatus(target)
	}

	fun clear(mask: Int) {
		ensureContextIsCurrent()
		if (clearIfComposited(mask) !== HowToClear.CombinedClear) {
			GLES20.glClear(mask)
		}
		updateCanvas()

	}

	fun clearColor(red: Float, green: Float, blue: Float, alpha: Float) {

		canvas.renderer.mClearColor[0] = red
		canvas.renderer.mClearColor[1] = green
		canvas.renderer.mClearColor[2] = blue
		canvas.renderer.mClearColor[3] = alpha
		ensureContextIsCurrent()
		GLES20.glClearColor(red, green, blue, alpha)

	}

	fun clearDepth(depth: Float) {

		canvas.renderer.mClearDepth = depth
		ensureContextIsCurrent()
		GLES20.glClearDepthf(depth)

	}

	fun clearStencil(stencil: Int) {

		canvas.renderer.mClearStencil = stencil
		ensureContextIsCurrent()
		GLES20.glClearStencil(stencil)

	}

	fun colorMask(red: Boolean, green: Boolean, blue: Boolean, alpha: Boolean) {

		canvas.renderer.mColorMask[0] = red
		canvas.renderer.mColorMask[1] = green
		canvas.renderer.mColorMask[2] = blue
		canvas.renderer.mColorMask[3] = alpha
		ensureContextIsCurrent()
		GLES20.glColorMask(red, green, blue, alpha)

	}

	fun reset() {
		GLES20.glDisable(GLES20.GL_SCISSOR_TEST)
		GLES20.glClearColor(0f, 0f, 0f, 0f)
		GLES20.glColorMask(true, true, true, true)
		var clearMask = GLES20.GL_COLOR_BUFFER_BIT
		if (canvas.renderer.contextDepth) {
			GLES20.glClearDepthf(1f)
			clearMask = clearMask or GLES20.GL_DEPTH_BUFFER_BIT
			GLES20.glDepthMask(true)
		}
		if (canvas.renderer.contextStencil) {
			GLES20.glClearStencil(0)
			clearMask = clearMask or GLES20.GL_STENCIL_BUFFER_BIT
			GLES20.glStencilMaskSeparate(GLES20.GL_FRONT, (0xFFFFFFFF).toInt())
		}
	}

	fun restoreStateAfterClear() {

		// Restore the state that the context set.
		if (canvas.renderer.mScissorEnabled) {
			GLES20.glEnable(GLES20.GL_SCISSOR_TEST)
		}
		GLES20.glClearColor(
			canvas.renderer.mClearColor[0],
			canvas.renderer.mClearColor[1],
			canvas.renderer.mClearColor[2],
			canvas.renderer.mClearColor[3]
		)
		GLES20.glColorMask(
			canvas.renderer.mColorMask[0], canvas.renderer.mColorMask[1],
			canvas.renderer.mColorMask[2], canvas.renderer.mColorMask[3]
		)
		GLES20.glClearDepthf(canvas.renderer.mClearDepth)
		GLES20.glClearStencil(canvas.renderer.mClearStencil)
		GLES20.glStencilMaskSeparate(GLES20.GL_FRONT, canvas.renderer.mStencilMask)
		GLES20.glDepthMask(canvas.renderer.mDepthMask)
	}

	@JvmOverloads
	fun clearIfComposited(mask: Int = 0): HowToClear {
		val combinedClear = mask > 0 && !canvas.renderer.mScissorEnabled
		val m = mask and GLES20.GL_COLOR_BUFFER_BIT
		GLES20.glDisable(GLES20.GL_SCISSOR_TEST)
		if (combinedClear && m == GLES20.GL_COLOR_BUFFER_BIT) {
			GLES20.glClearColor(
				if (canvas.renderer.mColorMask[0]) canvas.renderer.mClearColor[0] else 0f,
				if (canvas.renderer.mColorMask[1]) canvas.renderer.mClearColor[1] else 0f,
				if (canvas.renderer.mColorMask[2]) canvas.renderer.mClearColor[2] else 0f,
				if (canvas.renderer.mColorMask[3]) canvas.renderer.mClearColor[3] else 0f
			)
		} else {
			GLES20.glClearColor(0f, 0f, 0f, 0f)
		}
		GLES20.glColorMask(true, true, true, true)
		var clearMask = GLES20.GL_COLOR_BUFFER_BIT
		if (canvas.renderer.contextDepth) {
			if (!combinedClear || !canvas.renderer.mDepthMask || mask and GLES20.GL_DEPTH_BUFFER_BIT == 0) {
				GLES20.glClearDepthf(1f)
				clearMask = clearMask or GLES20.GL_DEPTH_BUFFER_BIT
				GLES20.glDepthMask(true)
			}
		}
		if (canvas.renderer.contextStencil) {
			if (combinedClear && mask and GLES20.GL_STENCIL_BUFFER_BIT != 0) {
				GLES20.glClearStencil(canvas.renderer.mClearStencil and canvas.renderer.mStencilMask)
			} else {
				GLES20.glClearStencil(0)
				clearMask = clearMask or GLES20.GL_STENCIL_BUFFER_BIT
				GLES20.glStencilMaskSeparate(GLES20.GL_FRONT, (0xFFFFFFFF).toInt())
			}
		}
		// mask
		//  GLES20.glBindFramebuffer(GLES20.GL_FRAMEBUFFER, 0);
		GLES20.glClear(mask)
		restoreStateAfterClear()
		return if (combinedClear) HowToClear.CombinedClear else HowToClear.JustClear
	}

	fun commit() {
		// NOOP
	}

	fun compileShader(shader: Int) {
		ensureContextIsCurrent()
		GLES20.glCompileShader(shader)

	}


	fun compressedTexImage2DByteBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: ByteBuffer
	) {
		compressedTexImage2D(target, level, internalformat, width, height, border, pixels)
	}

	fun compressedTexImage2DShortBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: ShortBuffer
	) {
		compressedTexImage2D(target, level, internalformat, width, height, border, pixels)
	}


	fun compressedTexImage2DIntBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: IntBuffer
	) {
		compressedTexImage2D(target, level, internalformat, width, height, border, pixels)
	}


	fun compressedTexImage2DFloatBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: FloatBuffer
	) {
		compressedTexImage2D(target, level, internalformat, width, height, border, pixels)
	}


	fun compressedTexImage2DByte(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: ByteArray
	) {
		compressedTexImage2D(target, level, internalformat, width, height, border, pixels)
	}

	fun compressedTexImage2DShort(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: ShortArray
	) {
		compressedTexImage2D(target, level, internalformat, width, height, border, pixels)
	}


	fun compressedTexImage2DInt(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: IntArray
	) {
		compressedTexImage2D(target, level, internalformat, width, height, border, pixels)
	}


	fun compressedTexImage2DFloat(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: FloatArray
	) {
		compressedTexImage2D(target, level, internalformat, width, height, border, pixels)
	}


	fun compressedTexImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: ByteArray
	) {
		ensureContextIsCurrent()
		val size = pixels.size
		val buffer = ByteBuffer.wrap(pixels)
		GLES20.glCompressedTexImage2D(
			target,
			level,
			internalformat,
			width,
			height,
			border,
			size,
			buffer
		)

	}

	fun compressedTexImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: ShortArray
	) {
		ensureContextIsCurrent()
		val size = pixels.size * SIZE_OF_SHORT
		val buffer = ShortBuffer.wrap(pixels)
		GLES20.glCompressedTexImage2D(
			target,
			level,
			internalformat,
			width,
			height,
			border,
			size,
			buffer
		)

	}

	fun compressedTexImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: IntArray
	) {
		ensureContextIsCurrent()
		val size = pixels.size * SIZE_OF_INT
		val buffer = IntBuffer.wrap(pixels)
		GLES20.glCompressedTexImage2D(
			target,
			level,
			internalformat,
			width,
			height,
			border,
			size,
			buffer
		)

	}

	fun compressedTexImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: FloatArray
	) {
		ensureContextIsCurrent()
		val size = pixels.size * SIZE_OF_FLOAT
		val buffer = FloatBuffer.wrap(pixels)
		GLES20.glCompressedTexImage2D(
			target,
			level,
			internalformat,
			width,
			height,
			border,
			size,
			buffer
		)

	}


	fun compressedTexImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: ByteBuffer
	) {
		ensureContextIsCurrent()
		GLES20.glCompressedTexImage2D(
			target,
			level,
			internalformat,
			width,
			height,
			border,
			pixels.capacity(),
			pixels
		)

	}

	fun compressedTexImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: ShortBuffer
	) {
		ensureContextIsCurrent()
		GLES20.glCompressedTexImage2D(
			target,
			level,
			internalformat,
			width,
			height,
			border,
			pixels.capacity() * SIZE_OF_SHORT,
			pixels
		)

	}

	fun compressedTexImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: IntBuffer
	) {
		ensureContextIsCurrent()
		GLES20.glCompressedTexImage2D(
			target,
			level,
			internalformat,
			width,
			height,
			border,
			pixels.capacity() * SIZE_OF_INT,
			pixels
		)

	}

	fun compressedTexImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		pixels: FloatBuffer
	) {
		ensureContextIsCurrent()
		GLES20.glCompressedTexImage2D(
			target,
			level,
			internalformat,
			width,
			height,
			border,
			pixels.capacity() * SIZE_OF_FLOAT,
			pixels
		)

	}


	fun compressedTexSubImage2DByteBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: ByteBuffer
	) {
		compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels)
	}


	fun compressedTexSubImage2DShortBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: ShortBuffer
	) {
		compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels)
	}

	fun compressedTexSubImage2DIntBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: IntBuffer
	) {
		compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels)
	}

	fun compressedTexSubImage2DFloatBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: FloatBuffer
	) {
		compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels)
	}


	fun compressedTexSubImage2DByte(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: ByteArray
	) {
		compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels)
	}


	fun compressedTexSubImage2DShort(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: ShortArray
	) {
		compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels)
	}

	fun compressedTexSubImage2DInt(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: IntArray
	) {
		compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels)
	}

	fun compressedTexSubImage2DFloat(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: FloatArray
	) {
		compressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, pixels)
	}

	fun compressedTexSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: ByteBuffer
	) {
		ensureContextIsCurrent()
		GLES20.glCompressedTexSubImage2D(
			target,
			level,
			xoffset,
			yoffset,
			width,
			height,
			format,
			pixels.capacity(),
			pixels
		)

	}


	fun compressedTexSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: ShortBuffer
	) {
		ensureContextIsCurrent()
		GLES20.glCompressedTexSubImage2D(
			target,
			level,
			xoffset,
			yoffset,
			width,
			height,
			format,
			pixels.capacity() * SIZE_OF_SHORT,
			pixels
		)

	}


	fun compressedTexSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: IntBuffer
	) {
		ensureContextIsCurrent()
		GLES20.glCompressedTexSubImage2D(
			target,
			level,
			xoffset,
			yoffset,
			width,
			height,
			format,
			pixels.capacity() * SIZE_OF_INT,
			pixels
		)

	}

	fun compressedTexSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: FloatBuffer
	) {
		ensureContextIsCurrent()
		GLES20.glCompressedTexSubImage2D(
			target,
			level,
			xoffset,
			yoffset,
			width,
			height,
			format,
			pixels.capacity() * SIZE_OF_FLOAT,
			pixels
		)

	}


	fun compressedTexSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: ByteArray
	) {
		ensureContextIsCurrent()
		val size = pixels.size
		val buffer = ByteBuffer.wrap(pixels)
		GLES20.glCompressedTexSubImage2D(
			target,
			level,
			xoffset,
			yoffset,
			width,
			height,
			format,
			size,
			buffer
		)

	}

	fun compressedTexSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: ShortArray
	) {
		ensureContextIsCurrent()
		val size = pixels.size * SIZE_OF_SHORT
		val buffer = ShortBuffer.wrap(pixels)
		GLES20.glCompressedTexSubImage2D(
			target,
			level,
			xoffset,
			yoffset,
			width,
			height,
			format,
			size,
			buffer
		)

	}

	fun compressedTexSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: IntArray
	) {
		ensureContextIsCurrent()
		val size = pixels.size * SIZE_OF_INT
		val buffer = IntBuffer.wrap(pixels)
		GLES20.glCompressedTexSubImage2D(
			target,
			level,
			xoffset,
			yoffset,
			width,
			height,
			format,
			size,
			buffer
		)

	}

	fun compressedTexSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		pixels: FloatArray
	) {
		ensureContextIsCurrent()
		val size = pixels.size * SIZE_OF_FLOAT
		val buffer = FloatBuffer.wrap(pixels)
		GLES20.glCompressedTexSubImage2D(
			target,
			level,
			xoffset,
			yoffset,
			width,
			height,
			format,
			size,
			buffer
		)

	}

	fun copyTexImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		border: Int
	) {
		ensureContextIsCurrent()
		clearIfComposited()
		GLES20.glCopyTexImage2D(target, level, internalformat, x, y, width, height, border)

	}

	fun copyTexSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		x: Int,
		y: Int,
		width: Int,
		height: Int
	) {
		ensureContextIsCurrent()
		clearIfComposited()
		GLES20.glCopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height)

	}

	fun createBuffer(): Int {

		val bufferId = IntArray(1)
		ensureContextIsCurrent()
		GLES20.glGenBuffers(1, bufferId, 0)

		return bufferId[0]
	}

	fun createFramebuffer(): Int {

		val frameBufferId = IntArray(1)
		ensureContextIsCurrent()
		GLES20.glGenFramebuffers(1, frameBufferId, 0)

		return frameBufferId[0]
	}

	fun createProgram(): Int {

		ensureContextIsCurrent()
		return GLES20.glCreateProgram()
	}

	fun createRenderbuffer(): Int {

		val renderBufferId = IntArray(1)
		ensureContextIsCurrent()
		GLES20.glGenRenderbuffers(1, renderBufferId, 0)

		return renderBufferId[0]
	}

	fun createShader(type: Int): Int {
		ensureContextIsCurrent()
		return GLES20.glCreateShader(type)
	}

	fun createTexture(): Int {
		val textureId = IntArray(1)
		ensureContextIsCurrent()
		GLES20.glGenTextures(1, textureId, 0)
		return textureId[0]
	}

	fun cullFace(mode: Int) {
		ensureContextIsCurrent()
		GLES20.glCullFace(mode)

	}

	fun deleteBuffer(buffer: Int) {
		ensureContextIsCurrent()
		val id = intArrayOf(buffer)
		GLES20.glDeleteBuffers(1, id, 0)
	}

	fun deleteFramebuffer(frameBuffer: Int) {
		ensureContextIsCurrent()
		val id = intArrayOf(frameBuffer)
		GLES20.glDeleteFramebuffers(1, id, 0)
	}

	fun deleteProgram(program: Int) {
		ensureContextIsCurrent()
		GLES20.glDeleteProgram(program)
	}

	fun deleteRenderbuffer(renderbuffer: Int) {
		ensureContextIsCurrent()
		val id = intArrayOf(renderbuffer)
		GLES20.glDeleteRenderbuffers(1, id, 0)
	}

	fun deleteShader(shader: Int) {
		ensureContextIsCurrent()
		GLES20.glDeleteShader(shader)

	}

	fun deleteTexture(texture: Int) {
		ensureContextIsCurrent()
		val id = intArrayOf(texture)
		GLES20.glDeleteTextures(1, id, 0)
	}

	fun depthFunc(func: Int) {
		ensureContextIsCurrent()
		GLES20.glDepthFunc(func)

	}

	fun depthMask(flag: Boolean) {
		ensureContextIsCurrent()
		GLES20.glDepthMask(flag)

	}

	fun depthRange(zNear: Float, zFar: Float) {
		ensureContextIsCurrent()
		GLES20.glDepthRangef(zNear, zFar)

	}

	fun detachShader(program: Int, shader: Int) {
		ensureContextIsCurrent()
		GLES20.glDetachShader(program, shader)

	}

	fun disable(cap: Int) {
		ensureContextIsCurrent()
		GLES20.glDisable(cap)

	}

	fun disableVertexAttribArray(index: Int) {
		ensureContextIsCurrent()
		GLES20.glDisableVertexAttribArray(index)
	}

	fun drawArrays(mode: Int, first: Int, count: Int) {
		ensureContextIsCurrent()
		clearIfComposited()
		GLES20.glDrawArrays(mode, first, count)
		updateCanvas()

	}

	fun drawElements(mode: Int, count: Int, type: Int, offset: Int) {
		ensureContextIsCurrent()
		clearIfComposited()
		GLES20.glDrawElements(mode, count, type, offset)
		updateCanvas()

	}

	fun enable(cap: Int) {
		ensureContextIsCurrent()
		GLES20.glEnable(cap)

	}

	fun enableVertexAttribArray(index: Int) {
		ensureContextIsCurrent()
		GLES20.glEnableVertexAttribArray(index)

	}

	fun finish() {
		ensureContextIsCurrent()
		GLES20.glFinish()

	}

	fun flush() {
		ensureContextIsCurrent()
		GLES20.glFlush()

	}

	fun framebufferRenderbuffer(
		target: Int,
		attachment: Int,
		renderbuffertarget: Int,
		renderbuffer: Int
	) {
		ensureContextIsCurrent()
		GLES20.glFramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer)

	}

	fun framebufferTexture2D(
		target: Int,
		attachment: Int,
		textarget: Int,
		texture: Int,
		level: Int
	) {
		ensureContextIsCurrent()
		GLES20.glFramebufferTexture2D(target, attachment, textarget, texture, level)

	}

	fun frontFace(mode: Int) {
		ensureContextIsCurrent()
		GLES20.glFrontFace(mode)

	}

	fun generateMipmap(target: Int) {
		ensureContextIsCurrent()
		GLES20.glGenerateMipmap(target)

	}

	fun getActiveAttrib(program: Int, index: Int): WebGLActiveInfo {

		val info = WebGLActiveInfo()
		ensureContextIsCurrent() //IntBuffer length = IntBuffer.allocate(1);
		//GLES20.glGetProgramiv(program, GLES20.GL_ACTIVE_ATTRIBUTE_MAX_LENGTH, length);
		val length = IntArray(1)
		GLES20.glGetProgramiv(program, GLES20.GL_ACTIVE_ATTRIBUTE_MAX_LENGTH, length, 0)
		val name = ByteArray(length[0])
		val size = IntArray(1)
		val type = IntArray(1)
		val nameLength = IntArray(1)
		GLES20.glGetActiveAttrib(
			program,
			index,
			length[0],
			nameLength,
			0,
			size,
			0,
			type,
			0,
			name,
			0
		)
		val result = if (Build.VERSION.SDK_INT >= VERSION_CODES.KITKAT) {
			String(name, 0, nameLength[0], StandardCharsets.UTF_8)
		} else {
			try {
				String(name, 0, nameLength[0], Charset.forName("UTF-8"))
			} catch (e: UnsupportedEncodingException) {
				String(name, 0, nameLength[0])
			}
		}
		info.name = result
		info.size = size[0]
		info.type = type[0]

		return info
	}

	fun getActiveUniform(program: Int, index: Int): WebGLActiveInfo {

		val info = WebGLActiveInfo()
		ensureContextIsCurrent()
		val length = IntArray(1)
		val size = IntArray(1)
		val type = IntArray(1)
		GLES20.glGetProgramiv(program, GLES20.GL_ACTIVE_UNIFORM_MAX_LENGTH, length, 0)
		val name = ByteArray(length[0])
		val nameLength = IntArray(1)
		GLES20.glGetActiveUniform(
			program,
			index,
			length[0],
			nameLength,
			0,
			size,
			0,
			type,
			0,
			name,
			0
		)
		val result = if (Build.VERSION.SDK_INT >= VERSION_CODES.KITKAT) {
			String(name, 0, nameLength[0], StandardCharsets.UTF_8)
		} else {
			try {
				String(name, 0, nameLength[0], Charset.forName("UTF-8"))
			} catch (e: UnsupportedEncodingException) {
				String(name, 0, nameLength[0])
			}
		}
		info.name = result
		info.size = size[0]
		info.type = type[0]

		return info
	}

	fun getAttachedShaders(program: Int): IntArray {

		ensureContextIsCurrent()
		val count = IntArray(1)
		GLES20.glGetProgramiv(program, GLES20.GL_ATTACHED_SHADERS, count, 0)
		val shaders = IntArray(count[0])
		val written = IntArray(1)
		GLES20.glGetAttachedShaders(program, count[0], written, 0, shaders, 0)

		return shaders
	}

	fun getAttribLocation(program: Int, name: String?): Int {

		ensureContextIsCurrent()
		return GLES20.glGetAttribLocation(program, name)
	}

	fun getBufferParameter(target: Int, pname: Int): Int {
		ensureContextIsCurrent() //                IntBuffer params = IntBuffer.allocate(1);
//                GLES20.glGetBufferParameteriv(target, pname, params);
		val params = IntArray(1)
		GLES20.glGetBufferParameteriv(target, pname, params, 0)

		return params[0]
	}

	private val alpha = true
	private val antialias = true
	private val depth = true
	private val failIfMajorPerformanceCaveat = false
	private val powerPreference = "default"
	private val premultipliedAlpha = true
	private val preserveDrawingBuffer = false
	private val stencil = false
	private val desynchronized = false

	// Return nil if context is lost
	val contextAttributes: Map<String, Any?>
		get() {
			// Return nil if context is lost
			val attrib: MutableMap<String, Any?> = HashMap()
			attrib["alpha"] = canvas.renderer.contextAlpha
			attrib["antialias"] = canvas.renderer.contextAntialias
			attrib["depth"] = canvas.renderer.contextDepth
			attrib["failIfMajorPerformanceCaveat"] = canvas.renderer.contextFailIfMajorPerformanceCaveat
			attrib["powerPreference"] = canvas.renderer.contextPowerPreference
			attrib["premultipliedAlpha"] = canvas.renderer.contextPremultipliedAlpha
			attrib["preserveDrawingBuffer"] = canvas.renderer.contextPreserveDrawingBuffer
			attrib["stencil"] = canvas.renderer.contextStencil
			attrib["desynchronized"] = canvas.renderer.contextDesynchronized
			return attrib
		}
	val error: Int
		get() {
			ensureContextIsCurrent()
			return GLES20.glGetError()
		}

	fun getExtension(name: String): Any? {
		var value: Any? = null
		ensureContextIsCurrent()
		val extensions = GLES20.glGetString(GLES20.GL_EXTENSIONS)
		if (name == "EXT_blend_minmax" && extensions.contains("GL_EXT_blend_minmax")) {
			value = EXT_blend_minmax()
		} else if (name == "EXT_color_buffer_half_float" && extensions.contains("GL_EXT_color_buffer_half_float")) {
			value = EXT_color_buffer_half_float()
		} else if (name == "EXT_disjoint_timer_query" && extensions.contains("GL_EXT_disjoint_timer_query")) {
			value = if (Build.VERSION.SDK_INT >= VERSION_CODES.JELLY_BEAN_MR2) {
				EXT_disjoint_timer_query(canvas)
			} else {
				null
			}
		} else if (name == "EXT_sRGB" && extensions.contains("GL_EXT_sRGB")) {
			value = EXT_sRGB()
		} else if (name == "EXT_shader_texture_lod") {
			if (extensions.contains("GL_EXT_shader_texture_lod")) {
				value = EXT_shader_texture_lod()
			}
		} else if (name == "EXT_texture_filter_anisotropic" && extensions.contains("GL_EXT_texture_filter_anisotropic")) {
			value = EXT_texture_filter_anisotropic()
		} else if (name == "OES_element_index_uint" && extensions.contains("GL_OES_element_index_uint")) {
			value = OES_element_index_uint()
		} else if (name == "OES_standard_derivatives" && extensions.contains("GL_OES_standard_derivatives")) {
			value = OES_standard_derivatives()
		} else if (name == "OES_texture_float" && (extensions.contains("GL_OES_texture_float") || canvas.renderer.glVersion > 2)) {
			value = OES_texture_float()
		} else if (name == "OES_texture_float_linear" && (extensions.contains("GL_OES_texture_float_linear") || canvas.renderer.glVersion > 2)) {
			value = OES_texture_float_linear()
		} else if (name == "OES_texture_half_float" && extensions.contains("GL_OES_texture_half_float")) {
			value = OES_texture_half_float()
		} else if (name == "OES_texture_half_float_linear" && extensions.contains("GL_OES_texture_half_float_linear")) {
			value = OES_texture_half_float_linear()
		} else if (name == "OES_vertex_array_object" && extensions.contains("GL_OES_vertex_array_object")) {
			value = if (Build.VERSION.SDK_INT >= VERSION_CODES.JELLY_BEAN_MR2) {
				OES_vertex_array_object(canvas)
			} else {
				null
			}
		} else if (name == "WEBGL_color_buffer_float" && extensions.contains("GL_OES_packed_depth_stencil")) {
			value = WEBGL_color_buffer_float()
		} else if (name == "WEBGL_compressed_texture_atc" && extensions.contains("GL_AMD_compressed_ATC_texture")) {
			value = WEBGL_compressed_texture_atc()
		} else if (name == "WEBGL_compressed_texture_etc1" && extensions.contains("GL_OES_compressed_ETC1_RGB8_texture")) {
			value = WEBGL_compressed_texture_etc1()
		} else if (name == "WEBGL_compressed_texture_s3tc" && extensions.contains("GL_EXT_texture_compression_dxt1") && extensions.contains(
				"GL_EXT_texture_compression_s3tc"
			)
		) {
			value = WEBGL_compressed_texture_s3tc()
		} else if (name == "WEBGL_compressed_texture_etc") {
			value = if (canvas.renderer.glVersion > 2) {
				WEBGL_compressed_texture_etc()
			} else {
				null
			}
		} else if (name == "WEBGL_compressed_texture_pvrtc" && extensions.contains("GL_IMG_texture_compression_pvrtc")) {
			value = WEBGL_compressed_texture_pvrtc()
		} else if (name == "WEBGL_lose_context") {
			value = WEBGL_lose_context(canvas)
		} else if (name == "ANGLE_instanced_arrays") {
			if (canvas.renderer.glVersion > 2) {
				value = if (Build.VERSION.SDK_INT >= VERSION_CODES.JELLY_BEAN_MR2) {
					ANGLE_instanced_arrays()
				} else {
					null
				}
			}
		} else if (name == "WEBGL_depth_texture" && extensions.contains("GL_OES_depth_texture")) {
			value = WEBGL_depth_texture()
		} else if (name == "WEBGL_draw_buffers") {
			if (Build.VERSION.SDK_INT >= VERSION_CODES.JELLY_BEAN_MR2) {
				if (canvas.renderer.glVersion > 2 || extensions.contains("GL_EXT_draw_buffers")) {
					value = WEBGL_draw_buffers()
				}
			} else {
				value = null
			}
		} else {
			value = null
		}

		return value
	}

	fun getFramebufferAttachmentParameter(
		target: Int,
		attachment: Int,
		pname: Int
	): TNSFramebufferAttachmentParameter {

		val result = TNSFramebufferAttachmentParameter()
		ensureContextIsCurrent() //                IntBuffer params = IntBuffer.allocate(1);
//                GLES20.glGetFramebufferAttachmentParameteriv(target, attachment, pname, params);
		val params = IntArray(1)
		GLES20.glGetFramebufferAttachmentParameteriv(target, attachment, pname, params, 0)
		if (attachment == FRAMEBUFFER_ATTACHMENT_OBJECT_NAME) {
//                    IntBuffer name = IntBuffer.allocate(1);
//                    GLES20.glGetFramebufferAttachmentParameteriv(target, attachment, GLES20.GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE, name);
			val name = IntArray(0)
			GLES20.glGetFramebufferAttachmentParameteriv(
				target,
				attachment,
				GLES20.GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE,
				name,
				0
			)
			when (name[0]) {
				GLES20.GL_RENDERBUFFER -> {
					result.isRenderbuffer = true
					result.value = params[0]
				}
				GLES20.GL_TEXTURE -> {
					result.isTexture = true
					result.value = params[0]
				}
				else -> result.value = params[0]
			}
		} else {
			result.value = params[0]
		}

		return result
	}

	private fun boolConverter(value: Int): Boolean {
		return value == GLES20.GL_TRUE
	}

	open fun getParameter(pname: Int): Any? {

		var parameter: Any? = null
		ensureContextIsCurrent()
		when (pname) {
			ACTIVE_TEXTURE, ALPHA_BITS, ARRAY_BUFFER_BINDING, BLEND_DST_ALPHA, BLEND_DST_RGB, BLEND_EQUATION, BLEND_EQUATION_ALPHA, BLEND_SRC_ALPHA, BLEND_SRC_RGB, BLUE_BITS, CULL_FACE_MODE, CURRENT_PROGRAM, DEPTH_BITS, DEPTH_FUNC, ELEMENT_ARRAY_BUFFER_BINDING, FRAMEBUFFER_BINDING, FRONT_FACE, GENERATE_MIPMAP_HINT, GREEN_BITS, IMPLEMENTATION_COLOR_READ_FORMAT, IMPLEMENTATION_COLOR_READ_TYPE, MAX_COMBINED_TEXTURE_IMAGE_UNITS, MAX_CUBE_MAP_TEXTURE_SIZE, MAX_FRAGMENT_UNIFORM_VECTORS, MAX_RENDERBUFFER_SIZE, MAX_TEXTURE_IMAGE_UNITS, MAX_TEXTURE_SIZE, MAX_VARYING_VECTORS, MAX_VERTEX_ATTRIBS, MAX_VERTEX_TEXTURE_IMAGE_UNITS, MAX_VERTEX_UNIFORM_VECTORS, PACK_ALIGNMENT, RED_BITS, RENDERBUFFER_BINDING, SAMPLE_BUFFERS, SAMPLES, STENCIL_BACK_FAIL, STENCIL_BACK_FUNC, STENCIL_BACK_PASS_DEPTH_FAIL, STENCIL_BACK_PASS_DEPTH_PASS, STENCIL_BACK_REF, STENCIL_BACK_VALUE_MASK, STENCIL_BACK_WRITEMASK, STENCIL_BITS, STENCIL_CLEAR_VALUE, STENCIL_FAIL, STENCIL_FUNC, STENCIL_PASS_DEPTH_FAIL, STENCIL_PASS_DEPTH_PASS, STENCIL_REF, STENCIL_VALUE_MASK, STENCIL_WRITEMASK, SUBPIXEL_BITS, TEXTURE_BINDING_2D, TEXTURE_BINDING_CUBE_MAP, UNPACK_ALIGNMENT -> {
				//                        IntBuffer param = IntBuffer.allocate(1);
//                        GLES20.glGetIntegerv(pname, param);
				val param = IntArray(1)
				GLES20.glGetIntegerv(pname, param, 0)
				parameter =
					if ((pname == CURRENT_PROGRAM || pname == ARRAY_BUFFER_BINDING || pname == ELEMENT_ARRAY_BUFFER_BINDING || pname == TEXTURE_BINDING_2D || pname == TEXTURE_BINDING_CUBE_MAP || pname == RENDERBUFFER_BINDING || pname == FRAMEBUFFER_BINDING) && param[0] == 0) {
						null
					} else {
						param[0]
					}
			}
			UNPACK_COLORSPACE_CONVERSION_WEBGL -> {
				var cs = colorSpaceConversionWebGL
				if (cs == -1) {
					cs = BROWSER_DEFAULT_WEBGL
				}
				parameter = cs
			}
			ALIASED_LINE_WIDTH_RANGE, ALIASED_POINT_SIZE_RANGE, DEPTH_RANGE -> {
				val param2 = FloatArray(2)
				GLES20.glGetFloatv(pname, param2, 0)
				parameter = param2
			}
			BLEND_COLOR, COLOR_CLEAR_VALUE -> {
				val param3 = FloatArray(4)
				GLES20.glGetFloatv(pname, param3, 0)
				parameter = param3
			}
			UNPACK_FLIP_Y_WEBGL -> parameter = flipYWebGL
			UNPACK_PREMULTIPLY_ALPHA_WEBGL -> parameter = premultiplyAlphaWebGL
			BLEND, CULL_FACE, DEPTH_TEST, DEPTH_WRITEMASK, DITHER, POLYGON_OFFSET_FILL, SAMPLE_COVERAGE_INVERT, SCISSOR_TEST, STENCIL_TEST -> {
				val param4 = BooleanArray(1)
				GLES20.glGetBooleanv(pname, param4, 0)
				parameter = param4[0]
			}
			COLOR_WRITEMASK -> {
				val param5 = BooleanArray(4)
				GLES20.glGetBooleanv(pname, param5, 0)
				parameter = param5
			}
			COMPRESSED_TEXTURE_FORMATS -> {
				val count = IntArray(1)
				GLES20.glGetIntegerv(GLES20.GL_NUM_COMPRESSED_TEXTURE_FORMATS, count, 0)
				val formats = IntArray(count[0])
				GLES20.glGetIntegerv(GLES20.GL_COMPRESSED_TEXTURE_FORMATS, formats, 0)
				parameter = formats
			}
			DEPTH_CLEAR_VALUE, LINE_WIDTH, POLYGON_OFFSET_FACTOR, POLYGON_OFFSET_UNITS, SAMPLE_COVERAGE_VALUE -> {
				val param6 = FloatArray(1)
				GLES20.glGetFloatv(pname, param6, 0)
				parameter = param6[0]
			}
			MAX_VIEWPORT_DIMS -> {
				val dims = IntArray(2)
				GLES20.glGetIntegerv(pname, dims, 0)
				parameter = dims
			}
			SCISSOR_BOX, VIEWPORT -> {
				val params7 = IntArray(4)
				GLES20.glGetIntegerv(pname, params7, 0)
				parameter = params7
			}
			RENDERER, SHADING_LANGUAGE_VERSION, VENDOR, VERSION -> parameter =
				GLES20.glGetString(pname)
			else -> parameter = null
		}

		return parameter
	}

	fun getProgramInfoLog(program: Int): String? {
		ensureContextIsCurrent()

		return GLES20.glGetProgramInfoLog(program)
	}

	fun getProgramParameter(program: Int, pname: Int): Any? {

		val parameter: Any?
		ensureContextIsCurrent()
		val param = IntArray(1)
		GLES20.glGetProgramiv(program, pname, param, 0)
		parameter = when (pname) {
			DELETE_STATUS, LINK_STATUS, VALIDATE_STATUS -> boolConverter(
				param[0]
			)
			else -> param[0]
		}

		return parameter
	}

	fun getRenderbufferParameter(target: Int, pname: Int): Int {
		ensureContextIsCurrent()
		val params = IntArray(1)
		GLES20.glGetRenderbufferParameteriv(target, pname, params, 0)
		return params[0]
	}

	fun getShaderInfoLog(shader: Int): String? {
		ensureContextIsCurrent()
		return GLES20.glGetShaderInfoLog(shader)
	}

	fun getShaderParameter(shader: Int, pname: Int): Any? {

		val parameter: Any
		ensureContextIsCurrent()
		val params = IntArray(1)
		GLES20.glGetShaderiv(shader, pname, params, 0)
		parameter = when (pname) {
			DELETE_STATUS, COMPILE_STATUS -> boolConverter(params[0])
			else -> params[0]
		}

		return parameter
	}

	fun getShaderPrecisionFormat(shaderType: Int, precisionType: Int): WebGLShaderPrecisionFormat {

		val precisionFormat = WebGLShaderPrecisionFormat()
		//final boolean[] hasError = new boolean[1];
		ensureContextIsCurrent()
		val range = IntArray(2)
		val precision = IntArray(1)
		GLES20.glGetShaderPrecisionFormat(shaderType, precisionType, range, 0, precision, 0)
		//                int error = GLES20.glGetError();
//                if (error == GLES20.GL_INVALID_ENUM || error == GLES20.GL_INVALID_OPERATION) {
//                    hasError[0] = true;
//                    lock.countDown();
//                    return;
//                }
		precisionFormat.rangeMin = range[0]
		precisionFormat.rangeMax = range[1]
		precisionFormat.precision = precision[0]

		/*if (hasError[0]) {
            return null;
        }*/return precisionFormat
	}

	fun getShaderSource(shader: Int): String? {
		ensureContextIsCurrent()
		return GLES20.glGetShaderSource(shader)
	}

	val supportedExtensions: Array<String>
		get() {
			ensureContextIsCurrent()
			val glExtensions = GLES20.glGetString(GLES20.GL_EXTENSIONS)
			return glExtensions.split(" ").toTypedArray()
		}

	fun getTexParameter(target: Int, pname: Int): Int {
		ensureContextIsCurrent()
		val params = IntArray(1)
		GLES20.glGetTexParameteriv(target, pname, params, 0)
		return params[0]
	}

	private fun getFloatSlice(count: Int): FloatArray {
		return FloatArray(count)
	}

	private fun getIntSlice(count: Int): IntArray {
		return IntArray(count)
	}

	fun getUniform(program: Int, location: Int): Any? {
		var uniform: Any? = null
		ensureContextIsCurrent()
		val type = IntArray(1)
		GLES20.glGetActiveUniform(program, location, 0, null, 0, null, 0, type, 0, null, 0)
		when (type[0]) {
			GLES20.GL_FLOAT -> {
				val single = getFloatSlice(1)
				GLES20.glGetUniformfv(program, location, single, 0)
				uniform = single[0]
			}
			GLES20.GL_FLOAT_VEC2 -> {
				val vec2 = getFloatSlice(2)
				GLES20.glGetUniformfv(program, location, vec2, 0)
				uniform = vec2
			}
			GLES20.GL_FLOAT_VEC3 -> {
				val vec3 = getFloatSlice(3)
				GLES20.glGetUniformfv(program, location, vec3, 0)
				uniform = vec3
			}
			GLES20.GL_FLOAT_VEC4 -> {
				val vec4 = getFloatSlice(4)
				GLES20.glGetUniformfv(program, location, vec4, 0)
				uniform = vec4
			}
			GLES20.GL_SAMPLER_2D, GLES20.GL_SAMPLER_CUBE, GLES20.GL_INT -> {
				val singleInt = getIntSlice(1)
				GLES20.glGetUniformiv(program, location, singleInt, 0)
				uniform = singleInt[0]
			}
			GLES20.GL_INT_VEC2 -> {
				val intVec2 = getIntSlice(2)
				GLES20.glGetUniformiv(program, location, intVec2, 0)
				uniform = intVec2
			}
			GLES20.GL_INT_VEC3 -> {
				val intVec3 = getIntSlice(3)
				GLES20.glGetUniformiv(program, location, intVec3, 0)
				uniform = intVec3
			}
			GLES20.GL_INT_VEC4 -> {
				val intVec4 = getIntSlice(4)
				GLES20.glGetUniformiv(program, location, intVec4, 0)
				uniform = intVec4
			}
			GLES20.GL_BOOL -> {
				val singleBool = getIntSlice(1)
				GLES20.glGetUniformiv(program, location, singleBool, 0)
				uniform = boolConverter(singleBool[0])
			}
			GLES20.GL_BOOL_VEC2 -> {
				val boolVec2 = getIntSlice(2)
				val boolVec2Result = BooleanArray(2)
				GLES20.glGetUniformiv(program, location, boolVec2, 0)
				boolVec2Result[0] = boolConverter(boolVec2[0])
				boolVec2Result[1] = boolConverter(boolVec2[1])
				uniform = boolVec2Result
			}
			GLES20.GL_BOOL_VEC3 -> {
				val boolVec3 = getIntSlice(3)
				val boolVec3Result = BooleanArray(3)
				GLES20.glGetUniformiv(program, location, boolVec3, 0)
				boolVec3Result[0] = boolConverter(boolVec3[0])
				boolVec3Result[1] = boolConverter(boolVec3[1])
				boolVec3Result[2] = boolConverter(boolVec3[2])
				uniform = boolVec3Result
			}
			GLES20.GL_BOOL_VEC4 -> {
				val boolVec4 = getIntSlice(4)
				val boolVec4Result = BooleanArray(4)
				GLES20.glGetUniformiv(program, location, boolVec4, 0)
				boolVec4Result[0] = boolConverter(boolVec4[0])
				boolVec4Result[1] = boolConverter(boolVec4[1])
				boolVec4Result[2] = boolConverter(boolVec4[2])
				boolVec4Result[3] = boolConverter(boolVec4[3])
				uniform = boolVec4Result
			}
			GLES20.GL_FLOAT_MAT2 -> {
				val mat2 = getFloatSlice(2)
				GLES20.glGetUniformfv(program, location, mat2, 0)
				uniform = mat2
			}
			GLES20.GL_FLOAT_MAT3 -> {
				val mat3 = getFloatSlice(3)
				GLES20.glGetUniformfv(program, location, mat3, 0)
				uniform = mat3
			}
			GLES20.GL_FLOAT_MAT4 -> {
				val mat4 = getFloatSlice(4)
				GLES20.glGetUniformfv(program, location, mat4, 0)
				uniform = mat4
			}
		}

		return uniform
	}

	fun getUniformLocation(program: Int, name: String?): Int {
		ensureContextIsCurrent()
		return GLES20.glGetUniformLocation(program, name)
	}

	fun getVertexAttrib(index: Int, pname: Int): Any? {

		ensureContextIsCurrent()
		return if (pname == CURRENT_VERTEX_ATTRIB) {
			val params = FloatArray(4)
			GLES20.glGetVertexAttribfv(index, pname, params, 0)
			params
		} else {
			val params = IntArray(1)
			GLES20.glGetVertexAttribiv(index, pname, params, 0)
			when (pname) {
				VERTEX_ATTRIB_ARRAY_ENABLED, VERTEX_ATTRIB_ARRAY_NORMALIZED -> boolConverter(
					params[0]
				)
				else -> params[0]
			}
		}
	}

	fun getVertexAttribOffset(index: Int, pname: Int): Long {
		ensureContextIsCurrent()
		val buffer = ByteBuffer.allocateDirect(SIZE_OF_LONG).order(ByteOrder.nativeOrder())
		// LongBuffer buffer = LongBuffer.allocate(1);
		nativeGetVertexAttribOffset(index, pname, buffer)


		return buffer[0].toLong()
	}

	fun hint(target: Int, mode: Int) {
		ensureContextIsCurrent()
		GLES20.glHint(target, mode)

	}

	fun isBuffer(buffer: Int): Boolean {
		ensureContextIsCurrent()

		return GLES20.glIsBuffer(buffer)
	}

	//        if let renderer = canvas.renderer as? GLRenderer {
//            return EAGLContext.current() != renderer.glkView.context
//        }
	val isContextLost: Boolean
		get() =//        if let renderer = canvas.renderer as? GLRenderer {
//            return EAGLContext.current() != renderer.glkView.context
//        }
			false

	fun isEnabled(cap: Int): Boolean {
		ensureContextIsCurrent()

		return GLES20.glIsEnabled(cap)
	}

	fun isFramebuffer(framebuffer: Int): Boolean {
		ensureContextIsCurrent()
		return GLES20.glIsFramebuffer(framebuffer)
	}

	fun isProgram(program: Int): Boolean {
		ensureContextIsCurrent()
		return GLES20.glIsProgram(program)
	}

	fun isRenderbuffer(renderbuffer: Int): Boolean {
		ensureContextIsCurrent()
		return GLES20.glIsRenderbuffer(renderbuffer)
	}

	fun isShader(shader: Int): Boolean {
		ensureContextIsCurrent()
		return GLES20.glIsShader(shader)
	}

	fun isTexture(texture: Int): Boolean {
		ensureContextIsCurrent()
		return GLES20.glIsTexture(texture)
	}

	fun lineWidth(width: Float) {
		ensureContextIsCurrent()
		GLES20.glLineWidth(width)

	}

	fun linkProgram(program: Int) {
		ensureContextIsCurrent()
		GLES20.glLinkProgram(program)

	}

	private fun objectToInt(value: Any?, defaultValue: Int): Int {
		return if (value != null) {
			try {
				value.toString().toInt()
			} catch (e: NumberFormatException) {
				defaultValue
			}
		} else defaultValue
	}

	private fun objectToBoolean(value: Any?, defaultValue: Boolean): Boolean {
		return if (value != null) {
			java.lang.Boolean.parseBoolean(value.toString())
		} else defaultValue
	}

	private fun objectToColorSpace(value: Any?, defaultValue: Int): Int {
		return if (value != null) {
			try {
				val `val` = value.toString().toInt()
				if (`val` == BROWSER_DEFAULT_WEBGL || `val` == GLES20.GL_NONE) {
					`val`
				} else BROWSER_DEFAULT_WEBGL
			} catch (e: NumberFormatException) {
				defaultValue
			}
		} else defaultValue
	}

	var flipYWebGL = false
	private var premultiplyAlphaWebGL = false
	private var colorSpaceConversionWebGL = -1
	fun pixelStorei(pname: Int, param: Any?) {
		ensureContextIsCurrent()
		when (pname) {
			GLES20.GL_PACK_ALIGNMENT, GLES20.GL_UNPACK_ALIGNMENT -> GLES20.glPixelStorei(
				pname,
				objectToInt(param, 4)
			)
			UNPACK_FLIP_Y_WEBGL -> flipYWebGL = objectToBoolean(param, false)
			UNPACK_PREMULTIPLY_ALPHA_WEBGL -> premultiplyAlphaWebGL = objectToBoolean(param, false)
			UNPACK_COLORSPACE_CONVERSION_WEBGL -> colorSpaceConversionWebGL =
				objectToColorSpace(param, BROWSER_DEFAULT_WEBGL)
			else -> {
			}
		}

	}

	fun polygonOffset(factor: Float, units: Float) {
		ensureContextIsCurrent()
		GLES20.glPolygonOffset(factor, units)

	}

	fun readPixelsByte(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ByteArray
	) {
		readPixels(x, y, width, height, format, type, pixels)
	}

	fun readPixelsShort(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ShortArray
	) {
		readPixels(x, y, width, height, format, type, pixels)
	}


	fun readPixelsInt(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: IntArray
	) {
		readPixels(x, y, width, height, format, type, pixels)
	}

	fun readPixelsFloat(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: FloatArray
	) {
		readPixels(x, y, width, height, format, type, pixels)
	}


	fun readPixelsByteBuffer(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ByteBuffer
	) {
		readPixels(x, y, width, height, format, type, pixels)
	}

	fun readPixelsShortBuffer(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ShortBuffer
	) {
		readPixels(x, y, width, height, format, type, pixels)
	}


	fun readPixelsIntBuffer(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: IntBuffer
	) {
		readPixels(x, y, width, height, format, type, pixels)
	}

	fun readPixelsFloatBuffer(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: FloatBuffer
	) {
		readPixels(x, y, width, height, format, type, pixels)
	}


	fun readPixels(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ByteBuffer
	) {
		ensureContextIsCurrent()
		//	clearIfComposited()
		GLES20.glReadPixels(x, y, width, height, format, type, pixels)

	}

	fun readPixels(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ShortBuffer
	) {
		ensureContextIsCurrent()
		// clearIfComposited()
		GLES20.glReadPixels(x, y, width, height, format, type, pixels)

	}

	fun readPixels(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: IntBuffer
	) {
		ensureContextIsCurrent()
		//	clearIfComposited()
		GLES20.glReadPixels(x, y, width, height, format, type, pixels)

	}


	fun readPixels(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: FloatBuffer
	) {
		ensureContextIsCurrent()
		//	clearIfComposited()
		GLES20.glReadPixels(x, y, width, height, format, type, pixels)

	}


	fun readPixels(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ByteArray
	) {
		ensureContextIsCurrent()
		//	clearIfComposited()
		GLES20.glReadPixels(x, y, width, height, format, type, ByteBuffer.wrap(pixels))

	}

	fun readPixels(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ShortArray
	) {
		ensureContextIsCurrent()
		//	clearIfComposited()
		GLES20.glReadPixels(x, y, width, height, format, type, ShortBuffer.wrap(pixels))

	}

	fun readPixels(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: FloatArray
	) {
		ensureContextIsCurrent()
		//	clearIfComposited()
		GLES20.glReadPixels(x, y, width, height, format, type, FloatBuffer.wrap(pixels))

	}

	fun readPixels(
		x: Int,
		y: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: IntArray
	) {
		ensureContextIsCurrent()
		//	clearIfComposited()
		GLES20.glReadPixels(x, y, width, height, format, type, IntBuffer.wrap(pixels))

	}

	fun renderbufferStorage(target: Int, internalFormat: Int, width: Int, height: Int) {
		ensureContextIsCurrent()
		GLES20.glRenderbufferStorage(target, internalFormat, width, height)

	}

	fun sampleCoverage(value: Float, invert: Boolean) {
		ensureContextIsCurrent()
		GLES20.glSampleCoverage(value, invert)

	}

	fun scissor(x: Int, y: Int, width: Int, height: Int) {
		ensureContextIsCurrent()
		GLES20.glScissor(x, y, width, height)

	}

	fun shaderSource(shader: Int, source: String?) {
		ensureContextIsCurrent()
		GLES20.glShaderSource(shader, source)

	}

	fun stencilFunc(func: Int, ref: Int, mask: Int) {
		ensureContextIsCurrent()
		GLES20.glStencilFunc(func, ref, mask)

	}

	fun stencilFuncSeparate(face: Int, func: Int, ref: Int, mask: Int) {

		when (face) {
			GLES20.GL_FRONT_AND_BACK -> {
				canvas.renderer.mStencilFuncRef = ref
				canvas.renderer.mStencilFuncRefBack = ref
				canvas.renderer.mStencilFuncMask = mask
				canvas.renderer.mStencilFuncMaskBack = mask
			}
			GLES20.GL_FRONT -> {
				canvas.renderer.mStencilFuncRef = ref
				canvas.renderer.mStencilFuncMask = mask
			}
			GLES20.GL_BACK -> {
				canvas.renderer.mStencilFuncRefBack = ref
				canvas.renderer.mStencilFuncMaskBack = mask
			}
			else -> {
			}
		}
		ensureContextIsCurrent()
		GLES20.glStencilFuncSeparate(face, func, ref, mask)

	}

	fun stencilMask(mask: Int) {

		canvas.renderer.mStencilMask = mask
		canvas.renderer.mStencilMaskBack = mask
		ensureContextIsCurrent()
		GLES20.glStencilMask(mask)

	}

	fun stencilMaskSeparate(face: Int, mask: Int) {

		when (face) {
			GLES20.GL_FRONT_AND_BACK -> {
				canvas.renderer.mStencilMask = mask
				canvas.renderer.mStencilMaskBack = mask
			}
			GLES20.GL_FRONT -> {
				canvas.renderer.mStencilMask = mask
			}
			GLES20.GL_BACK -> canvas.renderer.mStencilMaskBack = mask
			else -> {
			}
		}
		ensureContextIsCurrent()
		GLES20.glStencilMaskSeparate(face, mask)

	}

	fun stencilOp(fail: Int, zfail: Int, zpass: Int) {
		ensureContextIsCurrent()
		GLES20.glStencilOp(fail, zfail, zpass)

	}

	fun stencilOpSeparate(face: Int, fail: Int, zfail: Int, zpass: Int) {
		ensureContextIsCurrent()
		GLES20.glStencilOpSeparate(face, fail, zfail, zpass)

	}

	fun texImage2DByteBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: ByteBuffer?
	) {
		texImage2D(target, level, internalformat, width, height, border, format, type, pixels)
	}


	fun texImage2DShortBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: ShortBuffer?
	) {
		texImage2D(target, level, internalformat, width, height, border, format, type, pixels)
	}


	fun texImage2DIntBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: IntBuffer?
	) {
		texImage2D(target, level, internalformat, width, height, border, format, type, pixels)
	}


	fun texImage2DFloatBuffer(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: FloatBuffer?
	) {
		texImage2D(target, level, internalformat, width, height, border, format, type, pixels)
	}


	fun texImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: ByteBuffer?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			if (it.isDirect) {
				nativeTexImage2DBuffer(
					target,
					level,
					internalformat,
					width,
					height,
					border,
					format,
					type,
					it,
					flipYWebGL
				)
			} else {
				nativeTexImage2DByteArray(
					target,
					level,
					internalformat,
					width,
					height,
					border,
					format,
					type,
					it.array(),
					flipYWebGL
				)
			}

		} ?: run {
			GLES20.glTexImage2D(
				target,
				level,
				internalformat,
				width,
				height,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: ShortBuffer?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			if (it.isDirect) {
				nativeTexImage2DBuffer(
					target,
					level,
					internalformat,
					width,
					height,
					border,
					format,
					type,
					it,
					flipYWebGL
				)
			} else {
				nativeTexImage2DShortArray(
					target,
					level,
					internalformat,
					width,
					height,
					border,
					format,
					type,
					it.array(),
					flipYWebGL
				)
			}

		} ?: run {
			GLES20.glTexImage2D(
				target,
				level,
				internalformat,
				width,
				height,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: IntBuffer?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			if (it.isDirect) {
				nativeTexImage2DBuffer(
					target,
					level,
					internalformat,
					width,
					height,
					border,
					format,
					type,
					it,
					flipYWebGL
				)
			} else {
				nativeTexImage2DIntArray(
					target,
					level,
					internalformat,
					width,
					height,
					border,
					format,
					type,
					it.array(),
					flipYWebGL
				)
			}

		} ?: run {
			GLES20.glTexImage2D(
				target,
				level,
				internalformat,
				width,
				height,
				border,
				format,
				type,
				null
			)
		}

	}


	fun texImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: FloatBuffer?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			if (it.isDirect) {
				nativeTexImage2DBuffer(
					target,
					level,
					internalformat,
					width,
					height,
					border,
					format,
					type,
					it,
					flipYWebGL
				)
			} else {
				nativeTexImage2DFloatArray(
					target,
					level,
					internalformat,
					width,
					height,
					border,
					format,
					type,
					it.array(),
					flipYWebGL
				)
			}
		} ?: run {
			GLES20.glTexImage2D(
				target,
				level,
				internalformat,
				width,
				height,
				border,
				format,
				type,
				null
			)
		}

	}

	fun texImage2DByte(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: ByteArray?
	) {
		texImage2D(target, level, internalformat, width, height, border, format, type, pixels)
	}

	fun texImage2DShort(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: ShortArray?
	) {
		texImage2D(target, level, internalformat, width, height, border, format, type, pixels)
	}

	fun texImage2DInt(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: IntArray?
	) {
		texImage2D(target, level, internalformat, width, height, border, format, type, pixels)
	}

	fun texImage2DFloat(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: FloatArray?
	) {
		texImage2D(target, level, internalformat, width, height, border, format, type, pixels)
	}


	fun texImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: ByteArray?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			nativeTexImage2DByteArray(
				target,
				level,
				internalformat,
				width,
				height,
				border,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES20.glTexImage2D(
				target,
				level,
				internalformat,
				width,
				height,
				border,
				format,
				type,
				null
			)
		}

	}

	fun texImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: ShortArray?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			nativeTexImage2DShortArray(
				target,
				level,
				internalformat,
				width,
				height,
				border,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: kotlin.run {
			GLES20.glTexImage2D(
				target,
				level,
				internalformat,
				width,
				height,
				border,
				format,
				type,
				null
			)
		}

	}

	fun texImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: IntArray?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			nativeTexImage2DIntArray(
				target,
				level,
				internalformat,
				width,
				height,
				border,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: kotlin.run {
			GLES20.glTexImage2D(
				target,
				level,
				internalformat,
				width,
				height,
				border,
				format,
				type,
				null
			)
		}

	}

	fun texImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		width: Int,
		height: Int,
		border: Int,
		format: Int,
		type: Int,
		pixels: FloatArray?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			nativeTexImage2DFloatArray(
				target,
				level,
				internalformat,
				width,
				height,
				border,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: kotlin.run {
			GLES20.glTexImage2D(
				target,
				level,
				internalformat,
				width,
				height,
				border,
				format,
				type,
				null
			)
		}

	}

	fun texImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		format: Int,
		type: Int,
		source: TNSCanvas
	) {


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
			nativeTexImage2DBitmap(
				target,
				level,
				internalformat,
				source.width,
				source.height,
				0,
				format,
				type,
				bitmap,
				flipYWebGL
			)
		}
		if (ss != null) {
			nativeTexImage2DBuffer(
				target,
				level,
				internalformat,
				source.width,
				source.height,
				0,
				format,
				type,
				ss,
				flipYWebGL
			)
		}

	}

	fun texImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		format: Int,
		type: Int,
		bitmap: TNSImageBitmap
	) {
		ensureContextIsCurrent()
		nativeTexImage2DAsset(
			target,
			level,
			internalformat,
			0,
			format,
			type,
			bitmap.nativeImageAsset,
			flipYWebGL
		)

	}

	fun texImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		format: Int,
		type: Int,
		asset: TNSImageAsset
	) {
		ensureContextIsCurrent()
		nativeTexImage2DAsset(
			target,
			level,
			internalformat,
			0,
			format,
			type,
			asset.nativeImageAsset,
			flipYWebGL
		)

	}

	fun texImage2D(
		target: Int,
		level: Int,
		internalformat: Int,
		format: Int,
		type: Int,
		pixels: Bitmap
	) {
		ensureContextIsCurrent()
//			GLUtils.texImage2D(
//				target, level, internalformat, pixels, format, 0
//			)
		nativeTexImage2DBitmap(
			target,
			level,
			internalformat,
			pixels.width,
			pixels.width,
			0,
			format,
			type,
			pixels,
			flipYWebGL
		)

	}

	fun texParameterf(target: Int, pname: Int, param: Float) {
		ensureContextIsCurrent()
		GLES20.glTexParameterf(target, pname, param)

	}

	fun texParameteri(target: Int, pname: Int, param: Int) {
		ensureContextIsCurrent()
		GLES20.glTexParameteri(target, pname, param)

	}


	fun texSubImage2DByte(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ByteArray?
	) {
		texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels)
	}

	fun texSubImage2DShort(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ShortArray?
	) {
		texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels)
	}

	fun texSubImage2DInt(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: IntArray?
	) {
		texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels)
	}


	fun texSubImage2DByteBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ByteBuffer?
	) {
		texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels)
	}

	fun texSubImage2DShortBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ShortBuffer?
	) {
		texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels)
	}

	fun texSubImage2DIntBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: IntBuffer?
	) {
		texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels)
	}

	fun texSubImage2DFloatBuffer(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: FloatBuffer?
	) {
		texSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels)
	}


	fun texSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ByteBuffer?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			if (it.isDirect) {
				nativeTexSubImage2DBuffer(
					target,
					level,
					xoffset,
					yoffset,
					width,
					height,
					format,
					type,
					it,
					flipYWebGL
				)
			} else {
				nativeTexSubImage2DByteArray(
					target,
					level,
					xoffset,
					yoffset,
					width,
					height,
					format,
					type,
					it.array(),
					flipYWebGL
				)

			}
		} ?: run {
			GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, null)
		}

	}


	fun texSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ShortBuffer?
	) {
		ensureContextIsCurrent()
		GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels)

	}


	fun texSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: IntBuffer?
	) {
		ensureContextIsCurrent()
		GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels)

	}


	fun texSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: FloatBuffer?
	) {
		ensureContextIsCurrent()
		GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels)

	}


	fun texSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ByteArray?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			nativeTexSubImage2DByteArray(
				target,
				level,
				xoffset,
				yoffset,
				width,
				height,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, null)
		}

	}

	fun texSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: ShortArray?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			nativeTexSubImage2DShortArray(
				target,
				level,
				xoffset,
				yoffset,
				width,
				height,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, null)
		}

	}

	fun texSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: IntArray?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			nativeTexSubImage2DIntArray(
				target,
				level,
				xoffset,
				yoffset,
				width,
				height,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, null)
		}

	}

	fun texSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		width: Int,
		height: Int,
		format: Int,
		type: Int,
		pixels: FloatArray?
	) {
		ensureContextIsCurrent()
		pixels?.let {
			nativeTexSubImage2DFloatArray(
				target,
				level,
				xoffset,
				yoffset,
				width,
				height,
				format,
				type,
				it,
				flipYWebGL
			)
		} ?: run {
			GLES20.glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, null)
		}

	}

	fun texSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		format: Int,
		type: Int,
		source: TNSCanvas
	) {

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
			nativeTexSubImage2DBitmap(
				target,
				level,
				xoffset,
				yoffset,
				source.width,
				source.height,
				format,
				type,
				bitmap,
				flipYWebGL
			)
		}
		if (ss != null) {
			nativeTexSubImage2DBuffer(
				target,
				level,
				xoffset,
				yoffset,
				source.width,
				source.height,
				format,
				type,
				ss,
				flipYWebGL
			)
		}

	}

	fun texSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		format: Int,
		type: Int,
		pixels: Bitmap
	) {
		ensureContextIsCurrent()
		nativeTexSubImage2DBitmap(
			target,
			level,
			xoffset,
			yoffset,
			pixels.width,
			pixels.height,
			format,
			type,
			pixels,
			flipYWebGL
		)

	}

	fun texSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		format: Int,
		type: Int,
		bitmap: TNSImageBitmap
	) {
		ensureContextIsCurrent()
		nativeTexSubImage2DAsset(
			target,
			level,
			xoffset,
			yoffset,
			format,
			type,
			bitmap.nativeImageAsset,
			flipYWebGL
		)

	}

	fun texSubImage2D(
		target: Int,
		level: Int,
		xoffset: Int,
		yoffset: Int,
		format: Int,
		type: Int,
		asset: TNSImageAsset
	) {
		ensureContextIsCurrent()
		nativeTexSubImage2DAsset(
			target,
			level,
			xoffset,
			yoffset,
			format,
			type,
			asset.nativeImageAsset,
			flipYWebGL
		)

	}

	fun uniform1f(location: Int, v0: Float) {
		ensureContextIsCurrent()
		GLES20.glUniform1f(location, v0)

	}

	fun uniform1fv(location: Int, value: FloatArray?) {
		ensureContextIsCurrent()
		val count = value?.size?.div(1) ?: 1
		GLES20.glUniform1fv(location, count, value, 0)

	}

	fun uniform1fvBuffer(location: Int, value: FloatBuffer?) {
		ensureContextIsCurrent()
		val count = value?.capacity()?.div(1) ?: 1
		GLES20.glUniform1fv(location, count, value)

	}

	fun uniform1i(location: Int, v0: Int) {
		ensureContextIsCurrent()
		GLES20.glUniform1i(location, v0)

	}

	fun uniform1iv(location: Int, value: IntArray?) {
		ensureContextIsCurrent()
		val count = value?.size?.div(1) ?: 1
		GLES20.glUniform1iv(location, count, value, 0)

	}

	fun uniform1ivBuffer(location: Int, value: IntBuffer?) {
		ensureContextIsCurrent()
		val count = value?.capacity()?.div(1) ?: 1
		GLES20.glUniform1iv(location, count, value)

	}

	fun uniform2f(location: Int, v0: Float, v1: Float) {
		ensureContextIsCurrent()
		GLES20.glUniform2f(location, v0, v1)

	}

	fun uniform2fv(location: Int, value: FloatArray?) {
		ensureContextIsCurrent()
		val count = value?.size?.div(2) ?: 2
		GLES20.glUniform2fv(location, count, value, 0)

	}

	fun uniform2fvBuffer(location: Int, value: FloatBuffer?) {
		ensureContextIsCurrent()
		val count = value?.capacity()?.div(2) ?: 2
		GLES20.glUniform2fv(location, count, value)

	}

	fun uniform2i(location: Int, v0: Int, v1: Int) {
		ensureContextIsCurrent()
		GLES20.glUniform2i(location, v0, v1)

	}

	fun uniform2iv(location: Int, value: IntArray?) {
		ensureContextIsCurrent()
		val count = value?.size?.div(2) ?: 2
		GLES20.glUniform2iv(location, count, value, 0)

	}

	fun uniform2ivBuffer(location: Int, value: IntBuffer?) {
		ensureContextIsCurrent()
		val count = value?.capacity()?.div(2) ?: 2
		GLES20.glUniform2iv(location, count, value)

	}

	fun uniform3f(location: Int, v0: Float, v1: Float, v2: Float) {
		ensureContextIsCurrent()
		GLES20.glUniform3f(location, v0, v1, v2)

	}

	fun uniform3fv(location: Int, value: FloatArray?) {
		ensureContextIsCurrent()
		val count = value?.size?.div(3) ?: 3
		GLES20.glUniform3fv(location, count, value, 0)

	}

	fun uniform3fvBuffer(location: Int, value: FloatBuffer?) {
		ensureContextIsCurrent()
		val count = value?.capacity()?.div(3) ?: 3
		GLES20.glUniform3fv(location, count, value)

	}

	fun uniform3i(location: Int, v0: Int, v1: Int, v2: Int) {
		ensureContextIsCurrent()
		GLES20.glUniform3i(location, v0, v1, v2)

	}

	fun uniform3iv(location: Int, value: IntArray?) {
		ensureContextIsCurrent()
		val count = value?.size?.div(3) ?: 3
		GLES20.glUniform3iv(location, count, value, 0)

	}

	fun uniform3ivBuffer(location: Int, value: IntBuffer?) {
		ensureContextIsCurrent()
		val count = value?.capacity()?.div(3) ?: 3
		GLES20.glUniform3iv(location, count, value)

	}

	fun uniform4f(location: Int, v0: Float, v1: Float, v2: Float, v3: Float) {
		ensureContextIsCurrent()
		GLES20.glUniform4f(location, v0, v1, v2, v3)

	}

	fun uniform4fv(location: Int, value: FloatArray?) {
		ensureContextIsCurrent()
		val count = value?.size?.div(4) ?: 4
		GLES20.glUniform4fv(location, count, value, 0)

	}

	fun uniform4fvBuffer(location: Int, value: FloatBuffer?) {
		ensureContextIsCurrent()
		val count = value?.capacity()?.div(4) ?: 4
		GLES20.glUniform4fv(location, count, value)

	}

	fun uniform4i(location: Int, v0: Int, v1: Int, v2: Int, v3: Int) {
		ensureContextIsCurrent()
		GLES20.glUniform4i(location, v0, v1, v2, v3)

	}

	fun uniform4iv(location: Int, value: IntArray?) {
		ensureContextIsCurrent()
		val count = value?.size?.div(4) ?: 4
		GLES20.glUniform4iv(location, count, value, 0)

	}

	fun uniform4ivBuffer(location: Int, value: IntBuffer?) {
		ensureContextIsCurrent()
		val count = value?.capacity()?.div(4) ?: 4
		GLES20.glUniform4iv(location, count, value)

	}

	fun uniformMatrix2fv(location: Int, transpose: Boolean, value: FloatArray?) {
		ensureContextIsCurrent()
		val count = value?.size?.div(4) ?: 4
		GLES20.glUniformMatrix2fv(location, count, transpose, value, 0)

	}

	fun uniformMatrix2fvBuffer(location: Int, transpose: Boolean, value: FloatBuffer?) {
		ensureContextIsCurrent()
		val count = value?.capacity()?.div(4) ?: 4
		GLES20.glUniformMatrix2fv(location, count, transpose, value)

	}

	fun uniformMatrix3fv(location: Int, transpose: Boolean, value: FloatArray?) {
		ensureContextIsCurrent()
		val count = value?.size?.div(9) ?: 9
		GLES20.glUniformMatrix3fv(location, count, transpose, value, 0)

	}

	fun uniformMatrix3fvBuffer(location: Int, transpose: Boolean, value: FloatBuffer?) {
		ensureContextIsCurrent()
		val count = value?.capacity()?.div(9) ?: 9
		GLES20.glUniformMatrix3fv(location, count, transpose, value)

	}

	fun uniformMatrix4fv(location: Int, transpose: Boolean, value: FloatArray?) {
		ensureContextIsCurrent()
		val count = value?.size?.div(16) ?: 16
		GLES20.glUniformMatrix4fv(location, count, transpose, value, 0)

	}

	fun uniformMatrix4fvBuffer(location: Int, transpose: Boolean, value: FloatBuffer?) {
		ensureContextIsCurrent()
		val count = value?.capacity()?.div(16) ?: 16
		GLES20.glUniformMatrix4fv(location, count, transpose, value)

	}

	fun useProgram(program: Int) {
		ensureContextIsCurrent()
		GLES20.glUseProgram(program)

	}

	fun validateProgram(program: Int) {
		ensureContextIsCurrent()
		GLES20.glValidateProgram(program)

	}

	fun vertexAttrib1f(index: Int, v0: Float) {
		ensureContextIsCurrent()
		GLES20.glVertexAttrib1f(index, v0)

	}

	fun vertexAttrib2f(index: Int, v0: Float, v1: Float) {
		ensureContextIsCurrent()
		GLES20.glVertexAttrib2f(index, v0, v1)

	}

	fun vertexAttrib3f(index: Int, v0: Float, v1: Float, v2: Float) {
		ensureContextIsCurrent()
		GLES20.glVertexAttrib3f(index, v0, v1, v2)

	}

	fun vertexAttrib4f(index: Int, v0: Float, v1: Float, v2: Float, v3: Float) {
		ensureContextIsCurrent()
		GLES20.glVertexAttrib4f(index, v0, v1, v2, v3)

	}

	fun vertexAttrib1fv(index: Int, value: FloatArray?) {
		ensureContextIsCurrent()
		GLES20.glVertexAttrib1fv(index, value, 0)

	}

	fun vertexAttrib1fvBuffer(index: Int, value: FloatBuffer?) {
		ensureContextIsCurrent()
		GLES20.glVertexAttrib1fv(index, value)

	}

	fun vertexAttrib2fv(index: Int, value: FloatArray?) {
		ensureContextIsCurrent()
		GLES20.glVertexAttrib2fv(index, value, 0)

	}

	fun vertexAttrib2fvBuffer(index: Int, value: FloatBuffer?) {
		ensureContextIsCurrent()
		GLES20.glVertexAttrib2fv(index, value)

	}

	fun vertexAttrib3fv(index: Int, value: FloatArray?) {
		ensureContextIsCurrent()
		GLES20.glVertexAttrib3fv(index, value, 0)

	}

	fun vertexAttrib3fvBuffer(index: Int, value: FloatBuffer?) {
		ensureContextIsCurrent()
		GLES20.glVertexAttrib3fv(index, value)
	}

	fun vertexAttrib4fv(index: Int, value: FloatArray?) {
		ensureContextIsCurrent()
		GLES20.glVertexAttrib4fv(index, value, 0)
	}

	fun vertexAttrib4fvBuffer(index: Int, value: FloatBuffer?) {
		ensureContextIsCurrent()
		GLES20.glVertexAttrib4fv(index, value)
	}

	fun vertexAttribPointer(
		index: Int,
		size: Int,
		type: Int,
		normalized: Boolean,
		stride: Int,
		offset: Int
	) {
		// GLES20.glVertexAttribPointer(index, size, type, normalized, stride, offset);
		ensureContextIsCurrent()
		nativeVertexAttribPointer(index, size, type, normalized, stride, offset)
	}

	fun viewport(x: Int, y: Int, width: Int, height: Int) {
		ensureContextIsCurrent()
		GLES20.glViewport(x, y, width, height)
	}

	/* Clearing buffers */
	val DEPTH_BUFFER_BIT = GLES20.GL_DEPTH_BUFFER_BIT
	val COLOR_BUFFER_BIT = GLES20.GL_COLOR_BUFFER_BIT
	val STENCIL_BUFFER_BIT = GLES20.GL_STENCIL_BUFFER_BIT

	/* Clearing buffers */ /* Rendering primitives */
	val POINTS = GLES20.GL_POINTS
	val LINES = GLES20.GL_LINES
	val LINE_LOOP = GLES20.GL_LINE_LOOP
	val LINE_STRIP = GLES20.GL_LINE_STRIP
	val TRIANGLES = GLES20.GL_TRIANGLES
	val TRIANGLE_STRIP = GLES20.GL_TRIANGLE_STRIP
	val TRIANGLE_FAN = GLES20.GL_TRIANGLE_FAN

	/* Rendering primitives */ /* Blending modes */
	val ONE = GLES20.GL_ONE
	val ZERO = GLES20.GL_ZERO
	val SRC_COLOR = GLES20.GL_SRC_COLOR
	val ONE_MINUS_SRC_COLOR = GLES20.GL_ONE_MINUS_SRC_COLOR
	val SRC_ALPHA = GLES20.GL_SRC_ALPHA
	val ONE_MINUS_SRC_ALPHA = GLES20.GL_ONE_MINUS_SRC_ALPHA
	val DST_ALPHA = GLES20.GL_DST_ALPHA
	val ONE_MINUS_DST_ALPHA = GLES20.GL_ONE_MINUS_DST_ALPHA
	val DST_COLOR = GLES20.GL_DST_COLOR
	val ONE_MINUS_DST_COLOR = GLES20.GL_ONE_MINUS_DST_COLOR
	val SRC_ALPHA_SATURATE = GLES20.GL_SRC_ALPHA_SATURATE
	val CONSTANT_COLOR = GLES20.GL_CONSTANT_COLOR
	val ONE_MINUS_CONSTANT_COLOR = GLES20.GL_ONE_MINUS_CONSTANT_COLOR
	val CONSTANT_ALPHA = GLES20.GL_CONSTANT_ALPHA
	val ONE_MINUS_CONSTANT_ALPHA = GLES20.GL_ONE_MINUS_CONSTANT_ALPHA

	/* Blending modes */ /* Blending equations */
	val FUNC_ADD = GLES20.GL_FUNC_ADD
	val FUNC_SUBTRACT = GLES20.GL_FUNC_SUBTRACT
	val FUNC_REVERSE_SUBTRACT = GLES20.GL_FUNC_REVERSE_SUBTRACT

	/* Blending equations */ /* Getting GL parameter information */
	val BLEND_EQUATION = GLES20.GL_BLEND_EQUATION
	val BLEND_EQUATION_RGB = GLES20.GL_BLEND_EQUATION_RGB
	val BLEND_EQUATION_ALPHA = GLES20.GL_BLEND_EQUATION_ALPHA
	val BLEND_DST_RGB = GLES20.GL_BLEND_DST_RGB
	val BLEND_SRC_RGB = GLES20.GL_BLEND_SRC_RGB
	val BLEND_DST_ALPHA = GLES20.GL_BLEND_DST_ALPHA
	val BLEND_SRC_ALPHA = GLES20.GL_BLEND_SRC_ALPHA
	val BLEND_COLOR = GLES20.GL_BLEND_COLOR
	val ARRAY_BUFFER_BINDING = GLES20.GL_ARRAY_BUFFER_BINDING
	val ELEMENT_ARRAY_BUFFER_BINDING = GLES20.GL_ELEMENT_ARRAY_BUFFER_BINDING
	val LINE_WIDTH = GLES20.GL_LINE_WIDTH
	val ALIASED_POINT_SIZE_RANGE = GLES20.GL_ALIASED_POINT_SIZE_RANGE
	val ALIASED_LINE_WIDTH_RANGE = GLES20.GL_ALIASED_LINE_WIDTH_RANGE
	val CULL_FACE_MODE = GLES20.GL_CULL_FACE_MODE
	val FRONT_FACE = GLES20.GL_FRONT_FACE
	val DEPTH_RANGE = GLES20.GL_DEPTH_RANGE
	val DEPTH_WRITEMASK = GLES20.GL_DEPTH_WRITEMASK
	val DEPTH_CLEAR_VALUE = GLES20.GL_DEPTH_CLEAR_VALUE
	val DEPTH_FUNC = GLES20.GL_DEPTH_FUNC
	val STENCIL_CLEAR_VALUE = GLES20.GL_STENCIL_CLEAR_VALUE
	val STENCIL_FUNC = GLES20.GL_STENCIL_FUNC
	val STENCIL_FAIL = GLES20.GL_STENCIL_FAIL
	val STENCIL_PASS_DEPTH_FAIL = GLES20.GL_STENCIL_PASS_DEPTH_FAIL
	val STENCIL_PASS_DEPTH_PASS = GLES20.GL_STENCIL_PASS_DEPTH_PASS
	val STENCIL_REF = GLES20.GL_STENCIL_REF
	val STENCIL_VALUE_MASK = GLES20.GL_STENCIL_VALUE_MASK
	val STENCIL_WRITEMASK = GLES20.GL_STENCIL_WRITEMASK
	val STENCIL_BACK_FUNC = GLES20.GL_STENCIL_BACK_FUNC
	val STENCIL_BACK_FAIL = GLES20.GL_STENCIL_BACK_FAIL
	val STENCIL_BACK_PASS_DEPTH_FAIL = GLES20.GL_STENCIL_BACK_PASS_DEPTH_FAIL
	val STENCIL_BACK_PASS_DEPTH_PASS = GLES20.GL_STENCIL_BACK_PASS_DEPTH_PASS
	val STENCIL_BACK_REF = GLES20.GL_STENCIL_BACK_REF
	val STENCIL_BACK_VALUE_MASK = GLES20.GL_STENCIL_BACK_VALUE_MASK
	val STENCIL_BACK_WRITEMASK = GLES20.GL_STENCIL_BACK_WRITEMASK
	val VIEWPORT = GLES20.GL_VIEWPORT
	val SCISSOR_BOX = GLES20.GL_SCISSOR_BOX
	val COLOR_CLEAR_VALUE = GLES20.GL_COLOR_CLEAR_VALUE
	val COLOR_WRITEMASK = GLES20.GL_COLOR_WRITEMASK
	val UNPACK_ALIGNMENT = GLES20.GL_UNPACK_ALIGNMENT
	val PACK_ALIGNMENT = GLES20.GL_PACK_ALIGNMENT
	val MAX_TEXTURE_SIZE = GLES20.GL_MAX_TEXTURE_SIZE
	val MAX_VIEWPORT_DIMS = GLES20.GL_MAX_VIEWPORT_DIMS
	val SUBPIXEL_BITS = GLES20.GL_SUBPIXEL_BITS
	val RED_BITS = GLES20.GL_RED_BITS
	val GREEN_BITS = GLES20.GL_GREEN_BITS
	val BLUE_BITS = GLES20.GL_BLUE_BITS
	val ALPHA_BITS = GLES20.GL_ALPHA_BITS
	val DEPTH_BITS = GLES20.GL_DEPTH_BITS
	val STENCIL_BITS = GLES20.GL_STENCIL_BITS
	val POLYGON_OFFSET_UNITS = GLES20.GL_POLYGON_OFFSET_UNITS
	val POLYGON_OFFSET_FACTOR = GLES20.GL_POLYGON_OFFSET_FACTOR
	val TEXTURE_BINDING_2D = GLES20.GL_TEXTURE_BINDING_2D
	val SAMPLE_BUFFERS = GLES20.GL_SAMPLE_BUFFERS
	val SAMPLES = GLES20.GL_SAMPLES
	val SAMPLE_COVERAGE_VALUE = GLES20.GL_SAMPLE_COVERAGE_VALUE
	val SAMPLE_COVERAGE_INVERT = GLES20.GL_SAMPLE_COVERAGE_INVERT
	val COMPRESSED_TEXTURE_FORMATS = GLES20.GL_COMPRESSED_TEXTURE_FORMATS
	val VENDOR = GLES20.GL_VENDOR
	val RENDERER = GLES20.GL_RENDERER
	val VERSION = GLES20.GL_VERSION
	val IMPLEMENTATION_COLOR_READ_TYPE = GLES20.GL_IMPLEMENTATION_COLOR_READ_TYPE
	val IMPLEMENTATION_COLOR_READ_FORMAT = GLES20.GL_IMPLEMENTATION_COLOR_READ_FORMAT
	val BROWSER_DEFAULT_WEBGL = 0x9244

	/* Getting GL parameter information */ /* Buffers */
	val STATIC_DRAW = GLES20.GL_STATIC_DRAW
	val STREAM_DRAW = GLES20.GL_STREAM_DRAW
	val DYNAMIC_DRAW = GLES20.GL_DYNAMIC_DRAW
	val ARRAY_BUFFER = GLES20.GL_ARRAY_BUFFER
	val ELEMENT_ARRAY_BUFFER = GLES20.GL_ELEMENT_ARRAY_BUFFER
	val BUFFER_SIZE = GLES20.GL_BUFFER_SIZE
	val BUFFER_USAGE = GLES20.GL_BUFFER_USAGE

	/* Buffers */ /* Vertex attributes */
	val CURRENT_VERTEX_ATTRIB = GLES20.GL_CURRENT_VERTEX_ATTRIB
	val VERTEX_ATTRIB_ARRAY_ENABLED = GLES20.GL_VERTEX_ATTRIB_ARRAY_ENABLED
	val VERTEX_ATTRIB_ARRAY_SIZE = GLES20.GL_VERTEX_ATTRIB_ARRAY_SIZE
	val VERTEX_ATTRIB_ARRAY_STRIDE = GLES20.GL_VERTEX_ATTRIB_ARRAY_STRIDE
	val VERTEX_ATTRIB_ARRAY_TYPE = GLES20.GL_VERTEX_ATTRIB_ARRAY_TYPE
	val VERTEX_ATTRIB_ARRAY_NORMALIZED = GLES20.GL_VERTEX_ATTRIB_ARRAY_NORMALIZED
	val VERTEX_ATTRIB_ARRAY_POINTER = GLES20.GL_VERTEX_ATTRIB_ARRAY_POINTER
	val VERTEX_ATTRIB_ARRAY_BUFFER_BINDING = GLES20.GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING

	/* Vertex attributes */ /* Culling */
	val CULL_FACE = GLES20.GL_CULL_FACE
	val FRONT = GLES20.GL_FRONT
	val BACK = GLES20.GL_BACK
	val FRONT_AND_BACK = GLES20.GL_FRONT_AND_BACK

	/* Culling */ /* Enabling and disabling */
	val BLEND = GLES20.GL_BLEND
	val DEPTH_TEST = GLES20.GL_DEPTH_TEST
	val DITHER = GLES20.GL_DITHER
	val POLYGON_OFFSET_FILL = GLES20.GL_POLYGON_OFFSET_FILL
	val SAMPLE_ALPHA_TO_COVERAGE = GLES20.GL_SAMPLE_ALPHA_TO_COVERAGE
	val SAMPLE_COVERAGE = GLES20.GL_SAMPLE_COVERAGE
	val SCISSOR_TEST = GLES20.GL_SCISSOR_TEST
	val STENCIL_TEST = GLES20.GL_STENCIL_TEST

	/* Enabling and disabling */ /* Errors */
	val NO_ERROR = GLES20.GL_NO_ERROR
	val INVALID_ENUM = GLES20.GL_INVALID_ENUM
	val INVALID_VALUE = GLES20.GL_INVALID_VALUE
	val INVALID_OPERATION = GLES20.GL_INVALID_OPERATION
	val INVALID_FRAMEBUFFER_OPERATION = GLES20.GL_INVALID_FRAMEBUFFER_OPERATION
	val OUT_OF_MEMORY = GLES20.GL_OUT_OF_MEMORY
	val CONTEXT_LOST_WEBGL = 0x9242

	/* Errors */ /* Front face directions */
	val CW = GLES20.GL_CW
	val CCW = GLES20.GL_CCW

	/* Front face directions */ /* Hints */
	val DONT_CARE = GLES20.GL_DONT_CARE
	val FASTEST = GLES20.GL_FASTEST
	val NICEST = GLES20.GL_NICEST
	val GENERATE_MIPMAP_HINT = GLES20.GL_GENERATE_MIPMAP_HINT

	/* Hints */ /* Data types */
	val BYTE = GLES20.GL_BYTE
	val UNSIGNED_BYTE = GLES20.GL_UNSIGNED_BYTE
	val UNSIGNED_SHORT = GLES20.GL_UNSIGNED_SHORT
	val SHORT = GLES20.GL_SHORT
	val UNSIGNED_INT = GLES20.GL_UNSIGNED_INT
	val INT = GLES20.GL_INT
	val FLOAT = GLES20.GL_FLOAT

	/* Data types */ /* Pixel formats */
	val DEPTH_COMPONENT = GLES20.GL_DEPTH_COMPONENT
	val ALPHA = GLES20.GL_ALPHA
	val RGB = GLES20.GL_RGB
	val RGBA = GLES20.GL_RGBA
	val LUMINANCE = GLES20.GL_LUMINANCE
	val LUMINANCE_ALPHA = GLES20.GL_LUMINANCE_ALPHA

	/* Pixel formats */ /* Pixel types */ // public final int UNSIGNED_BYTE = GLES20.GL_UNSIGNED_BYTE;
	val UNSIGNED_SHORT_4_4_4_4 = GLES20.GL_UNSIGNED_SHORT_4_4_4_4
	val UNSIGNED_SHORT_5_5_5_1 = GLES20.GL_UNSIGNED_SHORT_5_5_5_1
	val UNSIGNED_SHORT_5_6_5 = GLES20.GL_UNSIGNED_SHORT_5_6_5

	/* Pixel types */ /* Shaders */
	val FRAGMENT_SHADER = GLES20.GL_FRAGMENT_SHADER
	val VERTEX_SHADER = GLES20.GL_VERTEX_SHADER
	val COMPILE_STATUS = GLES20.GL_COMPILE_STATUS
	val DELETE_STATUS = GLES20.GL_DELETE_STATUS
	val LINK_STATUS = GLES20.GL_LINK_STATUS
	val VALIDATE_STATUS = GLES20.GL_VALIDATE_STATUS
	val ATTACHED_SHADERS = GLES20.GL_ATTACHED_SHADERS
	val ACTIVE_ATTRIBUTES = GLES20.GL_ACTIVE_ATTRIBUTES
	val ACTIVE_UNIFORMS = GLES20.GL_ACTIVE_UNIFORMS
	val MAX_VERTEX_UNIFORM_VECTORS = GLES20.GL_MAX_VERTEX_UNIFORM_VECTORS
	val MAX_VARYING_VECTORS = GLES20.GL_MAX_VARYING_VECTORS
	val MAX_COMBINED_TEXTURE_IMAGE_UNITS = GLES20.GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS
	val MAX_VERTEX_TEXTURE_IMAGE_UNITS = GLES20.GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS
	val MAX_TEXTURE_IMAGE_UNITS = GLES20.GL_MAX_TEXTURE_IMAGE_UNITS
	val MAX_VERTEX_ATTRIBS = GLES20.GL_MAX_VERTEX_ATTRIBS
	val MAX_FRAGMENT_UNIFORM_VECTORS = GLES20.GL_MAX_FRAGMENT_UNIFORM_VECTORS
	val SHADER_TYPE = GLES20.GL_SHADER_TYPE
	val SHADING_LANGUAGE_VERSION = GLES20.GL_SHADING_LANGUAGE_VERSION
	val CURRENT_PROGRAM = GLES20.GL_CURRENT_PROGRAM

	/* Shaders */ /* Depth or stencil tests */
	val NEVER = GLES20.GL_NEVER
	val LESS = GLES20.GL_LESS
	val EQUAL = GLES20.GL_EQUAL
	val LEQUAL = GLES20.GL_LEQUAL
	val GREATER = GLES20.GL_GREATER
	val NOTEQUAL = GLES20.GL_NOTEQUAL
	val GEQUAL = GLES20.GL_GEQUAL
	val ALWAYS = GLES20.GL_ALWAYS

	/* Depth or stencil tests */ /* Stencil actions */
	val KEEP = GLES20.GL_KEEP
	val REPLACE = GLES20.GL_REPLACE
	val INCR = GLES20.GL_INCR
	val DECR = GLES20.GL_DECR
	val INVERT = GLES20.GL_INVERT
	val INCR_WRAP = GLES20.GL_INCR_WRAP
	val DECR_WRAP = GLES20.GL_DECR_WRAP

	/* Stencil actions */ /* Textures */
	val NEAREST = GLES20.GL_NEAREST
	val LINEAR = GLES20.GL_LINEAR
	val NEAREST_MIPMAP_NEAREST = GLES20.GL_NEAREST_MIPMAP_NEAREST
	val LINEAR_MIPMAP_NEAREST = GLES20.GL_LINEAR_MIPMAP_NEAREST
	val NEAREST_MIPMAP_LINEAR = GLES20.GL_NEAREST_MIPMAP_LINEAR
	val LINEAR_MIPMAP_LINEAR = GLES20.GL_LINEAR_MIPMAP_LINEAR
	val TEXTURE_MAG_FILTER = GLES20.GL_TEXTURE_MAG_FILTER
	val TEXTURE_MIN_FILTER = GLES20.GL_TEXTURE_MIN_FILTER
	val TEXTURE_WRAP_S = GLES20.GL_TEXTURE_WRAP_S
	val TEXTURE_WRAP_T = GLES20.GL_TEXTURE_WRAP_T
	val TEXTURE_2D = GLES20.GL_TEXTURE_2D
	val TEXTURE = GLES20.GL_TEXTURE
	val TEXTURE_CUBE_MAP = GLES20.GL_TEXTURE_CUBE_MAP
	val TEXTURE_BINDING_CUBE_MAP = GLES20.GL_TEXTURE_BINDING_CUBE_MAP
	val TEXTURE_CUBE_MAP_POSITIVE_X = GLES20.GL_TEXTURE_CUBE_MAP_POSITIVE_X
	val TEXTURE_CUBE_MAP_NEGATIVE_X = GLES20.GL_TEXTURE_CUBE_MAP_NEGATIVE_X
	val TEXTURE_CUBE_MAP_POSITIVE_Y = GLES20.GL_TEXTURE_CUBE_MAP_POSITIVE_Y
	val TEXTURE_CUBE_MAP_NEGATIVE_Y = GLES20.GL_TEXTURE_CUBE_MAP_NEGATIVE_Y
	val TEXTURE_CUBE_MAP_POSITIVE_Z = GLES20.GL_TEXTURE_CUBE_MAP_POSITIVE_Z
	val TEXTURE_CUBE_MAP_NEGATIVE_Z = GLES20.GL_TEXTURE_CUBE_MAP_NEGATIVE_Z
	val MAX_CUBE_MAP_TEXTURE_SIZE = GLES20.GL_MAX_CUBE_MAP_TEXTURE_SIZE
	val TEXTURE0 = GLES20.GL_TEXTURE0
	val TEXTURE1 = GLES20.GL_TEXTURE1
	val TEXTURE2 = GLES20.GL_TEXTURE2
	val TEXTURE3 = GLES20.GL_TEXTURE3
	val TEXTURE4 = GLES20.GL_TEXTURE4
	val TEXTURE5 = GLES20.GL_TEXTURE5
	val TEXTURE6 = GLES20.GL_TEXTURE6
	val TEXTURE7 = GLES20.GL_TEXTURE7
	val TEXTURE8 = GLES20.GL_TEXTURE8
	val TEXTURE9 = GLES20.GL_TEXTURE9
	val TEXTURE10 = GLES20.GL_TEXTURE10
	val TEXTURE11 = GLES20.GL_TEXTURE11
	val TEXTURE12 = GLES20.GL_TEXTURE12
	val TEXTURE13 = GLES20.GL_TEXTURE13
	val TEXTURE14 = GLES20.GL_TEXTURE14
	val TEXTURE15 = GLES20.GL_TEXTURE15
	val TEXTURE16 = GLES20.GL_TEXTURE16
	val TEXTURE17 = GLES20.GL_TEXTURE17
	val TEXTURE18 = GLES20.GL_TEXTURE18
	val TEXTURE19 = GLES20.GL_TEXTURE19
	val TEXTURE20 = GLES20.GL_TEXTURE20
	val TEXTURE21 = GLES20.GL_TEXTURE21
	val TEXTURE22 = GLES20.GL_TEXTURE22
	val TEXTURE23 = GLES20.GL_TEXTURE23
	val TEXTURE24 = GLES20.GL_TEXTURE24
	val TEXTURE25 = GLES20.GL_TEXTURE25
	val TEXTURE26 = GLES20.GL_TEXTURE26
	val TEXTURE27 = GLES20.GL_TEXTURE27
	val TEXTURE28 = GLES20.GL_TEXTURE28
	val TEXTURE29 = GLES20.GL_TEXTURE29
	val TEXTURE30 = GLES20.GL_TEXTURE30
	val TEXTURE31 = GLES20.GL_TEXTURE31
	val ACTIVE_TEXTURE = GLES20.GL_ACTIVE_TEXTURE
	val REPEAT = GLES20.GL_REPEAT
	val CLAMP_TO_EDGE = GLES20.GL_CLAMP_TO_EDGE
	val MIRRORED_REPEAT = GLES20.GL_MIRRORED_REPEAT

	/* Textures */ /* Uniform types */
	val FLOAT_VEC2 = GLES20.GL_FLOAT_VEC2
	val FLOAT_VEC3 = GLES20.GL_FLOAT_VEC3
	val FLOAT_VEC4 = GLES20.GL_FLOAT_VEC4
	val INT_VEC2 = GLES20.GL_INT_VEC2
	val INT_VEC3 = GLES20.GL_INT_VEC3
	val INT_VEC4 = GLES20.GL_INT_VEC4
	val BOOL = GLES20.GL_BOOL
	val BOOL_VEC2 = GLES20.GL_BOOL_VEC2
	val BOOL_VEC3 = GLES20.GL_BOOL_VEC3
	val BOOL_VEC4 = GLES20.GL_BOOL_VEC4
	val FLOAT_MAT2 = GLES20.GL_FLOAT_MAT2
	val FLOAT_MAT3 = GLES20.GL_FLOAT_MAT3
	val FLOAT_MAT4 = GLES20.GL_FLOAT_MAT4
	val SAMPLER_2D = GLES20.GL_SAMPLER_2D
	val SAMPLER_CUBE = GLES20.GL_SAMPLER_CUBE

	/* Uniform types */ /* Shader precision-specified types */
	val LOW_FLOAT = GLES20.GL_LOW_FLOAT
	val MEDIUM_FLOAT = GLES20.GL_MEDIUM_FLOAT
	val HIGH_FLOAT = GLES20.GL_HIGH_FLOAT
	val LOW_INT = GLES20.GL_LOW_INT
	val MEDIUM_INT = GLES20.GL_MEDIUM_INT
	val HIGH_INT = GLES20.GL_HIGH_INT

	/* Shader precision-specified types */ /* Framebuffers and renderbuffers */
	val FRAMEBUFFER = GLES20.GL_FRAMEBUFFER
	val RENDERBUFFER = GLES20.GL_RENDERBUFFER
	val RGBA4 = GLES20.GL_RGBA4
	val RGB565 = GLES20.GL_RGB565
	val RGB5_A1 = GLES20.GL_RGB5_A1
	val DEPTH_COMPONENT16 = GLES20.GL_DEPTH_COMPONENT16
	val STENCIL_INDEX8 = GLES20.GL_STENCIL_INDEX8
	val DEPTH_STENCIL = 0x84F9
	val RENDERBUFFER_WIDTH = GLES20.GL_RENDERBUFFER_WIDTH
	val RENDERBUFFER_HEIGHT = GLES20.GL_RENDERBUFFER_HEIGHT
	val RENDERBUFFER_INTERNAL_FORMAT = GLES20.GL_RENDERBUFFER_INTERNAL_FORMAT
	val RENDERBUFFER_RED_SIZE = GLES20.GL_RENDERBUFFER_RED_SIZE
	val RENDERBUFFER_GREEN_SIZE = GLES20.GL_RENDERBUFFER_GREEN_SIZE
	val RENDERBUFFER_BLUE_SIZE = GLES20.GL_RENDERBUFFER_BLUE_SIZE
	val RENDERBUFFER_ALPHA_SIZE = GLES20.GL_RENDERBUFFER_ALPHA_SIZE
	val RENDERBUFFER_DEPTH_SIZE = GLES20.GL_RENDERBUFFER_DEPTH_SIZE
	val RENDERBUFFER_STENCIL_SIZE = GLES20.GL_RENDERBUFFER_STENCIL_SIZE
	val FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE = GLES20.GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE
	val FRAMEBUFFER_ATTACHMENT_OBJECT_NAME = GLES20.GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME
	val FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL = GLES20.GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL
	val FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE =
		GLES20.GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE
	val COLOR_ATTACHMENT0 = GLES20.GL_COLOR_ATTACHMENT0
	val DEPTH_ATTACHMENT = GLES20.GL_DEPTH_ATTACHMENT
	val STENCIL_ATTACHMENT = GLES20.GL_STENCIL_ATTACHMENT
	val DEPTH_STENCIL_ATTACHMENT = 0x821A
	val NONE = GLES20.GL_NONE
	val FRAMEBUFFER_COMPLETE = GLES20.GL_FRAMEBUFFER_COMPLETE
	val FRAMEBUFFER_INCOMPLETE_ATTACHMENT = GLES20.GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT
	val FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT =
		GLES20.GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT
	val FRAMEBUFFER_INCOMPLETE_DIMENSIONS = GLES20.GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS
	val FRAMEBUFFER_UNSUPPORTED = GLES20.GL_FRAMEBUFFER_UNSUPPORTED
	val FRAMEBUFFER_BINDING = GLES20.GL_FRAMEBUFFER_BINDING
	val RENDERBUFFER_BINDING = GLES20.GL_RENDERBUFFER_BINDING
	val MAX_RENDERBUFFER_SIZE = GLES20.GL_MAX_RENDERBUFFER_SIZE

	//public final int INVALID_FRAMEBUFFER_OPERATION = GLES20.GL_INVALID_FRAMEBUFFER_OPERATION;
	/* Framebuffers and renderbuffers */ /* Pixel storage modes */
	val UNPACK_COLORSPACE_CONVERSION_WEBGL = 0x9243
	val UNPACK_FLIP_Y_WEBGL = 0x9240
	val UNPACK_PREMULTIPLY_ALPHA_WEBGL = 0x9241 /* Pixel storage modes */

	companion object {
		const val SIZE_OF_BYTE = 1
		const val SIZE_OF_SHORT = 2
		const val SIZE_OF_INT = 4
		const val SIZE_OF_LONG = 8
		const val SIZE_OF_FLOAT = 4
		const val SIZE_OF_DOUBLE = 8
		const val SIZE_OF_CHAR = 2

		@JvmStatic
		external fun nativeReadPixels(
			x: Int,
			y: Int,
			width: Int,
			height: Int,
			format: Int,
			type: Int
		)

		@JvmStatic
		private external fun nativeVertexAttribPointer(
			index: Int,
			size: Int,
			type: Int,
			normalized: Boolean,
			stride: Int,
			offset: Int
		)

		@JvmStatic
		private external fun nativeGetVertexAttribOffset(index: Int, pname: Int, buffer: ByteBuffer?)

		@JvmStatic
		private external fun nativeTexImage2DBuffer(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			i: Int,
			format: Int,
			type: Int,
			buffer: Buffer,
			flipYWebGL: Boolean
		)


		@JvmStatic
		private external fun nativeTexImage2DBufferEncoded(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			i: Int,
			format: Int,
			type: Int,
			buffer: Buffer,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexImage2DByteArray(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			i: Int,
			format: Int,
			type: Int,
			byteArray: ByteArray,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexImage2DShortArray(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			i: Int,
			format: Int,
			type: Int,
			shortArray: ShortArray,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexImage2DIntArray(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			i: Int,
			format: Int,
			type: Int,
			intArray: IntArray,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexImage2DFloatArray(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			i: Int,
			format: Int,
			type: Int,
			floatArray: FloatArray,
			flipYWebGL: Boolean
		)

		@JvmStatic
		private external fun nativeTexImage2DAsset(
			target: Int,
			level: Int,
			internalformat: Int,
			border: Int,
			format: Int,
			type: Int,
			asset: Long,
			flipY: Boolean
		)


		@JvmStatic
		private external fun nativeTexImage2DBitmap(
			target: Int,
			level: Int,
			internalformat: Int,
			width: Int,
			height: Int,
			border: Int,
			format: Int,
			type: Int,
			bitmap: Bitmap,
			flipY: Boolean
		)

		@JvmStatic
		private external fun nativeTexSubImage2DAsset(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			format: Int,
			image_type: Int,
			asset: Long,
			flipY: Boolean
		)

		@JvmStatic
		private external fun nativeTexSubImage2DBuffer(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			width: Int,
			height: Int,
			format: Int,
			type: Int,
			buffer: Buffer,
			flipYWebGL: Boolean
		)


		@JvmStatic
		private external fun nativeTexSubImage2DByteArray(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			width: Int,
			height: Int,
			format: Int,
			type: Int,
			byteArray: ByteArray,
			flipY: Boolean
		)


		@JvmStatic
		private external fun nativeTexSubImage2DShortArray(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			width: Int,
			height: Int,
			format: Int,
			type: Int,
			shortArray: ShortArray,
			flipY: Boolean,
		)

		@JvmStatic
		private external fun nativeTexSubImage2DIntArray(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			width: Int,
			height: Int,
			format: Int,
			type: Int,
			intArray: IntArray,
			flipY: Boolean,
		)

		@JvmStatic
		private external fun nativeTexSubImage2DFloatArray(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			width: Int,
			height: Int,
			format: Int,
			type: Int,
			floatArray: FloatArray,
			flipY: Boolean,
		)

		@JvmStatic
		private external fun nativeTexSubImage2DBitmap(
			target: Int,
			level: Int,
			xoffset: Int,
			yoffset: Int,
			width: Int,
			height: Int,
			format: Int,
			type: Int,
			pixels: Bitmap,
			flipYWebGL: Boolean
		)

		@JvmStatic
		external fun nativeTexImage2DTexture(
			width: Int,
			height: Int,
			src: Int,
			dst: Int,
		)

	}
}
