package org.nativescript.audiocontext;

import androidx.annotation.Nullable;

/** Runs an AudioWorklet Worker's drain on its own looper whenever the engine requests a quantum. */
public final class WorkletWaker {
	static {
		System.loadLibrary("native-audio");
	}

	private long handle;

	private WorkletWaker(long handle) {
		this.handle = handle;
	}

	/** Call on the Worker thread; null where that thread has no looper. */
	@Nullable
	public static WorkletWaker attach(Runnable drain) {
		long handle = nativeAttach(drain);
		return handle != 0 ? new WorkletWaker(handle) : null;
	}

	public synchronized void detach() {
		if (handle == 0) return;
		nativeDetach(handle);
		handle = 0;
	}

	private static native long nativeAttach(Runnable drain);

	private static native void nativeDetach(long handle);
}
