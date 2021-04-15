package org.nativescript.canvasdemo

import android.opengl.GLES20
import android.os.Bundle
import android.util.Log
import androidx.appcompat.app.AppCompatActivity
import org.nativescript.canvas.TNSCanvas
import org.nativescript.canvas.TNSWebGLRenderingContext
import java.nio.ByteBuffer
import java.nio.ByteOrder

class WebGLActivity : AppCompatActivity() {
	var canvas: TNSCanvas? = null
	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		setContentView(R.layout.activity_web_g_l)
		canvas = findViewById(R.id.canvasView)
		canvas?.listener = object : TNSCanvas.Listener {
			override fun contextReady() {
				Log.d("com.test", "Is Ready")
				drawTriangle()
			}
		}
	}

	fun drawTriangle() {
		canvas?.let { canvas ->
			val gl = canvas.getContext("webgl") as? TNSWebGLRenderingContext
			canvas.queueEvent {
				val vertex = floatArrayOf(
					-1f, 1f, 0f,
					-1f, -1f, 0f,
					1f, -1f, 0f

				)


				val vertexBuf = ByteBuffer.allocate(vertex.size * 4).order(ByteOrder.nativeOrder())
				vertexBuf.asFloatBuffer().put(vertex)
				vertexBuf.rewind()
				val vs_source = """
				#version 300 es
				in vec3 aPos;
				void main(){
				gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0f);
				}
			""".trimIndent()

				val fs_source = """
				#version 300 es
				precision mediump float;
				out vec4 FragColor;
				void main(){
				FragColor = vec4(1.0f,0.0f,0.0f,1.0f);
				}
			""".trimIndent()


				val program = GLES20.glCreateProgram()
				val vs = GLES20.glCreateShader(GLES20.GL_VERTEX_SHADER)
				GLES20.glShaderSource(vs, vs_source)


				val fs = GLES20.glCreateShader(GLES20.GL_FRAGMENT_SHADER)
				GLES20.glShaderSource(fs, fs_source)

				GLES20.glCompileShader(vs)
				GLES20.glCompileShader(fs)

				GLES20.glAttachShader(program, vs)
				GLES20.glAttachShader(program, fs)

				Log.d("com.test", "vs log ${GLES20.glGetShaderInfoLog(vs)}")
				Log.d("com.test", "fs log ${GLES20.glGetShaderInfoLog(fs)}")

				GLES20.glLinkProgram(program)

				Log.d("com.test", "GL Error ${GLES20.glGetError()}")


				val attr = GLES20.glGetAttribLocation(program, "aPos")
				val vbo = intArrayOf(0)
				GLES20.glGenBuffers(1, vbo, 0)
				GLES20.glBindBuffer(GLES20.GL_ARRAY_BUFFER, vbo[0])
				GLES20.glBufferData(
					GLES20.GL_ARRAY_BUFFER,
					vertexBuf.capacity(),
					vertexBuf,
					GLES20.GL_STATIC_DRAW
				)

				GLES20.glVertexAttribPointer(attr, 3, GLES20.GL_FLOAT, false, 3 * 4, 0)
				GLES20.glEnableVertexAttribArray(0)
				GLES20.glUseProgram(program)

			//	GLES20.glDrawArrays(GLES20.GL_TRIANGLES, 0, 3)
			//	gl?.updateCanvas()

			}
			gl?.drawArrays(GLES20.GL_TRIANGLES, 0, 3)
		}
	}
}
