package org.nativescript.audiocontext;

import androidx.annotation.Nullable;

public class AudioBufferSourceNode extends AudioScheduledSourceNode {
	private boolean mLoop;
	
	@Nullable
	private AudioBuffer mBuffer;

	public AudioBufferSourceNode(String id, @Nullable AudioBuffer buffer) {
		super(id);
		mBuffer = buffer;
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
		stop();
		mBuffer = buffer;
		AudioContext.getInstance().switchBufferSource(id, buffer);
	}

	@Override
	public void start() {
		AudioContext.getInstance().startBufferSource(id, mLoop);
	}
}
