package org.nativescript.canvas

import android.os.Handler
import android.os.Looper
import android.view.Choreographer
import android.view.Choreographer.FrameCallback
import java.util.*
import java.util.concurrent.Executors

/**
 * Created by triniwiz on 2019-08-13
 */
class AnimationFrame : FrameCallback {
	companion object {
		private var callbacks: HashMap<Long, (Long) -> Unit>? = null
		private var lastCall: Long = 0
		private var count = 0
		private var timer: Timer? = null
		private var instance: AnimationFrame? = null
		private var executorService = Executors.newCachedThreadPool()
		private val lock = Any()
		private var choreographer: Choreographer? = null
		private var handler: Handler? = null
		private var animationId: Long = 0
		private var _minFps = 1000
		private var framesRendered = 0
		private var frameStartTime: Long = 0
		private var inAnimationFrame = false
		private val timeInFrameBase: Long
			get() = System.nanoTime() / 1000000
		private val newId: Long
			get() {
				animationId++
				return animationId
			}

		@Synchronized
		fun requestAnimationFrame(callback: (Long) -> Unit): Long {
			if (!inAnimationFrame) {
				inAnimationFrame = true
				callback(timeInFrameBase)
				inAnimationFrame = false
				return newId
			}
			val id = newId
			callbacks!![id] = callback
			Choreographer.getInstance().postFrameCallback(instance)
			return id
		}

		fun cancelAnimationFrame(id: Long) {
			callbacks!!.remove(id)
		}

		init {
			callbacks = HashMap()
			instance = AnimationFrame()
			handler = Handler(Looper.getMainLooper())
		}
	}

	fun reset() {
		_minFps = 1000
		frameStartTime = 0
		framesRendered = 0
	}

	fun raf(fps: Long) {
		executorService.submit {
			val cbs = callbacks!!.clone() as HashMap<Long, (Long) -> Unit>
			callbacks!!.clear()
			inAnimationFrame = true
			val set: Set<Map.Entry<Long, (Long) -> Unit>> = cbs.entries
			for ((_, value) in set) {
				//if(cb.getKey() == animationId){
				handler!!.post { value(fps) }
				// }
			}
			inAnimationFrame = false
			reset()
		}
	}

	fun fps(currentTimeMillis: Long) {
		val fps: Int
		if (frameStartTime > 0) {
			// take the span in milliseconds
			val timeSpan = currentTimeMillis - frameStartTime
			framesRendered++
			if (timeSpan > 1000) {
				fps = (framesRendered * 1000 / timeSpan).toInt()
				if (fps < _minFps) {
					_minFps = fps
				}
				raf(fps.toLong())
				frameStartTime = currentTimeMillis
				framesRendered = 0
			}
		} else {
			frameStartTime = currentTimeMillis
		}
	}

	override fun doFrame(frameTimeNanos: Long) {
		fps(frameTimeNanos / 1000000)
		Choreographer.getInstance().postFrameCallback(instance)
	}
}
