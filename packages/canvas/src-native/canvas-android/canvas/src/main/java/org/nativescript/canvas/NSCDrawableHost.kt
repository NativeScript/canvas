package org.nativescript.canvas

import android.annotation.SuppressLint
import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.HardwareRenderer
import android.graphics.Matrix
import android.graphics.PixelFormat
import android.graphics.RectF
import android.graphics.RenderNode
import android.hardware.HardwareBuffer
import android.media.Image
import android.media.ImageReader
import android.os.Build
import android.os.Handler
import android.os.Looper
import android.os.SystemClock
import android.view.MotionEvent
import android.view.View
import android.view.ViewGroup
import android.widget.FrameLayout
import androidx.annotation.RequiresApi
import java.lang.reflect.Method

/**
 * Holds a `content="drawable"` canvas's children: laid out in the window, never drawn. Snapshots
 * them when they change and routes touches to wherever the page last drew them.
 */
@SuppressLint("ViewConstructor")
class NSCDrawableHost(context: Context) : ViewGroup(context) {

	/** A captured frame of one view. Drawn by reference: each user closes it once. */
	class Snapshot internal constructor(
		val width: Int,
		val height: Int,
		private val image: Image?,
		bitmap: Bitmap?,
		private val onRelease: (() -> Unit)?,
	) : AutoCloseable {
		val hardwareBuffer: HardwareBuffer? =
			if (image != null && Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) image.hardwareBuffer else null
		private var bitmap: Bitmap? = bitmap
		private var refs = 1

		@Synchronized
		fun retain(): Snapshot {
			refs++
			return this
		}

		@Synchronized
		override fun close() {
			if (refs <= 0) return
			refs--
			if (refs == 0) {
				hardwareBuffer?.close()
				image?.close()
				bitmap?.recycle()
				bitmap = null
				onRelease?.invoke()
			}
		}

		/** `AHardwareBuffer *`, or 0. Valid while the snapshot is. */
		val texturePointer: Long
			get() = Utils.hardwareBufferPointer(hardwareBuffer)

		/** A software copy, for consumers that take no hardware buffer. */
		@Synchronized
		fun toBitmap(): Bitmap? {
			bitmap?.let { if (!it.isRecycled) return it }
			val buffer = hardwareBuffer ?: return null
			if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) return null
			val wrapped = Bitmap.wrapHardwareBuffer(buffer, null) ?: return null
			val copy = wrapped.copy(Bitmap.Config.ARGB_8888, false)
			wrapped.recycle()
			bitmap = copy
			return copy
		}
	}

	private class Entry(val view: View) {
		@Volatile
		var dirty = true
		var forward: Matrix? = null
		var inverse: Matrix? = null
		var clip: RectF? = null
		var snapshot: Snapshot? = null
		var pending: Snapshot? = null
		var gpu: Any? = null
		var width = 0
		var height = 0
		/** Next poll (uptime ms) below API 26, staggered across entries. */
		var nextPoll = 0L
	}

	private val entries = LinkedHashMap<View, Entry>()
	/** Bottom to top. */
	private val hitOrder = ArrayList<Entry>()
	private val handler = Handler(Looper.getMainLooper())
	private var capturing = false
	private var touchTarget: Entry? = null
	private var touchInverse: Matrix? = null

	init {
		setWillNotDraw(true)
		clipChildren = false
		// Above the canvas's surfaces, which it adds and swaps later, for touch order.
		translationZ = 10000f
	}


	override fun dispatchDraw(canvas: Canvas) {}

	override fun generateDefaultLayoutParams(): LayoutParams = newLayoutParams()

	override fun generateLayoutParams(p: LayoutParams?): LayoutParams =
		if (p is FrameLayout.LayoutParams) p else newLayoutParams()

	override fun checkLayoutParams(p: LayoutParams?): Boolean = p is FrameLayout.LayoutParams

	override fun onMeasure(widthMeasureSpec: Int, heightMeasureSpec: Int) {
		var siblingWidth = 0
		var siblingHeight = 0
		(parent as? ViewGroup)?.let { group ->
			for (i in 0 until group.childCount) {
				val child = group.getChildAt(i)
				if (child !== this && child.visibility != GONE) {
					siblingWidth = maxOf(siblingWidth, child.measuredWidth)
					siblingHeight = maxOf(siblingHeight, child.measuredHeight)
				}
			}
		}
		val width = if (MeasureSpec.getMode(widthMeasureSpec) == MeasureSpec.EXACTLY) MeasureSpec.getSize(widthMeasureSpec) else siblingWidth
		val height = if (MeasureSpec.getMode(heightMeasureSpec) == MeasureSpec.EXACTLY) MeasureSpec.getSize(heightMeasureSpec) else siblingHeight
		val childWidth = MeasureSpec.makeMeasureSpec(width, MeasureSpec.EXACTLY)
		val childHeight = MeasureSpec.makeMeasureSpec(0, MeasureSpec.UNSPECIFIED)
		for (i in 0 until childCount) {
			val child = getChildAt(i)
			if (child.visibility == GONE) continue
			if (!NsLayout.measure(child, childWidth, childHeight)) {
				measureChildWithMargins(child, childWidth, 0, childHeight, 0)
			}
		}
		// Only a size the canvas already has, so the children never resize it.
		setMeasuredDimension(width, height)
	}

	override fun onLayout(changed: Boolean, l: Int, t: Int, r: Int, b: Int) {
		val width = r - l
		var y = 0
		for (i in 0 until childCount) {
			val child = getChildAt(i)
			if (child.visibility == GONE) continue
			val height = NsLayout.desiredHeight(child) ?: run {
				val lp = child.layoutParams as? MarginLayoutParams
				child.measuredHeight + (lp?.topMargin ?: 0) + (lp?.bottomMargin ?: 0)
			}
			if (!NsLayout.layout(child, 0, y, width, y + height)) {
				val lp = child.layoutParams as? MarginLayoutParams
				val left = lp?.leftMargin ?: 0
				val top = y + (lp?.topMargin ?: 0)
				child.layout(left, top, left + child.measuredWidth, top + child.measuredHeight)
			}
			y += height
		}
	}


	/** The views to snapshot, in tree order. */
	fun setDrawables(views: Array<View>) {
		val keep = HashSet<View>(views.size)
		for (view in views) {
			keep.add(view)
			entries.getOrPut(view) {
				Entry(view).also { it.nextPoll = SystemClock.uptimeMillis() + (entries.size * POLL_STAGGER_MS) % POLL_INTERVAL_MS }
			}
		}
		val iterator = entries.entries.iterator()
		while (iterator.hasNext()) {
			val (view, entry) = iterator.next()
			if (!keep.contains(view)) {
				release(entry)
				hitOrder.remove(entry)
				iterator.remove()
			}
		}
		if (entries.size != views.size || !entries.keys.toTypedArray().contentEquals(views)) {
			val ordered = LinkedHashMap<View, Entry>()
			for (view in views) ordered[view] = entries[view]!!
			entries.clear()
			entries.putAll(ordered)
		}
	}

	/** Snapshots what changed; returns the indices (in [setDrawables] order) of new snapshots. */
	fun update(scaleX: Float, scaleY: Float): IntArray? {
		val now = SystemClock.uptimeMillis()
		var changed: IntArray? = null
		var count = 0
		var index = -1
		for (entry in entries.values) {
			index++
			val view = entry.view
			if (view.width <= 0 || view.height <= 0) continue
			entry.pending?.let { next ->
				entry.pending = null
				entry.snapshot?.close()
				entry.snapshot = next
				if (changed == null) changed = IntArray(entries.size)
				changed!![count++] = index
			}
			val width = maxOf(1, Math.ceil((view.width * scaleX).toDouble() - 1e-3).toInt())
			val height = maxOf(1, Math.ceil((view.height * scaleY).toDouble() - 1e-3).toInt())
			val resized = width != entry.width || height != entry.height
			val poll = Build.VERSION.SDK_INT < Build.VERSION_CODES.O && now >= entry.nextPoll
			if (poll) entry.nextPoll = now + POLL_INTERVAL_MS
			if (!entry.dirty && !resized && !poll) continue
			entry.dirty = false
			entry.width = width
			entry.height = height
			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q && captureGpu(entry, width, height, scaleX, scaleY)) {
				continue
			}
			val bitmap = captureBitmap(entry, width, height, scaleX, scaleY) ?: continue
			entry.snapshot?.close()
			entry.snapshot = Snapshot(width, height, null, bitmap, null)
			if (changed == null) changed = IntArray(entries.size)
			changed!![count++] = index
		}
		return changed?.copyOf(count)
	}

	/** The latest snapshot of [view], retained: close it when done. */
	fun snapshot(view: View): Snapshot? = entries[view]?.snapshot?.retain()

	/** A snapshot of [view] that is not shared with the per-frame one. */
	fun capture(view: View, width: Int, height: Int, scaleX: Float, scaleY: Float, sourceX: Float, sourceY: Float): Snapshot? {
		if (view.width <= 0 || view.height <= 0 || width <= 0 || height <= 0) return null
		val bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
		drawInto(Canvas(bitmap), view, scaleX, scaleY, sourceX, sourceY, nestedOf(view))
		return Snapshot(width, height, null, bitmap, null)
	}

	fun markDirty(view: View) {
		entries[view]?.dirty = true
	}


	/** The matrix (Android Matrix values) maps [view]'s pixels to this view's; the clip is in its pixels. */
	fun setGeometry(
		view: View,
		scaleX: Float, skewX: Float, transX: Float,
		skewY: Float, scaleY: Float, transY: Float,
		persp0: Float, persp1: Float, persp2: Float,
		hasClip: Boolean, left: Float, top: Float, right: Float, bottom: Float,
		preserveOrder: Boolean,
	) {
		val entry = entries[view] ?: Entry(view).also { entries[view] = it }
		val forward = Matrix().apply {
			setValues(floatArrayOf(scaleX, skewX, transX, skewY, scaleY, transY, persp0, persp1, persp2))
		}
		val inverse = Matrix()
		if (!forward.invert(inverse)) return
		entry.forward = forward
		entry.inverse = inverse
		entry.clip = if (hasClip) RectF(left, top, right, bottom) else null
		val known = hitOrder.indexOf(entry)
		if (!(preserveOrder && known >= 0)) {
			if (known >= 0) hitOrder.removeAt(known)
			hitOrder.add(entry)
		}
	}

	fun clearGeometry(view: View) {
		val entry = entries[view] ?: return
		entry.forward = null
		entry.inverse = null
		entry.clip = null
		hitOrder.remove(entry)
		if (touchTarget === entry) touchTarget = null
	}


	override fun dispatchTouchEvent(event: MotionEvent): Boolean {
		val action = event.actionMasked
		if (action == MotionEvent.ACTION_DOWN) {
			touchTarget = null
			touchInverse = null
			val point = FloatArray(2)
			for (i in hitOrder.indices.reversed()) {
				val entry = hitOrder[i]
				val inverse = entry.inverse ?: continue
				val view = entry.view
				if (view.visibility != VISIBLE) continue
				point[0] = event.x
				point[1] = event.y
				inverse.mapPoints(point)
				if (point[0] < 0 || point[1] < 0 || point[0] >= view.width || point[1] >= view.height) continue
				if (entry.clip?.contains(point[0], point[1]) == false) continue
				touchTarget = entry
				touchInverse = Matrix(inverse)
				break
			}
		}
		val target = touchTarget ?: return false
		val copy = MotionEvent.obtain(event)
		copy.transform(touchInverse)
		val handled = try {
			target.view.dispatchTouchEvent(copy)
		} finally {
			copy.recycle()
		}
		if (action == MotionEvent.ACTION_UP || action == MotionEvent.ACTION_CANCEL) {
			touchTarget = null
		}
		if (action == MotionEvent.ACTION_DOWN && !handled) {
			touchTarget = null
			return false
		}
		// The canvas still sees the event, as if it bubbled.
		(parent as? View)?.onTouchEvent(event)
		return true
	}


	override fun onDescendantInvalidated(child: View, target: View) {
		super.onDescendantInvalidated(child, target)
		if (capturing) return
		var node: Any? = target
		while (node is View && node !== this) {
			val entry = entries[node]
			if (entry != null) {
				entry.dirty = true
				return
			}
			node = node.parent
		}
	}


	private fun nestedOf(view: View): List<View> {
		val out = ArrayList<View>()
		fun visit(node: View) {
			if (node !is ViewGroup) return
			for (i in 0 until node.childCount) {
				val child = node.getChildAt(i)
				if (entries.containsKey(child)) out.add(child) else visit(child)
			}
		}
		visit(view)
		return out
	}

	private fun drawInto(canvas: Canvas, view: View, scaleX: Float, scaleY: Float, sourceX: Float, sourceY: Float, nested: List<View>) {
		val hidden = nested.filter { it.visibility == VISIBLE }
		val save = canvas.save()
		capturing = true
		try {
			for (v in hidden) v.visibility = INVISIBLE
			canvas.scale(scaleX, scaleY)
			canvas.translate(-view.scrollX - sourceX, -view.scrollY - sourceY)
			view.draw(canvas)
		} finally {
			for (v in hidden) v.visibility = VISIBLE
			capturing = false
			canvas.restoreToCount(save)
		}
	}

	private fun captureBitmap(entry: Entry, width: Int, height: Int, scaleX: Float, scaleY: Float): Bitmap? {
		val bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
		drawInto(Canvas(bitmap), entry.view, scaleX, scaleY, 0f, 0f, nestedOf(entry.view))
		return bitmap
	}

	@RequiresApi(Build.VERSION_CODES.Q)
	private inner class Gpu(val width: Int, val height: Int) {
		val reader: ImageReader = ImageReader.newInstance(
			width, height, PixelFormat.RGBA_8888, MAX_IMAGES,
			HardwareBuffer.USAGE_GPU_SAMPLED_IMAGE or HardwareBuffer.USAGE_GPU_COLOR_OUTPUT,
		)
		val node = RenderNode("NSCDrawable")
		val renderer = HardwareRenderer()
		var outstanding = 0
		var retired = false

		init {
			node.setPosition(0, 0, width, height)
			renderer.setContentRoot(node)
			renderer.setSurface(reader.surface)
			renderer.isOpaque = false
			val density = resources.displayMetrics.density
			renderer.setLightSourceGeometry(width / 2f, 0f, 600f * density, 8f * density)
			renderer.setLightSourceAlpha(0.04f, 0.19f)
		}

		fun destroyIfDone() {
			if (retired && outstanding == 0) {
				renderer.destroy()
				node.discardDisplayList()
				reader.close()
			}
		}
	}

	@RequiresApi(Build.VERSION_CODES.Q)
	private fun captureGpu(entry: Entry, width: Int, height: Int, scaleX: Float, scaleY: Float): Boolean {
		if (!entry.view.isHardwareAccelerated) return false
		val current = entry.gpu as? Gpu
		val gpu: Gpu = if (current != null && current.width == width && current.height == height) {
			current
		} else {
			current?.let { retire(it) }
			val created = try {
				Gpu(width, height)
			} catch (e: Throwable) {
				return false
			}
			entry.gpu = created
			created.reader.setOnImageAvailableListener({ reader ->
				val image = try {
					reader.acquireLatestImage()
				} catch (e: IllegalStateException) {
					null
				} ?: return@setOnImageAvailableListener
				// The buffer arrives before the GPU finishes it: readers would see a partial frame.
				if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
					try {
						image.fence.await(java.time.Duration.ofMillis(FENCE_WAIT_MS))
					} catch (e: Throwable) {
					}
				}
				created.outstanding++
				val snapshot = Snapshot(created.width, created.height, image, null) {
					handler.post {
						created.outstanding--
						created.destroyIfDone()
					}
				}
				if (entry.gpu === created) {
					entry.pending?.close()
					entry.pending = snapshot
				} else {
					snapshot.close()
				}
			}, handler)
			created
		}
		val canvas = gpu.node.beginRecording(width, height)
		try {
			drawInto(canvas, entry.view, scaleX, scaleY, 0f, 0f, nestedOf(entry.view))
		} finally {
			gpu.node.endRecording()
		}
		gpu.renderer.createRenderRequest()
			.setWaitForPresent(Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU)
			.syncAndDraw()
		return true
	}

	@RequiresApi(Build.VERSION_CODES.Q)
	private fun retire(gpu: Gpu) {
		gpu.retired = true
		gpu.reader.setOnImageAvailableListener(null, null)
		gpu.destroyIfDone()
	}

	private fun release(entry: Entry) {
		entry.snapshot?.close()
		entry.snapshot = null
		entry.pending?.close()
		entry.pending = null
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
			(entry.gpu as? Gpu)?.let { retire(it) }
		}
		entry.gpu = null
	}

	/** Releases every snapshot and renderer. */
	fun dispose() {
		for (entry in entries.values) release(entry)
		entries.clear()
		hitOrder.clear()
		touchTarget = null
	}

	companion object {
		private const val MAX_IMAGES = 4
		private const val FENCE_WAIT_MS = 100L
		/** Below API 26 there is no onDescendantInvalidated: idle views are polled (ms, not frames: refresh rates differ). */
		private const val POLL_INTERVAL_MS = 166L
		private const val POLL_STAGGER_MS = 17L

		private fun newLayoutParams(): LayoutParams =
			NsLayout.newParams() ?: FrameLayout.LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT)

		/** Draws [snapshot] into a 2D context on the GPU. False when it can't: draw its bitmap instead. */
		@JvmStatic
		fun draw2D(
			context: Long, snapshot: Snapshot,
			sx: Float, sy: Float, sw: Float, sh: Float,
			dx: Float, dy: Float, dw: Float, dh: Float,
		): Boolean {
			val buffer = snapshot.hardwareBuffer ?: return false
			snapshot.retain()
			val drawn = Utils.drawVideoFrameHardwareBuffer(
				context, buffer, snapshot, snapshot.width, snapshot.height,
				sx, sy, sw, sh, dx, dy, dw, dh,
			)
			if (!drawn) snapshot.close()
			return drawn
		}
	}

	/** NativeScript's own measure/layout; widgets ships with the app, so it is looked up at runtime. */
	private object NsLayout {
		private val params: Class<*>? = try {
			Class.forName("org.nativescript.widgets.CommonLayoutParams")
		} catch (e: Throwable) {
			null
		}
		private val measureChild: Method? = find("measureChild", View::class.java, Int::class.java, Int::class.java)
		private val layoutChild: Method? = find("layoutChild", View::class.java, Int::class.java, Int::class.java, Int::class.java, Int::class.java)
		private val desiredHeight: Method? = find("getDesiredHeight", View::class.java)

		private fun find(name: String, vararg types: Class<*>): Method? = try {
			params?.getDeclaredMethod(name, *types)?.apply { isAccessible = true }
		} catch (e: Throwable) {
			null
		}

		fun newParams(): LayoutParams? = try {
			params?.getConstructor()?.newInstance() as? LayoutParams
		} catch (e: Throwable) {
			null
		}

		fun measure(child: View, widthSpec: Int, heightSpec: Int): Boolean {
			val method = measureChild ?: return false
			method.invoke(null, child, widthSpec, heightSpec)
			return true
		}

		fun layout(child: View, left: Int, top: Int, right: Int, bottom: Int): Boolean {
			val method = layoutChild ?: return false
			method.invoke(null, child, left, top, right, bottom)
			return true
		}

		fun desiredHeight(child: View): Int? = desiredHeight?.invoke(null, child) as? Int
	}
}
