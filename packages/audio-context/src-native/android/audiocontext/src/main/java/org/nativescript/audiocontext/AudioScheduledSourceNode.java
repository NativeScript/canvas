package org.nativescript.audiocontext;

public class AudioScheduledSourceNode implements NativeObject, AudioNode {
	public String id;
	@androidx.annotation.Nullable
	String contextId;

	public AudioScheduledSourceNode(String id) {
		this.id = id;
	}

	public void start() {
	}

	public void stop() {
		AudioContext.getInstance().stopTrack(id);
	}

	// `when` is in the context's currentTime seconds; the render thread starts the voice on
	// the first frame at or after it.
	public void startAt(double when) {
		start();
	}

	public void stopAt(double when) {
		AudioContext.getInstance().stopTrackAt(id, resolveContextId(), when);
	}

	@androidx.annotation.Nullable
	String resolveContextId() {
		if (contextId == null) contextId = AudioContext.getInstance().findContextForTrack(id);
		return contextId;
	}

	public void addEndedListener(@androidx.annotation.NonNull AudioContext.EndedListener listener) {
		AudioContext.getInstance().addEndedListener(id, listener);
	}

	public void removeEndedListener(@androidx.annotation.NonNull AudioContext.EndedListener listener) {
		AudioContext.getInstance().removeEndedListener(id, listener);
	}

	@Override
	public String getId() {
		return id;
	}

	@Override
	public void release() {
		stop();
	}
}
