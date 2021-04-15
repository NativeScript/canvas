package org.nativescript.canvas

import android.graphics.SurfaceTexture
import android.opengl.GLES11Ext
import android.opengl.GLES20
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.nio.FloatBuffer

class TextureRender {
	private var mProgram = 0
	var rbo = -1
	var fbo = -1
	var textureId = -1
	var width: Int = -1
	var height: Int = -1
	var ab = -1
	var pos = -1
	var matrixPos = -1
	var samplerPos = -1
	var matrix = FloatArray(16)


	fun drawFrame(
		st: SurfaceTexture,
		width: Int,
		height: Int,
		internalFormat: Int,
		format: Int,
		flipYWebGL: Boolean
	) {
		nativeDrawFrame(
			st,
			flipYWebGL,
			fbo,
			rbo,
			mProgram,
			textureId,
			samplerPos,
			ab,
			pos,
			matrix,
			matrixPos,
			width,
			height,
			this.width,
			this.height,
			internalFormat,
			format,
			4
		)
		this.width = width
		this.height = height
	}

	fun surfaceCreated() {
		mProgram = GLES20.glCreateProgram()
		val vs = GLES20.glCreateShader(GLES20.GL_VERTEX_SHADER)
		GLES20.glShaderSource(vs, VERTEX_SHADER)

		val fs = GLES20.glCreateShader(GLES20.GL_FRAGMENT_SHADER)
		GLES20.glShaderSource(fs, FRAGMENT_SHADER)

		GLES20.glCompileShader(vs)
		GLES20.glCompileShader(fs)

		GLES20.glAttachShader(mProgram, vs)
		GLES20.glAttachShader(mProgram, fs)

		GLES20.glLinkProgram(mProgram)

		val buffers = IntArray(1)
		GLES20.glGenBuffers(1, buffers, 0)
		ab = buffers[0]

		val rbos = IntArray(1)
		GLES20.glGenRenderbuffers(1, rbos, 0)
		rbo = rbos[0]

		val fbos = IntArray(1)
		GLES20.glGenFramebuffers(1, fbos, 0)
		fbo = fbos[0]

		val textures = IntArray(1)
		GLES20.glGenTextures(1, textures, 0)
		textureId = textures[0]

		GLES20.glBindBuffer(GLES20.GL_ARRAY_BUFFER, ab)
		GLES20.glBufferData(
			GLES20.GL_ARRAY_BUFFER,
			vextexBuf.capacity() * SIZE_OF_FLOAT,
			vextexBuf,
			GLES20.GL_STATIC_DRAW
		)

		samplerPos = GLES20.glGetUniformLocation(mProgram, "uSampler")
		matrixPos = GLES20.glGetUniformLocation(mProgram, "uTextureMatrix")
		pos = GLES20.glGetAttribLocation(mProgram, "aPosition")

		GLES20.glVertexAttribPointer(pos, 2, GLES20.GL_FLOAT, false, 2 * SIZE_OF_FLOAT, 0)
		GLES20.glEnableVertexAttribArray(pos)

		val previousTexture = IntArray(1)
		GLES20.glGetIntegerv(
			GLES20.GL_TEXTURE_BINDING_2D,
			previousTexture, 0
		)

		GLES20.glBindTexture(GLES11Ext.GL_TEXTURE_EXTERNAL_OES, textureId)

		GLES20.glTexParameteri(
			GLES11Ext.GL_TEXTURE_EXTERNAL_OES, GLES20.GL_TEXTURE_MIN_FILTER,
			GLES20.GL_LINEAR
		)
		GLES20.glTexParameteri(
			GLES11Ext.GL_TEXTURE_EXTERNAL_OES, GLES20.GL_TEXTURE_MAG_FILTER,
			GLES20.GL_LINEAR
		)
		GLES20.glTexParameteri(
			GLES11Ext.GL_TEXTURE_EXTERNAL_OES, GLES20.GL_TEXTURE_WRAP_S,
			GLES20.GL_CLAMP_TO_EDGE
		)
		GLES20.glTexParameteri(
			GLES11Ext.GL_TEXTURE_EXTERNAL_OES, GLES20.GL_TEXTURE_WRAP_T,
			GLES20.GL_CLAMP_TO_EDGE
		)

		GLES20.glBindTexture(GLES11Ext.GL_TEXTURE_EXTERNAL_OES, 0)

		GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, previousTexture[0])


	}

	companion object {
		@JvmStatic
		private external fun nativeDrawFrame(
			surfaceTexture: SurfaceTexture,
			flipYWebGL: Boolean,
			fbo: Int,
			rbo: Int,
			program: Int,
			externalTexture: Int,
			samplerPos: Int,
			arrayBuffer: Int,
			pos: Int,
			matrix: FloatArray,
			matrixPos: Int,
			width: Int,
			height: Int,
			renderWidth: Int,
			renderHeight: Int,
			internalFormat: Int,
			format: Int,
			drawCount: Int,
		)

		/*
		* 		val vextexCoords = floatArrayOf(
			0f, 1f,
			1f, 1f,
			0f, 0f,

			1f, 1f,
			1f, 0f,
			0f, 0f
		)
		* */




		val vextexCoords = floatArrayOf(
			0f, 0f,
			1f, 0f,
			0f, 1f,
			1f, 1f,
		)


		var vextexBuf: FloatBuffer
		private const val TAG = "TextureRender"
		private const val SIZE_OF_FLOAT = 4
		private const val VERTEX_SHADER = """
precision highp float;
attribute vec4 aPosition;
uniform mat4 uTextureMatrix;
varying vec2 TexCoord;
void main(){
vec2 clipSpace = (1.0 - 2.0 * aPosition.xy);
TexCoord = (uTextureMatrix * aPosition).xy;
gl_Position = vec4(clipSpace, 0.0, 1.0);
}
		"""
		private const val FRAGMENT_SHADER = """
			#extension GL_OES_EGL_image_external : require
			precision highp float;
			varying vec2 TexCoord;
uniform samplerExternalOES uSampler;
void main(){
gl_FragColor = texture2D(uSampler, TexCoord);
}
		"""


		init {
			val vb =
				ByteBuffer.allocateDirect(vextexCoords.size * SIZE_OF_FLOAT).order(ByteOrder.nativeOrder())
			vextexBuf = vb.asFloatBuffer().put(vextexCoords)
			vextexBuf.position(0)
		}
	}
}
