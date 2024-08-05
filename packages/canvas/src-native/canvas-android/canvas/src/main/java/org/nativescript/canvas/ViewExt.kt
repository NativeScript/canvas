package org.nativescript.canvas

import android.view.View
import java.nio.FloatBuffer

fun View.getBoundingClientRect(buffer: FloatBuffer) {
	val density = context.resources.displayMetrics.density
	val densityInverse = 1.0f / density
	buffer.put(0, top * densityInverse)
	buffer.put(1, right * densityInverse)
	buffer.put(2, bottom * densityInverse)
	buffer.put(3, left * densityInverse)
	buffer.put(4, width * densityInverse)
	buffer.put(5, height * densityInverse)
	buffer.put(6, x * densityInverse)
	buffer.put(7, y * densityInverse)
}

fun View.getBoundingClientRectJSON(): String {
	val density = context.resources.displayMetrics.density

	val sb = StringBuilder()
	sb.append("{")
	val densityInverse = 1.0f / density

	NSCCanvas.append("top", top * densityInverse, sb)
	NSCCanvas.append("right", right * densityInverse, sb)
	NSCCanvas.append("bottom", bottom * densityInverse, sb)
	NSCCanvas.append("left", left * densityInverse, sb)
	NSCCanvas.append("width", width * densityInverse, sb)
	NSCCanvas.append("height", height * densityInverse, sb)
	NSCCanvas.append("x", x * densityInverse, sb)
	NSCCanvas.append("y", y * densityInverse, sb, true)

	sb.append("}")

	return sb.toString()
}
