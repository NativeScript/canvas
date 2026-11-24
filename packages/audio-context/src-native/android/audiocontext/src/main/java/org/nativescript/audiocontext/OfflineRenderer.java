package org.nativescript.audiocontext;

import android.os.Handler;
import android.os.Looper;

import java.nio.ByteBuffer;
import java.util.TreeSet;

/** Renders an OfflineAudioContext on its own thread, holding at each suspend frame until resumed. */
public class OfflineRenderer {
	public interface Callback {
		void onSuspend(long frame);

		void onComplete(AudioBuffer buffer);
	}

	private static final int QUANTUM = 128;
	// Frames per native call between hold checks.
	private static final int CHUNK = QUANTUM * 32;

	private final String contextId;
	private final int frames;
	private final int channels;
	private final int sampleRate;
	private final Object lock = new Object();
	private final TreeSet<Long> suspends = new TreeSet<>();
	private long position;
	// End of the chunk being rendered; a suspend before it comes too late.
	private long committed;
	private boolean held;
	private boolean started;

	OfflineRenderer(String contextId, int frames, int channels, int sampleRate) {
		this.contextId = contextId;
		this.frames = Math.max(0, frames);
		this.channels = Math.max(1, channels);
		this.sampleRate = sampleRate;
	}

	/** Holds before rendering {@code frame}, a quantum boundary; false if it has been rendered. */
	public boolean suspendAt(long frame) {
		synchronized (lock) {
			if (frame < committed || (held && frame == position) || frame >= frames) return false;
			return suspends.add(frame);
		}
	}

	public void resume() {
		synchronized (lock) {
			held = false;
			lock.notifyAll();
		}
	}

	public long getPosition() {
		synchronized (lock) {
			return position;
		}
	}

	public void start(final Callback callback) {
		synchronized (lock) {
			if (started) return;
			started = true;
		}
		Looper looper = Looper.myLooper();
		final Handler handler = new Handler(looper != null ? looper : Looper.getMainLooper());
		Thread thread = new Thread(() -> {
			AudioBuffer buffer = new AudioBuffer(frames, channels, sampleRate);
			ByteBuffer[] data = buffer.getAllChannelDataRaw();
			AudioContext context = AudioContext.getInstance();
			while (true) {
				long from;
				int count;
				synchronized (lock) {
					if (position >= frames) break;
					Long next = suspends.ceiling(position);
					if (next != null && next == position) {
						suspends.remove(next);
						held = true;
						final long at = position;
						handler.post(() -> callback.onSuspend(at));
						while (held) {
							try {
								lock.wait();
							} catch (InterruptedException e) {
								return;
							}
						}
						continue;
					}
					long end = next != null ? Math.min(next, frames) : frames;
					from = position;
					count = (int) Math.min(end - position, CHUNK);
					committed = from + count;
				}
				if (!context.renderOfflineFrames(contextId, from, count, data)) break;
				synchronized (lock) {
					position = from + count;
				}
			}
			handler.post(() -> callback.onComplete(buffer));
		}, "OfflineAudioContext");
		thread.start();
	}
}
