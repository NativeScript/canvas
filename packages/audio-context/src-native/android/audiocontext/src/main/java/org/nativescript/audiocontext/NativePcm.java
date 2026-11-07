package org.nativescript.audiocontext;

import androidx.annotation.Nullable;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.concurrent.ScheduledThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;

/**
 * Float PCM, one buffer per channel, in memory malloc'd natively.
 *
 * On Android a direct ByteBuffer from allocateDirect counts against the Java
 * heap, so a few decoded tracks threw OutOfMemoryError; malloc'd memory
 * wrapped by NewDirectByteBuffer doesn't. The GC doesn't free it either: every
 * holder (the buffer's id, each AudioBuffer wrapper, each native voice playing
 * it, each channel view handed to JS) retains it once and releases it once, and
 * the last release frees it.
 *
 * Neither VM sees this memory, so while the Java heap stays small nothing
 * collects the holders. When a lot of native memory is live, nudgeGc() has
 * Java collect, and the NativeScript runtime then collects V8 too if the app
 * sets gcThrottleTime.
 */
final class NativePcm {
	private static final long GC_THRESHOLD_BYTES = 128L << 20;
	private static final long GC_MIN_DELAY_MS = 3_000;
	private static final long GC_MAX_DELAY_MS = 60_000;
	private static final AtomicLong liveBytes = new AtomicLong();
	private static final AtomicBoolean gcPending = new AtomicBoolean();
	private static final ScheduledThreadPoolExecutor gcExecutor = new ScheduledThreadPoolExecutor(1, r -> {
		Thread t = new Thread(r, "NativePcmGc");
		t.setDaemon(true);
		return t;
	});

	final ByteBuffer[] channels;
	private final boolean nativeMemory;
	// The creator's reference.
	private final AtomicInteger refs = new AtomicInteger(1);

	private NativePcm(ByteBuffer[] channels, boolean nativeMemory) {
		this.channels = channels;
		this.nativeMemory = nativeMemory;
	}

	/** Zeroed buffers for {@code channelCount} channels of {@code frames} frames, or null. */
	@Nullable
	static NativePcm allocate(int channelCount, int frames) {
		int count = Math.max(1, channelCount);
		long bytes = Math.max(1L, (long) frames) * 4;
		if (bytes > Integer.MAX_VALUE - 8) return null;
		boolean nativeMemory = AudioContext.isNativeAvailable();
		ByteBuffer[] channels = new ByteBuffer[count];
		for (int i = 0; i < count; i++) {
			ByteBuffer bb = nativeMemory ? nativeAlloc(bytes) : ByteBuffer.allocateDirect((int) bytes);
			if (bb == null) {
				if (nativeMemory) {
					for (int j = 0; j < i; j++) nativeFree(channels[j]);
				}
				return null;
			}
			channels[i] = bb.order(ByteOrder.LITTLE_ENDIAN);
		}
		if (nativeMemory) {
			liveBytes.addAndGet(bytes * count);
			nudgeGc();
		}
		return new NativePcm(channels, nativeMemory);
	}

	int frames() {
		return channels[0].capacity() / 4;
	}

	/**
	 * Resizes every channel to {@code frames}, keeping what fits. Only while the
	 * creator is the sole holder (while decoding into it): it moves the memory.
	 */
	boolean resize(int frames) {
		long bytes = Math.max(1L, (long) frames) * 4;
		if (bytes > Integer.MAX_VALUE - 8 || refs.get() != 1) return false;
		for (int i = 0; i < channels.length; i++) {
			ByteBuffer bb;
			long before = channels[i].capacity();
			if (nativeMemory) {
				bb = nativeRealloc(channels[i], bytes);
			} else {
				bb = ByteBuffer.allocateDirect((int) bytes);
				ByteBuffer src = channels[i].duplicate();
				src.limit((int) Math.min(src.capacity(), bytes));
				bb.put(src);
			}
			if (bb == null) return false;
			channels[i] = bb.order(ByteOrder.LITTLE_ENDIAN);
			if (nativeMemory) liveBytes.addAndGet(bytes - before);
		}
		if (nativeMemory) nudgeGc();
		return true;
	}

	/** Takes a reference; false once it has been freed. */
	boolean retain() {
		for (; ; ) {
			int r = refs.get();
			if (r <= 0) return false;
			if (refs.compareAndSet(r, r + 1)) return true;
		}
	}

	void release() {
		if (refs.decrementAndGet() != 0) return;
		if (nativeMemory) {
			for (ByteBuffer bb : channels) {
				liveBytes.addAndGet(-bb.capacity());
				nativeFree(bb);
			}
		}
	}

	private static void nudgeGc() {
		if (liveBytes.get() < GC_THRESHOLD_BYTES || !gcPending.compareAndSet(false, true)) return;
		scheduleGc(GC_MIN_DELAY_MS);
	}

	// The holders only become unreachable after V8 has collected the JS side,
	// which it does on its own schedule, so keep asking while memory stays high.
	private static void scheduleGc(long delayMs) {
		gcExecutor.schedule(() -> {
			long before = liveBytes.get();
			System.gc();
			// Let the finalizer thread run the cleanups the collection queued.
			try {
				Thread.sleep(500);
			} catch (InterruptedException ignored) {
			}
			long after = liveBytes.get();
			if (after < GC_THRESHOLD_BYTES) {
				gcPending.set(false);
				return;
			}
			scheduleGc(after < before ? GC_MIN_DELAY_MS : Math.min(delayMs * 2, GC_MAX_DELAY_MS));
		}, delayMs, TimeUnit.MILLISECONDS);
	}

	/** A little-endian view of a channel, for reading and writing in place. */
	ByteBuffer view(int channel) {
		ByteBuffer v = channels[channel].duplicate().order(ByteOrder.LITTLE_ENDIAN);
		v.position(0);
		v.limit(v.capacity());
		return v;
	}

	/** A view that holds its own reference, released once the view is unreachable. */
	@Nullable
	ByteBuffer retainedView(int channel) {
		if (!retain()) return null;
		ByteBuffer v = view(channel);
		NativeFinalizer.register(v, this::release);
		return v;
	}

	private static native ByteBuffer nativeAlloc(long bytes);

	private static native ByteBuffer nativeRealloc(ByteBuffer buffer, long bytes);

	private static native void nativeFree(ByteBuffer buffer);
}
