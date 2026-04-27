package org.nativescript.audiocontextdemo

import android.content.Context
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.util.AttributeSet
import android.view.View
import java.nio.ByteBuffer
import androidx.core.graphics.toColorInt

class VisualizerView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null,
    defStyleAttr: Int = 0
) : View(context, attrs, defStyleAttr) {
    private val paint = Paint()
    private var analyser: org.nativescript.audiocontext.AnalyserNode? = null
    private var buf: ByteBuffer? = null
		private var arr: ByteArray? = null
    private val frameDelayMs = 16L
    private val updater = object : Runnable {
        override fun run() {
            invalidate()
            postDelayed(this, frameDelayMs)
        }
    }

    fun setAnalyser(a: org.nativescript.audiocontext.AnalyserNode?) {
        analyser = a
        // allocate buffer sized to frequency bin count
        buf = null
    }

    override fun onAttachedToWindow() {
        super.onAttachedToWindow()
        post(updater)
    }

    override fun onDetachedFromWindow() {
        removeCallbacks(updater)
        super.onDetachedFromWindow()
    }

    override fun onDraw(canvas: Canvas) {
        super.onDraw(canvas)
        val a = analyser ?: return
        val binCount = try { a.frequencyBinCount
				} catch (_: Throwable) { 64 }
        if (binCount <= 0) return

        if (buf == null || buf!!.capacity() < binCount) {
            buf = ByteBuffer.allocateDirect(binCount)
        }
        val b = buf!!
        b.position(0)
        try {
            a.getByteFrequencyData(b)
        } catch (e: Throwable) {
            // fallthrough
        }

        // read bytes
				if (arr == null){
					arr = ByteArray(binCount)
				}
				val arr = arr!!

        for (i in 0 until binCount) arr[i] = b.get(i)

        val w = width.toFloat()
        val h = height.toFloat()
        canvas.drawColor("#071026".toColorInt())

        val barCount = 64
        val step = 1.coerceAtLeast(binCount / barCount)
        val barWidth = (w / barCount) * 0.9f
        var x = 0f
        for (i in 0 until barCount) {
            val vi = (arr[i * step].toInt() and 0xff)
            val v = vi / 255f
            val barHeight = v * h
            val r = (200 * v + 55).toInt().coerceIn(0,255)
            val g = (120 * (1 - v) + 80).toInt().coerceIn(0,255)
            paint.color = Color.rgb(r, g, 120)
            canvas.drawRect(x, h - barHeight, x + barWidth, h, paint)
            x += barWidth + 2
        }
    }
}
