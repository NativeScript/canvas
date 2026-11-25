package org.nativescript.audiocontext;

/** A biquad whose frequency, Q, gain and detune are gain-style params the engine reads every quantum. */
public class AudioBiquadNode implements NativeObject, AudioNode {
	private final String id;
	private final AudioParam mFrequency;
	private final AudioParam mQ;
	private final AudioParam mGain;
	private final AudioParam mDetune;
	private String currentType;

	AudioBiquadNode(String id, String type, String frequencyId, String qId, String gainId, String detuneId) {
		this.id = id;
		this.currentType = type != null ? type : "lowpass";
		this.mFrequency = new AudioParam(frequencyId);
		this.mQ = new AudioParam(qId);
		this.mGain = new AudioParam(gainId);
		this.mDetune = new AudioParam(detuneId);
		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.BIQUAD, id);
	}

	public String getId() {
		return id;
	}

	public AudioParam getFrequency() {
		return mFrequency;
	}

	public AudioParam getQ() {
		return mQ;
	}

	public AudioParam getGain() {
		return mGain;
	}

	public AudioParam getDetune() {
		return mDetune;
	}

	public String getType() {
		return currentType;
	}

	public void setType(String type) {
		if (type == null) return;
		currentType = type;
		AudioContext.getInstance().setBiquadType(id, type);
	}

	@Override
	public void release() {
		AudioContext.getInstance().releaseBiquad(id);
	}
}
