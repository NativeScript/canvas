package org.nativescript.audiocontext;

import androidx.annotation.Nullable;

public class AudioBufferSourceNode extends AudioScheduledSourceNode {
	private boolean mLoop;
	
	@Nullable
	private AudioBuffer mBuffer;

	@Nullable
	private String playbackRateId;

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
			if (playbackRateId != null) {
				ctx.attachPlaybackRateToVoice(id, "");
				ctx.attachPlaybackRateToVoice(newId, playbackRateId);
			}
			id = newId;
		}
	}

	@Override
	public void start() {
		AudioContext.getInstance().startBufferSource(id, mLoop);
	}

	@Nullable
	public AudioParam getPlaybackRateParam() {
		if (playbackRateId == null) return null;
		return new AudioParam(playbackRateId);
	}
}
