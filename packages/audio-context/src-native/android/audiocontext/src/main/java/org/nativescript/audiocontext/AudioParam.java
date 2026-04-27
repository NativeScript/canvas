package org.nativescript.audiocontext;

import java.util.Arrays;

public class AudioParam implements NativeObject {
	public enum Type {
		GAIN,
		BIQUAD_FREQUENCY,
		BIQUAD_Q,
		BIQUAD_GAIN,
		BIQUAD_DETUNE,
		PANNER_POSITION_X,
		PANNER_POSITION_Y,
		PANNER_POSITION_Z,
		PANNER_ORIENTATION_X,
		PANNER_ORIENTATION_Y,
		PANNER_ORIENTATION_Z,
		PANNER_PAN
	}

	private final String id;
	private final Type type;
	private final AudioPannerNode owner;
	private final AudioBiquadNode biquadOwner;

	// 0 = k-rate, 1 = a-rate
	private int automationRate = 1;

	AudioParam(String id) {
		this(id, Type.GAIN, null, null);
	}

	AudioParam(String id, Type type) {
		this(id, type, null, null);
	}

	private AudioParam(String id, Type type, AudioPannerNode owner, AudioBiquadNode biquadOwner) {
		this.id = id;
		this.type = type;
		this.owner = owner;
		this.biquadOwner = biquadOwner;
		if (owner == null && biquadOwner == null && type == Type.GAIN) {
			NativeFinalizer.register(this, NativeFinalizer.ResourceKind.GAIN, id);
		}
	}

	AudioParam(AudioPannerNode owner, Type type) {
		this(owner.getId(), type, owner, null);
	}

	AudioParam(AudioBiquadNode owner, Type type) {
		this(owner.getId(), type, null, owner);
	}

	@Override
	public String getId() {
		return id;
	}

	public void setValue(double v) {
		if (owner != null) {
			switch (type) {
				case PANNER_POSITION_X:
				case PANNER_POSITION_Y:
				case PANNER_POSITION_Z:
					owner.__setPositionComponent(type, v);
					return;
				case PANNER_ORIENTATION_X:
				case PANNER_ORIENTATION_Y:
				case PANNER_ORIENTATION_Z:
					owner.__setOrientationComponent(type, v);
					return;
				case PANNER_PAN:
					owner.setPan(v);
					return;
				default:
					break;
			}
		}
		if (biquadOwner != null) {
			biquadOwner.setParam(type, v);
			return;
		}
		AudioContext.getInstance().setGain(id, v);
	}

	public double getValue() {
		if (owner != null) {
			switch (type) {
				case PANNER_POSITION_X:
					return owner.__getPositionX();
				case PANNER_POSITION_Y:
					return owner.__getPositionY();
				case PANNER_POSITION_Z:
					return owner.__getPositionZ();
				case PANNER_ORIENTATION_X:
					return owner.__getOrientationX();
				case PANNER_ORIENTATION_Y:
					return owner.__getOrientationY();
				case PANNER_ORIENTATION_Z:
					return owner.__getOrientationZ();
				case PANNER_PAN:
					return owner.__getPan();
				default:
					break;
			}
		}
		if (biquadOwner != null) {
			return biquadOwner.getParam(type);
		}
		return AudioContext.getInstance().getGain(id);
	}

	public void setValueAtTime(double v, double t) {
		if (biquadOwner != null) {
			setValue(v);
			return;
		}
		if (owner != null) {
			AudioContext.getInstance().schedulePannerSet(owner.getId(), this.type.ordinal(), this.automationRate, v, t);
			if (t <= 0.0) setValue(v);
			return;
		}
		AudioContext.getInstance().scheduleGainSet(id, this.automationRate, v, t);
		if (t <= 0.0) AudioContext.getInstance().setGain(id, v);
	}

	public void linearRampToValueAtTime(double v, double t) {
		if (biquadOwner != null) {
			setValue(v);
			return;
		}
		if (owner != null) {
			AudioContext.getInstance().schedulePannerRamp(owner.getId(), this.type.ordinal(), this.automationRate, v, t);
			if (t <= 0.0) setValue(v);
			return;
		}
		AudioContext.getInstance().scheduleGainRamp(id, this.automationRate, v, t);
	}

	public String getAutomationRate() {
		return this.automationRate == 0 ? "k-rate" : "a-rate";
	}

	public void setAutomationRate(String rate) {
		if ("k-rate".equals(rate)) this.automationRate = 0;
		else this.automationRate = 1;
	}

	public int getAutomationRateCode() {
		return this.automationRate;
	}

	public void cancelScheduledValues(double t) {
		if (biquadOwner != null) {
			return;
		}
		if (owner != null) {
			AudioContext.getInstance().cancelPannerScheduledValues(owner.getId(), this.type.ordinal(), t);
			return;
		}
		AudioContext.getInstance().cancelGainScheduledValues(id, t);
	}

	public void cancelAndHoldAtTime(double heldValue, double t) {
		if (biquadOwner != null) {
			setValue(heldValue);
			return;
		}
		if (owner != null) {
			AudioContext.getInstance().cancelAndHoldPannerScheduledValues(owner.getId(), this.type.ordinal(), this.automationRate, heldValue, t);
			return;
		}
		AudioContext.getInstance().cancelAndHoldGainScheduledValues(id, this.automationRate, heldValue, t);
	}

	public double[] getValuesForRange(double startTime, double sampleRate, int frameCount) {
		if (biquadOwner != null) {
			double value = biquadOwner.getParam(type);
			double[] out = new double[frameCount];
			Arrays.fill(out, value);
			return out;
		}
		if (owner != null) {
			return AudioContext.getInstance().getPannerParamValues(owner.getId(), this.type.ordinal(), startTime, sampleRate, frameCount);
		}
		return AudioContext.getInstance().getGainParamValues(id, startTime, sampleRate, frameCount);
	}

	@Override
	public void release() {
		if (owner == null && biquadOwner == null && type == Type.GAIN) {
			AudioContext.getInstance().releaseGain(id);
		}
	}
}
