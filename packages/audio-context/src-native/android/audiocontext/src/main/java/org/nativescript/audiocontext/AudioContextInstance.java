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

	public void setListenerParams(double positionX, double positionY, double positionZ, double forwardX, double forwardY, double forwardZ, double upX, double upY, double upZ) {
		AudioContext.getInstance().setListenerParams(this.id, positionX, positionY, positionZ, forwardX, forwardY, forwardZ, upX, upY, upZ);
	}

	public AudioParam getListenerPositionXParam() {
		return new AudioParam(this, AudioParam.Type.LISTENER_POSITION_X);
	}

	public AudioParam getListenerPositionYParam() {
		return new AudioParam(this, AudioParam.Type.LISTENER_POSITION_Y);
	}

	public AudioParam getListenerPositionZParam() {
		return new AudioParam(this, AudioParam.Type.LISTENER_POSITION_Z);
	}

	public AudioParam getListenerForwardXParam() {
		return new AudioParam(this, AudioParam.Type.LISTENER_FORWARD_X);
	}

	public AudioParam getListenerForwardYParam() {
		return new AudioParam(this, AudioParam.Type.LISTENER_FORWARD_Y);
	}

	public AudioParam getListenerForwardZParam() {
		return new AudioParam(this, AudioParam.Type.LISTENER_FORWARD_Z);
	}

	public AudioParam getListenerUpXParam() {
		return new AudioParam(this, AudioParam.Type.LISTENER_UP_X);
	}

	public AudioParam getListenerUpYParam() {
		return new AudioParam(this, AudioParam.Type.LISTENER_UP_Y);
	}

	public AudioParam getListenerUpZParam() {
		return new AudioParam(this, AudioParam.Type.LISTENER_UP_Z);
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

	public void resumeAsync(@androidx.annotation.Nullable AudioContext.AsyncCallback cb) {
		AudioContext.getInstance().resumeAsync(cb);
	}

	public void suspendAsync(@androidx.annotation.Nullable AudioContext.AsyncCallback cb) {
		AudioContext.getInstance().suspendAsync(cb);
	}

	public void closeAsync(@androidx.annotation.Nullable AudioContext.AsyncCallback cb) {
		AudioContext.getInstance().closeAsync(cb);
	}

	public AudioOscillatorNode createOscillatorNodeFrequency(String type, double frequency) {
		String id = AudioContext.getInstance().createOscillator(getId(), type, frequency);
		if (id == null) return null;
		return new AudioOscillatorNode(id, type, frequency);
	}

	public ExternalPcmSourceNode createExternalPcmSource(int sampleRate, int channels) {
		String id = AudioContext.getInstance().createExternalPcmSource(sampleRate, channels);
		if (id == null) return null;
		return new ExternalPcmSourceNode(id, sampleRate, channels);
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
