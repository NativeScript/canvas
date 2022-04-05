package org.nativescript.canvasdemo

import android.graphics.SurfaceTexture
import android.media.MediaPlayer
import android.opengl.GLES20
import android.os.Bundle
import android.util.Log
import android.view.Surface
import android.view.View
import android.view.ViewGroup
import androidx.appcompat.app.AppCompatActivity
import com.google.android.exoplayer2.SimpleExoPlayer
import org.nativescript.canvas.*
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.nio.FloatBuffer
import java.nio.IntBuffer

class VideoActivity : AppCompatActivity() {
	var canvas: TNSCanvas? = null
	var surfaceTexture: SurfaceTexture? = null
	var surface: Surface? = null
	var hasFrame = true
	var render: TextureRender? = null
	var width: Int = -1
	var height: Int = -1
	var exoPlayer: SimpleExoPlayer? = null
	var didInit = false
	val SIZE_OF_FLOAT = 4
	val SIZE_OF_INT = 4

//	val vextexCoords = floatArrayOf(
//		-1f, 1f, 0f,
//		-1f, -1f, 0f,
//		1f, -1f, 0f
//	)


	val vextexCoords = floatArrayOf(
		// position    // texture
		-1f, 1f, 0f, 0f, 1f,
		1f, 1f, 0f, 1f, 1f,
		1f, -1f, 0f, 1f, 0f,
		-1f, -1f, 0f, 0f, 0f
	)


	val vextexCoordsInner = floatArrayOf(
		// position    // texture
		-.5f, .5f, 0f, 0f, 1f,
		.5f, .5f, 0f, 1f, 1f,
		.5f, -.5f, 0f, 1f, 0f,
		-.5f, -.5f, 0f, 0f, 0f
	)


	var indexCoords = intArrayOf(
		0, 1, 2,
		2, 3, 0
	)

	var indices: IntBuffer
	var vextexBuf: FloatBuffer
	var vextexBufInner: FloatBuffer


	var vs = """
attribute vec3 aPos;
attribute vec2 aTexCoord;
varying highp vec2 TexCoord;
void main(){
gl_Position = vec4(aPos, 1.0);
TexCoord = aTexCoord;
}
	""".trimIndent()

	var fs = """
varying highp vec2 TexCoord;
uniform sampler2D uSampler;
void main(){
gl_FragColor = texture2D(uSampler, TexCoord);
}
	""".trimIndent()


	init {
		val index =
			ByteBuffer.allocateDirect(indexCoords.size * SIZE_OF_INT).order(ByteOrder.nativeOrder())
		indices = index.asIntBuffer().put(indexCoords)
		indices.position(0)
		val vb =
			ByteBuffer.allocateDirect(vextexCoords.size * SIZE_OF_FLOAT).order(ByteOrder.nativeOrder())
		vextexBuf = vb.asFloatBuffer().put(vextexCoords)
		vextexBuf.position(0)

		val vbi =
			ByteBuffer.allocateDirect(vextexCoordsInner.size * SIZE_OF_FLOAT)
				.order(ByteOrder.nativeOrder())
		vextexBufInner = vbi.asFloatBuffer().put(vextexCoordsInner)
		vextexBufInner.position(0)
	}


	var ab = -1
	var ebo = -1
	var texture = -1
	var program = -1
	var pos = -1
	var texPos = -1


	var ab2 = -1
	var ebo2 = -1
	var texture2 = -1
	var program2 = -1
	var pos2 = -1
	var texPos2 = -1


	fun createProgram(gl: TNSWebGLRenderingContext, vs: String, fs: String): Int {
		val program = gl.createProgram()

		val vs1 = gl.createShader(GLES20.GL_VERTEX_SHADER)
		gl.shaderSource(vs1, vs)
		gl.compileShader(vs1)

		val compiled1 = gl.getShaderParameter(vs1, GLES20.GL_COMPILE_STATUS)

		if (compiled1 == 0) {
			Log.e("com.test", "Could not compile shader GL_VERTEX_SHADER:")
			Log.e("com.test", " " + gl.getShaderInfoLog(vs1))
			gl.deleteShader(vs1)
		}


		val fs1 = gl.createShader(GLES20.GL_FRAGMENT_SHADER)
		gl.shaderSource(fs1, fs)
		gl.compileShader(fs1)


		val compiled2 = gl.getShaderParameter(fs1, GLES20.GL_COMPILE_STATUS,)
		if (compiled2 == 0) {
			Log.e("com.test", "Could not compile shader GL_FRAGMENT_SHADER:")
			Log.e("com.test", " " + gl.getShaderInfoLog(fs1))
			gl.deleteShader(fs1)

		}

		gl.attachShader(program, vs1)
		gl.attachShader(program, fs1)
		gl.linkProgram(program)
		return program
	}

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		setContentView(R.layout.activity_video)
		TNSCanvas.enableDebug = true
		player = MediaPlayer()
		player.setOnPreparedListener {
			this.width = it.videoWidth
			this.height = it.videoHeight
			player.start()
			setup()
		}
		canvas = findViewById(R.id.canvasView)

		canvas?.listener = object : TNSCanvas.Listener {
			override fun contextReady() {
				Log.d("com.test", "Is Ready")
				val ctx = canvas?.getContext("webgl") as? TNSWebGLRenderingContext
				ctx?.let { gl ->
					gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true)

					program = createProgram(gl, vs, fs)
					program2 = createProgram(gl, vs, fs)

					ab = gl.createBuffer()
					ab2 = gl.createBuffer()

					texture = gl.createTexture()
					texture2 = gl.createTexture()

					ebo = gl.createBuffer()
					ebo2 = gl.createBuffer()


					gl.bindBuffer(GLES20.GL_ARRAY_BUFFER, ab)
					gl.bufferData(
						GLES20.GL_ARRAY_BUFFER,
						vextexBuf,
						GLES20.GL_STATIC_DRAW
					)

					pos = gl.getAttribLocation(program, "aPos")
					gl.vertexAttribPointer(pos, 3, GLES20.GL_FLOAT, false, 5 * SIZE_OF_FLOAT, 0)
					GLES20.glEnableVertexAttribArray(pos)


					texPos = gl.getAttribLocation(program, "aTexCoord")
					gl.vertexAttribPointer(
						texPos,
						2,
						GLES20.GL_FLOAT,
						false,
						5 * SIZE_OF_FLOAT,
						3 * SIZE_OF_FLOAT
					)

					gl.enableVertexAttribArray(texPos)

					gl.bindBuffer(GLES20.GL_ELEMENT_ARRAY_BUFFER, ebo)
					gl.bufferData(
						GLES20.GL_ELEMENT_ARRAY_BUFFER,
						indices,
						GLES20.GL_STATIC_DRAW
					)



					gl.bindTexture(GLES20.GL_TEXTURE_2D, texture)


					val blue = byteArrayOf(0, 0, 255.toByte(), 255.toByte())
					val buf = ByteBuffer.allocateDirect(4).order(ByteOrder.nativeOrder())
					buf.put(blue)
					buf.position(0)
					gl.texImage2D(
						GLES20.GL_TEXTURE_2D,
						0,
						GLES20.GL_RGBA,
						1,
						1,
						0,
						GLES20.GL_RGBA,
						GLES20.GL_UNSIGNED_BYTE,
						buf
					)

					gl.texParameteri(
						GLES20.GL_TEXTURE_2D,
						GLES20.GL_TEXTURE_MIN_FILTER,
						GLES20.GL_LINEAR
					)
					gl.texParameteri(
						GLES20.GL_TEXTURE_2D,
						GLES20.GL_TEXTURE_MAG_FILTER,
						GLES20.GL_LINEAR
					)
					gl.texParameteri(
						GLES20.GL_TEXTURE_2D,
						GLES20.GL_TEXTURE_WRAP_S,
						GLES20.GL_CLAMP_TO_EDGE
					)
					gl.texParameteri(
						GLES20.GL_TEXTURE_2D,
						GLES20.GL_TEXTURE_WRAP_T,
						GLES20.GL_CLAMP_TO_EDGE
					)

					gl.bindTexture(GLES20.GL_TEXTURE_2D, 0)


					/*
					GLES20.glBindBuffer(GLES20.GL_ARRAY_BUFFER, ab2)
					GLES20.glBufferData(
						GLES20.GL_ARRAY_BUFFER,
						vextexBufInner.capacity() * SIZE_OF_FLOAT,
						vextexBufInner,
						GLES20.GL_STATIC_DRAW
					)

					pos2 = GLES20.glGetAttribLocation(program2, "aPos")
					GLES20.glVertexAttribPointer(pos2, 3, GLES20.GL_FLOAT, false, 5 * SIZE_OF_FLOAT, 0)
					GLES20.glEnableVertexAttribArray(pos2)


					texPos2 = GLES20.glGetAttribLocation(program2, "aTexCoord")
					GLES20.glVertexAttribPointer(
						texPos2,
						2,
						GLES20.GL_FLOAT,
						false,
						5 * SIZE_OF_FLOAT,
						3 * SIZE_OF_FLOAT
					)

					GLES20.glEnableVertexAttribArray(texPos2)



					GLES20.glBindBuffer(GLES20.GL_ELEMENT_ARRAY_BUFFER, ebo2)
					GLES20.glBufferData(
						GLES20.GL_ELEMENT_ARRAY_BUFFER,
						indices.capacity() * SIZE_OF_INT,
						indices,
						GLES20.GL_STATIC_DRAW
					)





					GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, texture2)

					val red = byteArrayOf(255.toByte(), 0,0, 255.toByte())
					val redBuf = ByteBuffer.allocateDirect(4).order(ByteOrder.nativeOrder())
					redBuf.put(red)
					redBuf.position(0)
					GLES20.glTexImage2D(
						GLES20.GL_TEXTURE_2D,
						0,
						GLES20.GL_RGBA,
						1,
						1,
						0,
						GLES20.GL_RGBA,
						GLES20.GL_UNSIGNED_BYTE,
						redBuf
					)

					GLES20.glTexParameteri(
						GLES20.GL_TEXTURE_2D,
						GLES20.GL_TEXTURE_MIN_FILTER,
						GLES20.GL_LINEAR
					)
					GLES20.glTexParameteri(
						GLES20.GL_TEXTURE_2D,
						GLES20.GL_TEXTURE_MAG_FILTER,
						GLES20.GL_LINEAR
					)
					GLES20.glTexParameteri(
						GLES20.GL_TEXTURE_2D,
						GLES20.GL_TEXTURE_WRAP_S,
						GLES20.GL_CLAMP_TO_EDGE
					)
					GLES20.glTexParameteri(
						GLES20.GL_TEXTURE_2D,
						GLES20.GL_TEXTURE_WRAP_T,
						GLES20.GL_CLAMP_TO_EDGE
					)

					GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, 0)

					 */

					// draw bg


					gl.bindTexture(GLES20.GL_TEXTURE_2D, texture)

					gl.useProgram(program)

					gl.bindBuffer(GLES20.GL_ARRAY_BUFFER, ab)

					gl.vertexAttribPointer(pos, 3, GLES20.GL_FLOAT, false, 5 * SIZE_OF_FLOAT, 0)

					gl.enableVertexAttribArray(pos)


					gl.bindBuffer(GLES20.GL_ELEMENT_ARRAY_BUFFER, ebo)

					texPos = gl.getAttribLocation(program, "aTexCoord")
					gl.vertexAttribPointer(
						texPos,
						2,
						GLES20.GL_FLOAT,
						false,
						5 * SIZE_OF_FLOAT,
						3 * SIZE_OF_FLOAT
					)

					gl.enableVertexAttribArray(texPos)

					//GLES20.glUniform1i(GLES20.glGetUniformLocation(program, "uSampler"), 0)


					gl.drawElements(GLES20.GL_TRIANGLES, 6, GLES20.GL_UNSIGNED_INT, 0)


					/*
					GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, texture2)

					GLES20.glUseProgram(program2)

					GLES20.glBindBuffer(GLES20.GL_ARRAY_BUFFER, ab2)

					GLES20.glVertexAttribPointer(pos2, 3, GLES20.GL_FLOAT, false, 5 * SIZE_OF_FLOAT, 0)
					GLES20.glEnableVertexAttribArray(pos2)


					GLES20.glBindBuffer(GLES20.GL_ELEMENT_ARRAY_BUFFER, ebo2)

					texPos2 = GLES20.glGetAttribLocation(program2, "aTexCoord")
					GLES20.glVertexAttribPointer(
						texPos2,
						2,
						GLES20.GL_FLOAT,
						false,
						5 * SIZE_OF_FLOAT,
						3 * SIZE_OF_FLOAT
					)

					GLES20.glEnableVertexAttribArray(texPos2)


					//GLES20.glUniform1i(GLES20.glGetUniformLocation(program, "uSampler"), 0)


					GLES20.glDrawElements(GLES20.GL_TRIANGLES, 6, GLES20.GL_UNSIGNED_INT, 0)


					 */

					gl.updateCanvas()

					//	ctx?.drawElements(GLES20.GL_TRIANGLES, 6, GLES20.GL_UNSIGNED_INT, 0)
					//	ctx?.drawArrays(GLES20.GL_TRIANGLES, 0, 3)
				}
				//	setup()
			}
		}
		initPlayer()
	}


	fun setup() {
		if (width == -1 && height == -1 || didInit) {
			return
		}
		val ctx = canvas?.getContext("webgl") as? TNSWebGLRenderingContext
		ctx?.let {
			val result = Utils.createSurfaceTexture(it)
			surfaceTexture = result[0] as? SurfaceTexture
			render = result[1] as? TextureRender
			//	surfaceTexture?.setDefaultBufferSize(width, height)
			surfaceTexture?.setOnFrameAvailableListener {
				hasFrame = true
				renderFrame()
			}

			surface = Surface(surfaceTexture)
			player.setSurface(surface)
			didInit = true
		}
	}

	fun renderFrame() {
		val gl = canvas!!.getContext("webgl") as TNSWebGLRenderingContext
		//GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, texture)
		Utils.updateTexImage(
			gl,
			surfaceTexture!!,
			render!!,
			width,
			height,
			GLES20.GL_RGBA,
			GLES20.GL_RGBA
		)

		gl.useProgram(program)
		gl.bindBuffer(GLES20.GL_ARRAY_BUFFER, ab)

		gl.vertexAttribPointer(pos, 3, GLES20.GL_FLOAT, false, 5 * SIZE_OF_FLOAT, 0)
		gl.enableVertexAttribArray(pos)


		gl.bindBuffer(GLES20.GL_ELEMENT_ARRAY_BUFFER, ebo)

		gl.vertexAttribPointer(
			texPos,
			2,
			GLES20.GL_FLOAT,
			false,
			5 * SIZE_OF_FLOAT,
			3 * SIZE_OF_FLOAT
		)

		gl.enableVertexAttribArray(texPos)

		gl.drawElements(GLES20.GL_TRIANGLES, 6, GLES20.GL_UNSIGNED_INT, 0)


		/*

		GLES20.glUseProgram(program2)
		GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, texture2)
		GLES20.glBindBuffer(GLES20.GL_ARRAY_BUFFER, ab2)

		GLES20.glVertexAttribPointer(pos2, 3, GLES20.GL_FLOAT, false, 5 * SIZE_OF_FLOAT, 0)
		GLES20.glEnableVertexAttribArray(pos2)



		GLES20.glBindBuffer(GLES20.GL_ELEMENT_ARRAY_BUFFER, ebo2)

		GLES20.glVertexAttribPointer(
			texPos2,
			2,
			GLES20.GL_FLOAT,
			false,
			5 * SIZE_OF_FLOAT,
			3 * SIZE_OF_FLOAT
		)

		GLES20.glEnableVertexAttribArray(texPos2)

		GLES20.glDrawElements(GLES20.GL_TRIANGLES, 6, GLES20.GL_UNSIGNED_INT, 0)
		*/



		gl.updateCanvas()
		//	render?.drawFrame(surfaceTexture!!)

	}

	override fun onDestroy() {
		super.onDestroy()
		player.stop()
		player.release()
		surfaceTexture?.release()
		surface?.release()
		surfaceTexture = null
		surface = null
	}

	lateinit var player: MediaPlayer
	fun initPlayer() {
		try {
			// https://github.com/mdn/webgl-examples/raw/gh-pages/tutorial/sample8/Firefox.mp4
			player.setDataSource("https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4")
			player.prepareAsync()
		} catch (e: Exception) {
			e.printStackTrace()
		}
	}
}
