package org.nativescript.canvas

import android.util.Log
import java.lang.ref.PhantomReference
import java.lang.ref.Reference
import java.lang.ref.ReferenceQueue
import java.nio.ByteBuffer
import java.util.concurrent.Executors
import java.util.concurrent.atomic.AtomicBoolean
import java.util.concurrent.atomic.AtomicInteger


class TNSGcUtils {
	companion object {
		private val bufferRefQue: ReferenceQueue<in ByteBuffer?> = ReferenceQueue()
		private val keyMap: HashMap<Reference<in ByteBuffer?>, Long> = HashMap()
		private val map: HashMap<Long, Reference<in ByteBuffer?>> = HashMap()

		private var executor = Executors.newSingleThreadExecutor()

		private val monitoring: AtomicBoolean = AtomicBoolean()

		private fun startWatch() {
			if (monitoring.get()) {
				return
			}
			monitoring.set(true)
			val dropped = AtomicInteger()
			executor.execute {
				while (monitoring.get()) {
					val tmp = bufferRefQue.poll()
					if (tmp != null) {
						val key = keyMap[tmp]
						if (key != null) {
							val value = map[key]
							Log.d("com.test", "dropping value $value")
							keyMap.remove(tmp)
							map.remove(key)
							nativeDestroyU8Array(key)
							dropped.getAndIncrement()
						}
					}
				}
			}
		}

		private fun stopWatch() {
			monitoring.set(false)
			executor.shutdown()
		}

		fun restartWatch() {
			startWatch()
			executor = Executors.newSingleThreadExecutor()
			startWatch()
		}

		@JvmStatic
		fun watchItem(key: Long, value: ByteBuffer?) {
			val ref = PhantomReference(value, bufferRefQue)
			keyMap[ref] = key
			map[key] = ref
		}

		private fun watchRef(buffer: ByteBuffer) {
			val ref: Reference<ByteBuffer> = PhantomReference(buffer, bufferRefQue)
		}

		init {
			startWatch()
		}

		@JvmStatic
		private external fun nativeDestroyU8Array(array: Long)
	}

}
