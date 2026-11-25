package org.nativescript.audiocontext;

import androidx.annotation.Nullable;

public class AudioBufferSourceNode extends AudioScheduledSourceNode {
	private boolean mLoop;
	private double mLoopStart;
	private double mLoopEnd;
	@Nullable
	private String detuneId;
	@Nullable
	private AudioParam mDetune;
	
	@Nullable
	private AudioBuffer mBuffer;

	@Nullable
	private String playbackRateId;

	// The native buffer made for this source, released once the source is
	// unreachable and has stopped playing.
	@Nullable
	private String ownedId;
	@Nullable
	private Object ownedBufferHandle;

	public AudioBufferSourceNode(String id, @Nullable AudioBuffer buffer) {
		super(id);
		mBuffer = buffer;
		this.playbackRateId = null;
	}

	public AudioBufferSourceNode(String id, @Nullable AudioBuffer buffer, @Nullable String playbackRateId) {
		super(id);
		mBuffer = buffer;
		this.playbackRateId = playbackRateId;
	}

	public AudioBufferSourceNode(String id) {
		super(id);
		mBuffer = null;
	}

	public boolean getLoop() {
		return mLoop;
	}

	public void setLoop(boolean loop) {
		mLoop = loop;
		syncLoop();
	}

	public double getLoopStart() {
		return mLoopStart;
	}

	public void setLoopStart(double value) {
		mLoopStart = value;
		syncLoop();
	}

	public double getLoopEnd() {
		return mLoopEnd;
	}

	public void setLoopEnd(double value) {
		mLoopEnd = value;
		syncLoop();
	}

	private void syncLoop() {
		AudioContext.getInstance().setSourceLoop(id, mLoop, mLoopStart, mLoopEnd);
	}

	public AudioParam getDetune() {
		if (mDetune == null) {
			detuneId = AudioContext.getInstance().createDetune(id, resolveContextId());
			mDetune = new AudioParam(detuneId);
		}
		return mDetune;
	}

	@Nullable
	public AudioBuffer getBuffer() {
		return mBuffer;
	}

	public void setBuffer(@Nullable AudioBuffer buffer) {
		mBuffer = buffer;
		AudioContext ctx = AudioContext.getInstance();
		String newId = ctx.switchBufferSource(id, buffer);
		if (newId != null && !newId.equals(id)) {
			ctx.replaceSource(id, newId);
			if (playbackRateId != null) {
				ctx.attachPlaybackRateToVoice(id, "");
				ctx.attachPlaybackRateToVoice(newId, playbackRateId);
			}
			id = newId;
			if (detuneId != null) ctx.attachDetune(newId, detuneId);
			syncLoop();
			if (ctx.ownsSourceBuffer(newId, buffer)) ownBuffer(newId);
		}
	}

	void ownBuffer(String bufferId) {
		// The previous one was stopped by switchBufferSource.
		if (ownedBufferHandle != null) {
			NativeFinalizer.unregister(ownedBufferHandle);
			AudioContext.getInstance().releaseSourceWhenIdle(ownedId);
		}
		ownedId = bufferId;
		ownedBufferHandle = NativeFinalizer.register(this, idleReleaser(bufferId));
	}

	// Static, so the cleanup doesn't hold the node it's waiting on.
	private static Runnable idleReleaser(String bufferId) {
		return () -> AudioContext.getInstance().releaseSourceWhenIdle(bufferId);
	}

	@Override
	public void start() {
		AudioContext.getInstance().startBufferSource(id, mLoop);
	}

	@Override
	public void startAt(double when) {
		startAt(when, 0.0);
	}

	// `offset` is in seconds into the buffer.
	public void startAt(double when, double offset) {
		AudioContext.getInstance().startBufferSourceAt(id, mLoop, resolveContextId(), when, offset);
	}

	@Nullable
	public AudioParam getPlaybackRateParam() {
		if (playbackRateId == null) return null;
		return new AudioParam(playbackRateId);
	}
}
