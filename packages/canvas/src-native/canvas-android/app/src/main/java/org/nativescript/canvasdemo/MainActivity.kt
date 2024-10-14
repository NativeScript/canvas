package org.nativescript.canvasdemo

import android.content.Intent
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.graphics.Canvas
import android.graphics.Color
import android.opengl.GLES20
import android.os.Bundle
import android.os.Environment
import android.os.Handler
import android.os.Looper
import android.os.StrictMode
import android.util.Log
import android.view.MotionEvent
import android.view.View
import android.view.ViewGroup
import android.widget.FrameLayout
import android.widget.ImageView
import androidx.appcompat.app.AppCompatActivity
import org.json.JSONArray
import org.json.JSONObject

import org.nativescript.canvas.*
import java.io.*
import java.net.URL
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.util.concurrent.Executors
import kotlin.Exception


class MainActivity : AppCompatActivity() {
	var canvas: NSCCanvas? = null
	var svg: NSCSVG? = null
	val executor = Executors.newSingleThreadExecutor()
	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		setContentView(R.layout.activity_main)
		canvas = findViewById(R.id.canvasView)
		NSCCanvas.forceGL = false
		//	svg = findViewById(R.id.svgView)
		//  svg?.ignorePixelScaling = false
//		findViewById<androidx.constraintlayout.widget.ConstraintLayout>(R.id.parent)
//			.addView(canvas)

//		System.loadLibrary("canvasnative")
		//  canvas?.ignorePixelScaling = false
		canvas?.touchEventListener = object : NSCCanvas.TouchEvents {
			override fun onEvent(event: String, motionEvent: MotionEvent) {
				try {
					val json = JSONObject(event)
					Log.d("CANVAS", json.toString())
				} catch (e: Exception) {
					val array = JSONArray(event)
					Log.d("CANVAS", array.toString())
				}

			}
		}

		canvas?.listener = object : NSCCanvas.Listener {
			override fun contextReady() {
				Log.d("com.test", "Is Ready")
						canvas?.let { canvas ->

					val context = canvas.create2DContext(
						true,
						true,
						true,
						false,
						0,
						true,
						false,
						false,
						false,
						false
					)
						canvas.surfaceWidth = 600
							canvas.surfaceHeight = 900
//
//					NSCCanvas.context2DImageTest(context)
					NSCCanvas.context2DPathTest(context)
				//			NSCCanvas.context2DTest(context)
//
//                    Log.d("com.test", "windows $context")


					//canvas?.initContext("2d")
//                    params.width = 300
//
//                    canvas.layoutParams = params
//                    canvas.requestLayout()


							/*

					executor.execute {
						try {
							//	val docs = getExternalFilesDir(Environment.DIRECTORY_DOCUMENTS)
							val file = File(filesDir, "canvas_createpattern.jpeg")
							if (file.exists()) {
								file.delete()
							}

							val url =
								URL("https://picsum.photos/seed/picsum/600/600")
							val fs = FileOutputStream(file)
							url.openStream().use { input ->
								fs.use { output ->
									input.copyTo(output)
								}
							}
							val bm = BitmapFactory.decodeFile(file.absolutePath)

							val asset = NSCImageAsset.createImageAsset()
							//val done = NSCImageAsset.loadImageFromBitmap(asset, bm)
							val done = NSCImageAsset.loadFromPath(asset, file.absolutePath)
							val dim = NSCImageAsset.getDimensions(asset)
							var error = ""
							if (!done) {
								error = NSCImageAsset.getError(asset)
							}
							runOnUiThread {
								NSCCanvasRenderingContext2D.drawImage(context, asset, 0F, 0F)
								NSCCanvas.context2DRender(context)

								//	NSCCanvas.context2DPathTest(context)
							}
						} catch (e: IOException) {
							e.printStackTrace()
						}
					}

							*/

				}



				//	draw2D()
				//drawPatterWithCanvas(canvas!!)
				//drawText(canvas!!)


//				val view = NSCCanvas(this@MainActivity)
//				view.layoutParams =  ViewGroup.LayoutParams(512, 512)
//				NSCCanvas.layoutSurface(512, 512, view)
//
//				val ctx = view.create2DContext(
//					true,
//					true,
//					true,
//					true,
//					0,
//					true,
//					true,
//					true,
//					true,
//					true,
//					Color.BLACK
//				)
//
				val opts = BitmapFactory.Options()
				opts.inScaled = false
				val bm =	BitmapFactory.decodeResource(resources, R.drawable.di_3d, opts)
//
//				val scale = resources.displayMetrics.density
//
//				NSCCanvasRenderingContext2D.drawImage(ctx, bm, 0f,0f, 512 / scale,512 / scale)
//
//				canvas!!.initContext("webgl2", true)
//				val gl = canvas!!.nativeContext
//
//
//				NSCCanvas.WebGLContextRender(gl, ctx, GLES20.GL_ALPHA, GLES20.GL_ALPHA)

				/*

				val asset = NSCImageAsset.createImageAsset()
				canvas?.let { canvas ->

					val ctx = canvas.create2DContext(
						alpha = true,
						antialias = true,
						depth = true,
						failIfMajorPerformanceCaveat = true,
						powerPreference = 0,
						premultipliedAlpha = true,
						preserveDrawingBuffer = true,
						stencil = true,
						desynchronized = true,
						xrCompatible = true,
					)
					canvas.fit = CanvasFit.FitX
					canvas.surfaceHeight = (canvas.height / resources.displayMetrics.density).toInt()
					canvas.surfaceWidth = (canvas.width / resources.displayMetrics.density).toInt()


					executor.execute {
						try {
							//	val docs = getExternalFilesDir(Environment.DIRECTORY_DOCUMENTS)
							val file = File(filesDir, "canvas_createpattern.jpeg")
							if (file.exists()) {
								file.delete()
							}

							val url =
								URL("https://raw.githubusercontent.com/NativeScript/canvas/master/tools/demo/canvas-pixi/assets/images/star.png")
							val fs = ByteArrayOutputStream()
							url.openStream()
							url.openStream().use { input ->
								fs.use { output ->
									input.copyTo(output)
								}
							}

							val b = ByteBuffer.allocateDirect(fs.size())
							b.order(ByteOrder.nativeOrder())
							b.put(fs.toByteArray())
							b.rewind()
							val bm = NSCImageAsset.createImageAsset()
							NSCImageBitmap.createFrom(bm, b, object: NSCImageBitmap.Callback{
								override fun onComplete(done: Boolean) {
									runOnUiThread {
										NSCCanvasRenderingContext2D.drawImage(
											ctx,
											bm,
											0F,
											0F
										)
										NSCCanvas.context2DRender(ctx)
									}
								}
							})

//							val bm = BitmapFactory.decodeFile(file.absolutePath)


//							NSCImageAsset.loadFromPath(asset, file.absolutePath)

//							NSCImageAsset.loadImageFromBitmap(asset, bm)
//							runOnUiThread {
//
//								NSCCanvasRenderingContext2D.drawImage(ctx, asset, 0F, 0F, canvas.surfaceWidth.toFloat(), canvas.surfaceHeight.toFloat())
//								NSCCanvas.context2DRender(ctx)
//							}

						}catch (e: Exception){
							e.printStackTrace()
						}
					}


//					NSCImageAsset.loadImageFromUrlAsync( asset, "https://picsum.photos/seed/picsum/600/600", object :
//						NSCImageAsset.Callback {
//						override fun onComplete(done: Boolean) {
//							NSCCanvasRenderingContext2D.drawImage(ctx, asset, 0F, 0F, canvas.surfaceWidth.toFloat(), canvas.surfaceHeight.toFloat())
//							NSCCanvas.context2DRender(ctx)
//						}
//
//					})


//					NSCImageAsset.loadImageFromResourceAsync(resources, asset, R.drawable.dp, object :
//						NSCImageAsset.Callback {
//						override fun onComplete(done: Boolean) {
//							NSCCanvasRenderingContext2D.drawImage(ctx, asset, 0F, 0F, canvas.surfaceWidth.toFloat(), canvas.surfaceHeight.toFloat())
//							NSCCanvas.context2DRender(ctx)
//						}
//
//					})


//
					//	NSCCanvas.context2DImageTest(context)
				//		NSCCanvas.context2DPathTest(ctx)

				//	NSCCanvasRenderingContext2D.scale(ctx, resources.displayMetrics.density,  resources.displayMetrics.density)
//					NSCCanvasRenderingContext2D.drawImage(ctx, webp, 0F, 0F, canvas.surfaceWidth.toFloat(), canvas.surfaceHeight.toFloat())
//					NSCCanvas.context2DRender(ctx)
				}

				*/

			}

			override fun surfaceResize(width: Int, height: Int) {
				Log.d("com.test", "surfaceResize $width $height")
			}

			override fun surfaceDestroyed() {}

			override fun surfaceCreated() {}
		}

//		val view = NSCCanvas(this)
//
//		val ctx = view.create2DContext(
//			true,
//			true,
//			true,
//			true,
//			0,
//			true,
//			true,
//			true,
//			true,
//			true,
//			Color.BLACK
//		)
//
//		Log.d("com.test", "ctx $ctx ${view.drawingBufferWidth} ${view.drawingBufferHeight}")

		//NSCCanvas.context2DPathTest(ctx)



		/*
		val count = 500

		for (i in 0 until count){
				val offscreen = NSCCanvas(this)

				//offscreen.setBackgroundColor(Color.GRAY)
				NSCCanvas.layoutView(1, 1, offscreen)
				offscreen.initContext("webgl", antialias = false)

		 //   Log.d("com.test", "ctx ${offscreen.nativeGL}")
		}

		System.gc()

		*/


		/*
						val offscreen = NSCCanvas(this)
						offscreen.listener = object : NSCCanvas.Listener {
								override fun contextReady() {
										Log.d("com.test", "offscreen ready")
								}

								override fun surfaceResize(width: Int, height: Int) {
										Log.d("com.test", "offscreen surfaceResize: " + width + " : " +  height)
								}
						}

						//offscreen.setBackgroundColor(Color.GRAY)
						NSCCanvas.layoutView(500, 500, offscreen)

						val ctx = offscreen.create2DContext(
								true,
								true,
							true,
							true,
								"default",
							true,
							true,
							true,
							true,
							true,
								Color.BLACK
						)

					Log.d("com.test", "ctx $ctx")

					NSCCanvas.context2DPathTest(ctx)

					val ss = offscreen.snapshot()?.let {
						Log.d("com.test", "ctx ${it.width} ${it.height}")
						val image = ImageView(this)
						image.setImageBitmap(it)

						val root = findViewById<ViewGroup>(android.R.id.content)

						root.addView(image)
					}

					*/


		/*
						val root = findViewById<ViewGroup>(android.R.id.content)

						root.addView(offscreen)


						Log.d("com.test", "offscreen.surfaceTexture " + offscreen.textureView.surfaceTexture)

						NSCCanvas.layoutView(1000, 1000, offscreen)


					NSCCanvas.context2DTest(ctx)
					*/
		//offscreen.initContext("webgl")


//        val ctx = offscreen.create2DContext(
//            false,
//            false,
//            false,
//            false,
//            "default",
//            false,
//            false,
//            false,
//            false,
//            false,
//            Color.BLACK
//        )
		// NSCCanvas.context2DTest(ctx)
		//val root = findViewById<ViewGroup>(android.R.id.content)
		// root.addView(offscreen)
		//  NSCCanvas.layoutView(400, 400, offscreen)

		// Log.d("com.test", "Help ${offscreen.nativeContext}")

		//   Log.d("com.test", "" + offscreen.snapshot())
		//drawTransformPathSvg()
//		svg?.setSrc(
//			"""
//				<svg width="100" height="100" xmlns="svg">
//				  <defs>
//    <linearGradient id="myGradient" gradientTransform="rotate(90)">
//      <stop offset="5%"  stop-color="gold" />
//      <stop offset="95%" stop-color="red" />
//    </linearGradient>
//  </defs>
//
//  <!-- using my linear gradient -->
//  <circle cx="50" cy="50" r="30" fill="url('#myGradient')" />
//				</svg>
//			""".trimIndent()
//		)


//				svg?.setSrc(
//			"""
//				<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
//  <rect width="300" height="300" />
//				</svg>
//			""".trimIndent()
//		)


//		svg?.setSrc(
//			"""
//				<?xml version="1.0" encoding="UTF-8" standalone="no"?>
//				<svg xmlns="http://www.w3.org/2000/svg">
//  			<rect width="100" height="200" stroke="pink" />
//				</svg>
//			""".trimIndent()
//		)


//		svg?.setSrc(
//			"""
//				<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
//			 <defs>
//    <linearGradient id="PadGradient"
//                    x1="33%" x2="67%">
//      <stop offset="0%"  stop-color="fuchsia"/>
//      <stop offset="100%" stop-color="orange"/>
//    </linearGradient>
//    <linearGradient id="ReflectGradient" spreadMethod="reflect"
//                    x1="33%" x2="67%">
//      <stop offset="0%"  stop-color="fuchsia"/>
//      <stop offset="100%" stop-color="orange"/>
//    </linearGradient>
//    <linearGradient id="RepeatGradient" spreadMethod="repeat"
//                    x1="33%" x2="67%">
//      <stop offset="0%"  stop-color="fuchsia"/>
//      <stop offset="100%" stop-color="orange"/>
//    </linearGradient>
//  </defs>
//
//  <rect fill="url(#PadGradient)"
//          x="10" y="0" width="200" height="40"/>
//  <rect fill="url(#ReflectGradient)"
//          x="10" y="50" width="200" height="40"/>
//  <rect fill="url(#RepeatGradient)"
//          x="10" y="100" width="200" height="40"/>
//				</svg>
//			""".trimIndent()
//		)

		//	init()

//				svg?.setSrc(
//			"""
//				<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
//
//				  <defs>
//    <radialGradient id="myRadialGradient">
//      <stop offset="10%" stop-color="gold" />
//      <stop offset="95%" stop-color="red" />
//    </radialGradient>
//
//		<linearGradient id="myLinearGradient" gradientTransform="rotate(90)">
//      <stop offset="5%"  stop-color="gold" />
//      <stop offset="95%" stop-color="red" />
//    </linearGradient>
//  </defs>
//
//  <!-- using my radial gradient -->
//  <circle cx="50" cy="30" r="25" fill="url('#myRadialGradient')" />
//
//	 <!-- using my linear gradient -->
//  <circle cx="50" cy="70" r="25" fill="url('#myLinearGradient')" />
//
//				</svg>
//			""".trimIndent()
//		)
		/*

								svg?.setSrc(
					"""
			<svg viewBox="0 0 80 20" xmlns="http://www.w3.org/2000/svg"
				 xmlns:xlink="http://www.w3.org/1999/xlink">
			<!-- Our symbol in its own coordinate system -->
			<symbol id="myDot" width="10" height="10" viewBox="0 0 2 2">
				<circle cx="1" cy="1" r="1" />
			</symbol>

			 <!-- A grid to materialize our symbol positioning -->
			<path d="M0,10 h80 M10,0 v20 M25,0 v20 M40,0 v20 M55,0 v20 M70,0 v20" fill="none" stroke="pink" />

			<!-- All instances of our symbol -->
			<use xlink:href="#myDot" x="5"  y="5" style="opacity:1.0" />
			<use xlink:href="#myDot" x="20" y="5" style="opacity:0.8" />
			<use xlink:href="#myDot" x="35" y="5" style="opacity:0.6" />
			<use xlink:href="#myDot" x="50" y="5" style="opacity:0.4" />
			<use xlink:href="#myDot" x="65" y="5" style="opacity:0.2" />
		</svg>
					""".trimIndent()
				)
		*/

//		svg?.setSrc("""
//<svg width="200" height="100" style="border: 1px solid #cccccc;">
//    <defs>
//        <clipPath id="clipPath4">
//            <rect x="10" y="20" width="100" height="20"></rect>
//
//        </clipPath>
//    </defs>
//
//    <g style="clip-path: url(#clipPath4);">
//        <rect x="5" y="5" width="190" height="90" style="stroke: none; fill:#00ff00;"></rect>
//        <circle cx="20" cy="20" r="20" style="stroke: none; fill: #ff0000;"></circle>
//    </g>
//</svg>
//		""".trimIndent())

//		svg?.setSrc("""
//			<svg width="200" height="100" style="border: 1px solid #cccccc;">
//			<defs>
//			    <clipPath id="clipPath5">
//			        <text x="10" y="20" style="font-size: 20px; ">This is a text</text>
//			    </clipPath>
//			</defs>
//
//			<g style="clip-path: url(#clipPath5);">
//			    <rect x="5" y="5" width="190" height="90"
//
//			          style="stroke: none; fill:#00ff00;"/>
//			    <circle cx="20" cy="20" r="20" style="stroke: none; fill: #ff0000;" />
//			</g>
//			</svg>
//		""".trimIndent())

//		svg?.setSrc("""
//			<svg height="1000" width="1000">
//			  <defs>
//			    <radialGradient id="grad1" cx="50%" cy="50%" r="50%" fx="50%" fy="50%">
//			      <stop offset="0%" style="stop-color:rgb(255,255,255);
//			      stop-opacity:0" />
//			      <stop offset="100%" style="stop-color:rgb(0,0,255);stop-opacity:1" />
//			    </radialGradient>
//			  </defs>
//			  <ellipse cx="200" cy="70" rx="85" ry="55" fill="url(#grad1)" />
//			</svg>
//		""".trimIndent())


//		svg?.setSrc("""
//			<svg viewBox="-40 0 150 100" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
//			  <g fill="grey"
//			     transform="rotate(-10 50 100)
//			                translate(-36 45.5)
//			                skewX(40)
//			                scale(1 0.5)">
//			    <path id="heart" d="M 10,30 A 20,20 0,0,1 50,30 A 20,20 0,0,1 90,30 Q 90,60 50,90 Q 10,60 10,30 z" />
//			  </g>
//
//			  <use xlink:href="#heart" fill="none" stroke="red"/>
//			</svg>
//		""".trimIndent())

		//	drawTransformMatrixSvg()
		//drawTransformRotateSvg()
		//drawTransformScaleSvg()
		//drawTransformTranslateSvg()
		//	drawTransformSkewX()
		//drawTransformSkewY()

		//drawLinearGradientSvg()
		//drawLinearGradientCircleSvg()

		//drawRadialGradientCircleSvg()

		//drawTransformGradientSvg()

		//drawClipPathUnitsSvg()
		//	downloadSvg()
//		svg?.setSrc("""
//			<svg xmlns="http://www.w3.org/2000/svg">
//			  <!-- Using g to inherit presentation attributes -->
//			  <g fill="none" stroke="green" stroke-width="5">
//			    <circle stroke="green" cx="40" cy="40" r="25" />
//			    <circle stroke="green" cx="60" cy="60" r="25" />
//			  </g>
//			</svg>
//		""".trimIndent())

		//	drawTransformPathSvg()
		//drawUsePathSvg()

//		svg?.setSrc("""
//			<svg xmlns="http://www.w3.org/2000/svg">
//			  <defs>
//			    <pattern id="star" viewBox="0,0,10,10" width="10%" height="10%">
//			      <polygon points="0,0 2,5 0,10 5,8 10,10 8,5 10,0 5,2"/>
//			    </pattern>
//			  </defs>
//
//			  <circle cx="50" cy="50" r="50" fill="none" stroke-width="20" stroke="url(#star)"/>
//			</svg>
//		""".trimIndent())


//		svg?.setSrc("""
//			<svg viewBox="-10 -10 120 120">
//			  <mask id="myMask">
//			    <!-- Everything under a white pixel will be visible -->
//			    <rect x="0" y="0" width="100" height="100" fill="white" />
//
//			    <!-- Everything under a black pixel will be invisible -->
//			    <path d="M10,35 A20,20,0,0,1,50,35 A20,20,0,0,1,90,35 Q90,65,50,95 Q10,65,10,35 Z" fill="black" />
//			  </mask>
//
//			  <polygon points="-10,110 110,110 110,-10" fill="orange" />
//
//			  <!-- with this mask applied, we "punch" a heart shape hole into the circle -->
//			  <circle fill="green" cx="50" cy="50" r="50" mask="url(#myMask)" />
//			</svg>
//		""".trimIndent())

		/*	svg?.setSrc("""
						<svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%">
										<defs>
														<marker id="marker_circle" markerHeight="5" markerWidth="5" markerUnits="strokeWidth" orient="auto" refX="0" refY="0" viewBox="-6 -6 12 12">
																		<path d="M 0, 0  m -5, 0  a 5,5 0 1,0 10,0  a 5,5 0 1,0 -10,0" fill="#1f77b4"/>
														</marker>
														<marker id="marker_square" markerHeight="5" markerWidth="5" markerUnits="strokeWidth" orient="auto" refX="0" refY="0" viewBox="-5 -5 10 10">
																		<path d="M 0,0 m -5,-5 L 5,-5 L 5,5 L -5,5 Z" fill="#ff7f0e"/>
														</marker>
														<marker id="marker_arrow" markerHeight="5" markerWidth="5" markerUnits="strokeWidth" orient="auto" refX="0" refY="0" viewBox="-5 -5 10 10">
																		<path d="M 0,0 m -5,-5 L 5,0 L -5,5 Z" fill="#2ca02c"/>
														</marker>
														<marker id="marker_stub" markerHeight="5" markerWidth="5" markerUnits="strokeWidth" orient="auto" refX="0" refY="0" viewBox="-1 -5 2 10">
																		<path d="M 0,0 m -1,-5 L 1,-5 L 1,5 L -1,5 Z" fill="#d62728"/>
														</marker>
										</defs>
										<rect width="100%" height="100%" fill="green"/>
										<line fill="none" stroke="#000000" stroke-width="9" x1="25%" x2="60%" y1="40%" y2="60%" id="svg_3" marker-end="url(#marker_circle)" marker-start="url(#marker_circle)"/>
						</svg>

				""".trimIndent())
				*/
	}

	override fun onPostResume() {
		super.onPostResume()
		if (canvas?.nativeContext != 0L) {
			NSCCanvas.context2DPathTest(canvas!!.nativeContext)
		}
	}

	fun goToVideo(view: View) {
		val intent = Intent(this, VideoActivity::class.java)
		startActivity(intent)
	}


	fun goToWebGl(view: View) {
		val intent = Intent(this, WebGLActivity::class.java)
		startActivity(intent)
	}

	fun drawUsePathSvg() {
		svg?.setSrc(
			"""
			<svg viewBox="0 0 80 20" xmlns="http://www.w3.org/2000/svg"
			     xmlns:xlink="http://www.w3.org/1999/xlink">
			  <!-- Our symbol in its own coordinate system -->
			  <symbol id="myDot" width="10" height="10" viewBox="0 0 2 2">
			    <circle cx="1" cy="1" r="1" />
			  </symbol>

			   <!-- A grid to materialize our symbol positioning -->
			  <path d="M0,10 h80 M10,0 v20 M25,0 v20 M40,0 v20 M55,0 v20 M70,0 v20" fill="none" stroke="pink" />

			  <!-- All instances of our symbol -->
			  <use xlink:href="#myDot" x="5"  y="5" style="opacity:1.0" />
			  <use xlink:href="#myDot" x="20" y="5" style="opacity:0.8" />
			  <use xlink:href="#myDot" x="35" y="5" style="opacity:0.6" />
			  <use xlink:href="#myDot" x="50" y="5" style="opacity:0.4" />
			  <use xlink:href="#myDot" x="65" y="5" style="opacity:0.2" />
			</svg>
		""".trimIndent()
		)
	}

	fun drawTransformPathSvg() {
		svg?.setSrc(
			"""
  <svg version="1.1"
        xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
        width="128" height="128">
        <image width="128" height="128" transform="rotate(45)" transform-origin="64 64"
            xlink:href="https://www.rust-lang.org/logos/rust-logo-128x128.png"/>
        </svg>
		""".trimIndent()
		)
	}

	fun drawRadialGradientCircleSvg() {
		svg?.setSrc(
			"""
<svg  viewBox="0 0 1200 800" version="1.1"
     xmlns="http://www.w3.org/2000/svg">
  <desc>Example radgrad01 - fill a rectangle by referencing a
           radial gradient paint server</desc>
  <g>
    <defs>
      <radialGradient id="MyGradient" gradientUnits="userSpaceOnUse"
                      cx="400" cy="200" r="300" fx="400" fy="200">
        <stop offset="0%" stop-color="red" />
        <stop offset="50%" stop-color="blue" />
        <stop offset="100%" stop-color="red" />
      </radialGradient>
    </defs>

    <!-- Outline the drawing area in blue -->
    <rect fill="none" stroke="blue"
          x="1" y="1" width="798" height="398" />

    <!-- The rectangle is filled using a radial gradient paint server -->
    <rect fill="url(#MyGradient)" stroke="black" stroke-width="5"
          x="100" y="100" width="600" height="200"/>
  </g>
</svg>
		""".trimIndent()
		)
	}

	fun drawLinearGradientCircleSvg() {
		svg?.setSrc(
			"""
			<svg he=>
			  <defs>
			    <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="0%">
			      <stop offset="0%" style="stop-color:rgb(255,255,0);stop-opacity:1" />
			      <stop offset="100%" style="stop-color:rgb(255,0,0);stop-opacity:1" />
			    </linearGradient>
			  </defs>
			  <ellipse cx="200" cy="70" rx="85" ry="55" fill="url(#grad1)" />
			  Sorry, your browser does not support inline SVG.
			</svg>
		""".trimIndent()
		)
	}

	fun drawLinearGradientSvg() {
		svg?.setSrc(
			"""
			<svg width="400" height="550">
			  <defs>
			    <linearGradient id="MyGradient" gradientUnits="userSpaceOnUse" x1="100" y1="0" x2="300" y2="0">
			      <stop offset="0" style="stop-color:#000000" />
			      <stop offset=".33" style="stop-color:#ffffff" />
			      <stop offset=".67" style="stop-color:#ffff00" />
			      <stop offset="1" style="stop-color:#808080" />
			    </linearGradient>
			    <filter id="normal">
			      <feBlend mode="normal" in="SourceGraphic" />
			    </filter>
			    <filter id="multiply">
			      <feBlend mode="multiply" in="SourceGraphic" />
			    </filter>
			    <filter id="screen">
			      <feBlend mode="screen" in="SourceGraphic" />
			    </filter>
			    <filter id="darken">
			      <feBlend mode="darken" in="SourceGraphic" />
			    </filter>
			    <filter id="lighten">
			      <feBlend mode="lighten" in="SourceGraphic" />
			    </filter>
			  </defs>
			  <g style="enable-background:new">
			    <rect x="40" y="20" width="300" height="450" style="fill:url(#MyGradient)" />
			    <g style="font-size:75px;fill:#888888;fill-opacity:.6">
			      <text x="50" y="90" filter="url(#normal)">Normal</text>
			      <text x="50" y="180" filter="url(#multiply)">Multiply</text>
			      <text x="50" y="270" filter="url(#screen)">Screen</text>
			      <text x="50" y="360" filter="url(#darken)">Darken</text>
			      <text x="50" y="450" filter="url(#lighten)">Lighten</text>
			    </g>
			  </g>
			  Sorry, your browser does not support inline SVG.
			</svg>
		""".trimIndent()
		)
	}

	// https://upload.wikimedia.org/wikipedia/commons/4/4c/The_Hague%2C_Netherlands%2C_the_old_city_center.svg
	// https://upload.wikimedia.org/wikipedia/commons/6/6c/Trajans-Column-lower-animated.svg

	// https://upload.wikimedia.org/wikipedia/commons/b/b1/Cluse_de_Chamb%C3%A9ry_-_Carte_de_l%27occupation_des_sols_%28CORINE%29.svg


	//https://upload.wikimedia.org/wikipedia/commons/b/b6/Moldova_%281483%29-en.svg

	// https://upload.wikimedia.org/wikipedia/commons/a/a0/Location_map_San_Francisco_Bay_Area.svg // 40mb

	// https://upload.wikimedia.org/wikipedia/commons/c/c1/Propane_flame_contours-en.svg

	// https://upload.wikimedia.org/wikipedia/commons/9/95/Kaiserstandarte_Version1.svg

	// https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/car.svg

	// https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/lineargradient1.svg

	//http://thenewcode.com/assets/images/thumbnails/homer-simpson.svg

	// https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/AJ_Digital_Camera.svg

	// https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/tiger.svg

	// https://upload.wikimedia.org/wikipedia/commons/f/ff/1_42_polytope_7-cube.svg

	// https://upload.wikimedia.org/wikipedia/commons/7/7c/Map_of_the_world_by_the_US_Gov_as_of_2016_no_legend.svg

	// https://upload.wikimedia.org/wikipedia/commons/9/9d/The_Rhodopes_on_The_Paths_Of_Orpheus_And_Eurydice_Project_Map.svg

	// https://upload.wikimedia.org/wikipedia/commons/1/1c/KINTETSU23000_20140424A.svg

	// https://raw.githubusercontent.com/RazrFalcon/resvg/7b26adbcc9698dcca687214c84d216794f60a5be/tests/svg/e-radialGradient-013.svg
	fun downloadSvg() {
		executor.execute {
			try {
				val svgFile = File(filesDir, "svg_file.svg")
				if (svgFile.exists()) {
					//svg?.setSrcPath(svgFile.absolutePath)
					svgFile.delete()
				}

				val url =
					URL("https://upload.wikimedia.org/wikipedia/commons/9/9d/The_Rhodopes_on_The_Paths_Of_Orpheus_And_Eurydice_Project_Map.svg")
				val fs = FileOutputStream(svgFile)
				url.openStream().use { input ->
					fs.use { output ->
						input.copyTo(output)
					}
				}
				svg?.setSrcPath(svgFile.absolutePath)
			} catch (e: IOException) {
				e.printStackTrace()
			}
		}
	}

	fun drawClipPathUnitsSvg() {
		svg?.setSrc(
			"""<?xml version="1.0"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN"
  "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">

<svg xmlns="http://www.w3.org/2000/svg"
      width="526" height="233">
  <rect x="13" y="14" width="500" height="200" rx="50" ry="100"
      fill="none" stroke="blue" stroke-width="10" />
</svg>
		""".trimIndent()
		)
	}

	fun drawTransformGradientSvg() {
		svg?.setSrc(
			"""
		<svg width="500" height="500" viewBox="0 0 500 500" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
    <defs>
        <linearGradient id="user-grad" gradientUnits="userSpaceOnUse" x1="0" y1="0" x2="100" y2="100" gradientTransform="scale(2, 1)">
          <stop stop-color="orange" offset="0"/>
          <stop stop-color="blue" offset="1"/>
        </linearGradient>
        <linearGradient id="box-grad" x1="0" y1="0" x2="100%" y2="100%">
          <stop stop-color="orange" offset="0"/>
          <stop stop-color="blue" offset="1"/>
        </linearGradient>
    </defs>
    <rect x="0" y="0" width="200" height="100" fill="url(#user-grad)"/>
    <rect x="250" y="0" width="200" height="100" fill="url(#box-grad)"/>
</svg>
		""".trimIndent()
		)
	}

	fun drawTransformMatrixSvg() {

		svg?.setSrc(
			"""
			<svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
		<rect x="10" y="10" width="30" height="20" fill="green" />

		<!--
		In the following example we are applying the matrix:
		[a c e]    [3 -1 30]
		[b d f] => [1  3 40]
		[0 0 1]    [0  0  1]

		which transform the rectangle as such:

		top left corner: oldX=10 oldY=10
		newX = a * oldX + c * oldY + e = 3 * 10 - 1 * 10 + 30 = 50
		newY = b * oldX + d * oldY + f = 1 * 10 + 3 * 10 + 40 = 80

		top right corner: oldX=40 oldY=10
		newX = a * oldX + c * oldY + e = 3 * 40 - 1 * 10 + 30 = 140
		newY = b * oldX + d * oldY + f = 1 * 40 + 3 * 10 + 40 = 110

		bottom left corner: oldX=10 oldY=30
		newX = a * oldX + c * oldY + e = 3 * 10 - 1 * 30 + 30 = 30
		newY = b * oldX + d * oldY + f = 1 * 10 + 3 * 30 + 40 = 140

		bottom right corner: oldX=40 oldY=30
		newX = a * oldX + c * oldY + e = 3 * 40 - 1 * 30 + 30 = 120
		newY = b * oldX + d * oldY + f = 1 * 40 + 3 * 30 + 40 = 170
		-->

		</svg>
			""".trimIndent()
		)
	}

	fun drawTransformTranslateSvg() {
		/// translate transform

		svg?.setSrc(
			"""
		<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <!-- No translation -->
  <rect x="5" y="5" width="40" height="40" fill="green" />

  <!-- Horizontal translation -->
  <rect x="5" y="5" width="40" height="40" fill="blue"
        transform="translate(50)" />

  <!-- Vertical translation -->
  <rect x="5" y="5" width="40" height="40" fill="red"
        transform="translate(0 50)" />

  <!-- Both horizontal and vertical translation -->
  <rect x="5" y="5" width="40" height="40" fill="yellow"
         transform="translate(50,50)" />
</svg>
			""".trimIndent()
		)
	}

	fun drawTransformScaleSvg() {
		svg?.setSrc(
			"""
				<svg viewBox="-50 -50 100 100" xmlns="http://www.w3.org/2000/svg">
				  <!-- uniform scale -->
				  <circle cx="0" cy="0" r="10" fill="red"
				          transform="scale(4)" />

				  <!-- vertical scale -->
				  <circle cx="0" cy="0" r="10" fill="yellow"
				          transform="scale(1,4)" />

				  <!-- horizontal scale -->
				  <circle cx="0" cy="0" r="10" fill="pink"
				          transform="scale(4,1)" />

				  <!-- No scale -->
				  <circle cx="0" cy="0" r="10" fill="black" />
				</svg>
			""".trimIndent()
		)
	}

	fun drawTransformRotateSvg() {
		svg?.setSrc(
			"""
			<svg viewBox="-12 -2 34 14" xmlns="http://www.w3.org/2000/svg">
			  <rect x="0" y="0" width="10" height="10" />

			  <!-- rotation is done around the point 0,0 -->
			  <rect x="0" y="0" width="10" height="10" fill="red"
			        transform="rotate(100)" />

			  <!-- rotation is done around the point 10,10 -->
			  <rect x="0" y="0" width="10" height="10" fill="green"
			        transform="rotate(100,10,10)" />
			</svg>
		""".trimIndent()
		)
	}

	fun drawTransformSkewX() {
		svg?.setSrc(
			"""
			<svg viewBox="-5 -5 10 10" xmlns="http://www.w3.org/2000/svg">
			  <rect x="-3" y="-3" width="6" height="6" />

			  <rect x="-3" y="-3" width="6" height="6" fill="red"
			        transform="skewX(30)" />
			</svg>
		""".trimIndent()
		)
	}

	fun drawTransformSkewY() {
		svg?.setSrc(
			"""
			<svg viewBox="-5 -5 10 10" xmlns="http://www.w3.org/2000/svg">
			  <rect x="-3" y="-3" width="6" height="6" />

			  <rect x="-3" y="-3" width="6" height="6" fill="red"
			        transform="skewY(30)" />
			</svg>
		""".trimIndent()
		)
	}

	internal class ByteArrayOutputStream2 : ByteArrayOutputStream {
		constructor() : super() {}
		constructor(size: Int) : super(size) {}

		/**
		 * Returns the internal buffer of this ByteArrayOutputStream, without copying.
		 */
		@Synchronized
		fun buf(): ByteArray {
			return buf
		}
	}

	var t0 = 0.0
	var rr = 0.0 // the radius that changes over time
	var a = 0.0 // angle
	private val PI2 = Math.PI * 2
	private var lastTime = 0L
	var timeToCall = 0L
	var handler = Handler(Looper.getMainLooper())


	companion object {
		@JvmStatic
		fun init() {
			val policy = StrictMode.ThreadPolicy.Builder().permitAll().build()
			StrictMode.setThreadPolicy(policy)
		}
	}

	var CanvasXSize = 800;
	var CanvasYSize = 200;
	var speed = 30; // lower is faster
	var scale = 1.05;
	var y = -4.5; // vertical offset

// Main program

	var dx = 0.75;
	var imgW = 0
	var imgH = 0
	var x = 0;
	var clearX = 0
	var clearY = 0


	fun log(message: Any) {
		println("${message}")
	}

	@SafeVarargs
	fun log(vararg message: Any) {
		var msg = "";
		message.forEach {
			msg += ":$it:"
		}
		println(msg)
	}

	val circle = "<svg height=\"100\" width=\"100\">" +
		"<circle cx=\"50\" cy=\"50\" r=\"40\" stroke=\"black\" stroke-width=\"3\" fill=\"red\" />\n" +
		"  Sorry, your browser does not support inline SVG.  \n" +
		"</svg> "
	val rect = "<svg width=\"400\" height=\"110\">\n" +
		"  <rect width=\"300\" height=\"100\" style=\"fill:rgb(0,0,255);stroke-width:3;stroke:rgb(0,0,0)\" />\n" +
		"  Sorry, your browser does not support inline SVG.  \n" +
		"</svg>"


	val alphaRect = "<svg width=\"400\" height=\"180\">\n" +
		"  <rect x=\"50\" y=\"20\" width=\"150\" height=\"150\" style=\"fill:blue;stroke:pink;stroke-width:5;fill-opacity:0.1;stroke-opacity:0.9\" />\n" +
		"  Sorry, your browser does not support inline SVG.  \n" +
		"</svg>"

	var path = "<svg height=\"400\" width=\"450\">\n" +
		"  <path id=\"lineAB\" d=\"M 100 350 l 150 -300\" stroke=\"red\"\n" +
		"  stroke-width=\"3\" fill=\"none\" />\n" +
		"  <path id=\"lineBC\" d=\"M 250 50 l 150 300\" stroke=\"red\"\n" +
		"  stroke-width=\"3\" fill=\"none\" />\n" +
		"  <path d=\"M 175 200 l 150 0\" stroke=\"green\" stroke-width=\"3\"\n" +
		"  fill=\"none\" />\n" +
		"  <path d=\"M 100 350 q 150 -300 300 0\" stroke=\"blue\"\n" +
		"  stroke-width=\"5\" fill=\"none\" />\n" +
		"  <!-- Mark relevant points -->\n" +
		"  <g stroke=\"black\" stroke-width=\"3\" fill=\"black\">\n" +
		"    <circle id=\"pointA\" cx=\"100\" cy=\"350\" r=\"3\" />\n" +
		"    <circle id=\"pointB\" cx=\"250\" cy=\"50\" r=\"3\" />\n" +
		"    <circle id=\"pointC\" cx=\"400\" cy=\"350\" r=\"3\" />\n" +
		"  </g>\n" +
		"  <!-- Label the points -->\n" +
		"  <g font-size=\"30\" font-family=\"sans-serif\" fill=\"black\" stroke=\"none\"\n" +
		"  text-anchor=\"middle\">\n" +
		"    <text x=\"100\" y=\"350\" dx=\"-30\">A</text>\n" +
		"    <text x=\"250\" y=\"50\" dy=\"-10\">B</text>\n" +
		"    <text x=\"400\" y=\"350\" dx=\"30\">C</text>\n" +
		"  </g>\n" +
		"</svg>"

	fun drawSVG(view: View) {

	}

}
