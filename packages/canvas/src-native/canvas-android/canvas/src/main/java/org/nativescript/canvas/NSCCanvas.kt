package org.nativescript.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Color
import android.graphics.SurfaceTexture
import android.os.*
import android.util.AttributeSet
import android.view.Surface
import android.widget.FrameLayout
import androidx.core.text.TextUtilsCompat
import androidx.core.view.ViewCompat
import org.json.JSONObject
import java.util.*
import java.util.concurrent.ConcurrentHashMap


/**
 * Created by triniwiz on 3/29/20
 */
class NSCCanvas : FrameLayout {

    var nativeGL: Long = 0
        private set

    var nativeContext: Long = 0
        private set

    enum class SurfaceType {
        Texture,
        Surface
    }

    var ignorePixelScaling: Boolean
        set(value) {
            if (surfaceType == SurfaceType.Surface) {
                surfaceView.ignorePixelScaling = value
            }
            textureView.ignorePixelScaling = value
        }
        get() {
            if (surfaceType == SurfaceType.Surface) {
                return surfaceView.ignorePixelScaling
            }
            return textureView.ignorePixelScaling
        }


    private var surface: Surface? = null
    private var surfaceType = SurfaceType.Texture
    private lateinit var textureView: GLView
    private lateinit var surfaceView: GLViewSV

    constructor(context: Context) : super(context, null) {
        init(context, null, SurfaceType.Texture)
    }

    constructor(context: Context, type: SurfaceType) : super(context, null) {
        init(context, null, type)
    }

    constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
        init(context, attrs, SurfaceType.Texture)
    }

    private fun init(context: Context, attrs: AttributeSet?, type: SurfaceType) {
        textureView = GLView(context, attrs)
        textureView.canvas = this
        surfaceView = GLViewSV(context, attrs)
        surfaceView.canvas = this
        surfaceType = type
        setBackgroundColor(Color.TRANSPARENT)
        when (type) {
            SurfaceType.Texture -> {
                addView(textureView)
            }
            SurfaceType.Surface -> {
                addView(surfaceView)
            }
        }
    }

    val drawingBufferWidth: Int
        get() {
            if (surfaceType == SurfaceType.Surface) {
                return surfaceView.width
            }
            return textureView.width
        }

    val drawingBufferHeight: Int
        get() {
            if (surfaceType == SurfaceType.Surface) {
                return surfaceView.height
            }
            return textureView.height
        }

    @Synchronized
    @Throws(Throwable::class)
    protected fun finalize() {
        if (nativeGL != 0L) {
            nativeReleaseGL(nativeGL)
            nativeGL = 0
        }
        if (nativeContext != 0L) {
            nativeReleaseGLPointer(nativeContext)
            nativeContext = 0
        }
    }

    @JvmOverloads
    fun initContext(
        type: String,
        alpha: Boolean = true,
        antialias: Boolean = true,
        depth: Boolean = true,
        failIfMajorPerformanceCaveat: Boolean = false,
        powerPreference: String = "default",
        premultipliedAlpha: Boolean = true,
        preserveDrawingBuffer: Boolean = false,
        stencil: Boolean = false,
        desynchronized: Boolean = false,
        xrCompatible: Boolean = false
    ) {
        initContextWithContextAttributes(
            type,
            alpha,
            antialias,
            depth,
            failIfMajorPerformanceCaveat,
            powerPreference,
            premultipliedAlpha,
            preserveDrawingBuffer,
            stencil,
            desynchronized,
            xrCompatible
        )
    }

    fun initContextWithJsonString(type: String, contextAttributes: String?) {
        if (nativeGL != 0L) {
            return
        }
        var alpha = true

        var antialias = true

        var depth = true

        var failIfMajorPerformanceCaveat = false

        var powerPreference = "default"

        var premultipliedAlpha = true

        var preserveDrawingBuffer = false

        var stencil = false

        var desynchronized = false

        var xrCompatible = false

        contextAttributes?.let {
            try {
                val attr = JSONObject(it)
                attr.keys().forEach { key ->
                    val value = attr[key]
                    when (key) {
                        "alpha" -> {
                            alpha = (value as? Boolean) ?: true
                        }
                        "antialias" -> {
                            antialias = value as? Boolean ?: true
                        }
                        "depth" -> {
                            depth = value as? Boolean ?: true
                        }
                        "failIfMajorPerformanceCaveat" -> {
                            failIfMajorPerformanceCaveat = value as? Boolean ?: false
                        }
                        "premultipliedAlpha" -> {
                            premultipliedAlpha = value as? Boolean ?: true
                        }
                        "preserveDrawingBuffer" -> {
                            preserveDrawingBuffer = value as? Boolean ?: false
                        }
                        "stencil" -> {
                            stencil = value as? Boolean ?: false
                        }
                        "xrCompatible" -> {
                            xrCompatible = value as? Boolean ?: false
                        }
                        "desynchronized" -> {
                            desynchronized = value as? Boolean ?: false
                        }
                        "powerPreference" -> {
                            powerPreference = value as? String ?: "default"
                        }
                        else -> {}
                    }
                }
            } catch (_: Exception) {
            }
        }

        initContext(
            type,
            alpha,
            antialias,
            depth,
            failIfMajorPerformanceCaveat,
            powerPreference,
            premultipliedAlpha,
            preserveDrawingBuffer,
            stencil,
            desynchronized,
            xrCompatible
        )
    }

    fun initContextWithContextAttributes(
        type: String,
        alpha: Boolean,
        antialias: Boolean,
        depth: Boolean,
        failIfMajorPerformanceCaveat: Boolean,
        powerPreference: String,
        premultipliedAlpha: Boolean,
        preserveDrawingBuffer: Boolean,
        stencil: Boolean,
        desynchronized: Boolean,
        xrCompatible: Boolean
    ) {
        if (nativeGL != 0L) {
            return
        }
        var version = -1
        var isCanvas = false
        when (type) {
            "2d" -> {
                version = 0
                isCanvas = true
            }
            "experimental-webgl", "webgl" -> {
                version = 1
            }
            "webgl2" -> {
                version = 2
            }
        }

        if (version == -1) {
            return
        }
        val surface = if (surfaceType == SurfaceType.Surface) {
            surfaceView.holder.surface
        } else {
            textureView.isOpaque = !alpha
            textureView.surface
        }

        surface?.let {
            nativeGL = nativeInitGL(
                it,
                alpha,
                antialias,
                depth,
                failIfMajorPerformanceCaveat,
                powerPreference,
                premultipliedAlpha,
                preserveDrawingBuffer,
                stencil,
                desynchronized,
                xrCompatible,
                version,
                isCanvas
            )
            nativeContext = nativeGetGLPointer(nativeGL)
        } ?: run {
            nativeGL = nativeInitGLNoSurface(
                this.drawingBufferWidth,
                this.drawingBufferHeight,
                alpha,
                antialias,
                depth,
                failIfMajorPerformanceCaveat,
                powerPreference,
                premultipliedAlpha,
                preserveDrawingBuffer,
                stencil,
                desynchronized,
                xrCompatible,
                version,
                isCanvas
            )

            nativeContext = nativeGetGLPointer(nativeGL)
        }
    }

    fun create2DContext(
        alpha: Boolean,
        antialias: Boolean,
        depth: Boolean,
        failIfMajorPerformanceCaveat: Boolean,
        powerPreference: String,
        premultipliedAlpha: Boolean,
        preserveDrawingBuffer: Boolean,
        stencil: Boolean,
        desynchronized: Boolean,
        xrCompatible: Boolean,
        fontColor: Int
    ): Long {

        initContext(
            "2d",
            alpha,
            antialias,
            depth,
            failIfMajorPerformanceCaveat,
            powerPreference,
            premultipliedAlpha,
            preserveDrawingBuffer,
            stencil,
            desynchronized,
            xrCompatible
        )

        val density = resources.displayMetrics.densityDpi.toFloat()

        val samples = if (antialias) {
            4
        } else {
            0
        }

        return nativeCreate2DContext(
            nativeGL, this.drawingBufferWidth, this.drawingBufferHeight,
            alpha, density, samples, fontColor, density * 160, direction
        )
    }

    internal var isPaused = false
    internal var isAttachedToWindow = false

    override fun onDetachedFromWindow() {
        super.onDetachedFromWindow()
        isPaused = true
        isAttachedToWindow = false
    }

    override fun onAttachedToWindow() {
        if (surfaceType == SurfaceType.Texture && textureView.st != null) {
            // the texture view needs to be removed and clean up from the view tree when moving from offscreen to onscreen
            removeView(textureView)
            textureView.surface?.release()
            textureView.surface = null
            textureView.st?.release()
            textureView.st = null
            textureView.isCreated = false
            textureView.isReady = false
            addView(textureView)
        }
        super.onAttachedToWindow()
        isPaused = false
        isAttachedToWindow = true
    }

    interface Listener {
        fun contextReady()
        fun surfaceResize(width: Int, height: Int)
    }

    var listener: Listener? = null

    companion object {
        var views: ConcurrentHashMap<*, *> = ConcurrentHashMap<Any?, Any?>()
        internal var isLibraryLoaded = false
        const val TAG = "CanvasView"

        init {
            loadLib()
        }

        @JvmStatic
        fun loadLib() {
            if (!isLibraryLoaded) {
                System.loadLibrary("canvasnativev8")
                isLibraryLoaded = true
            }
        }

        @JvmStatic
        var enableDebug = false

        @JvmStatic
        fun layoutView(width: Float, height: Float, NSCCanvas: NSCCanvas) {
            layoutView(width.toInt(), height.toInt(), NSCCanvas)
        }

        @JvmStatic
        fun layoutView(width: Int, height: Int, canvas: NSCCanvas) {
            var rootParams = canvas.layoutParams

            if (rootParams != null && width == rootParams.width && height == rootParams.height) {
                return
            }

            if (width != 0 && height != 0) {
                if (rootParams == null) {
                    rootParams = LayoutParams(0, 0)
                }
                rootParams.width = width
                rootParams.height = height



                if (canvas.surfaceType == SurfaceType.Texture) {
                    if (canvas.textureView.surfaceTexture == null) {
                        val st = SurfaceTexture(0)
                        st.setDefaultBufferSize(width, height)
                        val surface = Surface(st)
                        canvas.textureView.st = st
                        canvas.textureView.setSurfaceTexture(st)
                        canvas.textureView.surface = surface
                        canvas.textureView.isCreated = true
                    } else if (canvas.textureView.st != null) {
                        canvas.textureView.st?.setDefaultBufferSize(width, height)
                    }
                }



                canvas.layoutParams = rootParams

                val w = MeasureSpec.makeMeasureSpec(width, MeasureSpec.EXACTLY)
                val h = MeasureSpec.makeMeasureSpec(height, MeasureSpec.EXACTLY)
                canvas.measure(w, h)
                canvas.layout(0, 0, width, height)
            }
        }

        @JvmStatic
        external fun nativeInitGL(
            surface: Surface,
            alpha: Boolean,
            antialias: Boolean,
            depth: Boolean,
            failIfMajorPerformanceCaveat: Boolean,
            powerPreference: String,
            premultipliedAlpha: Boolean,
            preserveDrawingBuffer: Boolean,
            stencil: Boolean,
            desynchronized: Boolean,
            xrCompatible: Boolean,
            version: Int,
            isCanvas: Boolean,
        ): Long

        @JvmStatic
        external fun nativeInitGLNoSurface(
            width: Int,
            height: Int,
            alpha: Boolean,
            antialias: Boolean,
            depth: Boolean,
            failIfMajorPerformanceCaveat: Boolean,
            powerPreference: String,
            premultipliedAlpha: Boolean,
            preserveDrawingBuffer: Boolean,
            stencil: Boolean,
            desynchronized: Boolean,
            xrCompatible: Boolean,
            version: Int,
            isCanvas: Boolean,
        ): Long


        @JvmStatic
        external fun nativeCreate2DContext(
            context: Long,
            width: Int,
            height: Int,
            alpha: Boolean,
            density: Float,
            samples: Int,
            font_color: Int,
            ppi: Float,
            direction: Int
        ): Long

        @JvmStatic
        external fun nativeUpdateGLSurface(
            surface: Surface,
            context: Long
        )

        @JvmStatic
        external fun nativeReleaseGL(
            context: Long
        )

        @JvmStatic
        external fun nativeGetGLPointer(
            context: Long
        ): Long

        @JvmStatic
        external fun nativeReleaseGLPointer(
            gl: Long
        )

        @JvmStatic
        external fun nativeInitContextWithCustomSurface(
            width: Float,
            height: Float,
            density: Float,
            alpha: Boolean,
            fontColor: Int,
            ppi: Float,
            direction: Int
        ): Long


        @JvmStatic
        external fun nativeResizeCustomSurface(
            context: Long,
            width: Float,
            height: Float,
            density: Float,
            alpha: Boolean,
            ppi: Int,
        )


        @JvmStatic
        external fun nativeCustomWithBitmapFlush(context: Long, view: Bitmap)


        @JvmStatic
        internal val direction: Int
            get() {
                var direction = 0
                if (TextUtilsCompat.getLayoutDirectionFromLocale(Locale.getDefault()) == ViewCompat.LAYOUT_DIRECTION_RTL) {
                    direction = 1
                }
                return direction
            }
    }
}
