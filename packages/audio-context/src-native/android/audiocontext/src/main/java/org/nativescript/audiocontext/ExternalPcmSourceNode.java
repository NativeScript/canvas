package org.nativescript.audiocontext;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

public class ExternalPcmSourceNode extends AudioScheduledSourceNode {
	private int sampleRate;
	private int channels;
	private boolean ended = false;

	@Nullable
	private String playbackRateId;

	public ExternalPcmSourceNode(@NonNull String id, int sampleRate, int channels) {
		this(id, sampleRate, channels, null);
	}

	public ExternalPcmSourceNode(@NonNull String id, int sampleRate, int channels, @Nullable String playbackRateId) {
		super(id);
		this.sampleRate = sampleRate <= 0 ? 48000 : sampleRate;
		this.channels = channels <= 0 ? 2 : channels;
		this.playbackRateId = playbackRateId;
	}

	public int getSampleRate() {
		return sampleRate;
	}

	public int getChannels() {
		return channels;
	}

	public void configureFormat(int sampleRate, int channels) {
		if (ended) return;
		this.sampleRate = sampleRate <= 0 ? 48000 : sampleRate;
		this.channels = channels <= 0 ? 2 : channels;
		AudioContext.getInstance().configureExternalPcmSource(id, this.sampleRate, this.channels);
	}

	@Nullable
	public AudioParam getPlaybackRateParam() {
		if (playbackRateId == null) return null;
		return new AudioParam(playbackRateId);
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
