package org.nativescript.audiocontext;


public class AudioOscillatorNode extends AudioScheduledSourceNode {
	private String mType = "sine";
	private AudioParam mFrequency;

	private void init(double frequency) {
		this.mFrequency = new AudioParam(id);
		this.mFrequency.setValue(frequency);
	}

	AudioOscillatorNode(String id) {
		super(id);
		init(440);
	}

	AudioOscillatorNode(String id, String type, double frequency) {
		super(id);
		if (type != null) this.mType = type;
		init(frequency);
	}

	public AudioOscillatorNode(AudioContextInstance context) {
		super(AudioContext.getInstance().createOscillator(context.getId(), "sine", 440));
		init(440);
	}

	public AudioOscillatorNode(AudioContextInstance context, String type, double frequency) {
		super(AudioContext.getInstance().createOscillator(context.getId(), type, frequency));
		init(frequency);
	}

	public String getType() {
		return mType;
	}

	public void setType(String mType) {
		this.mType = mType;
	}

	public AudioParam getFrequency() {
		return mFrequency;
	}

	@Override
	public void start() {
		AudioContext.getInstance().startOscillator(id, mType, mFrequency.getValue());
	}

	public void setPeriodicWave(PeriodicWave pw) {
		if (pw == null) return;
		AudioContext.getInstance().attachPeriodicWaveToVoice(id, pw.getId());
	}
}
