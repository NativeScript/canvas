package org.nativescript.audiocontext;

public class GainNode implements NativeObject, AudioNode {
	private final AudioParam gainParam;
	private final String id;

	GainNode(String id) {
		this.id = id;
		gainParam = new AudioParam(id);
		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.GAIN, id);
	}

	public AudioParam getGain() {
		return gainParam;
	}

	@Override
	public String getId() {
		return id;
	}

	public void setValue(double v) {
		AudioContext.getInstance().setGain(id, v);
	}

	public void setValueAtTime(double v, double t) {
		AudioContext.getInstance().scheduleGainSet(id, gainParam.getAutomationRateCode(), v, t);
	}

	public void linearRampToValueAtTime(double v, double t) {
		AudioContext.getInstance().scheduleGainRamp(id, gainParam.getAutomationRateCode(), v, t);
	}

	@Override
	public void release() {
		AudioContext.getInstance().releaseGain(id);
	}
}
