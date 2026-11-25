package org.nativescript.audiocontext;

public class AudioBiquadNode implements NativeObject, AudioNode {
	private final String id;
	private AudioParam mFrequency;
	private AudioParam mDetune;
	private AudioParam mGain;
	private AudioParam mQ;
	private double currentFrequency;
	private double currentQ;
	private double currentGain;
	private double currentDetune;
	private String currentType = "lowpass";

	AudioBiquadNode(String id) {
		this.id = id;
		this.currentType = "lowpass";
		init(350, 1, 0, 0);
		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.BIQUAD, id);
	}

	public AudioBiquadNode(AudioContextInstance context) {
		this.id = AudioContext.getInstance().createBiquad(context.getId(), "lowpass", 350, 1, 0);
		this.currentType = "lowpass";
		init(350, 1, 0, 0);
	}

	public AudioBiquadNode(AudioContextInstance context, String type, double frequency, double Q, double gain) {
		this.id = AudioContext.getInstance().createBiquad(context.getId(), type, frequency, Q, gain);
		this.currentType = type != null ? type : "lowpass";
		init(frequency, Q, gain, 0);
	}

	public AudioBiquadNode(AudioContextInstance context, String type, double frequency, double Q, double gain, double detune) {
		this.id = AudioContext.getInstance().createBiquad(context.getId(), type, frequency, Q, gain);
		this.currentType = type != null ? type : "lowpass";
		init(frequency, Q, gain, detune);
	}

	private void init(double frequency, double Q, double gain, double detune) {
		this.currentFrequency = frequency;
		this.currentQ = Q;
		this.currentGain = gain;
		this.currentDetune = detune;

		this.mFrequency = new AudioParam(this, AudioParam.Type.BIQUAD_FREQUENCY);
		this.mQ = new AudioParam(this, AudioParam.Type.BIQUAD_Q);
		this.mGain = new AudioParam(this, AudioParam.Type.BIQUAD_GAIN);
		this.mDetune = new AudioParam(this, AudioParam.Type.BIQUAD_DETUNE);
	}

	double getParam(AudioParam.Type type) {
		switch (type) {
			case BIQUAD_FREQUENCY:
				return currentFrequency;
			case BIQUAD_Q:
				return currentQ;
			case BIQUAD_GAIN:
				return currentGain;
			case BIQUAD_DETUNE:
				return currentDetune;
			default:
				return 0.0;
		}
	}

	void setParam(AudioParam.Type type, double value) {
		switch (type) {
			case BIQUAD_FREQUENCY:
				currentFrequency = value;
				break;
			case BIQUAD_Q:
				currentQ = value;
				break;
			case BIQUAD_GAIN:
				currentGain = value;
				break;
			case BIQUAD_DETUNE:
				currentDetune = value;
				break;
			default:
				break;
		}

		AudioContext.getInstance().setBiquadParams(id, currentFrequency, currentQ, currentGain, currentType);
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
		AudioContext.getInstance().setBiquadParams(id, currentFrequency, currentQ, currentGain, currentType);
	}

	public void setParams(String type, double frequency, double Q, double gain) {
		if (type != null) {
			currentType = type;
		}
		currentFrequency = frequency;
		currentQ = Q;
		currentGain = gain;
		AudioContext.getInstance().setBiquadParams(id, frequency, Q, gain, type);
	}

	@Override
	public void release() {
		AudioContext.getInstance().releaseBiquad(id);
	}
}
