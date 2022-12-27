package org.nativescript.canvas

import java.lang.ref.PhantomReference
import java.lang.ref.ReferenceQueue
import java.nio.ByteBuffer
import java.util.concurrent.Executors
import java.util.concurrent.atomic.AtomicBoolean


class GC {
	interface Object {
		fun dispose()
	}

	class BufferWrapper(private val nativePtr: Long, val value: Any) : Object {
		override fun dispose() {
			disposeByteBufMut(nativePtr)
		}
	}

	private var executor = Executors.newSingleThreadExecutor()
	private val monitoring = AtomicBoolean()

	private fun startWatch() {
		if (monitoring.get()) {
			return
		}
		monitoring.set(true)
		executor.execute {
			while (monitoring.get()) {
				val tmp = bufferRefQue.poll()
				tmp?.get()?.dispose()
			}
		}
	}

	private fun stopWatch() {
		monitoring.set(false)
		executor.shutdown()
	}

	fun restartWatch() {
		stopWatch()
		executor = Executors.newSingleThreadExecutor()
		startWatch()
	}

	fun watchItem(value: Object) {
		PhantomReference(value, bufferRefQue)
	}

	companion object {

		@JvmStatic
		val bufferRefQue: ReferenceQueue<Object> = ReferenceQueue()

		@JvmStatic
		val instance = GC()

		init {
			instance.startWatch()
		}

		@JvmStatic
		fun watchObject(key: Long, value: ByteBuffer) {
			PhantomReference(BufferWrapper(key, value), bufferRefQue)
		}

		@JvmStatic
		private external fun disposeByteBufMut(buffer: Long)
	}
}
