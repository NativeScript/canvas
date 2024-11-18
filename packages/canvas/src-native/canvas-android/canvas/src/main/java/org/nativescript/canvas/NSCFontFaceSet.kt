package org.nativescript.canvas

import android.content.Context
import android.util.Log
import java.util.concurrent.Executors

val familyNamePattern = Regex("""(?:\d+px\s+)(["']?)([\w\s]+)\1$""")

class NSCFontFaceSet {
	private val fontCache = mutableSetOf<NSCFontFace>()
	private var executor = Executors.newSingleThreadExecutor()

	enum class NSCFontFaceSetStatus {
		loading,
		loaded
	}

	var status = NSCFontFaceSetStatus.loading
		private set

	var onStatus: ((NSCFontFaceSetStatus) -> Unit)? = null

	val iter: Iterator<NSCFontFace>
		get() {
			return fontCache.iterator()
		}

	val array: Array<NSCFontFace>
		get() {
			return fontCache.toTypedArray()
		}

	fun add(font: NSCFontFace) {
		fontCache.add(font)
	}

	fun clear() {
		fontCache.clear()
	}

	fun delete(font: NSCFontFace) {
		fontCache.remove(font)
	}

	val size: Int
		get() {
			return fontCache.size
		}

	fun check(
		font: String,
		text: String?
	): Boolean {
		val matchResult = familyNamePattern.find(font)
		return matchResult?.let { match ->
			return fontCache.find { font ->
				font.fontFamily == match.groups[2]?.value
			}?.let {
				it.font != null
			} ?: false
		} ?: false
	}

	fun load(
		context: Context,
		font: String,
		text: String?,
		callback: (List<NSCFontFace>, String?) -> Unit
	) {
		executor.execute {
			status = NSCFontFaceSetStatus.loading
			onStatus?.let {
				it(NSCFontFaceSetStatus.loading)
			}
			val matchResult = familyNamePattern.find(font)
			if (matchResult != null) {
				val first = fontCache.find {
					it.fontFamily == matchResult.groups[2]?.value
				}

				if (first != null) {
					first.loadSync(context) {}
					if (first.status == NSCFontFace.NSCFontFaceStatus.loaded) {
						status = NSCFontFaceSetStatus.loaded
						onStatus?.let {
							it(NSCFontFaceSetStatus.loaded)
						}
					}
					callback(listOf(first), null)
				} else {
					callback(emptyList(), null)
				}
			} else {
				callback(emptyList(), null)
			}
		}
	}

	companion object {
		@JvmStatic
		val instance = NSCFontFaceSet()
	}
}
