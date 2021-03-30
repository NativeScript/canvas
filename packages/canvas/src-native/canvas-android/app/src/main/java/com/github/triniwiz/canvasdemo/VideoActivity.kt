package com.github.triniwiz.canvasdemo

import android.graphics.SurfaceTexture
import android.media.MediaPlayer
import android.net.Uri
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import android.view.Surface
import com.github.triniwiz.canvas.TNSCanvas
import com.github.triniwiz.canvas.TNSWebGLRenderingContext
import com.github.triniwiz.canvas.TextureRender
import com.github.triniwiz.canvas.Utils
import java.lang.Exception

class VideoActivity : AppCompatActivity() {
	var canvas: TNSCanvas? = null
	var surfaceTexture: SurfaceTexture? = null
	var surface: Surface? = null
	var hasFrame = true
	var render: TextureRender? = null
	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		setContentView(R.layout.activity_video)
		player = MediaPlayer()
		player.setOnPreparedListener {
			player.start()
		}
		player.setOnVideoSizeChangedListener { mp, width, height ->
		//	surfaceTexture?.setDefaultBufferSize(width, height)
		}
		canvas = findViewById(R.id.canvasView)
		canvas?.listener = object : TNSCanvas.Listener {
			override fun contextReady() {
				Log.d("com.test", "Is Ready")
				canvas?.let {
					val ctx = it.getContext("webgl") as TNSWebGLRenderingContext
					val result = Utils.createSurfaceTexture(ctx)
					surfaceTexture = result[0] as SurfaceTexture
					render = result[1] as TextureRender
					surfaceTexture?.setOnFrameAvailableListener {
						hasFrame = true
						render()
					}
					surface = Surface(surfaceTexture)
					player.setSurface(surface)
				}
			}
		}
		initPlayer()
	}

	fun render(){
		canvas?.let {
			val ctx = it.getContext("webgl") as TNSWebGLRenderingContext
			Utils.updateTexImage(ctx, surfaceTexture!!, render!!)
		}
	}

	override fun onDestroy() {
		super.onDestroy()
		surfaceTexture?.release()
		surface?.release()
		surfaceTexture = null
		surface = null
	}

	lateinit var player: MediaPlayer
	fun initPlayer(){
		try {
			player.setDataSource("https://github.com/mdn/webgl-examples/blob/gh-pages/tutorial/sample8/Firefox.mp4?raw=true")
			player.prepareAsync()
		}catch (e: Exception){
			e.printStackTrace()
		}
	}
}
