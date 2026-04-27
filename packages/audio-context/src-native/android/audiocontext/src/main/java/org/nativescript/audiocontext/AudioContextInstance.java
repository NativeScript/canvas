package org.nativescript.audiocontext;

public class AudioContextInstance {
	private final String id;

	AudioContextInstance(String id) {
		this.id = id;
	}

	public AudioContextInstance() {
		this.id = AudioContext.getInstance().createContext();
	}

	public String getId() {
		return id;
	}

	public double getCurrentTime() {
		return AudioContext.getInstance().getContextCurrentTime(id);
	}

	public int getSampleRate() {
		return AudioContext.getInstance().getSampleRate(id);
	}


	public void renderOfflineAsync(int frames, int sampleRate, int channels, DecodeCallback cb) {
		AudioContext.getInstance().renderOfflineForContext(id, frames, sampleRate, channels, cb);
	}

	public void release() {
		AudioContext.getInstance().releaseContext(id);
	}

	public boolean setSinkId(String deviceId) {
		return AudioContext.getInstance().setSinkId(deviceId);
	}

	public AudioOscillatorNode createOscillatorNodeFrequency(String type, double frequency) {
		String id = AudioContext.getInstance().createOscillator(getId(), type, frequency);
		if (id == null) return null;
		return new AudioOscillatorNode(id, type, frequency);
	}

	public GainNode getDestination() {
		return AudioContext.getInstance().getDestination(this);
	}

	void registerContextTrack(String trackId) {
		AudioContext.getInstance().registerContextTrack(this.id, trackId);
	}

	void unregisterContextTrack(String trackId) {
		AudioContext.getInstance().unregisterContextTrack(this.id, trackId);
	}

	String[] getContextTrackIds() {
		return AudioContext.getInstance().getContextTrackIds(this.id);
	}
}
