package com.github.triniwiz.canvasdemo

import android.animation.TimeAnimator
import android.annotation.SuppressLint
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.graphics.Color
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.os.StrictMode
import android.util.Log
import android.view.View
import android.widget.FrameLayout
import androidx.appcompat.app.AppCompatActivity
import com.github.triniwiz.canvas.*
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.net.URL
import java.util.*
import java.util.concurrent.Executors
import kotlin.math.*


class MainActivity : AppCompatActivity() {
    var canvas: CanvasView? = null
    var svg: SVGView? = null
    var ctx: CanvasRenderingContext2D? = null
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        canvas = findViewById(R.id.canvasView)
        canvas?.setListener {
            print("Is Ready")
        }
        init()
    }

    var r = 100f; // Radius
    var p0 = KeyValue(0f, 50f);

    var p1 = KeyValue(100f, 100f)
    var p2 = KeyValue(150f, 50f);
    var p3 = KeyValue(200f, 100f);

    private fun textPoint(
        ctx: CanvasRenderingContext2D,
        p: KeyValue,
        offset: KeyValue,
        i: Int = 0
    ) {
        val x = offset.x
        val y = offset.y
        ctx.beginPath()
        ctx.arc(p.x, p.y, 2f, 0f, (Math.PI * 2).toFloat())
        ctx.fill()
        ctx.fillText("" + i + ":" + p.x + "," + p.y, p.x + x, p.y + y)
    }

    fun drawPoints(ctx: CanvasRenderingContext2D, points: Array<KeyValue>) {
        for (point in points) {
            val i = points.indexOf(point)
            val p = points[i]
            textPoint(ctx, p, KeyValue(0, -20), i)
        }
    }

    fun drawCreateImageData(ctx: CanvasRenderingContext2D) {
        val imageData = ctx.createImageData((100 * scale).toInt(), (100 * scale).toInt())
// Iterate through every pixel
        var i = 0
        imageData.data.rewind()
        val size = imageData.data.remaining()

        while (i <= size) {
            var next = 0
            if (i + 0 <= size) {
                imageData.data.put(190.toByte())
                next = 1
            }
            if (i + 1 <= size) {
                imageData.data.put(0.toByte())
                next = 2
            }
            if (i + 2 <= size) {
                imageData.data.put(210.toByte())
                next = 3
            }
            if (i + 3 <= size) {
                imageData.data.put(255.toByte())
                next = 4
            }
            i += next
        }
        ctx.putImageData(imageData)
    }

    fun drawArc(ctx: CanvasRenderingContext2D, points: Array<KeyValue>, r: Float) {
        val p0 = points[0]
        val p1 = points[1]
        val p2 = points[2]
        ctx.beginPath();
        ctx.moveTo(p0.x, p0.y);
        ctx.arcTo(p1.x, p1.y, p2.x, p2.y, r);
        ctx.lineTo(p2.x, p2.y);
        ctx.stroke();
    }

    var t0 = 0.0
    var rr = 0.0 // the radius that changes over time
    var a = 0.0 // angle
    private val PI2 = Math.PI * 2
    private var lastTime = 0L
    var timeToCall = 0L
    var handler = Handler(Looper.getMainLooper())


    fun solarAnimation(ctx: CanvasRenderingContext2D) {
        AnimationFrame.requestAnimationFrame { called ->
            run {
                animateSolarSystem(ctx, called.toFloat())
            }
        }
    }

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

    fun drawPano(ctx: CanvasRenderingContext2D, img: Bitmap) {
        val runnableCode = object : Runnable {
            override fun run() {
                ctx.clearRect(0F, 0F, clearX.toFloat(), clearY.toFloat()); // clear the canvas

                // if image is <= Canvas Size
                if (imgW <= CanvasXSize) {
                    // reset, start from beginning
                    if (x > CanvasXSize) {
                        x += -imgW;
                    }
                    // draw additional image1
                    if (x > 0) {
                        ctx.drawImage(
                            img,
                            (-imgW + x).toFloat(),
                            y.toFloat(),
                            imgW.toFloat(),
                            imgH.toFloat()
                        );
                    }
                    // draw additional image2
                    if (x - imgW > 0) {
                        ctx.drawImage(
                            img,
                            (-imgW * 2 + x).toFloat(),
                            y.toFloat(),
                            imgW.toFloat(),
                            imgH.toFloat()
                        );
                    }
                }

                // image is > Canvas Size
                else {
                    // reset, start from beginning
                    if (x > (CanvasXSize)) {
                        x = CanvasXSize - imgW;
                    }
                    // draw aditional image
                    if (x > (CanvasXSize - imgW)) {
                        ctx.drawImage(
                            img,
                            (x - imgW + 1).toFloat(),
                            y.toFloat(),
                            imgW.toFloat(),
                            imgH.toFloat()
                        );
                    }
                }
                // draw image
                ctx.drawImage(img, x.toFloat(), y.toFloat(), imgW.toFloat(), imgH.toFloat());
                // amount to move
                x += (dx).toInt()
                handler.postDelayed(this, speed.toLong())
            }
        }
        handler.post(runnableCode)
    }

    fun panoramaSectionAnimation(ctx: CanvasRenderingContext2D) {
        CanvasXSize = ctx.canvas.width
        CanvasYSize = ctx.canvas.height
        Log.d("com.github", "w " + CanvasXSize + " H " + CanvasYSize)
        try {
            val file = File(filesDir, "Capitan_Meadows,_Yosemite_National_Park.jpg")
            var img: Bitmap?
            if (file.exists()) {
                img = BitmapFactory.decodeFile(file.absolutePath)
                Log.d("com.github", "w " + img?.width + " H " + img?.height)
            } else {
                val url =
                    URL("https://mdn.mozillademos.org/files/4553/Capitan_Meadows,_Yosemite_National_Park.jpg")
                val fs = FileOutputStream(file)
                url.openStream().use { input ->
                    fs.use { output ->
                        input.copyTo(output)
                    }
                }
                img = BitmapFactory.decodeFile(file.absolutePath)
            }

            imgW = (img!!.width * scale).toInt()

            imgH = (img.height * scale).toInt()

            if (imgW > CanvasXSize) {
                // image larger than canvas
                x = CanvasXSize - imgW;
            }
            if (imgW > CanvasXSize) {
                // image width larger than canvas
                clearX = imgW;
            } else {
                clearX = CanvasXSize;
            }
            if (imgH > CanvasYSize) {
                // image height larger than canvas
                clearY = imgH;
            } else {
                clearY = CanvasYSize;
            }

            drawPano(ctx, img)

        } catch (e: IOException) {

        }
    }

    var sun: Bitmap? = null
    var moon: Bitmap? = null
    var earth: Bitmap? = null

    fun animateSolarSystem(ctx: CanvasRenderingContext2D, t: Float) {
        try {
            val sunFile = File(filesDir, "Canvas_sun.png")
            val moonFile = File(filesDir, "Canvas_moon.png")
            val earthFile = File(filesDir, "Canvas_earth.png")


            if (sun == null) {
                if (sunFile.exists()) {
                    sun = BitmapFactory.decodeFile(sunFile.absolutePath)
                } else {
                    val url = URL("https://mdn.mozillademos.org/files/1456/Canvas_sun.png")
                    val fs = FileOutputStream(sunFile)
                    url.openStream().use { input ->
                        fs.use { output ->
                            input.copyTo(output)
                        }
                    }
                    sun = BitmapFactory.decodeFile(sunFile.absolutePath)
                }
            }


            if (moon == null) {
                if (moonFile.exists()) {
                    moon = BitmapFactory.decodeFile(moonFile.absolutePath)
                } else {
                    val url = URL("https://mdn.mozillademos.org/files/1443/Canvas_moon.png")
                    val fs = FileOutputStream(moonFile)
                    url.openStream().use { input ->
                        fs.use { output ->
                            input.copyTo(output)
                        }
                    }
                    moon = BitmapFactory.decodeFile(moonFile.absolutePath)
                }
            }

            if (earth == null) {
                if (earthFile.exists()) {
                    earth = BitmapFactory.decodeFile(earthFile.absolutePath)
                } else {
                    val url = URL("https://mdn.mozillademos.org/files/1429/Canvas_earth.png")
                    val fs = FileOutputStream(earthFile)
                    url.openStream().use { input ->
                        fs.use { output ->
                            input.copyTo(output)
                        }
                    }
                    earth = BitmapFactory.decodeFile(earthFile.absolutePath)
                }
            }



            ctx.globalCompositeOperation = CanvasCompositeOperationType.DestinationOver
            ctx.clearRect(0F, 0F, 300F, 300F) // clear canvas

            ctx.fillStyle =
                Color("rgba(0, 0, 0, 0.4)")
            ctx.strokeStyle =
                Color("rgba(0, 153, 255, 0.4)")
            ctx.save()
            ctx.translate(150F, 150F)

            // Earth
            val time = Date()
            ctx.rotate((2 * Math.PI / 60 * (time.time / 1000) + 2 * Math.PI / 60000 * time.time).toFloat())
            ctx.translate(105F, 0F)
            ctx.fillRect(0F, -12F, 40F, 24F) // Shadow
            ctx.drawImage(earth, -12F, -12F)

            // Moon
            ctx.save()
            ctx.rotate((2 * Math.PI / 6 * (time.time / 1000) + 2 * Math.PI / 6000 * time.time).toFloat())
            ctx.translate(0F, 28.5F)
            ctx.drawImage(moon, -3.5F, -3.5F)
            ctx.restore()

            ctx.restore()

            ctx.beginPath()
            ctx.arc(150F, 150F, 105F, 0F, (PI * 2).toFloat(), false) // Earth orbit
            ctx.stroke()

            ctx.drawImage(sun, 0F, 0F, 300F, 300F)

            AnimationFrame.requestAnimationFrame { called ->
                run {
                    animateSolarSystem(ctx, called.toFloat())
                }
            }

        } catch (e: IOException) {
            e.printStackTrace()
        }
    }


    fun loop(ctx: CanvasRenderingContext2D, t: Float) {
        AnimationFrame.requestAnimationFrame { called ->
            loop(ctx, called.toFloat())
        }
        t0 = t / 1000.0
        a = t0 % PI2
        rr = abs(cos(a) * r)
        ctx.clearRect(0f, 0f, ctx.canvas.width.toFloat(), ctx.canvas.height.toFloat());
        val points = arrayOf(p1, p2, p3)
        drawArc(ctx, points, rr.toFloat())
        drawPoints(ctx, points)
    }

    fun getImageData(ctx: CanvasRenderingContext2D) {
        ctx.rect(10F, 10F, 100F, 100F)
        ctx.fill()

        val imageData = ctx.getImageData(60F, 60F, 200, 100)
        ctx.putImageData(imageData, 150F, 10F)
    }

    @SuppressLint("NewApi")
    fun drawShadowAlpha(ctx: CanvasRenderingContext2D) {
        // Shadow

        ctx.shadowColor = Color.argb(0.8F, (255 / 255).toFloat(), 0F, 0F)
        ctx.shadowBlur = 8F
        ctx.shadowOffsetX = 30F
        ctx.shadowOffsetY = 20F

// Filled rectangle
        ctx.fillStyle = Color(
            Color.argb(
                0.2F,
                0F,
                (255 / 255).toFloat(),
                0F
            )
        )
        ctx.fillRect(10F, 10F, 150F, 100F)

// Stroked rectangle
        ctx.lineWidth = 10F
        ctx.strokeStyle = Color(
            Color.argb(
                0.6F,
                0F,
                0F,
                (255 / 255).toFloat()
            )
        )
        ctx.strokeRect(10F, 10F, 150F, 100F);
    }

    fun drawShadow(ctx: CanvasRenderingContext2D) {
        // Shadow
        ctx.shadowColor = Color.RED;
        ctx.shadowOffsetX = 10F;
        ctx.shadowOffsetY = 10F;

// Filled rectangle
        ctx.fillRect(20F, 20F, 100F, 100F);

// Stroked rectangle
        ctx.lineWidth = 6F;
        ctx.strokeRect(170F, 20F, 100F, 100F);
    }

    fun drawText(ctx: CanvasRenderingContext2D) {
        ctx.font = (50).toString() + "px serif"
        ctx.shadowColor = Color.BLUE
        ctx.shadowBlur = 10F
        ctx.shadowOffsetX = 100F
        ctx.shadowOffsetY = 100F
        ctx.strokeText("Hello world", 50F, 90F);
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

    fun faceLoop(ctx: CanvasRenderingContext2D) {
        ctx.fillStyle = Color("white")
        ctx.fillRect(0f, 0f, ctx.canvas.width.toFloat(), ctx.canvas.height.toFloat())
        ctx.fillStyle = Color("black")
        drawFace(ctx)
        AnimationFrame.requestAnimationFrame {
            faceLoop(ctx)
        }
    }


    var vertexShaderSource = """
    #version 100
        void main() {
            gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
            gl_PointSize = 64.0;
        }
    """
    var fragmentShaderSource = """
        precision mediump float;
            void main() {
                vec2 fragmentPosition = 2.0*gl_PointCoord - 1.0;
                float distance = length(fragmentPosition);
                float distanceSqrd = distance * distance;
                gl_FragColor = vec4(
                    0.2/distanceSqrd,
                    0.1/distanceSqrd,
                    0.0, 1.0 );
                }
    """


    var buffer: Int? = null
    var gl: WebGLRenderingContext? = null
    var program: Int? = null
    fun initializeAttributes() {
        gl!!.enableVertexAttribArray(0)
        buffer = gl!!.createBuffer()
        gl!!.bindBuffer(gl!!.ARRAY_BUFFER, buffer!!)
        val data = floatArrayOf(0f, 0f)
        gl!!.bufferData(gl!!.ARRAY_BUFFER, data, gl!!.STATIC_DRAW);
        gl!!.vertexAttribPointer(0, 2, gl!!.FLOAT, false, 0, 0)
    }

    fun cleanup() {
        gl!!.useProgram(0)
        if (buffer != null) {
            gl!!.deleteBuffer(buffer!!)
        }
        if (program != null) {
            gl!!.deleteProgram(program!!)
        }

    }


    fun debugLog(message: String) {
        Log.d("com.test", message);
    }

    fun drawGL(gl: WebGLRenderingContext) {
        gl.viewport(
            0, 0,
            gl.drawingBufferWidth, gl.drawingBufferHeight
        )
        debugLog("vp " + gl.error)
        val vp = gl.getParameter(gl.VIEWPORT)
        if (vp is Array<*>) {
            debugLog("array $vp")
            debugLog("vp size " + vp[0] + " " + vp[1])
        }
        val vertexShader = gl.createShader(gl.VERTEX_SHADER)
        Log.d("com.test", "vertexShader: " + vertexShader)
        gl.shaderSource(vertexShader, vertexShaderSource)
        gl.compileShader(vertexShader)
        val fragmentShader = gl.createShader(gl.FRAGMENT_SHADER)
        Log.d("com.test", "fragmentShader: " + fragmentShader)
        gl.shaderSource(fragmentShader, fragmentShaderSource)
        gl.compileShader(fragmentShader)
        val program = gl.createProgram()
        gl.attachShader(program, vertexShader)
        gl.attachShader(program, fragmentShader)
        gl.linkProgram(program)
        gl.detachShader(program, vertexShader)
        gl.detachShader(program, fragmentShader)
        gl.deleteShader(vertexShader)
        gl.deleteShader(fragmentShader)

        var status = (gl.getProgramParameter(program, gl.LINK_STATUS) as Boolean)
        if (!status) {
            val linkErrLog = gl.getProgramInfoLog(program)
            Log.d("com", "error: " + linkErrLog)
            cleanup()
        }

        initializeAttributes()
        gl.useProgram(program)
        gl.drawArrays(gl.POINTS, 0, 1)
        cleanup()
        canvas?.flush()
    }


    fun drawPattern(canvas: CanvasView) {
        val img = File(filesDir, "Canvas_createpattern.png")

        val asset = ImageAsset()

        if (img.exists()) {
            asset.loadImageFromPath(img.absolutePath)
        } else {
            val url = URL("https://mdn.mozillademos.org/files/222/Canvas_createpattern.png")
            val fs = FileOutputStream(img)
            url.openStream().use { input ->
                fs.use { output ->
                    input.copyTo(output)
                }
            }
            asset.loadImageFromPath(img.absolutePath)
        }

        val ctx = canvas.getContext("2d") as CanvasRenderingContext2D
        val pattern = ctx.createPattern(asset, Pattern.PatternRepetition.Repeat)
        ctx.fillStyle = pattern

        ctx.fillRect(0f, 0f, 300f, 300f);


    }

    var didPause = false;
    override fun onPause() {
        super.onPause()
        canvas?.onPause()
        didPause = true;
    }

    override fun onResume() {
        super.onResume()
        if (didPause) {
            canvas?.onResume()
            ctx = canvas?.getContext("2d") as CanvasRenderingContext2D?
            ballExample(ctx!!)
        }
    }

    val executor = Executors.newSingleThreadExecutor()

    @SuppressLint("NewApi")
    fun drawFill(view: View) {
        ctx = canvas?.getContext("2d") as CanvasRenderingContext2D?
        // ballExample(ctx!!)
        //drawPattern(canvas!!)
        // drawFace(ctx!!)
        //drawPattern(canvas!!)
       // drawElements(canvas!!)
       // drawPatterWithCanvas(canvas!!)
        executor.submit {
           // drawPatterWithCanvas(canvas!!)
          //  drawPatterWithCanvas(canvas!!)
          //  canvas?.isHandleInvalidationManually = true
            //drawElements(canvas!!)
            //ctx = canvas?.getContext("2d") as CanvasRenderingContext2D?
            //drawFace(ctx!!)
            //canvas?.flush()
            // ballExample(ctx!!)
        }
      //  ctx = canvas?.getContext("2d") as CanvasRenderingContext2D?

      //  drawImageExample(ctx!!)
        // drawPattern(canvas!!)
        // ctx = canvas?.getContext("2d") as CanvasRenderingContext2D?
        //drawImageExample(ctx!!)
        // drawFace(ctx!!)
        //ctx?.fillStyle = Color.BLACK
        // ctx?.fillRect(0F,0F,200f,200f)
        // ballExample(ctx!!)
        /* ctx?.fillStyle = CanvasColorStyle.Color(Color.BLUE)
         ctx?.clearRect(0F,0F, canvas!!.width.toFloat(), canvas!!.height.toFloat())
         ctx?.fillRect(0F,0F,200f,200f)
         postDelayed(handler, Runnable{
             ctx?.clearRect(0F,0F, canvas!!.width.toFloat(), canvas!!.height.toFloat())
             ctx?.fillStyle = CanvasColorStyle.Color(Color.BLACK)
             drawImageExample(ctx!!)
         },null,4000)*/
        //drawHouse(ctx!!)
        //drawSVG(svgView!!)
        /* val cs = CanvasView(this)
         Log.d("com.test", "params " +  cs.layoutParams)
         cs.layoutParams = ViewGroup.LayoutParams(300,300)
         cs.surface.layoutParams =  ViewGroup.LayoutParams(300,300)
         Log.d("com.test", "www: " +  cs.surface.width + "  " + cs.width)
        // cs.onResume()
         val os = cs.getContext("2d") as CanvasRenderingContext2D
         os.strokeStyle = Color(Color.RED)
         val data = os.createImageData(300,300)
         Log.d("com.test",  " f " +  data.data[0] + " s" + data.data[1])
         os.strokeRect(0f,0f,300f,300f)
         ctx?.drawImage(cs,0f,0f)

         */

       //  drawHouse(ctx!!)
        /*canvasView.toDataURLAsync {
            Log.d("com.test", "aaaa: " + it)
        }
        canvasView.toDataURLAsync {
            Log.d("com.test", "bbbb: " + it)
        }
        canvasView.toDataURLAsync {
            Log.d("com.test", "cccc: " + it)
        }
        canvasView.toDataURLAsync {
            Log.d("com.test", "dddd: " + it)
        }*/

        //  drawPatterWithCanvas(canvas!!)
        // drawImageExample(ctx!!)
        // drawImageSmoothingEnabled(ctx!!)
      //  gl = canvas?.getContext("webgl") as WebGLRenderingContext
        // drawElements(canvas!!)
        //  Log.d("com.test", "ext: " +   gl!!.getExtension("ANGLE_instanced_arrays"))

        // drawRotatingCube(gl!!)
      /*

      "points" ->
            "line_strip" ->
            "line_loop" ->
            "triangle_strip" ->
            "triangle_fan" ->
            "triangles" ->
       */
       // drawModes(canvas!!, "triangle_strip")
        // draw(ctx!!)

        // Create clipping path
        // Create clipping path

         solarAnimation(ctx!!)
        // draw(ctx!!)

        // canvas?.flush()
        //  loop(ctx!!, 0F)
        /*val data = canvas!!.toDataURL();
        Log.d("com.test", "url: " + data)

        //drawImageExample(ctx!!)
       // val data = canvas!!.toData()
        //Log.d("com.test", "stfff: " + data)
        val file = File(applicationContext.filesDir,"base64.txt")
        val fos = FileOutputStream(file)
        fos.write(data.toByteArray(StandardCharsets.UTF_8))
        fos.close()
        */

    }


    fun drawImageWithCanvasIngl(canvas: CanvasView) {
        val patternCanvas = CanvasView(this)
        val patternContext = patternCanvas.getContext("2d") as CanvasRenderingContext2D
// Give the pattern a width and height of 50
        val scale = resources.displayMetrics.density
        val width = (300 * scale).toInt()
        val height = (300 * scale).toInt()
        patternCanvas.layoutParams = FrameLayout.LayoutParams(width, height)
        patternCanvas.surface.layoutParams = FrameLayout.LayoutParams(width, height)
        val w = View.MeasureSpec.makeMeasureSpec(width, View.MeasureSpec.EXACTLY)
        val h = View.MeasureSpec.makeMeasureSpec(height, View.MeasureSpec.EXACTLY)
        patternCanvas.measure(w, h)
        patternCanvas.layout(0, 0, width, height)

        drawImageSmoothingEnabled(patternContext)

// Create our primary canvas and fill it with the pattern

        val ctx = canvas.getContext("webgl") as WebGLRenderingContext
    }

    fun drawPatterWithCanvas(canvas: CanvasView) {
        val patternCanvas = CanvasView(this)
        val patternContext = patternCanvas.getContext("2d") as CanvasRenderingContext2D
// Give the pattern a width and height of 50
        val scale = resources.displayMetrics.density
        val width = (50 * scale).toInt()
        val height = (50 * scale).toInt()
        patternCanvas.layoutParams = FrameLayout.LayoutParams(width, height)
        patternCanvas.surface.layoutParams = FrameLayout.LayoutParams(width, height)
        val w = View.MeasureSpec.makeMeasureSpec(width, View.MeasureSpec.EXACTLY)
        val h = View.MeasureSpec.makeMeasureSpec(height, View.MeasureSpec.EXACTLY)
        patternCanvas.measure(w, h)
        patternCanvas.layout(0, 0, width, height)


// Give the pattern a background color and draw an arc
        patternContext.fillStyle = "#fec"
        patternContext.fillRect(
            0f, 0f, patternCanvas.width.toFloat(),
            patternCanvas.height.toFloat()
        );
        patternContext.arc(0f, 0f, 50 * scale, 0f, (0.5 * Math.PI).toFloat());
        patternContext.stroke();
// Create our primary canvas and fill it with the pattern
        val ctx = canvas.getContext("2d") as CanvasRenderingContext2D
        val pattern = ctx.createPattern(patternCanvas, Pattern.PatternRepetition.Repeat)
        ctx.fillStyle = pattern
        ctx.fillRect(0f, 0f, canvas.width * scale, canvas.height * scale);
    }


    var index_buffer = 0
    var indices: ShortArray = shortArrayOf()
    var Pmatrix: Int = 0
    var Vmatrix: Int = 0
    var Mmatrix: Int = 0
    var proj_matrix: FloatArray = floatArrayOf()
    var mov_matrix: FloatArray = floatArrayOf()
    var view_matrix: FloatArray = floatArrayOf()
    var time_old: Float = 0f

    var vertCode = """
    attribute vec3 position;
    uniform mat4 Pmatrix;
    uniform mat4 Vmatrix;
    uniform mat4 Mmatrix;
    attribute vec3 color;
    varying vec3 vColor;
    void main() {
    gl_Position = Pmatrix * Vmatrix * Mmatrix * vec4(position, 1.0);
    vColor = color;
    }
    """

    var fragCode = """
    precision mediump float;
    varying vec3 vColor;
    void main() {
    gl_FragColor = vec4(vColor, 1.0);
    }
    """

    fun drawRotatingCube(gl: WebGLRenderingContext) {
        val width = gl.drawingBufferWidth
        var height = gl.drawingBufferHeight
        val vertices = floatArrayOf(
            -1f, -1f, -1f, 1f, -1f, -1f, 1f, 1f, -1f, -1f, 1f, -1f,
            -1f, -1f, 1f, 1f, -1f, 1f, 1f, 1f, 1f, -1f, 1f, 1f,
            -1f, -1f, -1f, -1f, 1f, -1f, -1f, 1f, 1f, -1f, -1f, 1f,
            1f, -1f, -1f, 1f, 1f, -1f, 1f, 1f, 1f, 1f, -1f, 1f,
            -1f, -1f, -1f, -1f, -1f, 1f, 1f, -1f, 1f, 1f, -1f, -1f,
            -1f, 1f, -1f, -1f, 1f, 1f, 1f, 1f, 1f, 1f, 1f, -1f
        )
        var colors = floatArrayOf(
            5f, 3f, 7f, 5f, 3f, 7f, 5f, 3f, 7f, 5f, 3f, 7f,
            1f, 1f, 3f, 1f, 1f, 3f, 1f, 1f, 3f, 1f, 1f, 3f,
            0f, 0f, 1f, 0f, 0f, 1f, 0f, 0f, 1f, 0f, 0f, 1f,
            1f, 0f, 0f, 1f, 0f, 0f, 1f, 0f, 0f, 1f, 0f, 0f,
            1f, 1f, 0f, 1f, 1f, 0f, 1f, 1f, 0f, 1f, 1f, 0f,
            0f, 1f, 0f, 0f, 1f, 0f, 0f, 1f, 0f, 0f, 1f, 0f
        )

        indices = shortArrayOf(
            0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7,
            8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15,
            16, 17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23
        )


        // Create and store data into vertex buffer
        val vertex_buffer = gl.createBuffer()
        gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer)
        gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW)

        // Create and store data into color buffer
        val color_buffer = gl.createBuffer()
        gl.bindBuffer(gl.ARRAY_BUFFER, color_buffer)
        gl.bufferData(gl.ARRAY_BUFFER, colors, gl.STATIC_DRAW)

        // Create and store data into index buffer
        index_buffer = gl.createBuffer()
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, index_buffer)
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, indices, gl.STATIC_DRAW)


        val vertShader = gl.createShader(gl.VERTEX_SHADER)
        gl.shaderSource(vertShader, vertCode)
        gl.compileShader(vertShader)


        val fragShader = gl.createShader(gl.FRAGMENT_SHADER)
        gl.shaderSource(fragShader, fragCode)
        gl.compileShader(fragShader)


        val shaderProgram = gl.createProgram()
        gl.attachShader(shaderProgram, vertShader)
        gl.attachShader(shaderProgram, fragShader)
        gl.linkProgram(shaderProgram)

        /* ====== Associating attributes to vertex shader =====*/
        Pmatrix = gl.getUniformLocation(shaderProgram, "Pmatrix")
        Vmatrix = gl.getUniformLocation(shaderProgram, "Vmatrix")
        Mmatrix = gl.getUniformLocation(shaderProgram, "Mmatrix")


        gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer)
        val position = gl.getAttribLocation(shaderProgram, "position")
        gl.vertexAttribPointer(position, 3, gl.FLOAT, false, 0, 0)

        // Position
        gl.enableVertexAttribArray(position)
        gl.bindBuffer(gl.ARRAY_BUFFER, color_buffer)
        val color = gl.getAttribLocation(shaderProgram, "color")
        gl.vertexAttribPointer(color, 3, gl.FLOAT, false, 0, 0)

        // Color
        gl.enableVertexAttribArray(color)
        gl.useProgram(shaderProgram)


        proj_matrix = get_projection(40f, (width / height).toFloat(), 1f, 100f)

        mov_matrix = floatArrayOf(1f, 0f, 0f, 0f, 0f, 1f, 0f, 0f, 0f, 0f, 1f, 0f, 0f, 0f, 0f, 1f)
        view_matrix = floatArrayOf(1f, 0f, 0f, 0f, 0f, 1f, 0f, 0f, 0f, 0f, 1f, 0f, 0f, 0f, 0f, 1f)

        // translating z
        view_matrix[14] = view_matrix[14] - 6;//zoom


        cubeRotationAnimation(gl, 0f)

    }


    fun cubeRotationAnimation(gl: WebGLRenderingContext, time: Float) {
        val width = gl.drawingBufferWidth
        val height = gl.drawingBufferHeight
        var dt = time - time_old
        rotateZ(mov_matrix, (dt * 0.005).toFloat()) //time
        rotateY(mov_matrix, (dt * 0.002).toFloat())
        rotateX(mov_matrix, (dt * 0.003).toFloat())
        time_old = time

        gl.enable(gl.DEPTH_TEST)
        gl.depthFunc(gl.LEQUAL)
        gl.depthMask(true)
        gl.clearColor(0.5f, 0.5f, 0.5f, 0.9f)
        gl.clearDepth(1.0f)
        gl.viewport(0, 0, width, height)
        gl.clear(gl.COLOR_BUFFER_BIT or gl.DEPTH_BUFFER_BIT)
        gl.uniformMatrix4fv(Pmatrix, false, proj_matrix)
        gl.uniformMatrix4fv(Vmatrix, false, view_matrix)
        gl.uniformMatrix4fv(Mmatrix, false, mov_matrix)
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, index_buffer)
        gl.drawElements(gl.TRIANGLES, indices.size, gl.UNSIGNED_SHORT, 0)

        AnimationFrame.requestAnimationFrame {
            cubeRotationAnimation(gl, it.toFloat())
        }

    }

    fun get_projection(angle: Float, a: Float, zMin: Float, zMax: Float): FloatArray {
        val ang = tan((angle * 0.5f) * PI / 180).toFloat() //angle*.5
        return floatArrayOf(
            0.5f / ang, 0f, 0f, 0f,
            0f, 0.5f * (a / ang), 0f, 0f,
            0f, 0f, -(zMax + zMin) / (zMax - zMin), -1f,
            0f, 0f, (-2 * zMax * zMin) / (zMax) - (zMin), 0f
        )
    }

    fun rotateZ(m: FloatArray, angle: Float) {
        val c = cos(angle)
        val s = sin(angle)
        val mv0 = m[0]
        val mv4 = m[4]
        val mv8 = m[8]

        m[0] = c * m[0] - s * m[1]
        m[4] = c * m[4] - s * m[5]
        m[8] = c * m[8] - s * m[9]

        m[1] = c * m[1] + s * mv0
        m[5] = c * m[5] + s * mv4
        m[9] = c * m[9] + s * mv8
    }

    fun rotateX(m: FloatArray, angle: Float) {
        val c = cos(angle)
        val s = sin(angle)
        val mv1 = m[1]
        val mv5 = m[5]
        val mv9 = m[9]

        m[1] = m[1] * c - m[2] * s
        m[5] = m[5] * c - m[6] * s
        m[9] = m[9] * c - m[10] * s

        m[2] = m[2] * c + mv1 * s
        m[6] = m[6] * c + mv5 * s
        m[10] = m[10] * c + mv9 * s
    }

    fun rotateY(m: FloatArray, angle: Float) {
        val c = cos(angle)
        val s = sin(angle)
        val mv0 = m[0]
        val mv4 = m[4]
        val mv8 = m[8]

        m[0] = c * m[0] + s * m[2]
        m[4] = c * m[4] + s * m[6]
        m[8] = c * m[8] + s * m[10]

        m[2] = c * m[2] - s * mv0
        m[6] = c * m[6] - s * mv4
        m[10] = c * m[10] - s * mv8
    }

    fun drawFace(ctx: CanvasRenderingContext2D) {
        ctx.beginPath();
        ctx.arc(240f, 20f, 40f, 0f, Math.PI.toFloat());
        ctx.moveTo(100f, 20f);
        ctx.arc(60f, 20f, 40f, 0f, Math.PI.toFloat());
        ctx.moveTo(215f, 80f);
        ctx.arc(150f, 80f, 65f, 0f, Math.PI.toFloat());
        ctx.closePath();
        ctx.lineWidth = 6f;
        ctx.stroke();
    }


    fun drawElements(canvas: CanvasView) {
        var gl = canvas.getContext("webgl") as WebGLRenderingContext

        var vertexShaderSrc = """
        attribute vec2 a_position;

        uniform vec2 u_resolution;

        void main() {
            vec2 zeroToOne = a_position / u_resolution;

            vec2 zeroToTwo = zeroToOne * 2.0;

            vec2 clipSpace = zeroToTwo - 1.0;

            gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
        }
        """

        var fragmentShaderSrc = """
        precision mediump float;

        uniform vec4 u_color;

        void main() {
            gl_FragColor = u_color;
        }
        """

        // setup GLSL program

        val vertexShader = gl.createShader(gl.VERTEX_SHADER);
        gl.shaderSource(vertexShader, vertexShaderSrc);
        gl.compileShader(vertexShader);

        var compiled = gl.getShaderParameter(vertexShader, gl.COMPILE_STATUS) as Boolean
        debugLog("compiled: " + compiled)
        if (!compiled) {
            // Something went wrong during compilation; get the error
            val lastError = gl.getShaderInfoLog(vertexShader);
            debugLog(
                "*** Error compiling vertexShader '" + vertexShader + "':" + lastError
            );
            gl.deleteShader(vertexShader);
            return
        }

        val fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);
        gl.shaderSource(fragmentShader, fragmentShaderSrc);
        gl.compileShader(fragmentShader);

        compiled = gl.getShaderParameter(fragmentShader, gl.COMPILE_STATUS) as Boolean
        if (!compiled) {
            // Something went wrong during compilation; get the error
            val lastError = gl.getShaderInfoLog(fragmentShader);
            debugLog(
                "*** Error compiling fragmentShader '" +
                        fragmentShader +
                        "':" +
                        lastError
            );
            gl.deleteShader(fragmentShader);
            return
        }

        val program = gl.createProgram();

        gl.attachShader(program, vertexShader);
        gl.attachShader(program, fragmentShader);
        gl.linkProgram(program);

        // Check the link status
        val linked = gl.getProgramParameter(program, gl.LINK_STATUS) as Boolean
        if (!linked) {
            // something went wrong with the link
            val lastError = gl.getProgramInfoLog(program);
            debugLog("Error in program linking:" + lastError);

            gl.deleteProgram(program);
            return
        }

        // look up where the vertex data needs to go.
        val positionAttributeLocation = gl.getAttribLocation(program, "a_position");

        // look up uniform locations
        val resolutionUniformLocation = gl.getUniformLocation(
            program,
            "u_resolution"
        );
        val colorUniformLocation = gl.getUniformLocation(program, "u_color");

        // Create a buffer to put three 2d clip space points in
        val positionBuffer = gl.createBuffer();

        // Bind it to ARRAY_BUFFER (think of it as ARRAY_BUFFER = positionBuffer)
        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

        // Tell WebGL how to convert from clip space to pixels
        gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);

        // Clear the canvas
        gl.clearColor(0f, 0f, 0f, 0f);
        gl.clear(gl.COLOR_BUFFER_BIT);

        // Tell it to use our program (pair of shaders)
        gl.useProgram(program);

        // Bind the position buffer.
        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

        // create the buffer
        val indexBuffer = gl.createBuffer();

        // make this buffer the current 'ELEMENT_ARRAY_BUFFER'
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);

        // Fill the current element array buffer with data
        val indices = shortArrayOf(
            0,
            1,
            2, // first triangle
            2,
            1,
            3 // second triangle
        )
        gl.bufferData(
            gl.ELEMENT_ARRAY_BUFFER,
            indices,
            gl.STATIC_DRAW
        );

        // code above this line is initialization code
        // --------------------------------
        // code below this line is rendering code

        // Turn on the attribute
        gl.enableVertexAttribArray(positionAttributeLocation);

        // Tell the attribute how to get data out of positionBuffer (ARRAY_BUFFER)
        var size = 2; // 2 components per iteration
        var type = gl.FLOAT; // the data is 32bit floats
        var normalize = false; // don't normalize the data
        var stride =
            0; // 0 = move forward size * sizeof(type) each iteration to get the next position
        var offset = 0; // start at the beginning of the buffer
        gl.vertexAttribPointer(
            positionAttributeLocation,
            size,
            type,
            normalize,
            stride,
            offset
        );

        // bind the buffer containing the indices
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);

        // set the resolution
        gl.uniform2f(
            resolutionUniformLocation,
            gl.drawingBufferWidth.toFloat(),
            gl.drawingBufferHeight.toFloat()
        );

        // draw 50 random rectangles in random colors
        for (i in 0 until 50) {
            // Setup a random rectangle
            // This will write to positionBuffer because
            // its the last thing we bound on the ARRAY_BUFFER
            // bind point
            setRectangle(
                gl,
                randomInt(300),
                randomInt(300),
                randomInt(300),
                randomInt(300)
            );

            // Set a random color.
            gl.uniform4f(
                colorUniformLocation,
                Math.random().toFloat(),
                Math.random().toFloat(),
                Math.random().toFloat(),
                1f
            );

            // Draw the rectangle.
            var primitiveType = gl.TRIANGLES;
            var offset = 0;
            var count = 6;
            var indexType = gl.UNSIGNED_SHORT;
            gl.drawElements(primitiveType, count, indexType, offset);
        }
    }

    // Returns a random integer from 0 to range - 1.
    fun randomInt(range: Int): Float {
        return floor(Math.random() * range).toFloat();
    }

    // Fill the buffer with the values that define a rectangle.
    fun setRectangle(gl: WebGLRenderingContext, x: Float, y: Float, width: Float, height: Float) {
        var x1 = x.toFloat()
        var x2 = (x + width).toFloat()
        var y1 = y.toFloat()
        var y2 = (y + height).toFloat()
        gl.bufferData(
            gl.ARRAY_BUFFER,
            floatArrayOf(x1, y1, x2, y1, x1, y2, x2, y2),
            gl.STATIC_DRAW
        );
    }


    fun drawModes(canvas: CanvasView, mode: String = "line") {
        /*======= Creating a canvas =========*/

        val gl = canvas.getContext("webgl") as WebGLRenderingContext

        /*======= Defining and storing the geometry ======*/

        val vertices = floatArrayOf(
            -0.7f,
            -0.1f,
            0f,
            -0.3f,
            0.6f,
            0f,
            -0.3f,
            -0.3f,
            0f,
            0.2f,
            0.6f,
            0f,
            0.3f,
            -0.3f,
            0f,
            0.7f,
            0.6f,
            0f
        )

        // Create an empty buffer object
        var vertex_buffer = gl.createBuffer();

        // Bind appropriate array buffer to it
        gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);

        // Pass the vertex data to the buffer
        gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

        // Unbind the buffer
        gl.bindBuffer(gl.ARRAY_BUFFER, null);

        /*=================== Shaders ====================*/

        // Vertex shader source code
        val vertCode =
            "attribute vec3 coordinates;" +
                    "void main(void) {" +
                    " gl_Position = vec4(coordinates, 1.0);" +
                    "}";

        // Create a vertex shader object
        val vertShader = gl.createShader(gl.VERTEX_SHADER);

        // Attach vertex shader source code
        gl.shaderSource(vertShader, vertCode);

        // Compile the vertex shader
        gl.compileShader(vertShader);

        // Fragment shader source code
        val fragCode =
            "void main(void) {" + "gl_FragColor = vec4(0.0, 0.0, 0.0, 0.1);" + "}";

        // Create fragment shader object
        val fragShader = gl.createShader(gl.FRAGMENT_SHADER);

        // Attach fragment shader source code
        gl.shaderSource(fragShader, fragCode);

        // Compile the fragmentt shader
        gl.compileShader(fragShader);

        // Create a shader program object to store
        // the combined shader program
        var shaderProgram = gl.createProgram();

        // Attach a vertex shader
        gl.attachShader(shaderProgram, vertShader);

        // Attach a fragment shader
        gl.attachShader(shaderProgram, fragShader);

        // Link both the programs
        gl.linkProgram(shaderProgram);

        // Use the combined shader program object
        gl.useProgram(shaderProgram);

        /*======= Associating shaders to buffer objects ======*/

        // Bind vertex buffer object
        gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);

        // Get the attribute location
        var coord = gl.getAttribLocation(shaderProgram, "coordinates");

        // Point an attribute to the currently bound VBO
        gl.vertexAttribPointer(coord, 3, gl.FLOAT, false, 0, 0);

        // Enable the attribute
        gl.enableVertexAttribArray(coord);

        /*============ Drawing the triangle =============*/

        // Clear the canvas
        gl.clearColor(0.5f, 0.5f, 0.5f, 0.9f)

        // Enable the depth test
//  gl.enable(gl.DEPTH_TEST);


        // Clear the color and depth buffer
        gl.clear(gl.COLOR_BUFFER_BIT or gl.DEPTH_BUFFER_BIT)

        // Set the view port
        gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);

        // Draw the triangle
        var m = gl.LINES;

        when (mode) {
            "points" ->
                m = gl.POINTS
            "line_strip" ->
                m = gl.LINE_STRIP
            "line_loop" ->
                m = gl.LINE_LOOP
            "triangle_strip" ->
                m = gl.TRIANGLE_STRIP
            "triangle_fan" ->
                m = gl.TRIANGLE_FAN
            "triangles" ->
                m = gl.TRIANGLES
            else ->
                m = gl.LINES
        }
        gl.drawArrays(m, 0, 6);
        canvas.flush()
        // POINTS, LINE_STRIP, LINE_LOOP, LINES,
        // TRIANGLE_STRIP,TRIANGLE_FAN, TRIANGLES
    }

    fun drawHouse(ctx: CanvasRenderingContext2D) {
        ctx.shadowBlur = 10.0F
        ctx.shadowColor = Color.BLUE
        ctx.shadowOffsetX = 0F
        ctx.shadowOffsetY = 0F
        ctx.lineWidth = 10f;
// Wall
        ctx.strokeRect(75f, 140f, 150f, 110f);
// Door
        ctx.fillRect(130f, 190f, 40f, 60f);

// Roof
        ctx.moveTo(50f, 140f);
        ctx.lineTo(150f, 60f);
        ctx.lineTo(250f, 140f);
        ctx.closePath();
        ctx.stroke();
    }


    class Ball {
        var x = 100.0f
        var y = 100.0f
        var vx = 5.0f
        var vy = 2.0f
        var radius = 25f
        var color = Color.BLUE
        fun draw(ctx: CanvasRenderingContext2D) {
            ctx.beginPath();
            ctx.arc(x, y, radius, 0f, (Math.PI * 2).toFloat(), true);
            ctx.closePath();
            ctx.fillStyle = Color(color)
            ctx.fill();
        }
    }


    fun draw(ctx: CanvasRenderingContext2D) {
        var s = resources.displayMetrics.density
        var canvas = ctx.canvas
        ctx.fillStyle = "rgba(255,255,255,0.3)"
        var width = canvas.width
        var height = canvas.height
        ctx.fillRect(0f, 0f, width.toFloat(), height.toFloat())
        //ctx.clearRect(0f,0f, canvas.width.toFloat(), canvas.height.toFloat());
        ball.draw(ctx)
        ball.x += ball.vx;
        ball.y += ball.vy;
        ball.vy *= 0.99f
        ball.vy += 0.25f

        if ((ball.y + ball.vy) > height ||
            ball.y + ball.vy < 0
        ) {
            ball.vy = -ball.vy;
        }
        if ((ball.x + ball.vx) > (width) ||
            ball.x + ball.vx < 0
        ) {
            ball.vx = -ball.vx;
        }

        AnimationFrame.requestAnimationFrame { called ->
            draw(ctx)
            Log.d("com.test", "requestAnimationFrame")
            // canvas?.flush()
        }
    }

    var ball = Ball()
    var ti = TimeAnimator()

    fun ballExample(ctx: CanvasRenderingContext2D) {
        //canvas?.isHandleInvalidationManually = true
        AnimationFrame.requestAnimationFrame { called ->
            draw(ctx)
        }
    }


    fun drawImageSmoothingQuality(ctx: CanvasRenderingContext2D) {
        try {
            val file = File(filesDir, "Canvas_createpattern.png")
            if (file.exists()) {
                val img = BitmapFactory.decodeFile(file.absolutePath)

                ctx.imageSmoothingQuality = CanvasRenderingContext2D.ImageSmoothingQuality.Low;
                ctx.drawImage(img, 0F, 0F, 300f, 150f);
            } else {
                val policy = StrictMode.ThreadPolicy.Builder().permitAll().build()
                StrictMode.setThreadPolicy(policy)
                val url = URL("https://mdn.mozillademos.org/files/222/Canvas_createpattern.png")
                val fs = FileOutputStream(file)
                url.openStream().use { input ->
                    fs.use { output ->
                        input.copyTo(output)
                    }
                }
                val img = BitmapFactory.decodeFile(file.absolutePath)

                ctx.imageSmoothingQuality = CanvasRenderingContext2D.ImageSmoothingQuality.Low;
                ctx.drawImage(img, 0F, 0F, 300F, 150F);

            }

        } catch (e: IOException) {

        }
    }

    fun drawImageSmoothingEnabled(ctx: CanvasRenderingContext2D) {
        try {
            val file = File(filesDir, "star.jpg")
            if (file.exists()) {
                val img = BitmapFactory.decodeFile(file.absolutePath)

                val w = img.width.toFloat()
                val h = img.height.toFloat()

                ctx.fillText("Source", (w * .5).toFloat(), 20F);
                ctx.drawImage(img, 0F, 24F, w, h);

                ctx.fillText("Smoothing = TRUE", (w * 2.5).toFloat(), 20F);
                ctx.imageSmoothingEnabled = true;
                ctx.drawImage(img, w, 24F, w * 3, h * 3);

                ctx.fillText("Smoothing = FALSE", w * 5.5F, 20F);
                ctx.imageSmoothingEnabled = false;
                ctx.drawImage(img, w * 4, 24F, w * 3, h * 3);
            } else {
                val policy = StrictMode.ThreadPolicy.Builder().permitAll().build()
                StrictMode.setThreadPolicy(policy)
                val url =
                    URL("https://interactive-examples.mdn.mozilla.net/media/examples/star.png")
                val fs = FileOutputStream(file)
                url.openStream().use { input ->
                    fs.use { output ->
                        input.copyTo(output)
                    }
                }
                val img = BitmapFactory.decodeFile(file.absolutePath)

                val w = img.width.toFloat()
                val h = img.height.toFloat()

                ctx.fillText("Source", (w * .5).toFloat(), 20F);
                ctx.drawImage(img, 0F, 24F, w, h);

                ctx.fillText("Smoothing = TRUE", (w * 2.5).toFloat(), 20F);
                ctx.imageSmoothingEnabled = true;
                ctx.drawImage(img, w, 24F, w * 3, h * 3);

                ctx.fillText("Smoothing = FALSE", w * 5.5F, 20F);
                ctx.imageSmoothingEnabled = false;
                ctx.drawImage(img, w * 4, 24F, w * 3, h * 3);

            }

        } catch (e: IOException) {

        }
    }

    fun drawImageExample(ctx: CanvasRenderingContext2D) {
        try {
            val file = File(filesDir, "rhino.jpg")
            if (file.exists()) {
                val image = ImageAsset()
                image.loadImageFromPath(file.absolutePath)
                ctx.drawImage(image, 33F, 71F, 104F, 124F, 21F, 20F, 87F, 104F)
            } else {
                val policy = StrictMode.ThreadPolicy.Builder().permitAll().build()
                StrictMode.setThreadPolicy(policy)
                val url =
                    URL("https://mdn.mozillademos.org/files/5397/rhino.jpg") // URL("https://mdn.mozillademos.org/files/5397/rhino.jpg")
                val fs = FileOutputStream(file)
                url.openStream().use { input ->
                    fs.use { output ->
                        input.copyTo(output)
                    }
                }
                val image = BitmapFactory.decodeFile(file.absolutePath)
                ctx.drawImage(image, 0f, 0f)
            }

        } catch (e: IOException) {
            e.printStackTrace()
        }
    }


    fun drawTriangle(ctx: CanvasRenderingContext2D) {
        ctx.beginPath();
        ctx.moveTo(20f, 140f);   // Move pen to bottom-left corner
        ctx.lineTo(120f, 10f);   // Line to top corner
        ctx.lineTo(220f, 140f);  // Line to bottom-right corner
        ctx.closePath();       // Line to bottom-left corner
        ctx.stroke();
    }

    fun drawArcMDN(ctx: CanvasRenderingContext2D) {
        // Tangential lines
        ctx.beginPath();
        ctx.strokeStyle = Color(Color.GRAY);
        ctx.moveTo(200f, 20f);
        ctx.lineTo(200f, 130f);
        ctx.lineTo(50f, 20f);
        ctx.stroke();

// Arc
        ctx.beginPath();
        ctx.strokeStyle = Color(Color.BLACK);
        ctx.lineWidth = 5f;
        ctx.moveTo(200f, 20f);
        ctx.arcTo(200f, 130f, 50f, 20f, 40f);
        ctx.stroke();

// Start point
        ctx.beginPath();
        ctx.fillStyle = Color(Color.BLUE);
        ctx.arc(200f, 20f, 5f, 0f, ((2 * Math.PI).toFloat()));
        ctx.fill();

// Control points
        ctx.beginPath();
        ctx.fillStyle = Color(Color.RED);
        ctx.arc(200f, 130f, 5f, 0f, (2 * Math.PI).toFloat()); // Control point one
        ctx.arc(50f, 20f, 5f, 0f, (2 * Math.PI).toFloat());   // Control point two
        ctx.fill();
    }

    class KeyValue(val x: Float, val y: Float) {
        constructor(x: Int, y: Int) : this(x.toFloat(), y.toFloat())
    }

    fun drawBezierCurveTo(ctx: CanvasRenderingContext2D) {
        // Define the points as {x, y}
        var start = KeyValue(50f, 20f)
        var cp1 = KeyValue(230f, 30f)
        var cp2 = KeyValue(150f, 80f)
        var end = KeyValue(250f, 100f)

// Cubic Bézier curve
        ctx.beginPath();
        ctx.moveTo(start.x, start.y);
        ctx.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, end.x, end.y);
        ctx.stroke();

// Start and end points
        ctx.fillStyle = Color(Color.BLUE);
        ctx.beginPath();
        ctx.arc(start.x, start.y, 5f, 0f, (2 * Math.PI).toFloat());  // Start point
        ctx.arc(end.x, end.y, 5f, 0f, (2 * Math.PI).toFloat());      // End point
        ctx.fill();

// Control points
        ctx.fillStyle = Color(Color.RED);
        ctx.beginPath();
        ctx.arc(cp1.x, cp1.y, 5f, 0f, (2 * Math.PI).toFloat());  // Control point one
        ctx.arc(cp2.x, cp2.y, 5f, 0f, (2 * Math.PI).toFloat());  // Control point two
        ctx.fill();
    }
}
