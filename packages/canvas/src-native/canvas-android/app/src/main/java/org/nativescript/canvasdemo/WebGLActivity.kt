package org.nativescript.canvasdemo

import android.os.Bundle
import android.util.Log
import android.view.ViewGroup
import androidx.appcompat.app.AppCompatActivity
import org.nativescript.canvas.*
import java.util.concurrent.Executors

class WebGLActivity : AppCompatActivity() {
	var canvas: NSCCanvas? = null
	var executor = Executors.newSingleThreadExecutor()
	var contentView: ViewGroup? = null
	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		setContentView(R.layout.activity_web_g_l)
		contentView = findViewById(android.R.id.content)
		canvas = findViewById(R.id.canvasView)
		canvas?.listener = object : NSCCanvas.Listener {
			override fun contextReady() {
				Log.d("com.test", "Is Ready")
				//drawTriangle()
			//	drawImage()
			}

			override fun surfaceResize(width: Int, height: Int) {
				TODO("Not yet implemented")
			}
		}
	}

	/*
	fun drawTriangle() {
		canvas?.let { canvas ->
			val gl = canvas.getContext("webgl") as? TNSWebGLRenderingContext
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

			gl?.drawArrays(GLES20.GL_TRIANGLES, 0, 3)
		}
	}


	fun drawImage() {
		val tmpCanvas = TNSCanvas(this)
		executor.execute {
			val file = File(filesDir, "tmpImage.jpg")
			if (file.exists()) {
				file.delete()
			}

			val url =
				URL("https://webglfundamentals.org/webgl/resources/leaves.jpg")
			val fs = FileOutputStream(file)
			url.openStream().use { input ->
				fs.use { output ->
					input.copyTo(output)
					input.close()
				}
				fs.close()
			}

			val view = ImageView(this)


			val asset = TNSImageAsset()
			asset.loadImageFromPath(file.absolutePath)


			runOnUiThread {
				TNSCanvas.layoutView(contentView!!.width, contentView!!.height, tmpCanvas)
			}
			val ctx = tmpCanvas.getContext("2d") as TNSCanvasRenderingContext2D
			ctx.drawImage(asset, 300f, 300f)


			val glCanvas = TNSCanvas(this)

			runOnUiThread {
				TNSCanvas.layoutView(asset.width, asset.height, glCanvas)
			}

			val gl = glCanvas.getContext("webgl") as TNSWebGLRenderingContext

			gl.clearColor(1f, 0f, 0f, 1f)
			gl.clear(gl.COLOR_BUFFER_BIT)

			ctx.drawImage(glCanvas, 0f, 0f)

//			runOnUiThread {
//			//	TNSCanvas.layoutView(asset.width, asset.height, tmpCanvas)
//				findViewById<ViewGroup>(android.R.id.content).addView(
//					tmpCanvas
//				)
//			}

			render(tmpCanvas)


//			val bm = Bitmap.createBitmap(asset.width, asset.height, Bitmap.Config.ARGB_8888)
//			asset.copyToBitmap(bm)
//
//			view.setImageBitmap(bm)
//			runOnUiThread {
//				findViewById<ViewGroup>(android.R.id.content).addView(
//					view, ViewGroup.LayoutParams(
//						ViewGroup.LayoutParams.MATCH_PARENT,
//						ViewGroup.LayoutParams.MATCH_PARENT
//					)
//				)
//			}
		}

	}

	fun render(image: TNSCanvas) {
		// Get A WebGL context
		/** @type {HTMLCanvasElement} */

		/** @type {HTMLCanvasElement} */

		val gl = canvas?.getContext("webgl") as? TNSWebGLRenderingContext ?: return


		val fragCode = """
			precision mediump float;
			uniform sampler2D u_image;
			varying vec2 v_texCoord;
			void main() {
			   gl_FragColor = texture2D(u_image, v_texCoord);
			}
		""".trimIndent()


		val vertCode = """
		attribute vec2 a_position;
		attribute vec2 a_texCoord;
		uniform vec2 u_resolution;
		varying vec2 v_texCoord;
		void main() {
		vec2 zeroToOne = a_position / u_resolution;
		vec2 zeroToTwo = zeroToOne * 2.0;
		vec2 clipSpace = zeroToTwo - 1.0;
		gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
		v_texCoord = a_texCoord;
		}
		""".trimIndent()


		val vertShader = gl.createShader(gl.VERTEX_SHADER)
		gl.shaderSource(vertShader, vertCode)
		gl.compileShader(vertShader)


		val fragShader = gl.createShader(gl.FRAGMENT_SHADER)
		gl.shaderSource(fragShader, fragCode)
		gl.compileShader(fragShader)


		// setup GLSL program

		val program = gl.createProgram()
		gl.attachShader(program, vertShader)
		gl.attachShader(program, fragShader)


		gl.linkProgram(program)


		// look up where the vertex data needs to go.
		val positionLocation = gl.getAttribLocation(program, "a_position");
		val texcoordLocation = gl.getAttribLocation(program, "a_texCoord");

		// Create a buffer to put three 2d clip space points in
		val positionBuffer = gl.createBuffer();

		// Bind it to ARRAY_BUFFER (think of it as ARRAY_BUFFER = positionBuffer)
		gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
		// Set a rectangle the same size as the image.
		setRectangle(gl, 0f, 0f, image.width.toFloat(), image.height.toFloat());

		// provide texture coordinates for the rectangle.
		val texcoordBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, texcoordBuffer);


		val dataBuffer = floatArrayOf(
			0.0f, 0.0f,
			1.0f, 0.0f,
			0.0f, 1.0f,
			0.0f, 1.0f,
			1.0f, 0.0f,
			1.0f, 1.0f,
		)

		gl.bufferData(gl.ARRAY_BUFFER, dataBuffer, gl.STATIC_DRAW);

		// Create a texture.
		val texture = gl.createTexture();
		gl.bindTexture(gl.TEXTURE_2D, texture);

		// Set the parameters so we can render any size image.
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);

		// Upload the image into the texture.
		gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, image);

		// lookup uniforms
		val resolutionLocation = gl.getUniformLocation(program, "u_resolution");

		//webglUtils.resizeCanvasToDisplaySize(gl.canvas);

		// Tell WebGL how to convert from clip space to pixels
		gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);

		// Clear the canvas
		gl.clearColor(0f, 0f, 0f, 0f);
		gl.clear(gl.COLOR_BUFFER_BIT);

		// Tell it to use our program (pair of shaders)
		gl.useProgram(program);

		// Turn on the position attribute
		gl.enableVertexAttribArray(positionLocation);

		// Bind the position buffer.
		gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

		// Tell the position attribute how to get data out of positionBuffer (ARRAY_BUFFER)
		var size = 2;          // 2 components per iteration
		var type = gl.FLOAT;   // the data is 32bit floats
		var normalize = false; // don't normalize the data
		var stride =
			0;        // 0 = move forward size * sizeof(type) each iteration to get the next position
		var offset = 0;        // start at the beginning of the buffer
		gl.vertexAttribPointer(
			positionLocation, size, type, normalize, stride, offset
		);

		// Turn on the texcoord attribute
		gl.enableVertexAttribArray(texcoordLocation);

		// bind the texcoord buffer.
		gl.bindBuffer(gl.ARRAY_BUFFER, texcoordBuffer);

		// Tell the texcoord attribute how to get data out of texcoordBuffer (ARRAY_BUFFER)
		size = 2;          // 2 components per iteration
		type = gl.FLOAT;   // the data is 32bit floats
		normalize = false; // don't normalize the data
		stride =
			0;        // 0 = move forward size * sizeof(type) each iteration to get the next position
		offset = 0;        // start at the beginning of the buffer
		gl.vertexAttribPointer(
			texcoordLocation, size, type, normalize, stride, offset
		);

		// set the resolution
		gl.uniform2f(resolutionLocation, gl.canvas.width.toFloat(), gl.canvas.height.toFloat());

		// Draw the rectangle.
		val primitiveType = gl.TRIANGLES;
		offset = 0;
		val count = 6;
		gl.drawArrays(primitiveType, offset, count);
	}

	fun setRectangle(gl: TNSWebGLRenderingContext, x: Float, y: Float, width: Float, height: Float) {
		var x1 = x;
		var x2 = x + width;
		var y1 = y;
		var y2 = y + height;

		val buffer = floatArrayOf(
			x1, y1,
			x2, y1,
			x1, y2,
			x1, y2,
			x2, y1,
			x2, y2,
		)
		gl.bufferData(gl.ARRAY_BUFFER, buffer, gl.STATIC_DRAW);
	}


	*/
}
