package org.nativescript.audiocontext;


public class AudioOscillatorNode extends AudioScheduledSourceNode {
	private String mType = "sine";
	private AudioParam mFrequency;
	private AudioParam mDetune;

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
		contextId = context.getId();
		init(440);
	}

	public AudioOscillatorNode(AudioContextInstance context, String type, double frequency) {
		super(AudioContext.getInstance().createOscillator(context.getId(), type, frequency));
		contextId = context.getId();
		if (type != null) this.mType = type;
		init(frequency);
	}

	public String getType() {
		return mType;
	}

	public void setType(String type) {
		if (type == null) return;
		this.mType = type;
		AudioContext.getInstance().setOscillatorType(id, type);
	}

	public AudioParam getFrequency() {
		return mFrequency;
	}

	public AudioParam getDetune() {
		if (mDetune == null) mDetune = new AudioParam(AudioContext.getInstance().createDetune(id, resolveContextId()));
		return mDetune;
	}

	@Override
	public void start() {
		AudioContext.getInstance().startOscillator(id, mType, mFrequency.getValue());
	}

	@Override
	public void startAt(double when) {
		AudioContext.getInstance().startOscillatorAt(id, mType, mFrequency.getValue(), resolveContextId(), when);
	}

	public void setPeriodicWave(PeriodicWave pw) {
		if (pw == null) return;
		AudioContext.getInstance().attachPeriodicWaveToVoice(id, pw.getId());
	}
}
