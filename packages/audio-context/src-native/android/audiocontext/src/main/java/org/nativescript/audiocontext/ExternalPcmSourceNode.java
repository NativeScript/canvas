package org.nativescript.audiocontext;

import androidx.annotation.NonNull;

public class ExternalPcmSourceNode extends AudioScheduledSourceNode {
	private final int sampleRate;
	private final int channels;
	private boolean ended = false;

	public ExternalPcmSourceNode(@NonNull String id, int sampleRate, int channels) {
		super(id);
		this.sampleRate = sampleRate <= 0 ? 48000 : sampleRate;
		this.channels = channels <= 0 ? 2 : channels;
	}

	public int getSampleRate() {
		return sampleRate;
	}

	public int getChannels() {
		return channels;
	}

	public void pushFrames(@NonNull float[] data) {
		if (ended) return;
		AudioContext.getInstance().pushPcmFrames(id, data);
	}

	public void pushFrames(@NonNull java.nio.FloatBuffer data) {
		if (ended) return;
		AudioContext.getInstance().pushPcmFrames(id, data);
	}

	public void endStream() {
		if (ended) return;
		ended = true;
		AudioContext.getInstance().endExternalPcmSource(id);
	}

	@Override
	public void stop() {
		if (!ended) {
			ended = true;
			AudioContext.getInstance().endExternalPcmSource(id);
		}
		super.stop();
	}
}
