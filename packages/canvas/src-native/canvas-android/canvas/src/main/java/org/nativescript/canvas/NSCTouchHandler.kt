package org.nativescript.canvas

import android.view.MotionEvent
import android.view.ScaleGestureDetector


class NSCTouchHandler(val canvas: NSCCanvas) {
	private var mScaleFactor = 1f
	private var previousSpanX: Float = 0F
	private var previousSpanY: Float = 0F
	private var currentEvent: MotionEvent? = null
	private var isScaling = false

	private val scaleListener: ScaleGestureDetector.SimpleOnScaleGestureListener =
		object : ScaleGestureDetector.SimpleOnScaleGestureListener() {

			override fun onScaleBegin(detector: ScaleGestureDetector): Boolean {
				previousSpanX = detector.currentSpanX
				previousSpanY = detector.currentSpanY
				isScaling = true
				return true
			}

			override fun onScale(detector: ScaleGestureDetector): Boolean {
				mScaleFactor *= detector.scaleFactor

				val scale = canvas.context.resources.displayMetrics.density

				val isInProgress = mScaleDetector.isInProgress

				val deltaX = (mScaleDetector.currentSpanX - previousSpanX) / scale
				val deltaY = (mScaleDetector.currentSpanY - previousSpanY) / scale
				val span = (mScaleDetector.currentSpan - mScaleDetector.previousSpan) / scale


				val sb = StringBuilder()
				sb.append("{")
				append("event", "scale", sb)
				append("deltaX", deltaX, sb)
				append("deltaY", deltaY, sb)
				// todo
				append("deltaZ", 0, sb)
				append("deltaMode", 0, sb, true)



				sb.append(",\"pointers\": [")

				currentEvent?.let { currentEvent ->
					val pointerCount = currentEvent.pointerCount ?: 0
					val last = pointerCount - 1
					for (p in 0 until pointerCount) {
						try {
							val pid: Int = currentEvent.getPointerId(p)
							sb.append("{")
							append("ptrId", pid, sb)
							append("x", currentEvent.getX(p) / scale, sb)
							append("y", currentEvent.getY(p) / scale, sb, true)

							if (p != last) {
								sb.append("},")
							} else {
								sb.append("}")
							}
						} catch (_: IllegalArgumentException) {
						}
					}
				}
				sb.append("],")


				// legacy
				//	append("wheelDelta", span, sb)
				//append("wheelDeltaX", deltaX, sb)
				append("isInProgress", isInProgress, sb, true)
				//append("wheelDeltaY", deltaY, sb, true)

				sb.append("}")

				canvas.touchEventListener?.onEvent(sb.toString(), currentEvent!!)

				return true
			}

			override fun onScaleEnd(detector: ScaleGestureDetector) {
				isScaling = false
			}
		}

	private val mScaleDetector = ScaleGestureDetector(canvas.context, scaleListener)

	fun handle(me: MotionEvent) {

		val scale = canvas.context.resources.displayMetrics.density
		currentEvent = MotionEvent.obtain(me)
		mScaleDetector.onTouchEvent(me)
		val action = me.actionMasked

		val move = action == MotionEvent.ACTION_MOVE

		if (isScaling) {
			return
		}


		val ptridx = me.actionIndex
		val x = me.getX(ptridx)
		val y = me.getY(ptridx)
		val down = (action == MotionEvent.ACTION_DOWN
			|| action == MotionEvent.ACTION_POINTER_DOWN)
		val up = (action == MotionEvent.ACTION_UP
			|| action == MotionEvent.ACTION_POINTER_UP)

		val cancel = action == MotionEvent.ACTION_CANCEL
		val ptrId: Int = try {
			me.getPointerId(ptridx)
		} catch (e: IllegalArgumentException) {
			return
		}

		if (down) {
			onPress(ptrId, x, y, me, scale)
		}

		if (move) {
			onMove(me, scale)
		}

		if (up) {
			onRelease(ptrId, x, y, me, scale)
		}

		if (cancel) {
			onCancel(ptrId, x, y, me, scale)
		}

		currentEvent?.recycle()
		currentEvent = null

	}

	private fun onPress(
		ptrId: Int, x: Float, y: Float,
		me: MotionEvent,
		scale: Float = 1f
	) {
		val sb = StringBuilder()
		sb.append("{")

		append("event", "down", sb)
		append("ptrId", ptrId, sb)
		append("x", x / scale, sb)
		append("y", y / scale, sb, true)

		sb.append("}")

		canvas.touchEventListener?.onEvent(sb.toString(), me)

	}

	private fun onRelease(
		ptrId: Int, x: Float, y: Float,
		me: MotionEvent,
		scale: Float = 1f
	) {
		val sb = StringBuilder()
		sb.append("{")

		append("event", "up", sb)
		append("ptrId", ptrId, sb)
		append("x", x / scale, sb)
		append("y", y / scale, sb, true)

		sb.append("}")

		canvas.touchEventListener?.onEvent(sb.toString(), me)
	}

	private fun onCancel(
		ptrId: Int, x: Float, y: Float,
		me: MotionEvent,
		scale: Float = 1f
	) {
		val sb = StringBuilder()
		sb.append("{")

		append("event", "cancel", sb)
		append("ptrId", ptrId, sb)
		append("x", x / scale, sb)
		append("y", y / scale, sb, true)

		sb.append("}")

		canvas.touchEventListener?.onEvent(sb.toString(), me)
	}

	private fun onMove(
		me: MotionEvent,
		scale: Float = 1f
	) {

		/*		val ret = JSONObject()
				ret.put("event", "move")
				val array = JSONArray()

				val pointerCount = me.pointerCount

				for (p in 0 until pointerCount) {
					try {
						val pid: Int = me.getPointerId(p)
						val json = JSONObject()
						json.put("event", "touchCoordinatesCallback")
						json.put("ptrId", pid)
						json.put("x", me.getX(p) / scale)
						json.put("y", me.getY(p) / scale)
						array.put(json)
					} catch (_: IllegalArgumentException) {
					}
				}

				ret.put("pointers", array)

		 */


		val sb = StringBuilder()
		sb.append("{")

		append("event", "move", sb, true)
		val pointerCount = me.pointerCount
		sb.append(",\"pointers\": [")
		val last = pointerCount - 1
		for (p in 0 until pointerCount) {
			try {
				val pid: Int = me.getPointerId(p)
				sb.append("{")
				append("ptrId", pid, sb)
				append("x", me.getX(p) / scale, sb)
				append("y", me.getY(p) / scale, sb, true)

				if (p != last) {
					sb.append("},")
				} else {
					sb.append("}")
				}
			} catch (_: IllegalArgumentException) {
			}
		}
		sb.append("]")

		sb.append("}")




		canvas.touchEventListener?.onEvent(sb.toString(), me)
	}
	/*
		private fun onTouchEvent(
			ptrId: Int, x: Float, y: Float,
			press: Boolean, release: Boolean,
			me: MotionEvent,
			scale: Float = 1f
		) {

			val sb = StringBuilder()
			sb.append("{")

			sb.append("\"move\": {")
			append("event", "mouseMoveCallback", sb)
			append("ptrId", ptrId, sb)
			append("x", x / scale, sb)
			append("y", y / scale, sb, true)
			sb.append("}")

			val pointerCount = me.pointerCount
			sb.append(",\"pointers\": [")
			val last = pointerCount - 1
			for (p in 0 until pointerCount) {
				try {
					val pid: Int = me.getPointerId(p)
					sb.append("{")
					append("event", "touchCoordinatesCallback", sb)
					append("ptrId", pid, sb)
					append("x", me.getX(p) / scale, sb)
					append("y", me.getY(p) / scale, sb, true)

					if (p != last) {
						sb.append("},")
					} else {
						sb.append("}")
					}
				} catch (_: IllegalArgumentException) {
				}
			}
			sb.append("]")

			if (press) {
				sb.append(",\"press\": {")
				append("event", "mouseDownCallback", sb)
				append("ptrId", ptrid, sb, true)
				sb.append("}")
			}

			if (release) {
				sb.append(",\"release\": {")
				append("event", "mouseUpCallback", sb)
				append("ptrId", ptrid, sb, true)
				sb.append("}")
			}

			sb.append("}")

			canvas.touchEventListener?.onEvent(sb.toString(), me)

		}


		private fun onMultitouchCoordinates(
			ptrid: Int,
			x: Float,
			y: Float,
			me: MotionEvent,
			scale: Float = 1f
		) {
			val sb = StringBuilder()
			sb.append("{")
			append("event", "touchCoordinatesCallback", sb)
			append("ptrId", ptrid, sb)
			append("x", x / scale, sb)
			append("y", y / scale, sb, true)
			sb.append("}")

			canvas.touchEventListener?.onEvent(sb.toString(), me)
		}

		private fun onMultitouchCoordinates(me: MotionEvent, scale: Float = 1f) {

			val pointerCount = me.pointerCount

			if (pointerCount == 0) {
				return
			}
			val last = pointerCount - 1
			val sb = StringBuilder()
			sb.append("[")
			for (p in 0 until pointerCount) {
				try {
					val pid: Int = me.getPointerId(p)
					sb.append("{")
					append("event", "touchCoordinatesCallback", sb)
					append("ptrId", pid, sb)
					append("x", me.getX(p) / scale, sb)
					append("y", me.getY(p) / scale, sb, true)

					if (p != last) {
						sb.append("},")
					} else {
						sb.append("}")
					}
				} catch (_: IllegalArgumentException) {
				}
			}
			sb.append("]")
			canvas.touchEventListener?.onEvent(sb.toString(), me)
		}

	 */

	private fun append(key: String, value: String, sb: StringBuilder, isLast: Boolean = false) {
		sb.append("\"${key}\": \"${value}\"${if (isLast) "" else ","}")
	}

	private fun append(key: String, value: Int, sb: StringBuilder, isLast: Boolean = false) {
		sb.append("\"${key}\": ${value}${if (isLast) "" else ","}")
	}

	private fun append(key: String, value: Float, sb: StringBuilder, isLast: Boolean = false) {
		sb.append("\"${key}\": ${value}${if (isLast) "" else ","}")
	}

	private fun append(key: String, value: Boolean, sb: StringBuilder, isLast: Boolean = false) {
		sb.append("\"${key}\": ${value}${if (isLast) "" else ","}")
	}
}
